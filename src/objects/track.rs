use crate::objects::image::Image;
use std::{
    collections::VecDeque,
    cmp::{PartialEq, Eq},
    hash::{Hash, Hasher}
};
use serde::{Serialize, Serializer, Deserialize, Deserializer};
use serde_json::{Value, Number};

#[derive(Debug, Serialize, Clone)]
pub struct Track {
    id: Option<String>,
    title: Option<String>,
    duration: String, // Given in ms; convert to [MM:SS]
    added_at: String,
    is_local: bool,
    track_isrc: Option<String>, // Part of ExternalIds object
    track_ean: Option<String>, // Part of ExternalIds object
    track_upc: Option<String>, // Part of ExternalIds object
    contributor_id: String,
    contributor_uri: String,
    contributor_spotify_url: Option<String>, // ExternalUrls
    album_id: Option<String>,
    album_title: Option<String>,
    album_total_tracks: Option<Number>,
    // album_available_markets: String, // pipe separated values (e.g. CA|BR|IT)
    album_spotify_url: Option<String>, // ExternalUrls
    album_img_url: String,
    album_img_width: Option<Number>,
    album_img_height: Option<Number>,
    album_release_date: Option<String>,
    album_uri: Option<String>,
    album_isrc: Option<String>, // Part of ExternalIds object
    album_ean: Option<String>, // Part of ExternalIds object
    album_upc: Option<String>, // Part of ExternalIds object
    // album_genres: String, // pipe separated values (e.g. rock|punk)
    // album_label: Option<String>,
    // album_popularity: Option<Number>,
    // album_artists_num: Number,
    // album_artists_name: String, // pipe separated values (e.g. Immortal|Darkthrone)
    artists_num: Number,
    artists_name: String,
    artists_id: String,
    // artists_genres: String, // pipe separated values (e.g. rock|punk)
    // artists_popularity: Option<String>,
    artists_uri: String,
    // artists_img_url: String,
    // artists_img_width: String,
    // artists_img_height: String,
    // artists_followers: String,
    // artists_spotify_url: String, // ExternalUrls
    disc_number: Number,
    explicit: bool,
    // is_playable: Option<bool>,
    popularity: Number,
    preview_url: Option<String>,
    track_number: Number,
    uri: Option<String>,
}

#[derive(Debug, Deserialize)]
struct Outer {
    added_at: String,
    added_by: Contributor,
    is_local: bool,
    track: Option<TrackObject>,
}

#[derive(Debug, Deserialize)]
struct TrackObject {
    album: Album,
    artists: Vec<Artist>,
    disc_number: Number,
    duration_ms: Number,
    explicit: bool,
    external_ids: Option<ExternalIds>,
    external_urls: ExternalUrls,
    id: Option<String>,
    name: Option<String>,
    popularity: Number,
    preview_url: Option<String>,
    track_number: Number,
    uri: Option<String>,
    // is_playable: Option<bool>,
}

impl Default for TrackObject {
    fn default() -> Self {
        TrackObject {
            album: Album::default(),
            artists: Vec::new(),
            disc_number: Number::from(0),
            duration_ms: Number::from(0),
            explicit: false,
            external_ids: Some(ExternalIds::default()),
            external_urls: ExternalUrls::default(),
            id: None,
            name: None,
            popularity: Number::from(0),
            preview_url: None,
            track_number: Number::from(0),
            uri: None
        }
    }
}

#[derive(Debug, Deserialize, Default)]
struct Album {
    total_tracks: Option<Number>,
    // available_markets: Vec<String>,
    external_urls: ExternalUrls,
    id: Option<String>,
    images: VecDeque<Image>,
    name: Option<String>,
    release_date: Option<String>,
    uri: Option<String>,
    external_ids: Option<ExternalIds>,
    // genres: Option<Vec<String>>,
    // label: Option<String>,
    // popularity: Option<Number>,
}

#[derive(Debug, Deserialize, Clone)]
struct Artist {
    external_urls: ExternalUrls,
    genres: Option<Vec<String>>,
    id: Option<String>,
    images: Option<VecDeque<Image>>,
    name: Option<String>,
    popularity: Option<Number>,
    uri: Option<String>,
}

