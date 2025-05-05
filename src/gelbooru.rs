use nanoserde::DeJson;

pub struct GelbooruPost {
    pub file_url: String,
    pub post_url: String,
}

impl GelbooruPost {
    pub fn new_random(booru_url: &str, tags: &Vec<String>) -> Result<Self, ()> {
        let query = format!(
            "{}/index.php?page=dapi&s=post&q=index&tags={}&json=1&limit=1",
            booru_url,
            tags.join("+")
        );

        Ok(Self {
            file_url: todo!(),
            post_url: todo!(),
        })
    }
}
