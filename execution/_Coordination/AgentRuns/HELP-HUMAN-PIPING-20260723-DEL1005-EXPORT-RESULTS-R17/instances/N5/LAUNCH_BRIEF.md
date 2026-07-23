---
doc_id: R17-DEL1005-N5-LAUNCH-BRIEF
doc_kind: coordination.agent_brief
status: dispatched
created: 2026-07-23
run_id: HELP-HUMAN-PIPING-20260723-DEL1005-EXPORT-RESULTS-R17
instance_id: N5
parent_instance_id: W-PKG10
role: fresh_independent_verifier
package_id: PKG-10
deliverable_id: DEL-10-05
frozen_git_basis: 1f2ecc1d06375c01a409041b8380e4d65b2a9f9a
candidate: CB-2026-07-23-DEL-10-05-EXPORT-RESULTS-001-v2
---

# N5 fresh independent verification brief

## Objective

Independently verify the sealed N4 implementation against the exact adopted v2
candidate, exact owner adoption, exact 24-row `WRITE_MATRIX.csv`, frozen Git
basis, implementation diff, witnesses, native evidence, registered evidence,
and sole DEC-025 sweep. Return exactly one terminal verdict:

- `COMMIT-SAFE`, with concise evidence and any non-blocking residuals; or
- `BLOCK`, with prioritized, reproducible findings and the exact required
  remediation or authority amendment.

N5 is a fresh Agent 2 verifier. N5 does not delegate and does not perform W3.

## Authoritative run-local inputs

Read:

- `execution/_Coordination/CANDIDATE_BRIEF_2026-07-23_DEL-10-05_EXPORT_RESULTS.md`
- `execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260723-DEL1005-EXPORT-RESULTS-R17/OWNER_ADOPTION.md`
- `execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260723-DEL1005-EXPORT-RESULTS-R17/WRITE_MATRIX.csv`
- `execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260723-DEL1005-EXPORT-RESULTS-R17/AMENDMENT_V2.md`
- N1, N3, N3B, and N4 briefs/returns/evidence under the managed run root
- `software-workflow.json`
- the complete dirty diff against frozen basis
  `1f2ecc1d06375c01a409041b8380e4d65b2a9f9a`
- the sole R17 sweep artifact
  `validation/evidence/sweeps/SWEEP_20260723T093917Z_1f2ecc1d0637-dirty.json`

Repository authority and live files govern on disagreement.

## Permission boundary

Read the repository and run non-mutating verification commands. Do not edit,
format, stage, commit, reset, checkout, merge, push, create a PR, regenerate
witnesses, rerun DEC-025, or modify any product, test, schema, Cargo, witness,
deliverable, receipt, lifecycle, stage, release, or issuance surface.

Ordinary ignored compiler/test caches are permitted consequences of
read-only verification commands, but no tracked or candidate output may
change. If verification would require a governed mutation, return `BLOCK`.

## Required verification

Independently establish all of the following:

1. Branch and basis are exact; the implementation diff is fully contained by
   the adopted write matrix; W3-only paths are untouched.
2. The shared PKG-08 extraction is behavior-preserving: the core wire adapter
   owns the DTO/conversion, the desktop bridge is a thin linked-identity
   wrapper, and the package assembler/member/schema contract is unchanged.
3. Only the existing `export-results` stub is replaced. The five runner verbs,
   benchmark/regression paths, controlled structured-JSON `--output`
   semantics, and desktop picker/atomic-save ownership remain unchanged.
4. Success returns the exact report-package bytes once plus the bounded
   identity/hash/member projection. Model, manifest, unit-system, load-basis,
   audit, run, and source-basis cross-bindings fail closed with the adopted
   diagnostic vocabulary.
5. Exact `$.report_package` is classified as one aggregate local-private
   subtree. No intent yields one blocking decision/finding, null payload, and
   no file; explicit intent yields one warning decision/finding and exact
   unmodified subtree retention. Cardinality is independent of byte length.
6. Missing, invalid, mismatched, producer-blocked, and no-intent cases expose
   no package or output file. Source inputs are not mutated.
7. Invented witnesses are deterministic and parse; native runner evidence
   proves six-member ZIP/hash integrity at or above 3,189,621 bytes; packaged
   desktop evidence proves cancel/new-save/replace and no temporary residue.
8. Focused tests, the final registered union, claims/path/receipt/JSON/diff
   checks, collateral hashes, and the known unchanged 74/75 desktop Rust
   residual are accurately represented.
9. Exactly one new DEC-025 sweep exists for R17, it passed, and no
   product/test/schema/Cargo/witness edits occurred after its invocation.
10. No unauthorized schema, package-member, runner-verb, decomposition,
    lifecycle, stage, release, issuance, or portable/public persistence claim
    appears.

Inspect implementation quality and security directly; do not accept N4's
summary as proof. Run focused tests only where needed to reproduce a finding or
establish independent confidence. Do not rerun the registered evidence sweep.

## Return contract

Return:

- terminal verdict (`COMMIT-SAFE` or `BLOCK`);
- exact branch/basis and changed-path containment result;
- findings ordered by severity with file/line evidence;
- independent test/static checks run and results;
- sweep count, sweep artifact/hash/status, and post-sweep mutation finding;
- treatment of the unchanged desktop Rust residual;
- confirmation that W3 is either released or remains blocked.

N5's verdict is implementation verification only. It is not merge, lifecycle,
stage, release, issuance, or professional acceptance.
