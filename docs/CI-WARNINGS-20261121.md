# CI warning audit — 2026-11-21

This maintenance record supports [GA-REQ-020](REQUIREMENTS.md). Baseline main is
`943fa3ad3a1f8c3e02e166d85bfd3d44d061bbfc` (merged PR #13). No source/data,
collector policy, API, package version or lockfile change is required.

## Annotation baseline

Read all job check-run annotations for these main runs through the authenticated
GitHub API (`GET repos/rustcommons/glibc-abilists/actions/runs/ID/jobs`, then
`GET repos/rustcommons/glibc-abilists/check-runs/JOB_ID/annotations`). The sampled
runs had no warning/failure annotations and no Node-20 deprecation annotations;
they had these runner-migration **notice** annotations:

| Run | Notice count | Scope |
| --- | --- | --- |
| [CI 44237392560](https://github.com/rustcommons/glibc-abilists/actions/runs/44237392560) | 5 | Linux Rust, docs, MSRV, package and upstream comparison. |
| [Security 44237392708](https://github.com/rustcommons/glibc-abilists/actions/runs/44237392708) | 2 | zizmor and cargo-deny. |
| [LF 44237392664](https://github.com/rustcommons/glibc-abilists/actions/runs/44237392664) | 1 | Required lf-only job. |
| [Scorecard 44237393254](https://github.com/rustcommons/glibc-abilists/actions/runs/44237393254) | 1 | Scorecard reporting. |
| [Release docs 44237392666](https://github.com/rustcommons/glibc-abilists/actions/runs/44237392666) | 0 | Already Ubuntu 24.04. |
| [Managed CodeQL push 44237393090](https://github.com/rustcommons/glibc-abilists/actions/runs/44237393090) | 3 | Actions, Python and Rust analyses; coordinator-managed default setup. |
| [Newer managed CodeQL schedule 44686220277](https://github.com/rustcommons/glibc-abilists/actions/runs/44686220277) | 0 | No annotation in this later sample. |

Total: **12 notices**, nine in repository-owned workflows and three in the
sampled managed CodeQL push run. Each described the ubuntu-latest migration to
Ubuntu 26. No other annotations were found in these samples. This is a bounded
inventory, not an assertion about every historical/future run. CodeQL default
setup remains configured and unchanged; its runner image is controlled outside
these repository workflow files.

## Action runtime and release-note review

Read `action.yml`/`action.yaml` at each workflow's exact SHA, plus the linked
release notes. All direct JavaScript actions already use Node 24; no Node-20
upgrade or artifact action migration is applicable to this repository.

| Action/version | Runtime | Review outcome |
| --- | --- | --- |
| [checkout v7.0.1](https://github.com/actions/checkout/releases/tag/v7.0.1) | node24 | [v7 changes](https://github.com/actions/checkout/releases/tag/v7.0.0) include fork checkout restrictions for privileged events. Our workflows use ordinary PR events and keep `persist-credentials: false`, overriding the action's true default. |
| [setup-python v7.1.0](https://github.com/actions/setup-python/releases/tag/v7.1.0) | node24 | [v7](https://github.com/actions/setup-python/releases/tag/v7.0.0) removed `pip-install`; we do not use it. Hashed pip installation remains explicit. |
| [codeql-action v4.39.3](https://github.com/github/codeql-action/releases/tag/v4.39.3) | node24 | Existing SARIF paths/categories remain unchanged. |
| [crates-io-auth-action v1.0.6](https://github.com/rust-lang/crates-io-auth-action/releases/tag/v1.0.6) | node24 | Existing tag-only OIDC exchange remains behind the crates-io environment. |
| [Scorecard v2.5.0](https://github.com/ossf/scorecard-action/releases/tag/v2.5.0) | Docker | Not a Node action. |
| ClusterFuzzLite build/run v1 | Docker | Not Node actions; existing SHA pins remain. |

No repository workflow references upload-artifact/download-artifact actions, so
there is no paired-version, artifact naming, merge or extraction-path change.
No open Dependabot PRs existed at the 2026-11-21 inventory; none were closed or
merged. The parked `docs/abi-unwind-release-gate` branch is preserved.

## Runner and compiler changes

Replace every maintained ubuntu-latest/windows-latest reference, including the
bot/full Rust matrix, with `ubuntu-24.04`/`windows-2025`. The fuzzing schedules,
budgets and Dependabot conditions are unchanged. Moving to Ubuntu 26.04 will be
a deliberate reviewed change, not an automatic label migration.

No dtolnay action is used: PR #13 already replaced it with runner-provided rustup,
self-updates disabled and exact versions. This PR adds explicit `rustc --version`
output to all Rust setup jobs and compiler/active-toolchain output to the fuzz
builder. Expected compilers are 1.99.0 for normal CI, 1.90.0 for MSRV/security
and release dry-run/publishing, and nightly-2025-09-05 (1.91.0-nightly) for fuzzing.

The baseline CI/security logs above show these intended 1.99.0/1.90.0 versions.
The earlier [MSRV job 133338036532](https://github.com/rustcommons/glibc-abilists/actions/runs/43878574241/job/133338036532)
also logged rustc 1.90.0 and no unexpected-toolchain-input warning. Its former
SHA's metadata declares the toolchain input. No wrong compiler was observed in
these samples; this does not retroactively qualify uninspected historical jobs.

## PR release dry-run and remaining evidence

The release workflow now has a separate credential-free PR job using Rust 1.90.0
and `cargo publish --dry-run --locked -p glibc-abilists`. It does not request OIDC,
enter the crates-io environment or upload a crate. The real publishing job has an
explicit tag-push guard. Only superseded PR runs cancel; tag publishing retains
its previous non-cancelling behavior. Cargo's upload-aborted dry-run log message
is expected and does not represent a failed real publication.

Candidate CI results, compiler logs and after-annotations are **pending** at PR
preparation. The coordinator will relay results; no CI polling/watch process is
started. A green release dry-run and an annotation recheck are required before
closure. Scorecard and weekly batch are not exercised by this PR event; their
fixed runner labels are source-checked, with live results awaiting main/schedule.
Real OIDC/registry publication is not exercised or authorized by this patch.
