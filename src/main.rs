#![allow(dead_code)]
#![allow(unused)]

mod authenticate;
use authenticate as auth;

use std::{
    collections::HashMap,
    time::{Duration, SystemTime},
    fs::File,
    io::Write
};
use csv::Writer;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use serde_json::Value;
use reqwest::header::{HeaderMap, AUTHORIZATION, CONTENT_TYPE};

// async fn main() -> Result<(), reqwest::Error> {
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let token = auth::token().await?;
    playlists(token).await?;
    Ok(())
}

async fn playlists(token: String) -> Result<(), Box<dyn std::error::Error>> {
    let mut headers = HeaderMap::new();
    headers.insert(AUTHORIZATION, format!("Bearer {token}").parse().unwrap());
    headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());

    let response: Response = reqwest::Client::new()
        .get("https://api.spotify.com/v1/me/playlists")
        .headers(headers)
        .query(&[("offset", 360), ("limit", 10)])
        .send()
        .await?
        .json()
        .await?;

    println!("{:#?}", response);

    // let mut wtr = Writer::from_writer(vec![]);
    // for playlist in response.items.iter() {
    //     wtr.serialize(playlist)?;
    // }
    // let mut output = File::create("test.csv")?;
    // write!(output, "{}", String::from_utf8(wtr.into_inner()?)?)?;

    Ok(())
}


#[derive(Debug, Deserialize, Serialize)]
struct Response {
    // href: String,
    // limit: i32,
    // previous: Option<String>,
    next: Option<String>,
    // offset: i32,
    // total: i32,
    items: Vec<Playlist>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Playlist {
    id: String,
    name: String,
    description: String,
    collaborative: bool,
    #[serde(flatten)]
    external_urls: HashMap<String, Value>,
    images: Vec<Image>,
    owner: Owner,
    primary_color: Option<String>,
    public: bool,
    snapshot_id: String,
    #[serde(flatten)]
    tracks: HashMap<String, Value>,
    #[serde(rename="type")]
    _type: String,
    uri: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct Image {
    height: Option<i32>,
    width: Option<i32>,
    url: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct Owner {
    display_name: String,
    external_urls: ExternalUrl,
    href: String,
    id: String,
    #[serde(rename="type")]
    _type: String,
    uri: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct ExternalUrl {
    spotify: String
}

#[derive(Debug, Deserialize)]
struct Account {
    country: String,
    display_name: String,
    // email: String,
    #[serde(flatten)]
    explicit_content: HashMap<String, Value>,
    #[serde(flatten)]
    external_urls: HashMap<String, Value>,
    #[serde(flatten)]
    followers: HashMap<String, Value>,
    href: String,
    id: String,
    product: String,
    #[serde(rename="type")]
    _type: String,
    uri: String,
}
