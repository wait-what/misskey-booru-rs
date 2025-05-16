use super::misskey::PostVisibility;
use nanoserde::TomlParser;
use std::fs;

#[derive(Debug)]
pub struct Config {
    pub token: String,
    pub instance_url: String,
    pub message: String,
    pub append_post_url: bool,
    pub visibility: PostVisibility,
    pub sensitive: bool,
    pub booru_url: String,
    pub tags: Vec<String>,
    pub range: u32,
    pub post_interval: f64,
    pub error_timeout: f64,
}

impl Config {
    pub fn new(path: &str) -> Result<Self, ()> {
        let content = fs::read_to_string(path).unwrap();
        let toml = TomlParser::parse(&content).unwrap();

        let config = Self {
            token: toml.get("account.token").unwrap().str().to_string(),
            instance_url: toml.get("account.instance_url").unwrap().str().to_string(),
            message: toml.get("post.message").unwrap().str().to_string(),
            append_post_url: toml.get("post.append_post_url").unwrap().boolean(),
            visibility: toml.get("post.visibility").unwrap().str().into(),
            sensitive: toml.get("post.sensitive").unwrap().boolean(),
            booru_url: toml.get("gelbooru.booru_url").unwrap().str().to_string(),
            #[rustfmt::skip]
            tags: toml.get("gelbooru.tags").unwrap().simple_arr().iter().map(|s| s.str().to_string()).collect(),
            range: {
                let range = toml.get("gelbooru.range").unwrap().num() as u32;
                if range > 20000 {
                    log::warn!("Range is over 20000, setting to 20000");
                    20000
                } else {
                    range
                }
            },
            post_interval: toml.get("bot.post_interval").unwrap().num(),
            error_timeout: toml.get("bot.error_timeout").unwrap().num(),
        };

        Ok(config)
    }
}
