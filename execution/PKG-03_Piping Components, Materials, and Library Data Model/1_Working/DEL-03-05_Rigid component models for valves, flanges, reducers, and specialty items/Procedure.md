# Procedure: DEL-03-05 Rigid component models for valves, flanges, reducers, and specialty items

## Purpose

Define the setup procedure for carrying DEL-03-05 into future implementation while preserving the data boundary, unit discipline, and architecture-basis constraints.

## Prerequisites

- Sealed brief for DEL-03-05 and write scope limited to this deliverable folder.
- `_CONTEXT.md`, `_REFERENCES.md`, and decomposition/register rows for DEL-03-05, SOW-009, OBJ-004.
- `docs/CONTRACT.md` invariants listed in the sealed brief.
- No proprietary, protected, or vendor component data in the working inputs.

## Steps

1. Confirm the component families in scope: valves, flanges, reducers, and specialty items.
2. Record required model slots as user-supplied or lawfully imported: dimensions, weights, centers of gravity, and stiffness behavior.
3. Mark unresolved implementation choices as `TBD`, including field taxonomy, coordinate convention, stiffness representation, and mandatory-per-family validation rules.
4. Apply public/private data controls: every reusable component datum needs provenance, license/redistribution status where applicable, and contributor/review disposition before public acceptance.
5. Define future fixture expectations using synthetic or rights-cleared values only; do not use protected dimensional tables, catalog weights, vendor examples, or invented engineering defaults.
6. Create family-specific validation profiles only after schema scope is sealed; profiles should distinguish common fields from valve, flange, reducer, and specialty-item constraints.
7. Preserve architecture constraints from AB-00-01, AB-00-02, AB-00-04, AB-00-06, AB-00-07, and AB-00-08 when future implementation is dispatched.
8. Surface gaps as diagnostics, TBD markers, or human-ruling items rather than resolving them silently.

## Verification

- Four local setup documents exist and retain the default sections.
- No numeric component defaults, vendor data, protected tables, or standards text are introduced.
- All unknown implementation specifics are marked `TBD` or as assumptions/proposals.
- Dependency register validates against v3.1 schema.
- `_STATUS.md` is not set to `ISSUED`.

## Records

- `Datasheet.md`
- `Specification.md`
- `Guidance.md`
- `Procedure.md`
- `_SEMANTIC.md`
- `_SEMANTIC_LENSING.md`
- `Dependencies.csv`
- `_DEPENDENCIES.md`
- `_run_records/TASK_RUN_*.md`
