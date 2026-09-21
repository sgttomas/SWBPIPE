# Worker carry-forward notes — W2 PKG-04, DEL-04-04 / DEL-04-05 / DEL-04-06

TASK worker (Type 2). Running record of how recurring situations are judged,
so the three ledgers treat them the same way. Agent judgments, not owner
rulings. Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).

## Recurring situations

1. **SOW frontmatter pin** (`decomposition_basis` @69ac259a71 = SOFTWARE_DECOMP
   revision 0.8). Recorded once on the SOW SURFACE row: CP-02,
   STALE_REVIEW_OR_EVIDENCE · BASIS_POINTER_STALE · LOCAL_DESIGN · NONE ·
   RECORD · NO.
2. **CONTEXT SURFACE** carries the 0.7 pins of CS-01/CS-04: same CP-02 fields.
3. **Section wrappers and heading-only CLM blocks**: NON_NORMATIVE /
   NOT_ASSESSED (pre-typed).
4. **Purpose-and-objective-traceability block**: assessed directly (it has its
   own traceability sentence), DECLARED_STATE · ALIGNED when the SOW/OBJ
   mapping matches SOFTWARE_DECOMP.
5. **CLM-002 identification table** with "Decomposition basis revision 0.7":
   DECLARED_STATE, CP-02 fields.
6. **CLM-006 references** citing revision 0.7: CP-02 fields.
7. **Four-document residue** (CLM Documentation / setup Verification /
   Records blocks listing Datasheet/Specification/Guidance/Procedure): CP-01,
   STALE_SETUP_SPECIFICATION (text present at 7bee9ae41) ·
   REPRESENTATION_MIGRATED · LOCAL_DESIGN · NONE · RECORD · NO.
8. **Setup-pass purpose / steps / guidance purpose / considerations** read as
   current and contradicted by implemented code, text present at 7bee9ae41:
   STALE_SETUP_SPECIFICATION · DOC_BEHIND_CODE · LOCAL_DESIGN · RECORD · NO.
   The D-41 "earlier setup-era statements retained as historical" clause does
   not change them (C1 blanket rule).
9. **Setup text overtaken by a named later ruling** (e.g. friction model TBD
   vs DEC-067): STALE_SETUP_SPECIFICATION if origin 7bee9ae41, else
   STALE_REVIEW_OR_EVIDENCE; cause SCOPE_REDIRECTED_BY_RULING; LOCAL_DESIGN;
   RECORD; NO (catch-up needs no decision, C3).
10. **Sparse live-path / default promotion text** ("remains governed/TBD")
    after DEC-050/DEC-053: SCOPE_REDIRECTED_BY_RULING, stale class by origin.
11. **D-41 PDU-054/055 current declarations**: CP-03, own DECLARED_STATE row;
    ALIGNED when the declared state holds. Duplicates carry DUPLICATE_OF and
    the same disposition.
12. **OUT-001 matrix row**: CP-09 per EVIDENCE_MAP. DEL-04-04 matches
    (ALIGNED); DEL-04-05 and DEL-04-06 PASS records do not match the frozen SOW
    (STALE_REVIEW_OR_EVIDENCE · EVIDENCE_OVERTAKEN · LOCAL_DESIGN · RECORD · NO).
13. **VER-001-type review claims** with parity done but formal review
    disposition pending: PARTIALLY_IMPLEMENTED · PARTIAL_SLICE ·
    LOCAL_DESIGN · RECORD · REVIEW.
14. **Requirements restating a CONTRACT OPS-K invariant** with an
    implementation or verification gap touching that invariant's subject:
    tier INVARIANT (F8). Layer: BASELINE for contract invariants outside the
    named subjects (units, mechanics boundary), VALIDATION for engineering
    validation, IP_DATA for IP, CLAIMS for the claims boundary; removed test →
    PROTECTED_CHECK · BASELINE · VERIFICATION_REMOVED. Stale record clause only
    → LOCAL_DESIGN · RECORD.
15. **Requirement whose own verification column names a test not located**
    while implementation holds: PARTIALLY_IMPLEMENTED (F1), not ALIGNED.
16. **Remaining items (F2)**: accurate + open + carried by a non-aligned
    governing row → ALIGNED with OPEN_ACTION. Accurate + open + no governing
    row: threshold-promotion items deferred by DEC-052 →
    PARTIALLY_IMPLEMENTED · DEFERRED_BY_RULING · PROJECT_BASELINE · BASELINE ·
    OWNER; external-validation items → INVARIANT · VALIDATION; history-model
    item gated on a future ruling → DOCUMENTED_UNIMPLEMENTED ·
    DEFERRED_BY_RULING · PROJECT_BASELINE · BASELINE · OWNER.
17. **Architecture Basis Injection** (CS-04): pin on the keyed row; `.s01` for
    the PKG-00 SEMANTIC_READY statement (all PKG-00 statuses IN_PROGRESS):
    STALE_REVIEW_OR_EVIDENCE · RECORD_DRIFT · LOCAL_DESIGN · RECORD · NO;
    `.s02` for the still-TBD list where the solver numerical library, lock
    files and hosted CI exist: STALE_SETUP_SPECIFICATION · DOC_BEHIND_CODE ·
    LOCAL_DESIGN · RECORD · NO (MEDIUM).
18. **STATUS history, CONTEXT preparation notes, MEMORY**: HISTORY · ALIGNED
    when accurate as dated history.
19. **Evidence files inside the deliverable folder** have spaces in their
    path, which Part D evidence columns cannot hold; they are cited in
    ContextRefs and named in Notes.
20. **Gate evidence**: per-test pass status only from the B4_4 sweep log
    (GATE record), always "not rerun".
