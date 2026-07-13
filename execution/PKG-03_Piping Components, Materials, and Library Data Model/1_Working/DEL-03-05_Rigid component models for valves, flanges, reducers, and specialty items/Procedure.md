# Procedure: DEL-03-05 Rigid component models for valves, flanges, reducers, and specialty items

## Purpose

Define the reconciliation procedure for keeping DEL-03-05 documentation aligned with implemented component schema evidence while preserving the data boundary, unit discipline, and architecture-basis constraints.

## Prerequisites

- Sealed brief for DEL-03-05 and write scope limited to this deliverable folder.
- `_CONTEXT.md`, `_REFERENCES.md`, and decomposition/register rows for DEL-03-05, SOW-009, OBJ-004.
- `docs/CONTRACT.md` invariants listed in the sealed brief.
- No proprietary, protected, or vendor component data in the working inputs.
- Read-only evidence: `schemas/component.schema.yaml`, `fixtures/component/invented_component_library_valid.json`, `tests/test_component_section_schema.py`, `MEMORY.md`, `_REVIEW.md`, and `Review_Findings.csv`.

## Steps

1. Confirm the component families in scope from implementation evidence: `valve`, `flange`, `reducer`, `rigid`, and `specialty`.
2. Confirm implemented model slots from schema/fixture/test evidence: rigid body length, end-size/reference slots, weight, COG, `linear_stiffness`, `rotational_stiffness`, stiffness behavior reference, and source/manufacturer reference slots.
3. Confirm strict fixture behavior: public component values remain omitted, missing values are explicit, provenance/review metadata is present, and blocking diagnostics represent incomplete mechanics inputs.
4. Preserve unresolved choices as `TBD`: accepted source catalogs, public fixture-value policy, coordinate convention, exact stiffness solver treatment, concrete import formats, per-family engineering profiles, dependency satisfaction, human disposition, and lifecycle closure.
5. Apply public/private data controls: every reusable component datum needs provenance, license/redistribution status where applicable, and contributor/review disposition before public acceptance.
6. Preserve review finding semantics: `Review_Findings.csv` records PKG03-DEL-03-05-PKG02-001 and PKG03-DEL-03-05-PKG02-002 as `ACCEPT_AS_IS` / `RESOLVED` under Gate C on 2026-06-05; documentation maintenance must not rewrite that formal disposition.
7. Preserve architecture constraints from AB-00-01, AB-00-02, AB-00-04, AB-00-06, AB-00-07, and AB-00-08 for any downstream implementation handoff.
8. Surface gaps as diagnostics, `TBD` markers, or human-ruling items rather than resolving them silently.
9. Run targeted validation when feasible: `python3 -m pytest tests/test_component_section_schema.py` and a stale-language `rg` scoped to `Datasheet.md`, `Specification.md`, `Guidance.md`, and `Procedure.md`.

## Verification

- Four local reconciliation documents exist and retain the default sections.
- No numeric component defaults, vendor data, protected tables, or standards text are introduced.
- Implemented schema/fixture/test evidence is distinguished from unresolved policy, dependency, human-disposition, and lifecycle gaps.
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

## D-41 R5 T2B PDU-013 Check

Before any COG value is accepted for mechanics use, verify that an authorized record defines both its coordinate convention and reference frame. Until then, preserve the slot as non-operative metadata and record the gap; do not infer axes, origins, signs, or component-local/global transforms.

For PDU-023, validate paired scalar paths on copied component geometry quantities and reject incomplete quantity metadata; do not infer a runtime envelope producer.
