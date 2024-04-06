use std::path::PathBuf;

use log::info;
use clap::{arg, command, Args, Parser, Subcommand};

use anyhow::Result as OtherResult;
use dab_s3_logs::{app, s3, commands};

#[tokio::main]
async fn main() -> OtherResult<()> {
    let app = app::setup().unwrap();
    let client = s3::get_aws_client().await.unwrap();
    let args = CliArgs::parse();
  
    match args.cmd {
        Commands::Fetch { bucket, prefix } => {
            let result = commands::fetch::fetch(&client, &app, bucket, prefix).await;
            match result {
                Ok(_) => {}
                Err(e) => {
                    eprintln!("Failed to fetch logs: {:?}", e);
                }
            }
        }
        Commands::Preview { bucket, prefix } => {
            let result = commands::fetch::preview(&client, bucket, prefix).await;
            match result {
                Ok(_) => {}
                Err(e) => {
                    eprintln!("Failed to preview logs: {:?}", e);
                }
            }
        }
        Commands::Config(config) => match config.cmd {
            Some(config) => match config {
                ConfigCommands::SetDownloadDir { path } => {
                    let result = commands::config::set_download_directory(path);
                    match result {
                        Ok(_) => {}
                        Err(e) => {
                            eprintln!("Failed to set download directory: {:?}", e);
                        }
                    }
                }
                ConfigCommands::SetMaxStorage { size } => {
                    let result = commands::config::set_max_storage(size);
                    match result {
                        Ok(_) => {}
                        Err(e) => {
                            eprintln!("Failed to set max storage: {:?}", e);
                        }
                    }
                }
                ConfigCommands::List => {
                    let result = commands::config::list();
                    match result {
                        Ok(_) => {}
                        Err(e) => {
                            eprintln!("Failed to list configuration: {:?}", e);
                        }
                    }
                }
            }
            None => {}
        }
        Commands::Output => {
            commands::output::output_files(&app).await?;
        }
        Commands::Reset => {
            commands::reset::delete_downloaded_logs().await?;
        }
    }

    exit();
    Ok(())
}

fn exit() {
    info!("Exiting");
}

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(name = "dab-s3-logs")]
#[command(about = "Tool to fetch and output logs from S3 buckets", long_about = None)]
#[command(version, version = "0.1.0")]
pub struct CliArgs {
  #[command(subcommand)]
  cmd: Commands
}

#[derive(Subcommand, Debug, Clone)]
enum Commands {
    /// Preview fetch results
    #[command(arg_required_else_help = true)]
    Preview {
        /// Name of the bucket to pull logs from
        #[arg(short, long)]
        bucket: String,
    
        /// Prefix to search for logs
        #[arg(short, long)]
        prefix: String,
    },
    /// Fetch logs from S3
    #[command(arg_required_else_help = true)]
    Fetch {
        /// Name of the bucket to pull logs from
        #[arg(short, long)]
        bucket: String,
    
        /// Prefix to search for logs
        #[arg(short, long)]
        prefix: String,
    },
    /// Output downloaded logs to stdout
    Output,
    /// Manage configuration options
    Config(ConfigArgs),
    /// Clear storage directory
    Reset,
}
#[derive(Debug, Args, Clone)]
#[command(args_conflicts_with_subcommands = true)]
#[command(flatten_help = true)]
struct ConfigArgs {
    #[command(subcommand)]
    cmd: Option<ConfigCommands>,
}

#[derive(Debug, Subcommand, Clone)]
enum ConfigCommands {
    /// Set the download directory
    #[command(arg_required_else_help = true)]
    SetDownloadDir {
        /// Path to the download directory
        #[arg(short, long)]
        path: PathBuf,
    },
    /// Set the max storage size
    #[command(arg_required_else_help = true)]
    SetMaxStorage {
        /// Maximum storage size in bytes
        #[arg(short, long)]
        size: u64,
    },
    /// List configuration values
    List,
}