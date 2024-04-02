use d3_tui::output::output_files;
use d3_tui::app::App;
use log::info;

use anyhow::Result as OtherResult;
use d3_tui::{app::{self, download}, s3};

#[tokio::main]
async fn main() -> OtherResult<()> {
    let app = app::setup().unwrap();
    
    test_download(&app).await?;

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