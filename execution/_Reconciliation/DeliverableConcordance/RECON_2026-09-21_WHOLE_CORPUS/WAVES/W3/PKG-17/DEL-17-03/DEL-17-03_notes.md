# DEL-17-03 notes — W3 PKG-17, worker G1

Forward: `DEL-17-03_forward.csv` (72 rows). It has 64 required keys, the
seven `CLM-011.rNN` rows and one `.s01`. CLM-002, CLM-004, CLM-005 and CLM-012
are assessed directly. Sealed per `DEL-17-03_SEAL.txt`.
Reverse: `DEL-17-03_reverse.csv` (320 capabilities).

## Path aliases

- `D` = `projects/chirality-piping/execution/PKG-17_Export Format Interoperability/1_Working/DEL-17-03_Native open JSON export package/`.
- Foundation artifacts: `core/handoff/native_json/package.py`,
  `schemas/native_json_export.schema.json`,
  `fixtures/native_json/invented/native_json_export_package.json` and
  `tests/test_native_json_export_package.py`.
- CONTEXT's anticipated `exports/native_json` has never existed. The package
  lives at `core/handoff/native_json` (CP-02).

## Judgment calls

- **F7.** No app or runtime module imports `core/handoff/native_json`. The
  desktop `NativePackagePanel.tsx` builds its own review record. Every ALIGNED
  row resting on the builder carries `PRODUCT_CALLER: NONE`. All such claims
  are engine-level: CLM-003 and CLM-009 exclude API, GUI and persistence.
- **AC-001 hash wording (FG-DEL-17-03-01; IMPLEMENTED_DIFFERENTLY ·
  RECORD_DRIFT · PROJECT_BASELINE · OWNER, MEDIUM).**
  - REQ-006 was narrowed under DEC-074 R5 T2A E1/PDU-002 on 2026-07-12. It
    requires the project-local `deterministic_sorted_compact_json_payload_hash`
    label and asserts no RFC 8785 conformance. Code and tests match REQ-006.
  - AC-001 was written by the SOW migration on 2026-07-14 (commit `5b866c5e8`).
    It still says "canonical JSON/JCS-compatible hashing".
  - This is the same owner question as DEL-17-02 REQ-007.
- **Loss-category token (FG-DEL-17-03-03).** REQ-004 is IMPLEMENTED_DIFFERENTLY
  · POSSIBLE_DEFECT, MEDIUM. The native builder and schema use `TBD`, matching
  REQ-004's own wording. The DEL-17-02 contract it cites enumerates `tbd`, as
  do the other four exporters. Either text could be the one to catch up.
- **CLM-006 profile basis.** The native profile declares no canonical ID
  families, member path policy or timestamp policy, so the row is
  PARTIALLY_IMPLEMENTED.
- **Remaining R01 (RF-002).** The text is accurate: RF-002 is OPEN with
  HumanDisposition TBD. No governing row in this ledger carries the action, so
  the row is DOCUMENTED_UNIMPLEMENTED · NOT_STARTED · REVIEW (F2). RF-002's
  premise has aged: it cites DAG-005 flags and the four-document kit.
- **Four-document residue (FG-DEL-17-03-02, CP-01).** This covers CLM-011.r01
  and CLM-015 step 5. The CLM-012 "source reread evidence" column and the
  CLM-017/018 records are past-tense history, so they are ALIGNED (MEDIUM).
- **Gate evidence.** CLM-011.r05, r06 and REQ-007 cite
  `GATE:GATE_EVIDENCE/B4_4_SWEEP_9D55/SUMMARY.json` (Python suite PASS; not
  rerun). CLM-011.r02 cites the 2026-06-16 dependency-refresh run record's PASS
  (not rerun).

## Canonical departures

None.

## Convention friction

- AC-001 was written after the ruling-driven narrowing it contradicts, so
  "stale" does not fit. It is IMPLEMENTED_DIFFERENTLY with cause RECORD_DRIFT.

## UNKNOWN rows

None.

## Reverse pass

- CLAIMED_BY (3):
  - RC-17-0020 (native schema);
  - RC-17-0046 (native fixture);
  - RC-17-0209 (native builder and writer; no product caller).
- NOT_MINE with specific reasons where the ledger cites the path:
  - RC-17-0086 NativePackagePanel: GUI, excluded by CLM-003/CLM-009, and it
    does not call the builder;
  - RC-17-0034 adapter SDK (DEL-17-09);
  - RC-17-0108 registers.
- Everything else is NOT_MINE.
- **Did the reverse pass change my view of anything sealed?** No.

## Batch consistency

`validate_ledger_v2.py --batch` over DEL-17-01/02/03: **PASS, 0 findings.**

## Selectability

`SelectableUnderCurrentLoop` is `NOT_APPLICABLE` on every row (C9). Since
2026-09-19 Piping selects work through owner-steered work graphs.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081). No row
states or implies release, approval, compliance or certification. These
dispositions are agent judgments, not owner rulings.
