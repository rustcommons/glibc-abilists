# Roadmap

Open work only.

## Rust API documentation

The crate is published and its docs.rs page is the main reference for its consumers.
It has a crate-level description with an example; 7 public items are undocumented
(measured with `-W missing_docs` on 2026-10-11).

- [ ] Document every public item: what it returns, when it returns `None`, and the
  meaning of each field of the release and ABI list types. Then add
  `#![warn(missing_docs)]` (CI denies warnings).
- [ ] Keep the README example as a doc test (`cargo test --doc`).
- [ ] Add `cargo doc --workspace --no-deps` with `RUSTDOCFLAGS="-D warnings"` to CI, so
  broken intra-doc links and malformed docs fail the build.
- [ ] Add `documentation` (the docs.rs URL) and `[package.metadata.docs.rs]` (target
  `x86_64-unknown-linux-gnu`, all features documented) to Cargo.toml, and a docs.rs
  badge to the README.
- [ ] Release the result as a patch version, then confirm the docs.rs build succeeded
  and the pages read correctly.
