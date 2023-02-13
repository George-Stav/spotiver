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

// async fn main() -> Result<(), reqwest::Error> {
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let token = auth::token().await?;
    objects::tracks(token, "4ZBu3Yz2pzW5zY7n1dRZXg".to_string()).await; // melodic good metal "33VnWGWkL4o26g6Z2ETH9X"
    // objects::playlists(token).await
    Ok(())
}
