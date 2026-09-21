# DEL-07-09 — R0 calibration notes

Worker: TASK (Type 2), run HELP-HUMAN-PIPING-20260921-RECONCILIATION. Frozen
state `00115c71931bcae79909602d653740d3bb72dfa1`. Forward ledger sealed at
SHA-256 `2677b94117a00a2e036214577fd08dfc3b12ad6a51e58aa2da5d0d157af4174a`
(109 rows: 93 issued keys + 16 `.sNN` sub-claims). These are calibration
dispositions, not owner rulings.

## Path aliases

- The deliverable documents use project-relative paths (`apps/...`, `core/...`,
  `execution/...`). The ledger writes them repository-relative with the prefix
  `projects/chirality-piping/`.
- `App.tsx::handleToolkitCommand`, `handleQueueOperationBatch`,
  `handleRunOperationBatch`, `handleUndoSessionModelEdit` and
  `handleRedoSessionModelEdit` are now defined in
  `apps/desktop/src/features/workspace/workspaceSession.ts`. PR #803 (SWBPIPE B2
  stage 3) moved them and states no behaviour change. The ledger cites the new
  location as evidence and records the old anchor as stale.
- `DisplayUnitSelector` (default id `display-unit-preference`) now renders in
  `features/workspace/shell/ShellToolbar.tsx`, not in the viewport. The
  `view.units` route still resolves.
- `PRECOMMIT_PARENT_FAN_IN_V1.md` sits at the root of the toolkit-parity run, not
  under `N7_FINAL_REVIEW/V2_BACKCHECK/`. `_STATUS.md`'s phrase "in the same run"
  is accurate.
- Gate evidence is cited in `ContextRefs` as `GATE_EVIDENCE/...`, because those
  paths post-date the freeze and would fail the validator's frozen-path check.

## Judgment calls

- **Coverage items versus the uniform "N7 pending" field.** Every row in both
  CSVs has a CurrentAcceptance / derivative-status cell saying final N7 review is
  pending. The N7 V2 review has since returned PASS and the parent fan-in was
  accepted. I assessed item rows on their capability substance: route, focus
  target, change-token linkage and residual. I recorded the uniform staleness
  once, on each SURFACE row. The alternative, 54 identical STALE rows, would bury
  the real findings.
- **Anchor relocation.** Rows whose only defect is a relocated symbol with
  unchanged behaviour are marked STALE_REVIEW_OR_EVIDENCE with cause
  DOC_BEHIND_CODE: coverage rows 9–11 in both CSVs, routing `.s01`/`.s04`, and
  organization `.s03`.
- **Sub-claims.** I split blocks that carry several distinct claims:
  - the common executable route: 4 sub-claims;
  - organization and ownership: 4 sub-claims;
  - each N7 F1/F2/F3 amendment block: 4 sub-claims, so the verified repairs stay
    separate from the stale "rereview pending" status.
- **Phase A baseline columns** in `Capability_Comparison.csv` are explicitly
  labelled as historical. I did not assess them as current claims.
- **Remaining items (A4).** R02 is REMAINING_STATE_MISMATCH with cause
  SCOPE_REDIRECTED_BY_RULING. D-70's revised-decisions table rewrites this exact
  wording for DEL-07-01, DEL-07-02 and DEL-07-06 but leaves out DEL-07-09. R09
  cites `HISTORICAL_N7_ACCEPTANCE_INTAKE_V1.json`; `git log --all -S` shows it
  was never committed. DEL-07-02 cites the same missing file.
- **Row 23.** I marked it ALIGNED on substance and noted DEC-103 item 5 in
  `Notes`. That item says `LibraryKind::Hanger` is recordable in this ledger and
  that the hanger surface carries the BS-IP short variant "at implementation".
  The variant does not appear in `HangerSelectionPanel.tsx` at freeze. This is
  routed to the owning deliverables, not disposed of here.
- **Palette component ownership.** No governing source names where
  `ToolkitPalette.tsx` and `capabilityCatalog.ts` land. DEC-094 makes DEL-07-09
  the single palette-surface owner, and DEL-07-09's `_CONTEXT.md` says
  implementation never lands here. I recorded UNKNOWN with cause
  AUTHORITY_UNCLEAR and routed it to the owner.
- **Selectability.** `SelectableUnderCurrentLoop` is `NOT_APPLICABLE` on every
  row. Since 2026-09-19 Piping has selected work through owner-steered work
  graphs, not through `## Remaining` (C9 default).

## Convention friction

