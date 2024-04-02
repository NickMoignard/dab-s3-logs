#![allow(unused)]

use std::path::Path;
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc;
use file_format::{FileFormat, Kind};
use d3_tui::app::App;
use log::info;
use human_bytes::human_bytes;

use anyhow::Result as OtherResult;
use d3_tui::{app, s3, storage};
use d3_tui::storage::config::{load, StorageConfig};
use mime_guess::{mime, Mime};
use flate2::read::GzDecoder;
use std::io::Read;


use fs_extra::dir::get_size;

use serde_json::{Result, Value};

#[tokio::main]
async fn main() -> OtherResult<()> {
    let mut app = setup().unwrap();
    
    test_download(&app).await?;
    // test_file_type();

    // process(&app.clone()).await;

    exit();
    Ok(())
}

async fn process (app: &App) {
    let (tx, mut rx) = mpsc::channel::<String>(128);

    for index in 0..app.parallelism {
        let tx_clone = tx.clone();

        tokio::spawn(async move {
            // DO SOME WORK

            // PUBLISH THE WORK
            tx_clone.send(format!("Hi from thread: {}", index)).await.unwrap();
        });
    }

    drop(tx);
    while let Some(message) = rx.recv().await {
        // CONSUME THE WORK BACK ON THE MAIN THREAD
        println!("GOT = {}", message);
    }
}

fn test_file_type() -> OtherResult<()> {
    let path = Path::new("/Users/nlm/code/rust-test/d3-tui/temp/storage/staging/social/2024-04-01/1711945253-cd25a588-b105-4b9a-88f1-9de332db00f7.log.gz");

    let fmt = FileFormat::from_file(path).unwrap();

    match fmt {
        FileFormat::Gzip => {
            let bytes = std::fs::read(path).unwrap();
            let mut d = GzDecoder::new(&bytes[..]);
            let mut s = String::new();

            d.read_to_string(&mut s).unwrap();

            let v: Value = serde_json::from_str(&s)?;

            if v.is_array() {
                v.as_array().unwrap().iter().for_each(|x| {
                    // individual log line
                    println!("\n{}", x);
                });
            }
        },
        _ => {
            info!("Not a Gzip file. Media type is: {:?}", fmt.media_type());
        }
    }

    Ok(())
}

async fn test_download(app: &App) -> OtherResult<()> {
    const BUCKET: &str = "dabble-staging-kube-logs";

    let client = s3::get_aws_client().await?;

    let buckets = s3::get_buckets(&client).await?;

    let query = s3::list_keys(&client, BUCKET, "staging/social/2024-03-20").await?;

    log_query(&query);
    let mut buckets_binding = app.buckets.lock().unwrap();
    let mut queries_binding = app.queries.lock().unwrap();
    let mut storage_binding = app.storage_config.lock().unwrap();
    
    buckets_binding.extend(buckets);
    
    
    let path = Path::new(storage_binding.as_ref().unwrap().download_directory.as_str());
    
    let (tx, mut rx) = mpsc::channel::<String>(128);
    
    let num_downloads = query.objects.len();
    for index in 0..num_downloads {
        let tx_clone = tx.clone();
        let client_clone = client.clone();
        let path_clone = path.to_path_buf();
        let object = query.objects.values().nth(index).unwrap().clone();

        tokio::spawn(async move {
            let key_temp = object.key.unwrap().clone();

            s3::download_file(&client_clone, BUCKET, key_temp.as_str(), &path_clone).await.unwrap();

            tx_clone.send(format!("Downloaded file")).await.unwrap();
        });
    }

    queries_binding.push(query);
    drop(queries_binding);
    drop(tx);
    while let Some(message) = rx.recv().await {
        // CONSUME THE WORK BACK ON THE MAIN THREAD
        println!("GOT = {}", message);
    }

    Ok(())
}

fn log_query(query: &s3::Query) {
    info!("Bucket: {}", query.bucket);
    info!("Prefix: {}", query.prefix);
    info!("Size: {}", human_bytes(query.size as f64));
    info!("Objects: {}", query.objects.len());
}

fn setup() -> OtherResult<d3_tui::app::App> {
    env_logger::init();
    info!("Setting up");

    let mut app = d3_tui::app::App::new();
    {
        let mut storage_config_binding = app.storage_config.lock().unwrap();
        let config = load();
        storage_config_binding.replace(config.unwrap());

        setup_storage_dir(storage_config_binding.as_ref().unwrap());
    }

    let download_dir = {
        let storage_config = app.storage_config.lock().unwrap();
        storage_config.as_ref().unwrap().download_directory.as_str().to_string()
    };
    let storage_dir_clone = download_dir.clone();
    let size = get_size(download_dir).unwrap();
    
    app.set_used_storage(size); 

    Ok(app)
}

fn setup_storage_dir (config: &StorageConfig) -> std::io::Result<()> {
    if Path::new(config.download_directory.as_str()).exists() {
        return Ok(())
    }

    std::fs::create_dir_all(config.download_directory.as_str())
}

fn exit() {
    info!("Exiting");
}