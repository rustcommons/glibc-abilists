# glibc-abilists 0.1.2 release record

Date: 2026-11-01. Status: published by the coordinator, independently verified.

## Release identity and scope

- Released commit: `d0cf8ee40529880373286c500dd22b9c21d79e2c`.
- [Release PR #3](https://github.com/rustcommons/glibc-abilists/pull/3), normal
  merge preserving reviewed head `24b6f0a2ce707f348225730a06f7f835eecb9e0b`.
- Annotated [tag v0.1.2](https://github.com/rustcommons/glibc-abilists/tree/v0.1.2):
  tag object `da2cd5edd5c7ea11f8d955d5b9f526217e0fbc4f`, peeled commit exactly
  the released commit above. Confirmed by `git ls-remote origin 'refs/tags/v0.1.2*'`.
- Registry publication time: `2026-11-01T04:32:05.239152Z`.

Previous `Signature::Exception` API wording overstated signature guarantees.
The existing collector never required signature presence or a match to the
configured exception key on this path. Version 0.1.2 delivers corrected API
documentation and the current README. This is not a claim of data compromise.
Public API/behavior, generated source, data, keys, upstream selection and
collector policy are unchanged. No downstream minimum or lock change is needed
solely for this documentation release; deliberate lock adoption is separate.
Published 0.1.1 remains immutable and historical; 0.1.2 delivers the fix now.

## Official artifact and source evidence

Fetched independently on 2026-11-01:

- [Official archive](https://static.crates.io/crates/glibc-abilists/glibc-abilists-0.1.2.crate).
- [Version metadata](https://crates.io/api/v1/crates/glibc-abilists/0.1.2) and
  [registry index](https://index.crates.io/gl/ib/glibc-abilists).
- SHA-256: `0f76191d504fe7944029bf5ac1e3ee901b050e7ce3fc946fa2661a9094045287`.
  Python `hashlib.sha256` of the downloaded archive matches both registry
  checksum fields; both metadata and index report `yanked = false`.
- Parsed `.cargo_vcs_info.json`: `git.sha1` equals the exact released commit,
  no dirty marker, and `path_in_vcs` is empty.
- Python `tarfile` comparison confirms the same 404 paths and byte-identical
  extracted contents as the clean merged local package. The local prepublication
  compressed archive hash was
  `0abd5457841e93c2093dfed9553a382905ece7dc52237fbc99475c46fc5b7a53`;
  archive bytes differ, while every extracted file is identical. The official
  registry checksum above identifies the published artifact.
- Package includes README, NOTICE, library source/data and license files;
  `docs/`, collector, keys and CI are excluded as the manifest specifies.
  New repository-only bookkeeping cannot change this published archive.
- Official 0.1.1 archive SHA-256
  `b09a1df98771a4aaaa6523290bf42c3bfd89962e5e621b6d57c221c198bfbf8b`
  matched its registry index. Its VCS commit is
  `d7e5228dae5b2591fc7849a9b82607a7d6bc25bb`.
  Comparing all embedded paths confirms byte-identical data, generated source
  and hash implementation. Removing Rust doc-comment lines from `src/lib.rs`
  yields identical source; the exported API and behavior are unchanged.
  Normalized/original manifest semantic comparison permits only package version
  and descriptive documentation changes. `git diff` from that commit over
  `collector/`, `keys/` and `upstream.toml` is empty.

## Candidate and merged package validation

Local Rust 1.90.0, aarch64 Linux, on 2026-11-01, package-bearing candidate
`2337d8ea1ab5b38b05ed07cb4ea6183ba0a32859`:

```sh
cargo +1.90.0 fmt --all -- --check
cargo +1.90.0 test --workspace --locked --offline -j 2
cargo +1.90.0 clippy --workspace --all-targets --locked --offline -j 2 -- -D warnings
RUSTDOCFLAGS="-D warnings" cargo +1.90.0 doc --workspace --no-deps --locked --offline -j 2
cargo +1.90.0 package --locked --offline -j 2
cargo +1.90.0 package --list --locked --offline
git diff --check origin/main...HEAD
```

All passed, including README and API doctests. Nine GitHub GFM renders, local
Markdown paths/anchors, requirements tables and generated Signature HTML/links
were checked. The subsequent reviewed head changed only repository-only link
evidence; no fresh local full suite was run for that link change.

Final reviewed-head CI: [push run](https://github.com/rustcommons/glibc-abilists/actions/runs/41272540131)
and [PR run](https://github.com/rustcommons/glibc-abilists/actions/runs/41272543888).
Both passed docs, MSRV, package, Linux/Windows Rust checks and the online
collector `--check`. Normal merge was gated on this exact head and green checks.
The clean merged commit was repackaged with
`cargo +1.90.0 package --locked --offline -j 2`; package verification passed.
Its tree equals the reviewed tree, and package VCS metadata names the merge
commit. No data regeneration or publication command ran during this verification.

## Delivered docs.rs evidence

On 2026-11-01, canonical HTTPS URLs returned HTTP 200:

- [0.1.2 version page](https://docs.rs/crate/glibc-abilists/0.1.2), including the
  shipped README's 0.1.2 dependency/API links.
- [Build list](https://docs.rs/crate/glibc-abilists/0.1.2/builds) and successful
  [build #5104080](https://docs.rs/crate/glibc-abilists/0.1.2/builds/5104080),
  dated `2026-11-01 04:33:23`, for `x86_64-unknown-linux-gnu` using
  `rustc 1.101.0-nightly (f9ceb4a4d 2026-10-31)`. The log finishes HTML
  generation. It also contains a target-filter no-op warning; this is not
  evidence of broader example or runtime qualification.
- [Rendered Signature API](https://docs.rs/glibc-abilists/0.1.2/glibc_abilists/enum.Signature.html#variant.Exception).
  Parsed actual rendered HTML text confirms the explicit verification exception,
  the absence of signature-presence/configured-key guarantees, the possibly
  empty GPG-reported key rather than a verified identity, and that a copied
  reason does not attest fallback checks. The old unavailable-key-only wording
  is absent. Official archive source contains the same corrected doc comments.

Initial requests returned 404 before the pages were available; fresh requests
then succeeded, followed by successful canonical-URL checks. Those initial
observations are superseded by the delivered-page evidence above.

## Boundaries and follow-up

These checks establish this artifact's identity, unchanged selected inputs and
delivered documentation. They are not universal runtime compatibility or
continuous upstream authenticity guarantees. Existing offline trust limits and
missing adversarial exception tests remain in [requirements](REQUIREMENTS.md)
and the [roadmap](ROADMAP.md); no new enforcement is claimed. Future collector
validation records and signature/key enforcement still require policy decisions.
