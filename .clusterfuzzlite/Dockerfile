# Includes Rust nightly-2025-09-05 and cargo-fuzz, pinned as one build environment.
FROM gcr.io/oss-fuzz-base/base-builder-rust@sha256:3428d0a90b1e7efbe2ba947bd1d0c9a70594986b4b6553804547f1fb24cfc92f
ENV RUSTUP_TOOLCHAIN=nightly-2025-09-05
ENV CARGO_BUILD_JOBS=2
COPY . $SRC/glibc-abilists
WORKDIR $SRC/glibc-abilists
COPY .clusterfuzzlite/build.sh $SRC/build.sh