#[derive(Debug, Deserialize)]
struct Contributor {
    id: String,
    uri: String,
    external_urls: ExternalUrls,
}

#[derive(Debug, Deserialize, Clone, Default)]
struct ExternalUrls {
    spotify: Option<String>
}

#[derive(Debug, Deserialize, Default)]
struct ExternalIds {
    isrc: Option<String>,
    ean: Option<String>,
    upc: Option<String>,
}

impl<'de> Deserialize<'de> for Track {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D: Deserializer<'de>
    {
        macro_rules! artist_concat {
            ($artists:expr, $var:ident) => {
                $artists.iter().filter_map(|artist| artist.$var.clone()).collect::<Vec<String>>().join("|")
            };
        }

        // let mut h = Outer::deserialize(deserializer).expect("Unsuccessful deserialization in Track");
        let mut h = match Outer::deserialize(deserializer) {
            Ok(outer) => outer,
            Err(e) => {
                println!("Error");
                return Err(e);
            }
        };
        let mut track = h.track.unwrap_or_default();
        let ms: f64 = track.duration_ms.as_f64().unwrap();
        let album_img: Image = match track.album.images.pop_front() {
            Some(image) => image,
            None => Image::default(),
        };
        let track_ext_ids = track.external_ids.unwrap_or_default();
        let album_ext_ids = track.album.external_ids.unwrap_or_default();

        Ok(Track {
            // TODO: Deal with nested Images
            id: track.id,
            title: track.name,
            duration: format!("{}:{:.0}", (ms/60000_f64).floor(), (ms%60000_f64)/1000_f64),// Given in ms; convert to [MM:SS]
            added_at: h.added_at,
            is_local: h.is_local,
            track_isrc: track_ext_ids.isrc, // Part of ExternalIds object
            track_ean: track_ext_ids.ean, // Part of ExternalIds object
            track_upc: track_ext_ids.upc, // Part of ExternalIds object
            contributor_id: h.added_by.id,
            contributor_uri: h.added_by.uri,
            contributor_spotify_url: h.added_by.external_urls.spotify, // ExternalUrls
            album_id: track.album.id,
            album_title: track.album.name,
            album_total_tracks: track.album.total_tracks,
            // album_available_markets: h.track.album.available_markets.join("|"), // pipe separated values (e.g. CA|BR|IT)
            album_spotify_url: track.album.external_urls.spotify, // ExternalUrls
            album_img_url: album_img.url,
            album_img_width: album_img.width,
            album_img_height: album_img.height,
            album_release_date: track.album.release_date,
            album_uri: track.album.uri,
            album_isrc: album_ext_ids.isrc, // Part of ExternalIds object
            album_ean: album_ext_ids.ean, // Part of ExternalIds object
            album_upc: album_ext_ids.upc, // Part of ExternalIds object
            // album_genres: h.track.album.genres.unwrap_or_default().join("|"), // pipe separated values (e.g. rock|punk)
            // album_label: h.track.album.label,
            // album_popularity: h.track.album.popularity, // {Some(p) => p, None => Number::from(0)},
            // album_artists_num: Number,
            // album_artists_name: String, // pipe separated values (e.g. Immortal|Darkthrone)
            artists_num: Number::from(track.artists.len()),
            artists_name: artist_concat!(track.artists, name),
            artists_id: artist_concat!(track.artists, id),
            // artist_genres: artist_concat!(h.track.artists, genres), // pipe separated values (e.g. rock|punk)
            // artists_popularity: artist_concat!(h.track.artists, popularity),
            artists_uri: artist_concat!(track.artists, name),
            // artists_img_url: artist_concat!(h.track.artists, name),
            // artists_img_width: artist_concat!(h.track.artists, name),
            // artists_img_height: artist_concat!(h.track.artists, name),
            // artists_followers: artist_concat!(h.track.artists, name),
            // artists_spotify_url: artist_concat!(h.track.artists, name), // ExternalUrls
            disc_number: track.disc_number,
            explicit: track.explicit,
            // is_playable: h.track.is_playable,
            popularity: track.popularity,
            preview_url: track.preview_url,
            track_number: track.track_number,
            uri: track.uri,
        })
    }
}

impl PartialEq for Track {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
impl Eq for Track {}

impl Hash for Track {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}
