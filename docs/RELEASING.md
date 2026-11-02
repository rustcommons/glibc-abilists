# Release documentation and publication policy

This policy governs the library package `glibc-abilists`. The collector is
unpublished. [AGENTS.md](https://github.com/rustcommons/glibc-abilists/blob/main/AGENTS.md)
remains primary; package publication, tags and GitHub Release publication need
authorization. Preparing a PR or passing checks does not provide it.

## Keep the reader record current

Update the root [changelog](https://github.com/rustcommons/glibc-abilists/blob/main/CHANGELOG.md)
under Unreleased when user-visible work is completed. Separate behavior/data
changes from documentation-only changes and repository-only guides. Keep
planned features in the roadmap, not in a shipped-version entry.

Before an authorized future version is published, commit dated changelog text
and `docs/releases/vX.Y.Z.md` with a version title, concise outcome, and these
four sections: Changes, Upgrade, Validation and limits, Links. State the actual
package identity, source tag/commit, dependency impact and known limits. Link
the detailed qualification evidence rather than substituting logs for release
notes. Run the Python 3.11+ standard-library-only checker locally:

```sh
python3 scripts/check-release-docs.py --repo rustcommons/glibc-abilists
```

The local check has no network access. It verifies current manifest-version
coverage, dated changelog entries, note sections and absolute release-body links.
The [Release documentation workflow](https://github.com/rustcommons/glibc-abilists/blob/main/.github/workflows/release-docs.yml)
runs that check on pushes/PRs. Routine merges require the registered CI check.
For its first installation, GitHub cannot dispatch the workflow until it exists
on the default branch. Independently review local checker and workflow syntax
evidence, then authorize a normal bootstrap merge to register it. A green
post-merge release-documentation workflow run is mandatory before bootstrap is
complete or any GitHub Release notes are published; no candidate CI pass is
claimed from the initial registration 404. For this remediation, use `[skip ci]`
commits to avoid unrelated Rust matrices and explicitly dispatch the docs workflow
after registration.

Its manual `verify_published` input additionally checks all live note bodies and
the latest version; release events check the corresponding body using current
default-branch docs, because old tags predate the backfill. Structure checks do
not establish historical correctness or runtime behavior.

Software release qualification still follows the
[collection guide](https://github.com/rustcommons/glibc-abilists/blob/main/docs/COLLECTING.md#validate-the-change)
and [release plan](https://github.com/rustcommons/glibc-abilists/blob/main/docs/RELEASE-PLAN.md).
A documentation-history repair does not rerun old Rust tests or establish new
runtime/platform qualification. Never regenerate ABI data to manufacture a
documentation release.

## Backfill existing releases without replacing them

For the 2026-11-02 backfill, preserve tags `v0.1.0`, `v0.1.1`, `v0.1.2`, their
commits and immutable crates.io archives. Reconstruct user impact from source
diffs, registry metadata and retained CI/records. Record original publication
dates separately from the new GitHub Release publication time. State missing
historical evidence; do not invent tests or change an old outcome.

The new changelog and versioned notes did not exist at those tags. Links from
release bodies must be absolute and use valid current `main` documentation or
the reviewed documentation commit. Never link to a nonexistent historical
`blob/vOLD/CHANGELOG.md`. The
[history evidence](https://github.com/rustcommons/glibc-abilists/blob/main/docs/RELEASE-HISTORY-20261102.md)
identifies original source/registry dates and bounded checks.

## Publish documentation after approval

After reviewer/coordinator GO and a normal merge, read notes from the reviewed
committed files. Create GitHub Release entries for existing tags in ascending
order, using `gh release create` with `--verify-tag` and `--notes-file`.
Use `--latest=false` for older releases and `--latest` only for the actual newest
stable release. Confirm the target tag first; do not move or recreate it.
The GitHub publication date is the actual backfill date, not the original
crates.io date. If an entry already exists, inspect it and reconcile the approved
documentation rather than creating a duplicate or changing its source identity.

GitHub Releases document the software and may link source tags, crates.io,
docs.rs and maintained guides. Distribution remains the existing crates.io
package; no standalone binaries or other new attached assets are authorized.
This repair needs no new crate version, dependency/lock change or `cargo publish`.

## Completion evidence

A release is complete under this convention when its source tag, applicable
registry archive, live GitHub Release notes, changelog, documentation and required
verification agree. A tag or successful crate upload alone is insufficient.
After publishing notes, verify the live API's tag/name/body, non-draft status,
source commit, latest designation, absence of new assets and all body links.
Run the published check after the authorized backfill:

```sh
python3 scripts/check-release-docs.py --repo rustcommons/glibc-abilists --published
```

It reads GitHub with an optional existing `GH_TOKEN`/`GITHUB_TOKEN`; no new
credentials are required for public read access. Explicitly dispatch the workflow
with `verify_published=true` as well. Record the GitHub URL/publication timestamp
and exact documentation commit in the requirements/release evidence. Keep old artifact/test outcomes identifiable.

The initial 2026-11-02 inventory found no GitHub Release entries. The approved
backfill is now complete: v0.1.0, v0.1.1 and v0.1.2 have exact committed notes,
unchanged source tags and zero assets, with v0.1.2 latest. Bootstrap and full
published-verification workflows passed after merge
`5c180bd6985f8e9ee9fff54f3291ffe8f8a58da0`; see the dated
[completion evidence](RELEASE-HISTORY-20261102.md#completed-backfill-and-verification).
The initial registration 404 and transient unauthenticated API failure remain
historical observations. Existing 0.1.2 artifact records remain unchanged; this
evidence additionally covers GitHub Release documentation completion.
