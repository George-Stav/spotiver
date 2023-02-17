#!/usr/bin/env sh

source "../.env"

token=$(echo -n "$CLIENT_ID:$CLIENT_SECRET" | base64 --wrap=100)
echo "$token"
curl --request GET \
    -H "Authorization: Bearer $token" \
    -H "Content-Type: application/json" \
    "https://api.spotify.com/v1/playlists/4ZBu3Yz2pzW5zY7n1dRZXg/tracks"
