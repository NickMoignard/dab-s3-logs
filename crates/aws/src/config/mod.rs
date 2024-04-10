use std::path::PathBuf;

use thiserror::Error;

const AWS_CONFIG_FILE: &str = ".aws/config";

pub fn get_config_file_path () -> Result<PathBuf, GetAwsConfigFilePathError> {
  let home_dir = {
    let option = dirs::home_dir();
    match option {
      Some(path) => path,
      None => return Err(GetAwsConfigFilePathError::HomeDirNotFound)
    }
  };
  let path = home_dir.join(AWS_CONFIG_FILE);

  Ok(path)
}

#[derive(Error, Debug)]
pub enum GetAwsConfigFilePathError {
  #[error("Home directory not found")]
  HomeDirNotFound,
}