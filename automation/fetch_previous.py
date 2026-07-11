# /// script
# requires-python = ">=3.12"
# dependencies = [
#     "requests",
# ]
# ///
from pathlib import Path

import requests

# directory where this script lives
SCRIPT_DIR = Path(__file__).resolve().parent

# Aggregate ndjson of every completed round, maintained by the
# neofoodclub-previous-jsonl Lambda (one JSON object per line, sorted by
# round). Replaces fetching+caching individual rounds/{round}.json files one
# at a time - this is the whole history in a single request.
resp = requests.get("https://cdn.neofood.club/previous.jsonl")
resp.raise_for_status()

output_dir = SCRIPT_DIR / "output"
output_dir.mkdir(exist_ok=True)
(output_dir / "previous.jsonl").write_text(resp.text)
