# DEL-07-09 notes — W1 PKG-07 (worker G1)

TASK (Type 2) worker, run HELP-HUMAN-PIPING-20260921-RECONCILIATION. Frozen
state `00115c719`; code and documents were read only from the evidence
checkout. These are agent dispositions, not owner rulings.

- Forward ledger `DEL-07-09_forward.csv`: 111 rows. They cover 65 required
  keys; the 27 `.rNN` rows of the routing table and the 7 of the organization
  table; and 12 `.sNN` sub-claims (common route ×4, and ×4 in each copy of the
  N7 amendment). SHA-256
  `34fb9f7831ce9002dadb7879f44765af675b9b350bbe25ba91e7c430b56d6cc2`, sealed
  2026-09-21T20:45:01Z. Not edited after sealing.
- Validator v2: PASS (forward, and forward+reverse).
- Selectability: `SelectableUnderCurrentLoop` is `NOT_APPLICABLE` on every row
  (C9); Piping selects work through owner-steered work graphs.
- Lifecycle is OPEN, the corpus's only OPEN deliverable, and is recorded that
  way.

## Path aliases

- Deliverable files (the two CSVs, the two contract documents, _STATUS.md,
  _CONTEXT.md) are cited in ContextRefs only, because their path has spaces.
- `App.tsx::handleToolkitCommand`, `handleQueueOperationBatch`,
  `handleRunOperationBatch` and `handleUndo/RedoSessionModelEdit` are defined
  in `apps/desktop/src/features/workspace/workspaceSession.ts` since PR #803
  (merged 2026-09-19, stated no behaviour change). The undo branch of
  handleToolkitCommand already existed at `8468a33c`.
- `DisplayUnitSelector` renders in `features/workspace/shell/ShellToolbar.tsx`.
- `PRECOMMIT_PARENT_FAN_IN_V1.md` sits at the root of the toolkit-parity run.

## Judgment calls

- **R0 named repairs carried.**
  - `verification-and-handoff` → BaselineClass NONE.
  - The handleToolkitCommand capability (the R0 pilot's CAP-WS-025) → COVERS.
  - The mirrored CSV rows are collapsed: extractor v2 keys each vocabulary row
    once across both CSVs.
  - ROW-17 notes the direct/review Section table route (PR #834). ROW-1 and
    ROW-3 note the Node and Material tables.
- **Common defect on the ROWS SURFACE.** Every row's
  CurrentAcceptance/derivative-status cell says N7 is pending. It is recorded
  once, on the SURFACE row (C1). Items are judged on route, linkage and
  residual.
- **Relocated anchors → CP-02**: ROWS/ROW-9, 10 and 11; routing `.s01` and
  `.s04`. This replaces R0's DOC_BEHIND_CODE. Routing-table rows 9-11 name no
  file anchor, so they are ALIGNED.
- **Routing table split.** Only rows 2 and 16 carry "review pending" in their
  residual cell (EVIDENCE_OVERTAKEN). The other 25 rows are ALIGNED.
- **Organization block → UNKNOWN · AUTHORITY_UNCLEAR · PROJECT_BASELINE ·
  OWNER.** The block has `.rNN` keys, so its prose cannot take `.sNN` and is
  assessed as one unit.
  - DEC-094 makes DEL-07-09 the single palette-surface owner.
  - No annex row, register or ruling names where `ToolkitPalette.tsx` and
    `capabilityCatalog.ts` land, and `_CONTEXT.md` says implementation never
    lands here.
  - The stale anchor is recorded as secondary in Notes.
- **Remaining.**
  - R02 → CP-07 with RULED_CRITERION. D-70 made the reserved sequencing
    decision and D-72 ruled the criteria in part. D-70's table of revised
    records updates the corresponding wording for 07-01, 07-02 and 07-06 but
    not 07-09.
  - R03 → CP-07 with NONE.
  - R04 → UNKNOWN · EVIDENCE_NOT_LOCATED.
  - R08 → CP-06 with NO_OPEN_ACTION.
  - R09 → CP-08.
  - The STATUS SURFACE is not CP-05: Last Updated is later than every History
    entry.
- **ROW-23.** DEC-103 item 5 (LibraryKind::Hanger recordable here; BS-IP short
  variant on the hanger surface "at implementation") is not reflected. DEC-103
  executes only through an owner-authorized tranche, so this is noted as
  context and the row stays ALIGNED.
- **Context envelope.** "Editor implementation does not land here" is read as
  consistent with the decomposition's "landing zone for R-005/R-006": the annex
  routes the load-case editor surface to DEL-07-03 (rows 4 and 8), so the
  landing is ownership of coverage. MEDIUM confidence.

## Canonical departures

None. Five CS-06/CS-02 assignments are inherited unchanged.

## Convention friction

1. The org block's prose mixes an authority-silence finding with a stale
   anchor. Because the block has `.rNN` keys, C1 forbids `.sNN`, so one row
   carries both, and the anchor sits in Notes.
2. The duplicated N7 amendment block (DUPLICATE_OF) had to be split into
   mirrored `.sNN` rows to keep the COVERED_BY_CHILDREN pairing consistent
   with its original.

## Smallest next check for each UNKNOWN row

- `PALETTE_ORGANIZATION_CONTRACT#organization-and-ownership`: at R3, see
  whether any deliverable claims the palette component and catalog. My reverse
  answers are PARTIAL on RC-07-0225 and RC-07-0366. If no deliverable answers
  CLAIMED_BY, the owner rules the landing (DEL-07-09, 07-01 or 07-02).
- `STATUS#remaining/R04`: locate a native re-observation of short-panel
  scrolling and the inspector AX omission on the post-SWBPIPE-B shell.

## Reverse pass and its effect on the sealed ledger

Answers: 3 PARTIAL, 54 COVERS, 430 NOT_MINE.

- The COVERS answers are the vocabulary-row routes and linkages that the
  coverage ledger records while implementation lands elsewhere. They are
  consistent with DEL-07-09 owning coverage only.
- PARTIAL on the catalog, the palette and the shell commands listed in the
  palette: DEL-07-09 owns the organization, while the code landing is open.
  This confirms, and does not resolve, the sealed UNKNOWN.
- No sealed disposition would change.

## Batch consistency

`--batch` over the three forward ledgers: PASS, 0 findings. The shared
Remaining bodies (R03 = 07-02 R03 = 07-01 R04; R04 = 07-02 R04 = 07-01 R05;
R09 = 07-02 R11) carry identical fields.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081). No
release, approval, compliance or certification claim is stated or implied.
