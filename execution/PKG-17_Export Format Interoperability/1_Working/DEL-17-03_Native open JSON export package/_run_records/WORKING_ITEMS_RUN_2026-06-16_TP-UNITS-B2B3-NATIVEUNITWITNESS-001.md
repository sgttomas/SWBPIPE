# WORKING_ITEMS Run Record — TP-UNITS-B2B3-NATIVEUNITWITNESS-001

**Date:** 2026-06-16
**Persona:** WORKING_ITEMS
**Primary deliverable:** DEL-17-03 — Native open JSON export package
**Supporting deliverable:** DEL-02-02 — Unit system and dimensional-analysis core contract
**Plan lane:** Phase B2/B3 unit-aware I/O and mixed-unit evidence

## Scope

Add desktop native JSON package unit-preservation witnesses for project-owned
JSON quantity fields. This tranche is limited to the browser-preview package
review packet and its regression evidence.

## Evidence

- The native JSON package review packet now declares
  `maps/unit_preservation_witnesses.json` and an export-profile
  `unit_witness_policy`.
- The packet carries `unit_preservation` evidence with
  `unit-system:dec-018-si-dual-display`, source/target refs, quantity
  value+unit copies, basis refs, and `conversion_performed=false`.
- Witness coverage is 6 project unit declarations, 18 model quantity
  witnesses, and 739 result quantity witnesses for the invented preview
  fixture.
- Regression coverage proves concrete preservation witnesses for
  `pipe:P-120` outside diameter and `result:force:pipe-P-120:axial`.
- R2 Playwright smoke and in-app Browser verification both observed the
  visible Native JSON package unit witness counts.

## Validation

- `npm test --workspace apps/desktop -- App.test.tsx` — passed, 54/54.
- `npm run build --workspace apps/desktop` — passed with the existing Vite
  chunk-size warning.
- `npm test --workspace apps/desktop` — passed, 18 files / 386 tests.
- `npm run test:e2e --workspace apps/desktop` — passed, 10/10.
- In-app Browser at `http://127.0.0.1:4179/` — solve completed with
  `result_rows=737`; Native JSON package reported `project_units=6`,
  `model_quantities=18`, `result_quantities=739`, and `conversion=false`.
- `python3 tools/release/run_evidence_sweep.py --execute` — passed all five
  DEC-025 surfaces on the dirty tree. Summary:
  `validation/evidence/sweeps/SWEEP_20260616T025334Z_92ba64e2b4e3-dirty.json`.

## Boundary Review

No Python native export schema, writer, fixture, parser, public API, downstream
target adapter, target compatibility claim, protected standard content,
private project data, network/telemetry path, release readiness, professional
approval, certification, sealing, authentication, or code-compliance claim
changed.
