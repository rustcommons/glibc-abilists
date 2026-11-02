# Requirements and evidence

## Scope and sources

This index records confirmed scope from [AGENTS.md](../AGENTS.md), the public
[API](../src/lib.rs), [manifest](../Cargo.toml), [upstream selection](../upstream.toml),
[collector](../collector/src/main.rs), and maintained guides. It introduces no
new collector or compatibility policy. Requirements were indexed on 2026-11-01;
IDs GA-REQ-001 onward are stable and must never be renumbered or reused.

Evidence below is bounded to the named checks and references. The baseline is
`8abbd27040c770c858d55f6647960f23a1d80e9e`; its
[CI run](https://github.com/rustcommons/glibc-abilists/actions/runs/41271487763)
passed on 2026-11-01. This historical run does not qualify a future candidate.
At initial index preparation, final 0.1.2 candidate checks had not yet run.
The [release record](RELEASE-0.1.2.md) now records their passing commands/CI,
exact candidate and released commits, official payload and live API-doc evidence.
It supersedes preparation-time gaps without changing those historical outcomes.
No test count establishes universal compatibility or upstream authenticity.

Shared-link resolution on 2026-11-01: the convention was unavailable on central
`main` during index preparation. It is now live at rustcommons/.github commit
`c8a0b9c01aae5657dd0ef649bd37b18d81919ce5`. The Contents API confirms
`docs/DOCUMENTATION.md` (blob `1b659ff536d43bcae5b09626069e89488058ba6f`),
and its final GitHub URL returns HTTP 200. This resolves the initial shared-link
gap without changing the earlier preparation observations or shared AGENTS text.

## Requirements

| ID | Requirement and source | Implementation / documentation | Verification evidence | Status and gaps |
| --- | --- | --- | --- | --- |
| GA-REQ-001 | Embed unmodified upstream ABI lists for x86_64 Linux's 64-bit ABI, glibc 2.17–2.44; AGENTS scope and `upstream.toml`. | `data/`, `src/generated.rs`; [consumer guide](USAGE.md). | Baseline CI on 2026-11-01: online collector `--check`; `src/lib.rs::tests::covers_every_release_from_2_17_to_2_44` and `every_embedded_file_matches_its_recorded_hash`. | Verified (bounded named evidence): baseline selected files/range and recorded hashes; not distribution patches or runtime compatibility. The 2026-11-01 [release comparison](RELEASE-0.1.2.md#official-artifact-and-source-evidence) confirms unchanged data against official 0.1.1. |
| GA-REQ-002 | Std-only library and unpublished collector; Rust 1.90 minimum; manifests and AGENTS. | `Cargo.toml`, `collector/Cargo.toml`, `Cargo.lock`; [setup](../README.md#getting-started). | Baseline CI `msrv`: Rust 1.90.0 `cargo test --workspace --locked --offline`; manifests have no crate dependencies and collector has `publish = false`. | Verified (bounded named evidence): baseline MSRV tests and manifest inspection on 2026-11-01; Rust 1.90 local gates and final-candidate MSRV CI passed; see the [release record](RELEASE-0.1.2.md#candidate-and-merged-package-validation). |
| GA-REQ-003 | Provide static raw-data lookup/enumeration; unknown releases/libraries return `None`; public API. | `release`, `releases`, `abilist`, `abilists`; [consumer guide](USAGE.md#find-and-enumerate-data). | Baseline CI runs `src/lib.rs::tests::lookups_return_none_outside_the_data` and README/API doctests via `cargo test --workspace --locked --offline`. | Verified (bounded named evidence): documented examples and selected missing lookups; not an exhaustive input proof. |
| GA-REQ-004 | `AbiList::verify` compares SHA-256 of embedded text to its record; public API. | `src/lib.rs`, `src/sha256.rs`; [verification limits](USAGE.md#interpret-verification). | Baseline CI: `every_embedded_file_matches_its_recorded_hash`; `src/sha256.rs::tests::known_vectors`. | Verified (bounded named evidence): embedded records and named hash tests; no signature or network verification at runtime. |
| GA-REQ-005 | Record release/file identities and license evidence; AGENTS and provenance format. | `data/releases.tsv`, `data/files.tsv`, `Release`, `AbiList`; [Provenance](PROVENANCE.md). | Baseline CI: `provenance_fields_are_complete` checks selected field shape/nonempty reason; online collector `--check` checks recorded outputs. | Verified (bounded named evidence): baseline output comparison and selected fields; records do not say whether a run skipped remote checks. |
| GA-REQ-006 | Online collection compares upstream tag identity and available mirror tags; collector and upstream configuration. | `collector/src/main.rs::check_remote`, `upstream.toml`; [collection guide](COLLECTING.md). | Baseline CI online collector `--check` in `.github/workflows/ci.yml` passed 2026-11-01. | Verified (bounded named evidence): that run's configured tags/remotes; no continuous authentication guarantee or offline comparison. |
| GA-REQ-007 | Verify signatures with checked-in keys; permit configured tag/reason exceptions subject to existing checks; current collector contract. | `collect_release`, `Keys::load`, `keys/`; [signature limits](PROVENANCE.md#signatures), `Signature::Exception` docs. | Source inspection on 2026-11-01: `ERRSIG` field can be empty; exception lookup uses tag/reason, not configured key. Baseline online check covers current dataset only. | Implemented—partial verification: no dedicated unsigned/mismatched-key exception tests; signature presence and configured-key match are not enforced. Historical 2.30 evidence is not a stronger contract. |
| GA-REQ-008 | Keep offline write supported; skip remote comparisons with `--offline`; current collector options. | `run`, `collect_release`; [offline trust](PROVENANCE.md#offline-collection-and-trust), [options](COLLECTING.md#options). | Source/parser inspection on 2026-11-01; no adversarial offline check/write run performed for this documentation release. | Implemented—partial verification: trusted local inputs required; copied exception reasons do not attest fallback checks. |
| GA-REQ-009 | Never hand-edit data/generated tables; compare without rewriting via `--check`; AGENTS. | Collector `render`, `compare`, `write`; [rewrite warning](COLLECTING.md#rewrite-deliberately). | Baseline CI online `--check`; source inspection on 2026-11-01 confirms destructive, non-atomic write path. | Implemented—partial verification: no fault-injection/atomicity testing; no regeneration authorized for 0.1.2. |
| GA-REQ-010 | Preserve LGPL data/crate and MIT collector license/notice policy; AGENTS and REUSE. | `LICENSES/`, `NOTICE.md`, `REUSE.toml`, `LICENSE_TEXT`; [redistribution](USAGE.md#redistribute-files). | Baseline CI: `license_text_is_the_lgpl_2_1`, package job; 2026-11-01 [official archive inspection](RELEASE-0.1.2.md#official-artifact-and-source-evidence) confirms license/notice inclusion. | Verified (bounded named evidence): baseline license marker/package checks; not a legal review. |
| GA-REQ-011 | Package the library/data/README/notices/licenses; exclude collector, keys and repository-only guides; manifest include list. | `Cargo.toml`; absolute GitHub guide links in README; [release plan](RELEASE-PLAN.md). | Baseline CI `cargo package --locked` and `cargo package --list --locked`; 2026-11-01 [release inspection](RELEASE-0.1.2.md) verifies the 404-file official archive, links and exact clean merged VCS metadata. | Verified (bounded named evidence): candidate, clean merged package and official archive contents; repository-only guides remain excluded. |
| GA-REQ-012 | Keep AGENTS primary, CLAUDE an exact shim, no attribution; existing repository conventions. | [AGENTS](../AGENTS.md), [CLAUDE](../CLAUDE.md), [shared documentation convention](https://github.com/rustcommons/.github/blob/main/docs/DOCUMENTATION.md). | File/diff inspection against baseline on 2026-11-01; `gh api repos/rustcommons/.github/contents/docs/DOCUMENTATION.md` confirms the live file at central commit `c8a0b9c01aae5657dd0ef649bd37b18d81919ce5`; final GitHub URL HTTP 200 on 2026-11-01. | Verified (bounded named evidence): instruction/shim diff and final shared-link checks; no tool identity belongs in repository artifacts. |
| GA-REQ-013 | Consider separate online/offline validation evidence and exception signature/key enforcement only after a policy decision; roadmap. | [Offline provenance follow-up](ROADMAP.md#offline-provenance-follow-up). | No implementation or acceptance runs; roadmap is a decision proposal. | Planned: no new validation schema or enforcement authorized by this documentation release. |
| GA-REQ-014 | Other architectures/libcs and later glibc releases are future work; roadmap and current selection. | [Roadmap](ROADMAP.md); collector rejects other targets. | `upstream.toml`/parser inspection on 2026-11-01; no target/runtime qualification beyond current scope. | Planned: no additional dataset or compatibility claim. |
| GA-REQ-015 | Runtime compatibility certification, parsing, stub generation and installation are outside this library's role; README. | [Consumer guide](USAGE.md#read-raw-text), raw accessor API. | API/source inspection on 2026-11-01; not a runtime qualification suite. | Not applicable: no such features or guarantees are required of this crate. |
| GA-REQ-016 | Maintain user-facing changelog/versioned notes and release policy; verify release-documentation CI and live GitHub Release completion; 2026-11-02 authorization and [AGENTS release documentation](../AGENTS.md#release-documentation). | [Changelog](../CHANGELOG.md), `docs/releases/vX.Y.Z.md`, [policy](RELEASING.md), [history evidence](RELEASE-HISTORY-20261102.md), [checker](../scripts/check-release-docs.py) and [workflow](../.github/workflows/release-docs.yml). | 2026-11-02: reviewed bootstrap merged at `5c180bd6985f8e9ee9fff54f3291ffe8f8a58da0`; [bootstrap run 41387759314](https://github.com/rustcommons/glibc-abilists/actions/runs/41387759314) and [full published run 41387980296](https://github.com/rustcommons/glibc-abilists/actions/runs/41387980296) succeeded. Authenticated `--published` passed; exact three live note bodies, unchanged source tags, latest v0.1.2 and zero assets verified. [Completion evidence](RELEASE-HISTORY-20261102.md#completed-backfill-and-verification) preserves preparation, registration 404 and transient API observations. | Verified (bounded named evidence): release-documentation bootstrap and live verification completed for 0.1.0, 0.1.1 and 0.1.2. No release-event runs were recorded; the explicit full-history workflow verified all releases. Future routine merges still require the registered check. No new software/crate publication or runtime qualification is implied. |
| GA-REQ-017 | Changed ABI-list data/new glibc releases require downstream unwind-impact assessment and fresh relevant qualification; 2026-11-02 user direction and [repo-specific release gate](../AGENTS.md#abi-data-releases-and-downstream-unwinding). | [ABI-data checklist](ABI-UNWIND-RELEASE-CHECKLIST.md), [release policy](RELEASING.md#abi-data-changes-downstream-unwind-impact-gate); exact refs/hashes, LLVM-runtime libc symbol/version map, 2.17/2.22/new-release comparison, producer/Cargo/shared/PyO3 and closure evidence. | Documentation proposal only on 2026-11-02 at base `896783bf4f6e5e48c0954e897b98dfd3b39b003b`; local links/render/diff and release-doc structure CI are PR evidence, not runtime qualification. No candidate ABI payload or 0.4.0 runtime tests were run. | Implemented—partial verification: checklist text and blocking rule drafted; canonical consumer/producer refs and final driver commands must be integrated before merge. Fresh required downstream results remain release blockers. Data-only is not exempt; identical-payload documentation reuse is scoped. |

## Documentation map

| Document | Maintained purpose |
| --- | --- |
| [README](../README.md) | Purpose, Rust prerequisite, dependency setup/example, absolute guide links for package readers. |
| [Consumer guide](USAGE.md) and API rustdoc in `src/lib.rs` | Lookup/raw formats, evidence interpretation, redistribution. |
| [Collection guide](COLLECTING.md) | [Collection flow and trust boundaries](COLLECTING.md#collection-flow-and-trust-boundaries), maintainer tools, commands, offline limits, non-atomic regeneration warning, validation. |
| [Provenance](PROVENANCE.md) | Recorded evidence versus enforcement, historical key findings and offline trust limits. |
| [Roadmap](ROADMAP.md) | Unimplemented work and policy decisions. |
| [Changelog](../CHANGELOG.md) and [versioned release notes](releases/v0.1.2.md) | User-facing shipped history and upgrade/validation limits; notes for each existing tag are backfilled, not historical-tag files. |
| [ABI-data unwind-impact checklist](ABI-UNWIND-RELEASE-CHECKLIST.md) | Candidate evidence fields, floor/new-release and downstream regression/closure gates; final cross-repository driver refs pending. |
| [Release policy](RELEASING.md) and [history reconstruction](RELEASE-HISTORY-20261102.md) | Maintained publication/completion contract and dated source/registry/CI evidence for the backfill. |
| [Release plan](RELEASE-PLAN.md) and [0.1.2 release record](RELEASE-0.1.2.md) | Published status, exact artifact/docs evidence and separately labeled historical planning/candidate evidence. |
| [AGENTS](../AGENTS.md) and [CLAUDE](../CLAUDE.md) | Primary technical/contribution rules and unchanged redirect shim. |
| [NOTICE](../NOTICE.md), [REUSE annotations](../REUSE.toml) and `LICENSES/` | Licensing, attribution/credit history and redistribution evidence. |
| [Release-doc checker](../scripts/check-release-docs.py) and [workflow](../.github/workflows/release-docs.yml) | Local structure/coverage gate; optional live published-body/latest verification. |
| [CI](../.github/workflows/ci.yml), manifests and REUSE annotations | Actual commands/platforms, package selection/version/MSRV, licensing metadata. |

## Maintenance

Follow the [shared documentation convention](https://github.com/rustcommons/.github/blob/main/docs/DOCUMENTATION.md).
Update affected rows and guides together. Keep evidence tied to its command,
date and revision; record not-run gaps explicitly. Preserve historical snapshots
and immutable published-package facts. Add new GA-REQ IDs without renumbering
or reusing old IDs; retain retired IDs with their disposition. Planned policy
changes need a separate decision before implementation. Release approval,
publication and post-publication verification are separate gates.
