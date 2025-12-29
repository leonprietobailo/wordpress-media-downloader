pub fn resolve_media_url(input: &str) -> String {
    let trimmed = input.trim_end_matches('/');

    if trimmed.contains("/wp-json/wp/v2/media") {
        trimmed.to_string()
    } else {
        format!("{}/wp-json/wp/v2/media", trimmed)
    }
}
