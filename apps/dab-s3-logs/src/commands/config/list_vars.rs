use crate::config;
use anyhow::Result;
use human_bytes::human_bytes;

/// List all configurations
pub fn list_vars () -> Result<()> {
  let conf = config::get_config().unwrap();

  eprintln!("Download Thread Concurrency: {}", conf.download_thread_concurrency);
  eprintln!("Output Thread Concurrency: {}", conf.output_thread_concurrency);
  eprintln!("Max Storage: {}", human_bytes(conf.max_storage as f64));
  eprintln!("AWS Config Path: {:?}", conf.aws_config_path);
  eprintln!("Download Directory Path: {:?}", conf.download_directory);
  eprintln!("Cache Directory Path: {:?}", conf.cache_directory);
  eprintln!("Home Directory Path: {:?}", conf.home_directory);
  
  Ok(())
}