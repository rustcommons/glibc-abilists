# Changelog

Noteworthy changes to `glibc-abilists`, the Rust library containing unmodified
x86_64 Linux glibc ABI lists. Dates below are original crates.io publication
dates in UTC. Entries and versioned notes were reconstructed on 2026-11-02;
they were not present at the historical source tags. See the
[history evidence](https://github.com/rustcommons/glibc-abilists/blob/main/docs/RELEASE-HISTORY-20261102.md)
and [release policy](https://github.com/rustcommons/glibc-abilists/blob/main/docs/RELEASING.md).

## [Unreleased]

### Repository

- Use the unified attestation action for release provenance, preserving subject
  paths, bundle outputs, permissions and workflow-bound verification.

- Keep all release compilation/verification in Build. Repackage without
  verification and compare attested bytes in publish, then scope credentials
  only to the no-verify upload step. Retain already-published version skips and
  skip release Build for fork PRs.

- Adopt the standard Trusted Publishing pipeline with release gate, Build,
  attestations, owner-approved crates.io publication, automatic GitHub Release
  creation and verification. Use dated changelog entries as the source of
  release notes and attach only the authorized crate/SBOM/checksum/provenance
  files. The collector remains unpublished; adoption performs no release.

### Collector

- Replace the archived GitHub cross-check endpoint with upstream's maintained
  GNU Toolchain GitLab mirror, retaining sourceware as primary. Retry selected
  transient remote-ref query errors with bounded backoff and distinguish source
  unavailability from tag mismatch; both still block online collection after
  the applicable checks. Dataset, signature/exception policy and library API
  are unchanged; the collector remains unpublished.

### Repository security

- Pin CI runners to Ubuntu 24.04/Windows 2025, report Rust compiler versions,
  and validate release-workflow PRs with a credential-free Cargo publish dry-run.
  No package, data or release version changes are involved.

- Add isolated library-input and hash-padding fuzz targets, seeded corpora and
  digest-pinned ClusterFuzzLite builds with PR/batch fuzzing and crash SARIF.
  The library still does not parse ABI text or load runtime tables.
- Pin zizmor wheels by hash and cargo-deny installs by exact locked version;
  retire CI-skip message guidance so CI and managed CodeQL remain enabled.

- Add monthly grouped dependency/action updates (two open version PRs per ecosystem), pinned workflow actions, zizmor
  and cargo-deny gates, Scorecard reporting and an owner-reviewed Trusted Publishing
  workflow with archive checksum verification. Installation does not publish a
  version or change ABI data; future crate publication uses the owner-approved
  Trusted Publishing environment exclusively after owner-machine token revocation.
- Cancel superseded PR checks and keep Dependabot PRs on core checks with one
  Linux Rust job; normal PRs, main and scheduled runs retain qualification.
  Dependabot security updates remain immediate, independent of the monthly
  version-update schedule.

### Documentation

- Add a collection/delivery diagram explaining online/offline checks, exception
  limits, compare versus rewrite, and the embedded-data API. This repository-only
  guide was added after 0.1.2; it is not in that published archive.
- Backfill this changelog and user-facing notes for existing 0.1.0, 0.1.1 and
  0.1.2 tags. Add release-documentation policy, traceability, a standard-library-only
  checker and dedicated CI workflow. The existing-tag GitHub Releases are now
  published and verified; no new package version is involved.

## [0.1.2] - 2026-11-01

### Documentation

- Correct `Signature::Exception` API wording: a recorded exception does not
  establish signature presence or a match to the configured exception key.
  The collector already behaved this way; this is not new enforcement or a
  claim of compromised data.
- Ship the revised README, dependency/API links and clearer package description.
  Link maintained usage, collection, provenance, requirements and release guides;
  repository-only guides remain outside the crate archive.
- Clarify online/offline provenance and historical exception evidence in the
  repository guides. API behavior, ABI data, generated tables, keys, upstream
  selection and collector policy are unchanged from 0.1.1. No downstream
  minimum requirement or lock update is needed solely for this docs-only patch.

[0.1.2 notes](https://github.com/rustcommons/glibc-abilists/blob/main/docs/releases/v0.1.2.md)

## [0.1.1] - 2026-10-14

### Documentation

- Document previously undocumented public fields and lookup behavior, including
  unknown releases/libraries returning `None`.
- Add a documentation URL and explicit docs.rs target configuration. Enable
  missing-documentation warnings and a warning-denied rustdoc CI job.
- Exercise the README as a doctest and add selected missing-lookup tests. Public
  API/runtime behavior and embedded data are unchanged from 0.1.0. The exception
  wording in this version still overstates guarantees; 0.1.2 corrects it.

[0.1.1 notes](https://github.com/rustcommons/glibc-abilists/blob/main/docs/releases/v0.1.1.md)

## [0.1.0] - 2026-10-11

### Added

- First published std-only Rust library of glibc's own unmodified ABI lists for
  the x86_64 Linux 64-bit ABI, glibc 2.17 through 2.44.
- Static release/library lookups and enumeration, raw ABI text, recorded release
  and file provenance, and `AbiList::verify()` for content-hash comparison.
- Embedded LGPL text for redistribution. The crate/data are LGPL-2.1-or-later;
  the repository's MIT collector is unpublished. Rust 1.90 is the minimum.

The lists are upstream declarations, not a parser, stub generator or runtime
compatibility certificate. Signature exceptions and offline collection limits
already exist in this version; see the current provenance guide before relying
on historical wording.

[0.1.0 notes](https://github.com/rustcommons/glibc-abilists/blob/main/docs/releases/v0.1.0.md)
