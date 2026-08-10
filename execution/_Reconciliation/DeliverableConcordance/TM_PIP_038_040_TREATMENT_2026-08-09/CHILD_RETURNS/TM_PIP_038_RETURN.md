# Agent 2 return — TM-PIP-038

## Structured return

- Status: `PASS`.
- Parent: `/root/reconciliation_tm038_040_treatment`.
- Child: `/root/reconciliation_tm038_040_treatment/tm038_discovery`.
- Frozen source state: `da40d7dc4192c9aa2f49e9438729179aae281b61`.
- Objective result: the factual defect is substantiated in all three exact
  target blobs. W3 discovery used `opus` pilots; the per-package fan-in used
  `fable` verifiers. The summaries conflate these roles.
- Preservation result: the three summaries are members of the accepted
  immutable discovery snapshot. Direct repair is prohibited by the frozen
  manager contract and method. No historical file was edited.
- Treatment recommendation:
  `ADOPT_WITH_PRESERVATION — CORRECTION BY NEW DERIVATIVE RECORD; NO HISTORICAL EDIT`.
- Exact evidence:
  `SOURCE_EVIDENCE/TM_PIP_038_DISCOVERY_AND_PRESERVATION.md`; SHA-256
  `8621cfcb1c599c4e935d3950675486ff47d5c791ea8d1eee2dae985ca9e11809`;
  9,318 bytes.
- Correction executed by this child: none. The evidence record preserves the
  exact 42-byte before segment, exact 51-byte corrected projection, and
  whole-file before/projected-after object and SHA-256 identities for all
  three targets.
- Register/evidence-outcome acts: none.
- Git mutations: none.
- Delegation: none.
- Changed paths: exactly this return and the source-evidence record above.
- Remaining manager action: issue the new derivative correction record and
  include this preservation result in the owner-facing treatment verdict and
  handoff. Do not characterize the projected after blobs as historical edits.
- Blockers: none to the derivative correction-record treatment; the
  preservation rule blocks only direct edits to the accepted summaries.
