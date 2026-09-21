# DEL-03-01 notes — Material library schema with provenance

Worker: W2 PKG-03 G1 (TASK). Frozen state `00115c71931bcae79909602d653740d3bb72dfa1`.
Forward ledger 95 rows (65 required keys, 25 optional `.rNN` for CLM-006,
CLM-012, CLM-020 and CLM-027, and 5 `.sNN` sub-claims); sealed before the
routing file was read.

## Path aliases

- `DEL0301/` = `projects/chirality-piping/execution/PKG-03_Piping Components, Materials, and Library Data Model/1_Working/DEL-03-01_Material library schema with provenance/`.
  The folder name has spaces, so its files cannot go in evidence columns
  (Part D). `DEL0301/Review_Findings.csv`, `MEMORY.md`, `_STATUS.md`,
  `_CONTEXT.md` and the PDU-024 run record are cited in `ContextRefs` in full.
- Parity records are root-level: `execution/_Coordination/AgentRuns/SOW-STAGE2-EXEC-20260712-01/instances/WORKING-P1-PKG03/manager-validation/DEL-03-01/`.
- `schemas/…`, `fixtures/…`, `tests/…` are project-root tokens.

## Judgment calls

- **Interpolation policy drift (FG-DEL-03-01-01).** DEC-077 (D-38) and DEC-092
  (D-45) ruled bounded E/alpha/G interpolation. The schema description carries
  it, `temperature_interpolation_policy` was removed from the `OpenDecision`
  topics, and the test asserts both (commits `e2ed4d347`, `c394365ca`). SOW text
  first declared 2026-06-04 (`1b62eb5b8`) still lists the policy as TBD:
  CLM-005(.s01), CLM-006.r10, CLM-013, CLM-019(.s02), CLM-025. Cause
  SCOPE_REDIRECTED_BY_RULING, LOCAL_DESIGN: a catch-up needs no decision.
- **Fixture drift (FG-DEL-03-01-04).** Commit `c394365ca` added an invented
  temperature-point G value (123 MPa, `public_permissive`, licence "project
  invented public example"). The no-engineering-values boundary holds (it is
  labelled invented), but CLM-005.s02, CLM-006.r06, CLM-026, CLM-027.r01 and
  CLM-028 still describe the fixture as fully omitted with TBD licence.
  DOC_BEHIND_CODE, MEDIUM confidence.
- **Review state (FG-DEL-03-01-02).** Both PKG-02 findings are `ACCEPT_AS_IS` /
  `RESOLVED` (human Gate A, `a7865b528`). CLM-014, CLM-020.r08 and CLM-021
  still frame them as pending: RECORD_DRIFT.
- **Round-trip (FG-DEL-03-01-03).** REQ-03-01-007's own verification says the
  material serialization round-trip harness is TBD, and no such harness or
  deferring ruling was located (the "separately recorded deferral" in CLM-010
  was not found as a ruling). PARTIALLY_IMPLEMENTED · PARTIAL_SLICE ·
  PROJECT_BASELINE (AB-00-04). VER-001, which asks for deterministic persistence
  compatibility, takes the same. CLM-010 itself stays ALIGNED: it declares the
  round-trip as an exclusion.
- **CLM-004 / CLM-020.r07 privacy.** Judged at the data-object level. Private
  material library handling is also exercised by
  `tests/security/test_secret_private_library_handling.py` (owned elsewhere).
- **CLM-003 evidence-status cell** ("pending review and dependency
  disposition") is ALIGNED: protected-content review and 2 of 13 dependency
  rows are still open.
- **F7.** No row relies on a product caller: all claims are about the schema,
  fixture and test. The schema is consumed by the library-import seam and the
  Library Manager.

## Canonical departures

None. The CS rows inherit their fields. CP-02, CP-04 and CP-09 were applied as
written.

## Convention friction

- C1 says `.sNN` "whenever parts of a block would take different
  dispositions". I used `.sNN` only where a block has two diverging parts with
  different causes (CLM-005, CLM-019) and the architecture-basis `.s01` that
  CS-04 prescribes. Elsewhere the block row carries its single gap and Notes
  name the part. With F1, making the aligned remainder a separate row would
  not change any census. Blocks with issued `.rNN` keys were split in full
  where rows differ (CLM-006, CLM-012, CLM-020, CLM-027).
- F3 against CS-05 for the PKG-00 SEMANTIC_READY sentence: origin is the
  initial migration, but I treated it as a lifecycle-state declaration (the F3
  review-state/metadata exception) so it matches CS-05.
- Deliverable-folder paths with spaces cannot be evidence tokens, so
  review-state evidence sits in `ContextRefs`.

## UNKNOWN rows

None.

## Reverse pass

The routing file has 376 capabilities. Two are CLAIMED_BY: RC-03-0079
(material schema) and RC-03-0162 (material fixture). The rest are NOT_MINE.
F5 overlaps (units schema, CONTRACT, registers) carry specific reasons. The
model-document material temperature table features (RC-03-0015, 0221, 0321)
are NOT_MINE because they act on the canonical model material, not the library
schema. The reverse pass did not change my view of any sealed row. It
confirmed that the DEC-068/077/092 semantics live in the schema file this
deliverable owns, so FG-DEL-03-01-01 is a documentation catch-up on its own
artifact.

## Batch consistency

`validate_ledger_v2.py --batch` over the three G1 forward ledgers: PASS, 0
findings. By design, REQ-03-01-007 (PARTIALLY_IMPLEMENTED) differs from
DEL-03-02 REQ-09 (ALIGNED). The DEL-03-01 requirement records the round-trip
as its own open element; the DEL-03-02 requirement excludes it from its claim.
I judge the difference justified.

## Selectability

`SelectableUnderCurrentLoop` is NOT_APPLICABLE on every row (C9). Since
2026-09-19, Piping selects work through owner-steered work graphs.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
