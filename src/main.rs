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
    playlists(token).await?;
    Ok(())
}

async fn playlists(token: String) -> Result<(), Box<dyn std::error::Error>> {
    let mut headers = HeaderMap::new();
    headers.insert(AUTHORIZATION, format!("Bearer {token}").parse().unwrap());
    headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());

    let client: Client = Client::new();
    let mut response: Response = client
        .get("https://api.spotify.com/v1/me/playlists")
        .headers(headers.clone())
        .query(&[("offset", 0), ("limit", 50)])
        .send().await?
        .json().await?;
    let mut playlists: Vec<Playlist> = response.items;

    // println!("{:#?}", &response);
    while let Some(href) = response.next {
        response = client
            .get(href)
            .headers(headers.clone())
            .send().await?
            .json().await?;
        playlists.append(&mut response.items);
        println!("{:?}", response.next);
    }

    println!("{:#?}", playlists[0]);

    let mut wtr = Writer::from_writer(vec![]);
    for playlist in playlists {
        wtr.serialize(playlist)?;
    }
    let mut output = File::create("test.csv")?;
    write!(output, "{}", String::from_utf8(wtr.into_inner()?)?)?;

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

#[derive(Debug, Serialize)]
struct Playlist {
    id: String,
    name: String,
    description: String,
    collaborative: bool,
    spotify_url: String,
    // images: Vec<Image>,
    owner_name: String,
    owner_href: String,
    owner_id: String,
    owner_type: String,
    owner_uri: String,
    primary_color: Option<String>,
    public: bool,
    snapshot_id: String,
    track_href: String,
    track_num: Number,
    #[serde(rename="type")]
    _type: String,
    uri: String,
}

impl<'de> Deserialize<'de> for Playlist {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>
    {
        #[derive(Deserialize)]
        struct Outer {
            id: String,
            name: String,
            description: String,
            collaborative: bool,
            external_urls: ExternalUrl,
            // images: Vec<Image>,
            owner: Owner,
            primary_color: Option<String>,
            public: bool,
            snapshot_id: String,
            tracks: Tracks,
            #[serde(rename="type")]
            _type: String,
            uri: String,
        }

        #[derive(Debug, Deserialize, Serialize)]
        struct Owner {
            display_name: String,
            // external_urls: ExternalUrl,
            href: String,
            id: String,
            #[serde(rename="type")]
            _type: String,
            uri: String,
        }

        #[derive(Deserialize)]
        struct Tracks {
            href: String,
            total: Number,
        }

        #[derive(Deserialize, Default)]
        struct ExternalUrl {
            spotify: String
        }

        let helper = Outer::deserialize(deserializer)?;
        Ok(Playlist {
            id: helper.id,
            name: helper.name,
            description: helper.description,
            collaborative: helper.collaborative,
            spotify_url: helper.external_urls.spotify,
            // images: helper.images,
            owner_name: helper.owner.display_name,
            owner_href: helper.owner.href,
            owner_id: helper.owner.id,
            owner_type: helper.owner._type,
            owner_uri: helper.owner.uri,
            primary_color: helper.primary_color,
            public: helper.public,
            snapshot_id: helper.snapshot_id,
            track_href: helper.tracks.href,
            track_num: helper.tracks.total,
            _type: helper._type,
            uri: helper.uri
        })
    }

}

// impl Serialize for Playlist  {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//         where S: Serializer
//     {
//         Ok(self)
//     }
// }

#[derive(Debug, Deserialize, Serialize)]
struct Tracks {
    total: Number
}

#[derive(Debug, Deserialize, Serialize)]
struct Image {
    height: Option<i32>,
    width: Option<i32>,
    url: String,
}


// #[derive(Debug, Deserialize)]
// struct Account {
//     country: String,
//     display_name: String,
//     // email: String,
//     #[serde(flatten)]
//     explicit_content: HashMap<String, Value>,
//     #[serde(flatten)]
//     external_urls: HashMap<String, Value>,
//     #[serde(flatten)]
//     followers: HashMap<String, Value>,
//     href: String,
//     id: String,
//     product: String,
//     #[serde(rename="type")]
//     _type: String,
//     uri: String,
// }
