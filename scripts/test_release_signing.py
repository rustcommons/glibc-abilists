#!/usr/bin/env python3
"""Bounded GnuPG tests: committed pin plus ephemeral keys only; no real secrets."""
import hashlib
import os
from pathlib import Path
import subprocess
import tempfile
import unittest
from unittest.mock import patch

import release_signing as signing


class SigningTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        context = signing._home()
        cls.home = context.__enter__()
        cls.addClassCleanup(context.__exit__, None, None, None)
        cls.passphrase = 'ephemeral-test-passphrase'
        options = ('--pinentry-mode', 'loopback', '--passphrase-fd', '0')
        signing._gpg(cls.home, *options, '--quick-generate-key',
                     'Ephemeral Release Test <release@example.invalid>', 'rsa2048', 'cert', '1d',
                     input=(cls.passphrase + '\n').encode())
        listing = signing._gpg(cls.home, '--with-colons', '--fingerprint', '--list-keys').decode()
        cls.primary = next(line.split(':')[9] for line in listing.splitlines()
                           if line.startswith('fpr:'))
        signing._gpg(cls.home, *options, '--quick-add-key', cls.primary, 'rsa2048', 'sign', '1d',
                     input=(cls.passphrase + '\n').encode())
        listing = signing._gpg(cls.home, '--with-colons', '--fingerprint', '--list-keys').decode()
        cls.subkey = [line.split(':')[9] for line in listing.splitlines()
                      if line.startswith('fpr:')][-1]
        cls.public = signing._gpg(cls.home, '--armor', '--export', cls.primary)
        cls.secret = signing._gpg(cls.home, *options, '--armor', '--export-secret-subkeys',
                                  cls.primary, input=(cls.passphrase + '\n').encode())

    def setUp(self):
        self.temp = tempfile.TemporaryDirectory(prefix='release-signing-test-')
        self.addCleanup(self.temp.cleanup)
        self.root = Path(self.temp.name)
        asset = self.root / 'fixture.crate'
        asset.write_bytes(b'ephemeral release fixture')
        self.checksums = self.root / 'SHA256SUMS'
        self.checksums.write_text(hashlib.sha256(asset.read_bytes()).hexdigest() + '  fixture.crate\n')
        self.signature = self.root / 'SHA256SUMS.asc'

    def sign_fixture(self, passphrase=None):
        signing._sign_and_verify(self.checksums, self.signature, self.secret,
                                 self.passphrase if passphrase is None else passphrase,
                                 self.public, self.primary, self.subkey)

    def test_committed_public_key_has_only_the_pinned_primary(self):
        self.assertEqual(signing.PRIMARY_FINGERPRINT, 'B281C3010A76A11737B4D066C7396F6EA7647CC6')
        with signing._home() as home:
            self.assertEqual(home.stat().st_mode & 0o777, 0o700)
            signing._gpg(home, '--import', input=signing.PUBLIC_KEY.read_bytes())
            signing._require_primary(home, signing.PRIMARY_FINGERPRINT)
        self.assertFalse(home.exists())

    def test_ephemeral_roundtrip_and_all_asset_checks(self):
        self.sign_fixture()
        signing._verify_signature(self.checksums, self.signature, self.public, self.primary)
        signing.check_assets(self.checksums)
        (self.root / 'fixture.crate').write_bytes(b'changed asset')
        with self.assertRaises(ValueError):
            signing.check_assets(self.checksums)

    def test_tampered_checksum_list_is_rejected(self):
        self.sign_fixture()
        self.checksums.write_text('0' * 64 + '  fixture.crate\n')
        with self.assertRaises(ValueError):
            signing._verify_signature(self.checksums, self.signature, self.public, self.primary)

    def test_wrong_passphrase_fails_and_temporary_homes_are_removed(self):
        original = signing._home
        homes = []
        from contextlib import contextmanager
        @contextmanager
        def tracked():
            with original() as home:
                homes.append(home)
                yield home
        with patch.object(signing, '_home', tracked), self.assertRaises(ValueError):
            self.sign_fixture('wrong-test-passphrase')
        self.assertTrue(homes)
        self.assertTrue(all(not home.exists() for home in homes))

    def test_validsig_primary_is_checked_independently(self):
        self.sign_fixture()
        original = signing._gpg
        def altered_status(home, *args, **kwargs):
            result = original(home, *args, **kwargs)
            if args[:2] == ('--status-fd', '1'):
                return result.replace(self.primary.encode(), b'0' * 40)
            return result
        with patch.object(signing, '_gpg', side_effect=altered_status), self.assertRaises(ValueError):
            signing._verify_signature(self.checksums, self.signature, self.public, self.primary)

    def test_wrong_trust_pin_is_rejected(self):
        self.sign_fixture()
        with self.assertRaises(ValueError):
            signing._verify_signature(self.checksums, self.signature, self.public,
                                      signing.PRIMARY_FINGERPRINT)

    def test_production_entry_points_cannot_use_test_key_or_env_pin(self):
        self.sign_fixture()
        with patch.dict(os.environ, {'RELEASE_SIGNING_PRIMARY_FINGERPRINT': self.primary}):
            with self.assertRaises(ValueError):
                signing.verify_release(self.checksums, self.signature)
        self.signature.unlink()
        with patch.dict(os.environ, {'RELEASE_SIGNING_KEY': self.secret.decode(),
                                     'RELEASE_SIGNING_PASSPHRASE': self.passphrase}), self.assertRaises(ValueError):
            signing.sign_release(self.checksums)
        self.assertFalse(self.signature.exists())
        result = subprocess.run(['python3', str(Path(signing.__file__)), 'verify',
                                 '--checksums', str(self.checksums), '--primary', self.primary],
                                capture_output=True)
        self.assertNotEqual(result.returncode, 0)


if __name__ == '__main__':
    unittest.main()
