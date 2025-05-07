use nanoserde::DeJson;

pub struct GelbooruPost {
    pub file_url: String,
    pub post_url: String,
}

#[derive(DeJson)]
struct GelbooruResponse {
    #[nserde(rename = "@attributes")]
    attributes: GelbooruResponseAttributes,
    post: [GelbooruResponsePost; 1],
}

#[derive(DeJson)]
struct GelbooruResponseAttributes {
    count: u32,
}

#[derive(DeJson)]
struct GelbooruResponsePost {
    file_url: String,
    id: u32,
}

impl GelbooruPost {
    pub fn new_random(booru_url: &str, tags: &Vec<String>) -> Result<Self, String> {
        // Search to get the count of posts
        let query = format!(
            "{}/index.php?page=dapi&s=post&q=index&tags={}&json=1&limit=1",
            booru_url,
            tags.join("+")
        );
        let body = match ureq::get(query).call() {
            Ok(response) => response.into_body().read_to_string().unwrap(),
            Err(error) => return Err(error.to_string())
        };
        let response = GelbooruResponse::deserialize_json(&body).unwrap();
        let max_page = response.attributes.count;

        // Select a random post
        let page = rand::random::<u32>() % max_page;

        // Search again to get the selected post
        let query = format!(
            "{}/index.php?page=dapi&s=post&q=index&tags={}&json=1&limit=1&pid={}",
            booru_url,
            tags.join("+"),
            page
        );
        let body = match ureq::get(query).call() {
            Ok(response) => response.into_body().read_to_string().unwrap(),
            Err(error) => return Err(error.to_string())
        };
        let response = GelbooruResponse::deserialize_json(&body).unwrap();
        let post = response.post.get(0).unwrap(); // todo: error handling

        Ok(Self {
            file_url: post.file_url.clone(),
            post_url: format!(
                "{}/index.php?page=post&s=view&id={}",
                booru_url, post.id
            ),
        })
    }
}
