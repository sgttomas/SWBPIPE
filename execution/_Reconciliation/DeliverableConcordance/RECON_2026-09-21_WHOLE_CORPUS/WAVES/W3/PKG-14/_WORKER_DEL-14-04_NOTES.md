# Worker notebook — W3 PKG-14 G2 (DEL-14-04, DEL-14-05)

Running record of recurring judgments, so the same situation gets the same
treatment in both ledgers.

1. **Python engines with no product caller.** `core/comparison/analysis_run/engine.py`
   is reached only from tests (desktop Comparison panel is a separate TS preview;
   report projection is an intentionally empty parity fixture). Engine claims
   ALIGNED with `PRODUCT_CALLER: NONE` (F7). Claims about app/runtime behaviour
   would be PARTIALLY_IMPLEMENTED.
2. **D-41 "current declaration" blocks (CP-03).** Own DECLARED_STATE row; they pin
   SOFTWARE_DECOMP rev 0.8 and DAG-007 (frozen: 0.12, DAG-010) -> CP-02 fields
   (STALE_REVIEW_OR_EVIDENCE, BASIS_POINTER_STALE, LOCAL_DESIGN, NONE, RECORD, NO),
   CanonicalSituation CP-03. Delegation to `## Remaining` recorded as not relied on (A4).
3. **Revision/DAG pins in SOW text (rev 0.7, DAG-006/007, frontmatter @e8f59a63 = rev 0.8)** -> CP-02.
4. **Setup-era "TBD" status wording overtaken by code/tests**, first present at
   `7bee9ae41` by `git log -S` -> STALE_SETUP_SPECIFICATION, DOC_BEHIND_CODE,
   LOCAL_DESIGN, NONE, RECORD, NO (F3).
5. **Rows declaring a held gap** (PDU-011 output schema; PDU-047 validation) take
   the gap's disposition (C6(b), F1) and share a FindingGroup. Pure guard rows
   ("do not claim X until Y"; complied with) are ALIGNED with GAP_WORDING_CHECKED.
   - PDU-011 output schema: PARTIALLY_IMPLEMENTED, PARTIAL_SLICE, PROJECT_BASELINE,
     NONE, BASELINE, OWNER.
   - PDU-047 validation: VERIFIED_NOT_VALIDATED, VALIDATION_GAP, INVARIANT, NONE,
     VALIDATION, ENGINEERING.
   - OI-014 tolerance defaults / mapping workflows (open question rows):
     DOCUMENTED_UNIMPLEMENTED, DEFERRED_BY_RULING, PROJECT_BASELINE, NONE, RECORD, OWNER.
6. **Remaining items** that accurately record a held gap: ALIGNED with
   `OPEN_ACTION: <governing SOW key>` (F2).
7. **Rust-core architecture basis vs Python engine.** DEC-009 adopts Rust
   core/application services; the engine is Python under `core/`. No ruling
   located that permits Python domain engines (DEC-025 registers pytest as a
   gate surface only). -> IMPLEMENTED_DIFFERENTLY, AUTHORITY_UNCLEAR,
   PROJECT_BASELINE, NONE, RECORD;BASELINE, OWNER, MEDIUM.
8. **Entity-category coverage** (nodes, supports, terminals named in the claim):
   engine handles any `object_ref` generically, tests exercise only
   `PipeElement` refs -> PARTIALLY_IMPLEMENTED, PARTIAL_SLICE, LOCAL_DESIGN.
9. **Rename residue (CP-04)** only on keyed SURFACE rows that carry the former
   name; code-level residue in engine.py noted in notes, not keyed.
10. **Stale "TBD" riders.** A requirement whose substance is met but which
    carries a setup-era "TBD" rider (field names, command, columns) is ALIGNED
    with the rider noted (C6(b)). A status/condition row whose whole substance
    is the overtaken TBD is STALE_SETUP_SPECIFICATION (F3).
11. **Enum values settled by schema where the SOW required human approval or
    a source** (DEL-14-05 unmatched classifications) -> CP-10
    (IMPLEMENTED_DIFFERENTLY, AUTHORITY_UNCLEAR, PROJECT_BASELINE, NONE, RECORD,
    OWNER), MEDIUM.
12. **CP-05** applies to DEL-14-05 `_STATUS.md` (Last Updated 2026-07-12 older
    than the 2026-07-16 history entry); not to DEL-14-04 (dates agree).
13. **Four-document residue (CP-01)** in DEL-14-05 CLM-021 Records list:
    STALE_SETUP_SPECIFICATION, REPRESENTATION_MIGRATED (origin 7bee9ae41).
