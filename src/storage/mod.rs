use anyhow::Result;
use fs_extra::dir::get_size;
use walkdir::{DirEntry, WalkDir};

use crate::app::App;

pub mod config;

pub fn get_used_storage (app: &App) -> Result<u64> {
  let download_dir = {
    let storage_config = app.storage_config.lock().unwrap();
    storage_config.as_ref().unwrap().download_directory.as_str().to_string()
  };
  let size = get_size(download_dir).unwrap();

  Ok(size)
}

pub fn get_all_files (app: &App) -> Result<Vec<String>> {
  let download_dir = {
    let storage_config = app.storage_config.lock().unwrap();
    storage_config.as_ref().unwrap().download_directory.as_str().to_string()
  };

  let mut files = Vec::new();
  let walker = WalkDir::new(download_dir).into_iter();
  for entry in walker.filter_entry(|e| !is_hidden(e) || !e.file_type().is_dir()) {
      files.push(entry?.path().to_str().unwrap().to_string());
  }

  Ok(files)
}

fn is_hidden(entry: &DirEntry) -> bool {
  entry.file_name()
    .to_str()
    .map(|s| s.starts_with("."))
    .unwrap_or(false)
}