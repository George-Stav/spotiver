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
    error::Error,
    collections::HashSet,
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

pub async fn file_per_playlist(client: &Client) -> Result<(), Box<dyn Error>> {
    let mut playlists: Vec<Playlist> = playlists(client).await?;

    for playlist in playlists.iter() {
        println!("Pulling tracks from: {}", playlist.name);
        match tracks(client, &playlist.id).await {
            Ok(tracks) => {
                spotiver::save_to_csv(&tracks, &format!("playlists/{}.csv", playlist.name));
            },
            Err(err) => {
                println!("Encountered an error while fetching tracks for {}\n {}", playlist.name, err);
            }
        };
    }

    Ok(())
}

pub async fn all_tracks(client: &Client) -> Result<(), Box<dyn Error>> {
    let mut playlists: Vec<Playlist> = playlists(client).await?;
    let mut tracks_set: HashSet<Track> = HashSet::new();

    for playlist in playlists.iter() {
        println!("Pulling tracks from: {}", playlist.name);
        // let t: Vec<Track> = tracks(client, &playlist.id).await?;
        let t: Vec<Track> = match tracks(client, &playlist.id).await {
            Ok(r) => r,
            Err(err) => {
                // let t: Vec<Track> = Vec::from_iter(tracks_set);
                // spotiver::save_to_csv(&t, "all-tracks.csv")?;
                println!("Encountered an error while fetching tracks for {}\n {}", playlist.name, err);
                Vec::new()
            }
        };

        for track in t.iter() {
            tracks_set.insert(track.clone());
        }
    }

    let t: Vec<Track> = Vec::from_iter(tracks_set);
    spotiver::save_to_csv(&t, "all-tracks.csv")?;

    Ok(())
}

pub async fn tracks(client: &Client, id: &str) -> Result<Vec<Track>, Box<dyn Error>> {
    let mut response: Response<Track> = client
        .get(format!("https://api.spotify.com/v1/playlists/{id}/tracks"))
        .query(&[("offset", 0), ("limit", 50)])
        .send().await?
        .json().await?;
    let mut tracks: Vec<Track> = response.items;

    while let Some(href) = response.next {
        response = client
            .get(href)
            .send().await?
            .json().await?;
        tracks.append(&mut response.items);
    }

    spotiver::save_to_csv(&tracks, "2nd_coming.csv")?;

    Ok(tracks)
}

pub async fn playlists(client: &Client) -> Result<Vec<Playlist>, Box<dyn Error>> {
    let mut response: Response<Playlist> = client
        .get("https://api.spotify.com/v1/me/playlists")
        .query(&[("offset", 0), ("limit", 50)])
        .send().await?
        .json().await?;
    let mut playlists: Vec<Playlist> = response.items;

    while let Some(href) = response.next {
        response = client
            .get(href)
            .send().await?
            .json().await?;
        playlists.append(&mut response.items);
    }

    spotiver::save_to_csv(&playlists, "nikfisto-playlists.csv")?;

    Ok(playlists)
}
