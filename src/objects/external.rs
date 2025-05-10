use serde::{Serialize, Deserialize};

/* Spotify API Object */
#[derive(Debug, Deserialize, Serialize, Clone, Default)]
#[serde(default)]
pub struct ExternalURLsObject {
    spotify: String
}

/* Spotify API Object */
#[derive(Debug, Deserialize, Serialize, Clone, Default)]
#[serde(default)]
pub struct ExternalIDsObject {
    isrc: String,
    ean: String,
    upc: String
}
