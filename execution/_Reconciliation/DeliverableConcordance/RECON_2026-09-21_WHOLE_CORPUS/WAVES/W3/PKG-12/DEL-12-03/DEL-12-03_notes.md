# DEL-12-03 — W3 notes (worker G1)

Forward SHA-256 in `DEL-12-03_SEAL.txt`. 98 rows: 76 required keys, 16 optional `.rNN` rows (CLM-005, CLM-021 split) and 6 `.sNN` sub-claims. DEL-12-03 was an R0 pilot; encoded afresh under the bound conventions.

## R0 repairs carried (R0_REVIEW §4)

- OUT-001 (matrix) and VER-001: CP-09 citing CHANGE-P3 and manager-validation parity (production hash differs from the frozen SOW).
- TEL-REQ-009: adds `core/adapters/framework/adapter_framework.py` and `plugin_verification.py` evidence.
- CLM-010, CLM-018, CLM-024: under the R0 ruling (DEC-101 does not reach SOWs) rename residue moves to the SOW surface row (CP-04, AuthorityNeeded OWNER); these items are now judged on substance and are ALIGNED.
- UNVERIFIED changed to NOT_APPLICABLE on test-evidence rows.
- Other bound-convention changes: CLM-003 typed DECLARED_STATE (identification with lifecycle field); STATUS surface CP-05; TEL-TEST-004 re-disposed PARTIALLY_IMPLEMENTED under F1 (rule-pack, material and component classes not exercised by a test case); CLM-015/CLM-022 split into four-doc and aligned parts; CLM-008 pin rule; R01/R02 ALIGNED with OPEN_ACTION to TEL-REQ-009 / TEL-REQ-005 (F2); R03 takes the gap disposition.

## Path aliases

- `D` = `projects/chirality-piping/execution/PKG-12_Security, Privacy, and Private Data Handling/1_Working/DEL-12-03_Telemetry off-by-default design/`.
- Seam = `apps/desktop/src/services/telemetryPolicyService.ts`; panel = `apps/desktop/src/features/telemetry/TelemetryBoundaryPanel.tsx`; App evidence = `apps/desktop/src/App.test.tsx::renders the engineering workspace from invented local fixtures`.

## Judgment calls

- DEC-074 O7 is adopted by reference and its text sits in an excluded July-run PROPOSED_* file; not read. CLM-010, CLM-024 and TEL-REQ-009 carry AdoptedByReference=YES.
- CLM-005.r05: code rejects the whole event rather than excluding the field (stricter; IMPLEMENTED_DIFFERENTLY, not a privacy weakening).
- INVARIANT partials: TEL-REQ-009, TEL-REQ-010, TEL-TEST-006 (whole-runtime no-bypass and product-level no-outbound proof absent; no security review record; OWNER).
- TEL-REQ-008: reads "project diagnostic envelope when available" as the Diagnostic definition in schemas/model.schema.yaml.
- Coverage telemetry tooling (RC-12-0199) is developer test-coverage measurement, not product telemetry.

## Canonical departures

- CLM-020 (INIT.md pointer): CP-02 with the F3 setup-origin class (CANONICAL_DEPARTURE written).

## UNKNOWN rows

None.

## Reverse pass

Claimed: 0060 (Python guard), 0152 (desktop seam), 0187 (policy), 0321 (panel) — the routing file now has capabilities for the seam and panel that the R0 pilot inventory lacked. Adapter framework capabilities (0183, 0092, 0272) answered COVERS rather than R0's PARTIAL: they are DEL-10-02-owned declaration controls, and COVERS exists in the bound B2 vocabulary for this relation. Reading the R0 reverse file after sealing changed nothing in the sealed forward rows.

## Batch consistency

`validate_ledger_v2.py --batch` over the three PKG-12 G1 ledgers: PASS, 0 findings.

## Selectability

`SelectableUnderCurrentLoop` is `NOT_APPLICABLE` on every row (C9): since 2026-09-19 Piping selects work through owner-steered work graphs, not `## Remaining`.

## Process disclosure

Before the manager told me the write boundary for scratch, I kept builder scripts and two SOW text dumps in the session scratchpad (shared). One of my files there (`_scratch_lib.py`) was overwritten by another worker's same-named file; I left their version untouched and moved my own scratch into my deliverable folders, then deleted it. No file I do not own was modified by me. Forward ledgers were built by script from my own judgments; the R0 pilot rows were read (DEL-12-03 only) but not copied.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081). These dispositions are agent judgments, not owner rulings.
