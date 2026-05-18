# Procedure: DEL-09-04 Validation manual skeleton

## Purpose

Use this procedure to produce and check the deliverable-local validation manual skeleton. The procedure is documentation-oriented and does not move any artifact to `ISSUED`.

## Prerequisites

| Prerequisite | Requirement |
|---|---|
| Sealed context | Confirm `DEL-09-04`, `PKG-09`, SOW-027, OBJ-008, and OBJ-011 from `_CONTEXT.md` and the registers. |
| Governing sources | Read the local references listed in `_REFERENCES.md`, especially `docs/VALIDATION_STRATEGY.md`, `docs/CONTRACT.md`, `docs/TYPES.md`, `docs/SPEC.md`, and `docs/IP_AND_DATA_BOUNDARY.md`. |
| Data boundary | Do not introduce protected standards data, proprietary commercial data, or private user data into public setup artifacts. |
| Professional boundary | Do not certify, approve, seal, authenticate, or declare code compliance. |
| Write scope | Write only inside this deliverable folder. |

## Steps

1. Confirm the deliverable identity and current lifecycle state in `_STATUS.md`.
2. Draft the four-document kit:
   - `Datasheet.md` captures identity, source basis, boundaries, and the manual outline.
   - `Specification.md` captures requirements, exclusions, and acceptance checks.
   - `Guidance.md` captures interpretation principles, trade-offs, and prohibited overclaims.
   - `Procedure.md` captures production and verification steps.
3. Check that the validation manual outline includes the ten sections listed in `docs/VALIDATION_STRATEGY.md` section 3.
4. Check that mechanics verification, workflow validation, user rule checks, and professional reliance remain separate.
5. Check that release-gate language describes software quality evidence only.
6. Mark missing future evidence as `TBD` or as a visible open issue; do not invent benchmark results, source citations, or human rulings.
7. Generate semantic matrix and lensing artifacts as setup evidence.
8. Refresh dependency artifacts with conservative, evidence-cited rows.
9. Run local validation checks and record warnings in `_run_records`.

## Verification

| Check | Command or review method | Pass condition |
|---|---|---|
| Four-document kit | `tools/validation/check_four_documents.sh <deliverable-folder>` | All four documents are present. |
| Dependency schema | `python3 tools/validation/validate_dependencies_schema.py <deliverable-folder>/Dependencies.csv` | Schema validator reports `VALID`. |
| Enum checks | `python3 tools/validation/validate_enum.py <enum> <value>` for emitted dependency enum values | Emitted values are accepted by the enum validator. |
| Status check | Inspect `_STATUS.md`. | Current State is `SEMANTIC_READY` only after setup artifacts pass. |
| Protected-content check | Manual review and targeted search for protected/compliance overclaims. | No protected data or software certification/compliance claims are present. |

## Records

Required records for this setup run:

- four-document kit;
- `_SEMANTIC.md`;
- `_SEMANTIC_LENSING.md`;
- `Dependencies.csv`;
- refreshed `_DEPENDENCIES.md`;
- `_STATUS.md`;
- `_run_records/*` entries for the setup sequence and validations.
