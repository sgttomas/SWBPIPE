---
run-id: WORKING_ITEMS_RUN_2026-06-12_units_schema_crate_contract
timestamp: 2026-06-12T02:48:48-06:00
persona: WORKING_ITEMS
tranche-id: TP-UNITS-B2B3-CONTRACT-001
package-id: PKG-02
deliverable-id: DEL-02-02
run-status: SUCCESS
authority:
  - docs/PRD.md §10 FR-002
  - docs/CONTRACT.md OPS-K-UNIT-1
  - execution/_Decomposition/SOFTWARE_DECOMP.md DEC-018
  - plans/PLAN_2026-06-10_prd_completion.md Phase B B2/B3 riders
write-scope:
  - core/units/**
  - execution/PKG-02_Domain Model, Units, and Core Schemas/1_Working/DEL-02-02_Unit system and dimensional-analysis core contract/MEMORY.md
  - execution/PKG-02_Domain Model, Units, and Core Schemas/1_Working/DEL-02-02_Unit system and dimensional-analysis core contract/_run_records/**
  - plans/PLAN_2026-06-10_prd_completion.md
  - plans/PLAN_COMPLETION_LOG.md
---

# WORKING_ITEMS Run - TP-UNITS-B2B3-CONTRACT-001

## Scope

Implemented a bounded B2/B3 contract rider after the B1 crate landed. This
tranche closes the two non-blocking 2026-06-12 findings about unit conversion
metadata and schema-to-crate dimension-vocabulary drift. It does not claim the
full B2 app/solver/report unit I/O retrofit.

## Changes

- Added crate-visible conversion metadata to `UnitDefinition`:
  `factor_representation`, optional `offset_representation`, explicit
  `ConversionProvenance`, and `ReviewStatus`.
- Replaced the prior binary provenance derivation with explicit provenance
  assignment, including `ConventionalPublicConstant` for lbf/psi-family
  conversions and `ProjectGovernedDecision` for project-governed semantic
  canonical bindings.
- Added a `DIMENSIONS` crate vocabulary constant.
- Added a Rust regression that parses `schemas/units.schema.yaml` and asserts
  exact set equality between the schema `DimensionId` enum and the crate
  dimension vocabulary.
- Added `serde_json` as a dev-dependency for the schema-parity test.

## Validation

- `cargo fmt --manifest-path core/units/Cargo.toml --check` - PASS.
- `cargo test --manifest-path core/units/Cargo.toml` - PASS, 13 unit tests and
  0 doctests.
- `python3 tests/test_units_schema.py` - PASS.
- `python3 -m pytest tests/test_units_schema.py` - PASS, 3 tests.

## Boundary Review

- No desktop unit picker/display retrofit, solver-boundary normalization,
  report rendering change, import/export change, or rule-pack unit evaluator
  change is claimed by this tranche.
- No protected standards text, protected dimensional tables, proprietary
  vendor data, private project data, material allowables, SIF/flexibility
  factors, or code-specific defaults were added.
- No lifecycle issuance, release-readiness, professional approval,
  certification, sealing, authentication, or code-compliance claim is made.

## Handoffs

- B2 still must wire unit-aware I/O through schemas, desktop fields,
  solver-boundary normalization, reports, imports, exports, and rule-pack
  evaluation while preserving entered units.
- B3 still must add the broader mixed-unit round-trip, conversion-witness,
  incompatible-unit rejection, and D-04/DEC-026 tolerance corpus.
