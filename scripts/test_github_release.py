#!/usr/bin/env python3
"""Offline release-state/asset checks; no GitHub release is created by tests."""
import copy
import hashlib
import json
from pathlib import Path
import tempfile
import unittest
from unittest.mock import patch

import github_release as releases


class ReleaseTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory(prefix='release-assets-test-')
        self.addCleanup(self.temp.cleanup)
        self.root = Path(self.temp.name)
        names = ['glibc-abilists-1.2.3.crate', 'glibc-abilists-1.2.3.cdx.json',
                 'SHA256SUMS', 'SHA256SUMS.asc', 'glibc-abilists-1.2.3.crates.intoto.jsonl']
        for name in names:
            (self.root / name).write_text('fixture ' + name)
        self.notes = self.root / 'notes.md'
        self.notes.write_text('# Release 1.2.3\n\nReviewed notes.\n')
        self.files = releases.release_files(self.root, '1.2.3')
        self.release = dict(id=123, tag_name='v1.2.3', name='Release 1.2.3',
                            target_commitish='a' * 40, body=self.notes.read_text(),
                            draft=True, prerelease=False, assets=[],
                            upload_url='https://uploads.github.com/repos/test/repo/releases/123/assets{?name,label}')

    def asset(self, name):
        p = self.files[name]
        return dict(name=name, size=p.stat().st_size,
                    digest='sha256:' + hashlib.sha256(p.read_bytes()).hexdigest())

    def call(self):
        releases.publish_release('test/repo', '1.2.3', 'a' * 40, self.notes, self.root)

    def test_signature_is_a_required_asset(self):
        (self.root / 'SHA256SUMS.asc').unlink()
        with self.assertRaises(ValueError):
            releases.release_files(self.root, '1.2.3')

    def test_asset_size_and_digest_are_both_enforced(self):
        for field, value in [('size', 0), ('digest', 'sha256:' + '0' * 64)]:
            asset = self.asset('SHA256SUMS.asc')
            asset[field] = value
            with self.assertRaises(ValueError):
                releases.asset_matches(asset, self.files['SHA256SUMS.asc'])

    def test_published_release_is_read_only(self):
        self.release['draft'] = False
        self.release['assets'] = [self.asset(name) for name in self.files]
        with patch.object(releases, 'gh', return_value=json.dumps([[self.release]])) as gh:
            self.call()
            self.assertEqual(gh.call_count, 1)
        self.release['assets'].pop()
        with patch.object(releases, 'gh', return_value=json.dumps([[self.release]])) as gh:
            with self.assertRaises(ValueError):
                self.call()
            self.assertEqual(gh.call_count, 1)

    def test_existing_asset_or_notes_mismatch_stops_before_writes(self):
        for bad_notes in [False, True]:
            record = copy.deepcopy(self.release)
            record['assets'] = [self.asset('SHA256SUMS.asc')]
            if bad_notes:
                record['body'] = 'unreviewed body'
            else:
                record['assets'][0]['digest'] = 'sha256:' + '0' * 64
            with patch.object(releases, 'gh', return_value=json.dumps([[record]])) as gh:
                with self.assertRaises(ValueError):
                    self.call()
                self.assertEqual(gh.call_count, 1)

    def test_create_or_resume_draft_checks_all_assets_before_publication(self):
        for resume in [False, True]:
            state = copy.deepcopy(self.release)
            state['assets'] = [self.asset('SHA256SUMS.asc')] if resume else []
            exists = resume
            def fake(*args):
                nonlocal exists
                if args[:2] == ('release', 'create'):
                    self.assertIn('--draft', args)
                    self.assertIn('--verify-tag', args)
                    exists = True
                    return 'draft URL'
                if '--paginate' in args:
                    return json.dumps([[state] if exists else []])
                if 'POST' in args:
                    name = Path(args[args.index('--input') + 1]).name
                    self.assertNotIn(name, [a['name'] for a in state['assets']])
                    asset = self.asset(name)
                    state['assets'].append(asset)
                    return json.dumps(asset)
                if 'PATCH' in args:
                    self.assertEqual({a['name'] for a in state['assets']}, set(self.files))
                    state['draft'] = False
                return json.dumps(state)
            with patch.object(releases, 'gh', side_effect=fake):
                self.call()
            self.assertFalse(state['draft'])


if __name__ == '__main__':
    unittest.main()
