use crate::objects::{
    external::ExternalURLsObject,
};
use serde::{Serialize, Deserialize};

/* Spotify API object */
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SimplifiedArtistObject {
    external_urls: ExternalURLsObject,
    href: Option<String>,
    id: Option<String>,
    pub name: String,
    r#type: String,
    uri: Option<String>
}
