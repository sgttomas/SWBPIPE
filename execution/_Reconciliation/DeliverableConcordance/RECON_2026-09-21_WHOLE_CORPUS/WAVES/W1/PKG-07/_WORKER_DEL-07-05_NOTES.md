# Worker notebook — W1 PKG-07 worker G3 (DEL-07-05, DEL-07-07, DEL-07-08)

Carry-forward record of how recurring situations are judged, so the three
ledgers treat the same situation the same way. Agent judgments, not rulings.

## Recurring situations

| Situation | Treatment |
|---|---|
| SOW frontmatter `decomposition_basis` pins `SOFTWARE_DECOMP.md@eaad463` (rev 0.8) | `<DEL>:SOW.s01` SUBCLAIM, CP-02 (STALE_REVIEW_OR_EVIDENCE · BASIS_POINTER_STALE · LOCAL_DESIGN · NONE · RECORD · NO). The SOW SURFACE row carries CP-04 when the surface has rename residue, so the pin needs its own sub-claim |
| SOW text naming "OpenPipeStress governance and technical documents" (CLM Standards block) | CP-04 on the SOW SURFACE row only; the Standards block is then judged on its substance |
| D-41 R5 T7 PDU-055 "current declaration" blocks (rev 0.8, DAG-007) | Own DECLARED_STATE row, `CanonicalSituation=CP-03`, CP-02 treatment (pins superseded). Delegation to `## Remaining` recorded as not relied on (A4) |
| Datasheet/Specification/Procedure/Guidance heading blocks (pre-typed NON_NORMATIVE) | NOT_ASSESSED, no override |
| Four-document kit named as current records or checks (Documentation, Acceptance Notes, Steps, setup Verification, Records, VER-...-001) | CP-01 STALE_SETUP_SPECIFICATION · REPRESENTATION_MIGRATED |
| Identification table with a `Current source basis` pin to rev 0.7 | Split into `.rNN`; the basis row CP-02, others judged on their own; parent CONTAINER |
| `docs/SPEC.md` section numbers in source columns (SOW says §7 GUI / §8 results-reporting; frozen SPEC has §7 rule-pack evaluator, §8 GUI, §9 reporting) | Pointer inaccuracy goes in Notes (C6(b)); disposition follows the requirement's substance |
| Output-matrix OUT-001 and praxeology VER-001 (parity/verification of the SOW) | CP-09: PASS records exist, none match the frozen SOW → STALE_REVIEW_OR_EVIDENCE · EVIDENCE_OVERTAKEN, citing the CHANGE-P2 parity record and claim map |
| Purpose-section OUT-001 (output definition) | Judged on substance: the SOW exists and covers the named topics → ALIGNED; unit-level drift recorded on the units |
| Open-issue / Conditions TBD rows | Accurate open TBD → ALIGNED (declared state accurate, no behaviour claimed). Settled in code with no ruling → CP-10. Settled by another deliverable's contract under an adopted plan → STALE_REVIEW_OR_EVIDENCE · DOC_BEHIND_CODE |
| Open-issue TBD whose owner column delegates the choice to a future implementation/GUI task that has landed (DEL-07-07 OI-07-07-00n) | STALE_REVIEW_OR_EVIDENCE · DOC_BEHIND_CODE, not CP-10 (no ruling was needed: the SOW delegated it) |
| Setup-era text that denies an implementation which has since landed (no four-document naming) | STALE_SETUP_SPECIFICATION · DOC_BEHIND_CODE |
| Requirement that is met in substance but carries an overtaken "exact … TBD" clause | ALIGNED on substance; the overtaken clause goes in Notes |
| Fixed placeholder counts shown in the product UI as contract evidence (DEL-07-08 core_contract_evidence) | IMPLEMENTED_DIFFERENTLY · POSSIBLE_DEFECT · REVIEW on the requirement and principle that forbid placeholder semantics |
| Pre-typed NON_NORMATIVE empty `STATUS#remaining` | Keep the pre-type; put the gap observation in Notes |
| Unimplemented feature named as owned residual | Unit about the residual alone → DOCUMENTED_UNIMPLEMENTED · NOT_STARTED · LOCAL_DESIGN · OWNER (a design choice is still open); unit about a slice that partly landed → PARTIALLY_IMPLEMENTED · PARTIAL_SLICE. One FindingGroup per residual |
| Remaining item accurate with an open action | Disposition describes the subject (C6(b)): DOCUMENTED_UNIMPLEMENTED/PARTIALLY_IMPLEMENTED; accuracy in Notes |
| Remaining item whose text disagrees with frozen code | REMAINING_STATE_MISMATCH · DOC_BEHIND_CODE (code moved) or RECORD_DRIFT (condition passed) |
| `_STATUS.md` Last Updated older than own history (CP-05) | On STATUS SURFACE row |
| `_CONTEXT.md` SURFACE | STALE_REVIEW_OR_EVIDENCE · BASIS_POINTER_STALE (CP-02): rev 0.7 pins and package-reference drift make the file stale as a whole declaration |
| CS-04 Architecture Basis Injection | Inherited on the pin; `.s01` for "PKG-00 at SEMANTIC_READY supplies ... constraints" (all PKG-00 deliverables are IN_PROGRESS at the freeze) → STALE_REVIEW_OR_EVIDENCE · RECORD_DRIFT · LOCAL_DESIGN · NONE · RECORD · NO. "Still TBD" items remain TBD in SOFTWARE_DECOMP rev 0.12 (DEC-012, OI-002, OI-006) → no sub-claim |
| "`_STATUS.md` is the sole work-discovery home" in current (non-history) SOW text | STALE_REVIEW_OR_EVIDENCE · RECORD_DRIFT: project `AGENTS.md` replaced deliverable-only work selection with the session work graph |
| MEMORY | HISTORY, ALIGNED when entries are dated and accurate |
| Deliverable-folder paths contain spaces | Cannot be evidence tokens (validator rejects spaces); cited in ContextRefs and Notes instead |
| Suite-level pass | `GATE:GATE_EVIDENCE/B4_4_SWEEP_9D55/SUMMARY.json` (desktop 1,621 and Python 1,138 PASS, not rerun); per-test status not asserted |

## Evidence landmarks

- Results viewer: `apps/desktop/src/features/results/ResultsPanel.tsx`; `GoverningRatioState` is always rendered with `ratioCount={0}` and the semantic contract `fixtures/results/semantic_contract_v0_2.json` has no ratio-family row (all `governing_ratio_eligible=false`), so a supplied ratio row shows under "Other" and the governing-ratio state is always unavailable (App.test.tsx asserts this). Deliberate since PR #787 (result integrity).
- Deformation overlay: `PipeViewport.tsx::buildDeformationOverlay` consumes `global_nodal_displacement_x/y/z` only; `global_nodal_rotation_*` rows are emitted by `core/product_physics/src/lib.rs` and listed in the results table, not visualized.
