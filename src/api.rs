use reqwest::{Client, Response};
use std::error::Error;

const PER_PAGE: u32 = 100;

pub async fn fetch_page(
    client: &Client,
    base_url: &str,
    page: u32,
) -> Result<Response, Box<dyn Error>> {
    let url = format!("{}?per_page={}&page={}", base_url, PER_PAGE, page);

    println!("⬇ Fetching page {}", page);

    Ok(client.get(url).send().await?.error_for_status()?)
}

pub fn extract_total_pages(response: &Response) -> Result<u32, Box<dyn Error>> {
    let header = response
        .headers()
        .get("X-WP-TotalPages")
        .ok_or("Missing X-WP-TotalPages header")?
        .to_str()?;

    Ok(header.parse()?)
}
