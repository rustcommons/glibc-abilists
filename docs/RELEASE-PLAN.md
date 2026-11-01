# Next release plan

Date: 2026-10-31. Status: proposed; no version bump or publication authorized.

## Scope and decision

The documentation cleanup makes AGENTS.md authoritative, simplifies the README,
adds consumer/collection guides and release planning links, and corrects stale
wording and a manifest comment. It changes no API, data, generated tables,
dependencies, or lock resolutions. Keep
all generated data byte-for-byte unchanged. Do not manufacture a data refresh
for a documentation release.

GitHub-only instruction files and this guide need no crates.io release. The
README is packaged: its new link reaches GitHub readers immediately, but an
existing crate archive and its registry README do not change. A future package
release is needed to distribute a changed README, Rust API documentation on
docs.rs, code, or embedded data. `docs/**`, the collector, and agent instructions
are excluded by the current package include list; publishing alone would not
ship this guide. The README therefore links to its GitHub location.

Recommendation: defer publishing for this cleanup alone. If maintainers choose
to ship packaged README/API-documentation corrections with unchanged behavior,
use a patch candidate, **0.1.2**, only after checking that number is still unused.
Compatible data corrections need explicit consumer validation before treating
them as a patch. New architectures, changed lookup/data semantics, removed
records, or breaking APIs need a separate scope decision; a breaking 0.x change
requires a minor release (for example 0.2.0). Adding a glibc release is not
implicitly qualified downstream. This is a plan, not a release record.

## Evidence checked on 2026-10-31

Live GitHub manifests, locks, tags and crates.io API responses were read. These
are dated observations, not promises about future registry state. External
consumers are unknown, not absent.

