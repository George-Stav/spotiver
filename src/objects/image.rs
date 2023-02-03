use serde::{Serialize, Serializer, Deserialize, Deserializer};
use serde_json::{Value, Number};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Image {
    pub url: String,
    pub width: Option<Number>,
    pub height: Option<Number>,
}

impl Default for Image {
    fn default() -> Self {
        Image {
            url: "".to_string(),
            width: Some(Number::from(0)),
            height: Some(Number::from(0)),
        }
    }
}
