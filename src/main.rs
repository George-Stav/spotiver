#![allow(dead_code)]
#![allow(unused)]

mod authenticate;
mod objects;
use authenticate as auth;

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
use serde_json::{Value, Number};
use reqwest::{
    header::{HeaderMap, AUTHORIZATION, CONTENT_TYPE},
    Client
};

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

    // let pl_json = objects::pl_json(Path::new("/home/george/BULK/spotiver/playlists.json")).await?;
    // objects::backup(&client, Path::new("/home/george/BULK/spotiver"), false, false).await?;
    // let playlist_id = "7ojBoCyhFa615na068v1PB";
    // let playlist_id = "7KFoK4LJ23EncELJwYmTDG";
    // let _t = objects::tracks(&client, playlist_id).await;


    env::set_var("RUST_BACKTRACE", "full");
    
    let bt = Backtrace::capture();
    for frame in bt.frames() {
	println!("{:?}", frame);
    }

    Ok(())
}
