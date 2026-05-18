# Run Record: dependency-extract

| Field | Value |
|---|---|
| Deliverable | DEL-08-03 |
| Skill | dependency-extract |
| Date | 2026-04-30 |
| Actor | TASK |
| Run Status | PASS_WITH_WARNING |

## Inputs Read

- `Datasheet.md`
- `Specification.md`
- `Guidance.md`
- `Procedure.md`
- `_DEPENDENCIES.md`
- `_REFERENCES.md`
- `docs/_Decomposition/SOFTWARE_DECOMP.md`
- `docs/_Registers/Deliverables.csv`
- `docs/_Registers/ScopeLedger.csv`
- `skills/dependency-extract/SKILL.md`

## Outputs Written

- `Dependencies.csv`
- `_DEPENDENCIES.md`

## Extracted Rows

- Anchor rows: 3
- Execution rows: 9
- Active rows: 12
- Retired rows: 0

## Validation Notes

- Schema validation: PASS - `python3 tools/validation/validate_dependencies_schema.py <deliverable>/Dependencies.csv` reported 29 required columns and 12 data rows.
- Enum validation: PASS for all unique values used in dependency class, anchor type, direction, dependency type, target type, explicitness, confidence, origin, status, and satisfaction status.
- ID-format helper warning: `tools/validation/validate_id_format.sh PKG PKG-08`, `DEL DEL-08-03`, `DEP DEP-08-03-001`, and `SOW SOW-024` fail because the helper uses old `PKG-000`/`DEL-000-00`/`SOW-0000` patterns; project-governing `docs/TYPES.md` uses `PKG-XX`/`DEL-XX-YY` and current registers use `SOW-024`.

