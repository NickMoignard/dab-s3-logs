use std::fs;
use fs_extra::remove_items;
use anyhow::Result;

/// Empties storage directory
pub async fn delete_downloaded_logs () -> Result<()>{
  let mut from_paths = Vec::new();

  fs::read_dir("./temp/storage").unwrap().for_each(|entry| {
    let entry = entry.unwrap();
    let path = entry.path().to_str().unwrap().to_string();
    from_paths.push(path);
  });

  let result = remove_items(&from_paths).unwrap();

  Ok(result)
}