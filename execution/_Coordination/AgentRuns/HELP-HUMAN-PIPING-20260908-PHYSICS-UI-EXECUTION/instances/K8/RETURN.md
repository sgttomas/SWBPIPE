# K8 WORKING_ITEMS return

## Objective and basis

Completed the Owner-selected PKG08/DEL-08-04 route-A canonical result schema and consumer migration design against source `779dedb8670625b36af07b89fc5557470e47c50e`, W10 V1 plus controlling V2, and the accepted 2026-09-08 run controls. W10 V3/V4 were consumed only as representation relocations; original evidence hashes were not reinterpreted.

## Result

- Complete Draft 2020-12 `0.2.0` schema candidate generated and validated.
- Exact native preservation uses required `source_record`; separate closed `classification` prevents source-specific strings from masquerading as newly normalized mechanics semantics.
- 45 observed native `(kind, unit)` pairs have one explicit transport mapping.
- W10 exact coverage is bound: 2,348 current canonical rows, 2,010 valid identity rows, 338 invalid metadata rows, 518 invalid metadata occurrences, and two legacy canonicalization-label errors per document.
- Exact omissions are bound by ID: linear 9 of 786 native rows, nonlinear 33 of 830, zero pressure 39 of 813.
- Full 830-row nonlinear candidate migration validates with every native source record, value, unit and metadata preserved; referenced checksum algorithm/payload-ref/value triples remain unchanged.
- Pressure rows preserve existing native meaning and P5-pending status; no new wall/effective-force or closure-transfer meaning was assigned.
- Current consumers, ownership, sequencing, backward-compatibility options, version negotiation, unknown-version handling and executable acceptance checks are specified.

## Exact writes

- `{RUN_ROOT}/instances/K8/**`
- `{WORKING_ROOT}/execution/PKG-08_Reporting, Audit, and Reproducibility/1_Working/DEL-08-04_Result export format/_run_records/PHYSICS_UI_EXECUTION_20260908/**`

No other path was authored by K8.

## Validation

- `uv run --with jsonschema python3 {RUN_ROOT}/instances/K8/evidence/build_candidate.py`: PASS
- Draft 2020-12 schema meta-validation: PASS
- full native nonlinear transformation and schema validation: PASS, 830/830
- unknown schema version rejection: PASS
- unknown native kind/unit rejection: PASS
- unique result ID check: PASS
- LF and portable-control scan: PASS
- write containment: PASS

## Decision and closure

Design closure: `COMPLETE`. Public adoption/implementation closure: `BLOCKED_BY_RETAINED_OWNER_GATE`, as intended by the launch.

Exact audit-D05 response requested: choose one compatibility bundle (`K-A`, `K-B`, `K-C`) and one unknown-version behavior (`K-U1`, `K-U2`), or amend. Recommendation: `K-A + K-U1` — pre-1.0 `0.2.0`, one current writer, bounded dual-read migration, structured refusal of unknown versions with native payload retained.

Next owner: `/root` HELP_HUMAN for fan-in and Owner presentation. No rerun is needed unless a named source hash, public choice, mechanics meaning, or bridge mapping changes.
