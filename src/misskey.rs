use nanoserde::{DeJson, SerJson};

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

pub type FileId = String;

impl<'a> MisskeyClient<'a> {
    pub fn new(token: &'a str, base_url: &'a str) -> Self {
        Self { token, base_url }
    }

    pub fn upload_file_from_url(&self, url: &str) -> Result<FileId, String> {
        // https://miruku.cafe/api-doc#tag/drive/POST/drive/files/upload-from-url

        todo!()
    }

    #[rustfmt::skip]
    pub fn post_message(&self, content: &str, attachments: Vec<FileId>, visibility: PostVisibility) -> Result<(), String> {
        // https://miruku.cafe/api-doc#tag/notes/POST/notes/create

        todo!()
    }
}
