#![allow(dead_code)]
#![allow(unused)]

mod authenticate;
use authenticate as auth;

use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use serde_json::Value;
use reqwest::header::{HeaderMap, AUTHORIZATION, CONTENT_TYPE};

#[tokio::main]
// async fn main() -> Result<(), reqwest::Error> {
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let token = auth::refresh().await?;
    println!("{:?}", token);
    playlists(token).await?;
    Ok(())
}

async fn playlists(token: String) -> Result<(), Box<dyn std::error::Error>> {
    let mut headers = HeaderMap::new();
    headers.insert(AUTHORIZATION, format!("Bearer {}", token).parse().unwrap());
    headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());

    let response = reqwest::Client::new()
        .get("https://api.spotify.com/v1/me/playlists")
        .headers(headers)
        .query(&[("offset", 0), ("limit", 50)])
        .send()
        .await?;
        // .json()
        // .await?;

    println!("{:#?}", response.text().await?);
    Ok(())
}

#[derive(Debug, Deserialize)]
struct Account {
    country: String,
    display_name: String,
    // email: String,
    #[serde(flatten)]
    explicit_content: HashMap<String, Value>,
    #[serde(flatten)]
    external_urls: HashMap<String, Value>,
    #[serde(flatten)]
    followers: HashMap<String, Value>,
    href: String,
    id: String,
    product: String,
    #[serde(rename="type")]
    _type: String,
    uri: String,
}

#[derive(Debug, Deserialize)]
struct Playlists {
    items: Vec<Playlists>
}

#[derive(Debug, Deserialize)]
struct Playlist {
    collaborative: bool,
    description: String,
    #[serde(flatten)]
    external_urls: HashMap<String, String>
}






#[derive(Debug, Serialize, Deserialize)]
struct Todo {
    #[serde(rename = "userId")]
    user_id: i32,
    id: Option<i32>,
    title: String,
    completed: bool,
}

async fn post_json() -> Result<(), reqwest::Error> {
    let todo = Todo {
        user_id: 1,
        id: None,
        title: "This is the title".to_owned(),
        completed: false
    };

    let todo: Todo = reqwest::Client::new()
        .post("https://jsonplaceholder.typicode.com/todos")
        .json(&todo)
        // .json(&serde_json::json!({
        //     "userId": 1,
        //     "title": "This is the title".to_owned(),
        //     "completed": false
        // }))
        .send()
        .await?
        .json()
        .await?;

    println!("{:#?}", todo);
    Ok(())
}

async fn get_json() -> Result<(), reqwest::Error> {
    let todos: Vec<Todo> = reqwest::Client::new()
        .get("https://jsonplaceholder.typicode.com/todos?userId=1")
        .send()
        .await?
        .json()
        .await?;

    println!("{:#?}", todos);
    Ok(())
}
