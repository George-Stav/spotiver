use crate::objects::external::ExternalURLsObject;
use serde::{Serialize, Deserialize};
use serde_json::Number;

/* Custom Object */
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct OwnerObject {
    external_urls: ExternalURLsObject,
    followers: FollowersObject,
    href: String,
    id: String,
    r#type: String,
    uri: String,
    display_name: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(default)]
struct FollowersObject {
    href: String,
    total: Number,
}

impl Default for FollowersObject {
    fn default() -> Self {
	FollowersObject {
	    href: "".to_string(),
	    total: Number::from(0),
	}
    }
}


// impl<'de> Deserialize<'de> for OwnerObject {
//     fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
//         where D: Deserializer<'de>
//     {
//         #[derive(Debug, Deserialize)]
//         struct Outer {
// 	    external_urls: ExternalUrlObject,
// 	    followers: Option<FollowersObject>,
// 	    href: String,
// 	    id: String,
// 	    r#type: String,
// 	    uri: String,
// 	    display_name: Option<String>,
//         }

//         let mut helper = Outer::deserialize(deserializer)?;

// 	Ok(OwnerObject {
// 	    external_urls: helper.ExternalUrlObject,
// 	    followers: Option<FollowersObject>,
// 	    href: String,
// 	    id: String,
// 	    r#type: String,
// 	    uri: String,
// 	    display_name: Option<String>,
// 	})
//     }
// }
