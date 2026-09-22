# DEL-05-03 notes (W3, PKG-05, worker G1)

## Path aliases

- Project-root evidence tokens (`core/...`, `validation/...`, `schemas/...`) resolve under `projects/chirality-piping/`.
- `execution/_Coordination/AgentRuns/SOW-STAGE2-EXEC-20260712-01/...` (parity reports, claim maps) is the **repository-root** AgentRuns tree; project-level AgentRuns records are cited with the explicit `projects/chirality-piping/` prefix.
- Deliverable-local files are cited by their full path with spaces (resolves at the freeze).
- `GATE:GATE_EVIDENCE/B4_4_SWEEP_9D55/sweep.log.gz` carries per-crate cargo test counts (primitive_loads 49, load_case_algebra 18, stress_recovery 26); no suite was rerun.
- `#Lnn` anchors on SOFTWARE_DECOMP.md point at §12 rows: DEC-017 L608, DEC-018 L609, DEC-022 L613, DEC-023 L614, DEC-025 L616, DEC-026 L617, DEC-028 L619, DEC-068 L659, DEC-077 L668, DEC-092 L683.

## Judgment calls

- **Pressure reference (FG-DEL-05-03-02).** The crate and product recover thin-wall membrane pressure stress (p*r/t, half for longitudinal). The DEL-05-03 investigation (2026-09-08) found the thin-wall reduction inconsistent with an exact-annulus reference and listed seven owner decisions; the physics-audit plan holds pressure interpretation (D01-D06); D-67 adopted only a dormant exact-annulus kernel. RQ-001 was split: the axial/bending/torsion part is ALIGNED (validation witnesses exist, UNVERIFIED); the pressure element `RQ-001.s01` and CONTEXT#description are `ENGINEERING_AUTHORITY_REQUIRED · AUTHORITY_UNCLEAR · INVARIANT · OWNER_HOLD · VALIDATION · ENGINEERING`. Declarative rows that only describe the implemented formula (CLM-003, CLM-005) stay on their own substance.
- **Unit catalog and tolerance TBDs (FG-DEL-05-03-01):** DEC-018 and DEC-024/DEC-026 overtake "conversion catalog TBD" and "production tolerance policy TBD" (CLM-003, CLM-007, CLM-013, CLM-014, CLM-025, RQ-006, AC-001), consistent with DEL-05-01 CLM-017.
- **RQ-005** diagnostics lack provenance (`StressFinding` = code, subject, message), same finding shape as DEL-05-02 REQ-05-02-008.
- **Remaining R01-R03** are accurate and record open work (production pressure activation; versioned public result contract; connectors) with no governing row in this ledger, so they take the gap themselves (F2): R01/R03 `PARTIALLY_IMPLEMENTED · DEFERRED_BY_RULING` (D-67 adopted a dormant kernel only; AdoptedByReference YES), R02 `PARTIAL_SLICE`.
- **Validation evidence:** stress hand calcs, the stress benchmark suite and validation-manual pages exist as agent-authored DRAFT_EVIDENCE with no human disposition, so `SourceReliability UNVERIFIED`; I did not treat their existence as a VERIFIED_NOT_VALIDATED gap for axial/bending/torsion.
- **F7:** stations, sweeps, ranges, unit-metadata recovery and boundary records have no product caller; product_physics calls `recover_stresses` only.

## Canonical departures

None (CP-03/.s01/.s02 as in DEL-05-01).

## Convention friction

- A requirement row split by `.s01` so the parent carries the non-pressure parts: the parent row's Notes name the split; verifier should confirm this is acceptable under F1.

## Reverse pass

Answers: CLAIMED_BY 5, PARTIAL 1, COVERS 4, UNKEYED 2 (exact-annulus kernel RC-05-0305 → nearest RQ-001.s01; bend/branch user multiplier review rows RC-05-0146 → nearest CLM-005), NOT_MINE 407. No change of view on sealed rows; the unkeyed kernel is the same open pressure work R01 records.

## Batch consistency

`validate_ledger_v2.py --batch` over the three sealed forward ledgers: **PASS, 0 consistency findings**. Shared bodies (CONTEXT Decomposition Reference, Architecture Basis Injection and its .s01/.s02, Package Reference, PREPARATION Notes, Identification Type/Context envelope rows) took identical fields in all three ledgers. No flagged pair to justify or correct.

## Selectability

`SelectableUnderCurrentLoop` is `NOT_APPLICABLE` on every row (C9): since 2026-09-19 Piping selects work through owner-steered work graphs, not `## Remaining`.

## UNKNOWN rows

None in this ledger.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081). No row states or implies a release, approval, compliance or certification claim; dispositions are agent judgments, not owner rulings.
