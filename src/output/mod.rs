pub mod stdout;

use std::path::Path;

use anyhow::Result;
use log::debug;
use tokio::sync::mpsc;

// TODO: Add to application configuration
const OUTPUT_PARALLELISM: usize = 10;

pub async fn output_files (files: Vec<String>) -> Result<()> {
  let num_files = files.len();
  let files_per_thread = num_files / OUTPUT_PARALLELISM;
  let nested_files = files.chunks(files_per_thread).map(|x| x.to_vec()).collect::<Vec<Vec<String>>>();

  let (tx, mut rx) = mpsc::channel::<String>(128);
  for slice in nested_files {
      let tx_clone = tx.clone();
      tokio::spawn(async move {
          for file in slice {
              stdout::output_logfile(Path::new(file.as_str()));
              tx_clone.send(file).await.unwrap();
          }
      });
  }

  drop(tx);
  while let Some(message) = rx.recv().await {
      debug!("processed file: {}", message);
  }

  Ok(())
}