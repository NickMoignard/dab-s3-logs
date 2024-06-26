
use log::{debug, info};

use aws_sdk_s3::{primitives::ByteStream, types::Object, Client};
use std::{collections::HashMap, fs::File, io::{BufWriter, Write}, path::Path};
use anyhow::{bail, Result, anyhow};
use human_bytes::human_bytes;

pub mod errors;
pub mod buckets;
pub mod cli_wrapper;

#[derive(Debug)]
pub struct Query {
  pub objects: ObjectMap,
  pub prefix: String,
  pub bucket: String,
  pub size: u64,
}

type ObjectMap = HashMap<String, Object>;

pub async fn list_keys(client: &Client, bucket: &str, prefix: &str) -> Result<Query> {
  // let req = client.list_objects_v2().prefix("staging/").bucket(bucket_name);
  let req = client.list_objects_v2().prefix(prefix).bucket(bucket).max_keys(1000);
  let mut res = req.into_paginator().send();

  let mut objects: HashMap<String, Object> = HashMap::new();
  let mut total_query_size: u64 = 0;

  while let Some(result) = res.next().await {
    match result {
        Ok(output) => {
          output.contents().iter().for_each(|o| {  
            total_query_size += u64::try_from(o.size.unwrap()).unwrap();
            objects.insert(o.key.clone().expect("Key missing").to_string(), o.to_owned());
          });
          debug!("Processed page of results")
        }
        Err(err) => {
            eprintln!("ERROR {err:?}")
        }
    }
  }

  let query = Query {
    objects,
    prefix: prefix.to_string(),
    bucket: bucket.to_string(),
    size: total_query_size,
  };

  log_query(&query);

  Ok(query)
}
  
pub async fn download_file(client: &Client, bucket_name: &str, key: &str, dir: &Path) -> Result<()> {
  // Validate directory
  if !dir.is_dir() {
    bail!("Path {} is not a dir", dir.display());
  }

  // Create file path
  let file_path = dir.join(key);
  let parent_dir = file_path.parent().ok_or_else(|| anyhow!("Invalid parent dir for {}", file_path.display()))?;
  if !parent_dir.exists() {
    std::fs::create_dir_all(parent_dir)?;
  }
  
  // Build and execute request
  let req = client.get_object().bucket(bucket_name).key(key);
  let res = req.send().await?;

  // Stream result into file
  let mut data: ByteStream = res.body;
  let file = File::create(&file_path)?;
  let mut buf_writer = BufWriter::new(file);
  while let Some(bytes) = data.try_next().await? {
    buf_writer.write_all(&bytes)?;
  }
  buf_writer.flush()?;

  debug!("Downloaded file: {}", key);

  Ok(())
}
  

fn log_query(query: &Query) {
  info!("Bucket: {}", query.bucket);
  info!("Prefix: {}", query.prefix);
  info!("Size: {}", human_bytes(query.size as f64));
  info!("Objects: {}", query.objects.len());
}
