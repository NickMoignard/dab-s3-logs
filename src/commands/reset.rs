use std::fs;
use fs_extra::remove_items;

/// Empties storage directory
pub fn reset () {
  let mut from_paths = Vec::new();

  fs::read_dir("./temp/storage").unwrap().for_each(|entry| {
    let entry = entry.unwrap();
    let path = entry.path().to_str().unwrap().to_string();
    from_paths.push(path);
  });

  remove_items(&from_paths).unwrap();
}