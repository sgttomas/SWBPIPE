# Worker G2 carry-forward notebook — DEL-07-06, DEL-07-03, DEL-07-04

Run `RECON_2026-09-21_WHOLE_CORPUS`, wave W1, package PKG-07. TASK (Type 2)
worker; parent is the PKG-07 WORKING_ITEMS manager. Frozen state
`00115c71931bcae79909602d653740d3bb72dfa1`. These are agent judgments, not
owner rulings.

## Recurring situations and the treatment used

| Situation | Treatment |
|---|---|
| Setup-era text saying the deliverable is setup-only, "future" GUI, no source/tests changed | `STALE_SETUP_SPECIFICATION` · `DOC_BEHIND_CODE` · `LOCAL_DESIGN` · NONE · RECORD · NO; one FindingGroup per deliverable |
| A requirement or check phrased as "future tests shall ..." whose tests now exist | Rule C6(b): the subject is met, so `ALIGNED`; the tense drift goes in Notes |
| Unconditional "WCAG/contrast target remains TBD" (DEL-07-06) | Overtaken for touched desktop controls by D-68: `STALE_SETUP_SPECIFICATION` (setup-origin) or `STALE_REVIEW_OR_EVIDENCE` (D-41 or later declaration) · `SCOPE_REDIRECTED_BY_RULING` · `LOCAL_DESIGN` · NONE · RECORD · NO; `AdoptedByReference=YES` (APPROVED_PLAN criteria) |
| Conditional "until a human decision, keep TBD" | Condition met by D-68; not false, so `ALIGNED` with a note |
| Report/export or generated-file target TBD | Still accurate after D-68 (explicitly excluded); `ALIGNED` |
| Rows that declare a requirement's evidence state as `VERIFIED_NOT_VALIDATED` (PDU-045/046/049) | `VERIFIED_NOT_VALIDATED` · `VALIDATION_GAP` · `INVARIANT` · `OWNER_HOLD` · VALIDATION (plus SECURITY for PDU-049) · OWNER; D-68 keeps independent usability held |
| Guidance or procedure rows that instruct preserving those holds | `ALIGNED` (the instruction is followed) |
| D-41 "current declaration" blocks | `CanonicalSituation=CP-03`; with superseded revision pins, CP-02 fields; without pins, judged on accuracy; Remaining delegation not relied on (A4) |
| Four-document residue | CP-01 |
| Stale file or section pointers (missing `INIT.md`, missing `skills/*/SKILL.md`, rev 0.7 pins) | CP-02 |
| Remaining item whose condition passed (merge done, N7 rereview returned PASS, D-72 ruled the criteria) | CP-07; `RULED_CRITERION` only when a ruling overtook it (D-72), else NONE |
| Accurate Remaining item with a real open action | `ALIGNED` (A4); accurate with no open action: `ALIGNED` + `NO_OPEN_ACTION` (CP-06) |
| Output-matrix OUT-001 and VER-001 parity expectation | CP-09: `STALE_REVIEW_OR_EVIDENCE` · `EVIDENCE_OVERTAKEN` (no PASS parity matches the frozen SOW for any of the three) |
| Purpose-section OUT-001 bullet | Judged on substance (the output exists or not); CP-09 is applied to the matrix row |
| Rename residue | CP-04, once on the SOW SURFACE row where the SOW carries the former name |
| `_CONTEXT.md` SURFACE row | `STALE_REVIEW_OR_EVIDENCE` · `BASIS_POINTER_STALE` (rev 0.7 pins), MEDIUM |
| `_STATUS.md` Last Updated older than its own history | CP-05 on the STATUS SURFACE row |
| Declared-open state-library / component-library hold settled in code | CP-10 (applied in DEL-07-03). DEL-07-06 CLM-015 was sealed before this was settled and took `DOC_BEHIND_CODE`; see DEL-07-06 notes |
| DEL-07-03 R-005/R-006 (load-case, support editors) and rows whose subject is them | `ACCEPTED_DIVERGENCE` · `OWNERSHIP_ELSEWHERE` · `PROJECT_BASELINE` · NONE · RECORD · NO, DecisionBasis SCA-009 / DEC-094 |

## Conventions friction noticed

- Evidence columns reject spaces, but every deliverable-folder path in PKG-07
  contains spaces. Run records and deliverable files used as evidence are
  therefore cited in `ContextRefs` and named in Notes.
- `DivergenceLayers` has no layer for a plain implementation gap without a
  protected layer; `RECORD` is used.
- PRD section citations older than 2026-07-16 resolve to PRD v0.1 under the
  PRD's own forward-authority clause and are not treated as stale. SPEC
  "section 7" citations for warning classes point to the rule-pack evaluator;
  the warning-class table is SPEC section 8 (noted on SURFACE rows, not
  findings on each requirement).

## Boundary disclosure

During the reverse pass one listing of routing-file rows was written for
about a minute to the session scratchpad outside the run folder, then deleted
and regenerated as a `_scratch_*` file in this worker's DEL-07-06 folder. No
repository file outside the permitted folders was written. All `_scratch_*`
files were deleted before return.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
