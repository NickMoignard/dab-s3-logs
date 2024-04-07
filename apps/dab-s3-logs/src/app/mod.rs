use std::sync::{Arc, Mutex};
use aws_sdk_s3::types::Bucket;
use fs_extra::dir::get_size;
use log::{debug, info, error as log_error};
use std::thread::available_parallelism;
use anyhow::Result;
use crate::config;

pub mod errors;
pub mod download;
use errors::ApplicationError::{self, DirectoryCreationError};

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

  setup_directories(&app)?;

  let size = get_size(cfg_from_file.download_directory).unwrap();
  
  app.set_used_storage(size); 

  Ok(app)
}

fn setup_directories(app: &App) -> Result<(), ApplicationError> {
  let cfg_clone = app.config.lock().unwrap().clone().unwrap();

  if !cfg_clone.download_directory.exists() {
    let cfg_binding = app.config.lock().unwrap();
    let dir = cfg_binding.as_ref().unwrap().download_directory.clone();

    debug!("Creating download directory: {:?}", &dir);
    let result = std::fs::create_dir_all(&dir);
    match result {
      Ok(_) => {},
      Err(e) => {
        log_error!("{}", e);
        return Err(DirectoryCreationError(dir));
      }
    }
  }

  if !cfg_clone.cache_directory.exists() {
    let cfg_binding = app.config.lock().unwrap();
    let dir = cfg_binding.as_ref().unwrap().cache_directory.clone();

    debug!("Creating cache directory: {:?}", &dir);
    let result = std::fs::create_dir_all(&dir);
    match result {
      Ok(_) => {},
      Err(e) => {
        log_error!("{}", e);
        return Err(DirectoryCreationError(dir));
      }
    }
  }

  if !cfg_clone.data_directory.exists() {
    let cfg_binding = app.config.lock().unwrap();
    let dir = cfg_binding.as_ref().unwrap().data_directory.clone();

    debug!("Creating data directory: {:?}", &dir);
    let result = std::fs::create_dir_all(&dir);
    match result {
      Ok(_) => {},
      Err(e) => {
        log_error!("{}", e);
        return Err(DirectoryCreationError(dir));
      }
    }
  }

  Ok(())
}

// fn rebuild_local_data() {
  // delete data directory
  // create data directory

  // fetch profiles
    // fetch buckets
    // save buckets to file for profile

  // ???? subdirectories ????
// }