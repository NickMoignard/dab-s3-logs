use aws_sdk_s3::{primitives::ByteStream, types::{Bucket, Object}, Client};
use aws_config::{self, meta::region::RegionProviderChain};
use log::{debug, info};
use regex::Regex;
use std::{collections::HashMap, fs::File, io::{BufWriter, Write}, path::{Path, PathBuf}};
use anyhow::{bail, Result, anyhow};
use human_bytes::human_bytes;

use crate::{app::App, config::update_config};
type ObjectMap = HashMap<String, Object>;
use std::fs::read_to_string;

use dialoguer::FuzzySelect;

pub mod errors;

#[derive(Debug)]
pub struct Query {
  pub objects: ObjectMap,
  pub prefix: String,
  pub bucket: String,
  pub size: u64,
}

// TODO: move to application config
const REGION: &str = "ap-southeast-2";

pub async fn get_buckets(client: &Client) -> Result<Vec<Bucket>> {
  

  let req = client.list_buckets();
  let res = req.send().await?;

  match res.buckets {
    Some(buckets) => {
      Ok(buckets)
    },
    None => Ok(Vec::new())
  }
}

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
    buf_writer.write(&bytes)?;
  }
  buf_writer.flush()?;

  debug!("Downloaded file: {}", key);

  Ok(())
}
  
pub async fn get_aws_client(profile: Option<String>, app: &App) -> Result<Client> {
  let region_provider = RegionProviderChain::default_provider().or_else(REGION);
  let mut config_builder = aws_config::from_env().region(region_provider);

  if let Some(profile) = profile {
    config_builder = config_builder.profile_name(profile);
  } else {
    let cfg = app.config.lock().unwrap().clone().unwrap();

    match cfg.aws_profile {
      Some(profile) => {
        config_builder = config_builder.profile_name(profile);
      }
      None => {}
    }
  }

  let config = config_builder.load().await;
  let client = Client::new(&config);

  Ok(client)
}

fn read_lines(filename: PathBuf) -> Vec<String> {
    read_to_string(filename) 
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect()  // gather them together into a vector
}

pub fn get_aws_profiles(app: &App) -> Result<Vec<String>, errors::AwsError> {
  let aws_config_path = {
    let cfg = app.config.lock().unwrap().clone().unwrap();
    cfg.aws_config_path
  };

  let mut results = vec![];
  // Create a regex to match text inside square brackets
  let re = Regex::new(r"\[(.*?)\]").unwrap();

  // Iterate over each line in the file
  let lines = read_lines(aws_config_path);
  for line in lines {
    let profile_name = re.find(&line);
    match profile_name {
      Some(profile) => {
        let profile = profile.as_str().to_string().clone();
        results.push(profile);
      }
      None => {}
    }
  }

    
  Ok(parse_profiles(results).unwrap())
}

fn parse_profiles(regex_matches: Vec<String>) -> Result<Vec<String>> {
  let profiles = regex_matches.iter().map(|regex_match| {
    regex_match.clone().replace("[", "").replace("]", "").replace("profile", "").trim().to_string()
  }).collect::<Vec<String>>();

  Ok(profiles)
}

pub fn select_aws_profile(app: &App) -> Result<(), errors::AwsError> {
  let profiles = get_aws_profiles(app).unwrap();
  let mut cfg_clone = app.config.lock().unwrap().clone().unwrap();


  let selection_index = FuzzySelect::new()
    .with_prompt("Select an AWS profile from `~/.aws/config` to use as the default.")
    .items(&profiles).interact().unwrap();

  let selected_profile = profiles[selection_index].clone();

  cfg_clone.aws_profile = Some(selected_profile.clone());
  update_config(cfg_clone).unwrap();

  eprint!("Selected profile: {}", &selected_profile);
  
  Ok(())
}

fn log_query(query: &Query) {
  info!("Bucket: {}", query.bucket);
  info!("Prefix: {}", query.prefix);
  info!("Size: {}", human_bytes(query.size as f64));
  info!("Objects: {}", query.objects.len());
}