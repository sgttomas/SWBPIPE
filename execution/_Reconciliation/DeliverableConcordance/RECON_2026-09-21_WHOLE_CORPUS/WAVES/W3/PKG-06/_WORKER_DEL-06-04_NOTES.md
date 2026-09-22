# Worker G2 carry-forward notebook (DEL-06-04, DEL-06-05)

These notes record recurring situations, so that each gets the same treatment
in both ledgers.

- **Setup-era text overtaken by landed code.** Where the text was first present
  at `7bee9ae41` (checked with `git grep -F` on that tree), the row is
  `STALE_SETUP_SPECIFICATION`. The cause is `DOC_BEHIND_CODE` when code
  advanced, `SCOPE_REDIRECTED_BY_RULING` when a DEC ruled the item (DEC-017,
  DEC-022, DEC-028), `REPRESENTATION_MIGRATED` for the four-document kit
  (CP-01), and `RECORD_DRIFT` for SEMANTIC_READY lifecycle gates.
- **INIT.md pointers.** INIT.md was removed by `9c4caf8fd`. These rows use CP-02
  with `STALE_SETUP_SPECIFICATION` and a `CANONICAL_DEPARTURE:` note (F3).
  They are recorded on References and Prerequisites rows only. Requirement
  rows whose source column cites INIT.md are judged on their substance.
- **D-41 T7 declarations (CP-03).** A declaration with no pin that is accurate
  is `ALIGNED` (DEL-06-04). A declaration pinning revision 0.8 or DAG-007 takes
  the CP-02 fields (DEL-06-05). In both cases the residual delegation is not
  relied on (A4).
- **CP-09.** Applies to VER-001 and the output-matrix OUT-001. The
  purpose-section OUT-001 is judged on its substance.
- **Architecture Basis Injection.** CS-04 covers the pin. `.s01` (PKG-00
  SEMANTIC_READY) is `STALE_REVIEW_OR_EVIDENCE · SCOPE_REDIRECTED_BY_RULING`.
  `.s02` (Still TBD: the rule grammar and container were ruled) is
  `STALE_SETUP_SPECIFICATION · SCOPE_REDIRECTED_BY_RULING`.
- **MEMORY with an undated "Remaining TBDs" or "Open Items" block now
  overtaken.** `STALE_SETUP_SPECIFICATION · DOC_BEHIND_CODE`.
- **Conditional guidance whose condition was met as written** (for example
  "TBD until X"). `ALIGNED`. A conditional instruction whose trigger can no
  longer arise and that still reads as live is stale setup text.
- **Engine support with the product caller passing empty data** (rule-pack
  report refs). `PARTIALLY_IMPLEMENTED · PARTIAL_SLICE` under F7.
- **F4 false hits on finding-code names** (`MISSING_*`). Use
  `GAP_WORDING_CHECKED`.
- **Dependency-register checks.** Cite the 2026-06-16 run record, whose
  validator result was VALID. The CSV has been unchanged since, so `ALIGNED`
  with "not rerun".

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
