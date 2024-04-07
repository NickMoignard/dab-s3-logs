use std::{path::PathBuf, rc::Rc};

use log::info;
use clap::{arg, command, Args, Parser, Subcommand};
use aws_sdk_s3::Client;
use anyhow::{Error, Result as OtherResult};
use dab_s3_logs::{app, aws::{s3, client, profiles}, commands};

#[tokio::main]
async fn main() -> OtherResult<()> {
    let app = app::setup().unwrap();
    let args = CliArgs::parse();

    let client = client::get_aws_client(args.profile, &app).await.unwrap();
  
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
                    let result = commands::config::set_max_storage(size.as_str());
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
                ConfigCommands::ListAwsProfiles => {
                    let result = profiles::get_aws_profiles(&app);
                    match result {
                        Ok(profiles) => {
                            for profile in profiles {
                                println!("{}", profile);
                            }
                        }
                        Err(e) => {
                            eprintln!("Failed to list AWS profiles: {:?}", e);
                        }
                    }
                }
                ConfigCommands::SelectAwsProfile => {
                    let result = profiles::select_aws_profile(&app);
                    match result {
                        Ok(_) => {}
                        Err(e) => {
                            eprintln!("Failed to select AWS profile: {:?}", e);
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
        Commands::Test => {
            test_buckets_code(&app, &client).await.unwrap();
        }
    }

    exit();
    Ok(())
}

fn exit() {
    info!("Exiting");
}

async fn test_buckets_code(app: &app::App, client: &Client) -> Result<(), Error> {
    let cfg = Rc::new(app.config.lock().unwrap().clone().unwrap());
    let profile = cfg.aws_profile.clone().unwrap();
    println!("Profile: {:?}", profile);

    let buckets = s3::buckets::get_buckets(client).await;
    match buckets {
        Ok(buckets) => {
            for bucket in &buckets {
                println!("Fetched Bucket {:?}", bucket);
            }
        
            let result = s3::buckets::save_buckets_to_file(&buckets, &app);
            match result {
                Ok(_) => {
                    let buckets_from_file = s3::buckets::get_buckets_from_file(&profile, app).unwrap();
        
                    for bucket in buckets_from_file {
                        println!("Bucket from file {:?}", bucket);
                    }
                }
                Err(e) => {
                    eprintln!("Failed to save buckets: {:?}", e);
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to fetch buckets: {:?}", e);
        }
    }

    
    Ok(())
}

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(name = "dab-s3-logs")]
#[command(about = "Tool to fetch and output logs from S3 buckets", long_about = None)]
#[command(version, version = "0.1.0")]
pub struct CliArgs {
  /// The subcommand to run
  #[command(subcommand)]
  cmd: Commands,
  /// AWS Profile to use when initializing the S3 client
  #[arg(long)]
  profile: Option<String>,
}

#[derive(Subcommand, Debug, Clone)]
enum Commands {
    /// Command to run whatever test fn is currently in main
    Test,
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
        /// Maximum storage size
        #[arg(short, long)]
        size: String,
    },
    /// List configuration values
    List,
    /// List AWS Profiles
    ListAwsProfiles,
    SelectAwsProfile,
}