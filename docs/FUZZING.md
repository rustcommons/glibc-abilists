# Library fuzzing and security-tool pins

This guide implements [GA-REQ-019](REQUIREMENTS.md). The library exposes raw ABI
text and compiled-in provenance; it **does not parse `.abilist` syntax or load
TSV/data tables at runtime**. See `src/lib.rs` and its module documentation.
No parser was added to satisfy a fuzzing metric. The collector has separate
configuration/key/Git/GPG parsing paths; those are outside these library harnesses
and remain a coverage gap, not a claim of qualification.

## Targets and seeds

| Target | Actual code under test | Checks and limits |
| --- | --- | --- |
| `library_inputs` | `release`, `abilist`, release/ABI enumeration and `AbiList::verify` in `src/lib.rs` | Arbitrary UTF-8 lookup strings, absent keys, selected immutable table entries, valid hash acceptance and incorrect hash rejection. Invalid UTF-8 cannot enter an `&str` API. No mutable table-loading or ABI-text parsing claim. |
| `hash_padding` | The actual `src/sha256.rs`, included by path without copying it or exposing a new API | Arbitrary bytes, block/padding boundaries, sanitizers/debug assertions and hex roundtrips. Existing known-vector unit tests remain the correctness oracle; this is not a differential cryptographic proof. |

Lookup seeds come from existing lookup tests. Hash seeds include `abc` from
known-vector tests and lengths around SHA-256's 56/64-byte padding boundaries.
The two `glibc-*.abilist` seeds are complete, unmodified copies of
`data/2.17/sysdeps/unix/sysv/linux/x86_64/64/nptl/libdl.abilist` and
`data/2.44/sysdeps/unix/sysv/linux/x86_64/64/libdl.abilist`, retaining their
LGPL-2.1-or-later license under `REUSE.toml`. Their original upstream identities
are in `data/files.tsv`. They exercise raw-byte hashing, not a nonexistent ABI
parser. No source dataset is rewritten.

The unpublished `fuzz/` package is its own workspace with its own lockfile.
Only that graph uses `libfuzzer-sys`; the shipped library and collector remain
std-only and the root lockfile is unchanged. The package include list excludes
fuzz targets, corpora, tooling and these repository-only guides.

## Local runs

Install a dated nightly and a pinned harness driver outside the project graph:

```sh
rustup toolchain install nightly-2025-09-05 --profile minimal --component rust-src
cargo install cargo-fuzz --locked --version 0.13.1 --jobs 2
cargo fetch --locked --manifest-path fuzz/Cargo.toml
CARGO_NET_OFFLINE=true cargo +nightly-2025-09-05 fuzz build -O --debug-assertions
cargo +nightly-2025-09-05 fuzz run library_inputs -- -max_total_time=60 -max_len=4096
cargo +nightly-2025-09-05 fuzz run hash_padding -- -max_total_time=60 -max_len=4096
```

Use one build/run at a time on small machines. Requires a native C++ compiler;
libFuzzer and the address sanitizer are supplied through the fuzz build. Inputs
and failures live under `fuzz/corpus/` and ignored `fuzz/artifacts/`. To reproduce,
pass a crash file instead of the corpus directory, for example
`cargo +nightly-2025-09-05 fuzz run library_inputs fuzz/artifacts/library_inputs/crash-HASH`.
Treat each crash as a real bug: preserve the original reproducer, minimize it,
fix the production cause and add a regression test. Escalate policy-dependent
fixes; do not hide crashes by weakening assertions or ignoring seeds.

## ClusterFuzzLite

The [builder](../.clusterfuzzlite/Dockerfile) pins the OSS-Fuzz Rust builder by
SHA-256 digest. Its image configuration includes nightly-2025-09-05 and the
cargo-fuzz driver; it installs nothing from mutable channels. The build script
uses the separate lockfile, optimized code, debug assertions and at most two
compiler jobs, then exports binaries and zipped seed corpora. Because the driver
does not forward `--locked`, the build first uses `cargo fetch --locked`, compiles
offline and compares the lockfile SHA-256 before/after compilation.

[PR fuzzing](../.github/workflows/cflite_pr.yml) runs code-change mode for 300
seconds when `src/`, `fuzz/`, the builder or its workflows change, skipping
Dependabot-authored PRs and cancelling superseded revisions. [Weekly batch
fuzzing](../.github/workflows/cflite_batch.yml) runs for 1800 seconds and supports
manual dispatch. Durations are the total fuzzing budgets, not per-target promises.
No external corpus-storage repository or token is provisioned: seeds are versioned
here, and learned corpora are not promised to accumulate across weekly runs.
ClusterFuzzLite's action SHA pins its wrapper; the upstream action internally
uses its own versioned service images, a transitive dependency outside this
repository's Dockerfile pins.

Crash reporting requests SARIF and uploads `cifuzz-sarif/results.sarif` when one
exists and code-scanning permissions are available. Fork PRs still fuzz, but
cannot upload with a read-only fork token. The fuzz job fails on a reported bug;
absence of SARIF does not mean execution or coverage succeeded. The first weekly
run and post-merge Scorecard recognition need subsequent live evidence.

## Installation and SAST audit

Never put CI-skip markers in commit or merge messages. Scoped workflow gates
bound expensive checks; coordinator-managed CodeQL default setup stays enabled.
Dated historical validation records remain historical and are not operational
instructions to skip current CI.

Security CI installs zizmor with `pip install --require-hashes` from the committed
[wheel hashes](../.github/requirements-security.txt), then runs the CLI directly.
It installs cargo-deny with `--locked --version 0.20.2`. Rust toolchain steps
use the runner-provided rustup with self-updates disabled and exact toolchain
versions, removing the external action's download-and-run bootstrap fallback.
No mutable installer script is downloaded by repository workflow commands. All directly referenced actions use full SHAs; the fuzz
builder image uses a digest. There are no workflow pip installs without hashes,
Cargo installs without exact versions/lock enforcement, unverified executable
downloads, or unpinned Dockerfile base images introduced here.

The [Scorecard rules](https://github.com/ossf/scorecard/blob/6eb56f6a7e62d8f5d55800f54cd92ea305107d1d/docs/checks.md#pinned-dependencies)
and its [shell-download logic](https://github.com/ossf/scorecard/blob/6eb56f6a7e62d8f5d55800f54cd92ea305107d1d/checks/raw/shell_download_validate.go)
are the audit reference: action hashes, container digests, package-install and
download verification patterns are bounded signals. Docker is unavailable in
the local maintenance environment, so no local Scorecard-container score is
claimed. Candidate build/run/check evidence is retained in the associated PR;
configuration alone does not promise a particular score or security outcome.

## Bounded local evidence, 2026-11-18

Against the source payload of main `77e21ed` with these harnesses, an aarch64
Linux build using nightly-2025-09-05 and cargo-fuzz 0.13.1 succeeded with address
sanitization and debug assertions. Each target ran with `-max_total_time=60
-max_len=4096`, using temporary copies of the committed seeds: `library_inputs`
completed 357,633 executions in 61 seconds; `hash_padding` completed 3,686,994
in 61 seconds. Neither reported a crash. Execution counts describe those runs
only; they are not universal qualification or evidence about collector parsers.

Local actionlint 1.7.12, zizmor 1.32.0, release-doc checks and diff checks passed.
The isolated workspace metadata and `cargo tree --workspace --locked --offline`
confirm no new library/collector dependencies. Container execution is validated
by candidate CI rather than claimed from a local Docker run. Weekly batch and
post-merge scoring are explicitly separate future observations.
