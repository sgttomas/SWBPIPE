# Worker G1 notebook (DEL-11-01, DEL-11-02, DEL-11-03)

Recurring treatments applied the same way across the three ledgers:
- PDU-055 declarations: CP-03 row with CP-02 fields (STALE_REVIEW_OR_EVIDENCE · BASIS_POINTER_STALE · LOCAL_DESIGN · NONE · RECORD · NO).
- SOW frontmatter `decomposition_basis@4d153302` (rev 0.8): `SOW.s01`, CP-02.
- Rename: CP-04 once on the SOW surface row; items judged on substance.
- Four-document residue: CP-01, STALE_SETUP_SPECIFICATION (origin 7bee9ae41).
- INIT.md pointer in a reference/read-list row: STALE_SETUP_SPECIFICATION · BASIS_POINTER_STALE with CANONICAL_DEPARTURE (F3). In a source cell of a substantive row: judged on subject (C6(b)); DEL-11-01 CLM-005.r02 departs from this (noted).
- Output-matrix OUT-001 and VER-001 parity: CP-09 EVIDENCE_OVERTAKEN (no PASS record matches the frozen SOW for any of the three).
- Review records that predate the frozen bytes: STALE_REVIEW_OR_EVIDENCE · EVIDENCE_OVERTAKEN, LOCAL_DESIGN (F8) when worker read shows the boundary holds; content requirements stay ALIGNED when later edits are rename/removal only or no later edit exists.
- CONTEXT ABI: CS-04 pin + .s01 SEMANTIC_READY (STALE_REVIEW_OR_EVIDENCE · RECORD_DRIFT, SR-1 cluster) + .s02 still-TBD list (TBD_RULED).
- STATUS surface: CP-05; STATUS#history HISTORY ALIGNED; MEMORY HISTORY ALIGNED, undated current declarations as MEMORY.s01.
- Conditional "TBD until decided" wording: ALIGNED when decisions are stated; unconditional TBD status lists: TBD_RULED.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
