use futures_util::StreamExt;
use reqwest::{Client, Response};
use serde_json::Value;
use std::error::Error;
use std::path::{Path, PathBuf};
use std::time::Duration;
use tokio::io::AsyncWriteExt;
use tokio::time::sleep;

const OUTPUT_DIR: &str = "downloads";

pub async fn process_page(
    response: Response,
    client: &Client,
    delay: Duration,
) -> Result<(), Box<dyn Error>> {
    let items: Value = response.json().await?;
    let items = items.as_array().ok_or("Expected JSON array")?;

    for (idx, item) in items.iter().enumerate() {
        let Some(source_url) = item.get("source_url").and_then(|v| v.as_str()) else {
            continue;
        };

        let filename = source_url.rsplit('/').next().ok_or("Invalid source_url")?;

        let target = Path::new(OUTPUT_DIR).join(filename);

        if target.exists() {
            println!("✔ Skipping existing: {}", filename);
        } else {
            println!("⬇ Downloading: {}", filename);
            download(client, source_url, &target).await?;
        }

        if idx + 1 < items.len() {
            sleep(delay).await;
        }
    }

    Ok(())
}

async fn download(client: &Client, url: &str, target: &PathBuf) -> Result<(), Box<dyn Error>> {
    let response = client.get(url).send().await?.error_for_status()?;

    let mut file = tokio::fs::File::create(target).await?;
    let mut stream = response.bytes_stream();

    while let Some(chunk) = stream.next().await {
        file.write_all(&chunk?).await?;
    }

    Ok(())
}
