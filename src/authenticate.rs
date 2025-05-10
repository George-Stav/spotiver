use std::{
    error::Error,
    collections::HashMap,
};
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize, de::Deserializer};
use reqwest::{
    header::{HeaderMap, AUTHORIZATION, CONTENT_TYPE},
    Client
};

pub async fn token() -> Result<String, Box<dyn Error>> {
    // Public facing function that returns the access token in String format.
    // Attempts to read it from file and refreshes it if one of the following conditions is met:
    //      - File could not be read properly (e.g. NotFound)
    //      - Key in file has expired
    let project_root = dotenv::var("PROJECT_ROOT").expect("PROJECT_ROOT should be present in .env");
    let token: Token = match std::fs::read_to_string(format!("{project_root}/data/token.json")) {
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

pub async fn refresh() -> Result<Token, Box<dyn Error>> {
    /* Refresh the access token using the REFRESH_TOKEN found in .env
       Code derived from Spotify Web API docs:
       - https://developer.spotify.com/documentation/general/guides/authorization/code-flow
       - https://developer.spotify.com/documentation/web-api/tutorials/refreshing-tokens
    */

    let project_root = dotenv::var("PROJECT_ROOT").expect("PROJECT_ROOT should be present in .env");
    let client_id = dotenv::var("CLIENT_ID").expect("CLIENT_ID should be present in .env");
    let client_secret = dotenv::var("CLIENT_SECRET").expect("CLIENT_SECRET should be present in .env");
    let refresh_token = dotenv::var("REFRESH_TOKEN").expect("REFRESH_TOKEN should be present in .env");

    let secrets = base64::encode(format!("{client_id}:{client_secret}"));

    let mut headers = HeaderMap::new();
    headers.insert(AUTHORIZATION, format!("Basic {secrets}").parse().unwrap());
    headers.insert(CONTENT_TYPE, "application/x-www-form-urlencoded".parse().unwrap());

    let mut params = HashMap::new();
    params.insert("grant_type", "refresh_token");
    params.insert("refresh_token", &refresh_token);

    let mut token: Token = Client::new()
        .post("https://accounts.spotify.com/api/token")
        .headers(headers)
        .form(&params)
        .send().await?
        .json().await?;

    // Add number of seconds until new token expires to the current UNIX timestamp
    // to create the new expiration timestamp
    token.expires_at += Utc::now().timestamp();

    // Save new token to file
    // Overwrite existing file if needed
    std::fs::write(
        format!("{project_root}/data/token.json"),
        serde_json::to_string_pretty(&token).unwrap()
    ).expect("Unable to write file");

    Ok(token)
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Token {
    access_token: String,
    #[serde(rename="expires_in")]
    expires_at: i64,
}
