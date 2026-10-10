# Provenance

What the dataset records, how each item is established, and what it does and
does not prove.

## Per release (`data/releases.tsv`, `glibc_abilists::Release`)

| Field | How it is established |
| --- | --- |
| `tag`, `tag_object` | The annotated tag in the local clone. The same object id is required at `https://sourceware.org/git/glibc.git` for every tag, and at each mirror in `upstream.toml` that has the tag (`git ls-remote`). |
| `commit`, `commit_date` | The commit the tag points to. Git object ids chain every file to it. |
| `tagger`, `tag_date` | From the tag object. |
| `signature` | `git verify-tag` with only the keys in `keys/`. See below. |
| `signing_key`, `primary_key` | The (sub)key that made the signature and its primary key. |
| `key_source_or_reason` | Where the key was obtained (`keys/keys.tsv`), or the reason for an exception. |
| `license_path`, `license_blob`, `license_sha256`, `license_spdx` | glibc's README must say "See the file COPYING.LIB for copying conditions"; `COPYING.LIB` is resolved through in-tree links and must be the GNU LGPL version 2.1. |

### Signatures

| Result | Meaning | Releases |
| --- | --- | --- |
| `verified` | Good signature; the key is valid today. | 2.19, 2.20, 2.23, 2.25, 2.26, 2.29, 2.31, 2.33, 2.38-2.44 |
| `verified-key-expired-later` | Good signature made **before** the key expired. A signature made after expiry fails. | 2.17, 2.18, 2.21, 2.22, 2.24, 2.27, 2.28, 2.32, 2.34-2.37 |
| `exception` | Signed, but the public key is not available; listed with its reason in `keys/exceptions.tsv`. | 2.30 |

A bad signature, a revoked key (`REVKEYSIG`), an expired signature, or an
unverifiable tag missing from `keys/exceptions.tsv` stops collection.

Findings while assembling the keys (2026-10-10):

- Some keys in the GNU keyring are stale copies: Carlos O'Donell's work key
  shows an expiry of 2020-08-01 there, while keyserver.ubuntu.com and
  keys.openpgp.org carry later self-signatures extending it to 2025-07-23;
  Andreas K. Huettel's key shows 2026-04-06 in the GNU keyring and on
  keyserver.ubuntu.com, and 2027-02 on keys.openpgp.org. Each file in `keys/`
  therefore merges every listed source.
- Carlos O'Donell's earlier key (2.21, 2.22) was revoked on 2016-08-24 with
  reason "no reason specified" and comment "Expiring this key." after it had
  expired on 2016-07-11; the 2015 signatures predate both.
- The key that signed 2.30 (`19DBA67103BE02611CBF20728E72AB4414F97278`) is
  not in the GNU keyring or on keyserver.ubuntu.com, keys.openpgp.org,
  pgp.mit.edu, or keys.gnupg.net. For 2.30 the evidence is the tag object
  agreeing at sourceware and at the GitHub mirror.
- Five keys come only from keyserver.ubuntu.com, which does not verify key
  ownership; their trust rests on the key fingerprints matching the
  signatures on glibc's official tags, not on the keyserver.

## Per file (`data/files.tsv`, `glibc_abilists::AbiList`)

| Field | Meaning |
| --- | --- |
| `path` | The path in glibc's tree at the tag. |
| `blob` | The git blob id: the SHA-1 of the object that the signed tag's tree names. |
| `sha256` | SHA-256 of the contents, checked again by the crate's tests and `AbiList::verify`. |
| `bytes` | Size. |
| `modified` | Always `no`: files are copied byte for byte. |

The selection rule is in `upstream.toml`: per library, the first of the
listed directories that has `<library>.abilist`.

## Reproducing

The collector regenerates everything from a glibc clone, and `--check`
compares without writing. CI runs the check on every change. Collection
needs `git`, `gpg`, and network access for the mirror comparison
(`--offline` skips it).

## What this does not prove

- That a release behaves as its ABI list says; the lists are glibc's own
  declarations, checked by glibc's build.
- Anything about a distribution's glibc, which may carry patches.
