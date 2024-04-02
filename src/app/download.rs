use std::path::Path;

use aws_sdk_s3::{types::Object, Client};
use tokio::sync::mpsc;
use anyhow::Result;

use crate::s3::{self, Query};

use super::App;

// TODO: Add to application config
const DOWNLOAD_PARALLELISM: usize = 128;

fn create_batches_from_query(query: &Query) -> Vec<Vec<Object>> {
  let num_downloads = query.objects.len();
  let downloads_per_thread = num_downloads / DOWNLOAD_PARALLELISM;
  let objects: Vec<Object> = query.objects.clone().into_values().collect();
  objects.chunks(downloads_per_thread).map(|x| x.to_vec()).collect::<Vec<Vec<Object>>>()
}

pub async fn download_query_results(query: &Query, bucket: String,  app: &App, client: &Client) -> Result<Vec<String>> {
  let storage_binding = app.storage_config.lock().unwrap();
  let download_dir = Path::new(storage_binding.as_ref().unwrap().download_directory.as_str());

  let (tx, mut rx) = mpsc::channel::<String>(128);

  let chunks = create_batches_from_query(query);

   for chunk in chunks {
      let tx_clone = tx.clone();
      let client_clone = client.clone();
      let download_dir_path_buf = download_dir.to_path_buf();
      let bucket = bucket.clone();
       
      tokio::spawn(async move {
          for object in chunk {
              let o = object.clone();
              let key_temp = o.key.unwrap().clone();

              s3::download_file(&client_clone, &bucket, key_temp.as_str(), &download_dir_path_buf).await.unwrap();
  
              let file_path = download_dir_path_buf.join(key_temp).to_str().unwrap().to_string();
              tx_clone.send(file_path).await.unwrap();
          }
      });
  }

  // wait for all downloads to complete
  drop(tx);
  let mut files = Vec::new();
  while let Some(message) = rx.recv().await {
      files.push(message);
  }

  Ok(files)
}