# WORKING_ITEMS Run Record - TP-UNITS-B2B3-RULECHECKNORM-001

Date: 2026-06-16
Persona: WORKING_ITEMS
Deliverable: DEL-02-02 Unit system and dimensional-analysis core contract
Package: PKG-02 Domain Model, Units, and Core Schemas

## Scope

Supporting B2/B3 unit evidence for rule-pack check execution over compatible
mixed units.

## Changes

- Connected the accepted DEC-018 unit catalog/conversion crate to
  `core/rules/rule_check_runner`.
- Rule-check required inputs and value-slot limits now normalize compatible
  catalog units to the declaration unit before the exact-unit evaluator runs.
- Exact non-catalog demonstration units remain exact-string-only; incompatible
  or unknown units block instead of being coerced.
- Witnesses cover compatible stress normalization (`MPa` and `kPa` to declared
  `Pa`) and incompatible stress/length rejection (`mm` for stress).

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

No new unit constants, protected-content ingestion, private-data ingestion,
rule grammar change, schema change, text parser, release-readiness claim,
professional approval, certification, sealing, authentication, or
code-compliance claim.

## Residuals

Broader B3 mixed-unit round-trip, remaining target-format conversion witnesses,
incompatible-unit rejection outside covered seams, and DEC-026 tolerance corpus
remain future guarded work.
