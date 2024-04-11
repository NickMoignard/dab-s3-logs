use aws_sdk_s3::{types::Bucket, Client};
use serde::{Deserialize, Serialize};
use std::{fs::{create_dir_all, File}, io::Write, path::PathBuf};
use toml;

use super::errors::BucketsError;

#[derive(Serialize, Deserialize, Debug)]
pub struct BucketDto {
  pub name: String,
  pub creation_date: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BucketsContainer {
  pub profile_name: String,
  pub buckets: Vec<BucketDto>,
}

fn bucket_to_dto(bucket: &Bucket) -> BucketDto {
  BucketDto {
    name: bucket.name.clone().unwrap(),
    creation_date: bucket.creation_date.unwrap().to_string(),
  }
}

const BUCKETS_DIRECTORY: &str = "buckets";

pub fn save_buckets_to_file(buckets: &[Bucket], profile: &str, data_directory: PathBuf) -> Result<(), BucketsError> {
  let bucket_dtos = buckets.iter().map(|bucket| {
    bucket_to_dto(bucket)
  }).collect::<Vec<BucketDto>>();

  let profile_with_buckets = BucketsContainer {
    profile_name: profile.to_string(),
    buckets: bucket_dtos,
  };

  let toml = toml::to_string(&profile_with_buckets).unwrap();

  let filename = format!("{}.toml", profile);
  let buckets_data_dir = data_directory.join(BUCKETS_DIRECTORY);

  if !buckets_data_dir.exists() {
    let result = create_dir_all(&buckets_data_dir);
    match result {
      Ok(_) => {},
      Err(e) => {
        return Err(BucketsError::DirectoryCreationError(e));
      }
    }
  }

  let file_path = buckets_data_dir.join(filename);
  let result = File::create(file_path);
  match result {
    Ok(mut file) => {
      let result = file.write_all(toml.as_bytes());
      match result {
        Ok(_) => {},
        Err(e) => {
          return Err(BucketsError::SaveBucketsError(e));
        }
      }
    },
    Err(e) => {
      return Err(BucketsError::SaveBucketsError(e));
    }
  }

  Ok(())
}

pub async fn get_buckets(client: &Client) -> Result<Vec<Bucket>, BucketsError> {
  let req = client.list_buckets();
  let response_result = req.send().await;
  match response_result {
    Ok(response) => {
      match response.buckets {
        Some(buckets) => {
          Ok(buckets)
        },
        None => Ok(Vec::new())
      }
    },
    Err(e) => {
      Err(BucketsError::ListBucketsError(e))
    }
  }
}

pub fn get_buckets_from_file(profile: &str, data_directory: PathBuf) -> Result<Vec<BucketDto>, BucketsError> {
  let filename = format!("{}.toml", profile);
  let buckets_data_dir = data_directory.join(BUCKETS_DIRECTORY);
  let file_path = buckets_data_dir.join(filename);

  let result = std::fs::read_to_string(file_path);
  match result {
    Ok(contents) => {
      let buckets_container: BucketsContainer = toml::from_str(&contents).unwrap();
      Ok(buckets_container.buckets)
    },
    Err(e) => {
      Err(BucketsError::SaveBucketsError(e))
    }
  }
}
