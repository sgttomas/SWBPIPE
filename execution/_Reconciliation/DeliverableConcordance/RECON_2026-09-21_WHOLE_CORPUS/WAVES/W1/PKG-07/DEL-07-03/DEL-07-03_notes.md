# DEL-07-03 notes — Material, component, and rule-pack editors

Worker G2, wave W1, PKG-07. Frozen state `00115c71931bcae79909602d653740d3bb72dfa1`.
Forward ledger sealed before the routing file was read. These are agent
judgments, not owner rulings.

## Path aliases

- `SOW`, `STATUS`, `CONTEXT`, `MEMORY` are the files under
  `projects/chirality-piping/execution/PKG-07_Graphical User Interface and Engineering Workflow/1_Working/DEL-07-03_Material, component, and rule-pack editors/`.
- DEL-07-09 records (routing, coverage, `_STATUS.md`) are read from the frozen
  DEL-07-09 deliverable folder as evidence, not from any worker ledger.
- Parity records are root `execution/_Coordination/AgentRuns/SOW-STAGE2-EXEC-20260712-01/...` paths.
- Deliverable-folder run records (PDU-049 hold, TP-C2-EDITOR-001) sit in
  `ContextRefs` because their paths contain spaces.

## Judgment calls

- **SCA-009 re-point (FG-DEL-07-03-02).** DEC-094 re-points the R-005/R-006
  ownership landing to DEL-07-09 and changes no requirement. Rows whose
  subject is the load-case/support editors (R-002, R-005, R-006, CLM-005.r04/.r05,
  CLM-008 rows, CLM-017, CLM-018) are `ACCEPTED_DIVERGENCE` ·
  `OWNERSHIP_ELSEWHERE` · `PROJECT_BASELINE` with DecisionBasis SCA-009/DEC-094.
  R-002 and CLM-018 are MEDIUM: R-002 is met across the GUI only with that
  sanctioned split. Guidance and procedure rows that preserve the absence
  (CLM-027, CLM-041) are `ALIGNED`.
- **CP-10 state/component-library hold (FG-DEL-07-03-03).** CLM-003.r05,
  R-009, V-004 and CLM-035 declare the state-management/component library and
  versions TBD; code settles them (React built-in state and custom session
  contexts, no external state or component library, pinned package.json) and
  no ruling was found. SOFTWARE_DECOMP SOW-060 (DEL-00-05) still says "exact
  state library remains TBD". Disposed CP-10, OWNER, MEDIUM. R-009's
  durable/transient split itself is implemented.
- **Rule-pack authoring beyond "reference editing."** CLM-036.r05 is
  `IMPLEMENTED_UNDOCUMENTED` · `SCOPE_GREW_BY_DIRECTION`: the GUI authors
  private rule packs (declarations, check definitions, AST composer, Phase C2
  slices). SOW-021 covers rule-pack editors, so this is documentation lag, not
  scope creation.
- **CLM-005.r08** (editor validation surfaces the six SPEC classes) is
  `PARTIALLY_IMPLEMENTED`: editors use EDITOR_* codes and a blocking/advisory
  split; the six-class vocabulary appears in other panels.
- **PDU-049 (FG-DEL-07-03-01).** R-011, CLM-007/PDU-049 and CLM-016 are
  `VERIFIED_NOT_VALIDATED` with layers `VALIDATION;SECURITY` and `OWNER_HOLD`.
- **Remaining R03/R04** say the N7 final rereview is pending; the frozen
  `N7_FINAL_REVIEW/V2_BACKCHECK/REVIEW_RETURN_V2.md` records PASS and
  DEL-07-09 `_STATUS.md` records fan-in accepted → CP-07, `NONE`. R01 is
  `ALIGNED` with `NO_OPEN_ACTION` (CP-06). R02 cites DEC-074 O7, whose text is
  in an excluded July `PROPOSED_*` file; flagged, not read.
- **CLM-006 / CLM-022.r05** cite `INIT.md` and `skills/*/SKILL.md`, which do
  not exist at the freeze (repository or project root) → CP-02.
- **CLM-024** gives tool paths from the repository root over
  `execution/PKG-07_…`, which since the 2026-05-18 migration lives under
  `projects/chirality-piping/` → CP-02, MEDIUM.
- **V-007.** Dependencies.csv last changed in f37eedcae (DAG-009 refresh); that
  run's FAN_IN records all five refreshed registers passing v3.1 validation
  (not rerun). MEDIUM `ALIGNED`.
- **STATUS SURFACE** is `ALIGNED`: Last Updated equals its latest history
  date; stale items are on their own rows.

## Canonical departures

None written. CS rows inherit; pattern rows follow CP-01, CP-02, CP-03, CP-04,
CP-06, CP-07, CP-09 and CP-10.

## Convention friction

- Spaces in deliverable paths (see aliases).
- ACCEPTED_DIVERGENCE fits a scope amendment that re-points ownership but
  leaves the requirement text in this SOW; the remaining work is optional
  catch-up, recorded in RemainingWork.
- The R0 known gap (inspector/grid creation and table editing of materials
  and sections have no explicit DEL-07-03 key) surfaced in the reverse pass;
  answered PARTIAL against CLM-005 construction rows.

## UNKNOWN rows

None.

## Reverse pass

27 non-NOT_MINE answers: 6 CLAIMED_BY (editor contract, editor contract panel,
Library Manager import and drafting, Rule Pack Manager, checksum stamping),
10 PARTIAL (backend stores and validation, material temperature form,
inspector/grid/table material-section editing, component creation), 5
UNKEYED (rule-pack authoring editors and helpers; nearest key CLM-036.r05),
6 COVERS. Load-case, support and hanger capabilities are NOT_MINE with the
SCA-009 reason. The routing note that the Python editor contract has no
product-runtime caller is consistent with the forward rows, which rely on the
desktop panels for runtime behaviour. The reverse pass did not change my view
of any sealed row.

## Batch consistency

`validate_ledger_v2.py --batch` over DEL-07-06, DEL-07-03 and DEL-07-04:
PASS, 0 findings. See DEL-07-06 notes for the unflagged CLM-015 versus CP-10
point.

## Selectability

`SelectableUnderCurrentLoop` is `NOT_APPLICABLE` on every row (C9): Piping
selects work through owner-steered work graphs since 2026-09-19.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
