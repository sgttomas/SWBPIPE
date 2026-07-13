# WORKING_ITEMS Run: D-41 R5 T2A PDU-003

- Date: 2026-07-12
- Deliverable: `DEL-14-02`
- Authority: `DEC-074` E1
- Claims: `DEL-14-02-REQ-007`; `DEL-14-02-ACC-004`

## Evidence and Finding

The accepted D-41 ledger and PDU-003 identify a fidelity gap: DEL-14-02 used
Python `json.dumps(sort_keys=True, separators=(",", ":"), ensure_ascii=True)`
but emitted `JCS` and `canonical_json_jcs_payload`. Determinism and mutation
tests did not demonstrate RFC 8785/JCS conformance.

## Applied Repair

- Preserved the existing serializer and SHA-256 hash inputs.
- Changed emitted checksum metadata to `SORTED_COMPACT_JSON`.
- Changed DEL-14-02 physical-container truth metadata to
  `sorted_compact_json_payload`.
- Documented the compatibility function name `canonical_json` as sorted-key,
  compact, ASCII-escaped JSON and explicitly not an RFC 8785/JCS
  implementation.
- Retained legacy `JCS` checksum and `canonical_json_jcs_payload` truth labels
  in schema enums only for backward compatibility; DEL-14-02 no longer emits
  them.
- Aligned the four-document kit, status, and memory.

## Validation

`python3 -m pytest -p no:cacheprovider tests/test_analysis_run_schema.py tests/test_analysis_run_records.py -q`

Result: `10 passed` (final rerun).

Focused evidence covers the exact emitted label, exact deterministic bytes and
SHA-256, equivalent-map stability, result mutation sensitivity, schema
alignment, and absence of a JCS claim in generated analysis-run envelopes.

## Fan-in Closure and Fences

Fan-in confirmed that the PDU-003 DEL-02-02 declaration-currentness repair is
present in the same diff, so the temporary DEL-14-02 cross-owner status
residual is closed. This owning task did not change DEL-02-02, the
`model_state` schema, DEL-14-01, dependencies, DAG, registers, lifecycle,
professional/release/compliance posture, protected content, or private data.
The D-41 program bootstrap and lifecycle `IN_PROGRESS` remain.
