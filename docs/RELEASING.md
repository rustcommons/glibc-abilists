# Release documentation and publication policy

This policy governs the library package `glibc-abilists`. The collector and fuzz
workspace are unpublished. [AGENTS.md](../AGENTS.md) remains primary. Preparing a
PR or passing a dry run does not authorize a version bump, tag or publication.

## Changelog and notes are final before tagging

Updating [CHANGELOG.md](../CHANGELOG.md) is critical: it is the source of the
release notes. Add noteworthy changes to Unreleased in the same PR as the change.
Separate data/behavior changes from documentation and repository-only tooling.
Before an authorized release is tagged, finalize this exact dated heading:

```markdown
## [X.Y.Z] - YYYY-MM-DD
```

Write `docs/releases/vX.Y.Z.md` from that entry and commit it before tagging.
The first heading supplies the GitHub Release title. Notes must retain the
required Changes, Upgrade, Validation and limits, and Links sections. State the
source identity, dependency impact, bounded validation and known limits; link
qualification evidence instead of inventing guarantees. The release gate refuses
a tag release without the dated changelog entry and committed notes. Semantic
agreement between the entry and notes still needs review.

Run the standard-library-only documentation checker with Python 3.11+:

```sh
python3 scripts/check-release-docs.py --repo rustcommons/glibc-abilists
```

