#![allow(dead_code)]
#![allow(unused)]

mod authenticate;
mod objects;
use authenticate as auth;
mod db;

use std::{
    env,
    backtrace::Backtrace,
    collections::{VecDeque, HashMap},
    time::{Duration, SystemTime},
    fs::File,
    path::Path,
    io::Write
};
use csv::Writer;
use chrono::{DateTime, Utc};
use serde::{Serialize, Serializer, Deserialize, Deserializer};
// use serde_json::{Value, Number};
use reqwest::{
    header::{HeaderMap, AUTHORIZATION, CONTENT_TYPE},
    Client
};
use crate::objects::{
    playlist::Playlist,
    track::Track
};
use crate::db::Db;
    

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let token = auth::token().await?;

    // let mut headers = HeaderMap::new();
    // headers.insert(AUTHORIZATION, format!("Bearer {token}").parse().unwrap());
    // headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());

    // let client = Client::builder()
    //     .connection_verbose(true)
    //     .default_headers(headers)
    //     .build()?;

    // let _tracks = objects::tracks(&client, "7KFoK4LJ23EncELJwYmTDG").await?;
    // objects::backup(&client, Path::new("/home/george/BULK/spotiver"), true, true).await?;

    db::create();
    
    Ok(())
}
