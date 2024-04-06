use thiserror::Error;

#[derive(Error, Debug)]
pub enum FetchError {
  #[error("Not enough storage space available")]
  NotEnoughStorage,
  #[error("Failed to delete logs")]
  LogDeletionFailed,
  #[error("Failed to download logs")]
  DownloadFailed,
}

#[derive(Error, Debug)]
pub enum PreviewError {
  #[error("Failed to preview logs")]
  PreviewFailed,
}