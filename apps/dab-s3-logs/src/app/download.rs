use aws_sdk_s3::{types::Object, Client};
use indicatif::{ProgressBar, ProgressStyle};
use tokio::sync::mpsc;
use anyhow::Result;
use std::fs;


use crate::aws::s3::{download_file, Query};

use super::App;

const PROGRESS_BAR_TEMPLATE: &str = "[{elapsed_precise}] {bar:40.cyan/blue} {bytes}/{total_bytes} {msg}";

fn create_batches_from_query(query: &Query, app: &App) -> Vec<Vec<Object>> {
  let num_downloads = query.objects.len();
  let download_thread_concurrency = {
    let cfg_clone = app.config.lock().unwrap().clone().unwrap();
    cfg_clone.download_thread_concurrency
  };

  let downloads_per_thread = num_downloads / download_thread_concurrency;
  let objects: Vec<Object> = query.objects.clone().into_values().collect();
  objects.chunks(downloads_per_thread).map(|x| x.to_vec()).collect::<Vec<Vec<Object>>>()
}

pub async fn download_query_results(query: &Query, bucket: String,  app: &App, client: &Client) -> Result<Vec<String>> {
  let progress_bar = ProgressBar::new(query.size);
  progress_bar.set_style(ProgressStyle::with_template(PROGRESS_BAR_TEMPLATE).unwrap());

  let download_dir = {
    let cfg_binding = app.config.lock().unwrap();
    cfg_binding.clone().unwrap().download_directory
  };

  let (tx, mut rx) = mpsc::channel::<DownloadResult>(128);

  let chunks = create_batches_from_query(query, app);

   for chunk in chunks {
      let tx_clone = tx.clone();
      let client_clone = client.clone();
      let download_dir_path_buf = download_dir.to_path_buf();
      let bucket = bucket.clone();
       
      tokio::spawn(async move {
          for object in chunk {
              let o = object.clone();
              let key_temp = o.key.unwrap().clone();

              download_file(&client_clone, &bucket, key_temp.as_str(), &download_dir_path_buf).await.unwrap();
  
              let file_path = download_dir_path_buf.join(key_temp).to_str().unwrap().to_string();
              let metadata = fs::metadata(file_path.clone()).unwrap();
              tx_clone.send(DownloadResult { bytes: metadata.len(), file: file_path }).await.unwrap();
          }
      });
  }

  // wait for all downloads to complete
  drop(tx);
  let mut files = Vec::new();
  while let Some(result) = rx.recv().await {
      files.push(result.file);
      progress_bar.inc(result.bytes);
  }

  progress_bar.finish_with_message("Downloaded");

  Ok(files)
}

struct DownloadResult {
  bytes: u64,
  file: String,
}