use aws_sdk_s3::Client;
use dialoguer::{theme::ColorfulTheme, Confirm};
use human_bytes::human_bytes;
use is_terminal::is_terminal;
use log::error as log_error;
use anyhow::Result;
use aws::s3::list_keys;
use crate::{app::{download, App}, storage::get_used_storage};

pub mod errors;

use super::reset;

const MAX_STORAGE_MSG: &str = "Not enough storage to download logs.";
const STORAGE_PROMPT: &str = "Would you like to delete existing downloaded logs?";

/// Fetch logs from S3
pub async fn fetch (client: &Client, app: &App, bucket: String, prefix: String) -> Result<Vec<std::string::String>, errors::FetchError> {
  let query = list_keys(client, &bucket, &prefix).await.unwrap();
  let used_storage = get_used_storage(app).unwrap();

  let max_storage = {
    let cfg = app.config.lock().unwrap().clone().unwrap();
    cfg.max_storage
  };

  let available_storage = max_storage as i64 - used_storage as i64;

  let mut storage_messages: Vec<String> = Vec::new();
  storage_messages.push(format!("Storage space required for query download: {}", human_bytes(query.size as f64)));
  storage_messages.push(format!("Currently used space: {}", human_bytes(used_storage as f64)));
  storage_messages.push(format!("Available storage space: {}", human_bytes(available_storage as f64)));

  if query.size as i64 > max_storage as i64 {
    log_error!("{}", MAX_STORAGE_MSG);
    for msg in storage_messages {
      log_error!("{}", msg);
    }
    return Err(errors::FetchError::NotEnoughStorage);
  }
  
  if query.size as i64 > available_storage {
    let mut should_delete_logs = true;

    if is_terminal(std::io::stdout()) {

      log_not_enough_storage_space_messages(storage_messages);

      let confirmed = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt(STORAGE_PROMPT)
        .interact()
        .unwrap();

      should_delete_logs = confirmed;
    }

    if should_delete_logs {
      let result = reset::delete_downloaded_logs().await;
      match result {
        Ok(_) => {}
        Err(e) => {
          log_error!("Failed to delete logs: {:?}", e);
          return Err(errors::FetchError::LogDeletionFailed);
        }
      }
    } else {
      return Err(errors::FetchError::NotEnoughStorage);
    }
  }

  let result = download::download_query_results(&query, bucket, app, client).await;
  match result {
    Ok(files) => {
      Ok(files)
    }
    Err(e) => {
      log_error!("Failed to download logs: {:?}", e);
      Err(errors::FetchError::DownloadFailed)
    }
  }
}

fn log_not_enough_storage_space_messages(msgs: Vec<String>) {
  log_error!("{}", MAX_STORAGE_MSG);
  for msg in msgs {
    log_error!("{}", msg);
    eprintln!("{}", msg);
  }
}

/// Preview query results before fetching
pub async fn preview (client: &Client, bucket: String, prefix: String) -> Result<(), errors::PreviewError>{
  let result = list_keys(client, &bucket, &prefix).await;

  match result {
    Ok(query) => {
      println!("Bucket: {}", query.bucket);
      println!("Prefix: {}", query.prefix);
      println!("Size: {}", human_bytes(query.size as f64));
      println!("Objects: {}", query.objects.len());
      Ok(())
    }
    Err(e) => {
      log_error!("{}", e);
      Err(errors::PreviewError::PreviewFailed)
    }
  }
}