use std::sync::{Arc, Mutex};
use aws_sdk_s3::types::Bucket;
use fs_extra::dir::get_size;
use std::thread::available_parallelism;

use crate::{s3::Query, storage::config::StorageConfig};

#[derive(Debug)]
pub struct App {
  pub queries: Arc<Mutex<Vec<Query>>>,
  pub buckets: Arc<Mutex<Vec<Bucket>>>,
  pub storage_config: Mutex<Option<StorageConfig>>,
  pub used_storage:u64,
  pub parallelism: usize,
}

impl Default for App {
  fn default() -> Self {
    Self {
      queries: Arc::new(Mutex::new(Vec::<Query>::new())),
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
