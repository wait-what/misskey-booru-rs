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

    pub fn find_file_by_name(&self, name: &str) -> Result<FileId, String> {
        let request = {
            #[derive(SerJson)]
            struct Request {
                name: String,
            }

            Request {
                name: name.to_string(),
            }.serialize_json()
        };

        let api_call = ureq::post(format!("{}/api/drive/files/find", self.base_url))
            .header("Authorization", format!("Bearer {}", self.token))
            .header("Content-Type", "application/json")
            .send(request);

        let body = match api_call {
            Ok(response) => match response.into_body().read_to_string() {
                Ok(body) => body,
                Err(error) => return Err(error.to_string()),
            },
            Err(error) => return Err(error.to_string()),
        };

        let response = {
            #[derive(Debug, DeJson)]
            struct DriveFile {
                id: String,
            }

            #[derive(Debug, DeJson)]
            struct Response {
                files: Vec<DriveFile>,
            }

            // The response is a DriveFile[], and nanoserde doesn't support deserializing root arrays
            match Response::deserialize_json(&format!("{{\"files\":{}}}", body)) {
                Ok(response) => response,
                Err(error) => return Err(error.to_string()),
            }
        };

        match response.files.get(0) {
            Some(file) => Ok(file.id.clone()),
            None => Err("File not found".to_string()),
        }
    }

    pub fn upload_file_from_url(&self, url: &str) -> Result<FileId, String> {
        // https://miruku.cafe/api-doc#tag/drive/POST/drive/files/upload-from-url

        todo!()
    }

    #[rustfmt::skip]
    pub fn post_message(&self, content: &str, attachments: Vec<FileId>, visibility: PostVisibility) -> Result<ureq::http::Response<ureq::Body>, ureq::Error> {

        let url = format!("{}/api/notes/create", self.base_url);

        let request = {
            #[derive(SerJson)]
            struct Request {
                visibility: String,
                fileIds: Vec<FileId>,
                text: String,
            }

            Request {
                visibility: visibility.into(),
                fileIds: attachments,
                text: content.to_string(),
            }.serialize_json()
        };

        ureq::post(url)
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}",self.token))
        .send_json(request)
    }
}
