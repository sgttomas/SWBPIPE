# Worker G1 carry-forward notebook — W3 PKG-17 (DEL-17-01, DEL-17-02, DEL-17-03)

Running record of recurring judgments, so the same situation gets the same
treatment across the three ledgers.

1. **PLAN-EXPORT-INTEROP absent.** `plans/EXPORT_FORMAT_INTEROPERABILITY_PLAN.md`
   was deleted by `349a2ab33` (2026-06-03) with no archive copy; a domain copy
   was removed by `511f1e1ba` (2026-08-20). A row whose own text names the plan
   as a current source: CP-08 fields (RECORD_DRIFT · LOCAL_DESIGN · NONE ·
   RECORD · OWNER), disposition class by F3 origin of the row's own text
   (7bee9ae41 → STALE_SETUP_SPECIFICATION; later → STALE_REVIEW_OR_EVIDENCE).
   Rows that do not name the plan are judged on their own substance.
2. **Rename residue (CP-04).** Recorded once on the SOW SURFACE row;
   default variant (LOCAL_DESIGN · NONE · RECORD · OWNER).
3. **SOW frontmatter decomposition pin** (`@e8f59a63`, revision 0.8): `.s01`
   sub-claim on the SOW SURFACE key, CP-02 (STALE_REVIEW_OR_EVIDENCE ·
   BASIS_POINTER_STALE).
4. **Four-document residue (CP-01).** Acceptance bullets, consumption steps
   and validation commands that require Datasheet/Specification/Guidance/
   Procedure.md or `check_four_documents.sh`: REPRESENTATION_MIGRATED, class by
   F3 origin.
5. **D-41 R5 T7 current-declaration blocks (CP-03)** pinning rev 0.8 / DAG-007:
   CP-02 fields; Remaining delegation recorded as not relied on (A4).
6. **Architecture Basis Injection naming only SCA-003/SCA-004** (no revision
   pin, so no CS-04): CP-02 · BASIS_POINTER_STALE; SOFTWARE_DECOMP rev 0.12
   says the basis is amended through SCA-008.
7. **Open TBDs still carried open by downstream SOWs/code**: ALIGNED with
   `GAP_WORDING_CHECKED`. A TBD whose first-profile answer a downstream
   deliverable selected without the register recording it: STALE (F3 class) ·
   DOC_BEHIND_CODE.
8. External vendor URLs are not fetched; rows resting on them are MEDIUM.
9. DecisionBasis is filled on every row; VerificationClass NONE when
   VerificationEvidence is NOT_APPLICABLE.
