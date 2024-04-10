use anyhow::Result;
use aws::profiles::get_aws_profiles::get_aws_profiles;
use dialoguer::FuzzySelect;

use crate::config::{update_config, get_config};

// TODO error handling
pub fn select_aws_profile() -> Result<()> {
  let profiles = get_aws_profiles().unwrap();
  let mut conf = get_config().unwrap();


  let selection_index = FuzzySelect::new()
    .with_prompt("Select an AWS profile from `~/.aws/config` to use as the default.")
    .items(&profiles).interact().unwrap();

  let selected_profile = profiles[selection_index].clone();

  conf.aws_profile = Some(selected_profile.clone());
  update_config(conf).unwrap();

  eprint!("Selected profile: {}", &selected_profile);
  
  Ok(())
}
