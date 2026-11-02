# ABI-data release: downstream unwind-impact checklist

Status: draft policy adoption, 2026-11-02. No dataset/software release or runtime
qualification is performed here. The approved direction for cargo-libc 0.4.0 is
static LLVM unwinding for supported standalone executables, retaining dynamic
glibc and `panic=unwind`. Shared libraries/Python keep their existing behavior.
The implementation's final test driver and immutable refs have not been supplied
to this proposal; no 0.4.0 test result is claimed.

## Scope and authority

This repository supplies ABI-list text and recorded provenance, not the LLVM
runtime or a runtime compatibility certificate. A changed dataset can still
affect the symbols available when downstream components link that runtime.

The canonical consumer checklist is reserved as
`rustcommons/cargo-libc: docs/UNWINDER-RELEASE-CHECKLIST.md` (not yet on main).
The producer checklist and exact test-driver references follow the implementation
design. These are pending integration references, not live links or invented
commands. The parent will supply their final paths/commits before merge.
This checklist records ABI-input impact and links downstream qualification;
it does not replace either downstream acceptance contract.

## Trigger and blocking rule

- [ ] Classify the candidate: changed `data/`/generated ABI payload, newly
  collected glibc release, or documentation-only with identical payload.
- [ ] For **any** changed ABI-list dataset/new glibc release, complete the impact
  assessment and relevant fresh producer/Cargo qualification below. Unchanged
  runtime source, a data-only label or a symbol comparison alone is no exemption.
- [ ] Tie all evidence to the exact candidate inputs, selected LLVM runtime and
  downstream refs. A previous green run against other inputs cannot silently
  qualify this candidate.
- [ ] Block release on missing refs/hashes, unresolved driver commands, missing
  required floor/new-release results, unexplained symbol/version changes, failed
  relevant regressions or runtime-closure differences from reviewed expectations.
  Resolve the issue and rerun affected evidence; do not mark a missing run green.

No actual data release is authorized by this policy proposal. Merge readiness
also requires the pending canonical checklist/test-driver links and command
mapping to be finished; release readiness requires actual qualifying results.

## Required assessment record

Retain one dated, reviewable record with these fields. Use immutable source refs
and content hashes alongside human-readable versions. Identify any not-run case
and its scope explicitly; a required case remains a blocker.

| Field | Required evidence |
| --- | --- |
| Candidate and decision | Assessment ID/date, reviewer, proposed ABI package version, base/candidate commits, changed glibc releases/libraries/paths and release-blocking decision. |
| ABI inputs | Base/candidate package or payload hashes, generated-table hash, exact glibc tag objects/commits, selection/config refs and file/hash delta. Link the existing online collector validation evidence for those same inputs. |
| Downstream identities | Exact cargo-libc and linux-abi-tools producer source/package refs, relevant lock/resolution hashes, target, compiler/linker/toolchain versions, sysroot/provider identities and build configuration. |
| LLVM runtime | Actual selected LLVM source/version/ref, archive/object names and hashes, build configuration and which selected objects enter the linked artifact. Do not infer selection from a dependency name alone. |
| Symbol/version map | For libc symbols used by selected LLVM runtime objects, record supplied ABI availability/version(s), linked-artifact bindings and supplying component; compare base/candidate and the floor/new-release rows below. Distinguish available definitions from actual imports/bindings. |
| Fresh tests and artifacts | Exact implementation-provided commands/job refs, input refs, environment/image hashes, run dates, exit/results, logs and produced artifact hashes for every applicable producer/Cargo/shared/PyO3 case. |
| Runtime closure | Actual observed loaded dependencies and their refs/hashes, transitive/dynamically loaded components in tested scenarios, unwinder provider/selection evidence and differences from the baseline. Record observation limits separately from link inspection. |
| Reuse or gaps | For scoped reuse, identify prior evidence, all relevant identical inputs, covered environments/cases and limits. For missing/failed evidence, state why it blocks the required claim and what remains to run. |

## Floor and new-release comparison

Do not turn the dataset's earliest version into a new consumer support claim.
The 2.17 row is the existing ABI-data baseline; the approved consumer floor is
2.22. Each candidate record must include both comparisons and the new-release
case when adding a release.

| Input case | Required comparison/linkage evidence |
| --- | --- |
| glibc 2.17 data baseline | Map LLVM-runtime libc symbol/version requirements against the earliest supplied ABI and the prior dataset. Identify availability/binding differences; do not imply that every downstream configuration is supported on 2.17. |
| glibc 2.22 consumer floor | Compare old/new supplied ABI and actual linked-artifact symbol versions against the consumer floor; retain fresh relevant link/run outcomes from the supported consumer driver. A raised requirement cannot silently pass as unchanged compatibility. |
| Newly collected glibc release | Map added/removed/changed supplied symbols and versions; retain fresh producer/consumer linkage results using that exact new-release input and applicable runtime outcomes. New-release success does not replace floor coverage. |

A no-impact conclusion must cite the actual symbol map and fresh relevant
results. Neither the highest printed symbol version alone nor static inspection
certifies runtime compatibility or distribution-patched glibc behavior.

## Downstream CI and runtime checklist

- [ ] Producer: exercise the implementation's ABI/provider-to-runtime preparation
  and link path against the candidate inputs; record selected runtime artifacts,
  symbol/version bindings, outputs and hashes. Capture relevant fresh tests.
- [ ] Cargo supported standalone executables: record the approved static LLVM
  unwinder selection with dynamic glibc and `panic=unwind`; run the final driver's
  supported unwind/panic/cleanup regression cases at the floor/new-release inputs.
- [ ] Shared libraries and PyO3/Python: retain fresh applicable producer/Cargo
  regression results showing their existing linkage/unwind behavior is preserved.
  Do not apply the standalone-only policy to these outputs by assumption.
- [ ] Runtime closure: test it separately from archive selection and direct
  dependency inspection. Observe the relevant transitive/dynamic-load scenarios
  and identify the supplying unwinder/components. Static LLVM selection is not
  a universal promise that libgcc can never be loaded by another component.
- [ ] Link every run to exact refs, inputs and artifacts in the assessment record;
  resolve unsupported/not-run scopes rather than treating test counts as broad
  platform qualification.

The command/job mapping is deliberately **pending**, not pseudocode executable
instructions: parent-supplied final producer driver, Cargo driver, shared/PyO3
cases and closure checks must be linked at their actual immutable implementation
refs before merge. No command, option or API is invented here. A release-doc CI
PASS checks documentation structure only and cannot satisfy these runtime gates.

## Documentation-only scoped reuse and existing limits

For a documentation-only candidate with identical relevant payloads, record the
hash comparison and cite prior qualification only for its named inputs/cases.
State whether runtime/toolchain/producer/consumer inputs also match and why the
recorded evidence still applies. ABI-byte identity alone cannot qualify a newly
changed LLVM runtime, Cargo 0.4.0 implementation or build configuration.

Keep existing license/provenance gates, signature-exception/offline trust limits
and Tier 3 exclusions. This proposal changes neither collection enforcement nor
legal/support policy and does not claim offline regeneration authenticates an
exception tag. See [collection validation](COLLECTING.md#validate-the-change),
[provenance limits](PROVENANCE.md#offline-collection-and-trust) and
[release policy](RELEASING.md#abi-data-changes-downstream-unwind-impact-gate).

Traceability: existing [GA-REQ-001, GA-REQ-005, GA-REQ-006 and GA-REQ-011](REQUIREMENTS.md#requirements)
cover dataset/provenance/package boundaries; the new gate is GA-REQ-017 in the
same index. No changed dataset, runtime or downstream test has been qualified by
this draft.
