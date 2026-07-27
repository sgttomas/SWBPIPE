# OD7-G1 Piping Candidate Handoff

Status: `READY_FOR_OWNER_GATE — CANDIDATE NOT APPLIED`

Validated candidates:

- C03 exact D-30 publication-SHA backfill;
- C04 exact `_LATEST` `0.6 → 0.10` plus the separately visible and separately
  owner-gated `SOFTWARE_DECOMP` frontmatter `0.9 → 0.10`; and
- C06 exact factual mismatch and non-client notice.

The combined Receipt-76 fragment is valid only if all three are approved and
its execution-time identity/cursor rescan passes. A subset or scan disagreement
requires a narrowed reissue.

The package passed publication/ancestry, patch, SCA-007 concordance, preserved
JSON hash, expected-bounded-failure, receipt-contract, structured-file,
path-containment, whitespace, and hash-manifest checks.

No live Piping surface changed. The remaining gate is explicit owner approval
of the final frozen candidate bytes.
