---
doc_id: TP-EXPORT-006R-REPAIR-CLOSEOUT
doc_kind: package.audit_repair_closeout
status: complete
created: 2026-05-18
package_id: PKG-17
parent_review: TP-EXPORT-006
commit: da359b93
---

# TP-EXPORT-006R Repair Closeout

## Scope

This repair closed `TP-EXPORT-006-F001`, the malformed `DEL-17-05/Dependencies.csv` row identified during PKG-17 fan-in review.

The repair and validation hardening were committed and pushed as `da359b93 Fix PKG-17 dependency CSV quoting and validation`.

## Files Touched

- `execution/PKG-17_Export Format Interoperability/1_Working/DEL-17-05_CAEPIPE external run harness and CSV parser/Dependencies.csv`
- `tools/validation/validate_dependencies_schema.py`
- `tools/validation/test_validate_dependencies_schema.py`
- `execution/PKG-17_Export Format Interoperability/1_Working/_audit/TP-EXPORT-006_FAN_IN_REVIEW.md`
- `execution/PKG-17_Export Format Interoperability/1_Working/_audit/TP-EXPORT-006_Findings.csv`
- `execution/PKG-17_Export Format Interoperability/1_Working/_audit/TP-EXPORT-006R_REPAIR_CLOSEOUT.md`

## Repair

`DEP-017-05-011` now quotes the comma-containing `TargetLocation` field. `validate_dependencies_schema.py` now rejects data rows whose parsed field count differs from the header width and rejects non-`v3.1` `RegisterSchemaVersion` values.

## Validation

- `python3 -m pytest -q tools/validation/test_validate_dependencies_schema.py` — PASS
- `python3 tools/validation/validate_dependencies_schema.py "execution/PKG-17_Export Format Interoperability/1_Working/DEL-17-05_CAEPIPE external run harness and CSV parser/Dependencies.csv"` — PASS
- Package-wide CSV row-width audit — `bad_rows= 0`
- Full PKG-17 validation sweep — PASS for all `DEL-17-01` through `DEL-17-09`
- DAG-005 blocker queue dry-check — `unblocked=101 blocked=0 active_edges=945 candidate_edges_excluded=11`
- `git diff --check` — PASS

## Closeout

No open package-level blocker remains from `TP-EXPORT-006`. The recommended next implementation planning target remains `DEL-17-03 Native open JSON export package`.

This repair does not change lifecycle state, DAG edges, candidate status, release posture, target compatibility posture, code-compliance posture, or professional-acceptance posture.
