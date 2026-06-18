# WORKING_ITEMS RUN - TP-UNITS-BTAIL-COMPTOLCORPUS-001

- **Date:** 2026-06-17
- **Persona:** WORKING_ITEMS
- **Supporting deliverable:** DEL-14-05 Comparison mapping, tolerance, and export contracts
- **Primary deliverable:** DEL-14-04 Analysis-run comparison engine
- **Tranche:** TP-UNITS-BTAIL-COMPTOLCORPUS-001
- **SMOKE:** TP-MAC-194

## Supporting Scope

Support DEL-14-04 by defining optional relative+absolute tolerance-pair
fields in the comparison tolerance schema, aligned with DEC-026, without
defining default numeric tolerances.

## Changes

- Added optional `relative_tolerance_value`, `absolute_tolerance_value`, and
  `tolerance_pair_policy` fields to `schemas/comparison_tolerance.schema.json`.
- Numeric pair values are guarded by externally governed or
  project-specific-review-required status paths.
- Extended `tests/test_comparison_contracts.py` to assert the pair fields,
  nonnegative numeric shape, placeholder vocabulary, and status guard.

## Validation

- `python3 tests/test_comparison_contracts.py` passed.
- Adjacent comparison pytest subset passed 23/23.
- Full Python suite passed 360/360.
- `git diff --check` passed.

## Boundary

Contract support for governed tolerance profiles only. No default tolerance,
release threshold, external validation decision, protected standards content,
private data, lifecycle transition, release-readiness claim, professional
approval, certification, sealing, authentication, or code-compliance claim
changed.

## Handoff

- Primary owner: DEL-14-04 for engine behavior.
- DEL-14-05 role: schema/tolerance contract vocabulary.
- Remaining B-tail items: broader app unit entry/pickers and remaining
  target-format conversion witnesses outside already-covered boundaries.
