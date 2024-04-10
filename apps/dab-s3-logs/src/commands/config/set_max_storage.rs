use crate::config;
use anyhow::Result;
use bytesize::ByteSize;

/// Set max usable storage
pub fn set_max_storage (size: &str) -> Result<()> {
  let size = size.parse::<ByteSize>().unwrap();

  let mut cfg = config::get_config().unwrap();
  cfg.max_storage = size.as_u64();

  config::update_config(cfg)?;

  Ok(())
}