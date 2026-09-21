# DEL-07-05 Results viewer — worker notes (W1, PKG-07, worker G3)

Forward ledger `DEL-07-05_forward.csv`: 105 rows (76 required keys, 7 canonical
assignments inherited without departure, `.rNN` splits of CLM-003, CLM-004,
CLM-005 and CLM-033, two `.s01` sub-claims). Sealed in `DEL-07-05_SEAL.txt`.
Evidence read from the frozen checkout at `00115c719` only. Not an R0 pilot.
Recurring-situation rules shared with DEL-07-07 and DEL-07-08 are in
`../_WORKER_DEL-07-05_NOTES.md`.

## Path aliases

- Deliverable folder paths contain spaces, so the validator rejects them as
  evidence tokens. Deliverable-local files (`ScopeOfWork.md`, `_STATUS.md`,
  run records, `Dependencies.csv`) are cited in `ContextRefs` and Notes.
- Parity records live at the repository root:
  `execution/_Coordination/AgentRuns/SOW-STAGE2-EXEC-20260712-01/instances/CHANGE-P2/checks/DEL-07-05/`
  (not under `projects/chirality-piping/`).
- Result row kinds: the SOW's `ux/uy/uz` and `rx/ry/rz` are now the kinds
  `global_nodal_displacement_x/y/z` and `global_nodal_rotation_x/y/z`; row ids
  keep the `:ux`…`:rz` tails (`core/product_physics/src/lib.rs` near L7394).
- `docs/SPEC.md` section numbers cited by the SOW are offset: frozen SPEC has
  §7 rule-pack evaluator, §8 GUI, §9 reporting. Recorded in Notes; the
  disposition follows the requirement's substance (C6(b)).

## Judgment calls

- **FG-DEL-07-05-01, rotational visualization.** `PipeViewport.buildDeformationOverlay`
  uses only translation components; rotation rows are emitted and listed in
  the table's Rotation group but never drawn. STATUS R01 → DOCUMENTED_UNIMPLEMENTED
  · NOT_STARTED; CLM-004.r03, CLM-010 and CONTEXT description → PARTIALLY_IMPLEMENTED
  · PARTIAL_SLICE. AuthorityNeeded OWNER because the SOW (CLM-024) says the
  rotational visual semantics must still be defined.
- **FG-DEL-07-05-02, ratio surface (possible reviewer attention).**
  `ResultsPanel` renders `<GoverningRatioState ratioCount={0} />` as a literal,
  and the v0.2 semantic contract has no ratio-family row (every row
  `governing_ratio_eligible=false`). A supplied user-rule ratio row therefore
  shows under "Other" as `unknown`, and the governing-ratio state always reads
  unavailable; `App.test.tsx` asserts this for an invented
  `user_rule_governing_ratio` row. I read this as deliberate (PR #787, result
  integrity: "actual ratios … have their proper separate meanings"), so
  CLM-016 and AC-001 are STALE_REVIEW_OR_EVIDENCE · DOC_BEHIND_CODE and STATUS
  R02 is REMAINING_STATE_MISMATCH · DOC_BEHIND_CODE (A2: the deliverable
  catches up). REQ-07-05-001/005 and CLM-004.r02/r07 are PARTIALLY_IMPLEMENTED
  (CP-11 for r07 and REQ-005: only the "unavailable" branch exists). If the
  owner meant supplied ratios to stay countable, the literal `0` is a defect
  rather than a design choice; the verifier should look at this.
- REQ-07-05-004 → PARTIALLY_IMPLEMENTED · OWNERSHIP_ELSEWHERE: mechanics, rule
  and professional statuses are shown in the Solve panel readiness list and the
  rule-check run panel, not in the Results panel. No `CODE_COMPLIANT` token
  exists in desktop source.
- CLM-005.r02 and CLM-033.r02 (UI component/state library and layout TBD)
  → CP-10. The code settles them; D-68's adopted production-UI plan is context
  and I did not treat it as settling them for this deliverable.
- CLM-005.r03 and CLM-033.r01 (result-envelope fields TBD) → STALE_REVIEW_OR_EVIDENCE
  · DOC_BEHIND_CODE, AdoptedByReference YES (D-67 context). The v0.2 semantic
  contract now fixes the fields; ownership stays with the result-envelope
  deliverables.
- CLM-024 → STALE_REVIEW_OR_EVIDENCE · RECORD_DRIFT for "`_STATUS.md` remains
  the sole work-discovery home" (project `AGENTS.md` replaces
  deliverable-only work selection with the session work graph).
- CLM-007 → CP-02: `INIT.md` does not exist in the frozen Piping tree.
- Purpose-section OUT-001 is ALIGNED on substance (the contract exists).
  Output-matrix OUT-001 and praxeology VER-001 follow CP-09: 7 PASS parity
  records, none matching the frozen SOW (`af413bb0…`).
- VER-07-05-003 is ALIGNED on the 2026-06-16 dependency-refresh run record
  ("VALID"); `Dependencies.csv` last changed in that commit (28219696d).
  Not rerun.
- CLM-006 and VER-07-05-005 are ALIGNED: their boundary lists and review steps
  hold. Their accurate rotational sentences are dispositioned on the FG-01 rows.

## Canonical departures

None. All 7 CS rows inherit their fields. CS-04 carries `.s01` for the
"PKG-00 at SEMANTIC_READY" statement (all PKG-00 deliverables are IN_PROGRESS
at the freeze) → STALE_REVIEW_OR_EVIDENCE · RECORD_DRIFT. Its "Still TBD" items
are still TBD in rev 0.12 (DEC-012, OI-002, OI-006), so they get no sub-claim.

## Convention friction

- Evidence columns cannot hold the deliverable's own files (spaces in the path).
- The SOW SURFACE row can carry only one profile. CP-04 (rename residue in
  CLM-012) takes it, so the frontmatter pin is `SOW.s01` (CP-02).
- A Remaining item that is accurate and still open has no named pattern. I
  applied C6(b): the disposition describes the subject, and accuracy goes in
  Notes.

## UNKNOWN rows

None.

## Reverse pass

487 routed capabilities: CLAIMED_BY 4 (ResultsPanel, deformation overlay, the
`core/gui/results_viewer` contract, mechanics gap ledger), PARTIAL 2 (result
semantics binding, result/diagnostic resolution), COVERS 3 (historical-run
display reusing ResultsPanel, solver rotation-row emission, r2 smoke journeys),
NOT_MINE 478. The reverse pass did not change my view of any sealed row.

## Batch consistency

`validate_ledger_v2.py --batch` over my three forward ledgers: PASS, 0 findings.

## Selectability

`SelectableUnderCurrentLoop` is `NOT_APPLICABLE` on every row (C9). Since
2026-09-19, Piping selects work through owner-steered work graphs, not
`## Remaining`.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081). These
dispositions are agent judgments, not owner rulings, and make no release,
approval, compliance or certification claim.
