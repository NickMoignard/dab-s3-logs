use log::debug;
use serde_derive::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Debug)]
pub struct StorageConfig {
  pub max_storage: u64,
  pub download_directory: String,
}

impl ::std::default::Default for StorageConfig {
  fn default() -> Self { 
    Self { 
      max_storage: 1_073_741_824, // 1GiB
      download_directory: "./temp/storage".to_string(),
    }
  }
}

pub fn load() -> Result<StorageConfig, confy::ConfyError> {
  let config = confy::load::<StorageConfig>("d3", "storage")?;

  let file_path = confy::get_configuration_file_path("storage", "storage");

  debug!("Loaded config: {:?} from {:?}", config, file_path?.as_path());

  Ok(config)
}