pub struct MisskeyClient<'a> {
    token: &'a str,
    base_url: &'a str,
}

#[derive(Debug, Copy, Clone)]
pub enum PostVisibility {
    Public,
    Home,
    Followers,
}

impl Into<String> for PostVisibility {
    fn into(self) -> String {
        match self {
            PostVisibility::Public => "public".to_string(),
            PostVisibility::Home => "home".to_string(),
            PostVisibility::Followers => "followers".to_string(),
        }
    }
}

impl From<&str> for PostVisibility {
    fn from(visibility: &str) -> Self {
        match visibility {
            "public" => PostVisibility::Public,
            "home" => PostVisibility::Home,
            "followers" => PostVisibility::Followers,
            _ => panic!("Invalid visibility type"),
        }
    }
}

impl<'a> MisskeyClient<'a> {
    pub fn new(token: &'a str, base_url: &'a str) -> Self {
        Self { token, base_url }
    }

    pub fn upload_file_from_url(&self, url: &str) -> Result<&str, ()> {
        todo!()
    }

    pub fn post_message(
        &self,
        content: &str,
        attachments: Vec<&str>,
        visibility: PostVisibility,
    ) -> Result<(), ()> {
        todo!()
    }
}
