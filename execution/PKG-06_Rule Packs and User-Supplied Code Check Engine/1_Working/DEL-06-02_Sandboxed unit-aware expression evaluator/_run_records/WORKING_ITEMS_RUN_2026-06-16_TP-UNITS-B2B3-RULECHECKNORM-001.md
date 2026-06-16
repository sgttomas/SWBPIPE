# WORKING_ITEMS Run Record - TP-UNITS-B2B3-RULECHECKNORM-001

Date: 2026-06-16
Persona: WORKING_ITEMS
Deliverable: DEL-06-02 Sandboxed unit-aware expression evaluator
Package: PKG-06 Rule Packs and User-Supplied Code Check Engine

## Scope

Primary B2/B3 units tranche for rule-check execution: normalize compatible
DEC-018 catalog units before formula evaluation and acceptability comparison,
while preserving exact non-catalog demonstration units and blocking
incompatible substitutions.

## Changes

- Added `open_pipe_stress_units` as a dependency of
  `core/rules/rule_check_runner`.
- Added rule-check boundary normalization for required inputs and value-slot
  limits:
  - exact unit string match remains accepted, including invented `demo_unit`
    and `ratio`;
  - compatible DEC-018 catalog units convert to the rule-pack declaration unit;
  - missing, unknown, or incompatible units block with `UnitMismatch` and do
    not reach formula evaluation as supplied values.
- Added unit tests proving:
  - `0.05 MPa` solver input and `100 kPa` user limit normalize to declared
    `Pa` and compute ratio `0.5`;
  - `mm` supplied for a stress declaration blocks at
    `RULE_INPUTS_INCOMPLETE` with `UnitMismatch`.
- Added a desktop Tauri command regression proving the same compatible-unit
  normalization through `run_rule_checks_core` with a re-stamped invented demo
  rule pack.

## Validation

- `cargo test --manifest-path core/rules/rule_check_runner/Cargo.toml` - PASS
  (13 unit tests, 4 acceptability-relation integration tests, 3 invented demo
  integration tests).
- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml` - PASS,
  62 tests.
- `npm test --workspace apps/desktop` - PASS, 18 files, 386 tests.
- `npm run build --workspace apps/desktop` - PASS with existing Vite
  chunk-size warning.

## Boundary

No grammar change, schema change, text parser, protected standards content,
private project data, network/telemetry path, release-readiness claim,
professional approval, certification, sealing, authentication, or
code-compliance claim changed. The runner still produces only the automatic
rule-check statuses and never emits a professional acceptance claim.

## Residuals

Broader B3 mixed-unit round-trip, target-format conversion witnesses outside
covered export boundaries, and D-04/DEC-026 tolerance corpus remain future
guarded work outside this rule-check normalization slice.
