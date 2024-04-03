use log::info;
use clap::{arg, command, Args, Parser, Subcommand};

use anyhow::Result as OtherResult;
use dab_s3_logs::{app::{self, download, App}, s3, commands, output::output_files};

#[tokio::main]
async fn main() -> OtherResult<()> {
    let app = app::setup().unwrap();
    let client = s3::get_aws_client().await.unwrap();

    let args = CliArgs::parse();
  
    match args.cmd {
        Commands::Fetch { bucket, prefix } => {
            let _ = commands::fetch::fetch(&client, &app, bucket, prefix).await;
        }
        Commands::Preview { bucket, prefix } => {
            let _ = commands::fetch::preview(&client, bucket, prefix).await;
        }
        Commands::Config(config) => match config.cmd {
            Some(config) => match config {
                ConfigCommands::SetDownloadDir { path } => {
                    commands::config::set_download_directory(path);
                }
                ConfigCommands::SetMaxStorage { size } => {
                    commands::config::set_max_storage(size);
                }
                ConfigCommands::List => {
                    commands::config::list();
                }
            }
            None => {}
        }
        Commands::Reset => {
            commands::reset::reset();
        }
        _ => {}
    }
    
    // test_download(&app).await?;

    exit();
    Ok(())
}

async fn test_download(app: &App) -> OtherResult<()> {
    const BUCKET: &str = "dabble-staging-kube-logs";
    let client = s3::get_aws_client().await?;

    let query = s3::list_keys(&client, BUCKET, "staging/social/2024-03-21").await?;
    let files = download::download_query_results(&query, BUCKET.to_string(), app, &client).await?;
    let _ = output_files(files).await.unwrap();

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
        path: String,
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