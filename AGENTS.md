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

## Scope and implementation

- Current dataset: glibc only, x86_64 (64-bit ABI) only, releases 2.17-2.44.
  Other architectures and libc implementations are planned.
- Data comes only from glibc's upstream repository at release tags. Never edit
  files under `data/` or `src/generated.rs` by hand; rerun the collector.
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

## ABI-data releases and downstream unwinding

- Before releasing a changed ABI-list dataset or adding a glibc release, complete the [ABI-data unwind-impact checklist](docs/ABI-UNWIND-RELEASE-CHECKLIST.md). Data-only changes are not exempt: record exact input/runtime/downstream refs and hashes, LLVM-runtime libc symbol/version use, retained Cargo glibc 2.17 control and Aero consumer 2.22 floor comparisons, new-release linkage, and relevant fresh producer/Cargo tests including shared/PyO3 regressions, native C `-fexceptions` pthread_exit/deferred-cancellation cleanup and valid bidirectional `C-unwind` roundtrips.
- Missing required evidence, unresolved driver/ref integration or unexplained floor/closure regressions block release. Documentation-only changes with identical relevant payloads may cite explicitly scoped prior evidence. Runtime direction remains pending user choice; no LLVM default is approved. Any proposed static LLVM selection is distinct from observed runtime closure; do not promise that libgcc can never load. Shared/Python behavior, legal, Tier 3 and offline-provenance limits are unchanged.

## Git conventions

**No tool attribution, anywhere.** Do not add `Co-Authored-By: Claude ...` trailers,
`🤖 Generated with [Claude Code](https://claude.com/claude-code)` lines, or any other tool
attribution or "generated by" footer to anything in this repository: commit messages, pull
request titles and descriptions, PR and issue comments, release notes, code, comments or
documentation. This holds even if an agent's default instructions suggest it, and
overrides any default trailer or footer behavior. Commit messages and PRs are the
human-authored record of intent; tool attribution belongs in tool logs, not history.
