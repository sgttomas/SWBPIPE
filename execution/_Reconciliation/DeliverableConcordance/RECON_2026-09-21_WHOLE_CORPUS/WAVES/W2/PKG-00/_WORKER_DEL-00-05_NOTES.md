# Worker notebook — W2 PKG-00 worker G2 (DEL-00-05 to DEL-00-08)

Carry-forward record so the same situation gets the same treatment across the
four architecture-basis members. Agent judgments, not owner rulings.

## Recurring situations and how each was judged

| Situation | Treatment used in all four ledgers |
|---|---|
| ArchitectureBasis header "Authority basis: SOFTWARE_DECOMP revision 0.9" (L6) | CP-02: STALE_REVIEW_OR_EVIDENCE · BASIS_POINTER_STALE · LOCAL_DESIGN · NONE · RECORD · NO, finding group FG-<DEL>-01. Pin written 2026-07-15, so the F3 pin exception applies either way. Carried on the AB SURFACE row (00-05, 00-06) or on AB.s01 where the SURFACE row carries rename residue (00-07, 00-08) |
| Currency block "revision 0.9 with DAG-007" | Same CP-02 fields, same FG-<DEL>-01 |
| `_CONTEXT.md` banner "SOFTWARE_DECOMP revision 0.9" | CONTEXT SURFACE row, same CP-02 fields and FG-<DEL>-01 |
| "codified in SOFTWARE_DECOMP §8.4" (00-05, 00-06 only) | CP-02 on the resolved-decisions block, FG-<DEL>-02. Neither rev 0.9 nor rev 0.12 has a §8.4. 00-07/00-08 cite §12 correctly |
| Record cells citing "rev 0.9 §8.2" | ALIGNED: §8.2 exists with the same row in 0.9 and 0.12, so the provenance is accurate |
| REQ table "Basis" cells citing "rev 0.9" | Not a finding on the item: the cell records where the requirement came from, not a currency claim |
| Active identifiers carrying the former name (.opsproj, openpipestress-runner) | CP-04 on the AB SURFACE row; .opsproj fields (PROJECT_BASELINE, NONE, RECORD, OWNER) plus the DEC-101 persistence obligation. Items that name them judged on substance |
| OPS-K-* invariant IDs | Not rename residue: they are the CONTRACT.md identifiers |
| Carried OPS-K invariants (paragraph or keyed block) | ALIGNED MEDIUM, contrary-evidence check only; INVARIANT tier would apply if it diverged |
| Open hold settled in code with no ruling | CP-10 (state library, undo storage, severity taxonomy) |
| Hold overtaken by a later ruling | STALE_REVIEW_OR_EVIDENCE · SCOPE_REDIRECTED_BY_RULING · PROJECT_BASELINE · OWNER (accessibility vs D-68; export format list vs SCA-004) |
| "Residuals are those in _STATUS.md ## Remaining (currently empty)" | A4: ALIGNED; delegation clause recorded as not relied on |
| `STATUS#remaining` (empty, pre-typed NON_NORMATIVE, shared by 36) | Kept pre-typed: NON_NORMATIVE · NOT_ASSESSED |
| STATUS SURFACE (IN_PROGRESS, Last Updated 2026-07-12) | ALIGNED; D-43 kept _STATUS.md byte-identical, so no CP-05 |
| CONTEXT anticipated artifacts (setup-era doc names never created) | STALE_SETUP_SPECIFICATION · DOC_BEHIND_CODE · LOCAL_DESIGN · NO (origin 7bee9ae41) |
| CONTEXT description, envelope, register refs | ALIGNED (identical to register / decomposition row) |
| Keyed CS-01, CS-02, CS-05, CS-06, CS-07 rows | Inherited unchanged; no CANONICAL_DEPARTURE |
| Suite pass status | Cited as GATE records (B4_4 sweep, PR834 CI); "not rerun" in Notes |
| Block with children plus own prose, no .rNN | CONTAINER with .sNN sub-claims for the prose |
| Block with .rNN keys and own prose | Block assessed directly; split into all .rNN only when rows differ |

## Reverse-pass conventions

- PKG-00 members own no implementation (their Purpose blocks say so), so the
  positive answer for a capability that realizes an AB constraint is
  CONSTRAINS with the constraining key; CLAIMED_BY is not used.
- RC-00-0285 (architecture-basis validator): COVERS the AB SURFACE for 00-05,
  00-06, 00-07; PARTIAL on REQ-08-04 for 00-08 (the gate REQ-08-04 asks this
  member to define).
- RC-00-0173 (registers): COVERS CONTEXT#register-references for all four.
- Every capability whose EntryPoints hit a path the member's own forward
  ledger cites got a capability-specific NOT_MINE reason (F5); the generator
  refused to fall back to the area template for such rows.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
