# WORKING_ITEMS Run Record — TP-UNITS-B2B3-REVIEWGEOMUNITWITNESS-001

**Date:** 2026-06-16
**Persona:** WORKING_ITEMS
**Primary deliverable:** DEL-17-08 — GLB glTF review geometry export
**Supporting deliverable:** DEL-02-02 — Unit system and dimensional-analysis core contract
**Plan lane:** Phase B2/B3 unit-aware I/O and mixed-unit evidence

## Scope

Add auditable coordinate unit witnesses to the desktop review-geometry glTF
JSON preview packet. This closes one remaining target-format unit-evidence
gap outside the earlier PCF and CAEPIPE smoke-package witness boundaries.

## Changes

- Added `unit_system_disclosure` to the review-geometry packet with the
  DEC-018 unit-system reference, source model units, target glTF coordinate
  unit `m`, `conversion_performed=false`, and the existing
  `preview_z_up_to_gltf_y_up_rotation_x_minus_90` axis-transform policy.
- Added `coordinate_unit_witnesses` and package member
  `coordinate_unit_witnesses.json`.
- Witnesses are generated in the same vertex order as emitted glTF positions:
  pipe segment endpoints, node markers, support markers, and component
  markers. Each record carries source value/unit/dimension/axis, target
  glTF value/unit/dimension/axis, vertex coordinate location, basis refs, and
  preview provenance.
- Surfaced the witness count and unit policy in the existing Review Geometry
  Export panel.

## Validation

- `npm test --workspace apps/desktop -- App.test.tsx` — passed, 54/54.
- `npm test --workspace apps/desktop` — passed, 18 files / 386 tests.
- `npm run build --workspace apps/desktop` — passed with the existing Vite
  chunk-size warning.
- `npm run test:e2e --workspace apps/desktop` — first attempt had transient
  Chrome launch SIGKILL before app startup on the modified smoke spec; serial
  reruns passed, then the full suite passed 10/10.
- In-app Browser verification was attempted for `http://127.0.0.1:4179/` but
  blocked by the Browser URL policy before navigation; no workaround was used.
- `python3 tools/release/run_evidence_sweep.py --execute` — passed all five
  DEC-025 surfaces. Summary:
  `validation/evidence/sweeps/SWEEP_20260616T013908Z_673a4de9628e-dirty.json`.

## Boundary Review

This is browser-preview review geometry evidence only. It does not add binary
GLB packaging, viewer compatibility claims, solver-geometry equivalence,
target-compatibility claims, protected content, private project data, release
readiness, professional approval, certification, sealing, authentication, or
code-compliance claims.
