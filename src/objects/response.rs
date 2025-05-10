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

	println!("[INFO]: {} non-nil items in Response", items.len());

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


// #[derive(Debug, Serialize, Deserialize)]
// pub struct Response<T: Serialize> {
//     pub href: String,
//     pub limit: Number,
//     pub next: Option<String>,
//     pub offset: Number,
//     pub previous: Option<String>,
//     pub total: Number,
//     pub items: Vec<T>,
//     #[serde(skip_deserializing)]
//     pub request_curl: String,
// }

// impl<T: Serialize> Response<T> {
//     pub fn set_request_curl(&mut self, curl: &str) {
//         self.request_curl = curl.to_string();
//     }
// }
