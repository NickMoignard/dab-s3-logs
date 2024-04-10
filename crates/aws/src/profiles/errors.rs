use thiserror::Error;
use crate::config;

#[derive(Error, Debug)]
pub enum GetProfilesError {
  #[error("Config file parse error: {0}")]
  FileError(#[from] ini::Error),
  #[error("Home directory not found")]
  HomeDirNotFound(#[from] config::GetAwsConfigFilePathError),
}