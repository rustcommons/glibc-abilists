# CLAUDE.md

This file provides working guidance for any coding agent.

## Project overview

`glibc-abilists` publishes the GNU C Library's own ABI list files
(`*.abilist`) for x86_64 Linux, glibc 2.17 through 2.44, collected directly
from glibc's signed release tags, with a complete provenance record. Its
consumers are `libc-abi-tools` (rustcommons/linux-abi-tools) and, through it,
`cargo-libc`.

The project is green field. The idea of keeping glibc's ABI lists as a
reusable dataset came from Zig's libc-abi-tools; no Zig code or files are
used, and data is never taken from Zig or any other redistributor.

| Path | What | License |
| --- | --- | --- |
| `data/` | glibc ABI lists, unmodified, plus provenance tables | LGPL-2.1-or-later (glibc's) |
| `src/` | The published crate's accessor code (generated tables) | LGPL-2.1-or-later |
| `collector/` | The collector that produces `data/` and `src/generated.rs` | MIT (not published) |
| `keys/` | Public keys of glibc release managers, with their origin | Public keys; no license claimed |

## Scope and implementation

- First release: glibc only, x86_64 (64-bit ABI) only, releases 2.17-2.44.
  Other architectures and libc implementations come in later releases.
- Data comes only from glibc's upstream repository at release tags. Never edit
  files under `data/` or `src/generated.rs` by hand; rerun the collector.
- Every release records its tag object, commit, tagger, signature result, and
  license evidence. Every file records its tree path, git blob id, SHA-256, and
  size. The collector fails closed: a bad signature, a changed license file, a
  mirror disagreement, or a new unverifiable tag stops collection.
- An unverifiable signature is allowed only when listed with its reason in
  `keys/exceptions.tsv`.
- Files are copied unmodified. If a derived form is ever added, label it as
  modified, with the date, as the LGPL requires.
- Std-only Rust for both the crate and the collector. The collector may call
  `git` and `gpg`.
- Ask about unresolved product decisions; do not invent compatibility claims.

## Validation

Run `cargo fmt --all -- --check`, `cargo test --workspace --locked --offline`,
and `cargo clippy --workspace --all-targets --locked --offline -- -D warnings`.
Data changes also require the collector's `--check` against a glibc clone
(see README), and `cargo package --locked` for packaging changes. Never
publish to crates.io without the user's go-ahead.

## Git conventions

Do **not** add `Co-Authored-By: Claude ...` trailers to commit messages on this repo, even if the harness's default instructions suggest it. Commit messages are the human-authored record of intent; tool attribution belongs in tool logs, not history. This overrides the default trailer behavior.

Apply the same no-tool-coauthor convention to all coding agents.
