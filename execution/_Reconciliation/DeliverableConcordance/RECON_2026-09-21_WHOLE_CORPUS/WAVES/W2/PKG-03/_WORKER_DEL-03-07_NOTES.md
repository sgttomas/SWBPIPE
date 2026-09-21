# Worker G3 carry-forward notebook (DEL-03-07, DEL-03-08)

These notes record how recurring situations were judged, so the two ledgers
treat them the same way.

- Revision pins (0.7 in the body, 0.8 through the frontmatter commit
  `69ac259a`) use CP-02: `STALE_REVIEW_OR_EVIDENCE · BASIS_POINTER_STALE ·
  LOCAL_DESIGN · RECORD · NO`. A pin inside an unsplit block goes on `.s01`.
  In a split table it goes on its `.rNN` row.
- Review-finding "pending human disposition" text, after the 2026-06-05 Gate A
  `ACCEPT_AS_IS / RESOLVED`, is `STALE_REVIEW_OR_EVIDENCE · RECORD_DRIFT ·
  LOCAL_DESIGN`, with one FindingGroup per deliverable. The text was first
  declared on 2026-06-04, after the migration.
- A unit invariant gap (OPS-K-UNIT-1) is `PARTIALLY_IMPLEMENTED ·
  PARTIAL_SLICE · INVARIANT · BASELINE`, MEDIUM.
- Four-document residue uses CP-01, with the class set by the `git log -S`
  origin in the deliverable folder.
- For CS-04 blocks, add `.s01` for the PKG-00 `SEMANTIC_READY` statement and
  `.s02` for the "Still TBD" list. Both are `STALE_SETUP_SPECIFICATION ·
  SCOPE_REDIRECTED_BY_RULING · LOCAL_DESIGN`, with origin `7bee9ae41`.
- An open human-ruling TBD with no ruling located is `ALIGNED` as
  DECLARED_STATE.
- A Python engine with no product caller gets `PRODUCT_CALLER: NONE` on its
  `ALIGNED` rows. DEL-03-07 omitted the marker on Python-only rows because its
  Rust port has product callers; that deviation is disclosed in its notes.
- Deliverable-folder paths contain spaces, so they go in ContextRefs only.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
