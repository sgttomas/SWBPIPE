# WORKING_ITEMS Run Record — TP-UNITS-B2B3-REVIEWGEOMUNITWITNESS-001

**Date:** 2026-06-16
**Persona:** WORKING_ITEMS
**Primary deliverable:** DEL-17-08 — GLB glTF review geometry export
**Supporting deliverable:** DEL-02-02 — Unit system and dimensional-analysis core contract
**Plan lane:** Phase B2/B3 unit-aware I/O and mixed-unit evidence

## Scope

Record supporting DEL-02-02 evidence for review-geometry coordinate unit
witnesses. No unit catalog, conversion constant, dimensional vocabulary, or
runtime conversion-engine behavior changed in this tranche.

## Evidence

- The desktop review-geometry packet now declares the accepted
  `unit-system:dec-018-si-dual-display` basis and target glTF coordinate unit
  `m`.
- Coordinate witnesses preserve source model metre values while documenting
  the target glTF axis transform. The packet records
  `conversion_performed=false` and `axis_transform_performed=true`.
- Regression coverage checks 54 witness records for the invented preview
  fixture and proves the concrete `pipe:P-120` source Y coordinate maps to
  target glTF Z with the documented sign flip.

## Validation

- `npm test --workspace apps/desktop -- App.test.tsx` — passed, 54/54.
- `npm test --workspace apps/desktop` — passed, 18 files / 386 tests.
- `npm run build --workspace apps/desktop` — passed with the existing Vite
  chunk-size warning.
- `npm run test:e2e --workspace apps/desktop` — final rerun passed 10/10 after
  an initial Chrome launch SIGKILL before app startup.
- `python3 tools/release/run_evidence_sweep.py --execute` — passed all five
  DEC-025 surfaces. Summary:
  `validation/evidence/sweeps/SWEEP_20260616T013908Z_673a4de9628e-dirty.json`.

## Boundary Review

No DEC-018 catalog constant, unit-conversion API, tolerance policy, schema
dimension enum, protected standard content, private project data, release
readiness, professional approval, certification, sealing, authentication, or
code-compliance claim changed.
