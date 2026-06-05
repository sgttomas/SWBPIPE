# Procedure: DEL-03-08 Pipe section property and mass-property calculator

## Purpose

Define the procedure for reconciling and maintaining implementation evidence for the pipe section property and mass-property calculator without introducing protected data or unapproved repo-level changes.

## Prerequisites

| Prerequisite | Status |
|---|---|
| Sealed deliverable context for DEL-03-08 | Available in `_CONTEXT.md`. |
| Unit-system and dimensional-analysis contract | Calculator requires explicit unit/dimension metadata and rejects mixed units; approved conversion API/catalog remains `TBD`. |
| Pipe section/component library schema contract | Schema-like mapping exists through `quantity_from_mapping`; accepted owner fields and dependency satisfaction remain `TBD`. |
| Material library provenance model | Calculator quantities require provenance; private-library record linkage remains `TBD`. |
| Diagnostic/result envelope contract | Calculator diagnostics carry class, source, affected object, and provenance; downstream result-envelope mapping remains `TBD`. |
| Synthetic or cleared fixture policy | Current tests use invented synthetic values; formal source catalog and fixture-value policy remain `TBD`. |

## Steps

1. Confirm the task is sealed to DEL-03-08 and that write scope is limited to the authorized deliverable folder or separately authorized implementation paths.
2. Read current implementation evidence in `core/section_properties/calculator.py`, `core/section_properties/README.md`, and `tests/test_section_properties.py`.
3. Confirm calculator inputs remain explicit user-entered or lawfully imported values. Do not introduce bundled public dimensional, material, contents, insulation, or corrosion defaults.
4. Confirm validation behavior remains explicit for missing required values, missing provenance, incompatible dimensions, mixed units, non-positive required values, and non-physical geometry.
5. Confirm output shape preserves magnitude, unit, dimension, and provenance for section properties and mass properties.
6. Confirm tests use synthetic or cleared values only and do not encode protected/reference-table content.
7. Run `python3 -m pytest tests/test_section_properties.py` when feasible.
8. Run a stale-language search over `Datasheet.md`, `Specification.md`, `Guidance.md`, and `Procedure.md`; reconcile implementation evidence while preserving unresolved `TBD` items.
9. Record unresolved policy, source catalog, fixture-value, dependency satisfaction, human disposition, lifecycle, and downstream integration inputs as `TBD` and route them to the responsible schema/unit/diagnostic/human owner.

## Verification

| Check | Expected result |
|---|---|
| Protected data check | No protected pipe tables, material tables, code tables, copied formulas, or proprietary fixtures are introduced. |
| Unit check | Inputs and outputs carry explicit units/dimensions; mixed units are rejected until conversion support is approved. |
| Missing value check | Missing required values produce explicit blocking diagnostics, not defaults. |
| Provenance check | Inputs require provenance and outputs preserve calculated provenance; exact library linkage remains `TBD`. |
| Review-finding check | Technical evidence remains aligned with `TECHNICALLY_ADDRESSED_PENDING_HUMAN`; `HumanDisposition` remains `TBD` until human action. |
| Boundary check | Calculator remains outside global solver and rule-pack compliance logic. |

## Records

- Updated four-document kit in this folder.
- `_SEMANTIC.md` and `_SEMANTIC_LENSING.md` historical semantic artifacts.
- `Dependencies.csv` v3.1 and `_DEPENDENCIES.md`.
- `_run_records/TASK_RUN_*.md` records for each TASK sequence step.
- Current evidence reconciliation run record: `_run_records/TASK_RUN_2026-06-05_DEL-03-08_evidence-reconciliation.md`.
