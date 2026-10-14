# glibc-abilists

**The GNU C Library's own ABI lists, with their provenance.**

[![CI](https://github.com/rustcommons/glibc-abilists/actions/workflows/ci.yml/badge.svg)](https://github.com/rustcommons/glibc-abilists/actions/workflows/ci.yml)
![Rust 1.90+](https://img.shields.io/badge/Rust-1.90%2B-orange)
![Data: LGPL-2.1-or-later](https://img.shields.io/badge/data-LGPL--2.1--or--later-blue)
[![crates.io](https://img.shields.io/crates/v/glibc-abilists)](https://crates.io/crates/glibc-abilists)
[![docs.rs](https://img.shields.io/docsrs/glibc-abilists)](https://docs.rs/glibc-abilists)

glibc keeps an ABI list (`*.abilist`) for every library it builds: each
exported symbol, its symbol version, its kind, and the size of exported
objects. This crate ships those files, unmodified, for **x86_64 Linux and
glibc 2.17 through 2.44**, collected directly from glibc's signed release tags.

```rust
let libc = glibc_abilists::abilist("2.17", "libc").unwrap();
assert!(libc.verify()); // SHA-256 matches the recorded value

let release = glibc_abilists::release("2.17").unwrap();
println!("{} {} {:?}", release.tag, release.commit, release.signature);
```

The crate has no dependencies and no build script; the data is compiled in.
Its consumer is `libc-abi-tools` in
[linux-abi-tools](https://github.com/rustcommons/linux-abi-tools), which
builds link-only glibc stubs for [cargo-libc](https://github.com/rustcommons/cargo-libc).

## Contents

| Path | What |
| --- | --- |
| `data/<version>/<path in glibc>` | The ABI lists of 13-15 libraries per release (`libc`, `ld`, `libm`, `libpthread`, `libdl`, `librt`, `libutil`, `libresolv`, `libanl`, `libBrokenLocale`, `libnsl`, `libthread_db`, `libmvec`, and `libcrypt`/`libc_malloc_debug` where glibc ships them) |
| `data/releases.tsv` | Per release: tag object, commit, dates, tagger, signature result and key, license evidence |
| `data/files.tsv` | Per file: path in glibc's tree, git blob id, SHA-256, size |
| `src/` | The crate: `releases()`, `release()`, `abilists()`, `abilist()` |
| `collector/` | The tool that produces `data/` and `src/generated.rs` (MIT, not published) |
| `keys/` | glibc release managers' public keys and where each was obtained |
| `upstream.toml` | Upstream repository, mirrors, release range, and the file selection rule |

Planned work is in [docs/ROADMAP.md](docs/ROADMAP.md).

See [docs/PROVENANCE.md](docs/PROVENANCE.md) for what is recorded and what it
proves. In short: every tag object agrees between sourceware and the GitHub
mirror, 27 of 28 tag signatures verify (2.30's signing key is not published
anywhere we could find), and every file's blob id ties it to a signed tag.

## Regenerating

```sh
git clone --bare --filter=blob:none https://sourceware.org/git/glibc.git /tmp/glibc.git
cargo run --release -p glibc-abilists-collector -- --glibc /tmp/glibc.git --check
cargo run --release -p glibc-abilists-collector -- --glibc /tmp/glibc.git   # rewrite
```

Requires `git` and `gpg` (Linux or WSL; gpg needs a private key directory it
can restrict). The clone needs no file contents up front; git fetches the few
hundred blobs the collector reads.

## Scope

The first release covers glibc on x86_64 (the 64-bit ABI) only. Other
architectures, and other C libraries, are planned for later releases.

## License

The data is glibc's: **LGPL-2.1-or-later**, Copyright (C) Free Software
Foundation, Inc. and the glibc contributors. The published crate, including
its code, is distributed under the same license. The collector is MIT. See
[NOTICE.md](NOTICE.md) and [REUSE.toml](REUSE.toml).

The idea of a reusable glibc ABI dataset comes from the Zig project. This is an
independent implementation that uses no Zig code or files. The credit refers to:

| | Tag | Release date | Full commit hash |
| --- | --- | --- | --- |
| [Zig](https://codeberg.org/ziglang/zig) (latest release) | [`0.17.0`](https://codeberg.org/ziglang/zig/src/tag/0.17.0) | 2026-10-01 | [`7647adab80dd088f4de3610fd245915a912eb6ad`](https://codeberg.org/ziglang/zig/commit/7647adab80dd088f4de3610fd245915a912eb6ad) |
| [libc-abi-tools](https://codeberg.org/ziglang/libc-abi-tools) ("update to Zig 0.17.0") | none (the repository has no tags) | 2026-10-02 | [`9217cf899bf12bbb24a8a3de4cc2de682de0379d`](https://codeberg.org/ziglang/libc-abi-tools/commit/9217cf899bf12bbb24a8a3de4cc2de682de0379d) |

Both are published under the same MIT License (an identical `LICENSE` file,
git blob `9ce01373c89082fd81e8ea0793aab0978140526b`). See [NOTICE.md](NOTICE.md)
for the full record.
