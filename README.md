# Spotiver

Spot(ify) (arch)iver. A program written in Rust that scrapes your spotify account's structure for backup purposes.

## Authentication

This app uses Spotify's [Authorization Code Flow](https://developer.spotify.com/documentation/general/guides/authorization/code-flow/) and assumes that a `.env` file exists with the appropriate values already populated:
- `client_id`
- `client_secret`
- `refresh_token`

A `.env.example` file is provided in order to show how to structure your actual `.env` file.
