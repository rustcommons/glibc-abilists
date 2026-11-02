# Release-history reconstruction — 2026-11-02

This is evidence for the backfilled root changelog and user-facing version
notes, not new software qualification. Inventory base:
`a2b630413bd17a4d9a729e91691ff1a4d46c2719`. Existing tags were fetched from
GitHub; no tag was moved. `gh release list` returned no GitHub Release entries.
At that initial inventory, documentation publication was pending review/coordinator
GO. The completed backfill is recorded below; the inventory is historical.

## Original identities and registry availability

Dates are original crates.io publication times in UTC, not the new backfill
publication time. All three versions were present and non-yanked in both the
registry API and sparse index when fetched on 2026-11-02.

| Version | Original registry timestamp | Source commit / archive VCS | Official archive SHA-256 |
| --- | --- | --- | --- |
| 0.1.0 | 2026-10-11T01:05:27.072634Z | `4ffb9f698264d7d8bd834f6f2b5e03926bdc02fb` | `f25a3e79840da5cfb915292689f0b269fd941de4e87517c17d0199acde662196` |
| 0.1.1 | 2026-10-14T12:00:16.913597Z | `d7e5228dae5b2591fc7849a9b82607a7d6bc25bb` | `b09a1df98771a4aaaa6523290bf42c3bfd89962e5e621b6d57c221c198bfbf8b` |
| 0.1.2 | 2026-11-01T04:32:05.239152Z | `d0cf8ee40529880373286c500dd22b9c21d79e2c` | `0f76191d504fe7944029bf5ac1e3ee901b050e7ce3fc946fa2661a9094045287` |

