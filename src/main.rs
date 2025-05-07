use std::thread;
use std::time::Duration;

mod gelbooru;
use gelbooru::GelbooruPost;

mod misskey;
use misskey::MisskeyClient;

mod config;
use config::Config;

fn main() {
    let config = Config::new("config.toml").unwrap();
    let client = MisskeyClient::new(&config.token, &config.instance_url);

    loop {
        println!("Posting image...");
        let gelbooru_post = GelbooruPost::new_random(&config.booru_url, &config.tags).unwrap(); // todo: error handling
        let file_id = {
            let file_name = gelbooru_post.file_url.split('/').last().unwrap();

            // Check if the file already exists to avoid re-uploading
            match client.find_file_by_name(file_name) {
                Ok(file_id) => file_id,
                Err(_) => {
                    client
                        .upload_file_from_url(&gelbooru_post.file_url)
                        .unwrap() // todo: error handling
                }
            }
        };

        let message = if config.append_post_url {
            &format!("{}\n\n{}", config.message, gelbooru_post.post_url)
        } else {
            &config.message
        };

        client
            .post_message(message, vec![file_id], config.visibility)
            .unwrap(); // todo: error handling

        println!("Image posted! {}", gelbooru_post.post_url);
        thread::sleep(Duration::from_secs_f64(config.post_interval));
    }
}
