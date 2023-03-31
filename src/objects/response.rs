use serde::{Serialize, Serializer, Deserialize, Deserializer};
use serde_json::{Value, Number};

#[derive(Debug, Deserialize, Serialize)]
pub struct SearchResponse<T: Serialize> {
    pub tracks: Response<T>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Response<T: Serialize> {
    pub href: String,
    pub limit: Number,
    pub previous: Option<String>,
    pub offset: Number,
    pub total: Number,
    pub next: Option<String>,
    pub items: Vec<T>,
    #[serde(skip_deserializing)]
    pub request_curl: String,
}

impl<T: Serialize> Response<T> {
    pub fn set_request_curl(&mut self, curl: &str) {
        self.request_curl = curl.to_string();
    }
}
