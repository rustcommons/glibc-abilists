#!/usr/bin/env python3
"""Sign/verify release checksums using the fixed repository OpenPGP trust pin."""
from __future__ import annotations

import argparse
from contextlib import contextmanager
import os
from pathlib import Path
import re
import subprocess
import tempfile

PRIMARY_FINGERPRINT = 'B281C3010A76A11737B4D066C7396F6EA7647CC6'
SIGNING_SUBKEY = '248DFD639B75D1EFF2A87289D7C7DE7BBCCF35EC'
PUBLIC_KEY = Path(__file__).resolve().parents[1] / '.github/release-signing-key.asc'


def _environment(home: Path) -> dict[str, str]:
    env = os.environ.copy()
    env.pop('RELEASE_SIGNING_KEY', None)
    env.pop('RELEASE_SIGNING_PASSPHRASE', None)
    env.update(GNUPGHOME=str(home), LC_ALL='C')
    return env


@contextmanager
def _home():
    # mkdtemp is the stdlib equivalent of mktemp -d; permissions are explicit.
    with tempfile.TemporaryDirectory(prefix='release-gnupg-') as directory:
        home = Path(directory)
        home.chmod(0o700)
        try:
            yield home
        finally:
            result = subprocess.run(
                ['gpgconf', '--homedir', str(home), '--kill', 'gpg-agent'],
                env=_environment(home), capture_output=True)
            if result.returncode:
                raise RuntimeError('Could not stop the temporary GnuPG agent')
        # TemporaryDirectory removes the home even when signing/verification fails.


def _gpg(home: Path, *args: str, input: bytes | None = None) -> bytes:
    result = subprocess.run(
        ['gpg', '--no-options', '--homedir', str(home), '--batch', '--no-tty', *args],
        input=input, env=_environment(home), capture_output=True)
    if result.returncode:
        # Never echo secret import/signing input or put it in exception details.
        raise ValueError('GnuPG operation failed')
    return result.stdout


def _require_primary(home: Path, expected: str) -> None:
    listing = _gpg(home, '--with-colons', '--fingerprint', '--list-keys').decode()
    primary, fingerprints = False, []
    for line in listing.splitlines():
        fields = line.split(':')
        if fields[0] in ('pub', 'sub'):
            primary = fields[0] == 'pub'
        elif fields[0] == 'fpr' and primary:
            fingerprints.append(fields[9])
            primary = False
    if fingerprints != [expected]:
        raise ValueError('Public key does not match the pinned primary fingerprint')


def _verify_signature(checksums: Path, signature: Path, public_key: bytes,
                      expected_primary: str) -> None:
    with _home() as home:
        _gpg(home, '--import', input=public_key)
        _require_primary(home, expected_primary)
        status = _gpg(home, '--status-fd', '1', '--verify', str(signature), str(checksums)).decode()
        records = [line.split()[1:] for line in status.splitlines()
                   if line.startswith('[GNUPG:] ')]
        forbidden = {'REVKEYSIG', 'KEYREVOKED', 'BADSIG', 'ERRSIG', 'EXPSIG', 'NO_PUBKEY'}
        if any(record[0] in forbidden for record in records):
            raise ValueError('Signature has a revoked, bad, expired-signature or unknown-key status')
        good = [record for record in records if record[0] in ('GOODSIG', 'EXPKEYSIG')]
        valid = [record for record in records if record[0] == 'VALIDSIG']
        # VALIDSIG names the signing subkey first, and the primary key last.
        if len(good) != 1 or len(valid) != 1 or len(valid[0]) != 11 or valid[0][-1] != expected_primary:
            raise ValueError('Expected exactly one good signature by the pinned primary')
        # KEYEXPIRED can describe an unrelated sibling subkey.
        if any(record[0] == 'EXPKEYSIG' for record in records):
            signing_key, signed_at = valid[0][1], valid[0][3]
            listing = _gpg(home, '--with-colons', '--fingerprint', '--list-keys').decode()
            expiry, signing_expiry = '', None
            for line in listing.splitlines():
                fields = line.split(':')
                if fields[0] in ('pub', 'sub'):
                    expiry = fields[6]
                elif fields[0] == 'fpr' and fields[9] == signing_key:
                    signing_expiry = expiry
                    break
            if (not signed_at.isdigit() or not signing_expiry or not signing_expiry.isdigit()
                    or int(signing_expiry) <= 0 or int(signed_at) >= int(signing_expiry)):
                raise ValueError('Signature was not made before the signing key expired')


def _sign_and_verify(checksums: Path, signature: Path, secret_key: bytes,
                     passphrase: str, public_key: bytes, primary: str, subkey: str) -> None:
    # This private primitive accepts test fixtures. Production entry points below
    # always supply the committed key and constants, never CLI/env trust overrides.
    with _home() as signing_home:
        _gpg(signing_home, '--import', input=secret_key)
        _require_primary(signing_home, primary)
        _gpg(signing_home, '--pinentry-mode', 'loopback', '--passphrase-fd', '0',
             '--local-user', subkey + '!', '--armor', '--detach-sign',
             '--output', str(signature), str(checksums),
             input=(passphrase + '\n').encode())
        # A second, isolated home imports only the supplied public key.
        _verify_signature(checksums, signature, public_key, primary)


def check_assets(checksums: Path) -> None:
    lines = checksums.read_text(encoding='utf-8').splitlines()
    names = []
    for line in lines:
        match = re.fullmatch(r'[0-9a-fA-F]{64} [ *]([^/\\\r\n]+)', line)
        if not match or match[1] in ('.', '..'):
            raise ValueError('Checksum entries must name files in the asset directory')
        names.append(match[1])
    if not names or len(names) != len(set(names)):
        raise ValueError('Checksum list must be nonempty and contain no duplicate names')
    result = subprocess.run(['sha256sum', '--check', checksums.name], cwd=checksums.parent,
                            capture_output=True, text=True)
    if result.returncode:
        raise ValueError('Release asset checksum verification failed')
    print(result.stdout, end='')


def sign_release(checksums: Path) -> None:
    secret = os.environ.pop('RELEASE_SIGNING_KEY', None)
    passphrase = os.environ.pop('RELEASE_SIGNING_PASSPHRASE', None)
    if not secret or passphrase is None:
        raise ValueError('Release signing environment secrets are required')
    check_assets(checksums)
    _sign_and_verify(checksums, checksums.with_name(checksums.name + '.asc'),
                     secret.encode(), passphrase, PUBLIC_KEY.read_bytes(),
                     PRIMARY_FINGERPRINT, SIGNING_SUBKEY)


def verify_release(checksums: Path, signature: Path) -> None:
    _verify_signature(checksums, signature, PUBLIC_KEY.read_bytes(), PRIMARY_FINGERPRINT)
    check_assets(checksums)


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('mode', choices=['sign', 'verify'])
    parser.add_argument('--checksums', type=Path, required=True)
    args = parser.parse_args()
    checksums = args.checksums.resolve()
    if args.mode == 'sign':
        sign_release(checksums)
    else:
        verify_release(checksums, checksums.with_name(checksums.name + '.asc'))
    print(f'Release checksum {args.mode} succeeded; primary {PRIMARY_FINGERPRINT}')


if __name__ == '__main__':
    main()
