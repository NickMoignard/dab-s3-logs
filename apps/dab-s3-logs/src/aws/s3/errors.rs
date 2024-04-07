use thiserror::Error;
use aws_sdk_s3::{self, error::SdkError, operation::list_buckets::ListBucketsError};
use std::io::Error as IoError;

#[derive(Error, Debug)]
pub enum BucketsError {
  #[error("Failed to save buckets to file")]
  SaveBucketsError(IoError),
  #[error("Failed to create buckets data directory")]
  DirectoryCreationError(IoError),
  #[error("Failed to get buckets")]
  ListBucketsError(SdkError<ListBucketsError>)
}
