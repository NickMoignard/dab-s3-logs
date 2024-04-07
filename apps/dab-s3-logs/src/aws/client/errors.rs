use thiserror::Error;

#[derive(Error, Debug)]
pub enum ClientError {
  #[error("Failed to get client")]
  GetClientError,
}