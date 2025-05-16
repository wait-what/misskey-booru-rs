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
pub type NoteId = String;

impl<'a> MisskeyClient<'a> {
    pub fn new(token: &'a str, base_url: &'a str) -> Self {
        Self { token, base_url }
    }

    // https://miruku.cafe/api-doc#tag/drive/POST/drive/files/find
    pub fn find_file_by_name(&self, name: &str) -> Result<FileId, String> {
        let request = {
            #[derive(SerJson)]
            struct Request<'a> {
                name: &'a str,
            }

            Request { name }.serialize_json()
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
            None => Err(format!("Misskey returned [] when searching for file {}", name)),
        }
    }

    // https://miruku.cafe/api-doc#tag/drive/POST/drive/files/upload-from-url
    pub fn upload_file_from_url(&self, url: &str, is_sensitive: bool) -> Result<(), String> {
        let request = {
            #[derive(SerJson)]
            struct Request<'a> {
                url: &'a str,
                #[nserde(rename = "isSensitive")]
                is_sensitive: bool,
            }

            Request { url, is_sensitive }.serialize_json()
        };

        let api_call = ureq::post(format!("{}/api/drive/files/upload-from-url", self.base_url))
            .header("Authorization", format!("Bearer {}", self.token))
            .header("Content-Type", "application/json")
            .send(request);

        match api_call {
            Ok(_) => Ok(()),
            Err(error) => Err(error.to_string()),
        }
    }

    // https://miruku.cafe/api-doc#tag/notes/POST/notes/create
    #[rustfmt::skip]
    pub fn post_message(&self, text: &str, file_ids: Vec<FileId>, visibility: PostVisibility) -> Result<NoteId, String> {
        let request = {
            #[derive(SerJson)]
            struct Request<'a> {
                text: &'a str,
                #[nserde(rename = "fileIds")]
                file_ids: Vec<FileId>,
                visibility: String,
            }

            Request {
                text,
                file_ids,
                visibility: visibility.into(),
            }.serialize_json()
        };

        let api_call = ureq::post(format!("{}/api/notes/create", self.base_url))
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
            struct Response {
                #[nserde(rename = "createdNote")]
                created_note: CreatedNote,
            }

            #[derive(Debug, DeJson)]
            struct CreatedNote {
                id: NoteId,
            }

            match Response::deserialize_json(&body) {
                Ok(response) => response,
                Err(error) => return Err(error.to_string()),
            }
        };

        Ok(response.created_note.id)
    }
}
