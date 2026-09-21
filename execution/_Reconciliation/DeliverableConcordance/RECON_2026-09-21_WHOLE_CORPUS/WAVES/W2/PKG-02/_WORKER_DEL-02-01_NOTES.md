# Worker carry-forward notebook — W2 PKG-02 (DEL-02-01, DEL-02-02, DEL-02-03)

How recurring situations were judged, so the three ledgers treat them the same.
Agent judgments, not owner rulings.

| Situation | Treatment |
|---|---|
| D-41 R5 T7 PDU-055 declaration blocks (four per SOW) | CP-03 own row; CP-02 pin (rev 0.8, DAG-007): STALE_REVIEW_OR_EVIDENCE · BASIS_POINTER_STALE · LOCAL_DESIGN · NONE · RECORD · NO. Remaining delegation not relied on (A4) |
| Identification table with a rev 0.7 drafting basis | DECLARED_STATE; CP-02 pin profile; not split |
| References list pinning rev 0.7 | CP-02 pin profile |
| Setup-era "future implementation / not implemented / outside write scope" text (git log -S first commit 7bee9ae41) | F3: STALE_SETUP_SPECIFICATION · DOC_BEHIND_CODE · LOCAL_DESIGN · NONE · RECORD · NO |
| Setup-era TBD since ruled (container DEC-017/DEC-028, migration DEC-019, grammar DEC-022) | STALE_SETUP_SPECIFICATION · SCOPE_REDIRECTED_BY_RULING · LOCAL_DESIGN · NONE · RECORD · NO |
| Four-document residue (Datasheet/Specification/Guidance/Procedure named as current) | CP-01, origin 7bee9ae41: STALE_SETUP_SPECIFICATION · REPRESENTATION_MIGRATED |
| Former product name in SOW text | CP-04 once on SOW SURFACE row: STALE_REVIEW_OR_EVIDENCE · RENAME_OR_IDENTITY · LOCAL_DESIGN · NONE · RECORD · OWNER (class kept per CP-04; the residue arises from the 2026-09-18 ruling) |
| Schema `$id` on openpipestress.org | Active-identifier rename residue (DEC-101 identity act); default CP-04 fields on the sub-claim naming the `$id` |
| Declared-open TBD settled in code with no ruling | CP-10: IMPLEMENTED_DIFFERENTLY · AUTHORITY_UNCLEAR · PROJECT_BASELINE · NONE · RECORD · OWNER |
| VER-001 and matrix OUT-001 | CP-09: PASS parity exists, none matches the frozen SOW: STALE_REVIEW_OR_EVIDENCE · EVIDENCE_OVERTAKEN · LOCAL_DESIGN · NONE · RECORD · NO; cite SOW-PACKAGE-BATCH-ADOPTION parity-2.md |
| Purpose OUT-001 | Judged on substance (output exists) |
| STATUS surface with Last Updated older than history | CP-05 |
| CONTEXT surface | CP-02 pin profile (pins in CS-01/CS-04 blocks) |
| Architecture basis injection | CS-04 pin; .s01 PKG-00 SEMANTIC_READY (all PKG-00 IN_PROGRESS): STALE_REVIEW_OR_EVIDENCE · RECORD_DRIFT (lifecycle state, F3 exception); .s02 Still-TBD items since ruled: STALE_SETUP_SPECIFICATION · SCOPE_REDIRECTED_BY_RULING |
| Conflict row resolved by later register refresh | STALE_SETUP_SPECIFICATION · SCOPE_REDIRECTED_BY_RULING |
| Conflict row about revision pointers | CP-02 pin profile |
| Headings, section wrappers, empty Remaining | NOT_ASSESSED (pre-typed) |
| Tables fully carried by their item rows | CONTAINER · COVERED_BY_CHILDREN |
| Gate evidence | `GATE:GATE_EVIDENCE/B4_4_SWEEP_9D55/SUMMARY.json` (suite pass, not rerun) |
| Paths with spaces (deliverable folders, PKG-00 status) | Never in evidence columns; alias `DEL02-0N/` in NormativeSource; spaced paths only in ContextRefs |
| Adapter/plugin no-bypass claims with no traced product path | UNKNOWN · EVIDENCE_NOT_LOCATED · PROJECT_BASELINE · NONE · BASELINE (DEL-02-01 REQ-10, DEL-02-02 U-002, DEL-02-03 CLM-010.r10) |
| Python persistence hash basis (SORTED_COMPACT_JSON, non-JCS) vs AB-00-04/DEC-010/DEC-017 | Finding where the claim restates it (DEL-02-02 FG-DEL-02-02-01: IMPLEMENTED_DIFFERENTLY · AUTHORITY_UNCLEAR · PROJECT_BASELINE · FROZEN_CONTRACT · RECORD;BASELINE · OWNER); DEL-02-01 REQ-09 judged on schema compatibility |
| Remaining item accurate with open action carried by a governing row | ALIGNED + `OPEN_ACTION: <full ClaimKey>` (F2) |
| Reverse pass | Specific NOT_MINE reason for every capability whose entry points hit a forward-cited path (F5) |
