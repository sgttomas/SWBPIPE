# DEL-11-02 notes (W3, PKG-11, worker G1)

Forward ledger 137 rows (76 required keys; `.rNN` splits of CLM-003/004/007/012/018/020/033; `.sNN` for SOW frontmatter, CLM-010, CLM-014, CONTEXT ABI).

## Path aliases

- "SOW", "_STATUS", "_CONTEXT", "MEMORY" mean the files in this deliverable's folder under `projects/chirality-piping/execution/PKG-11_Documentation, Examples, and Education/1_Working/`.
- Parity records are root `execution/_Coordination/AgentRuns/SOW-STAGE2-EXEC-20260712-01/instances/CHANGE-P3/checks/<DEL>/` (latest PASS record; EVIDENCE_MAP).
- `tools/validation/validate_dependencies_schema.py` is cited as the project copy (`projects/chirality-piping/tools/...`), the one the deliverable run records invoke; `check_four_documents.sh`, `validate_enum.py`, `validate_claims_language.py` are root tools.
- `GATE:` tokens are relative to the run folder (A6).
- "the guide" = `projects/chirality-piping/docs/developer_guide/index.md`.

## Judgment calls

- Most SOW text describes a "future" guide; the guide exists since 2026-05-03 (Tranche A). Exclusion/scope rows (CLM-010.s01, REQ-11-02-014, CLM-014.s01, CLM-020.r02) are STALE_SETUP_SPECIFICATION · DOC_BEHIND_CODE, `AuthorityNeeded NO`: no owner record grants an exception here (contrast DEL-11-01 A3a).
- REQ items judged on the guide as it stands; all ALIGNED except REQ-014.
- STATUS R01 (select contributor legal mechanism) is CP-07 REMAINING_STATE_MISMATCH · RULED_CRITERION: DEC-079 (2026-07-15) defers the instrument to intake activation, after the item was homed on 2026-07-12.
- CLM-033 open-decisions rows r01/r02/r04/r05/r06 are TBD_RULED (DEC-023, DEC-022/037, DEC-025/059, DEC-028/SCA-003, DEC-027/079 plus licence 2026-06-03); r03 (dependency versions) ALIGNED.
- AC-001 PARTIALLY_IMPLEMENTED · DOC_BEHIND_CODE: the contract lacks current implementation evidence; the guide itself still pins revision 0.7 / DAG-007 (L25-26) and keeps the sparse solve policy TBD after DEC-023 (L171).
- CLM-013 and CLM-020.r03 review rows: latest reviews (2026-05-16 audit, 2026-06-07 readiness) predate the 2026-07-12 and 2026-09-18 edits → CP-09-style EVIDENCE_OVERTAKEN, LOCAL_DESIGN (F8).
- CLM-034 conflict table STALE at LOW confidence (whether currency drift counts as a "source conflict" is a reading).
- The guide lists `schemas/caepipe_*` files; DEC-103 bars the vendor name from the product and user guide only, so not recorded as a finding.

## Canonical departures

CLM-007.r01, CLM-018.r05: CP-02 pointer with F3 origin → STALE_SETUP_SPECIFICATION, `CANONICAL_DEPARTURE`.

## UNKNOWN rows

None.

## Reverse pass

RC-11-0107 (developer guide) CLAIMED_BY; all others NOT_MINE with specific reasons where entry points are cited here. No change of view on sealed rows.

## Selectability

`SelectableUnderCurrentLoop` is `NOT_APPLICABLE` on every row (C9): since 2026-09-19 Piping selects work through owner-steered work graphs, not `## Remaining`.

## Batch consistency

`validate_ledger_v2.py --batch` over the three PKG-11 G1 forward ledgers: PASS, 0 findings. Cross-package shared bodies (mostly with DEL-09-04, DEL-11-05) are outside this batch and are left to the verifier.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081). These are agent dispositions, not owner rulings.
