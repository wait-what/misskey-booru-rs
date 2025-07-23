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
    pub api_key: Option<String>,
    pub user_id: Option<String>,
    pub tags: Vec<String>,
    pub range: u32,
    pub post_interval: f64,
    pub error_timeout: f64,
}

impl Config {
    pub fn new(path: &str) -> Result<Self, ()> {
        let content = fs::read_to_string(path).unwrap();
        let toml = TomlParser::parse(&content).unwrap();

        #[rustfmt::skip]
        let config = Self {
            token: toml.get("account.token").expect("config: account.token must be a string").str().to_string(),
            instance_url: toml.get("account.instance_url").expect("config: account.instance_url must be a string").str().to_string(),
            message: toml.get("post.message").expect("config: post.message must be a string").str().to_string(),
            append_post_url: toml.get("post.append_post_url").expect("config: post.append_post_url must be a boolean").boolean(),
            visibility: toml.get("post.visibility").expect("config: post.visibility must be a string").str().into(),
            sensitive: toml.get("post.sensitive").expect("config: post.sensitive must be a boolean").boolean(),
            booru_url: toml.get("gelbooru.booru_url").expect("config: gelbooru.booru_url must be a string").str().to_string(),
            api_key: toml.get("gelbooru.api_key").map(|v| v.str().to_string()),
            user_id: toml.get("gelbooru.user_id").map(|v| v.str().to_string()),
            tags: toml.get("gelbooru.tags").expect("config: gelbooru.tags must be an array of strings")
                .simple_arr().iter().map(|s| s.str().to_string()).collect(),
            range: {
                let range = toml.get("gelbooru.range").expect("config: gelbooru.range must be a number").num() as u32;
                if range > 20000 {
                    log::warn!("Range is over 20000, setting to 20000");
                    20000
                } else {
                    range
                }
            },
            post_interval: toml.get("bot.post_interval").expect("config: bot.post_interval must be a number").num(),
            error_timeout: toml.get("bot.error_timeout").expect("config: bot.error_timeout must be a number").num(),
        };

        Ok(config)
    }
}
