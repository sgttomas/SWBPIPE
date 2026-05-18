# TASK RUN: dependency-extract

| Field | Value |
|---|---|
| Deliverable | DEL-07-01 3D viewport and centerline editor |
| Package | PKG-07 Graphical User Interface and Engineering Workflow |
| Skill | dependency-extract |
| MODE | UPDATE |
| STRICTNESS | CONSERVATIVE |
| Generated | 2026-04-30 |
| Status | PASS |

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
- `skills/dependency-extract/QA_CHECKS.md`

## Outputs Written

- `Dependencies.csv`
- `_DEPENDENCIES.md`

## Local Quality Checks

- `Dependencies.csv` includes all 29 v3.1 required columns.
- Dependency IDs are unique within the file.
- ACTIVE rows include `EvidenceFile` and `SourceRef`.
- One parent anchor row exists.
- No source documents were modified by this dependency-extract step.
