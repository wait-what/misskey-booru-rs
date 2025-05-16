# misskey-booru-rs
Bot for Misskey that posts an image from Gelbooru every n seconds. Supports running in Docker.

## Running
1. Clone the repository
1. Copy `config_sample.toml` to `config.toml`
1. Edit `config.toml`
1. Run one of:
    - `cargo run --release`
    - `cargo install --path .` then `misskey-booru-rs` (assuming `~/.cargo/bin` is in your PATH)
    - `docker-compose up`
    - `docker build . -t misskey-booru-rs` then `docker run -v ./config.toml:/config.toml:ro misskey-booru-rs`

## License
[AGPL-3.0](./LICENSE)
