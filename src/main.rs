use reqwest::Client;
use std::env;
use std::error::Error;
use std::fs;
use std::time::Duration;

mod api;
mod downloader;
mod utils;

use api::{extract_total_pages, fetch_page};
use downloader::process_page;
use utils::resolve_media_url;

const OUTPUT_DIR: &str = "downloads";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut args = env::args().skip(1);

    let input_url = args
        .next()
        .ok_or("Usage: wp_media_downloader <site-or-media-url> <delay-ms>")?;

    let delay_ms: u64 = args
        .next()
        .ok_or("Missing <delay-ms> argument")?
        .parse()
        .map_err(|_| "delay-ms must be a positive integer")?;

    let delay = Duration::from_millis(delay_ms);
    let base_media_url = resolve_media_url(&input_url);

    println!("🔗 Media endpoint:");
    println!("   {}", base_media_url);

    fs::create_dir_all(OUTPUT_DIR)?;

    let client = Client::builder()
        .redirect(reqwest::redirect::Policy::limited(10))
        .build()?;

    // --- First page (discover total pages)
    let first_response = fetch_page(&client, &base_media_url, 1).await?;
    let total_pages = extract_total_pages(&first_response)?;

    println!("📄 Total pages: {}", total_pages);

    // --- Process page 1
    process_page(first_response, &client, delay).await?;

    // --- Remaining pages
    for page in 2..=total_pages {
        let response = fetch_page(&client, &base_media_url, page).await?;
        process_page(response, &client, delay).await?;
    }

    println!("\n✅ Done.");
    Ok(())
}
