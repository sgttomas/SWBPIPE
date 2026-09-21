# Worker notebook — W2 PKG-00, DEL-00-01 to DEL-00-04

Running record of recurring situations so they get the same treatment.

1. **AB header pin (L6, "revision 0.9").** CP-02 (STALE_REVIEW_OR_EVIDENCE ·
   BASIS_POINTER_STALE · LOCAL_DESIGN · RECORD · NO). Where the AB SURFACE row
   is taken by CP-04 rename residue, the header pin goes on `<DEL>:AB.s01`;
   otherwise on the SURFACE row. Note that AB bytes are hash-bound in the
   package `CONSOLIDATION_MANIFEST.md` (validator-checked), so a repin also
   rehashes the manifest entry.
2. **Purpose / currency sentences pinning 0.9 and DAG-007.** CP-02 as above.
   Mixed-disposition blocks are split into `.sNN`; the block becomes
   CONTAINER / COVERED_BY_CHILDREN.
3. **CONTEXT SURFACE.** The D-43 preamble (2026-07-15) points readers to
   revision 0.9: CP-02 on the SURFACE row, same fields in all four.
4. **CP-04 rename residue.** `.opsproj` (DEL-00-01, DEL-00-02): PROJECT_BASELINE,
   NONE, RECORD, OWNER, persistence-compatibility note. `openpipestress-runner`
   (DEL-00-03): default CP-04 fields (LOCAL_DESIGN). The batch check will flag
   the tier difference; it is CP-04's defined variant split, not an error.
5. **REQ rows that restate an AB-00-0x row of SOFTWARE_DECOMP section 8.1**
   (SCA-001 accepted baseline): a gap takes PROJECT_BASELINE, BaselineClass
   NONE, AuthorityNeeded NO unless a choice is needed. Kit-only requirements
   (for example REQ-02-05 drift checks) take LOCAL_DESIGN.
6. **ADR process versus decision-log rows.** Only ADR-0001 exists; later
   architecture decisions live only in the decision log without ADR fields.
   PARTIALLY_IMPLEMENTED · PARTIAL_SLICE · PROJECT_BASELINE · OWNER wherever a
   member claims decisions are governed or recorded through the ADR surface.
7. **Open-TBD lists that include since-ruled items.** Text written 2026-07-15
   after the rulings: STALE_REVIEW_OR_EVIDENCE · RECORD_DRIFT. Text overtaken by
   a later ruling (after 2026-07-15): STALE_REVIEW_OR_EVIDENCE ·
   SCOPE_REDIRECTED_BY_RULING. Setup-era text (first present 7bee9ae41):
   STALE_SETUP_SPECIFICATION (F3).
8. **CONTEXT anticipated-artifacts naming a never-created doc** that the
   register (SCA-006 / D-43 update) now replaces: STALE_SETUP_SPECIFICATION ·
   SCOPE_REDIRECTED_BY_RULING · LOCAL_DESIGN · NO. Anticipated artifacts that
   exist: ALIGNED.
9. **STATUS SURFACE** with Last Updated equal to the latest history entry:
   ALIGNED. Empty pre-typed Remaining: NOT_ASSESSED. MEMORY dated refresh
   entries: HISTORY · ALIGNED.
10. **D-41-era "current declaration" sentences inside Purpose** (DEL-00-03,
    DEL-00-04): CP-03, own `.sNN` row; disposition follows the subject; a
    delegation to an empty `## Remaining` is recorded as not relied on (A4).
