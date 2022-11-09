use std::{
    collections::HashMap,
    env
};
use dotenv::dotenv;
use serde::{Serialize, Deserialize};
use reqwest::header::{HeaderMap, AUTHORIZATION, CONTENT_TYPE};

// pub fn get_token() -> Result<String, Box<dyn std::error::Error>> {
//     dotenv().ok;
// }

pub async fn refresh() -> Result<String, Box<dyn std::error::Error>> {
    dotenv().ok();

    let client_id = env::var("CLIENT_ID")?;
    let client_secret = env::var("CLIENT_SECRET")?;
    let refresh_token = env::var("REFRESH_TOKEN")?;

    let secrets = base64::encode(format!("{}:{}", client_id, client_secret));

    let mut headers = HeaderMap::new();
    headers.insert(AUTHORIZATION, format!("Basic {}", secrets).parse().unwrap());
    headers.insert(CONTENT_TYPE, "application/x-www-form-urlencoded".parse().unwrap());

    let mut params = HashMap::new();
    params.insert("grant_type", "refresh_token");
    params.insert("refresh_token", &refresh_token);

    let response: Token = reqwest::Client::new()
        .post("https://accounts.spotify.com/api/token")
        .headers(headers)
        .form(&params)
        .send()
        .await?
        .json()
        .await?;
    println!("{:?}", response);

    // headers.clear();
    // let new_response = reqwest::Client::new()
    //     .post("https://accounts.spotify.com/api/token")
    //     .headers(headers)
    //     .form(&params)
    //     .send()
    //     .await?
    //     .json()
    //     .await?;
    // println!("{:#?}", new_response);

    Ok(response.access_token)
}

#[derive(Debug, Deserialize)]
struct Token {
    access_token: String,
    token_type: String,
    expires_in: i32,
}
