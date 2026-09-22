# DEL-12-02 — W3 notes (worker G1)

Forward SHA-256 in `DEL-12-02_SEAL.txt`. 147 rows: 101 required keys, 38 optional `.rNN` rows (CLM-003, CLM-005, CLM-022, CLM-035 split) and 8 `.sNN` sub-claims.

## Path aliases

- `D` = `projects/chirality-piping/execution/PKG-12_Security, Privacy, and Private Data Handling/1_Working/DEL-12-02_Private data redaction and export controls/`.
- "R15" = `D/_run_records/WORKING_ITEMS_RUN_2026-07-22_DEL-12-02_REDACTION-BREADTH-R15.md`; "TP-E4" = `D/_run_records/WORKING_ITEMS_RUN_2026-07-10_TP-E4-REDACTION-001.md`.
- Route control = `core/security/redaction/route_control.py` and its TypeScript mirror `apps/desktop/src/features/redaction-controls/redactionExportControls.ts`.

## Judgment calls

- **Runtime integration landed; SOW still says TBD (FG-DEL-12-02-01, 21 rows DOC_BEHIND_CODE).** R15 bound the 31-route inventory, TP-E4 added the desktop panel with context selection and explicit intent, 2026-08-21 added local-first evidence. Declarations saying runtime integration, UI controls, linter or export tests are TBD are stale.
- **Requirement rows judged on subject (C6(b))**: REXC-REQ-001..009, 013, 015 ALIGNED with the stale verification-column status noted. Partial: REQ-010 (privacy/redistribution metadata not carried from model/library records; INVARIANT), REQ-011 (findings lack source and provenance fields), REQ-012 and CLM-004 (plugin clause holds only by absence, CP-11; INVARIANT, OWNER), REQ-014 (no plugin-route tests).
- **Protected-content linter** exists in report-domain code and the report renderer calls it; treated as satisfying the template guard, with ownership elsewhere.
- **CP-10 rows**: REXC-OI-010 (storage root, same finding as DEL-12-01 FG-01) and REXC-CON-002 (conflict proposal implemented while the human-ruling column is TBD; R15's owner-adopted route matrix is context only, A3). **Owner items for R4.**
- **Empty Remaining** (pre-typed NON_NORMATIVE) left NOT_ASSESSED. Observation for R3: Review_Findings.csv RF-001/RF-002 are OPEN with HumanDisposition TBD and are not homed in Remaining, unlike DEL-12-01 and DEL-12-03.

## Canonical departures

- CLM-008 and CLM-019 (INIT.md pointer): CP-02 with the F3 setup-origin class (CANONICAL_DEPARTURE written). No CS departures.

## Convention friction

- Stale status clauses inside requirement rows (the "Verification" column) have no clean home: C6(b) sends them to Notes, so the drift is visible only on declared-state rows. R3 may want to count them.

## UNKNOWN rows

None.

## Reverse pass

Claimed: 0080, 0192, 0130, 0305, 0178, 0011, 0375, 0100. Partial: 0265 (shared with DEL-12-01's local-first admission), 0116, 0052. Covers: route-controlled writers and saves (0062, 0071, 0349, 0135, 0334, 0332, 0153, 0181), linter (0283, 0005), export review (0267). No sealed row changes; the reverse pass confirmed the R15 breadth that the DOC_BEHIND_CODE rows rely on.

## Batch consistency

`validate_ledger_v2.py --batch` over the three PKG-12 G1 ledgers: PASS, 0 findings.

## Selectability

`SelectableUnderCurrentLoop` is `NOT_APPLICABLE` on every row (C9): since 2026-09-19 Piping selects work through owner-steered work graphs, not `## Remaining`.

## Process disclosure

Before the manager told me the write boundary for scratch, I kept builder scripts and two SOW text dumps in the session scratchpad (shared). One of my files there (`_scratch_lib.py`) was overwritten by another worker's same-named file; I left their version untouched and moved my own scratch into my deliverable folders, then deleted it. No file I do not own was modified by me. Forward ledgers were built by script from my own judgments; the R0 pilot rows were read (DEL-12-03 only) but not copied.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081). These dispositions are agent judgments, not owner rulings.
