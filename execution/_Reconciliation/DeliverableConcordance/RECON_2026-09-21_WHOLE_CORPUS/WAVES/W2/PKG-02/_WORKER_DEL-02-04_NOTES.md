# Worker G2 carry-forward notebook (DEL-02-04, DEL-02-05)

How I judged situations that recur across both deliverables:

- **D-41 R5 T7 "current declaration" blocks:** CP-03 → STALE_REVIEW_OR_EVIDENCE · BASIS_POINTER_STALE ·
  LOCAL_DESIGN · RECORD · NO (they pin rev 0.8 and DAG-007). The delegation clause is not relied on (A4).
- **SOW frontmatter `decomposition_basis@69ac259` (rev 0.8):** its own `SOW.s01` sub-claim, CP-02. The
  SOW SURFACE row carries CP-04 rename residue.
- **"Exact schema file layout / field names TBD" where a draft schema has landed:**
  STALE_SETUP_SPECIFICATION · DOC_BEHIND_CODE · LOCAL_DESIGN (FG-…-01 in each ledger). "PROPOSAL"
  labels on implemented-but-unruled names stay ALIGNED.
- **TBDs later decided by a ruling (formats: SCA-004/OI-004; container: DEC-017/DEC-028; migration:
  DEC-019/DEC-033):** STALE_SETUP_SPECIFICATION · SCOPE_REDIRECTED_BY_RULING (FG-…-02). Text first
  declared at `bc3b65aa9` (AC/VER/OUT lines) is STALE_REVIEW_OR_EVIDENCE (F3).
- **OUT-001 matrix rows and VER-001:** CP-09 → STALE_REVIEW_OR_EVIDENCE · EVIDENCE_OVERTAKEN, because
  the parity PASS hashes predate the D-48 Wave 2 SOW edit (`8fac6631a`).
- **CS-04:** pin row plus `.s01` (PKG-00 SEMANTIC_READY stale: all eight PKG-00 deliverables are
  IN_PROGRESS; RECORD_DRIFT) plus `.s02` (still-TBD list items since ruled).
- **CONTEXT SURFACE:** STALE_REVIEW_OR_EVIDENCE · BASIS_POINTER_STALE (CP-02) as a whole declared state.
- **Deliverable-local paths contain spaces:** cite them in ContextRefs only.
- **F7:** mark `PRODUCT_CALLER: NONE` on rows whose evidence is test-only code. Lesson from the reverse
  pass: apply it to every ALIGNED row citing such code, including declared-state rows.
