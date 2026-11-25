# glibc-abilists

**The GNU C Library's own ABI lists, with their provenance.**

[![CI](https://github.com/rustcommons/glibc-abilists/actions/workflows/ci.yml/badge.svg)](https://github.com/rustcommons/glibc-abilists/actions/workflows/ci.yml)
![Rust 1.90+](https://img.shields.io/badge/Rust-1.90%2B-orange)
![Data: LGPL-2.1-or-later](https://img.shields.io/badge/data-LGPL--2.1--or--later-blue)
[![OpenSSF Scorecard](https://api.scorecard.dev/projects/github.com/rustcommons/glibc-abilists/badge)](https://scorecard.dev/viewer/?uri=github.com/rustcommons/glibc-abilists)
[![crates.io](https://img.shields.io/crates/v/glibc-abilists)](https://crates.io/crates/glibc-abilists)
[![docs.rs](https://img.shields.io/docsrs/glibc-abilists)](https://docs.rs/glibc-abilists)

glibc keeps ABI lists describing exported symbols, symbol versions, kinds, and
object sizes. This crate embeds those files, unmodified, for **x86_64 Linux's
64-bit ABI, glibc 2.17 through 2.44**, collected from upstream release tags.
It provides raw data and provenance, not a parser or a runtime compatibility test.

Its consumer is `libc-abi-tools` in
[linux-abi-tools](https://github.com/rustcommons/linux-abi-tools), which builds
link-only glibc stubs for [cargo-libc](https://github.com/rustcommons/cargo-libc).
Other architectures and C libraries remain planned.

## Getting started

Requires **Rust 1.90 or later**. Add the library to your project's `Cargo.toml`:

```toml
[dependencies]
glibc-abilists = "0.1.2"
```

There are no crate dependencies, feature switches, or build script. Data is
compiled in: consuming it requires no glibc checkout, Git, GPG, or runtime
network access. This is a library dependency, not a `cargo install` tool.

```rust
let libc = glibc_abilists::abilist("2.17", "libc").unwrap();
assert!(libc.verify()); // SHA-256 matches the recorded value

let release = glibc_abilists::release("2.17").unwrap();
println!("{} {} {:?}", release.tag, release.commit, release.signature);
```


## Guides

- [Changelog](https://github.com/rustcommons/glibc-abilists/blob/main/CHANGELOG.md), [GitHub Releases](https://github.com/rustcommons/glibc-abilists/releases), and [release policy](https://github.com/rustcommons/glibc-abilists/blob/main/docs/RELEASING.md): version history and publication/completion rules. Maintainers use the owner-reviewed [Trusted Publishing workflow](https://github.com/rustcommons/glibc-abilists/blob/main/.github/workflows/trusted-publishing.yml) (`trusted-publishing.yml`, environment `crates-io`).
- [Requirements and evidence](https://github.com/rustcommons/glibc-abilists/blob/main/docs/REQUIREMENTS.md): confirmed scope, verification limits, and documentation map.
- [Documentation convention](https://github.com/rustcommons/.github/blob/main/docs/DOCUMENTATION.md): shared documentation and traceability conventions.
- [Consumer guide](https://github.com/rustcommons/glibc-abilists/blob/main/docs/USAGE.md): lookups, raw formats, verification, and redistribution.
- [API reference](https://docs.rs/glibc-abilists/0.1.2/glibc_abilists/): types, functions, and constants.
- [Provenance](https://github.com/rustcommons/glibc-abilists/blob/main/docs/PROVENANCE.md): evidence and its limits. The recorded dataset has 27 verified tag signatures and one documented exception, glibc 2.30, whose signing key was unavailable during collection.
- [Collection guide](https://github.com/rustcommons/glibc-abilists/blob/main/docs/COLLECTING.md): maintainer setup, checking, regeneration, and validation.
- [Roadmap](https://github.com/rustcommons/glibc-abilists/blob/main/docs/ROADMAP.md) and [release plan](https://github.com/rustcommons/glibc-abilists/blob/main/docs/RELEASE-PLAN.md).

## Verify release assets

Releases produced after OpenPGP signing adoption include `SHA256SUMS.asc`.
Download the release assets into one directory, then fetch the public key and
verify with GnuPG and sha256sum (optional tools; not needed to use the library):

```sh
curl -fsSLo release-signing-key.asc https://raw.githubusercontent.com/rustcommons/glibc-abilists/main/.github/release-signing-key.asc
(
  set -eu
  export GNUPGHOME="$(mktemp -d)"
  chmod 700 "$GNUPGHOME"
  trap 'gpgconf --kill gpg-agent; rm -rf "$GNUPGHOME"' EXIT
  gpg --import release-signing-key.asc
  gpg --fingerprint B281C3010A76A11737B4D066C7396F6EA7647CC6
  gpg --status-fd 1 --verify SHA256SUMS.asc SHA256SUMS
  sha256sum --check SHA256SUMS
)
```

The only primary fingerprint must be
**B281C3010A76A11737B4D066C7396F6EA7647CC6**. Check it before trusting the key;
GnuPG trust warnings do not replace this fingerprint check. Also apply the
[release policy's signature-status checks](https://github.com/rustcommons/glibc-abilists/blob/main/docs/RELEASING.md#openpgp-release-signing):
GnuPG can exit successfully for a revoked or expired key. Repository users can
enforce the pin and status/expiry checks automatically with the [fixed-pin verification script](https://github.com/rustcommons/glibc-abilists/blob/main/scripts/release_signing.py)
instead. Earlier 0.1.0–0.1.2 releases keep their existing archive verification,
as described in the [release policy](https://github.com/rustcommons/glibc-abilists/blob/main/docs/RELEASING.md).

## License and credit

The data and published crate are **LGPL-2.1-or-later**, Copyright (C) Free
Software Foundation, Inc. and the glibc contributors. The unpublished collector
is MIT. See [NOTICE.md](NOTICE.md) and the repository's
[REUSE annotations](https://github.com/rustcommons/glibc-abilists/blob/main/REUSE.toml).

The idea of a reusable glibc ABI dataset comes from the Zig project's
libc-abi-tools. This independent implementation uses no Zig code or files;
data comes directly from glibc. [NOTICE.md](NOTICE.md#credit) preserves the
referenced releases, commits, and license evidence. No endorsement is implied.
