import os
from dotenv import load_dotenv
import subprocess

# Load the .env file
load_dotenv()

# Get the session token from the environment variable
session_token = os.getenv("SESSION")

if not session_token:
    raise ValueError("SESSION token is not set in .env file")

# Base URL for the Advent of Code inputs
base_url = "https://adventofcode.com/2024/day/{}/input"

# Headers
headers = [
    "accept: text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.7",
    "accept-language: en-US,en-DE;q=0.9,en;q=0.8,de-DE;q=0.7,de;q=0.6",
    "cache-control: max-age=0",
    f"cookie: session={session_token}",
    "priority: u=0, i",
    "referer: https://adventofcode.com/2024/day/{day}",
    'sec-ch-ua: "Google Chrome";v="131", "Chromium";v="131", "Not_A Brand";v="24"',
    "sec-ch-ua-mobile: ?0",
    'sec-ch-ua-platform: "Windows"',
    "sec-fetch-dest: document",
    "sec-fetch-mode: navigate",
    "sec-fetch-site: same-origin",
    "sec-fetch-user: ?1",
    "upgrade-insecure-requests: 1",
    "user-agent: Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36"
]

def download_day(day: int):
    url = base_url.format(day)
    output_file = f"src/day{day}/input.txt"
    header_str = " ".join([f"-H \"{header}\"" for header in headers])
    curl_command = f'curl "{url}" {header_str} -o {output_file}'

    # Execute the curl command
    subprocess.run(curl_command, shell=True)
    print(f"Downloaded input for day {day} to {output_file}")

user_input = input("Enter day:")
if user_input.isdigit():
    download_day(int(user_input))
    exit(0)

# Download input for each day
for day in range(1, 25):
    download_day(day)
