use crate::app::App;
use anyhow::Result;

/// Set download directory
pub fn set_download_directory (path: String, app: &App) -> Result<()> {
  let mut cfg = app.storage_config.lock().unwrap().clone().unwrap();
  cfg.download_directory = path;

  confy::store("d3", "storage", cfg)?;

  Ok(())
}

/// Set max usable storage
pub fn set_max_storage (size: u64, app: &App) -> Result<()> {
  let mut cfg = app.storage_config.lock().unwrap().clone().unwrap();
  cfg.max_storage = size;

  confy::store("d3", "storage", cfg)?;

  Ok(())
}

/// List all configurations
pub fn list (app: &App) -> Result<()> {
  let cfg = app.storage_config.lock().unwrap().clone().unwrap();
  println!("{:?}", cfg);
  
  Ok(())
}