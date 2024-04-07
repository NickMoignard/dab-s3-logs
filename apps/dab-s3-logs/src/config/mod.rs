use std::path::PathBuf;

use log::debug;
use serde_derive::{Deserialize, Serialize};
use bytesize::ByteSize;
use dirs;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ApplicationConfig {
  pub aws_profile: Option<String>,
  pub download_thread_concurrency: usize,
  pub output_thread_concurrency: usize,
  pub download_directory: PathBuf,
  pub aws_config_path: PathBuf,
  pub cache_directory: PathBuf,
  pub data_directory: PathBuf,
  pub home_directory: PathBuf,
  pub max_storage: u64,
}

pub const APPLICATION_NAME: &str = "dab-s3-logs"; /// "dab-s3-logs"
pub const APPLICATION_CONFIG_NAME: &str = "application"; /// "application"
const DEFAULT_AWS_CONFIG_PATH_SUFFIX: &str = ".aws/config"; /// .aws/config
const DEFAULT_DOWNLOAD_THREAD_CONCURRENCY: usize = 100; /// 100 tokio async threads
const DEFAULT_OUTPUT_THREAD_CONCURRENCY: usize = 10; /// 10 tokio async threads
const DEFAULT_MAX_STORAGE: u64 = ByteSize::gb(20).as_u64(); /// 20Gb


impl ::std::default::Default for ApplicationConfig {
  fn default() -> Self { 
    let download_directory = dirs::download_dir().unwrap().join(APPLICATION_NAME);
    let cache_directory = dirs::cache_dir().unwrap().join(APPLICATION_NAME);
    let data_directory = dirs::data_dir().unwrap().join(APPLICATION_NAME);
    let aws_config_path = dirs::home_dir().unwrap().join(DEFAULT_AWS_CONFIG_PATH_SUFFIX);

    Self {
      aws_profile: None,
      aws_config_path,
      download_directory,
      cache_directory,
      data_directory,
      download_thread_concurrency: DEFAULT_DOWNLOAD_THREAD_CONCURRENCY,
      output_thread_concurrency: DEFAULT_OUTPUT_THREAD_CONCURRENCY,
      max_storage: DEFAULT_MAX_STORAGE,
      home_directory: dirs::home_dir().unwrap(),
    }
  }
}

pub fn get_config() -> Result<ApplicationConfig, confy::ConfyError> {
  let config = confy::load::<ApplicationConfig>(APPLICATION_NAME, APPLICATION_CONFIG_NAME);

  let file_path = confy::get_configuration_file_path(APPLICATION_NAME, APPLICATION_CONFIG_NAME);
  debug!("Loaded config: {:?} from {:?}", config, file_path);

  return config
}

pub fn update_config(cfg: ApplicationConfig) -> Result<(), confy::ConfyError>{
  confy::store(APPLICATION_NAME, APPLICATION_CONFIG_NAME, cfg)
}
