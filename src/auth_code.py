#!/usr/bin/env python3

from urllib.parse import urlencode
from dotenv import load_dotenv

import requests
import webbrowser
import os
import base64

load_dotenv()

base_url = "https://accounts.spotify.com"
auth_headers = {"client_id": os.getenv('CLIENT_ID'),
                "response_type": "code",
                "redirect_uri": "http://localhost:8888",
                "scope": os.getenv('SCOPE')}

webbrowser.open(f"{base_url}/authorize?" + urlencode(auth_headers))

code = input("Code: ")
encoded_credentials = base64.b64encode(os.getenv('CLIENT_ID').encode() + b':' + os.getenv('CLIENT_SECRET').encode()).decode('utf-8')
token_headers = {"Authorization": "Basic " + encoded_credentials,
                 "Content-Type": "application/x-www-form-urlencoded"}
token_data = {"grant_type": "authorization_code",
              "code": code,
              "redirect_uri": os.getenv('REDIRECT_URI')}

r = requests.post(f"{base_url}/api/token", data=token_data, headers=token_headers)
print(f"[{r.status_code}] Refresh Token: {r.json()['refresh_token']}")
