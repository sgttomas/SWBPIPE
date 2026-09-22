# Worker notebook: DEL-09-04 and DEL-09-05 (W3, PKG-09)

This notebook records how recurring situations were judged, so that both
ledgers treat the same situation the same way.

| Situation | Treatment in both ledgers |
|---|---|
| D-41 PDU-055 current declaration (SOW copies, and the undated one in DEL-09-05 MEMORY) | CP-03 · STALE_REVIEW_OR_EVIDENCE · BASIS_POINTER_STALE · LOCAL_DESIGN · NONE · RECORD · NO. The Remaining delegation clause is not relied on. |
| Former product name in the SOW | CP-04 on the SOW SURFACE row only (default variant, OWNER). Items are judged on their substance. |
| Four-document names used as current surfaces, hooks or evidence pointers | CP-01 · STALE_SETUP_SPECIFICATION · REPRESENTATION_MIGRATED, even where the substance holds (F1). |
| Setup-origin section/file reference that is wrong or dead (INIT.md, SPEC section numbers, AGENTIC §5) | STALE_SETUP_SPECIFICATION · BASIS_POINTER_STALE · CP-02 with CANONICAL_DEPARTURE (F3 over CP-02). |
| Setup write-scope or no-repo-edit claims contradicted by later merged work | STALE_SETUP_SPECIFICATION · SCOPE_GREW_BY_DIRECTION · LOCAL_DESIGN · NONE · RECORD · NO. |
| Setup TBD lists overtaken by rulings | STALE_SETUP_SPECIFICATION · SCOPE_REDIRECTED_BY_RULING. |
| OUT-001 matrix row and VER-001 | CP-09 · STALE_REVIEW_OR_EVIDENCE · EVIDENCE_OVERTAKEN (no PASS record matches the frozen SOW). |
| OUT-001 purpose row | ALIGNED when the output exists (the parity currency sits on the CP-09 rows). |
| CONTEXT SURFACE | STALE_REVIEW_OR_EVIDENCE · BASIS_POINTER_STALE (CP-02), because of the 0.7 pins. |
| CONTEXT ABI "PKG-00 at SEMANTIC_READY" | `.s01` · STALE_REVIEW_OR_EVIDENCE · RECORD_DRIFT (all PKG-00 are IN_PROGRESS after D-40). |
| Section wrapper and heading-only blocks | NON_NORMATIVE / NOT_ASSESSED as pre-typed. Requirement tables and the Remaining header are CONTAINER. |
| Remaining item that is accurate but open, with no governing row carrying it | Gap disposition on the Remaining row itself (F2). |
| Remaining item whose subject was removed by a ruling | CP-07 REMAINING_STATE_MISMATCH with RULED_CRITERION. |
| Remaining item whose text disagrees with the frozen code | REMAINING_STATE_MISMATCH · DOC_BEHIND_CODE. |
| Implementation-doc drift that no key owns | Recorded in the per-deliverable notes for R3, not as rows. |

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
