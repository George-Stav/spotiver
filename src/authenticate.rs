use std::{
    collections::HashMap,
    env
};
use chrono::{DateTime, Utc};
use dotenv::dotenv;
use serde::{Serialize, Deserialize, de::Deserializer};
use reqwest::header::{HeaderMap, AUTHORIZATION, CONTENT_TYPE};

pub async fn token() -> Result<String, Box<dyn std::error::Error>> {
    // Public facing function that returns the access token in String format.
    // Attempts to read it from file and refreshes it if one of the following conditions is met:
    //      - File could not be read properly (e.g. NotFound)
    //      - Key in file has expired
    let token: Token = match std::fs::read_to_string("../token.json") {
        Ok(s) => { // File was read successfully
            let temp_token: Token = serde_json::from_str::<Token>(&s)?;
            let now = Utc::now().timestamp();

            if temp_token.expires_at <= now {
                // Refresh if expired
                refresh().await?
            } else {
                // Otherwise, return it
                temp_token
            }
        },
        // File could not be read
        // Refresh token
        Err(_) => refresh().await?
    };

    Ok(token.access_token)
}

async fn refresh() -> Result<Token, Box<dyn std::error::Error>> {
    // Refresh the token using the REFRESH_TOKEN found in .env
    // Code derived from Spotify Web API docs:
    // https://developer.spotify.com/documentation/general/guides/authorization/code-flow
    dotenv().ok();

    let client_id = env::var("CLIENT_ID")?;
    let client_secret = env::var("CLIENT_SECRET")?;
    let refresh_token = env::var("REFRESH_TOKEN")?;

    let secrets = base64::encode(format!("{client_id}:{client_secret}"));

    let mut headers = HeaderMap::new();
    headers.insert(AUTHORIZATION, format!("Basic {secrets}").parse().unwrap());
    headers.insert(CONTENT_TYPE, "application/x-www-form-urlencoded".parse().unwrap());

    let mut params = HashMap::new();
    params.insert("grant_type", "refresh_token");
    params.insert("refresh_token", &refresh_token);

    let mut response: Token = reqwest::Client::new()
        .post("https://accounts.spotify.com/api/token")
        .headers(headers)
        .form(&params)
        .send()
        .await?
        .json()
        .await?;

    // Add number of seconds until new token expires to the current UNIX timestamp
    // to create the new expiration timestamp
    response.expires_at += Utc::now().timestamp();

    // Save new token to file
    // Overwrite existing file if needed
    std::fs::write(
        "../token.json",
        serde_json::to_string_pretty(&response).unwrap()
    ).expect("Unable to write file");

    Ok(response)
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Token {
    access_token: String,
    #[serde(rename="expires_in")]
    expires_at: i64,
}