1. **No disposition for "governing sources are silent".** C7 has the cause
   AUTHORITY_UNCLEAR, but C6's only fits are AUTHORITY_CONFLICT (untrue here) and
   UNKNOWN (which implies missing evidence, not missing authority). *Fix:* add
   `AUTHORITY_SILENT`, or state that UNKNOWN combined with AUTHORITY_UNCLEAR
   means silence.
2. **No cause tag for a pure move with unchanged behaviour.** DOC_BEHIND_CODE
   says the code advanced, which overstates a no-behaviour-change move;
   REDESIGN_SUPERSEDED says behaviour was replaced, which is untrue.
   *Fix:* add `ANCHOR_RELOCATED`, or define DOC_BEHIND_CODE to include it.
3. **A field shared by every row.** C1 does not say whether a staleness shared
   by every row of a CSV surface is recorded per ITEM or once at SURFACE.
   *Fix:* one rule: "a defect common to all ITEM children is recorded on the
   SURFACE row; ITEM rows are judged on their own substance".
4. **STALE_REVIEW_OR_EVIDENCE versus REMAINING_STATE_MISMATCH** overlap for
   Remaining items that cite overtaken review states (R01, R03, R09). *Fix:* use
   REMAINING_STATE_MISMATCH for every `STATUS#remaining/*` divergence and keep
   STALE_REVIEW_OR_EVIDENCE for document bodies.
5. **Gate evidence is not citable in evidence columns.** The validator's path
   check rejects `GATE_EVIDENCE/` paths because they are outside the frozen tree.
   *Fix:* allow a `GATE:` token prefix that the validator skips, or add a
   `GateEvidence` column.
6. **Scope of AuthorityNeeded.** `NO` is overloaded: it covers "documents can
   catch up without a decision" and also "an owner decides at R4 anyway".
   *Fix:* add `DOC_UPDATE`.
7. **The reverse inventory lacks this deliverable's central capability.**
   `PILOT_CAPABILITIES.csv` has no row for the palette component or catalog
   (`ToolkitPalette.tsx`, `capabilityCatalog.ts`, `toolkitRoadmap`). The
   deliverable's one real code surface could not be tested for ownership in
   reverse. *Fix:* R3 inventory should include palette/catalog capabilities.
   Pilot sampling should include at least one capability drawn from each pilot
   deliverable's declared evidence paths.

## Unit-grain observations

- The claim-key grain is too fine for the two CSVs. Each has 28 row keys, and
  the two files carry near-identical current columns: the comparison is
  declared to be a projection of the same evidence. That produced 56 item rows
  with mirrored dispositions. One key per vocabulary row, covering both files,
  would halve the effort without losing coverage.
- The Markdown blocks are too coarse. Each N7 amendment block and the
  common-route block carries 3–4 independent claims, which needed sub-claims.
  Paragraph- or bullet-level keys would fit better there.
- The `_STATUS.md` Remaining grain (one key per item) was right.

## Smallest next check for each UNKNOWN row

- `PALETTE_ORGANIZATION_CONTRACT#organization-and-ownership.s01`: in the R3
  reverse pass, see which deliverable (if any) answers CLAIMED_BY for a
  palette/catalog capability. If none does, the owner rules the landing:
  DEL-07-09, DEL-07-01 or DEL-07-02.
- `STATUS#remaining/R04`: read the SWBPIPE Tranche B3 shell verification
  records (for example, the run behind commit `f5bc01f8b`) for a re-observation
  of short-panel scrolling and the inspector AX omission on the current shell.

## Effect of the reverse pass on the sealed ledger

- The reverse pass gave 1 CLAIMED_BY (CAP-WS-025, the toolkit command router),
  12 PARTIAL and 83 NOT_MINE answers. The PARTIALs are all the same kind: the
  coverage ledger records a vocabulary row whose implementation lands in another
  deliverable. That is consistent with DEL-07-09 owning coverage only.
- It confirmed the ownership gap rather than resolving it. No inventory row
  exposes the palette component, so UNKNOWN on `organization-and-ownership.s01`
  stands.
- CAP-WS-009 and CAP-WS-001 confirm the undocumented shell commands hosted in
  the palette, which the sealed SURFACE row already notes.
- CAP-TREE-006 and CAP-WS-030/032 show a direct-apply table editing route (node,
  material and section tables) that goes through the engine outside the palette.
  SOW-077's routing rule applies only to palette commands, so no sealed
  disposition changes. The coverage ledger's row 17 does not mention the new
  direct-apply table path for Section edits, however. The sealed ALIGNED
  disposition for `VOCABULARY_COVERAGE/ROW-17` still holds, but its Notes
  should have mentioned that newer surface.
- No sealed disposition would change.
