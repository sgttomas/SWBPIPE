# WORKING_ITEMS Run Record - TP-UNITS-BTAIL-NATIVEPACKAGEUNITS-001

**Date:** 2026-06-18
**Persona:** WORKING_ITEMS
**Primary tranche deliverable:** DEL-17-03 Native open JSON export package
**Supporting deliverable:** DEL-08-05 Report protected-content linter
**Smoke evidence:** apps/desktop/SMOKE.md TP-MAC-235

## Supporting Role

DEL-08-05 supplied the Report Content Lint inventory surface used to record
the Native JSON Package public unit-preservation target.

## Changes

- Report Content Lint now scans
  `apps/desktop/src/features/native-package/NativePackagePanel.tsx` as an
  explicit public preview target.
- The exported `unit_policy_evidence.target_refs` list now includes
  `unit_policy_surface_id=native-package-unit-witnesses`.
- The visible lint row now reports `unit_targets=19`,
  `conversion_witness_targets=2`, and `lint_conversion=false`.

## Evidence

- Focused App Vitest selected tests passed after updating stale
  target-count assertions and restoring one scenario-specific export-review
  lint count assertion.
- Focused Chromium desktop R2 Playwright smoke passed 1/1.
- Full desktop Vitest passed 18/18 files and 399/399 tests after repair.
- Desktop production build passed with the existing Vite large-chunk warning.
- Single-worker R2/R3 Playwright smoke passed 18/18 tests.

## Boundary Review

No report-linter protected-content semantics, legal clearance, redaction
controls, target writer compatibility, unit-conversion API, DEC-018 catalog
constant, schema dimension enum, protected standards content, private data,
lifecycle transition, release-readiness claim, professional approval,
certification, sealing, authentication, or code-compliance claim changed.
