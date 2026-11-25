#!/usr/bin/env python3
"""Create/resume a draft release; validate assets before publishing it once."""
import argparse
import hashlib
import json
from pathlib import Path
import subprocess
from urllib.parse import urlencode


def gh(*args):
    return subprocess.check_output(['gh', *args], text=True)


def metadata(release, tag, title, body, commit):
    if (release['tag_name'], release['name'], release['target_commitish']) != (tag, title, commit):
        raise ValueError('Release tag/title/target differs from the candidate')
    if release.get('prerelease') or release['body'].replace('\r\n', '\n').strip() != body.strip():
        raise ValueError('Release notes or prerelease state differs from the candidate')


def asset_matches(asset, path):
    expected = 'sha256:' + hashlib.sha256(path.read_bytes()).hexdigest()
    if asset['name'] != path.name or asset['size'] != path.stat().st_size or asset.get('digest') != expected:
        raise ValueError(f'Release asset differs from candidate: {path.name}')


def release_files(dist, version):
    crates, sboms = sorted(dist.glob('*.crate')), sorted(dist.glob('*.cdx.json'))
    if not crates or not sboms:
        raise ValueError('Release crate/SBOM files are required')
    files = crates + sboms + [dist / 'SHA256SUMS', dist / 'SHA256SUMS.asc',
                             dist / f'glibc-abilists-{version}.crates.intoto.jsonl']
    if not all(path.is_file() for path in files):
        raise ValueError('A required release asset is missing')
    return {path.name: path for path in files}


def publish_release(repo, version, commit, notes, dist):
    tag = 'v' + version
    body = notes.read_text(encoding='utf-8')
    title = next(line[2:] for line in body.splitlines() if line.startswith('# '))
    files = release_files(dist, version)
    endpoint = f'repos/{repo}/releases'
    pages = json.loads(gh('api', '--paginate', '--slurp', endpoint + '?per_page=100'))
    matches = [item for page in pages for item in page if item['tag_name'] == tag]
    if len(matches) > 1:
        raise ValueError('Multiple releases have the candidate tag')
    if matches:
        release = matches[0]
    else:
        gh('release', 'create', tag, '--repo', repo, '--draft', '--verify-tag',
           '--target', commit, '--title', title, '--notes-file', str(notes))
        # Discover drafts through the list, not the tags API. A failed attempt
        # leaves a draft that a rerun can resume; this is not a polling loop.
        pages = json.loads(gh('api', '--paginate', '--slurp', endpoint + '?per_page=100'))
        matches = [item for page in pages for item in page if item['tag_name'] == tag]
        if len(matches) != 1:
            raise ValueError('Created draft is not uniquely visible; rerun after investigation')
        release = matches[0]
    metadata(release, tag, title, body, commit)
    assets = {asset['name']: asset for asset in release['assets']}
    if len(assets) != len(release['assets']) or set(assets) - files.keys():
        raise ValueError('Release has duplicate or unexpected assets')
    # Validate all existing assets before any new upload; never replace one.
    for name, asset in assets.items():
        asset_matches(asset, files[name])
    missing = files.keys() - assets.keys()
    if not release['draft']:
        if missing:
            raise ValueError('Published release is incomplete; it will not be modified')
        print(f'{tag}: published release already matches; no modification')
        return
    upload = release['upload_url'].split('{', 1)[0]
    if upload != f'https://uploads.github.com/repos/{repo}/releases/{release["id"]}/assets':
        raise ValueError('Unexpected asset upload endpoint')
    for name in sorted(missing):
        asset = json.loads(gh('api', '--method', 'POST', upload + '?' + urlencode({'name': name}),
                              '--header', 'Content-Type: application/octet-stream',
                              '--input', str(files[name])))
        asset_matches(asset, files[name])
    release = json.loads(gh('api', endpoint + '/' + str(release['id'])))
    metadata(release, tag, title, body, commit)
    if not release['draft'] or {asset['name'] for asset in release['assets']} != files.keys():
        raise ValueError('Draft state/asset set changed before publication')
    for asset in release['assets']:
        asset_matches(asset, files[asset['name']])
    release = json.loads(gh('api', '--method', 'PATCH', endpoint + '/' + str(release['id']),
                            '-F', 'draft=false'))
    metadata(release, tag, title, body, commit)
    if release['draft'] or {asset['name'] for asset in release['assets']} != files.keys():
        raise ValueError('Published state/asset set differs from candidate')
    for asset in release['assets']:
        asset_matches(asset, files[asset['name']])
    print(f'{tag}: published with checked immutable assets')


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--repo', required=True)
    parser.add_argument('--version', required=True)
    parser.add_argument('--commit', required=True)
    parser.add_argument('--notes', type=Path, required=True)
    parser.add_argument('--dist', type=Path, required=True)
    args = parser.parse_args()
    publish_release(args.repo, args.version, args.commit, args.notes, args.dist)
