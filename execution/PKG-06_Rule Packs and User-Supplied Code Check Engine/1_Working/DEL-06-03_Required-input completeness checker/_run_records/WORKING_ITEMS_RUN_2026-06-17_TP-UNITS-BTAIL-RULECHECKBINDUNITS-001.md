---
doc_id: WORKING-ITEMS-RUN-2026-06-17-TP-UNITS-BTAIL-RULECHECKBINDUNITS-001-DEL-06-03
doc_kind: working_items.run_record
status: completed
created: 2026-06-17
agent: WORKING_ITEMS
tranche_id: TP-UNITS-BTAIL-RULECHECKBINDUNITS-001
smoke_id: TP-MAC-228
deliverable_id: DEL-06-03
package_id: PKG-06
role: primary
---

# WORKING_ITEMS Run Record - TP-UNITS-BTAIL-RULECHECKBINDUNITS-001

## Scope

Bounded Phase B-tail run-check binding unit-policy slice while C5.7 remains
human-execution gated. The tranche makes the existing Run Rule Checks binding
plan visibly report unit-binding policy evidence for runtime value, slot,
solver-result, and private-library binding routes.

## Implementation

- Added `rule-check-unit-binding-policy` to
  `apps/desktop/src/features/rule-check/RuleCheckRunPanel.tsx`.
- The policy row derives from the existing `RuleCheckBindingPlan` and current
  unit-catalog route:
  - user value input count;
  - value slot count;
  - run-panel solver selector count;
  - authored `solver_result_ref` count;
  - authored private-library reference count;
  - catalog route (`browser_manual_text_no_fallback`, `dec018_catalog(...)`,
    or `not_required`);
  - `conversion=false`.
- Added focused Vitest assertions for browser/manual-text, desktop DEC-018
  catalog, and private-library-reference cases.
- Added a focused Chromium desktop smoke assertion to the existing
  run-rule-checks journey.

## Validation

- `npm run test --workspace apps/desktop -- RuleCheckRunPanel.test.tsx` passed 18/18 tests.
- `npx playwright test e2e/r2-smoke.spec.ts -g "run-rule-checks panel loads the demo pack, derives bindings, and reports the desktop-only run seam" --project=chromium-desktop` passed 1/1 focused Chromium desktop test.
- `npm run test --workspace apps/desktop` passed 18/18 files and 399/399 tests.
- `npm run build --workspace apps/desktop` passed with the existing Vite large-chunk warning.
- `npx playwright test e2e/r2-smoke.spec.ts --workers=1` passed 18/18 tests.

## Boundary Review

No rule-pack schema, expression grammar, parser/text syntax, backend
completeness/evaluator behavior, rule-pack persistence, solver behavior, unit
conversion API, DEC-018 catalog constant, schema dimension enum, protected
standards content, private payload, lifecycle transition, release-readiness
claim, professional approval, certification, sealing, authentication, or
code-compliance claim changed.
