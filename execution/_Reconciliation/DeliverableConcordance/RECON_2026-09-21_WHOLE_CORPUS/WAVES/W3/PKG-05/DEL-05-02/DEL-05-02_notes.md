# DEL-05-02 notes (W3, PKG-05, worker G1)

## Path aliases

- Project-root evidence tokens (`core/...`, `validation/...`, `schemas/...`) resolve under `projects/chirality-piping/`.
- `execution/_Coordination/AgentRuns/SOW-STAGE2-EXEC-20260712-01/...` (parity reports, claim maps) is the **repository-root** AgentRuns tree; project-level AgentRuns records are cited with the explicit `projects/chirality-piping/` prefix.
- Deliverable-local files are cited by their full path with spaces (resolves at the freeze).
- `GATE:GATE_EVIDENCE/B4_4_SWEEP_9D55/sweep.log.gz` carries per-crate cargo test counts (primitive_loads 49, load_case_algebra 18, stress_recovery 26); no suite was rerun.
- `#Lnn` anchors on SOFTWARE_DECOMP.md point at §12 rows: DEC-017 L608, DEC-018 L609, DEC-022 L613, DEC-023 L614, DEC-025 L616, DEC-026 L617, DEC-028 L619, DEC-068 L659, DEC-077 L668, DEC-092 L683.

## Judgment calls

- **Rule-pack combination supply (FG-DEL-05-02-02).** SOW-014's note and REQ-05-02-004 say code-specific combinations are supplied by user rule packs. No rule-pack construct supplies combinations at the freeze (core/rules has none; the DEL-06-02 interface row is RETIRED/TBD, non-gating for the 2026-06-05 review only). "No bundled defaults" holds. Disposed `PARTIALLY_IMPLEMENTED · PARTIAL_SLICE · PROJECT_BASELINE · BASELINE · OWNER` on REQ-05-02-004, CLM-005 and CLM-023.r03; the owner may read user-authored combinations as meeting the note.
- **Diagnostic envelope fields (FG-DEL-05-02-03).** `AlgebraFinding` carries code, subject and message only; the product `Diagnostic` has no class, remediation or provenance. REQ-05-02-008, CLM-004 (bullet 4, AB-00-06) and CLM-012/REQ-05-02-008 are `PARTIALLY_IMPLEMENTED · PROJECT_BASELINE · BASELINE · NO`. The same shape recurs in DEL-05-03 RQ-005.
- **Grammar TBD (FG-DEL-05-02-01).** Text citing the `_CONTEXT` "rule expression grammar/library" Still-TBD item as open is overtaken by DEC-022 (CLM-003, CLM-023.r01, CLM-024, AC-001). MEDIUM confidence: DEC-022 rules the rule-pack grammar, and the algebra's closed enum is consistent with it. CLM-012/REQ-05-02-009's "CI/release-gate inclusion TBD" is overtaken by DEC-025 (the sweep registers this crate).
- **CLM-023 split** into all five `.rNN` rows because rows take different dispositions (C1); parent is CONTAINER.
- **Remaining R01** is CP-07 (Receipt 87 and the commit-bound sweep exist; closeout acts passed). **R02** is accurate with an open action owned by DEL-09-04 and no governing row here, so `DOCUMENTED_UNIMPLEMENTED · OWNERSHIP_ELSEWHERE` (F2).
- **CP-09:** PASS parity exists but none matches the frozen SOW, so both OUT-001 rows and VER-001 are `STALE_REVIEW_OR_EVIDENCE · EVIDENCE_OVERTAKEN`.
- **CLM-007** (PDU-054 + DEC-077/DEC-092 paragraph): behaviour matches the code; only the revision-0.8 pin is stale, so CP-03 with CP-02 fields.
- **F7:** `to_result_boundary_record` has no product caller (`PRODUCT_CALLER: NONE` on CLM-023.r05); the evaluators are called by product_physics.

## Canonical departures

None (see DEL-05-01 notes for the CP-03/.s01/.s02 treatment, identical here).

## Convention friction

- Whether an overtaken TBD clause inside an otherwise met acceptance row (AC-001) makes the row non-aligned is contested in W2; F1 applied strictly here.

## Reverse pass

Answers: CLAIMED_BY 6, PARTIAL 4, COVERS 4, NOT_MINE 405. No change of view on sealed rows. The product combination evaluation (RC-05-0103) and temperature-basis selection (RC-05-0050) are claimed; the material schema, material form and pre-solve validation are partial.

## Batch consistency

`validate_ledger_v2.py --batch` over the three sealed forward ledgers: **PASS, 0 consistency findings**. Shared bodies (CONTEXT Decomposition Reference, Architecture Basis Injection and its .s01/.s02, Package Reference, PREPARATION Notes, Identification Type/Context envelope rows) took identical fields in all three ledgers. No flagged pair to justify or correct.

## Selectability

`SelectableUnderCurrentLoop` is `NOT_APPLICABLE` on every row (C9): since 2026-09-19 Piping selects work through owner-steered work graphs, not `## Remaining`.

## UNKNOWN rows

None in this ledger.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081). No row states or implies a release, approval, compliance or certification claim; dispositions are agent judgments, not owner rulings.
