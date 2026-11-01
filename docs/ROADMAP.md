# Roadmap

Open product work is listed below. The completed documentation cleanup is
recorded in the [release plan](RELEASE-PLAN.md#documentation-cleanup-completed-on-2026-10-31);
release status remains proposed.

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
  local-clone trust requirement. The current behavior and CI mitigation are
  explained in [Provenance](PROVENANCE.md#offline-collection-and-trust).

This is collector hardening/documentation follow-up, not a confirmed library
runtime defect or a finding that the shipped data is compromised. No exception,
collection, or publication policy is changed by this item.
