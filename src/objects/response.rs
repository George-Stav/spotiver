use serde::{Serialize, Serializer, Deserialize, Deserializer};
use serde_json::{Value, Number};

#[derive(Debug, Deserialize, Serialize)]
pub struct Response<T: Serialize> {
    // href: String,
    // limit: i32,
    // previous: Option<String>,
    // offset: i32,
    // total: i32,
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
