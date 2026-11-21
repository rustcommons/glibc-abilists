# Roadmap

Open product work is listed below. The completed documentation cleanup is
recorded in the [release plan](RELEASE-PLAN.md#documentation-cleanup-completed-on-2026-10-31);
docs-only 0.1.2 is published, with delivered API corrections and evidence in the
[release record](RELEASE-0.1.2.md).

- [ ] Add each new glibc release (2.45 and later) when it is tagged: collect it with
  the collector, which checks its signature and provenance, and release a new
  version.
- [ ] Other architectures (aarch64 first), each from glibc's ABI lists for that
  architecture.
- [ ] Other C libraries, when a consumer needs them.

See the [next-release plan](RELEASE-PLAN.md) for scope, dependency order, and release gates.

## Offline provenance follow-up

- [ ] **Priority 1 — before relying on offline regeneration as release evidence:**
  evaluate a separate collection-validation record that identifies the exact
  candidate inputs/outputs and says whether upstream/mirror comparison was
  performed or skipped. Keep offline write supported and deterministic dataset
  formats unchanged unless a separate policy/schema decision is approved.
  Specify how a successful online `--check` supplies evidence for the same
  candidate without changing its generated data. Acceptance criteria: exercise
  online/offline check and write paths, including glibc 2.30's exception; prove
  that an offline record cannot imply completed remote verification, and that
  stale evidence cannot silently apply to changed outputs. Document any remaining
  local-clone trust requirement. As part of this task, evaluate whether exception
  handling should require a signature and match the configured signing key;
  cover unsigned annotated tags, missing GPG key information and mismatched
  keys in proposed acceptance tests. Such enforcement would change the current
  exception contract and needs an explicit policy decision before implementation.
  The current behavior and CI mitigation are
  explained in [Provenance](PROVENANCE.md#offline-collection-and-trust).

This is collector hardening/documentation follow-up, not a confirmed library
runtime defect or a finding that the shipped data is compromised. No exception,
collection, or publication policy is changed by this item.

## Standard release pipeline adoption — 2026-11-21

The owner-approved [org pipeline](https://github.com/rustcommons/.github/blob/main/docs/TRUSTED-PUBLISHING.md)
is implemented in `trusted-publishing.yml`, adapted from reviewed cargo-libc
PR #43. Its generic `scripts/release-gate.py` is reused unchanged. This repository
releases only `glibc-abilists`; the collector and fuzz workspace remain unpublished.
Existing release documentation, workspace checks and the credential-free packaging
dry-run are folded into Build. The authorized GitHub assets are the crate,
CycloneDX SBOM, `SHA256SUMS` and `<repo>-<version>.crates.intoto.jsonl`; see the
[release policy](RELEASING.md). No version, tag or release is created by adoption.

- [ ] Complete adoption review after the PR's Release gate and Build dry run are
  green with `publish=false`; retain the exact candidate and run evidence.
- [ ] At the next separately authorized release, exercise attestation, owner
  environment approval, crates.io publication, automatic GitHub Release creation
  and live verification. These are not proven by a dry run or by the historical
  0.1.2 release. Do not replay old tags or attach new assets to old releases.
- [ ] Record the docs.rs build result after that release; bounded waiting can
  report pending, which is not success and remains follow-up work.

Binary workflow consolidation (org phase 2) and clean executable installation
checks are not applicable here: this repository ships a data library, with no
binary release workflow. The existing architecture/libc and offline-provenance
roadmap above is preserved; adoption does not resolve those product decisions.
