use aws_sdk_s3::Client;
use human_bytes::human_bytes;
use crate::{app::{download, App}, s3::list_keys};

/// Fetch logs from S3
pub async fn fetch (client: &Client, app: &App, bucket: String, prefix: String) {
  let query = list_keys(client, &bucket, &prefix).await.unwrap();
  let files = download::download_query_results(&query, bucket, app, &client).await.unwrap();
}

/// Preview query results before fetching
pub async fn preview (client: &Client, bucket: String, prefix: String) {
  let query = list_keys(client, &bucket, &prefix).await.unwrap();

  println!("Bucket: {}", query.bucket);
  println!("Prefix: {}", query.prefix);
  println!("Size: {}", human_bytes(query.size as f64));
  println!("Objects: {}", query.objects.len());
}