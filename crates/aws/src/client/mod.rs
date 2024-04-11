use aws_config::meta::region::RegionProviderChain;
use aws_sdk_s3::Client;

pub mod errors;

// TODO: move to application config
const REGION: &str = "ap-southeast-2";

pub async fn get_aws_client(profile: Option<String>) -> Result<Client, errors::ClientError> {
  let region_provider = RegionProviderChain::default_provider().or_else(REGION);
  let mut config_builder = aws_config::from_env().region(region_provider);

  if profile.is_some() {
    config_builder = config_builder.profile_name(profile.unwrap());
  }

  let config = config_builder.load().await;
  let client = Client::new(&config);

  Ok(client)
}
