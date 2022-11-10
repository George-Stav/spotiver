#![allow(dead_code)]
#![allow(unused)]

mod authenticate;
use authenticate as auth;

use std::{
    collections::HashMap,
    time::{Duration, SystemTime}
};
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use serde_json::Value;
use reqwest::header::{HeaderMap, AUTHORIZATION, CONTENT_TYPE};

#[tokio::main]
// async fn main() -> Result<(), reqwest::Error> {
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let token = auth::token().await?;
    
    println!("{}", token);
    // playlists(token).await?;
    Ok(())
}

async fn playlists(token: String) -> Result<(), Box<dyn std::error::Error>> {
    let mut headers = HeaderMap::new();
    headers.insert(AUTHORIZATION, format!("Bearer {}", token).parse().unwrap());
    headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());

    let response = reqwest::Client::new()
        .get("https://api.spotify.com/v1/me/playlists")
        .headers(headers)
        .query(&[("offset", 0), ("limit", 50)])
        .send()
        .await?;
        // .json()
        // .await?;

    println!("{:#?}", response.text().await?);
    Ok(())
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

#[derive(Debug, Deserialize)]
struct Playlists {
    items: Vec<Playlists>
}

#[derive(Debug, Deserialize)]
struct Playlist {
    collaborative: bool,
    description: String,
    #[serde(flatten)]
    external_urls: HashMap<String, String>
}
