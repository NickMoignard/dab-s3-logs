use std::{path::Path, sync::{Arc, Mutex}};
use aws_sdk_s3::{types::Bucket, Client};
use fs_extra::dir::get_size;
use log::info;
use std::thread::available_parallelism;
use anyhow::Result;
use crate::{s3, storage::config::{StorageConfig, load}};

pub mod download;

#[derive(Debug)]
pub struct App {
  pub buckets: Arc<Mutex<Vec<Bucket>>>,
  pub storage_config: Mutex<Option<StorageConfig>>,
  pub used_storage:u64,
  pub parallelism: usize,
}

impl Default for App {
  fn default() -> Self {
    Self {
      buckets: Arc::new(Mutex::new(Vec::<Bucket>::new())),
      storage_config: Mutex::new(None),
      used_storage: 0,
      parallelism: available_parallelism().unwrap().get(),
    }
  }
}

impl App {
  pub fn new() -> Self {
    Self::default()
  }

  pub fn set_used_storage(&mut self, used_storage: u64) {
    self.used_storage = used_storage;
  }
}

pub async fn load_buckets (app: &App, client: &Client) {
  let buckets = s3::get_buckets(&client).await.unwrap();
  app.buckets.lock().unwrap().extend(buckets);
}

pub fn setup() -> Result<App> {
  env_logger::init();
  info!("Setting up");

  let mut app = App::new();
  {
      let mut storage_config_binding = app.storage_config.lock().unwrap();
      let config = load();
      storage_config_binding.replace(config.unwrap());

      let _ = setup_storage_dir(storage_config_binding.as_ref().unwrap());
  }

  let download_dir = {
      let storage_config = app.storage_config.lock().unwrap();
      storage_config.as_ref().unwrap().download_directory.as_str().to_string()
  };
  let size = get_size(download_dir).unwrap();
  
  app.set_used_storage(size); 

  Ok(app)
}

pub fn setup_storage_dir (config: &StorageConfig) -> std::io::Result<()> {
  if Path::new(config.download_directory.as_str()).exists() {
      return Ok(())
  }

  std::fs::create_dir_all(config.download_directory.as_str())
}
