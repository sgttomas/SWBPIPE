# Worker notebook — W3 PKG-06 worker G1 (DEL-06-01, DEL-06-02, DEL-06-03)

Running record of recurring judgments, so the same situation gets the same
treatment across the three ledgers. Agent judgments, not owner rulings.

1. **D-41 PDU current declarations** (CLM-002/009/016/023 and siblings): CP-03
   row, fields per CP-02 (STALE_REVIEW_OR_EVIDENCE · BASIS_POINTER_STALE ·
   LOCAL_DESIGN · NONE · RECORD · NO); delegation to `## Remaining` not relied
   on (A4); blanket supersession does not move siblings. One FindingGroup per
   deliverable for decomposition pins, shared with CLM-007, CLM-012 pin rows
   and the SOW frontmatter pin (SOW.s01, rev 0.8 commit eaad463).
2. **Setup-era "future / setup-only" framing** (identification evidence status,
   Scope, Procedure Purpose): STALE_SETUP_SPECIFICATION · DOC_BEHIND_CODE ·
   LOCAL_DESIGN (F3 origin at 7bee9ae41 checked with git log -S). REQ rows
   saying "the future X shall" are judged on substance (C6(b)); the framing is
   judged on the Scope row.
3. **Grammar/library TBD** unconditional statements: STALE_SETUP_SPECIFICATION ·
   SCOPE_REDIRECTED_BY_RULING (DEC-022, DEC-037); conditional statements
   ("unless later ruled") stay ALIGNED. AC-001 text written at SOW migration
   (2026-07-14, after DEC-022) is STALE_REVIEW_OR_EVIDENCE with the same cause.
   The decomposition's own OI-006 row still reads TBD at rev 0.12 (for R3).
4. **Implementation-level TBDs settled in code** (file layout, JCS library):
   STALE_SETUP_SPECIFICATION · DOC_BEHIND_CODE, not CP-10, because the
   architecture basis leaves them to implementation.
5. **Protected-content review not located**: no DEC-058 scan record exists
   (`validation/evidence/releases/` absent at the freeze) and no reviewer
   disposition of the public schema/demo was found. Rows that assert or require
   that review: UNKNOWN · EVIDENCE_NOT_LOCATED · INVARIANT · IP_DATA · REVIEW,
   one FindingGroup per deliverable.
6. **Verification-approach columns in requirement tables** are part of the
   row: met when a located test exercises the named case anywhere reachable
   (schema tests, lifecycle, completeness, runner, evaluator).
7. **Surface rows**: SOW → CP-04 default variant when the text names the former
   product; CONTEXT → STALE_SETUP_SPECIFICATION · BASIS_POINTER_STALE (setup-era
   basis common to the surface); STATUS → CP-05 when Last Updated predates its
   history; MEMORY → HISTORY ALIGNED, with undated Open Items as a `.s01`
   DECLARED_STATE sub-claim.
8. **Architecture basis injection**: CS-04 pin row; `.s01` PKG-00
   SEMANTIC_READY → STALE_REVIEW_OR_EVIDENCE · SCOPE_REDIRECTED_BY_RULING
   (Agent 0 reading; SR-1 cluster); `.s02` Still TBD items since ruled →
   STALE_SETUP_SPECIFICATION · SCOPE_REDIRECTED_BY_RULING.
9. **CP-09**: PASS parity records exist for all three, none matches the frozen
   SOW (DEC-081 Wave 2 edit 8fac6631a came after parity) → matrix OUT-001 and
   VER-001 take CP-09 overtaken fields; the purpose OUT-001 is judged on
   substance.
10. **Product callers**: desktop `validate_rule_pack` → rule_pack_document;
    `run_rule_checks` → rule_check_runner → completeness_checker and
    expression_evaluator. These engines have product callers (F7 marker not
    needed for them).
