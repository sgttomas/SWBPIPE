# DEL-08-06 State, comparison, and handoff report sections — worker notes (W3, PKG-08, worker G2)

Forward ledger sealed: `DEL-08-06_SEAL.txt`. Evidence read from the freeze
checkout at `00115c719` only. Agent judgments, not owner rulings.

## Path aliases

- Python assembler: `core/reporting/state_comparison_handoff_sections/engine.py`
  (`build_state_comparison_handoff_report_sections`,
  `build_persisted_project_report_sections`).
- Desktop projection: `apps/desktop/src/features/report/stateComparisonHandoffSections.ts`,
  fed into `reportPackageRequest.ts` and the Rust report package
  (`state_comparison_handoff_sections_*.json` members). It landed with the
  DEL-08-01 report-package seam (d2d8975ef, 2026-07-23).

## Judgment calls

- **Two implementations (F7).** The Python engine has no product caller; the
  product report package uses the TypeScript projection, which builds
  state/run and comparison sections only (handoff sections always empty) and
  is checked against the engine only for an empty invented parity fixture.
  Engine-claim rows (R1, R12, CLM-006.r03, CLM-016, CLM-024, AC-001) are
  ALIGNED with `PRODUCT_CALLER: NONE`. The parity limit is carried on VER-001
  (PARTIALLY_IMPLEMENTED).
- **Remaining R01** → REMAINING_STATE_MISMATCH · DOC_BEHIND_CODE (MEDIUM):
  written 2026-07-12 ("the T4 seam reads canonical persisted run history
  only"); on 2026-07-23 the desktop report package began carrying state/run and
  comparison sections, so the text understates bound producers. Handoff,
  adapter and non-JSON partitioning items remain open.
- **Evidence-state columns "TBD"** (CLM-014 unsplit, CLM-022.r01–r07) →
  STALE_REVIEW_OR_EVIDENCE · DOC_BEHIND_CODE (evidence-state metadata, F3
  exception); tests now exist for each topic.
- **CLM-004.r09** (22 ACTIVE DAG-006 rows): register now holds 29 rows (28
  ACTIVE) → STALE_REVIEW_OR_EVIDENCE · RECORD_DRIFT.
- **CLM-007, CLM-023:** DAG-006 / revision 0.7 pointers → CP-02; CLM-023 also
  four-document residue → CP-01.
- **CP-04 on SOW SURFACE:** the SOW text does not carry the former name, but
  the deliverable's active code does (engine provenance `source_name` and
  `contributor`; desktop projection provenance strings emitted into report
  packages). Applied the CP-04 default variant on the SURFACE row as the
  deliverable-level residue record; a verifier may prefer to treat code-only
  residue outside the SOW row — flagged here.
- **Sub-claims:** `SOW.s01` (front matter pin), `CLM-021.s01` (step 10 TBD
  list), `CLM-029.s01` (code interfaces TBD),
  `CONTEXT#architecture-basis-injection.s01` (PKG-00 SEMANTIC_READY).
- **Tables split:** CLM-003, CLM-004, CLM-005, CLM-006, CLM-012, CLM-022.
  Unsplit: CLM-008, CLM-013, CLM-014, CLM-020, CLM-028, CLM-030, CLM-032.

## Canonical departures

None.

## Convention friction

- CP-04 wording ("the deliverable, or an active code identifier it names")
  does not say where code-only residue is recorded when the SOW neither
  carries nor names the identifier; recorded on the SOW SURFACE row.

## UNKNOWN rows

None.

## Reverse pass

Read the routing file only after all three seals. It confirmed the
desktop-projection observation already recorded (RC-08-0243 claimed). No
change to sealed views. Overlapping capabilities answered specifically (F5):
RC-08-0028, 0034, 0056, 0093, 0160, 0183, 0238, 0243, 0244, 0254, 0283.

## Batch consistency

`validate_ledger_v2.py --batch` over DEL-08-04/05/06: PASS, 0 findings.

## Selectability

`SelectableUnderCurrentLoop` is `NOT_APPLICABLE` on every row (C9).

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
