# WP Media Downloader

Fast, single-binary utility to download all media items from a WordPress site via the public REST API (`/wp-json/wp/v2/media`). It paginates through the media collection, streams files to a local `downloads/` directory, skips existing files, and can throttle requests with a per-item delay.

## How it works

- Normalizes the input URL to `https://<site>/wp-json/wp/v2/media` if you pass a root site URL.
- Fetches page 1 to discover total pages from the `X-WP-TotalPages` header.
- Streams each media item from `source_url`, saving to `downloads/` (re-uses files if already present).
- Respects the provided delay between media downloads to be gentle on the origin.

## Requirements

- Rust (stable) with `cargo`.

## Build

```bash
cargo build --release
# binary: target/release/wp-media-downloader
```

Prebuilt binaries for Linux, macOS (x86_64/aarch64), and Windows are published with each GitHub release.

## Usage

```bash
wp-media-downloader <site-or-media-url> <delay-ms>
```

- `<site-or-media-url>`: Either the root site (e.g., `https://example.com`) or the media endpoint directly (e.g., `https://example.com/wp-json/wp/v2/media`).
- `<delay-ms>`: Milliseconds to wait between downloads (e.g., `250`).

### Example

```bash
# Download with a 250 ms pause between files
wp-media-downloader https://example.com 250
```

Downloads land in `./downloads/`. Existing files are skipped.

## Notes

- WordPress must expose the REST API endpoint publicly.
- Pagination uses `per_page=100`; adjust in `src/api.rs` if needed.
- The downloader follows up to 10 redirects.

## Contributing

Issues and PRs are welcome. Please run `cargo fmt` and `cargo clippy` before submitting.
