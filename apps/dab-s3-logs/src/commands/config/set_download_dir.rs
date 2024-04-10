use std::path::PathBuf;
use anyhow::Result;
use crate::config;

/// Set download directory
pub fn set_download_directory (path: PathBuf) -> Result<()> {
  let mut cfg = config::get_config().unwrap();
  cfg.download_directory = path;

  config::update_config(cfg)?;

  Ok(())
}