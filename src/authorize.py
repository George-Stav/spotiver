#!/usr/bin/env python3

from urllib.parse import urlencode
from dotenv import load_dotenv
from selenium import webdriver

import requests
import os
import base64
import string
import random

def random_state(length=16):
    return ''.join([random.choice(string.ascii_lowercase + string.digits) for _ in range(16)])

if not load_dotenv():
    raise Exception("[ERROR]: .env not loaded")

base_url = "https://accounts.spotify.com"
state = random_state()
auth_headers = {"client_id": os.getenv('CLIENT_ID'),
                "response_type": "code",
                "redirect_uri": "http://localhost:8888",
                "scope": os.getenv('SCOPE'),
                "state": state}

driver = webdriver.Firefox()
driver.get(f"{base_url}/authorize?" + urlencode(auth_headers))

input("Press Enter when done...")

url = driver.current_url
driver.quit()

new_state = url.split("=")[2]
code = url.split("=")[1].rstrip("&state")

if state != new_state:
    raise Exception(f"[ERROR]: Potential cross-site request detected => {state}!={new_state}")

encoded_credentials = base64.b64encode(os.getenv('CLIENT_ID').encode() + b':' + os.getenv('CLIENT_SECRET').encode()).decode('utf-8')
token_headers = {"Authorization": "Basic " + encoded_credentials,
                 "Content-Type": "application/x-www-form-urlencoded"}
token_data = {"grant_type": "authorization_code",
              "code": code,
              "redirect_uri": os.getenv('REDIRECT_URI')}

r = requests.post(f"{base_url}/api/token", data=token_data, headers=token_headers)

new_rft = r.json()['refresh_token']
old_rft = os.getenv("REFRESH_TOKEN")

with open(".env", "r+") as f:
    data = f.read()
    print(data)
    f.seek(0)
    f.write(data.replace(old_rft, new_rft))
    f.truncate()

print("\n[INFO]: Placed new Refresh Token in .env")
