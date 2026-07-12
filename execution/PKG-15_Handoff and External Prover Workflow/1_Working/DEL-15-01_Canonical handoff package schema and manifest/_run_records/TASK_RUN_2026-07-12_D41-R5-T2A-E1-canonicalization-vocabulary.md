---
run-id: TASK_RUN_2026-07-12_D41-R5-T2A-E1-canonicalization-vocabulary
run-status: SUCCESS
deliverable-id: DEL-15-01
package-id: PKG-15
agent: TASK
parent-agent: ORCHESTRATOR
tranche: D41-R5-T2A
date: 2026-07-12
lifecycle-changes: not_authorized
producer-output-changes: none
---
# TASK Run Record — D-41 R5 T2A E1 canonicalization vocabulary

## Objective

Add a precise non-JCS schema value for the existing Python sorted-key compact
JSON payload-hash basis while preserving the current JCS-compatible value for
backward compatibility. This is the DEL-15-01 prerequisite for D-41
`DEC-074` E1 / PDU-002; producer repairs remain separate tasks.

## Consumer audit

Read-only inspection covered the handoff package schema and fixture, focused
schema/workflow tests, and current Python handoff producers. Existing producers
and fixtures emit `JCS_compatible_json_payload_hash`; none was edited here.
Their current stable sorted-key compact serialization is not treated as proof
of RFC 8785 conformance.

## Changes

- `schemas/handoff_package.schema.json`: added
  `deterministic_sorted_compact_json_payload_hash` to `Checksum`
  canonicalization values with an explicit sorted-key/compact-separator and
  non-RFC-8785 description; retained all existing enum values.
- `tests/test_handoff_package_schema.py`: added focused validation for the new
  label, backward compatibility for `JCS_compatible_json_payload_hash`, and
  the explicit non-JCS description boundary.
- Updated DEL-15-01 context, specification, datasheet, guidance, procedure,
  status, and memory to define the E1 boundary.

## Boundaries

- No producer, fixture, downstream schema, runtime output, hash value, or
  serialization behavior changed.
- No RFC 8785/JCS conformance is claimed for the new label.
- No lifecycle, dependency, DAG, register, target mapping, package-container,
  external-prover, release, professional, certification, sealing,
  authentication, or code-compliance state changed.
- Both pre-run Remaining bullets are preserved verbatim, including the exact
  D-41 program item.

## Validation

- `python3 tests/test_handoff_package_schema.py`
- `python3 -m pytest -q -p no:cacheprovider tests/test_handoff_package_schema.py`
- JSON Schema Draft 2020-12 validation and existing invented fixture
  validation are included in the focused checks.
- Scoped diff/whitespace and write-containment checks.
