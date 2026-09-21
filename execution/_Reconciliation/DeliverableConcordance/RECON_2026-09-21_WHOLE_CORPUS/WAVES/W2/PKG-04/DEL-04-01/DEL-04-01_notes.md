# DEL-04-01 — 3D frame stiffness kernel: worker notes (W2, PKG-04)

Run `HELP-HUMAN-PIPING-20260921-RECONCILIATION`, gate wave W2, worker G1
(TASK). Frozen state `00115c71931bcae79909602d653740d3bb72dfa1`. Forward
ledger sealed before the routing file was read (B1). Agent judgments only;
nothing here is an owner ruling. Shared treatments are in
`../_WORKER_DEL-04-01_NOTES.md`.

## Path aliases

- Deliverable folder paths contain spaces, so `ScopeOfWork.md`, `_STATUS.md`,
  `_CONTEXT.md`, `MEMORY.md`, `_REVIEW.md` and `_run_records/*` are named in
  Notes and `NormativeSource`, never as evidence tokens.
- The SOW-STAGE2 parity records live at the repository root
  (`execution/_Coordination/AgentRuns/SOW-STAGE2-EXEC-20260712-01/…`), not
  under the project; cited in that root form.
- `core/…`, `validation/…`, `apps/…` tokens resolve under
  `projects/chirality-piping/`.
- Crate package names carry the former name (`open_pipe_stress_frame_kernel`,
  `open_pipe_stress_sparse_direct`, `open_pipe_stress_curved_bend`); the
  paths are `core/solver/frame_kernel` etc.

## Judgment calls

- **SURFACE row:** the frontmatter `decomposition_basis` pin (69ac259a,
  revision 0.8) is the recorded common defect (CP-02). CP-04 is not recorded
  because this SOW names the path `core/solver/frame_kernel`, not the package
  identifier. R3 may want to decide whether path mentions of crates whose
  package names carry the former name count as residue.
- **Resolved TBDs** (CLM-004, 006, 009, 011, 013, 016, 018, 024, 026, 027,
  028, REQ-004, CLM-012.r01/.r02, ABI `.s02`): STALE_SETUP_SPECIFICATION ·
  DOC_BEHIND_CODE (F3: first present at 7bee9ae41). DEC-023 settled the
  solver library and storage, DEC-026 the tolerance tiers, code the DOF
  order and transform convention.
- **Sparse as default:** CLM-014/CLM-021 say sparse-as-default is held. It
  was promoted to the product interactive default on 2026-06-22 (commit
  26d2cff2f, DEC-053 tranche), before the 2026-07-12 declaration.
- **REQ-011 / CLM-012.r03 (FG-DEL-04-01-01):** PARTIALLY_IMPLEMENTED. The
  desktop preview `Diagnostic` returned by product_physics carries id, code,
  severity, message, source and affected_refs only; class, remediation and
  provenance are dropped on that service boundary (the solver diagnostics
  crate and the runner envelope binding carry them).
- **CLM-012/REQ-010:** PARTIALLY_IMPLEMENTED. The registered local DEC-025
  sweep runs cargo (gate evidence, not rerun); the frozen hosted workflows run
  no cargo commands.
- **Protected-content review (FG-DEL-04-01-02: CLM-012/REQ-012, CLM-020):**
  STALE_REVIEW_OR_EVIDENCE · EVIDENCE_OVERTAKEN · INVARIANT ·
  IP_DATA;RECORD · REVIEW. The last dedicated check is the 2026-06-05
  lifecycle QA report (and `_REVIEW.md` PB-001); later kernel changes carry
  only tranche self-attestations; no DEC-058 scan record exists in the frozen
  tree. REQ-012 itself (no protected content imported) is ALIGNED on source
  inspection.
- **STATUS R01:** PARTIALLY_IMPLEMENTED · PARTIAL_SLICE · PROJECT_BASELINE ·
  OWNER. Accurate and open (R14 handoff; F4 prerequisite record 2026-09-08);
  no governing SOW row carries the work, so F2 gives the row the gap.
- **REQ-006:** ALIGNED. Performance targets remain unset (DEC-053 timing is
  observational), which is what the requirement says.
- **REQ-007:** ALIGNED with `PRODUCT_CALLER: NONE` for FrameKernelUnitBasis;
  the product path is unit-checked by its own normalization.
- **CLM-019 (procedure checks):** ALIGNED as a method; the protected-review
  check is conditioned on release.

## Canonical departures

None. CS rows inherit their fields exactly. CLM-002/CLM-023 use CP-02 (the
CP-03 pin treatment names CP-02); CLM-014/CLM-021 use CP-03.

## Convention friction

- CP-09 names only OUT-001 and SOW parity rows; applied to VER-001 as a SOW
  parity row. The protected-review rows use the CP-09 analogue
  (EVIDENCE_OVERTAKEN) because the text is accurate and the review evidence
  is what is overtaken; F3's origin test concerns stale text.
- SURFACE rows carry one disposition; when two common defects coexist (see
  DEL-04-03) one moves to a `.sNN` sub-claim.

## UNKNOWN rows

None.

## Reverse pass

- 13 CLAIMED_BY, 3 PARTIAL, 3 COVERS, 4 UNKEYED, 317 NOT_MINE.
- **UNKEYED:** the curved_bend crate capabilities (element, arc-consistent
  loads, radial pressure wall loads, arc section resultants) were built under
  DEL-04-01 tranches (TP-PMM-P1-CURVEDBEND-001..004, R14-W1-T3; DEC-070), but
  no DEL-04-01 key covers bend elements. AC-001 is the nearest key.
- Did the reverse pass change my view of anything sealed? One point. The
  UNKEYED curved-bend ownership shows the SOW has no requirement for work it
  hosted. That is an R3 scope finding, not a forward-row error. The sealed
  rows stand.

## Batch consistency

`--batch` over the three sealed forward ledgers: PASS, 0 findings.

## Selectability

`SelectableUnderCurrentLoop` is `NOT_APPLICABLE` on every row (C9): since
2026-09-19 Piping selects work through owner-steered work graphs, not
`## Remaining`.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
