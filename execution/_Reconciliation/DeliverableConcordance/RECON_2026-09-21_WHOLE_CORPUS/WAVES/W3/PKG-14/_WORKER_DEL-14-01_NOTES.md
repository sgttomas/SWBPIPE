# Worker carry-forward notes — W3 PKG-14 (DEL-14-01, DEL-14-02, DEL-14-03)

Running record of how recurring situations are judged, so the three ledgers
treat them the same way.

1. **Table splitting.** Only the Requirements table (CLM-011 or equivalent) is
   split into `.rNN` rows. Other tables are assessed as blocks; where parts
   differ, the most substantive gap sets the disposition and the other gaps
   are named in Notes.
2. **JCS versus sorted-compact JSON.** Where a claim requires the
   JCS-compatible canonical JSON basis (DEC-010, AB-00-04, SPEC 3.2/4.4) and
   the code emits `SORTED_COMPACT_JSON` (relabelled as explicitly not RFC
   8785/JCS under the D-41 R5 T2A repair, DEC-074 E1), the row is
   `PARTIALLY_IMPLEMENTED · PARTIAL_SLICE · PROJECT_BASELINE · NONE ·
   BASELINE;RECORD · NO` in a per-deliverable FindingGroup. Rows that only
   name "deterministic" or "canonical JSON" hashes, without JCS, are judged
   on determinism.
3. **Product caller (F7).** The Python persistence service, record builders
   and comparison engines have no desktop caller for saving named model
   states. Requirements about product behaviour ("the product shall save")
   are `PARTIALLY_IMPLEMENTED · PARTIAL_SLICE · PROJECT_BASELINE ·
   BASELINE`; contract and engine claims are `ALIGNED` with
   `PRODUCT_CALLER: NONE`.
4. **D-41 R5 T7 PDU-055 current declarations** (four per SOW): CP-03 with CP-02
   fields (`STALE_REVIEW_OR_EVIDENCE · BASIS_POINTER_STALE · LOCAL_DESIGN ·
   NONE · RECORD · NO`).
5. **Revision 0.7 / DAG-006 pins** in References and Prerequisites: CP-02.
   Prerequisite upstream items are `ALIGNED` when DAG-010 keeps the edge.
6. **Setup TBDs overtaken by code** (first present at `7bee9ae41`):
   `STALE_SETUP_SPECIFICATION · DOC_BEHIND_CODE · LOCAL_DESIGN · NONE ·
   RECORD · NO`.
7. **Protected-content/private-data review record not located** for
   verification and records claims: `UNKNOWN · EVIDENCE_NOT_LOCATED ·
   INVARIANT · NONE · IP_DATA;RECORD · REVIEW`, following the W1 DEL-07-02
   CLM-026 resolution. The prohibition requirement itself is judged on
   inspection of the fixtures.
8. **Rename residue (CP-04)** on the SOW SURFACE row only, default variant;
   the schema `$id`/title carrying the former name is named there.
9. **Architecture Basis Injection (CS-04)**: `.s01` PKG-00 SEMANTIC_READY
   (STALE_REVIEW_OR_EVIDENCE · RECORD_DRIFT); `.s02` Still TBD items since
   ruled (STALE_SETUP_SPECIFICATION · SCOPE_REDIRECTED_BY_RULING); `.s03`
   Resolved Baseline JCS clause (the FG for JCS).
10. **CONTEXT SURFACE**: CP-02 common defect (revision 0.7 control surface).
11. **Output matrix OUT-001 and VER-001**: CP-09, EVIDENCE_OVERTAKEN (no PASS
    parity record matches the frozen SOW).
12. **MEMORY**: sections under a dated entry are that entry's history;
    ALIGNED with `GAP_WORDING_CHECKED` when they mention overtaken states.
14. **Verification column inside a requirement row** (DEL-14-03) is part of
    the claim. An unmet verification method makes the row non-aligned.
15. **Remaining items whose work no SOW row carries** (review dispositions)
    are `DOCUMENTED_UNIMPLEMENTED · NOT_STARTED` under F2.
16. **Contract advanced by a ruling adopted by reference** (D-67, analysis
    record 0.2): `STALE_REVIEW_OR_EVIDENCE · CONTRACT_VERSION_ADVANCED ·
    LOCAL_DESIGN`, with `AdoptedByReference=YES`.
13. **Phase G Remaining item**: ALIGNED with `OPEN_ACTION` to the product-save
    requirement row, because the deliverable's product slice has not landed.
