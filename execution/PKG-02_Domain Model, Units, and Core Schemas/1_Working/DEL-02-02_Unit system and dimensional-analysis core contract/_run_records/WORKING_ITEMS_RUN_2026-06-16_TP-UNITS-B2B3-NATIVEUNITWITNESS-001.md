# WORKING_ITEMS Run Record — TP-UNITS-B2B3-NATIVEUNITWITNESS-001

**Date:** 2026-06-16
**Persona:** WORKING_ITEMS
**Primary deliverable:** DEL-17-03 — Native open JSON export package
**Supporting deliverable:** DEL-02-02 — Unit system and dimensional-analysis core contract
**Plan lane:** Phase B2/B3 unit-aware I/O and mixed-unit evidence

## Scope

Record supporting DEL-02-02 evidence for native JSON package unit-preservation
witnesses. No unit catalog, conversion constant, dimensional vocabulary, or
runtime conversion-engine behavior changed in this tranche.

## Evidence

- The desktop native JSON package review packet now declares the accepted
  `unit-system:dec-018-si-dual-display` basis for package unit witnesses.
- The package is a project-owned JSON preservation path, so model and result
  quantity fields carry identical source and target value+unit records with
  `conversion_performed=false`.
- The witness sidecar records 6 project unit declarations, 18 model quantity
  witnesses, and 739 result quantity witnesses for the invented preview
  fixture.
- Regression coverage checks concrete model and result witnesses, including
  `pipe:P-120` outside diameter and `result:force:pipe-P-120:axial`.

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

No DEC-018 catalog constant, unit-conversion API, tolerance policy, schema
dimension enum, protected standard content, private project data,
network/telemetry path, release readiness, professional approval,
certification, sealing, authentication, or code-compliance claim changed.
