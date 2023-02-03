#![allow(dead_code)]
#![allow(unused)]

mod response;
mod playlist;
mod track;
mod image;

use crate::objects::{
    playlist::Playlist,
    response::Response,
    track::Track
};
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

pub async fn tracks(token: String, id: String) -> Result<(), Box<dyn std::error::Error>> {
    let mut headers = HeaderMap::new();
    headers.insert(AUTHORIZATION, format!("Bearer {token}").parse().unwrap());
    headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());

    let client: Client = Client::new();
    let response2 = client
        .get(format!("https://api.spotify.com/v1/playlists/{id}/tracks"))
        .headers(headers.clone())
        .query(&[("offset", 0), ("limit", 50)])
        .send().await?;

    let mut response: Response<Track> = client
        .get(format!("https://api.spotify.com/v1/playlists/{id}/tracks"))
        .headers(headers.clone())
        .query(&[("offset", 0), ("limit", 50)])
        .send().await?
        .json().await?;

    // println!("{:#?}", response2.text().await?);
    // println!("{:#?}", response.items);

    // let mut wtr = Writer::from_writer(vec![]);
    // for track in response.items {
    //     wtr.serialize(track)?;
    // }
    // let mut output = File::create("2nd_coming.csv")?;
    // write!(output, "{}", String::from_utf8(wtr.into_inner()?)?)?;

    Ok(())
}

pub async fn playlists(token: String) -> Result<(), Box<dyn std::error::Error>> {
    let mut headers = HeaderMap::new();
    headers.insert(AUTHORIZATION, format!("Bearer {token}").parse().unwrap());
    headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());

    let client: Client = Client::new();
    // let mut response2 = client
    //     .get("https://api.spotify.com/v1/me/playlists")
    //     .headers(headers.clone())
    //     .query(&[("offset", 0), ("limit", 50)])
    //     .send().await?;
    // println!("{:#?}", response2.text().await?);

    let mut response: Response<Playlist> = client
        .get("https://api.spotify.com/v1/me/playlists")
        .headers(headers.clone())
        .query(&[("offset", 0), ("limit", 50)])
        .send().await?
        .json().await?;
    let mut playlists: Vec<Playlist> = response.items;

    while let Some(href) = response.next {
        response = client
            .get(href)
            .headers(headers.clone())
            .send().await?
            .json().await?;
        playlists.append(&mut response.items);
    }

    let mut wtr = Writer::from_writer(vec![]);
    for playlist in playlists {
        wtr.serialize(playlist)?;
    }
    let mut output = File::create("test.csv")?;
    write!(output, "{}", String::from_utf8(wtr.into_inner()?)?)?;

    Ok(())
}
