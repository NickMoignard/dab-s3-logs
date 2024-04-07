use std::{fs::read_to_string, path::PathBuf};

use dialoguer::FuzzySelect;
use regex::Regex;
use anyhow::Result;

use crate::{app::App, config::update_config};


pub mod errors;

// TODO replace this regex with TOML / Serde parse solution

// TODO error handling
pub fn get_aws_profiles(app: &App) -> Result<Vec<String>, errors::ProfilesError> {
  let aws_config_path = {
    let cfg = app.config.lock().unwrap().clone().unwrap();
    cfg.aws_config_path
  };

  let mut results = vec![];
  // Create a regex to match text inside square brackets
  let re = Regex::new(r"\[(.*?)\]").unwrap();

  // Iterate over each line in the file
  let lines = read_lines(aws_config_path);
  for line in lines {
    let profile_name = re.find(&line);
    match profile_name {
      Some(profile) => {
        let profile = profile.as_str().to_string().clone();
        results.push(profile);
      }
      None => {}
    }
  }

    
  Ok(parse_profiles(results).unwrap())
}

// TODO error handling
pub fn select_aws_profile(app: &App) -> Result<(), errors::ProfilesError> {
  let profiles = get_aws_profiles(app).unwrap();
  let mut cfg_clone = app.config.lock().unwrap().clone().unwrap();


  let selection_index = FuzzySelect::new()
    .with_prompt("Select an AWS profile from `~/.aws/config` to use as the default.")
    .items(&profiles).interact().unwrap();

  let selected_profile = profiles[selection_index].clone();

  cfg_clone.aws_profile = Some(selected_profile.clone());
  update_config(cfg_clone).unwrap();

  eprint!("Selected profile: {}", &selected_profile);
  
  Ok(())
}


// TODO error handling
fn parse_profiles(regex_matches: Vec<String>) -> Result<Vec<String>, errors::ProfilesError> {
  let profiles = regex_matches.iter().map(|regex_match| {
    regex_match.clone().replace("[", "").replace("]", "").replace("profile", "").trim().to_string()
  }).collect::<Vec<String>>();

  Ok(profiles)
}


fn read_lines(filename: PathBuf) -> Vec<String> {
  read_to_string(filename) 
      .unwrap()  // panic on possible file-reading errors
      .lines()  // split the string into an iterator of string slices
      .map(String::from)  // make each slice into a string
      .collect()  // gather them together into a vector
}
