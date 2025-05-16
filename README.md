# misskey-booru-rs
Bot for Misskey that posts an image from Gelbooru every n seconds. Can run in Docker.

## Running
1. Clone the repository
1. Copy `config_sample.toml` to `config.toml`
1. Edit `config.toml`
1. Run one of:
    - `cargo run --release`
    - `cargo install --path .` then `misskey-booru-rs` (assuming `~/.cargo/bin` is in your PATH)
    - `docker-compose up`
    - `docker build . -t misskey-booru-rs` then `docker run -v ./config.toml:/config.toml:ro misskey-booru-rs`

## Notes
- A or B tags are specified like this:
```toml
tags = [ "{1girl", "~", "2girls}", "rating:general" ]
```
- Other sites that run Gelbooru may work, but are untested
- Restarting the bot resets the cooldown
- The bot might post the same image multiple times in a row, especially if the amount available is low
- Don't set `bot.error_timeout` too low, or you might get rate limited

## License
This project is licensed under the [AGPL-3.0 license](./LICENSE).
