mod objects;

use crate::objects::{
    playlist::Playlist,
    track::Track
};

use chrono::DateTime;
use std::path::Path;
use std::fs;

fn main() {
    let bkp_path = Path::new("/home/george/BULK/spotiver.bkp");
    let playlists = pl_json(&bkp_path).unwrap();
    let total = playlists.len();

    // println!("{:?}", get_pl_creation_date(&tracks));
    for (idx, playlist) in playlists.iter().enumerate() {
	let pid = playlist.id.as_str();

	if let Some(tracks) = track_json(&bkp_path, pid).ok() {
	    let datetime = get_pl_creation_date(&tracks);
	    println!("{:?} <= [{}/{total} | {}]", datetime, idx+1, playlist.name);
	}
    }
}

fn get_pl_creation_date(tracks: &[Track]) -> Option<String> {
    let mut dates = tracks
	.iter()
	.filter_map(|track| DateTime::parse_from_str(track.added_at.as_str(), "%+").ok())
	.collect::<Vec<DateTime<_>>>();

    dates.sort();

    dates.first().map(|datetime| datetime.to_string())
}

fn track_json(location: &Path, playlist_id: &str) -> Result<Vec<Track>, Box<dyn std::error::Error>> {
    let tracks_path = location.join(playlist_id).as_path().join("tracks.json");
    let file_content = fs::read_to_string(&tracks_path)
        .map_err(|e| {
            println!("[ERROR]: Couldn't read file [{:?}]: {}", tracks_path, e);
            e
        })?;

    let tracks: Vec<Track> = serde_json::from_str(&file_content)
        .map_err(|e| {
            println!("[ERROR]: Couldn't deserialise object in file [{:?}]: {}", tracks_path, e);
            e
        }).unwrap();

    Ok(tracks)
}


fn pl_json(location: &Path) -> Result<Vec<Playlist>, ()> {
    let playlists_path = location.join("playlists.json");
    let file_content = fs::read_to_string(&playlists_path)
        .map_err(|e| {
            println!("[ERROR]: Couldn't read file [{:?}]: {}", playlists_path, e);
            e
        }).unwrap();

    let playlists: Vec<Playlist> = serde_json::from_str(&file_content)
        .map_err(|e| {
            println!("[ERROR]: Couldn't deserialise object in file [{:?}]: {}", playlists_path, e);
            e
        }).unwrap();

    Ok(playlists)
}
