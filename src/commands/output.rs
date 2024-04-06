

use std::path::Path;

use anyhow::Result;
use log::debug;
use tokio::sync::mpsc;

use crate::{app::App, output::stdout, storage::get_all_files};

pub async fn output_files (app: &App) -> Result<()> {
  let cfg = app.config.lock().unwrap().clone();
  let files = get_all_files(app).unwrap();
  let num_files = files.len();
  let files_per_thread = num_files / cfg.unwrap().output_thread_concurrency;

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