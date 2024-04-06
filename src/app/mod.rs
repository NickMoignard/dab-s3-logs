use std::sync::{Arc, Mutex};
use aws_sdk_s3::{types::Bucket, Client};
use fs_extra::dir::get_size;
use log::{debug, info};
use std::thread::available_parallelism;
use anyhow::Result;
use crate::s3;
use crate::config;

pub mod download;

#[derive(Debug)]
pub struct App {
  pub buckets: Arc<Mutex<Vec<Bucket>>>,
  pub config: Mutex<Option<config::ApplicationConfig>>,
  pub used_storage:u64,
  pub available_parallelism: usize,
}

impl Default for App {
  fn default() -> Self {
    Self {
      buckets: Arc::new(Mutex::new(Vec::<Bucket>::new())),
      config: Mutex::new(None),
      used_storage: 0,
      available_parallelism: available_parallelism().unwrap().get(),
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

  let cfg_from_file = config::get_config().unwrap();
  let mut app = App::new();

  {
      let cloned_cfg = cfg_from_file.clone();
      let mut app_config_binding = app.config.lock().unwrap();
      app_config_binding.replace(cloned_cfg);
  }

  setup_directories(&app);
  let size = get_size(cfg_from_file.download_directory).unwrap();
  
  app.set_used_storage(size); 

  Ok(app)
}

fn setup_directories(app: &App) {
  let cfg_clone = app.config.lock().unwrap().clone().unwrap();

  if !cfg_clone.download_directory.exists() {
    debug!("Creating download directory: {:?}", cfg_clone.download_directory);
    let result = std::fs::create_dir_all(cfg_clone.download_directory);
    match result {
      Ok(_) => {},
      Err(e) => {
        eprintln!("Failed to create download directory: {:?}", e);
      }
    }
  }

  if !cfg_clone.cache_directory.exists() {
    debug!("Creating cache directory: {:?}", cfg_clone.cache_directory);
    let result = std::fs::create_dir_all(cfg_clone.cache_directory);
    match result {
      Ok(_) => {},
      Err(e) => {
        eprintln!("Failed to create cache directory: {:?}", e);
      }
    }
  }
}
