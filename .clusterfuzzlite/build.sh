#!/bin/bash
set -euxo pipefail
cd "$SRC/glibc-abilists"
rustc --version
rustup show active-toolchain
export CARGO_TARGET_DIR="$PWD/fuzz/target"
# cargo-fuzz does not forward --locked. Fetch with lock enforcement, disable
# network during compilation, and fail if the driver changes the lockfile.
cargo fetch --locked --manifest-path fuzz/Cargo.toml
sha256sum fuzz/Cargo.lock > "$WORK/glibc-fuzz-lock.sha256"
CARGO_NET_OFFLINE=true cargo fuzz build -O --debug-assertions
sha256sum -c "$WORK/glibc-fuzz-lock.sha256"
for target in library_inputs hash_padding; do
  cp "$CARGO_TARGET_DIR/x86_64-unknown-linux-gnu/release/$target" "$OUT/"
  (cd "fuzz/corpus/$target" && zip -q "$OUT/${target}_seed_corpus.zip" ./*)
done
