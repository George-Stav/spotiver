#![allow(dead_code)]
#![allow(unused)]

mod authenticate;
mod objects;
use authenticate as auth;

use std::{
    collections::{VecDeque, HashMap},
    time::{Duration, SystemTime},
    fs::File,
    path::Path,
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

    let root_dir = Path::new("/mnt/HDD/MUSIC/WEEDIAN/flac");
    let mut sum = 0;
    let mut successful = 0;
    for album in objects::albums(&root_dir) {
	println!("[{}]", album);
	let v = objects::track_names(root_dir.join(album.clone()).as_path());
	sum += v.len();
	successful += objects::search_weedian_tracks(&client, root_dir.join(album.clone()).as_path()).await;
    }
    println!("Total: {successful}/{sum}");
    Ok(())
}
