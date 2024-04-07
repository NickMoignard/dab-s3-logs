use thiserror::Error;

#[derive(Error, Debug)]
pub enum ProfilesError {
  #[error("Failed to get profiles")]
  GetProfilesError,
}