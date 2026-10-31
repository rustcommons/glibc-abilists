# Collecting and maintaining the dataset

This guide is for repository maintainers. Crate consumers only need the
[dependency setup](../README.md#getting-started). Read [AGENTS.md](../AGENTS.md)
before changing the repository. The collector is an unpublished MIT workspace
package; its binary is `glibc-abilists-collect`.

## Prerequisites and layout

Use Rust/Cargo 1.90 or later, Git, GPG, and Linux or WSL. GPG needs a temporary
directory whose permissions can be restricted; the collector creates a private
keyring containing only the checked-in public keys and removes it afterward.
Normal collection needs network access to sourceware and the configured
mirrors. No Rust crate dependencies need installing, and glibc is not built.

Run the commands below from the root of a checkout of this repository. Choose
an unused path for the upstream clone; `/tmp/glibc.git` is only an example.

| Path | Purpose |
| --- | --- |
| `upstream.toml` | Upstream/mirrors, inclusive release range, target, directory precedence, license file |
| `keys/` | Public keys, sources in `keys.tsv`, explicit reasons in `exceptions.tsv` |
| `data/<version>/<upstream path>` | Unmodified selected ABI files |
| `data/releases.tsv`, `data/files.tsv` | Release and file provenance |
| `src/generated.rs` | Generated tables embedding the data |
| `collector/src/main.rs` | Option parser, validation, collection, comparison, and writing |

## Check without rewriting data

```sh
git clone --bare --filter=blob:none https://sourceware.org/git/glibc.git /tmp/glibc.git
cargo run --release --locked -p glibc-abilists-collector -- --glibc /tmp/glibc.git --check
```

The clone must have every configured release tag. A blobless clone fetches
contents as Git needs them. `--check` collects expected output, compares bytes,
and fails on missing, changed, or unexpected files within the selected release
directories. It leaves repository outputs untouched; Cargo may build the tool,
Git may fetch objects, and GPG uses temporary files.

## Options

Arguments after Cargo's `--` go to the collector:

| Option | Behavior |
| --- | --- |
| `--glibc REPO` | Required: local glibc clone, bare or non-bare |
| `--root DIR` | Repository containing configuration, keys, and outputs; defaults to the current directory |
| `--check` | Compare outputs; absence means rewrite |
| `--offline` | Skip upstream and mirror tag-object comparisons only |
| `-h`, `--help` | Print usage; the current implementation exits with failure even for help |

`--root` does not select Cargo's manifest. To launch from elsewhere, supply
both paths explicitly, replacing the example checkout path:

```sh
cargo run --manifest-path /path/to/glibc-abilists/Cargo.toml --release --locked -p glibc-abilists-collector -- --glibc /tmp/glibc.git --root /path/to/glibc-abilists --check
```

For disconnected operation, all needed Git objects must already be present.
A partial clone may still fetch blobs even with collector `--offline`. Cargo's
separate `--offline` flag controls Cargo, not Git subprocesses. With objects
and toolchain already available, the two flags can be combined:

```sh
cargo run --release --locked --offline -p glibc-abilists-collector -- --glibc /tmp/glibc.git --check --offline
```

This skips independent upstream/mirror comparison and is not a substitute for
the normal online release provenance gate. Signature and license checks remain.

## Rewrite deliberately

Review or save existing work first. Never hand-edit `data/` or
`src/generated.rs`. Omitting `--check` **overwrites generated outputs and deletes
unexpected files inside the configured release directories**. Directories for
versions outside the configured range are not cleaned. Writes are not atomic;
a filesystem error can leave partial output, so inspect failures before retrying.

```sh
cargo run --release --locked -p glibc-abilists-collector -- --glibc /tmp/glibc.git
cargo run --release --locked -p glibc-abilists-collector -- --glibc /tmp/glibc.git --check
git diff --stat
git diff --check
```

Collection stops on missing tags, upstream/mirror disagreements, unacceptable
signatures, or license evidence failures. Investigate the reported evidence;
do not bypass a failure by hand-editing generated files or inventing a signature
exception. See [PROVENANCE.md](PROVENANCE.md) for the existing glibc 2.30 exception.

For an approved release-range extension, first obtain the upstream tags and
review `upstream.toml`, key sources and license evidence. Regenerate, then update
range-specific tests, API documentation and scope statements to match. The
collector currently rejects targets other than `x86_64-linux-gnu`; changing the
configuration alone does not add architecture support. Follow the
[proposed release plan](RELEASE-PLAN.md) for downstream qualification.

## Validate the change

For documentation only, check links/anchors, commands against the parser, the
rendered Markdown, and the diff. Run relevant doc tests if Rust examples change;
do not run unrelated builds for prose edits.

For code/data changes and release qualification, the repository checks are:

```sh
cargo fmt --all -- --check
cargo test --workspace --locked --offline
cargo clippy --workspace --all-targets --locked --offline -- -D warnings
RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps --locked --offline
```

Data changes additionally require the online collector `--check` above. Package
changes require `cargo package --locked` and inspection with
`cargo package --list --locked`. CI also tests Rust 1.90.0 and Windows/Linux;
consult the [workflow](../.github/workflows/ci.yml) for the current matrix.
Keep LGPL data/crate notices and MIT collector licensing distinct. Publishing
packages or creating releases requires explicit authorization; regeneration
and passing checks do not grant it.
