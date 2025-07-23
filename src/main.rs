use std::thread;
use std::time::Duration;

mod gelbooru;
use gelbooru::{GelbooruPost, GelbooruConfig};

mod misskey;
use misskey::MisskeyClient;

mod config;
use config::Config;

fn main() {
    pretty_env_logger::formatted_builder()
        .filter(None, log::LevelFilter::Info)
        .init();

    let config = Config::new("config.toml").unwrap();
    let client = MisskeyClient::new(&config.token, &config.instance_url);

    let gelbooru_config = GelbooruConfig::new(
        config.booru_url,
        config.api_key,
        config.user_id,
    );

    'post: loop {
        log::info!("Searching for a random post...");
        let gelbooru_post = match GelbooruPost::new_random(&gelbooru_config, &config.tags, config.range) {
            Ok(post) => {
                log::info!("Found post: {}", post.post_url);
                post
            },
            Err(error) => {
                log::error!("Failed to get random post: {}", error);
                log::info!("Waiting {} seconds before retrying...", config.error_timeout);
                thread::sleep(Duration::from_secs_f64(config.error_timeout));
                continue;
            }
        };

        let file_id = {
            let file_name = gelbooru_post.file_url.split('/').last().unwrap();

            // Check if the file already exists to avoid re-uploading
            match client.find_file_by_name(file_name) {
                Ok(file_id) => {
                    log::info!("Skipping upload to Misskey: {}", file_id);
                    file_id
                },
                Err(_) => {
                    log::info!("Requesting Misskey to download {}", gelbooru_post.file_url);
                    match client.upload_file_from_url(&gelbooru_post.file_url, config.sensitive) {
                        Ok(_) => (),
                        Err(error) => {
                            log::error!("Failed to upload file: {}", error);
                            log::info!("Waiting {} seconds before retrying...", config.error_timeout);
                            thread::sleep(Duration::from_secs_f64(config.error_timeout));
                            continue;
                        },
                    };

                    // Search for the file_id again after requesting download
                    // It may take some time for the file to be uploaded, so we check every 3 seconds up to 10 times
                    log::info!("Waiting for Misskey to download the file...");
                    let mut attempts = 0;
                    loop {
                        match client.find_file_by_name(file_name) {
                            Ok(file_id) => {
                                log::info!("Misskey downloaded the file: {}", file_id);
                                break file_id
                            },
                            Err(error) => {
                                attempts += 1;
                                if attempts >= 10 {
                                    log::error!("Failed to find file on Misskey. Maybe out of drive space?: {}", error);
                                    log::info!("Waiting {} seconds before retrying...", config.error_timeout);
                                    thread::sleep(Duration::from_secs_f64(config.error_timeout));
                                    continue 'post;
                                }
                                thread::sleep(Duration::from_secs(3));
                            }
                        }
                    }
                }
            }
        };

        let message = if config.append_post_url {
            &format!("{}\n\n{}", config.message, gelbooru_post.post_url)
        } else {
            &config.message
        };

        match client.post_message(message, vec![file_id], config.visibility) {
            Ok(note_id) => log::info!("Posted note: {}", note_id),
            Err(error) => {
                log::error!("Failed to post note: {}", error);
                log::info!("Waiting {} seconds before retrying...", config.error_timeout);
                thread::sleep(Duration::from_secs_f64(config.error_timeout));
                continue;
            }
        };

        log::info!("Waiting {} seconds before posting again...", config.post_interval);
        thread::sleep(Duration::from_secs_f64(config.post_interval));
    }
}
