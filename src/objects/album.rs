use crate::objects::{
    external::ExternalURLsObject,
    artist::SimplifiedArtistObject,
    image::ImageObject,
    restriction::RestrictionsObject,
};
use serde::{Serialize, Deserialize};
use serde_json::Number;

/* Spotify API object */
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all="camelCase")]
enum AlbumType {
    Album, Single, Compilation, None
}

impl Default for AlbumType {
    fn default() -> Self {
	AlbumType::None
    }
}

/* Spotify API object */
#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct AlbumObject {
    album_type: Option<AlbumType>,
    total_tracks: Option<Number>,
    total: Option<Number>,
    available_markets: Vec<String>,
    external_urls: ExternalURLsObject,
    href: Option<String>,
    id: Option<String>,
    images: Vec<ImageObject>,
    name: String,
    release_date: Option<String>,
    release_date_precision: Option<String>,
    #[serde(default)]
    restrictions: RestrictionsObject,
    r#type: String,
    uri: Option<String>,
    artists: Vec<SimplifiedArtistObject>,
}
