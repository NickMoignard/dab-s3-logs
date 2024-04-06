use std::path::PathBuf;

use crate::config;
use anyhow::Result;
use bytesize::ByteSize;
use human_bytes::human_bytes;

/// Set download directory
pub fn set_download_directory (path: PathBuf) -> Result<()> {
  let mut cfg = config::get_config().unwrap();
  cfg.download_directory = path;

  config::update_config(cfg)?;

  Ok(())
}

/// Set max usable storage
pub fn set_max_storage (size: &str) -> Result<()> {
  let size = size.parse::<ByteSize>().unwrap();

  let mut cfg = config::get_config().unwrap();
  cfg.max_storage = size.as_u64();

  config::update_config(cfg)?;

  Ok(())
}

/// List all configurations
pub fn list () -> Result<()> {
  let cfg = config::get_config().unwrap();

  // download_thread_concurrency: DEFAULT_DOWNLOAD_THREAD_CONCURRENCY,
  eprintln!("Download Thread Concurrency: {}", cfg.download_thread_concurrency);
  // output_thread_concurrency: DEFAULT_OUTPUT_THREAD_CONCURRENCY,
  eprintln!("Output Thread Concurrency: {}", cfg.output_thread_concurrency);
  // max_storage: DEFAULT_MAX_STORAGE,
  eprintln!("Max Storage: {}", human_bytes(cfg.max_storage as f64));
  // aws_config_path,
  eprintln!("AWS Config Path: {:?}", cfg.aws_config_path);
  // download_directory: download_dir.to_str().unwrap().to_string(),
  eprintln!("Download Directory Path: {:?}", cfg.download_directory);
  // cache_directory: cache_dir.to_str().unwrap().to_string(),
  eprintln!("Cache Directory Path: {:?}", cfg.cache_directory);
  // home_directory: home_dir.to_str().unwrap().to_string(),
  eprintln!("Home Directory Path: {:?}", cfg.home_directory);
  
  Ok(())
}