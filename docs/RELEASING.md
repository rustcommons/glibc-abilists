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
notes. Run the release-documentation checker and its CI workflow before merge.
The shared checker/workflow is being integrated for this remediation; its
candidate result must be recorded in the review PR before approval.

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
Record the GitHub URL/publication timestamp and exact documentation commit in
the requirements/release evidence. Keep old artifact/test outcomes identifiable.

As of the 2026-11-02 inventory, all three crates are available and non-yanked,
but GitHub Release entries are absent. This PR prepares their backfill; it does
not claim completion before approved publication. Existing 0.1.2 records prove
crate/tag/API-doc delivery, not the new GitHub Release documentation gate.