Version API sources: [0.1.0](https://crates.io/api/v1/crates/glibc-abilists/0.1.0),
[0.1.1](https://crates.io/api/v1/crates/glibc-abilists/0.1.1),
[0.1.2](https://crates.io/api/v1/crates/glibc-abilists/0.1.2).
Independent checksum source: [sparse index](https://index.crates.io/gl/ib/glibc-abilists).
Archives were fetched from `https://static.crates.io/crates/glibc-abilists/glibc-abilists-VERSION.crate`.
Python `hashlib.sha256` matched both checksum fields for each download;
`tarfile` inspection found 404 files per archive, clean matching source VCS,
and byte-identical archived `src/` and `data/` files against each own tag.

Annotated tag metadata from `git for-each-ref`:

| Tag | Tag object | Original tagger timestamp |
| --- | --- | --- |
| v0.1.0 | `8503dbc9893a6b83b044156bf795f783db21ff02` | 2026-10-10T20:10:37-05:00 |
| v0.1.1 | `a0c497de6eed617c75646a01190ea30760620498` | 2026-10-14T07:00:22-05:00 |
| v0.1.2 | `da2cd5edd5c7ea11f8d955d5b9f526217e0fbc4f` | 2026-11-01T04:33:06Z |

The first tag's local calendar date differs from the registry UTC date; both
timestamps identify 2026-10-11 in UTC. This backfill does not alter either.

## Shipped changes reconstructed from source

- `git show v0.1.0:Cargo.toml`, `src/lib.rs`, README and initial source history:
  first std-only Rust 1.90 library, x86_64 Linux glibc 2.17–2.44, raw static
  lookups, provenance, content-hash verification and embedded LGPL text.
- `git diff v0.1.0 v0.1.1`: public field/lookup docs, missing-doc warnings,
  README doctest inclusion, selected missing-lookup tests, docs.rs metadata,
  rustdoc CI and navigation. No exported API or runtime behavior change is
  identified in the diff; new tests/lints are not new runtime features.
- `git diff v0.1.1 v0.1.2`: corrected exception docs and descriptive metadata,
  revised README/NOTICE and repository guides/traceability. The code never
  enforced signature presence or configured-key matching for exceptions.
  Clarifying that fact does not claim data compromise or policy hardening.
- `git diff v0.1.0 v0.1.2 -- data src/generated.rs src/sha256.rs collector keys upstream.toml LICENSES`
  is empty. No ABI/data, generated/hash implementation, collector, key,
  upstream-selection or license-text change is attributed to either patch.
- The collection-flow diagram landed after v0.1.2, at main merge
  `a2b630413bd17a4d9a729e91691ff1a4d46c2719`; it belongs under Unreleased
  repository documentation, not the 0.1.2 shipped-file list.

## Retained historical validation

Read on 2026-11-02 using `gh run view RUN_ID --json headSha,createdAt,conclusion,jobs`:

| Version / exact source ref above | Retained run | Result and bounded coverage |
| --- | --- | --- |
| 0.1.0 | [38100949010](https://github.com/rustcommons/glibc-abilists/actions/runs/38100949010), 2026-10-11 | Success: Linux/Windows Rust, MSRV, package and online data-match jobs. No separate rustdoc job in that workflow. |
| 0.1.1 | [38560616062](https://github.com/rustcommons/glibc-abilists/actions/runs/38560616062), 2026-10-14 | Success: the preceding jobs plus docs. |
| 0.1.2 | [41273530088](https://github.com/rustcommons/glibc-abilists/actions/runs/41273530088), 2026-11-01 | Success: Linux/Windows Rust, MSRV, package, docs and online data-match jobs. |

The workflow at each tag specifies fmt, workspace tests, Clippy, Rust 1.90 MSRV,
package verification and the online collector `--check`; 0.1.1 adds warning-denied
rustdoc. These retained outcomes do not prove arbitrary runtime compatibility,
distribution-patched glibc behavior or continuous provenance authentication.
No fresh Rust suite or data collection was run for this Markdown reconstruction.
Original local release logs beyond retained CI are not established here.
The [0.1.2 detailed record](RELEASE-0.1.2.md) remains unchanged evidence for its
separately recorded local/package/docs.rs validation.

## Editorial reference and initial completion gap

[Aero1553's changelog](https://github.com/joey-huckabee/aero1553/blob/main/CHANGELOG.md)
and [v5.8.0 notes](https://github.com/joey-huckabee/aero1553/releases/tag/v5.8.0)
were inspected for concrete user impact, upgrade guidance and evidence limits.
No Aero1553 features, binary distribution policy or validation claims are
adopted by this library's notes.

Backfilled files did not exist at the historical tags. They use absolute `main`
documentation links so future GitHub Release bodies do not point into missing
historical files. Current changelog/note/policy verification and the shared
release-documentation workflow result belonged in the remediation PR. At that
pre-publication stage, live GitHub Release URL/body/latest/asset checks were
pending authorized publication; none was an original historical outcome.
Completion evidence follows without changing the initial observation.

## Initial workflow registration evidence

At candidate `af09555da7741d478d677139c5b53abd36e2c618` on 2026-11-02,
`python3 scripts/check-release-docs.py --repo rustcommons/glibc-abilists` passed
locally for three versions/current 0.1.2. Actionlint 1.7.12 validated the supplied
workflow YAML/schema (`-shellcheck='' -pyflakes=''`); both shared files matched
the coordinator copies byte-for-byte. Markdown rendering, paths/anchors and
protected-file comparisons also passed. No fresh Rust suite was run.

The explicit candidate dispatch command was:

```sh
gh workflow run release-docs.yml --repo rustcommons/glibc-abilists --ref docs/release-history-20261102 -f verify_published=false
```

It returned `HTTP 404: workflow release-docs.yml not found on the default branch`.
This is the expected first-install registration constraint, not a candidate CI
pass. The 2026-11-02 shared AGENTS v2 contract requires independent review of
local checker/syntax evidence before an authorized normal merge registers the
workflow. A green post-merge docs-workflow run is then mandatory before
completing bootstrap or publishing any GitHub Release notes. Future routine
merges require the registered check. At this preparatory stage, post-merge CI and
live publication checks were pending; their completed outcomes follow.

## Completed backfill and verification

On 2026-11-02, normal bootstrap merge
`5c180bd6985f8e9ee9fff54f3291ffe8f8a58da0` preserved reviewed head
`383cede574cd126487d13b446d72e968dd48acb6` with an identical tree.
[Bootstrap docs run 41387759314](https://github.com/rustcommons/glibc-abilists/actions/runs/41387759314)
succeeded before publication. The unrelated automatically triggered Rust run
was cancelled; no new Rust qualification is claimed for this backfill.

Releases were created in ascending order using the committed note files,
`--notes-file` and `--verify-tag`; older releases used `--latest=false`, and
0.1.2 used `--latest`. Original source tags and registry artifacts did not change.

| GitHub Release | Actual backfill publication time (UTC) | Verified live state |
| --- | --- | --- |
| [v0.1.0](https://github.com/rustcommons/glibc-abilists/releases/tag/v0.1.0) | 2026-11-02T04:19:23Z | Exact committed body, unchanged source tag, published/non-draft/non-prerelease, zero assets. |
| [v0.1.1](https://github.com/rustcommons/glibc-abilists/releases/tag/v0.1.1) | 2026-11-02T04:19:24Z | Exact committed body, unchanged source tag, published/non-draft/non-prerelease, zero assets. |
| [v0.1.2](https://github.com/rustcommons/glibc-abilists/releases/tag/v0.1.2) | 2026-11-02T04:19:25Z | Exact committed body, unchanged source tag, published/non-draft/non-prerelease, zero assets; latest. |

The first unauthenticated full `--published` check failed with
`Missing published GitHub Release: v0.1.2` after all three creations succeeded.
Authenticated API reads showed all three Releases. The initial failure remains
a transient API-visibility observation; stale public data is plausible but its
precise cause was not established. No Release or tag was edited or recreated.

A retry with existing authentication supplied only through the child process
environment passed `python3 scripts/check-release-docs.py --repo rustcommons/glibc-abilists --published`
for all three versions/current 0.1.2; no token was printed or persisted.
[Full-history workflow 41387980296](https://github.com/rustcommons/glibc-abilists/actions/runs/41387980296),
explicitly dispatched with `verify_published=true`, succeeded on the same merge
commit, including all published-body/latest checks. Final read-only API checks
confirmed exact bodies, unchanged tag commits, latest 0.1.2 and zero assets;
local/remote main were clean at the merge commit.

No release-event workflow runs were recorded during inspection. The explicit
full-history workflow supplies the verification gate for these three Releases;
it does not claim historical-tag release-event triggering was exercised. Future
routine merges still require registered release-doc CI. These outcomes concern
release documentation, not new runtime compatibility or old test outcomes.