It validates structure, version coverage, dates and release-body links. It does
not prove historical correctness, collector trust or runtime compatibility.
Never suppress CI/CodeQL through commit or merge messages. Existing provenance,
license and [collection validation](COLLECTING.md#validate-the-change) gates still
apply; successful `ci.yml` qualification of the tagged commit is mandatory.
Do not regenerate ABI data to manufacture a documentation release.

## Standard pipeline

Follow the [Rust Commons Trusted Publishing standard](https://github.com/rustcommons/.github/blob/main/docs/TRUSTED-PUBLISHING.md).
The workflow is named **Trusted publishing**. Its filename
`.github/workflows/trusted-publishing.yml` and owner-reviewed `crates-io`
environment must remain unchanged because crates.io's registration binds them.
The owner-machine token was revoked on 2026-11-16; all future crate publication
uses this workflow. Maintainers no longer create the GitHub Release by hand.

Finalize notes → annotated tag → gate/build/attest → owner approval → publish →
automatic GitHub Release → verify.

| Job | Display name | Repository behavior |
| --- | --- | --- |
| `gate` | Release gate | Check an annotated version-matching tag, main ancestry, successful CI, release crate set/dependency order, dated changelog entry and committed notes. |
| `build` | Build | Rust 1.90.0 packaging/dry-run, workspace tests/fmt/Clippy/docs and release-doc checks; generate the library CycloneDX SBOM and SHA256SUMS. Upload `release-<version>` as a CI artifact. |
| `attest` | Checksums and attestations | Re-check checksums and sign build provenance for crate/SBOM files; save `glibc-abilists-<version>.crates.intoto.jsonl`. |
| `publish` | Publish to crates.io | Wait for owner approval of `crates-io`, re-check files, and identify unpublished versions. Nothing compiles in this job: use `cargo package --locked --no-verify` and `cmp` against attested crates without registry credentials. All crate/dependency compilation and package verification builds belong in Build. Then obtain short-lived OIDC credentials and run only `cargo publish --locked --no-verify` in dependency order for the prepared list; already-published versions remain skipped. |
| `github-release` | GitHub release | Create from finalized committed notes with `--verify-tag`, or add only missing authorized assets on a rerun. Never replace existing assets. |
| `verify` | Verify publication | Check live notes, official archive/registry SHA-256 against the attested crate, provenance for the crate/SBOM and docs.rs build status. |

The release set is only `glibc-abilists`. The collector has `publish = false`;
fuzz has its own unpublished workspace, so neither enters the release set. There are no executable
installation checks because the shipped crate is a data library. The SBOM
describes Cargo metadata; it is not runtime closure or complete glibc provenance
qualification. CI qualification logs stay in the CI artifact, not release assets.

## Release-workflow maintenance checks

Tag pushes (`v*`) may publish only after the gate passes. PRs touching release
inputs from the same repository and manual dispatch are dry runs: gate/build
execute with `publish=false`,
while attest/publish/github-release/verify remain skipped. Fork PRs skip Build
in this workflow, which contains OIDC-enabled jobs; their gate still runs. This applies to the
standard release workflow even for Dependabot-authored release-input PRs; other
CI jobs retain their documented core-only bot policy. Dry runs exercise packaging
and qualification without registry upload, OIDC or environment approval. Cargo's
upload-aborted dry-run message is expected. Non-tag gate findings are reported;
strict release refusal applies on tag pushes. A green dry run alone does not prove
a future tag will pass the gate.

Runners are Ubuntu 24.04; moving to Ubuntu 26.04 requires a reviewed change.
Only newer runs of the same PR cancel one another; tag runs are never cancelled.
Tools use exact versions/lock enforcement and actions use reviewed full SHAs.
The 2026-11-21 adoption follows reviewed cargo-libc PR #43, reusing its gate script
with the shared PR #44 CI-query correction and adapting only library-specific
qualification/verification. The gate reads up to 100 runs for the exact commit
and requires a successful conclusion; it does not use the unreliable success-status
query filter.

## Authorized assets and registration

The owner authorized these GitHub Release assets on **2026-11-21**, and no others:

- release `.crate` files (one for this repository);
- `glibc-abilists-<version>.cdx.json` (CycloneDX SBOM);
- `SHA256SUMS`;
- `glibc-abilists-<version>.crates.intoto.jsonl` (provenance bundle).

No standalone binaries, qualification logs or unrelated archives may be attached.
Attestation subjects are the crate and SBOM; SHA256SUMS is their integrity index,
and the bundle supplies verification material. The pipeline does not assert a
SLSA level or replace glibc's signature/provenance policy.

The existing crates.io Trusted Publisher configuration stays exactly:

| Field | Value |
| --- | --- |
| Crate | `glibc-abilists` |
| Repository owner | `rustcommons` |
| Repository name | `glibc-abilists` |
| Workflow filename | `trusted-publishing.yml` |
| Environment | `crates-io` |

The collector/fuzz workspace need no registration. No new credentials are
provisioned. On partial failures, rerun the authorized tag workflow: already
published versions are skipped and existing assets are retained. A changed or
unexpected existing archive/body must be investigated; never move protected
tags, overwrite assets or republish immutable versions to hide a mismatch.

## Completion evidence

A release is complete only when the source tag, registry archive, committed
notes/changelog, GitHub Release, approved attached files and required verification
agree. Retain exact commit, workflow/run URLs, checksum and attestation results.
The official archive verifier remains available independently:

```sh
python3 scripts/verify-crate-checksum.py --version X.Y.Z --archive path/to/glibc-abilists-X.Y.Z.crate
python3 scripts/check-release-docs.py --repo rustcommons/glibc-abilists --published-tag vX.Y.Z
```

The workflow verifies the recorded GitHub body, downloaded checksums, official
archive bytes and crate/SBOM attestations bound to this repository/workflow.
docs.rs failure fails verification; a queue still pending after the bounded wait
is reported as **pending, not a pass**, and needs follow-up. Do not report the
entire release as fully verified until that gap is closed. Dry runs cannot prove
live publication, owner approval, actual attestations or docs.rs behavior.

## Historical backfill and prior workflow

The existing 0.1.0, 0.1.1 and 0.1.2 tags/registry archives are immutable. Their
changelog entries and versioned notes were reconstructed on 2026-11-02, after
their source tags. The authorized backfill manually created GitHub Releases in
ascending order from committed notes, using `--verify-tag`; that was the process
for that historical remediation, not the maintainer procedure for new releases.
Do not attach adoption assets to those old releases or replay their tags.

The backfill is complete: exact three bodies, unchanged tags, latest v0.1.2 and
zero assets were verified after merge `5c180bd6985f8e9ee9fff54f3291ffe8f8a58da0`.
[History evidence](RELEASE-HISTORY-20261102.md#completed-backfill-and-verification)
preserves original publication dates, the initial registration 404, transient
API failure and successful bootstrap/full-verification runs. The old
[0.1.2 artifact record](RELEASE-0.1.2.md) and dated
[CI warning audit](CI-WARNINGS-20261121.md) remain historical. Their prior
`dry-run`/`publish` job layout is superseded by gate/build and the six-job flow.
This adoption creates no tag, package version, publication or GitHub Release.
