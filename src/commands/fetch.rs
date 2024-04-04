use aws_sdk_s3::Client;
use dialoguer::{theme::ColorfulTheme, Confirm};
use human_bytes::human_bytes;
use is_terminal::is_terminal;
use log::debug;
use crate::{app::{download, App}, s3::list_keys, storage::get_used_storage};

use super::reset;

/// Fetch logs from S3
pub async fn fetch (client: &Client, app: &App, bucket: String, prefix: String) {
  let query = list_keys(client, &bucket, &prefix).await.unwrap();
  let used_storage = get_used_storage(app).unwrap();
  let max_storage = app.storage_config.lock().unwrap().as_ref().unwrap().max_storage;

  debug!("Max Storage: {}", max_storage);
  debug!("Used Storage: {}", used_storage);
  debug!("Query Size: {}", query.size);

  let available_storage = max_storage as i64 - used_storage as i64;

  if query.size as i64 > max_storage as i64 {
    println!("Not enough storage to download logs.\n\tRequired: {}\n\tUsed: {}\n\tAvailable: {}\nPlease update max storage configuration variable or make a smaller query", human_bytes(query.size as f64), human_bytes(used_storage as f64), human_bytes(available_storage as f64));
    return;
  }
  
  if query.size as i64 > available_storage {
    let mut should_delete_logs = true;

    if is_terminal(std::io::stdout()) {
      let prompt = format!("Not enough storage to download logs.\n\tRequired: {}\n\tUsed: {}\n\tAvailable: {}\nWould you like to delete existing downloaded logs?", human_bytes(query.size as f64), human_bytes(used_storage as f64), human_bytes(available_storage as f64));
      let confirmed = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt)
        .interact()
        .unwrap();

      should_delete_logs = confirmed;
    }

    if should_delete_logs {
      let _ = reset::delete_downloaded_logs().await.unwrap();   
    } else {
      return;
    }
  }

  let _ = download::download_query_results(&query, bucket, app, &client).await.unwrap();
}

/// Preview query results before fetching
pub async fn preview (client: &Client, bucket: String, prefix: String) {
  let query = list_keys(client, &bucket, &prefix).await.unwrap();

  println!("Bucket: {}", query.bucket);
  println!("Prefix: {}", query.prefix);
  println!("Size: {}", human_bytes(query.size as f64));
  println!("Objects: {}", query.objects.len());
}