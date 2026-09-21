# Worker notebook — W2 PKG-03 worker G1 (DEL-03-01, DEL-03-02, DEL-03-03)

A running record of how recurring situations were judged, so the three
ledgers treat them the same way. Agent judgments, not owner rulings.

## Recurring situations

| Situation | Treatment used | Where |
|---|---|---|
| Schema `$id`/title and fixture provenance carry the former product name; SOW prose may too | CP-04 once on the SOW SURFACE row: STALE_REVIEW_OR_EVIDENCE · RENAME_OR_IDENTITY · LOCAL_DESIGN · NONE · RECORD · OWNER. Items judged on substance | all three SOW rows |
| Revision 0.7 or DAG-006 pin in SOW prose | CP-02: STALE_REVIEW_OR_EVIDENCE · BASIS_POINTER_STALE · LOCAL_DESIGN · NONE · RECORD · NO | 03-01 CLM-007, CLM-012.r02, CLM-018; 03-02 CLM-003.r08, CLM-007, CLM-012; 03-03 CLM-002.r07, CLM-006 |
| CONTEXT surface as a whole | CP-02 fields (it carries the CS-01/CS-04 pins) | all three |
| Architecture Basis Injection "PKG-00 at SEMANTIC_READY supplies dispatchable constraints" | `.s01` sub-claim: STALE_REVIEW_OR_EVIDENCE · SCOPE_REDIRECTED_BY_RULING · LOCAL_DESIGN · NONE · RECORD · NO (D-43, HUMAN-STEER-PKG00-EXCLUSION-001; all PKG-00 members IN_PROGRESS). Origin is the initial migration, but it is a lifecycle-state declaration, so the F3 review-state/metadata exception keeps SRE, matching CS-05 | all three |
| Four-document residue (CP-01) | Origin by `git log -S`: text first present at `7bee9ae41` gives STALE_SETUP_SPECIFICATION; later text gives STALE_REVIEW_OR_EVIDENCE; kit front matter (doc_id/status/created) is metadata, so SRE whatever its origin | 03-01 CLM-019.s01, CLM-020.r02; 03-02 CLM-001/008/015/022 (SRE), CLM-019 (SSS), CLM-020, CLM-021 (SRE); 03-03 CLM-011.s01, CLM-012 (SSS), CLM-018 (SRE) |
| PKG-02 review findings described as pending after the 2026-06-05 human Gate A/D disposition | STALE_REVIEW_OR_EVIDENCE · RECORD_DRIFT · LOCAL_DESIGN · NONE · RECORD · NO | 03-01 CLM-014, CLM-020.r08, CLM-021; 03-02 CLM-014; 03-03 CLM-011, CLM-011.s02 |
| Temperature interpolation policy described as TBD after DEC-077/DEC-092 | STALE_REVIEW_OR_EVIDENCE · SCOPE_REDIRECTED_BY_RULING · LOCAL_DESIGN · NONE · RECORD · NO; group FG-DEL-03-01-01 | 03-01 only |
| "Round-trip persistence" | Where the claim's own text records the round-trip as TBD (03-01 REQ-03-01-007) or asks it to be validated (VER-001 in 03-01 and 03-02): PARTIALLY_IMPLEMENTED · PARTIAL_SLICE · PROJECT_BASELINE (AB-00-04) · BASELINE. Where the requirement explicitly excludes round-trip from its own claim (03-02 REQ-09, REQ-12, CLM-013/REQ-12): ALIGNED with GAP_WORDING_CHECKED | 03-01, 03-02 |
| OUT-001 matrix row (CP-09) | 03-01 and 03-02: PASS parity matches the frozen SOW, ALIGNED. 03-03: parity overtaken by the D-48 Wave 2 claims-language edit, SRE · EVIDENCE_OVERTAKEN; VER-001 for 03-03 takes the same (its text has no persistence element) | all three |
| Mixed blocks | `.rNN` split whenever the block has issued `.rNN` keys and rows differ (block becomes CONTAINER). `.sNN` only when a block has two diverging parts with different causes; otherwise the block row carries its single gap and Notes name the part | see notes files |
| Suite-level test evidence | `GATE:GATE_EVIDENCE/B4_4_SWEEP_9D55/SUMMARY.json` (python surface PASS, not rerun); no per-test pass asserted | all rows with UNIT verification |
| Empty `## Remaining` | Pre-typed NON_NORMATIVE · NOT_ASSESSED (no F2 units exist) | all three |

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
