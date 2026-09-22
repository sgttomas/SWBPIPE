# Worker notebook — DEL-13-02 rerun (W3 PKG-13, cycle 1)

These are recurring-situation treatments, kept for consistency. My only deliverable is DEL-13-02.

- **Unit-contract divergence in a schema.** The divergence is either a `Parameter` that lets a
  bare string stand in a quantity, or a dimension enum that has fallen behind SPEC §4. I treat
  it as PARTIALLY_IMPLEMENTED · POSSIBLE_DEFECT · INVARIANT · NONE · BASELINE · REVIEW, with
  one FindingGroup. This applies to requirement, verification and declared-state rows alike,
  whenever the row's own text claims unit-awareness or alignment with the vocabulary.
- **A verification check worded for all unit-bearing values** is judged on the whole value
  space, not only on `Quantity`.
- **Downstream "remain TBD" wording, once downstream slices exist.** Where that status is the
  row's main substance, the row is STALE_REVIEW_OR_EVIDENCE · DOC_BEHIND_CODE. Where the clause
  only trails the row, it is read as the scope boundary.
- **Revision, DAG or INIT.md pointers** are CP-02 on rows whose substance is the pointer or
  reference list. Elsewhere they go in a note.
- **Parity records that bind an earlier SOW hash** are CP-09 EVIDENCE_OVERTAKEN, found through
  the A3 discovery step even when `EVIDENCE_MAP` says `NONE_FOUND`.
- **A Remaining item with an open action but an inaccurate framing clause** is
  REMAINING_STATE_MISMATCH.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
