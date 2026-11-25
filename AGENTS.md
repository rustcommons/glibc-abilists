# glibc-abilists

## Working conventions

- Read this file before changing the repository. Keep repository-specific constraints and validation instructions current here.
- Preserve changes made by the user or other agents. Keep work scoped to the requested task.
- Keep documentation aligned with implemented behavior. Record unconfirmed work as planned; do not invent capability or compatibility claims.
- Run checks appropriate to the change. For documentation-only changes, verify links, commands, and the diff; do not run unrelated builds.
- Publishing packages, creating releases, or changing production data requires explicit user authorization.

## Project overview

`glibc-abilists` publishes the GNU C Library's own ABI list files
(`*.abilist`) for x86_64 Linux, glibc 2.17 through 2.44, collected directly
from glibc's release tags, with recorded provenance evidence and the limits
described in [Provenance](docs/PROVENANCE.md). Its
consumers are `libc-abi-tools` (rustcommons/linux-abi-tools) and, through it,
`cargo-libc`.

The idea of keeping glibc's ABI lists as a
reusable dataset came from Zig's libc-abi-tools; no Zig code or files are
used, and data is never taken from Zig or any other redistributor.

| Path | What | License |
| --- | --- | --- |
| `data/` | glibc ABI lists, unmodified, plus provenance tables | LGPL-2.1-or-later (glibc's) |
| `src/` | The published crate's accessor code (generated tables) | LGPL-2.1-or-later |
| `collector/` | The collector that produces `data/` and `src/generated.rs` | MIT (not published) |
| `keys/` | Public keys of glibc release managers, with their origin | Public keys; no license claimed |
| `.github/release-signing-key.asc` | Organization release-checksum verification key; separate from upstream tag keys | Public key; no license claimed |

## Scope and implementation

- Current dataset: glibc only, x86_64 (64-bit ABI) only, releases 2.17-2.44.
  Other architectures and libc implementations are planned.
- Data comes only from glibc's upstream repository at release tags. Never edit
  files under `data/` or `src/generated.rs` by hand; rerun the collector.
- Keep sourceware primary. Review mirror URL changes against upstream documentation; remote-ref retries must stay bounded and may never mask tag mismatches or skip an unavailable configured source. See [online source checks](docs/PROVENANCE.md#online-source-availability-and-mirror-choice).
- Every release records its tag object, commit, tagger, signature result, and
  license evidence. Every file records its tree path, git blob id, SHA-256, and
  size. The collector fails closed: a bad signature, a changed license file, a
  mirror disagreement in online mode, or a new unverifiable tag stops collection.
  Offline mode skips remote comparisons, including in write mode.
- An unverifiable signature is allowed only when listed with its reason in
  `keys/exceptions.tsv`. This exception path does not require signature presence
  or match the configured key; recorded historical evidence is not enforcement.
- Files are copied unmodified. If a derived form is ever added, label it as
  modified, with the date, as the LGPL requires.
- Std-only Rust for both the crate and the collector. The collector may call
  `git` and `gpg`.
- Ask about unresolved product decisions; do not invent compatibility claims.

## Validation

For code changes and release qualification, run `cargo fmt --all -- --check`,
`cargo test --workspace --locked --offline`,
and `cargo clippy --workspace --all-targets --locked --offline -- -D warnings`.
Data changes also require the collector's `--check` against a glibc clone
(see [the collection guide](docs/COLLECTING.md)), and `cargo package --locked`
for packaging changes. Never publish to crates.io without the user's go-ahead.

## Documentation and requirements traceability

- Follow the [Rust Commons documentation conventions](https://github.com/rustcommons/.github/blob/main/docs/DOCUMENTATION.md).
- Maintain [docs/REQUIREMENTS.md](docs/REQUIREMENTS.md) as the repository's index of confirmed requirements, their sources, implementation or documentation, verification evidence, and gaps. Preserve stable repository-scoped IDs and existing identifiers; never renumber or reuse them.
- Distinguish verified behavior, partial verification, planned work, deferred work, and scope exclusions. A source reference or passing test supports only the scope it actually checks; record missing evidence explicitly.
- Keep the README focused on purpose, prerequisites, setup, and guide links. Keep detailed workflows in maintained guides, future work in the roadmap, and dated release or validation evidence identifiable as historical records.
- Update affected requirements, documentation, and trace links together when behavior, scope, interfaces, prerequisites, or validation changes. Use exact versions or revisions for release evidence, and do not rewrite historical outcomes or treat repository edits as changes to an immutable published package.
- Validate documentation changes with source and command checks, links, rendering, and a clean diff. Run additional builds or tests only when changed examples, package contents, or behavior require them.

## Release documentation

- For a repository that ships versioned software, maintain a root `CHANGELOG.md` with an `Unreleased` section and dated, user-facing entries for shipped versions. Record noteworthy changes and their user impact as work is completed; do not substitute a roadmap, Git tag, CI log or validation report for a changelog.
- Before publishing a version, commit its release notes under `docs/releases/vX.Y.Z.md` and link them, the changelog, and the maintained release policy. Explain changes, upgrade or dependency impact, validation and known limits; distinguish software changes from documentation-only releases.
- A software release is not complete until its immutable source tag, applicable registry archives, GitHub Release notes, changelog, documentation, and required verification agree. GitHub Releases document the release; do not attach standalone binaries or other new distribution artifacts without authorization.
- Run the repository's release-documentation CI check before routine merges and release completion. For its first installation, review local checker/syntax evidence, register the workflow on the default branch, and require a green run before completing bootstrap or publishing notes. Verify the live GitHub Release and its tag, text and links after publication; a successful crate upload or tag alone is insufficient.
- When backfilling history, reconstruct facts from existing tags, registry records and source/evidence. Preserve original source identities, distinguish original release dates from documentation backfill dates, and state unavailable historical validation. Never fabricate checks, move tags, republish versions, or imply that repository edits change immutable published archives.
- For an organization profile, placeholder or project with no shipped version, record release verification as not applicable until its first authorized release. Do not invent versions, features, changelog history or GitHub Releases to satisfy this convention.

## Security gates and publishing

- All changes to `main` go through reviewed PRs; never push directly to `main` or bypass required checks. Preserve protected release tags.
- Pin every external GitHub Action to a full commit SHA with its version tag in a trailing comment. Keep workflow permissions read-only and grant write permissions only to the job that needs them; disable persisted checkout credentials.
- Require the `lf-only`, release-documentation, zizmor and cargo-deny checks before merge, alongside applicable existing validation. Review findings rather than weakening the gates; document narrowly justified exceptions inline. Keep Dependabot's monthly grouped Cargo/action version updates capped at two open PRs per ecosystem, with the cooldown preserved; security updates remain immediate and independent of that schedule. Dependabot PRs run one Linux fmt/test/clippy job plus cargo-deny, zizmor, lf-only and managed CodeQL; docs, MSRV, packaging and upstream qualification are deferred to normal PRs, main and schedules. Keep per-PR cancellation on every repository-owned PR workflow. The release-documentation gate applies to normal PRs; the owner-approved core-only Dependabot exception does not establish release qualification. The standard release workflow separately runs gate/build for same-repository affected PRs, including Dependabot, with publishing disabled; fork PRs skip Build.
- Keep Scorecard reporting and SARIF upload scoped to its reporting job. Do not claim a score or SLSA level from configuration alone.
- The tag-triggered [Trusted Publishing workflow](.github/workflows/trusted-publishing.yml) must keep the canonical filename `trusted-publishing.yml` and requires the owner-reviewed `crates-io` environment and matching crates.io Trusted Publisher configuration. Its installation does not authorize a release. Following the owner-machine token revocation on 2026-11-16, all future crate publication must use this workflow and owner approval of the `crates-io` environment; the owner-machine route is retired. Never publish the collector.

- Use explicit runner images (`ubuntu-24.04`, `windows-2025`) in maintained workflows. Moving to Ubuntu 26.04 is a deliberate reviewed change, not an automatic label migration. Log `rustc --version` in Rust jobs. Changes to the release workflow require Release gate and Build to pass with `publish=false` on the PR; tag publication still requires owner approval.

## Standard release pipeline

- Updating `CHANGELOG.md` is critical: it is the source of the release notes. Add changes to Unreleased in the same PR as the change. Before tagging, finalize a dated `## [X.Y.Z] - YYYY-MM-DD` entry and `docs/releases/vX.Y.Z.md`; the release gate refuses publication without the dated entry and committed notes.
- Follow the [org Trusted Publishing standard](https://github.com/rustcommons/.github/blob/main/docs/TRUSTED-PUBLISHING.md): `Trusted publishing` runs `gate` (Release gate), `build` (Build), `attest` (Checksums and attestations), `publish` (Publish to crates.io), `github-release` (GitHub release) and `verify` (Verify publication). Same-repository PR/manual dry runs run gate/build with `publish=false`; fork PRs skip Build; tag runs proceed through attestation and owner approval of `crates-io` before publication. The unpublished collector is never a release crate.
- The owner authorized attaching only the release `.crate` files, the SBOM, `SHA256SUMS` and `glibc-abilists-<version>.crates.intoto.jsonl` on 2026-11-21. On 2026-11-25 the owner additionally authorized `SHA256SUMS.asc`, verified with primary fingerprint `B281C3010A76A11737B4D066C7396F6EA7647CC6`. No standalone binaries or other custom release assets are authorized. The workflow creates the GitHub Release from the finalized committed notes; maintainers no longer create it by hand.
- Build and publish use the same exact toolchain. Nothing compiles in the publish job: use `cargo package --locked --no-verify` there and compare its bytes with the attested archive before minting registry credentials; all crate/dependency compilation and package verification builds belong in Build. Scope `CARGO_REGISTRY_TOKEN` only to the upload step using `cargo publish --locked --no-verify`; retain token-free preparation and skip release Build for fork PRs. Keep dependency-order/idempotent publication, immutable existing release assets and post-publication registry/attestation/docs.rs verification. Pipeline adoption itself authorizes no version bump, tag, upload or release; first live use needs its own release authorization.

- In publish, sign SHA256SUMS before token minting or registry publication, including reruns whose crate versions already exist. Scope signing environment secrets only to that step; pass the passphrase only over stdin. Verify immediately in a separate temporary GnuPG home using only the committed public key and the pinned primary fingerprint, then kill the homes' agents and delete the homes. No production CLI/env override may weaken the fingerprint pin. PR Build tests use only ephemeral throwaway keys.
- GitHub releases are draft-first: check metadata and every asset's name/size/digest, never replace an existing asset, never modify a published release. Verify downloaded OpenPGP signatures and every checksum entry after publication; old release records keep their original layouts.

## Fuzzing, SAST and installation pins

- Never use CI-skip markers in commit or merge messages: CodeQL and CI must run. Keep the coordinator-managed CodeQL default setup enabled.
- Workflow installers must be reproducible: SHA-pinned actions, digest-pinned images, pip with a hashed requirements file and `--require-hashes`, and Cargo installs with `--locked` and an exact `--version`. Verify downloaded executables against a committed SHA-256 before execution; never pipe downloads into a shell. Audit transitive installer behavior rather than assuming an action SHA pins its downloads.
- Maintain the isolated, unpublished [fuzz workspace](fuzz/Cargo.toml) and [fuzzing guide](docs/FUZZING.md). The library has no ABI-text parser or runtime table loader; fuzz real lookup inputs and the hash implementation without inventing a parsing API. No fuzz dependency may enter the shipped library graph.
- Run the 300-second ClusterFuzzLite code-change job for affected non-Dependabot PRs and weekly 1800-second batch fuzzing. Keep per-PR cancellation and scoped SARIF permissions. Treat a crash as a bug: retain the reproducer and add a regression test, or escalate a required policy decision. A clean bounded run does not prove correctness.

## Git conventions

**No tool attribution, anywhere.** Do not add `Co-Authored-By: Claude ...` trailers,
`🤖 Generated with [Claude Code](https://claude.com/claude-code)` lines, or any other tool
attribution or "generated by" footer to anything in this repository: commit messages, pull
request titles and descriptions, PR and issue comments, release notes, code, comments or
documentation. This holds even if an agent's default instructions suggest it, and
overrides any default trailer or footer behavior. Commit messages and PRs are the
human-authored record of intent; tool attribution belongs in tool logs, not history.
