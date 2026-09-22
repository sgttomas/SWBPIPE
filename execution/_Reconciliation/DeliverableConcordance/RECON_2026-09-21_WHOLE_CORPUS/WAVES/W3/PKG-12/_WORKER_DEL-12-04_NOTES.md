# Worker notebook — W3 PKG-12 G2 (DEL-12-04, DEL-12-05)

Running record of recurring judgments, so the same situation gets the same
treatment in both ledgers. Agent judgments only; not owner rulings.

1. **Engine with no product caller (F7).** `core/security/secret_private_library/`
   is imported only by its tests. Claims about the helper or its tests are
   `ALIGNED` with `PRODUCT_CALLER: NONE`; claims about product behaviour
   (registration, storage, export seams) are `PARTIALLY_IMPLEMENTED ·
   PARTIAL_SLICE`.
2. **Tier for implementation gaps (F8).** Restates IP/data or privacy
   invariant (IP_AND_DATA_BOUNDARY, OPS-K-IP/DATA) and the gap touches that
   subject → `INVARIANT` (`IP_DATA`). Restates PRD or architecture-basis
   documents → `PROJECT_BASELINE` (`SECURITY`). Local design or ASSUMPTION
   basis → `LOCAL_DESIGN`. Record-only gaps where the boundary holds →
   `LOCAL_DESIGN · RECORD`, boundary named in Notes.
3. **Behaviour implemented by another deliverable.** If the claim is fully met
   at product level by another deliverable's code → `ALIGNED`, owner named in
   Notes (reverse-pass context). If only partly → `PARTIALLY_IMPLEMENTED`,
   cause of the remaining gap (`PARTIAL_SLICE`), or `OWNERSHIP_ELSEWHERE` when
   the claim is about this deliverable's own evidence that actually lives
   elsewhere.
4. **Met by construction (CP-11).** Plugin denial and credential storage hold
   partly because the governed behaviour does not exist → never `ALIGNED`.
5. **Pointers (CP-02).** Frontmatter decomposition pin (rev 0.8 commit),
   PRD/SPEC section references (PRD renumbered to v0.4) → one `SOW.s01`
   sub-claim per SOW. `revision 0.7` and removed `INIT.md` references → CP-02
   on the block that carries them.
6. **Four-document residue (CP-01).** Origin at `7bee9ae41` confirmed by
   `git log -S` → `STALE_SETUP_SPECIFICATION · REPRESENTATION_MIGRATED`.
7. **Settled TBDs.** "Physical project package/container TBD" was settled by
   SCA-003 → `STALE_SETUP_SPECIFICATION · SCOPE_REDIRECTED_BY_RULING ·
   LOCAL_DESIGN · RECORD` where the text asserts it is still TBD. Statements
   that only say "not resolved by this spec", "out of scope" or "deferred to
   owning workflows" stay accurate.
8. **Readiness states (F3).** "SEMANTIC_READY" declarations (own or PKG-00) →
   `STALE_REVIEW_OR_EVIDENCE · RECORD_DRIFT`, whatever the origin.
9. **Conflict-table rows awaiting a human ruling where governing sources are
   silent** → `UNKNOWN · AUTHORITY_UNCLEAR`, `AuthorityNeeded OWNER`.
10. **Parity (CP-09).** PASS records exist for both deliverables but none
    binds the frozen SOW (changed 2026-07-17) → `STALE_REVIEW_OR_EVIDENCE ·
    EVIDENCE_OVERTAKEN` on `output…/OUT-001` and on `VER-001`.
11. **CONTEXT SURFACE.** File-level rev 0.7 basis → CP-02. Architecture Basis
    Injection: CS-04 pin + `.s01` (PKG-00 SEMANTIC_READY statement) + `.s02`
    (Still TBD package/container, settled by SCA-003).
12. **Gate evidence.** Suite-level B4.4 sweep cited as `GATE:`; per-test pass
    counts only from run records; "not rerun".
