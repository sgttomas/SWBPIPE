# DEL-07-04 notes — Missing-data warning and blocking UX

Worker G2, wave W1, PKG-07. Frozen state `00115c71931bcae79909602d653740d3bb72dfa1`.
Forward ledger sealed before the routing file was read. These are agent
judgments, not owner rulings.

## Path aliases

- `SOW`, `STATUS`, `CONTEXT`, `MEMORY` are the files under
  `projects/chirality-piping/execution/PKG-07_Graphical User Interface and Engineering Workflow/1_Working/DEL-07-04_Missing-data warning and blocking UX/`.
- Parity records are root `execution/_Coordination/AgentRuns/SOW-STAGE2-EXEC-20260712-01/...` paths.
- Code tokens are project-root; the PDU-008 run record is in `ContextRefs`
  (path contains spaces).

## Judgment calls

- **Two warning vocabularies.** The desktop Missing Data Blocking panel types
  and inventories the six SPEC section 8 classes. The Python core contract
  keeps an older local vocabulary (solve_required, code_check_required, …)
  mapped to analysis statuses, accepted as-is in review finding
  PKG07-DEL0704-PKG02-002. R-001 is `ALIGNED` (MEDIUM) on the desktop GUI.
- **IP_BOUNDARY_WARNING.** The missing-data panel never activates it
  ("available_no_active_preview_item"); export redaction maps
  PROTECTED_CONTENT_BLOCKED to it and report lint carries it. R-009 is
  `ALIGNED` (MEDIUM, producers in other deliverables' surfaces). The
  verification rows CLM-013 and CLM-022 are `PARTIALLY_IMPLEMENTED`
  (FG-DEL-07-04-02): no frozen test activates IP_BOUNDARY_WARNING by name on
  export, report-preview or contribution paths.
- **Affected-object navigation.** Selecting a diagnostic resolves and selects
  its model entity (workspaceSession `handleSelectDiagnostic`); the
  Diagnostics panel shows affected refs. The rendered missing-data warning
  article omits affected refs (they are in its JSON). OUT-001, R-004 and R-013
  are `ALIGNED` at MEDIUM on that basis.
- **CLM-003** was not split: every row aligns and the "future UX" preamble is
  a C6(b) tense note. **CLM-004** was split because r06 (surface placement
  TBD, explicitly implementation-level) is documentation lag; CP-10 was not
  applied because the TBD was never a hold needing a ruling.
- **CLM-006** cites SPEC section 7 for the warning classes; the table is SPEC
  section 8 → CP-02.
- **SOW SURFACE** takes CP-02 for the revision 0.8 frontmatter pin; there is
  no former-name residue in this SOW.
- **Remaining R01** is an accurate responsibility boundary (producer-side
  nonlinear completeness) → `ALIGNED`, `NO_OPEN_ACTION`.
- **Observation for R3, not a row:** the implementing panel's export filename
  (`openpipestress-preview-missing-data-…`) and packet `document_kind`
  (`openpipestress.technical_preview.…`) are active code identifiers carrying
  the former name. The SOW does not name them, so CP-04 has no DEL-07-04
  surface row to sit on.

## Canonical departures

None written. CS rows inherit; pattern rows follow CP-01, CP-02, CP-03, CP-05
and CP-09.

## Convention friction

- Spaces in deliverable paths (see aliases).
- `DivergenceLayers` has no layer for an unprotected verification gap;
  `RECORD` used on the two PARTIALLY_IMPLEMENTED rows.

## UNKNOWN rows

None.

## Reverse pass

12 non-NOT_MINE answers: 2 CLAIMED_BY (missing-data panel, core warning
contract), 3 PARTIAL (Issues drawer, Diagnostics panel, diagnostic-to-entity
resolution), 7 COVERS (rule-check completeness, solve blocking producers and
panel, export classification, report lint, analysis-boundary schema, PKG-02
helpers). The routing note on the missing-data panel ("export packet embeds
hard-coded governance identifiers") matches the former-name observation
above. The reverse pass did not change my view of any sealed row.

## Batch consistency

`validate_ledger_v2.py --batch` over DEL-07-06, DEL-07-03 and DEL-07-04:
PASS, 0 findings.

## Selectability

`SelectableUnderCurrentLoop` is `NOT_APPLICABLE` on every row (C9): Piping
selects work through owner-steered work graphs since 2026-09-19.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
