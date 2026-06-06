---
run-id: TASK_RUN_2026-06-05_1958_FORCE-PER-LENGTH-WORKER-B
run-status: SUCCESS
deliverable-id: DEL-04-02
package-id: PKG-04
agent: TASK
worker: B
tranche: TP-DEL-04-01-04-02-05-03-FORCE-PER-LENGTH-BOUNDARY-001
date: 2026-06-05
lifecycle-changes: not_authorized
review-disposition-edits: not_authorized
dependency-edits: not_authorized
dag-edits: not_authorized
---

# TASK Run Record - Worker B Straight-Pipe Force-Per-Length Metadata

## Objective

Replace the stale straight-pipe `TBD` force-per-length metadata boundary with
the accepted shared `force_per_length` canonical dimension.

## Files Updated

- `core/solver/straight_pipe/src/lib.rs`
- `core/solver/straight_pipe/README.md`
- `MEMORY.md`
- This run record.

## Outputs Produced

- Updated `StraightPipeBoundaryMetadata::has_expected_dimensions()` so
  `weight_force_per_length_unit` must be
  `CanonicalDimension::ForcePerLength`.
- Updated the positive metadata test to assert `force_per_length`.
- Added a regression test proving `CanonicalDimension::Tbd` is no longer
  accepted for the force-per-length metadata field.
- Updated README boundary wording to identify explicit `force_per_length`
  unit metadata.

## Validation

- Passed:
  `cargo fmt --manifest-path core/solver/straight_pipe/Cargo.toml --check`
- Passed:
  `cargo test --manifest-path core/solver/straight_pipe/Cargo.toml --locked`
  with 33 tests.

## Boundaries Preserved

- No lifecycle transition, `_STATUS.md` edit, review disposition edit,
  dependency edit, DAG edit, schema edit, repo governance edit, release claim,
  code-compliance claim, professional approval claim, protected standards
  content, private data, conversion constant, load default, or tolerance policy.
