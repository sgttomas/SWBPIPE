# DEL-16-04 notes — Agent rationale and professional-boundary controls (worker G1, W1)

## Path aliases

- Deliverable-folder paths contain spaces, so they cannot be evidence tokens
  (W-13). They appear in `ContextRefs` or by alias in Notes:
  `RF-16xx` = `projects/chirality-piping/execution/PKG-16_Model Operation and Agent Proposal Framework/1_Working/DEL-16-xx_<name>/Review_Findings.csv`.
- `SOW`, `STATUS`, `CONTEXT`, `MEMORY` = that deliverable folder's
  `ScopeOfWork.md`, `_STATUS.md`, `_CONTEXT.md`, `MEMORY.md`.
- Parity evidence is the repository-root record
  `execution/_Coordination/AgentRuns/SOW-STAGE2-EXEC-20260712-01/instances/CHANGE-P4/checks/DEL-16-xx/{parity.md,claim-map.csv}`
  (latest per `EVIDENCE_MAP.csv`); migration parity `.../WORKING-P4-PKG16/children/AUTHOR-B1/members/DEL-16-xx/parity-a.md` goes in ContextRefs.
- Project-root tokens (`schemas/`, `core/`, `apps/`, `tests/`, `fixtures/`) resolve under `projects/chirality-piping/`; `tools/validation/...` is the repository-root lint (DEC-081 grant).

## Judgment calls

- **FG-DEL-16-04-01 (W-2).** DEC-081 (D-48) placed the claims registry
  `docs/claims_registry.md` and the deterministic claims-language lint
  (repo-root `tools/validation/validate_claims_language.py`, self-check
  GEN-13) under DEL-16-04. The SOW still declares linter integration and
  UI/API/report guard coverage TBD, and never names either artifact. Those
  rows are marked `STALE_REVIEW_OR_EVIDENCE · SCOPE_REDIRECTED_BY_RULING`
  (DecisionBasis DEC-081).
- **FG-DEL-16-04-02.** REQ-16-04-02 restates SOW-070 and is judged like
  DEL-16-03 REQ-005 → PARTIALLY_IMPLEMENTED. Rationale persists through
  editor_intents, but acceptance decisions and receipts are session-only.
- CLM-004's "final UI/agent workflow presentation TBD" is read as still
  accurate, because a desktop AgentProposalPanel exists but is review-only.
  MEDIUM confidence.
- REQ-16-04-09 is ALIGNED with MEDIUM confidence. The applier's structured
  route check, the required professional_boundary field and the claims lint
  partly evidence it; "integration TBD until paths are assigned" remains a
  fair statement.
- CLM-012 → CP-01 (`STALE_SETUP_SPECIFICATION`, setup-era origin text): it
  names the retired four-document kit.
- STATUS SURFACE: not literal CP-05, because no history entry is newer than
  Last Updated. CanonicalSituation is left empty but the fields match CP-05
  by analogy. The file was edited on 2026-07-28 (SCA-008/DEC-091 R02 gate
  correction, commit 9b5207670) with no Last Updated change or history line.
- STATUS R01 is ALIGNED as a standing constraint (NO_OPEN_ACTION beyond
  preserving the gate). R02 → `PARTIALLY_IMPLEMENTED · DEFERRED_BY_RULING`
  (DEC-042 R7 hold; DEC-094 reserves the generator's ownership). R03 is
  ALIGNED (W-6).
- CLM-003 cites the removed `INIT.md`: stale source pointer only.
- No `.rNN` split was used.

## Canonical departures

None. The STATUS surface's CP-05 analogy is not a departure, because no CS
or CP ID is assigned.

## Convention friction

- DEC-081 assigns a repo-root tool to this deliverable. Its path is a
  root-level evidence token, `tools/validation/...`, which no project copy
  shadows.

## Reverse pass

- It did not change my view of anything sealed.
- Answers:
  - CLAIMED_BY: the rationale engine (RC-16-0004).
  - PARTIAL: the review-only proposal panel and the sample-proposal loader.
  - UNKEYED: the claims registry (RC-16-0233), consistent with FG-01.
  - NOT_MINE: PROFESSIONAL_BOUNDARY.md, which the Scope excludes to DEL-01-04.
  - COVERS: ledger and offline-intake consumers.

## Batch consistency

`validate_ledger_v2.py --batch` over the four PKG-16 forward ledgers: **PASS, 0
consistency findings**. No `CANONICAL_DEPARTURE` was needed or used.

## UNKNOWN rows

None. No row in this ledger is `UNKNOWN`, so no smallest-next-check is owed.

## Selectability

`SelectableUnderCurrentLoop` is `NOT_APPLICABLE` on every row (C9): since
2026-09-19 Piping selects work through owner-steered work graphs, not
`## Remaining`.

## Fences

Gate evidence is suite-level only (`GATE:GATE_EVIDENCE/B4_4_SWEEP_9D55/SUMMARY.json`;
no suite rerun). No protected standards, vendor or private data is quoted. No
external-corpus equation artifact is used. Nothing here states or implies
release, approval, compliance or certification, and no disposition is an owner
ruling.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