| Repository / published package | Manifest / lock | Latest registry version, yanked | Existing tag |
| --- | --- | --- | --- |
| glibc-abilists / `glibc-abilists` | 0.1.1 / 0.1.1; unpublished collector 0.1.0 | [0.1.1, false](https://crates.io/api/v1/crates/glibc-abilists) | [v0.1.1](https://github.com/rustcommons/glibc-abilists/tree/v0.1.1) |
| glibc-provider / `glibc-provider-sources` | 0.1.0 / 0.1.0 | [0.1.0, false](https://crates.io/api/v1/crates/glibc-provider-sources) | [v0.1.0](https://github.com/rustcommons/glibc-provider/tree/v0.1.0) |
| linux-abi-tools / `libc-abi-tools` | 0.3.1 / 0.3.1 | [0.3.1, false](https://crates.io/api/v1/crates/libc-abi-tools) | [v0.3.1](https://github.com/rustcommons/linux-abi-tools/tree/v0.3.1) |
| linux-abi-tools / `linux-targets` | 0.3.1 / 0.3.1 | [0.3.1, false](https://crates.io/api/v1/crates/linux-targets) | same workspace tag |
| linux-abi-tools / `kernel-abi-tools` | 0.3.1 / 0.3.1 | [0.3.1, false](https://crates.io/api/v1/crates/kernel-abi-tools) | same workspace tag |
| linux-abi-tools / `linux-abi-scan` | 0.3.1 / 0.3.1 | [0.3.1, false](https://crates.io/api/v1/crates/linux-abi-scan) | same workspace tag |
| linux-abi-tools / `linux-abi-tools` | 0.3.1 / 0.3.1 | [0.3.1, false](https://crates.io/api/v1/crates/linux-abi-tools) | same workspace tag |
| cargo-libc / `cargo-libc` | 0.2.0 / 0.2.0 | [0.2.0, false](https://crates.io/api/v1/crates/cargo-libc) | [v0.2.0](https://github.com/rustcommons/cargo-libc/tree/v0.2.0) |

This repository was inspected at `d7e5228dae5b2591fc7849a9b82607a7d6bc25bb`.
Downstream snapshots: [linux-abi-tools manifests and lock](https://github.com/rustcommons/linux-abi-tools/tree/6c67fa61481a516edd020291085457a1b864be4a),
[cargo-libc manifests and lock](https://github.com/rustcommons/cargo-libc/tree/ed7033ddd49f189d8b964cac510908ec56feffd2),
and [glibc-provider](https://github.com/rustcommons/glibc-provider/tree/dff7aae4f1b7e2c3f4151d88349e5812aa03092b).
Re-read these repositories before executing the plan; other documentation work
may land independently. Repository names are not necessarily package names:
`glibc-provider` publishes `glibc-provider-sources`; `libc-abi` is a binary of
`libc-abi-tools`, not the workspace's umbrella crate.

## Dependency and artifact propagation

`glibc-abilists` has no dependencies or feature switches. Its text files and
provenance tables are compiled into the library. `glibc-provider-sources` also
has no external dependencies; its package includes `pack/sources.gz`, generated
accessors, and `qualification/accepted-differences.tsv`, with the applicable
license texts. It supplies source inputs, not a replacement runtime libc.

The linux-abi-tools root package is `libc-abi-tools`. Its optional registry
requirements are `glibc-abilists = "0.1.0"` and
`glibc-provider-sources = "0.1.0"`. Default features `bundled-glibc-data` and
`provider-sources` enable both. Its lock selects **0.1.0 for both**, despite
ABI-list 0.1.1 being available. Disabling default features omits these embedded
inputs unless another dependency enables them through feature unification.

All five workspace packages and their actual internal ordering:

| Package | Internal dependencies (all requirements 0.3.1) | Embedded inputs / role |
| --- | --- | --- |
| `linux-targets` | none | Distribution presets |
| `kernel-abi-tools` | linux-targets | Syscall/madvise tables; kernel simulation tools |
| `libc-abi-tools` | none | Optional ABI lists and provider sources; libgcc_s table and container build recipes |
| `linux-abi-scan` | kernel-abi-tools, linux-targets | Static binary inspection |
| `linux-abi-tools` | all four above | Re-exports; enables libc-abi-tools defaults |

The workspace lock resolves internal packages at 0.3.1. cargo-libc requires
`kernel-abi-tools`, `linux-abi-scan`, `linux-targets`, and `libc-abi-tools` at
**"0.3.0"** and locks each at **0.3.0**; it does not depend on the umbrella
`linux-abi-tools` package. Its transitive glibc packages are both locked at
0.1.0. It uses libc-abi-tools defaults without its own feature switch, so the
ABI lists and source pack reach the built coordinator and helper binaries.
Provider caches and exported provider directories are separate generated
artifacts: updating a registry dependency does not rewrite them.

A requirement such as "0.1.0" permits compatible 0.1.x updates; a lockfile still
pins the old resolution. Therefore:

- No downstream manifest edit is needed merely to permit ABI-list 0.1.2.
  Raise the minimum only when downstream behavior requires the newer contents;
  a 0.2 change requires revisiting incompatible requirements.
- Refresh each consuming lock deliberately when adopting a new package. Updating
  the library repository's lock does not update cargo-libc's lock or user locks.
- Rebuild to embed changed data/code. Docs-only changes do not require rebuilding
  working binaries or regenerating provider caches.
- A local lock refresh/rebuild is not a new downstream release. Publish a new
  downstream version when distributing changed package contents, new minimum
  requirements, or a refreshed packaged lock (notably cargo-libc's `--locked`
  installation path). Existing compatible requirements can select the new
  library in fresh unlocked resolutions without republishing the tools.

## Conditional release order

Publish changed glibc packages first, independently: there is no dependency
between glibc-abilists and glibc-provider-sources. Neither needs a release solely
because the other changes. Then, only if a tools release is needed, follow its
[existing release procedure](https://github.com/rustcommons/linux-abi-tools/blob/6c67fa61481a516edd020291085457a1b864be4a/docs/RELEASING.md):

1. linux-targets;
2. kernel-abi-tools;
3. linux-abi-scan;
4. libc-abi-tools (after any required glibc packages are available);
5. linux-abi-tools umbrella;
6. cargo-libc, if its own release is needed.

libc-abi-tools can technically precede the first three; the sequence above is
one valid topological order. The existing workspace policy is **lockstep while
0.x**, not selective publication: all five share a version and internal version
requirements. Selectively releasing only libc-abi-tools would require an
explicit policy decision in that repository. Avoid interpreting a dependency
update as an unconditional requirement to release the whole chain. Its current
release guide says "four" in one packaging paragraph although there are five;
use the five-package graph above and have that repository correct the typo.

## Future release gates (not executed for this documentation change)

Follow [AGENTS.md](../AGENTS.md), the [CI workflow](../.github/workflows/ci.yml),
and [provenance rules](PROVENANCE.md). Current MSRV is Rust 1.90; CI uses 1.90.0
for MSRV and 1.99.0 for main checks, with Rust tests on Linux and Windows.

From this repository root, a candidate release must pass:

```sh
cargo fmt --all -- --check
cargo test --workspace --locked --offline
cargo clippy --workspace --all-targets --locked --offline -- -D warnings
RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps --locked --offline
cargo package --locked
cargo package --list --locked
```

Also run the tests with the supported MSRV toolchain. Inspect the package for
README, embedded files and LGPL license/notice completeness, and for exclusion
of the unpublished MIT collector and keys. Check README doc tests and docs.rs
build success for the version actually published. This repository has CI but
no separate release automation/procedure file; existing tags use `vX.Y.Z`.
Publication, version edits, and tags remain a separately authorized step.

For unchanged data, review the diff and preserve recorded provenance; no data
regeneration or provider replay is needed just to edit prose. The existing CI
upstream check still runs on pushes. For regenerated data, never edit `data/`
or `src/generated.rs` by hand. Use the collector and then:

```sh
cargo run --release --locked -p glibc-abilists-collector -- --glibc /path/to/glibc.git --check
```

This needs Git, GPG and upstream/mirror access. Collector `--offline` skips
remote tag comparisons; a partial clone can still fetch missing blobs, and
Cargo's separate `--offline` flag does not disable those Git fetches. Require
matching tag identities, accepted signature results, license evidence, blob
identities and hashes; glibc 2.30's documented exception is not full signature
verification. Do not invent new exceptions to make a release pass.

There is no source-pack/replay tool in glibc-abilists. If provider source inputs
change in glibc-provider, its owners must regenerate the pack rather than edit
it, check pack/data correspondence and licenses (including Linux UAPI terms),
and run its [replay qualification](https://github.com/rustcommons/glibc-provider/blob/dff7aae4f1b7e2c3f4151d88349e5812aa03092b/docs/REPLAY.md).
Docs-only provider changes do not require regenerating source inputs.

Changed ABI lists or provider inputs require downstream integration with the
candidate dependency resolutions: default-feature provider preparation,
no-default-feature behavior where supported, generated symbol/version checks,
source and container modes, exhaustive links, application runs and provider
export/import verification. Use the tools workspace's
[validation guide](https://github.com/rustcommons/linux-abi-tools/blob/6c67fa61481a516edd020291085457a1b864be4a/docs/VALIDATION.md)
and [provider delivery qualification](https://github.com/rustcommons/linux-abi-tools/blob/6c67fa61481a516edd020291085457a1b864be4a/docs/PROVIDER-DELIVERY.md).
Exercise cargo-libc's build/link/inspect path with the adopted locks and fresh
providers, checking cache invalidation rather than silently trusting old cached
artifacts. Replay compares object interfaces, not a universal runtime guarantee.
Real target qualification is distinct from static scans and kernel simulation;
the separate musl probe does not certify glibc compatibility.

## Go/no-go and recovery

- [ ] Maintainer approves exact scope, version, publication and tags separately.
- [ ] Refresh registry/yank/tag evidence and review concurrent changes.
- [ ] Confirm unchanged-data claim or attach collector/provenance qualification.
- [ ] Pass relevant release CI, MSRV, docs, package and license gates.
- [ ] Record downstream minimums, lock adoption and feature combinations tested;
      qualify changed providers and consumers, or justify why unchanged inputs
      make replay unnecessary.
- [ ] Required dependency versions exist and are not yanked before publication.
- [ ] Review the exact packaged contents and commit; keep tags tied to that commit.
- [ ] After publication verify registry metadata/checksum and docs.rs; record the
      actual release only then. Stop if any required evidence is missing.

A published crates.io version cannot be overwritten. A Git revert does not
replace it, and moving a tag is not a rollback. Publish a new corrective version
when needed; yanking prevents normal new resolution but does not invalidate
existing locks or remove already installed binaries and exported providers.
Consumers may need an explicit lock rollback to a known good allowed version,
rebuild/reinstall, and provider regeneration or re-import. Do not assume yanking
alone repairs them. Keep prior artifacts and qualification evidence identifiable.

## Documentation cleanup completed on 2026-10-31

Follow-up on 2026-11-01: repository API documentation for `Signature::Exception`
now describes a recorded explicit verification exception without implying a
verified signature or a match to the configured key. Published 0.1.1 API docs
remain immutable; distributing this correction requires a future, separately
authorized docs-only release. This closure patch needs no version bump,
publication, or downstream lock changes.

- [x] README dependency setup and current requirements, purpose, one unchanged
  doctested example, and absolute GitHub links to excluded guides.
- [x] [Collection guide](COLLECTING.md): prerequisites, working directory, all
  options, offline limitations, destructive/non-atomic rewrite and validation.
- [x] [Consumer guide](USAGE.md): lookups, raw formats, verification limits,
  redistribution and links to the published API reference.
- [x] Short README credit with historical facts retained in NOTICE.md; dated
  signature interpretation and removal of stale latest/first-release wording.
- [x] Cargo.toml's comment points to AGENTS.md. REUSE.toml still lists both
  instruction files for licensing.

This completes the scoped documentation cleanup, not the release gates above.
The release remains proposed. GitHub readers get these guides without a
crates.io release; the existing published 0.1.1 README remains unchanged until
an explicitly authorized future package release. No generated data, dependency,
package version, or lockfile was changed. Recheck the dated downstream and
registry evidence when a release is actually scheduled.
