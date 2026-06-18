# WORKING_ITEMS Run Record - TP-UNITS-BTAIL-HEADLESSRUNNERUNITWITNESS-001

Date: 2026-06-17
Persona: WORKING_ITEMS
Primary deliverable: DEL-10-05 - Headless CLI and structured I-O analysis runner
Package: PKG-10 - Build, Packaging, API, and Interoperability
Tranche: TP-UNITS-BTAIL-HEADLESSRUNNERUNITWITNESS-001
Smoke target: TP-MAC-200

## Scope

Bounded Phase B-tail app integration slice while C5.7 remains human-execution
gated. Add unit-system disclosure and per-result unit-preservation witnesses
to the desktop Headless Runner envelope's local schema-first result-handoff
preview.

## Changes

- Updated `apps/desktop/src/features/headless-runner/HeadlessRunnerPanel.tsx`
  to include `result.unit_system_disclosure`, `result.unit_witness_policy`,
  and `result.unit_preservation_witnesses[]`.
- Added visible UI evidence rows:
  `headless-runner-units` and `headless-runner-unit-witnesses`.
- Extended `apps/desktop/src/App.test.tsx` to assert pre-run zero-witness
  disclosure and completed-run 737-witness preservation evidence.
- Extended `apps/desktop/e2e/r2-smoke.spec.ts` to assert the visible
  headless-runner unit disclosure and witness count in a real browser smoke.
- Recorded evidence in `apps/desktop/SMOKE.md`, `plans/PLAN_COMPLETION_LOG.md`,
  and the active completion plan B-tail row.

## Validation

- `npm test --workspace apps/desktop -- App.test.tsx` passed 55/55 tests.
- `npm run test:e2e --workspace apps/desktop -- -g "R2 desktop preview smoke"`
  passed 2/2 Playwright tests.
- `npm test --workspace apps/desktop` passed 18/18 files and 397/397 tests.
- `npm run build --workspace apps/desktop` passed with the existing Vite
  large-chunk warning.

DEC-025 evidence sweep and push closeout are handled by the parent
WORKING_ITEMS/CHANGE closeout.

## Boundary Review

Headless-runner result-handoff unit metadata only. This tranche did not choose
final CLI syntax, package scripts, process/network/filesystem policy, public
transport, CI/release matrix, runtime process launching, schema/public runtime
contract, unit conversion API, protected standards content, private payload,
lifecycle state, release-readiness status, professional approval,
certification, sealing, authentication, or code-compliance status.
