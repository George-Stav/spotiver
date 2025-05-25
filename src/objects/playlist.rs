use crate::objects::{
    image::{ImageObject, Image},
    external::ExternalURLsObject,
    owner::OwnerObject,
    track::TracksObject
};
use std::fmt;
use std::collections::VecDeque;
use serde::{Serialize, Serializer, Deserialize, Deserializer};
use serde_json::{Value, Number};

/* Custom Object */
#[derive(Debug, Serialize, Clone)]
pub struct Playlist {
    collaborative: bool,
    description: String,
    external_urls: ExternalURLsObject,
    href: String,
    pub id: String,
    images: Vec<ImageObject>,
    pub name: String,
    owner: OwnerObject,
    public: bool,
    snapshot_id: String,
    tracks: TracksObject,
    r#type: String, // needs to be escaped with r# since it's an identifier in serde
    uri: String,
    primary_color: String,
}

impl fmt::Display for Playlist {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\"{}\" - {}", self.name, self.id)
    }
}

impl<'de> Deserialize<'de> for Playlist {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>
    {
	/* Object from Spotify API */
	#[derive(Debug, Deserialize)]
	struct SimplifiedPlaylistObject { // spotify API
	    collaborative: bool,
	    description: String,
	    external_urls: ExternalURLsObject, // custom
	    href: String,
	    pub id: String,
	    images: Option<Vec<ImageObject>>, // spotify API
	    pub name: String,
	    owner: OwnerObject, // custom
	    public: bool,
	    snapshot_id: String,
	    tracks: TracksObject, // custom
	    r#type: String, // needs to be escaped with r# since it's an identifier in serde
	    uri: String,
	    primary_color: Option<String>,
	}

	let mut helper = SimplifiedPlaylistObject::deserialize(deserializer)?;

	Ok(Playlist {
	    collaborative: helper.collaborative,
	    description: helper.description,
	    external_urls: helper.external_urls,
	    href: helper.href,
	    id: helper.id,
	    images: helper.images.unwrap_or(vec![]),
	    name: helper.name,
	    owner: helper.owner,
	    public: helper.public,
	    snapshot_id: helper.snapshot_id,
	    tracks: helper.tracks,
	    r#type: helper.r#type,
	    uri: helper.uri,
	    primary_color: helper.primary_color.unwrap_or("".to_string()),
	})
    }
}



#[derive(Debug, Deserialize)]
pub struct CreatePlaylist {
    pub id: String
}

// #[derive(Debug, Serialize)]
// pub struct Playlist {
//     pub id: String,
//     pub name: String,
//     description: String,
//     collaborative: bool,
//     spotify_url: String,
//     image_url: String,
//     image_width: Option<Number>,
//     image_height: Option<Number>,
//     owner_name: String,
//     owner_id: String,
//     owner_type: String,
//     owner_uri: String,
//     owner_url: String,
//     primary_color: String,
//     public: bool,
//     snapshot_id: String,
//     track_href: String,
//     track_num: Number,
//     #[serde(rename="type")]
//     _type: String,
//     uri: String,
// }

// impl<'de> Deserialize<'de> for Playlist {
//     fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
//         where D: Deserializer<'de>
//     {
//         #[derive(Debug, Deserialize)]
//         struct Outer {
//             id: String,
//             name: String,
//             description: String,
//             collaborative: bool,
//             external_urls: ExternalUrl,
//             images: VecDeque<Image>,
//             owner: Owner,
//             primary_color: Option<String>,
//             public: bool,
//             snapshot_id: String,
//             tracks: Tracks,
//             #[serde(rename="type")]
//             _type: String,
//             uri: String,
//         }

//         #[derive(Debug, Deserialize)]
//         struct Owner {
//             display_name: String,
//             external_urls: ExternalUrl,
//             // href: String,
//             id: String,
//             #[serde(rename="type")]
//             _type: String,
//             uri: String,
//         }

//         #[derive(Debug, Deserialize)]
//         struct Tracks {
//             href: String,
//             total: Number,
//         }

//         #[derive(Debug, Deserialize, Default)]
//         struct ExternalUrl {
//             spotify: String
//         }

//         let mut helper = Outer::deserialize(deserializer)?;
//         let img: Image = match helper.images.pop_front() {
//             Some(image) => image,
//             None => Image::default()
//         };

//         Ok(Playlist {
//             id: helper.id,
//             name: helper.name,
//             description: helper.description,
//             collaborative: helper.collaborative,
//             spotify_url: helper.external_urls.spotify,
//             image_url: img.url,
//             image_height: img.height,
//             image_width: img.width,
//             owner_name: helper.owner.display_name,
//             owner_id: helper.owner.id,
//             owner_type: helper.owner._type,
//             owner_uri: helper.owner.uri,
//             owner_url: helper.owner.external_urls.spotify,
//             primary_color: match helper.primary_color {
//                 Some(pc) => pc,
//                 None => "".to_string()
//             },
//             public: helper.public,
//             snapshot_id: helper.snapshot_id,
//             track_href: helper.tracks.href,
//             track_num: helper.tracks.total,
//             _type: helper._type,
//             uri: helper.uri
//         })
//     }

// }


// #[derive(Debug, Deserialize)]
// struct Account {
//     country: String,
//     display_name: String,
//     // email: String,
//     #[serde(flatten)]
//     explicit_content: HashMap<String, Value>,
//     #[serde(flatten)]
//     external_urls: HashMap<String, Value>,
//     #[serde(flatten)]
//     followers: HashMap<String, Value>,
//     href: String,
//     id: String,
//     product: String,
//     #[serde(rename="type")]
//     _type: String,
//     uri: String,
// }
