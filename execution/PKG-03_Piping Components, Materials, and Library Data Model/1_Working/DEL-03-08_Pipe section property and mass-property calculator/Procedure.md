# Procedure: DEL-03-08 Pipe section property and mass-property calculator

## Purpose

Define the setup procedure for producing future implementation evidence for the pipe section property and mass-property calculator without introducing protected data or unapproved repo-level changes.

## Prerequisites

| Prerequisite | Status |
|---|---|
| Sealed deliverable context for DEL-03-08 | Available in `_CONTEXT.md`. |
| Unit-system and dimensional-analysis contract | Required input; exact API and accepted dimensions `TBD`. |
| Pipe section/component library schema contract | Required input for schema hooks; exact fields `TBD`. |
| Material library provenance model | Required input when density or material data originates from a library; exact fields `TBD`. |
| Diagnostic/result envelope contract | Required input for warnings/errors; exact codes `TBD`. |
| Synthetic or cleared fixture policy | Required before adding mass-property tests; fixture values `TBD`. |

## Steps

1. Confirm the task is sealed to DEL-03-08 and that write scope is limited to the authorized deliverable folder or later authorized implementation paths.
2. Read the accepted unit-system contract and identify required dimensions for length, area, second moment, volume, density, mass, and mass per length. If any dimension is absent, record `TBD`.
3. Read accepted schema contracts for pipe sections, materials, provenance, and redistribution status. If schema hook names are absent, record `TBD`.
4. Define calculator inputs as explicit user-entered or lawfully imported values. Do not introduce bundled public dimensional, material, contents, insulation, or corrosion defaults.
5. Define validation behavior for missing, dimensionally incompatible, non-physical, or unprovenanced inputs. Exact diagnostic codes remain `TBD` until the diagnostic contract is accepted.
6. Define output shape for section properties and mass properties, preserving units and provenance references where applicable.
7. Create tests using synthetic or cleared values only. Include negative tests for missing values, incompatible units, and missing provenance where applicable.
8. Run unit, schema, protected-content, and boundary checks required by the accepted architecture basis.
9. Record unresolved inputs as `TBD` and route them to the responsible schema/unit/diagnostic/human owner.

## Verification

| Check | Expected result |
|---|---|
| Protected data check | No protected pipe tables, material tables, code tables, copied formulas, or proprietary fixtures are introduced. |
| Unit check | Inputs and outputs are dimensionally validated through accepted unit contracts. |
| Missing value check | Missing required values produce explicit findings, not defaults. |
| Provenance check | Library/imported values preserve source and redistribution status. |
| Boundary check | Calculator remains outside global solver and rule-pack compliance logic. |

## Records

- Updated four-document kit in this folder.
- `_SEMANTIC.md` and `_SEMANTIC_LENSING.md` setup artifacts.
- `Dependencies.csv` v3.1 and `_DEPENDENCIES.md`.
- `_run_records/TASK_RUN_*.md` records for each setup sequence step.
