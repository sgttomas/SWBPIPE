# WORKING_ITEMS Run: D-41 R5 T2C PDU-030

- Date: 2026-07-12
- Deliverable: `DEL-14-04`
- Update: PDU-030
- Claims: `DEL-14-04-REQ-001`; `DEL-14-04-ACC-002`

## Bounded Repair

- Added deterministic automatic mapping production for result records only
  when `result_id` is unique on both sides and result ID, family, object ref,
  basis ref, and dimension all match.
- Produced records use the existing DEL-14-05 `MappingRecord` vocabulary:
  `automatic_match`, `stable_id_alignment`, preserved left/right refs, exact
  stable-ID confidence, pending review metadata, and explicit provenance.
- Preserved explicit caller-supplied `manual_match` as the required path for
  different, duplicate, ambiguous, or semantically inconsistent IDs.
- Added JSON-round-trip tests proving both paths preserve mapping ID and
  left/right result identity into deterministic result deltas.

## Validation

`python3 -m pytest -p no:cacheprovider tests/test_analysis_run_comparison.py tests/test_comparison_contracts.py -q`

Result: `12 passed`.

The automatic mapping instance also validates directly against the existing
DEL-14-05 `MappingRecord` definition. Four-document, minimum-fileset, and
scoped diff validation passed.

## Fences

No heuristic mapping, manual-review workflow policy, comparison-output schema,
normalization threshold, conversion/tolerance default, engineering-validation
outcome, DEL-17-08, dependency, DAG, register, review disposition, lifecycle,
release, professional, protected-content, or private-data surface changed.
PDU-011 and PDU-047 remain held; the D-41 bootstrap remains.
