# Worker G1 carry-forward notebook — W3 PKG-13 (DEL-13-01..04)

Running record of recurring judgments so the same situation gets the same
treatment across the four ledgers. Agent judgments, not owner rulings.

## Recurring situations

1. **Rename residue (CP-04).** Every PKG-13 schema `$id` uses the
   `openpipestress.org` host and the schema title carries the former name.
   Recorded once on each SOW SURFACE row (default CP-04 fields); item rows
   that restate the `$id` are judged on substance (accurate -> ALIGNED).
2. **Stale graph/decomposition pins (CP-02).** DAG-002/DAG-006/DAG-007 labels
   and decomposition rev 0.7/0.8 pins: STALE_REVIEW_OR_EVIDENCE ·
   BASIS_POINTER_STALE · LOCAL_DESIGN · NONE · RECORD · NO, whatever their
   origin (F3 pin exception). Current at the freeze: rev 0.12, DAG-010.
3. **D-41 R5 T7 PDU-055 current declaration blocks.** CanonicalSituation
   `CP-03`, fields per CP-02 (they pin rev 0.8 / DAG-007). Delegation to
   `## Remaining` recorded as not relied on (A4).
4. **Four-document residue (CP-01).** Rows naming Datasheet/Specification/
   Guidance/Procedure as current records: origin checked with `git log -S`;
   text first present at 7bee9ae41 -> STALE_SETUP_SPECIFICATION;
   later text -> STALE_REVIEW_OR_EVIDENCE.
5. **Parity / output matrix (CP-09).** EVIDENCE_MAP lists NONE_FOUND for all
   four, but the A3 discovery step found PASS parity records in root
   `execution/_Coordination/AgentRuns/SOW-STAGE1-20260712/.../RECON-FANIN/`
   and `SOW-STAGE2-EXEC-20260712-01/.../TASK-PIP-13-0N/stage1_evidence/`.
   None binds the frozen SOW bytes (SOWs edited by D-48 Wave 2, 2026-07-17),
   so OUT-001 matrix, AC-001 and VER-001 take CP-09 "PASS exists, none
   matches": STALE_REVIEW_OR_EVIDENCE · EVIDENCE_OVERTAKEN · LOCAL_DESIGN ·
   NONE · RECORD · NO.
6. **Architecture Basis Injection.** CS-04 row for the pin; `.s01` for the
   PKG-00 SEMANTIC_READY statement (STALE_REVIEW_OR_EVIDENCE · RECORD_DRIFT ·
   LOCAL_DESIGN · NONE · RECORD · OWNER, corpus cluster SR-1); `.s02` for the
   Still-TBD list's container item ruled by DEC-017/DEC-028
   (STALE_SETUP_SPECIFICATION · SCOPE_REDIRECTED_BY_RULING, origin
   7bee9ae41). For Python-engine deliverables the "Rust core/application
   services" Resolved Baseline is judged separately where it diverges.
7. **F7 product callers.** At the freeze: `design_knowledge.schema.json` and
   `constraint.schema.json` are loaded only by their structural tests;
   `core/constraints/validation` has only test callers;
   `core/model_transform/physical_to_analytical` is referenced by the handoff
   exporter only as a contract-path string. ALIGNED rows on these carry
   `PRODUCT_CALLER: NONE`.
8. **Desktop preview design knowledge.** The live desktop Knowledge panel
   reads `fixtures/product_preview/invented_design_knowledge.json`, a preview
   shape that is not the DEL-13-01 schema (bare-string provenance), without
   schema validation. Recorded on DEL-13-01 REQ-13-01-011 as
   IMPLEMENTED_DIFFERENTLY · POSSIBLE_DEFECT · INVARIANT · VALIDATION.
9. **Gate evidence.** Test-backed rows cite
   `GATE:GATE_EVIDENCE/B4_4_SWEEP_9D55/SUMMARY.json` (pytest -q tests, 1138
   passed, 0 failed; product paths identical to the freeze); not rerun.
10. **Parameter value loophole.** In both PKG-13 schemas `Parameter.value` is
    `oneOf string | Quantity | string[]` independent of `value_kind`, so a
    `quantity` parameter may carry a bare string. Unit-rule requirement rows:
    PARTIALLY_IMPLEMENTED · POSSIBLE_DEFECT · PROJECT_BASELINE · NONE ·
    BASELINE · REVIEW, MEDIUM confidence (check each schema before applying).
11. **Split policy.** `.rNN` blocks are split only where items take different
    dispositions and a non-aligned item would otherwise hide aligned
    boundary substance; pointer-only drift in an otherwise aligned block is
    disposed on the block row directly.
