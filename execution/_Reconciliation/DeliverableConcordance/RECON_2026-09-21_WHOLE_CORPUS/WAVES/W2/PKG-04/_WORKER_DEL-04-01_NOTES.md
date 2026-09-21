# Worker notebook — W2 PKG-04 G1 (DEL-04-01, DEL-04-02, DEL-04-03)

Carry-forward record so recurring situations get the same treatment. Agent
judgments only; not owner rulings. Standard claim fence applies (F-PIP-2;
claims taxonomy per DEC-081).

## Recurring treatments

1. **SOW SURFACE row.** Common defect = frontmatter `decomposition_basis`
   pin (commit 69ac259a, revision 0.8; frozen basis 0.12): CP-02
   STALE_REVIEW_OR_EVIDENCE · BASIS_POINTER_STALE · LOCAL_DESIGN · RECORD.
   Exception: where the SOW names an active former-name identifier
   (DEL-04-03 names `open_pipe_stress_frame_kernel`), CP-04 takes the SURFACE
   row and the frontmatter pin moves to a `SOW.s01` sub-claim.
2. **CP-04 scope.** Applied only where the SOW literally names the former-name
   identifier. A path (`core/solver/frame_kernel`) is not treated as naming
   the crate package name. MEMORY mentions are history, not residue.
3. **Resolved-TBD rule.** A met claim whose only gap is a setup-era TBD or
   future-tense clause since settled in code or by ruling (DEC-023, DEC-026,
   DEC-053) takes the stale class (F3 origin test) · DOC_BEHIND_CODE ·
   LOCAL_DESIGN. SCOPE_REDIRECTED_BY_RULING is kept for structural
   re-direction (PKG-00 SEMANTIC_READY statement, D-43).
4. **"Future implementation shall X" with X implemented and no TBD clause:**
   ALIGNED (C6(b); future framing noted).
5. **D-41 declarations.** Pins of revision 0.8 / DAG-007 → CanonicalSituation
   CP-02 (CP-03 names this treatment). Declarations stale for other reasons
   (sparse-as-default held) → CP-03 · DOC_BEHIND_CODE. Accurate declarations
   → CP-03 · ALIGNED.
6. **OUT-001 (both) and VER-001:** CP-09 (no PASS parity matches frozen SOW),
   citing the CHANGE-P1-PKG04 checks parity.md and claim-map.csv.
7. **Protected-content review evidence.** Where a dedicated review check
   exists but predates later code changes (2026-06-05 lifecycle QA reports;
   DEL-04-03 review-readiness record) and no DEC-058 scan record exists:
   STALE_REVIEW_OR_EVIDENCE · EVIDENCE_OVERTAKEN · INVARIANT · IP_DATA;RECORD ·
   REVIEW (CP-09 analogue). The prohibition requirement itself (no protected
   content imported) is ALIGNED on source inspection.
8. **Procedure "Verification/check" lists** are methods, not state claims:
   ALIGNED when the checks remain applicable. "Records" lists are
   requirements to hold records: judged by whether the records exist.
9. **CONTEXT SURFACE:** STALE_SETUP_SPECIFICATION · DOC_BEHIND_CODE (setup
   text, stale pins/PKG-00 statement/solver-library TBD judged on own rows).
   ABI `.s01` PKG-00 SEMANTIC_READY statement → STALE_SETUP_SPECIFICATION ·
   SCOPE_REDIRECTED_BY_RULING; `.s02` Still-TBD solver library →
   STALE_SETUP_SPECIFICATION · DOC_BEHIND_CODE (DEC-023).
10. **Product caller.** frame_kernel, straight_pipe (element/recovery),
    linear_supports `prepare_boundary`, sparse_direct are product-called via
    core/product_physics → apps/desktop/src-tauri. No product caller:
    FrameKernelUnitBasis, StraightPipeBoundaryMetadata,
    StraightPipeElement::weight_hook, apply_linear_supports.
11. **AB-00-06 diagnostics field set:** desktop preview `Diagnostic` lacks
    class, remediation, provenance → PARTIALLY_IMPLEMENTED · PARTIAL_SLICE ·
    PROJECT_BASELINE · BASELINE where a claim lists those fields.
12. **Hosted CI:** frozen `.github/workflows` run no cargo; registered local
    DEC-025 sweep does (GATE). Claims requiring "CI" solver gates →
    PARTIALLY_IMPLEMENTED · PARTIAL_SLICE · LOCAL_DESIGN.
13. **Conditional "keep X TBD until a brief or ruling seals it" rules** are
    ALIGNED when followed (DEL-04-01 CLM-025, DEL-04-03 R10); only a flat
    "X is TBD" statement since settled takes the stale class (item 3).
14. **Requirement naming a surface the product bypasses** (DEL-04-03 R12,
    apply_linear_supports): IMPLEMENTED_DIFFERENTLY · OWNERSHIP_ELSEWHERE ·
    LOCAL_DESIGN; descriptive rows about the same engine stay ALIGNED with
    `PRODUCT_CALLER: NONE` (F7).
15. **Stale test counts presented as current evidence** (14 vs 15 in
    linear_supports): STALE_REVIEW_OR_EVIDENCE · DOC_BEHIND_CODE when the text
    is post-migration; a dated record ("June 5 evidence records 14") is
    accurate as a record.

## Status

All three forward ledgers sealed 2026-09-21; batch consistency PASS, 0
findings. Routing file read only after sealing.
