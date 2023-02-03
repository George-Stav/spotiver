use serde::{Serialize, Serializer, Deserialize, Deserializer};
use serde_json::{Value, Number};

#[derive(Debug, Deserialize, Serialize)]
pub struct Response<T: Serialize> {
    // href: String,
    // limit: i32,
    // previous: Option<String>,
    pub next: Option<String>,
    // offset: i32,
    // total: i32,
    pub items: Vec<T>,
}
