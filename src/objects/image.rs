use serde::{Serialize, Deserialize};
use serde_json::Number;

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(default)]
pub struct ImageObject {
    pub url: String,
    pub width: Option<Number>,
    pub height: Option<Number>,
}

impl Default for ImageObject {
    fn default() -> Self {
	ImageObject {
	    url: "".to_string(),
	    width: None,
	    height: None
	}
    }
}

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
