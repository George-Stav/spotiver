#![allow(dead_code)]
#![allow(unused)]

mod authenticate;
mod objects;
use authenticate as auth;

use std::{
    collections::VecDeque,
    time::{Duration, SystemTime},
    fs::File,
    io::Write
};
use csv::Writer;
use chrono::{DateTime, Utc};
use serde::{Serialize, Serializer, Deserialize, Deserializer};
use serde_json::{Value, Number};
use reqwest::{
    header::{HeaderMap, AUTHORIZATION, CONTENT_TYPE},
    Client
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let token = auth::token().await?;

    let mut headers = HeaderMap::new();
    headers.insert(AUTHORIZATION, format!("Bearer {token}").parse().unwrap());
    headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());

    let client = Client::builder()
        .default_headers(headers)
        .build()?;

    objects::file_per_playlist(&client).await;
    // objects::tracks(&client, "4ZBu3Yz2pzW5zY7n1dRZXg").await; // melodic good metal "33VnWGWkL4o26g6Z2ETH9X"
    // objects::all_tracks(&client).await;
    // objects::playlists(&client).await;
    Ok(())
}
