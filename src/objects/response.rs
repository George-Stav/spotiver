use crate::objects::sj_number::SjNumber;

use serde::{Serialize, Serializer, Deserialize, Deserializer};
use serde_json::Number;
use std::clone::Clone;

#[derive(Debug, Deserialize, Serialize)]
pub struct SearchResponse<T: Serialize+Clone> {
    pub tracks: Response<T>
}

#[derive(Debug, Serialize, Clone)]
pub struct Response<T: Serialize> {
    pub href: String,
    pub limit: Number,
    pub next: Option<String>,
    pub offset: Number,
    pub previous: Option<String>,
    pub total: Number,
    // #[serde(deserialize_with = "deserialize_items")]
    pub items: Vec<T>,
}

impl<'de, T: Serialize+Deserialize<'de>+Clone> Deserialize<'de> for Response<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>
    {
        #[derive(Debug, Deserialize)]
        struct Outer<T: Serialize> {
	    href: String,
	    limit: Number,
	    next: Option<String>,
	    offset: Number,
	    previous: Option<String>,
	    total: Number,
	    items: Vec<Option<T>>,
        }

	let mut helper = Outer::deserialize(deserializer)?;
	let items: Vec<T> = helper.items
	    .iter()
	    .filter_map(|e| e.clone().unwrap_or(None))
	    .collect();

	// println!("[INFO]: {} non-nil items in Response out of a total of {}", items.len(), helper.total);

	Ok(Response {
	    href: helper.href,
	    limit: helper.limit,
	    next: helper.next,
	    offset: helper.offset,
	    previous: helper.previous,
	    total: helper.total,
	    items: items
	})	
    }
}

// fn deserialize_items<'de, T, D>(deserializer: D) -> Result<Vec<Option<T>>, D::Error>
// where
//     T: Deserialize<'de>,
//     D: Deserializer<'de>,
// {
//     let items: Vec<T> = Vec::deserialize(deserializer)?;
//     let mut result = Vec::with_capacity(items.len());

//     for item in items {
//         match serde_json::to_value(&item) {
//             Ok(value) => {
//                 // Attempt to deserialize the item
//                 match serde_json::from_value(value) {
//                     Ok(valid_item) => result.push(Some(valid_item)),
//                     Err(_) => result.push(None), // Push None if deserialization fails
//                 }
//             }
//             Err(_) => result.push(None), // Push None if conversion to value fails
//         }
//     }

//     Ok(result)
// }
