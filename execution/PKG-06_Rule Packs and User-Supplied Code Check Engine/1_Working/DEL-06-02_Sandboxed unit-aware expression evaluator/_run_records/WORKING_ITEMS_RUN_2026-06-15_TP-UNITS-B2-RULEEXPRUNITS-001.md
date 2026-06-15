# WORKING_ITEMS Run Record - TP-UNITS-B2-RULEEXPRUNITS-001

Date: 2026-06-15
Persona: WORKING_ITEMS
Deliverable: DEL-06-02 Sandboxed unit-aware expression evaluator
Package: PKG-06 Rule Packs and User-Supplied Code Check Engine

## Scope

Bounded B2/B3 app-integration tranche touching the rule-pack expression
authoring surface for the frozen DEC-022 AST. The slice is UI-only: it improves
authoring of existing expression unit metadata without changing the schema,
codec, evaluator grammar, or runtime evaluation semantics.

## Changes

- Updated `apps/desktop/src/features/rule-packs/ExpressionComposer.tsx` so
  literal `quantity.unit_ref`, table `argument_unit_ref`, and table
  `result_unit_ref` fields can use the accepted DEC-018 desktop unit catalog.
- Desktop selectors are filtered by the currently authored dimension and keep
  any stored out-of-catalog unit visible as `(current)` catalog-mismatch
  evidence instead of silently rewriting the rule-pack document.
- Browser preview remains manual text entry because the reviewed unit catalog
  is exposed through the desktop Tauri command only.
- Added mocked-Tauri `ExpressionComposer` tests and extended the existing
  browser Playwright rule-pack authoring journey.

## Validation

- `npm test --workspace apps/desktop -- ExpressionComposer` - PASS, 19 tests.
- `npm test --workspace apps/desktop` - PASS, 384 tests across 18 files.
- `npm run build --workspace apps/desktop` - PASS with the existing Vite
  chunk-size warning.
- `npm run test:e2e --workspace apps/desktop` - PASS, 10 Playwright tests
  across the configured desktop/compact Chromium projects.

## Boundary

No evaluator, schema, backend, parser, text-syntax, persistence, checksum, or
rule-check status change. The composer remains AST-only under D-02b / DEC-037;
the change does not embed private values, protected standards content, release
readiness, professional approval, certification, sealing, authentication, or
code-compliance claims.

## Residuals

Rule-pack expression evaluation still compares unit refs by authored metadata
according to the existing evaluator semantics. Broader Phase B conversion
witnesses and D-04/DEC-026 tolerance corpus coverage remain outside this
UI-only slice.
