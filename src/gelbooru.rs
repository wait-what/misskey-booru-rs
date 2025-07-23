use nanoserde::DeJson;

struct GelbooruAuth {
    api_key: String,
    user_id: String,
}

pub struct GelbooruConfig {
    booru_url: String,
    auth: Option<GelbooruAuth>,
}

impl GelbooruConfig {
    pub fn new(booru_url: String, api_key: Option<String>, user_id: Option<String>) -> Self {
        Self {
            booru_url,
            auth: match (api_key, user_id) {
                (Some(api_key), Some(user_id)) => Some(GelbooruAuth { api_key, user_id }),
                _ => None,
            },
        }
    }

    fn get_auth_query(&self) -> String {
        if let Some(auth) = &self.auth {
            format!("&api_key={}&user_id={}", auth.api_key, auth.user_id)
        } else {
            String::new()
        }
    }
}

pub struct GelbooruPost {
    pub file_url: String,
    pub post_url: String,
}

impl GelbooruPost {
    pub fn new_random(gelbooru: &GelbooruConfig, tags: &Vec<String>, range: u32) -> Result<Self, String> {
        #[derive(DeJson)]
        struct Response {
            #[nserde(rename = "@attributes")]
            attributes: ResponseAttributes,
            post: [ResponsePost; 1],
        }

        #[derive(DeJson)]
        struct ResponseAttributes {
            count: u32,
        }

        #[derive(DeJson)]
        struct ResponsePost {
            file_url: String,
            id: u32,
        }

        // Search to get the count of posts
        let query = format!(
            "{}/index.php?page=dapi&s=post&q=index&tags={}&json=1&limit=1{}",
            gelbooru.booru_url,
            tags.join("+"),
            gelbooru.get_auth_query()
        );
        let body = match ureq::get(query).call() {
            Ok(response) => match response.into_body().read_to_string() {
                Ok(body) => body,
                Err(error) => return Err(error.to_string()),
            },
            Err(error) => return Err(error.to_string()),
        };
        let response = match Response::deserialize_json(&body) {
            Ok(response) => response,
            Err(error) => return Err(error.to_string()),
        };

        // Select a random post
        // Gelbooru will not let you search with a page over 20000
        let mut post_count = response.attributes.count;
        if post_count > 20000 {
            post_count = 20000;
        }

        let page = if range == 0 || range > post_count {
            rand::random::<u32>() % post_count
        } else {
            rand::random::<u32>() % range
        };

        // Search again to get the selected post
        let query = format!(
            "{}/index.php?page=dapi&s=post&q=index&tags={}&json=1&limit=1&pid={}{}",
            gelbooru.booru_url,
            tags.join("+"),
            page,
            gelbooru.get_auth_query()
        );
        let body = match ureq::get(query).call() {
            Ok(response) => match response.into_body().read_to_string() {
                Ok(body) => body,
                Err(error) => return Err(error.to_string()),
            },
            Err(error) => return Err(error.to_string()),
        };
        let response = match Response::deserialize_json(&body) {
            Ok(response) => response,
            Err(error) => return Err(error.to_string()),
        };
        let post = match response.post.get(0) {
            Some(post) => post,
            None => return Err(format!("{} returned [] for {:?}", gelbooru.booru_url, tags)),
        };

        Ok(Self {
            file_url: post.file_url.clone(),
            post_url: format!("{}/index.php?page=post&s=view&id={}", gelbooru.booru_url, post.id),
        })
    }
}
