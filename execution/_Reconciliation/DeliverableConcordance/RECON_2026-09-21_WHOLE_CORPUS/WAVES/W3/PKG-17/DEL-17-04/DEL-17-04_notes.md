# DEL-17-04 — W3 notes (PKG-17, worker G2)

Forward ledger sealed at SHA-256
`6d3b3906ecf4eccc8d1f3d04de7937262d72f4bd13b577ed408467b4362ac68d`
(97 rows: 76 required keys, the CLM-005 and CLM-011 `.rNN` splits, and
`SOW.s01`). Dispositions are agent judgments, not owner rulings.

## Path aliases

- `NormativeSource` uses `<deliverable>/ScopeOfWork.md` plus a line or claim
  reference; evidence columns always hold full tokens.
- Deliverable-local evidence (with spaces) is cited in full and resolves at the
  freeze.
- Gate evidence: `GATE:GATE_EVIDENCE/B4_4_SWEEP_9D55/SUMMARY.json`; the sweep
  log lists `tests/test_caepipe_mbf_export_package.py` among the passing Python
  files. No suite was rerun.

## Judgment calls

- **GUI exclusion versus the desktop panel (FG-DEL-17-04-02).** CLM-003 and
  CLM-008 say the foundation implements no GUI action; CLM-015 step 9 keeps GUI
  integration out of scope unless separately authorized. The mounted
  `CaepipeMbfExportPanel.tsx` (added 2026-06-08, SMOKE.md TP-MAC-50) builds a
  package with `deliverable_id` DEL-17-04. Rated IMPLEMENTED_DIFFERENTLY ·
  DOC_BEHIND_CODE at MEDIUM. No separate authorization record was located in the
  deliverable's records.
- **Missing-loss-report diagnostic appears unreachable (FG-DEL-17-04-04,
  POSSIBLE_DEFECT).** `_loss_report` substitutes a default blocking `tbd`
  entry when the caller passes an empty list, and the diagnostics then receive
  the non-empty list, so `MBF-LOSS-REPORT-MISSING` cannot fire from the builder.
  With subset and sidecar present and `loss_report=[]`, the validation status
  would read `boundary_checked`. The only test supplies all three gaps at once.
  Read from code, not rerun. Rows: REQ-009, CLM-011.r10, CLM-023 (CLM-023 also
  notes that no diagnostic checks every canonical identity against the sidecar
  map or a loss entry). For contrast, the DEL-17-06 builder checks the supplied
  list before adding its default.
- **Pass-through options (FG-DEL-17-04-03, CP-11).** No option field exists in
  the closed ExportProfile schema or the builder, so REQ-006, CLM-005.r10 and the
  Context Envelope note hold only by absence (DOCUMENTED_UNIMPLEMENTED ·
  NOT_STARTED). AC-001 is PARTIALLY_IMPLEMENTED for the same gap and because
  line-ending and encoding questions are not carried in CLM-022 (the code fixes
  ASCII and LF through `canonical_text`).
- **Writer materialization.** Since the 2026-07-22 redaction tranche,
  `write_caepipe_mbf_export_package` routes through `control_route_export`
  (REXC-CORE-006, lossless required) and the test now asserts that the invented
  package is withheld. No SOW claim asserts file materialization, so no row
  changed; noted for the verifier.
- **Python foundation has no product caller.** ALIGNED rows citing only the
  Python builder carry `PRODUCT_CALLER: NONE` (F7). The claims they back are
  about the foundation itself.
- **Architecture Basis Injection** has no revision pin (no CS-04). ALIGNED at
  MEDIUM: the basis content matches DEC-009 and SCA-006 only changed
  representation. The Python-versus-Rust-core question is recorded as a
  project-level observation (the DEL-17-05 R0 pilot rated this UNKNOWN).
- **Frontmatter pin.** `SOW.s01` records the `e8f59a633` (revision 0.8)
  decomposition pin (CP-02).
- **OUT-001** follows CP-09: the CHANGE-P4 parity hash matches the frozen SOW.

## Canonical departures

None. CS-01, CS-03 and CS-06 rows inherit their assigned values.

## Convention friction

- The notes-gap scan flags diagnostic code names such as
  `MBF-SOURCE-BASIS-REFS-MISSING`; five ALIGNED rows carry
  `GAP_WORDING_CHECKED` for that reason alone.
- DivergenceLayers has no layer for an implementation gap with no protected
  layer; such rows use `RECORD`.

## UNKNOWN rows

None.

## Reverse pass

Five of 320 capabilities answered other than NOT_MINE: RC-17-0025, -0105 and
-0169 CLAIMED_BY; RC-17-0061 (desktop .mbf panel) PARTIAL because the SOW
excludes GUI behaviour; RC-17-0218 COVERS through CLM-012 (downstream consumer).
F5-specific reasons are given for RC-17-0108 and RC-17-0240, whose paths the
forward ledger cites as context. The reverse pass did not change my view of any
sealed row.

## Batch consistency

`--batch` over the three G2 forward ledgers: PASS, 0 findings.

## Selectability

`SelectableUnderCurrentLoop` is `NOT_APPLICABLE` on every row (C9): since
2026-09-19 Piping selects work through owner-steered work graphs, not
`## Remaining`.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
