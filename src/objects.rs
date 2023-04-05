#![allow(dead_code)]
#![allow(unused)]

mod response;
mod playlist;
mod track;
mod image;

use crate::objects::{
    playlist::{Playlist, CreatePlaylist},
    response::{SearchResponse, Response},
    track::{SimpleTrack, Track},
};
use std::{
    error::Error,
    collections::{HashSet, VecDeque},
    time::{Duration, SystemTime},
    fs::File,
    io::{Write, Read},
    path::Path,
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

#[derive(Debug, Deserialize, Serialize)]
struct User {
    id: String,
}

#[derive(Serialize)]
struct Image {
    base64: String,
}

pub async fn search_weedian_tracks(client: &Client, path: &Path) -> usize {
    let mut tracks: Vec<(String, String)> = track_names(path);
    let mut count = 0;

    let mut uris: Vec<String> = Vec::new();
    for (artist, track) in tracks.iter() {
	let res = search_track(client, &track, &artist).await;
	if let Some(r) = res {
	    count += 1;
	    uris.push(r.uri);
	}
    }
    let playlist_name = path.iter().last().unwrap().to_str().unwrap().replace("_", " ");
    let playlist_id = weedian_create_playlist(client, playlist_name.as_str()).await.unwrap();

    weedian_update_playlist_image(client, playlist_id.as_str(), path.join(Path::new("small_cover.png")).as_path()).await.unwrap();
    add_tracks_to_playlist(client, &uris, playlist_id.as_str()).await;
    add_tracks_to_playlist(client, &uris, "39Fzt1Um2z7j8tPqe2f2aK").await; // Trip Around the World

    println!("{} => {}/{}", path.display(), count, tracks.len());
    count
}

pub async fn weedian_update_playlist_image(client: &Client, id: &str, path: &Path) -> Result<(), Box<dyn Error>> {
    // doesn't work
    let mut image_file = File::open(path)?;
    let mut image_data = Vec::new();
    image_file.read_to_end(&mut image_data)?;
    let image_base64 = base64::encode(&image_data);

    let mut response = client
	.put(format!("https://api.spotify.com/v1/playlists/{}/images", id))
	.json(&Image {base64: image_base64})
	.send().await?;
    Ok(())
}

pub async fn search_track(client: &Client, track: &str, artist: &str) -> Option<SimpleTrack> {
    let mut response = client.get("https://api.spotify.com/v1/search")
	.query(&[
	    ("q", format!("remaster%20track:{track}%20artist:{artist}").as_str()),
	    ("type", "track"),
	    ("market", "GB"),
	    ("limit", "50"),
	])
	.send().await.expect("bad send");

    println!("    Attempting: [{artist} - {track}]");
    match response.json::<SearchResponse<SimpleTrack>>().await {
	Ok(mut r) => {
	    r.tracks.items
		.iter()
		.filter_map(|track_response| {
		    if track_response.name.to_lowercase() == track.to_lowercase() && track_response.artist.to_lowercase() == artist.to_lowercase() {
			println!("        Found: [{} - {}]", track_response.artist, track_response.name);
			Some(track_response)
		    }
		    else {
			// println!("        No match: [{} - {}]", track_response.artist, track_response.name);
			None
		    }
		})
		.last().cloned()
	},
	Err(e) => {
	    println!("{:?}", e);
	    None
	}
    }
}

pub async fn add_tracks_to_playlist(client: &Client, uris: &[String], playlist_id: &str) {
    let mut response = client.post(format!("https://api.spotify.com/v1/playlists/{playlist_id}/tracks"))
	.body(format!("{{\"uris\": {:?}}}", uris))
	.send().await.expect("bad send");
}

pub async fn weedian_create_playlist(client: &Client, name: &str) -> Result<String, Box<dyn Error>> {
    let user: User = client.get("https://api.spotify.com/v1/me").send().await?.json().await?; // nikfisto
    let mut response: CreatePlaylist = client
	.post(format!("https://api.spotify.com/v1/users/{}/playlists", user.id))
        .body(format!("{{\"name\": \"{name}\", \"public\": false}}"))
	.send().await?
	.json().await?;
    Ok(response.id)
}

pub fn track_names(path: &Path) -> Vec<(String, String)> {
    let iter = path.read_dir().expect("read_dir call failed in track_names()");
    let mut tracks: Vec<(String, String)> = Vec::new();

    for entry in iter
	.filter_map(|file| file.ok())
	.filter(|entry| entry.path().extension().unwrap() == "flac") {
	    if let Some(entry) = entry.file_name().to_str() {
		let mut temp: Vec<&str> = entry.split(" - ").collect();
		let (track, _) = temp.pop().unwrap().rsplit_once('.').unwrap();
		let (_, band) = temp.pop().unwrap().split_at(3);
		// println!("    {band} - {track}");
		tracks.push((band.into(), track.into()));
	    }
    }
    tracks
}

pub fn albums(path: &Path) -> Vec<String> {
    let iter = path.read_dir().expect("read_dir call failed in albums()");
    let mut albums: Vec<String> = Vec::new();

    for entry in iter.filter_map(|dir| dir.ok()) {
	if let Some(entry) = entry.file_name().to_str() {
	    albums.push(entry.into());
	}
    }
    albums
}
