# DEL-11-01 notes (W3, PKG-11, worker G1)

Forward ledger 119 rows (66 required keys, all-or-none `.rNN` splits of CLM-004/005/006/013/018/020, `.sNN` for SOW frontmatter, CLM-010, CLM-014, VER-001, CONTEXT ABI, MEMORY). R0 pilot: calibration repairs carried (OUT-001 and VER-001 cite CHANGE-P3 parity as CP-09 STALE_REVIEW_OR_EVIDENCE; accessibility kept partial on VER-001.s02 and AC-001; CLM-013.r03 no longer ALIGNED without a binding review; CLM-020.r03/.r04 cite recorded runs, "not rerun"; STATUS R18 reasoning moved to Notes as context).

## Path aliases

- "SOW", "_STATUS", "_CONTEXT", "MEMORY" mean the files in this deliverable's folder under `projects/chirality-piping/execution/PKG-11_Documentation, Examples, and Education/1_Working/`.
- Parity records are root `execution/_Coordination/AgentRuns/SOW-STAGE2-EXEC-20260712-01/instances/CHANGE-P3/checks/<DEL>/` (latest PASS record; EVIDENCE_MAP).
- `tools/validation/validate_dependencies_schema.py` is cited as the project copy (`projects/chirality-piping/tools/...`), the one the deliverable run records invoke; `check_four_documents.sh`, `validate_enum.py`, `validate_claims_language.py` are root tools.
- `GATE:` tokens are relative to the run folder (A6).
- "the guide" = `projects/chirality-piping/docs/user_guide/index.md`; R18 = the 2026-07-25 owner adoption record `projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260725-CANDIDATE-BRIEFS-R18/instances/WI-PKG11-DEL1101-EXECUTION/OWNER_ADOPTION.md`.

## Judgment calls

- **A3a ACCEPTED_DIVERGENCE (FG-DEL-11-01-01; CLM-010.s02, CLM-014.s02, CLM-017, CLM-018.r05).** The SOW says the guide is read-only / not edited; the guide was edited under this deliverable. R18 is a verbatim, hash-bound owner message naming "a guide-only SOW exception ... without editing ScopeOfWork.md", and DEC-107 (iii) directed guide edits while leaving live SOWs untouched. Routed to R4 for confirmation (`AuthorityNeeded OWNER`). Scope caveat for R4: the guide was created 2026-05-09 before either act.
- CLM-004.r02 (anticipated artifact framing) is STALE_SETUP_SPECIFICATION · DOC_BEHIND_CODE, not the A3a group: it is framing, not an exclusion.
- Conditional TBD clauses (UG-REQ-002, CLM-005.r06, CLM-024) ALIGNED: decided items are stated as decisions. Unconditional TBD lists (CLM-004.r08, CONTEXT ABI .s02) are TBD_RULED (DEC-022/023/057, SCA-003/004).
- CLM-006.r07: guide section 9 has no equipment-load slot → PARTIALLY_IMPLEMENTED.
- CLM-025 IMPLEMENTED_DIFFERENTLY at LOW: only under a lifecycle reading of "accepted implementation deliverable".
- Review-record rows (CLM-013.r03, CLM-020.r06): latest protected-content review (R18, agent) predates the 2026-09-18 rename/removal edits → STALE_REVIEW_OR_EVIDENCE · EVIDENCE_OVERTAKEN, LOCAL_DESIGN under F8 (boundary holds on worker read). Content requirement UG-REQ-008 ALIGNED with GAP_WORDING_CHECKED (diff e60ef7c34..00115c719 is rename/removal only).
- The CI self-check reports claims-lint failures only as WARN, so PR #834 CI is not cited as a lint pass.

## Canonical departures

CLM-005.r02, CLM-007, CLM-018.r02: CP-02 pointer (INIT.md, removed 2026-07-04) but F3 origin at 7bee9ae41 → STALE_SETUP_SPECIFICATION, marked `CANONICAL_DEPARTURE`.

## Convention friction / possible error in this sealed ledger

- CLM-005.r02 flags the INIT.md citation in a source-basis cell, while UG-REQ-005 (same kind of cell) is ALIGNED without it. Under C6(b) the row's subject holds; I would correct CLM-005.r02 to ALIGNED with a note (the later DEL-11-03 ledger applies that reading). Conservative direction; not a false alignment.
- `.sNN` sub-claims of SURFACE keys (SOW.s01, MEMORY.s01) are used for frontmatter pins and the undated PDU-055 MEMORY header, which no issued key covers.

## UNKNOWN rows

None.

## Reverse pass

RC-11-0173 (user guide) CLAIMED_BY; all others NOT_MINE, with capability-specific reasons for every entry point this ledger cites (F5: registers, TYPES, DIRECTIVE, CONTRACT, SPEC, analysis-status schema, invented examples, dependency validator, AGENTS.md, docs README). The reverse pass did not change my view of any sealed row.

## Selectability

`SelectableUnderCurrentLoop` is `NOT_APPLICABLE` on every row (C9): since 2026-09-19 Piping selects work through owner-steered work graphs, not `## Remaining`.

## Batch consistency

`validate_ledger_v2.py --batch` over the three PKG-11 G1 forward ledgers: PASS, 0 findings. Cross-package shared bodies (mostly with DEL-09-04, DEL-11-05) are outside this batch and are left to the verifier.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081). These are agent dispositions, not owner rulings.
