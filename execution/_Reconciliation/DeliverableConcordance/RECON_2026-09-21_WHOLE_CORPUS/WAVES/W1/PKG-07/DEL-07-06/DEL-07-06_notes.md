# DEL-07-06 notes — Accessibility and usability baseline

Worker G2, wave W1, PKG-07. Frozen state `00115c71931bcae79909602d653740d3bb72dfa1`.
Forward ledger sealed before the routing file was read. These are agent
judgments, not owner rulings.

## Path aliases

- `SOW`, `STATUS`, `CONTEXT`, `MEMORY` are the deliverable's `ScopeOfWork.md`,
  `_STATUS.md`, `_CONTEXT.md` and `MEMORY.md` under
  `projects/chirality-piping/execution/PKG-07_Graphical User Interface and Engineering Workflow/1_Working/DEL-07-06_Accessibility and usability baseline/`.
- Parity records are root paths `execution/_Coordination/AgentRuns/SOW-STAGE2-EXEC-20260712-01/...`
  (no project copy exists).
- Code tokens are project-root (`core/…`, `apps/…`, `tests/…`); project
  documents are cited as `projects/chirality-piping/docs/…`.
- Deliverable-folder paths contain spaces, which evidence columns reject. Run
  records used as evidence (PDU-045/046 run records, the 2026-06-16
  dependency-refresh record, the B1 design-token record) are therefore in
  `ContextRefs` and named in Notes.

## Judgment calls

- **D-68 bounded target.** D-68 adopted APPROVED_PLAN's WCAG 2.2 AA criteria
  for touched desktop controls. Unconditional "target TBD" text was disposed
  as overtaken (`SCOPE_REDIRECTED_BY_RULING`, LOCAL_DESIGN, NO, FG-DEL-07-06-02,
  `AdoptedByReference=YES`). Setup-origin text took `STALE_SETUP_SPECIFICATION`;
  D-41-era or later text took `STALE_REVIEW_OR_EVIDENCE`. Conditional text
  ("until a human decision") and report/export/generated-file TBDs stayed
  `ALIGNED`. The SOW's own "Current bounded target" block is a blanket
  supersession declaration (C1): its own `ALIGNED` row, never used to change
  siblings.
- **VERIFIED_NOT_VALIDATED.** Applied only where the SOW itself binds the row
  to the PDU-045/PDU-046 holds (CLM-008.r01/.r03, RQ-004, CLM-017, CLM-018,
  AC-001), with `OWNER_HOLD` because D-68 keeps independent usability held.
  Other requirement rows (RQ-001..003, 005..010) were judged on implementation
  and project verification; their usability validation is carried by the
  VNV group, not repeated.
- **Rule C6(b) for "future tests shall …".** Where the tests now exist
  (CLM-014 and most CLM-024 rows) the subject is met and the row is
  `ALIGNED`; the tense drift is a Notes item.
- **Desktop runtime evaluation.** CLM-008.r04 and CLM-017.s01 are stale:
  the core record still emits `not_performed`, but browser runtime
  accessibility checks (b3 e2e, dist contrast/target assertions) exist.
- **CLM-026** (hold check) is stale on its step 3 because the core baseline
  and its test still assert the product-wide target `TBD_by_human_project_authority`
  after D-68 (MEDIUM). This is a text/record lag, not a protected-check
  change: no check was removed or weakened.
- **Remaining R02** cites "prospective owner-ruled criteria" that D-72 ruled
  on 2026-09-18 → CP-07 with `RULED_CRITERION`. **R03**'s before-merge
  condition passed when PR #789 merged (2026-09-17T18:12Z, `gh pr view`) →
  CP-07 with `NONE` (overtaken by an event, not a ruling). R04 kept `ALIGNED`:
  nothing frozen resolves the transient AX omission or the scrolling limit.
- **OUT-001.** The purpose-section bullet was judged on substance (`ALIGNED`);
  CP-09 was applied to the matrix row and VER-001.
- **CLM-005, CLM-007, CLM-014, CLM-031** were not split because every row
  aligns; CLM-003, CLM-004, CLM-008 and CLM-024 were split because rows differ.

## Canonical departures

None written. CS rows inherit their assigned fields. Pattern rows follow
CP-01, CP-02 (via CP-03), CP-04, CP-05, CP-07 and CP-09 exactly.

**Possible inconsistency I would correct (sealed, not edited):** SOW#CLM-015
lists "component library, state library, …" as TBD and was sealed as
`STALE_SETUP_SPECIFICATION`/`DOC_BEHIND_CODE`, with a note that CP-10 was
considered. In DEL-07-03 (sealed later) the same kind of state-library hold
took CP-10 (`IMPLEMENTED_DIFFERENTLY`/`AUTHORITY_UNCLEAR`/`PROJECT_BASELINE`/OWNER),
following the canonical example "state library". On reflection CLM-015's
state/component-library clause fits CP-10 too; the rest of its TBD list
(checklist format, filenames, screenshot policy) is documentation lag. The
batch check does not flag it because the rows carry different
`CanonicalSituation` values. Left for the verifier or a fresh worker.

## Convention friction

- Spaces in deliverable paths block evidence-column citation (see aliases).
- No `DivergenceLayers` value for an unprotected implementation gap; `RECORD`
  used.
- PRD section citations resolve to PRD v0.1 under the PRD forward-authority
  clause (not treated as stale). "SPEC section 7" for warning classes points
  to the rule-pack evaluator; the table is SPEC section 8. Recorded once on
  the SOW SURFACE row, not per requirement.
- The SOW SURFACE row can carry only one disposition; CP-04 took it, and the
  stale frontmatter decomposition pin (rev 0.8) and parity drift are Notes.

## UNKNOWN rows

None.

## Reverse pass

19 non-NOT_MINE answers (4 CLAIMED_BY, 4 PARTIAL, 11 COVERS). The routing
notes that the desktop Accessibility Baseline panel builds its packet from
static in-code records and that the panel does not execute the Python
builder. That matches the forward treatment (panel and core are both project
verification; no conformance audit). The reverse pass did not change my view
of any sealed row other than the CLM-015 point above.

## Batch consistency

`validate_ledger_v2.py --batch` over DEL-07-06, DEL-07-03 and DEL-07-04:
PASS, 0 findings.

## Selectability

`SelectableUnderCurrentLoop` is `NOT_APPLICABLE` on every row (C9): since
2026-09-19 Piping selects work through owner-steered work graphs, not
`## Remaining`.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
