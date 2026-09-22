# DEL-11-03 notes (W3, PKG-11, worker G1)

Forward ledger 139 rows (69 required keys; `.rNN` splits of CLM-004/006/014/026; `.sNN` for SOW frontmatter, CLM-005, CLM-010, CLM-015, CONTEXT ABI).

## Path aliases

- "SOW", "_STATUS", "_CONTEXT", "MEMORY" mean the files in this deliverable's folder under `projects/chirality-piping/execution/PKG-11_Documentation, Examples, and Education/1_Working/`.
- Parity records are root `execution/_Coordination/AgentRuns/SOW-STAGE2-EXEC-20260712-01/instances/CHANGE-P3/checks/<DEL>/` (latest PASS record; EVIDENCE_MAP).
- `tools/validation/validate_dependencies_schema.py` is cited as the project copy (`projects/chirality-piping/tools/...`), the one the deliverable run records invoke; `check_four_documents.sh`, `validate_enum.py`, `validate_claims_language.py` are root tools.
- `GATE:` tokens are relative to the run folder (A6).
- "the note" = `projects/chirality-piping/docs/theory/centerline_analysis.md` (last changed 767f2eb69, 2026-06-07).

## Judgment calls

- **Rename residue in the artifact (for R3/R4):** the theory note still names the product OpenPipeStress 13 times and pins revision 0.7 in its source table; unlike the user and developer guides it was not renamed on 2026-09-18. Recorded on the SOW surface CP-04 row.
- The 2026-06-07 TASK-11-03-C fan-in review (agent; protected-content, professional-claim and source-inventory scans) binds the frozen note bytes, so note-content rows cite it. The setup-artifact review row CLM-014.r03 is EVIDENCE_OVERTAKEN because the SOW replaced the reviewed kit.
- CLM-010.s02 PARTIALLY_IMPLEMENTED · PARTIAL_SLICE: the classical lineage is conceptual only (TBD-public-history), deferred by a human disposition on RF-11-03-C-003, not a register ruling. STATUS R01 is ALIGNED with `OPEN_ACTION: DEL-11-03:SOW#CLM-010.s02` (F2).
- Source dispositions overtaken by the 2026-06-07 source acceptance (CLM-004.r07, CLM-006.r07, r08-r16 as FG-DEL-11-03-01, r19, CLM-012, CLM-026.r02) → STALE_SETUP_SPECIFICATION · DOC_BEHIND_CODE.
- CP-11 applied to REQ-11-03-08 and CLM-028 (examples "if any"): no examples exist → DOCUMENTED_UNIMPLEMENTED · NOT_STARTED; nothing requires examples. Conservative reading; the verifier may prefer ALIGNED.
- INIT.md citations in source cells judged on the row's subject (C6(b)) with `GAP_WORDING_CHECKED`; CLM-007 (a reference list) takes the pointer finding.
- AC-001 PARTIALLY_IMPLEMENTED: the contract states no verification-versus-validation distinction and no explicit bar on unverified extracted equations (DEC-043 subject); the note itself holds the boundary.

## Possible error in this sealed ledger

- CLM-004.r01 ("explain the lineage from classical flexibility analysis ...") is ALIGNED while CLM-010.s02 records the same lineage as partial. Under F1 I would correct CLM-004.r01 to PARTIALLY_IMPLEMENTED · PARTIAL_SLICE sharing CLM-010.s02's gap. Verifier please check.

## UNKNOWN rows

None.

## Reverse pass

RC-11-0091 (theory note) CLAIMED_BY; all others NOT_MINE, specific where entry points are cited here (registers, CONTRACT, IP boundary, PROFESSIONAL_BOUNDARY, dependency validator). No change of view on sealed rows beyond the CLM-004.r01 point above.

## Selectability

`SelectableUnderCurrentLoop` is `NOT_APPLICABLE` on every row (C9): since 2026-09-19 Piping selects work through owner-steered work graphs, not `## Remaining`.

## Batch consistency

`validate_ledger_v2.py --batch` over the three PKG-11 G1 forward ledgers: PASS, 0 findings. Cross-package shared bodies (mostly with DEL-09-04, DEL-11-05) are outside this batch and are left to the verifier.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081). These are agent dispositions, not owner rulings.
