# Procedure: DEL-06-05 Invented non-code example rule pack

## Purpose

Define the local setup procedure for preparing, checking, and handing off the invented non-code example rule-pack documentation without editing repo-level examples or introducing protected/proprietary engineering content.

## Prerequisites

| Prerequisite | Required state |
|---|---|
| Sealed deliverable context | `DEL-06-05`, `PKG-06`, and local write scope are known |
| Governing sources | `_CONTEXT.md`, `_REFERENCES.md`, `INIT.md`, `AGENTS.md`, `docs/CONTRACT.md`, `docs/DIRECTIVE.md`, `docs/TYPES.md`, `docs/SPEC.md`, decomposition, and registers are read |
| Current lifecycle state | `OPEN`, `INITIALIZED`, or `SEMANTIC_READY` before setup refresh |
| Protected-data posture | No protected standards text, formulas, tables, examples, or proprietary data are used |
| Professional-boundary posture | No certification, sealing, authentication, approval, or code-compliance reliance claim is made |

## Steps

1. Confirm the working path is `execution/PKG-06_Rule Packs and User-Supplied Code Check Engine/1_Working/DEL-06-05_Invented non-code example rule pack`.
2. Read the local context, dependency placeholder, references, semantic placeholder, and status files.
3. Draft or refresh `Datasheet.md`, `Specification.md`, `Guidance.md`, and `Procedure.md` from local governing sources.
4. Mark unknown engineering or schema-specific values as `TBD`; do not invent checksums, code formulas, allowables, or pass/fail criteria.
5. Include explicit non-engineering, protected-content, and professional-responsibility boundary language in the four-document kit.
6. Run the semantic matrix setup and write `_SEMANTIC.md` only inside the deliverable folder.
7. Run the lens-register setup and write `_SEMANTIC_LENSING.md` only inside the deliverable folder.
8. Run the P3-only four-document pass using `_SEMANTIC_LENSING.md` as a candidate worklist, not as authority.
9. Run dependency extraction and write only `Dependencies.csv` and `_DEPENDENCIES.md`.
10. Create `_run_records/*` entries for each setup step.
11. Run local validation commands and record pass/fail results.
12. Leave the current state as `SEMANTIC_READY` only if the setup artifacts and local gates pass.

## Verification

| Check | Command or review |
|---|---|
| Four-document kit present | `tools/validation/check_four_documents.sh <DELIVERABLE_PATH>` |
| Dependency CSV schema valid | `python3 tools/validation/validate_dependencies_schema.py <DELIVERABLE_PATH>/Dependencies.csv` |
| Dependency enums valid | `python3 tools/validation/validate_enum.py <ENUM_NAME> <VALUE>` for emitted enum values |
| No repo-level example edited | `git status --short -- <DELIVERABLE_PATH> examples docs` with review focused on assigned write scope |
| No protected data | Search local artifacts for standards-derived tables, formulas, allowables, or certification language |
| Run records present | Confirm `_run_records/` contains one record for each required setup skill step |

## Records

The setup run should leave these records in the deliverable folder:

- four production documents;
- semantic matrix and semantic lensing artifacts;
- dependency register and dependency summary;
- run records for `four-documents` P1/P2, `semantic-matrix-build`, `lens-register`, `four-documents` P3-only, and `dependency-extract`;
- `_STATUS.md` history showing the safe local state transitions.
