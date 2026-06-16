# WORKING_ITEMS Run Record - TP-UNITS-B2B3-PCFCONVWITNESS-001

Date: 2026-06-16
Persona: WORKING_ITEMS
Deliverable: DEL-17-07 Conservative PCF subset exporter
Package: PKG-17 Export Format Interoperability

## Scope

Bounded B2/B3 desktop PCF export tranche adding target-format conversion
witnesses beyond the previously landed unit-system disclosure.

## Changes

- Added `conversion_witnesses` to the desktop conservative PCF export package.
- Added manifest member `conversion_witnesses.json`.
- Each witness records source object/field, source value/unit/dimension,
  target PCF value/unit/field, conversion factor, DEC-018/DEL-02-02 basis refs,
  and preview provenance.
- Added validation checks for expected witness count and `MM` target units.
- Added App regression coverage for the downloaded PCF JSON package, including
  `pipe:P-120` OD `0.168 m` to `168 MM` and a node-coordinate `MM` witness.

## Validation

- `npm test --workspace apps/desktop -- App.test.tsx` - PASS, 54 tests.
- `npm test --workspace apps/desktop` - PASS, 386 tests.
- `npm run build --workspace apps/desktop` - PASS with existing Vite
  chunk-size warning.
- `npm run test:e2e --workspace apps/desktop -- --grep "R2 desktop preview smoke"` - PASS, 2 tests.

## Boundary

Desktop PCF package only. No PCF completeness claim, downstream import
compatibility claim, solver-validation claim, code-compliance claim,
release-readiness claim, professional-acceptance claim, protected standards
content, private data, commercial solver behavior, or reverse-engineering was
introduced.

## Residuals

PCF target profile/version, nominal size, downstream import behavior,
support/restraint semantics, hidden translator defaults, direct PCF stable-ID
carriage, broader entity coverage, import round-trip unit I/O, lifecycle
decisions, and target-specific compatibility claims remain future guarded work.
