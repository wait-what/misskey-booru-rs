use super::misskey::PostVisibility;
use nanoserde::TomlParser;
use std::fs;

#[derive(Debug)]
pub struct Config {
    pub token: String,
    pub instance_url: String,
    pub message: String,
    pub append_post_url: bool,
    pub post_interval: f64,
    pub visibility: PostVisibility,
    pub booru_url: String,
    pub tags: Vec<String>,
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
            post_interval: toml.get("post.interval").unwrap().num(),
            visibility: toml.get("post.visibility").unwrap().str().into(),
            booru_url: toml.get("gelbooru.booru_url").unwrap().str().to_string(),
            #[rustfmt::skip]
            tags: toml.get("gelbooru.tags").unwrap().simple_arr().iter().map(|s| s.str().to_string()).collect(),
        };

        Ok(config)
    }
}
