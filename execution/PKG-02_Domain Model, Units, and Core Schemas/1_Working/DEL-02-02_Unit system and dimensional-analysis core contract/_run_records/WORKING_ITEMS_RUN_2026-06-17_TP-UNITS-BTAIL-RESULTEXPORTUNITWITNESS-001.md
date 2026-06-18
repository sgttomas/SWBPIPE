# WORKING_ITEMS RUN - TP-UNITS-BTAIL-RESULTEXPORTUNITWITNESS-001

- **Date:** 2026-06-17
- **Persona:** WORKING_ITEMS
- **Supporting deliverable:** DEL-02-02 Unit system and dimensional-analysis core contract
- **Primary deliverable:** DEL-08-04 Result export format
- **Tranche:** TP-UNITS-BTAIL-RESULTEXPORTUNITWITNESS-001
- **SMOKE:** TP-MAC-193

## Supporting Scope

Support DEL-08-04 by recording that schema-first result exports can carry
row-level value/unit/dimension preservation evidence using the project unit
system reference and no conversion.

## Changes

- `schemas/results.schema.yaml` now defines optional
  `UnitPreservationWitness` and `UnitPreservationQuantity` result-envelope
  vocabulary.
- Desktop result export witnesses reference the accepted preview unit-system
  basis and preserve each exported row's source value, unit, and dimension
  with `conversion_performed=false`.

## Validation

- `python3 tests/test_results_schema.py` passed.
- `npm --prefix apps/desktop test -- App.test.tsx` passed 55/55 tests.
- `npm test --workspace apps/desktop` passed 18/18 files and 391/391 tests.
- `npm run build --workspace apps/desktop` passed with the existing Vite
  large-chunk warning.
- `git diff --check` passed.

## Boundary

Supporting unit evidence only. No DEC-018 catalog constant change, unit
conversion API, tolerance policy, solver behavior, protected standards
content, private data, lifecycle transition, release-readiness claim,
professional approval, certification, sealing, authentication, or
code-compliance claim changed.

## Handoff

- Primary owner: DEL-08-04.
- DEL-02-02 role: unit-system/dimensional contract support for the
  result-envelope witness shape.
- Remaining B-tail items: broader app unit entry/pickers beyond landed
  surfaces, remaining target-format conversion witnesses outside already
  covered boundaries, and D-04/DEC-026 mixed-unit tolerance corpus.
