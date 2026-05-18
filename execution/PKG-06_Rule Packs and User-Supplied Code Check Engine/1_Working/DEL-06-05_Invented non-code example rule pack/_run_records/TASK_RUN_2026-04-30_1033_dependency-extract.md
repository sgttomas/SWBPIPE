# TASK RUN: dependency-extract

| Field | Value |
|---|---|
| Deliverable | DEL-06-05 Invented non-code example rule pack |
| Package | PKG-06 Rule Packs and User-Supplied Code Check Engine |
| Skill | dependency-extract |
| SCOPE | DEL-06-05 |
| MODE | UPDATE |
| STRICTNESS | CONSERVATIVE |
| Generated | 2026-04-30 |
| Status | PASS with warning |

## Inputs Read

- `_CONTEXT.md`
- `_DEPENDENCIES.md`
- `_REFERENCES.md`
- `Datasheet.md`
- `Specification.md`
- `Guidance.md`
- `Procedure.md`
- `docs/_Decomposition/SOFTWARE_DECOMP.md`
- `docs/_Registers/Deliverables.csv`
- `docs/_Registers/ScopeLedger.csv`
- `skills/dependency-extract/SKILL.md`

## Outputs Written

- `Dependencies.csv`
- `_DEPENDENCIES.md`

## Extraction Summary

- ACTIVE rows: 7.
- RETIRED rows: 0.
- Parent anchors: 1.
- Trace anchors: 3.
- Execution constraints/interfaces: 3.

## Local Validation Results

- `python3 tools/validation/validate_dependencies_schema.py Dependencies.csv`: PASS; 29 required columns; 7 data rows.
- `python3 tools/validation/validate_enum.py <ENUM_NAME> <VALUE>` for emitted enum values: PASS.
- Evidence/source fields: PASS; every active row has `EvidenceFile` and `SourceRef`.
- Dependency ID uniqueness: PASS.
- `FromDeliverableID` consistency: PASS.
- `tools/validation/validate_id_format.sh`: WARNING; helper expects legacy `PKG-[0-9]{3}`, `DEL-[0-9]{3}-[0-9]{2}`, and `SOW-[0-9]{4}` formats, while the active project registers use `PKG-06`, `DEL-06-05`, and `SOW-016`.

## Boundary Notes

- Dependencies are information-flow and evidence constraints only.
- No scheduling-only dependency was created.
- No protected data or professional-authentication claim was introduced.
