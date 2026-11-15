#!/usr/bin/env python3
"""Compare a local package, crates.io metadata and the official download."""
import argparse
import hashlib
import json
from pathlib import Path
import re
import tomllib
import urllib.request


def fetch(url):
    request = urllib.request.Request(url, headers={"User-Agent": "glibc-abilists-release-check"})
    with urllib.request.urlopen(request, timeout=60) as response:
        return response.read()


def verify(version, archive):
    if not re.fullmatch(r"[0-9]+\.[0-9]+\.[0-9]+(?:-[0-9A-Za-z.-]+)?", version):
        raise ValueError("Invalid release version")
    local = hashlib.sha256(archive.read_bytes()).hexdigest()
    url = f"https://crates.io/api/v1/crates/glibc-abilists/{version}"
    metadata = json.loads(fetch(url))["version"]
    if metadata["num"] != version or metadata["yanked"]:
        raise ValueError("Registry version differs or is yanked")
    official = hashlib.sha256(fetch(url + "/download")).hexdigest()
    if local != official or local != metadata["checksum"]:
        raise ValueError("Local package, official archive and registry checksum disagree")
    print(f"glibc-abilists {version}: local/archive/registry SHA-256 {local}")


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--version", help="Defaults to the root manifest version")
    parser.add_argument("--archive", type=Path, help="Defaults to target/package/<crate>-<version>.crate")
    args = parser.parse_args()
    version = args.version
    if version is None:
        with open("Cargo.toml", "rb") as source:
            version = tomllib.load(source)["package"]["version"]
    archive = args.archive or Path(f"target/package/glibc-abilists-{version}.crate")
    verify(version, archive)


if __name__ == "__main__":
    main()
