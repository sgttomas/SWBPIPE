# Sealed Brief — A2-09 Mechanics Observation API Author Remediation

RequestedBy: `N1-WORKING-ITEMS-PKG09-DEL0901`

RunID: `HELP-HUMAN-PIPING-20260811-DEC025-MECHANICS-RUNNER-BINDING`

ParentInstanceID: `N1-WORKING-ITEMS-PKG09-DEL0901`

ChildInstanceID: `A2-09-AUTHOR-REMEDIATION`

Agent form: fresh non-delegating ephemeral Agent 2

PackageID / DeliverableID: `PKG-09` / `DEL-09-01`

Attempt: 2, bounded fresh-author remediation after attempt 1 was interrupted
for non-execution without any source write.

## Objective and exact write fence

Implement now, in exactly
`projects/chirality-piping/validation/benchmarks/mechanics/src/lib.rs`, one
additive suite-owned observation API and in-file tests covering all 14 current
formerly blocked cases and exactly 115 names/values. You may also write only
this instance's `RETURN.md` and `STATUS.json`. Do not delegate.

## Frozen basis

Resolve paths from the active checkout. Branch/HEAD is
`codex/piping-dec025-case-runner-binding-20260811` at
`f1e311fb7ab1c2a0800b1d32c59445368428dee9`. Adopted candidate SHA-256 is
`c8d2b63d6a1649102e2b233542975cc8e8062e582914900a2ec75c5acc961626`.
Baseline `lib.rs` SHA-256 is
`8f015dfeeb1fd670065f467335419ab0da8b0c9a4e6d6b3fb565ceea2d6f6a26`.
Frozen current runner output SHA-256 is
`e41f8545cf6fc0603cb41a0bdc08d90726f59ce57e13ad6809036589db3ded8f`.
C-B SHA-256 is
`1829a2a6a608dbbdd479248133f2c2a05406c0eabe7a03be57b998735fbe78b6`.
Read the adopted candidate and the complete attempt-1 brief for the full
declared reads, exclusions, failure semantics, and acceptance contract; those
terms are incorporated unchanged.

## Required implementation result

- Deterministic named observations from existing public-original solver and
  fixture logic, never from `expected_values`.
- Exactly 14/14 blocked cases and 115/115 expected observation names, finite,
  with no missing/extra/duplicate names.
- Preserve inventory, expected values, predicate/tolerance, provenance, C-B,
  all existing behavior/tests, and all non-writable bytes.
- Externalize Cargo target/cache/temp outputs to `/private/tmp`; no network or
  provisioning. Run mechanics fmt-check and full crate tests.
- Return exact API signature/semantics, source after-hash/blob, coverage,
  tests, containment, side effects, blockers, and rerun triggers.

Do not edit fixtures/pages/Cargo manifests/policies/state/evidence/receipt/
PKG-10 or any other path. Do not stage, commit, push, PR, merge, fetch, rebase,
reset, clean, delete, or expand scope. HOLD rather than widen scope.

