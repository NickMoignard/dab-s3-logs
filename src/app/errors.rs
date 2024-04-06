use std::path::PathBuf;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApplicationError {
  #[error("Failed to create directory: `{0}`")]
  DirectoryCreationError(PathBuf),
}