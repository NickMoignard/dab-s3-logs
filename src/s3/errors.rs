use thiserror::Error;

#[derive(Error, Debug)]
pub enum S3Error {
  
}

#[derive(Error, Debug)]
pub enum AwsError {
  #[error("Failed to get profiles")]
  GetProfilesError,
}