# WORKING_ITEMS Run Record - TP-UNITS-BTAIL-NATIVEPACKAGEUNITS-001

**Date:** 2026-06-18
**Persona:** WORKING_ITEMS
**Primary deliverable:** DEL-17-03 Native open JSON export package
**Supporting deliverables:** DEL-02-02 Unit system and dimensional-analysis core contract; DEL-08-05 Report protected-content linter
**Plan item:** Phase B-tail unit-aware I/O remainder
**Smoke evidence:** apps/desktop/SMOKE.md TP-MAC-235

## Scope

Record the existing Native JSON Package unit-preservation surface in the
Report Content Lint public unit-policy inventory while C5.7 remains
human-execution gated.

## Changes

- Added `apps/desktop/src/features/native-package/NativePackagePanel.tsx` to
  the Report Content Lint explicit public target roots and lint targets.
- Added `native-package-unit-witnesses` to Report Content Lint
  `unit_policy_evidence.target_refs`.
- Updated App Vitest and R2 Playwright assertions for
  `unit_targets=19`, report-lint target counts, and the native package target
  ref.
- Updated `apps/desktop/SMOKE.md`, the active completion plan, the completion
  log, and touched deliverable memories.

## Evidence

- `npm run test --workspace apps/desktop -- src/App.test.tsx -t "renders the engineering workspace from invented local fixtures"` initially caught stale target-count assertions, then passed 1/1 selected test after updates.
- `npm run test --workspace apps/desktop -- src/App.test.tsx -t "computed mechanics diagnostics"` initially caught stale target-count assertions, then passed 1/1 selected test after updates.
- `npm run test --workspace apps/desktop -- src/App.test.tsx -t "round trips local create, save, and open project controls without external file copies"` passed 1/1 selected test.
- `npm run test --workspace apps/desktop -- src/App.test.tsx -t "carries queued editor intents into the report packet as review-only operation context"` passed 1/1 selected test after restoring the scenario-specific export-review lint count assertion.
- `npx playwright test e2e/r2-smoke.spec.ts -g "R2 desktop preview smoke covers solve, results, report, and viewport overlay" --project=chromium-desktop` passed 1/1 focused Chromium desktop test.
- `npm run test --workspace apps/desktop` initially caught the over-broad export-review manifest count assertion and then passed 18/18 files and 399/399 tests after repair.
- `npm run build --workspace apps/desktop` passed with the existing Vite large-chunk warning.
- `npx playwright test e2e/r2-smoke.spec.ts --workers=1` passed 18/18 tests.

## Boundary Review

The tranche records inventory evidence over the existing native package unit
witness surface only. It does not change native JSON packet schema, package
members, package hash basis, target writers, target compatibility posture,
unit conversion behavior, DEC-018 catalog constants, schema dimension enums,
protected standards content, private payload handling, lifecycle state,
release readiness, professional approval, certification, sealing,
authentication, or code-compliance posture.

## Residuals

C5.7 remains human-execution gated. Broader B-tail unit-entry/picker and
target-format conversion-witness cleanup remains selectable only when it does
not displace C5 human-pass recording.
