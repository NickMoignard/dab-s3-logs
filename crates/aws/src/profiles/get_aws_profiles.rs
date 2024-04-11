use std::path::PathBuf;

use ini::Ini;

use crate::profiles::errors;

use crate::config::get_config_file_path;

pub fn get_aws_profiles()  -> Result<Vec<String>, errors::GetProfilesError> {
  let aws_config_path = get_config_file_path()?;
  let conf = Ini::load_from_file::<PathBuf>(aws_config_path)?;
  let profiles = parse_profiles_from_ini(conf);

  Ok(profiles)
}

fn parse_profiles_from_ini(ini: Ini) -> Vec<String> {
  ini.iter()
    .filter_map(
      |(sec, _props)| {
        sec.map(|section| section.to_string()
            .replace("profile", "")
            .trim()
            .to_string())        
      }
    ).collect()
}
