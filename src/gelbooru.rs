use nanoserde::DeJson;

pub struct GelbooruPost {
    pub file_url: String,
    pub post_url: String,
}

impl GelbooruPost {
    pub fn new_random(booru_url: &str, tags: &Vec<String>, range: u32) -> Result<Self, String> {
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
            "{}/index.php?page=dapi&s=post&q=index&tags={}&json=1&limit=1",
            booru_url,
            tags.join("+")
        );
        let body = match ureq::get(query).call() {
            Ok(response) => match response.into_body().read_to_string() {
                Ok(body) => body,
                Err(error) => return Err(error.to_string()),
            },
            Err(error) => return Err(error.to_string())
        };
        let response = match Response::deserialize_json(&body) {
            Ok(response) => response,
            Err(error) => return Err(error.to_string())
        };

        // Select a random post
        let page = if range == 0 || range > response.attributes.count {
            rand::random::<u32>() % response.attributes.count
        } else {
            rand::random::<u32>() % range
        };

        // Search again to get the selected post
        let query = format!(
            "{}/index.php?page=dapi&s=post&q=index&tags={}&json=1&limit=1&pid={}",
            booru_url,
            tags.join("+"),
            page
        );
        let body = match ureq::get(query).call() {
            Ok(response) => match response.into_body().read_to_string() {
                Ok(body) => body,
                Err(error) => return Err(error.to_string()),
            },
            Err(error) => return Err(error.to_string())
        };
        let response = match Response::deserialize_json(&body) {
            Ok(response) => response,
            Err(error) => return Err(error.to_string())
        };
        let post = match response.post.get(0) {
            Some(post) => post,
            None => return Err(format!("{} returned [] for {:?}", booru_url, tags)),
        };

        Ok(Self {
            file_url: post.file_url.clone(),
            post_url: format!(
                "{}/index.php?page=post&s=view&id={}",
                booru_url, post.id
            ),
        })
    }
}
