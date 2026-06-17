---
run_id: WORKING_ITEMS_RUN_2026-06-17_TP-UNITS-BTAIL-MATLIBFIELDUNITS-001
tranche_id: TP-UNITS-BTAIL-MATLIBFIELDUNITS-001
smoke_id: TP-MAC-197
agent: WORKING_ITEMS
primary_deliverable: DEL-02-02
supporting_deliverables:
  - DEL-07-03
  - DEL-03-01
status: passed
created: 2026-06-17
---

# TP-UNITS-BTAIL-MATLIBFIELDUNITS-001 — Supporting Unit Evidence

## Scope

Supporting unit-system evidence for the DEL-07-03/DEL-03-01 material-library
property helper. The work is an app draft-authoring tranche and does not alter
the DEC-018 catalog or conversion core.

## Changes

- Desktop/Tauri mode uses the reviewed DEC-018 unit catalog route to filter
  material-property selector options by compatible dimension.
- Browser preview remains explicit no-fallback behavior: the selected property
  default unit ref remains visible, and the desktop-only catalog diagnostic is
  shown.
- The drafted payload uses schema-native `unit_ref` plus `dimension_id`,
  preserving the existing material-schema unit-bearing quantity contract.

## Validation

Passed:

- `npm test --workspace apps/desktop -- LibraryManagerPanel.test.tsx` — 13/13.
- `npm test --workspace apps/desktop` — 18/18 files, 395/395 tests.
- `npm run build --workspace apps/desktop` — passed with existing Vite
  large-chunk warning.
- `npm run test:e2e --workspace apps/desktop -- -g "library manager loads"` —
  2/2 Playwright tests.

## Boundary Review

No DEC-018 catalog constant change, schema dimension enum change, unit
conversion API change, material engineering allowable, public material value,
protected standards content, private project payload, lifecycle transition,
release-readiness claim, professional approval, certification, sealing,
authentication, or code-compliance claim changed.
