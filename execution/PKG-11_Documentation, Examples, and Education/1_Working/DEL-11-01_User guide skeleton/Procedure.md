# Procedure: DEL-11-01 User guide skeleton

## Purpose

Use this procedure to produce and check the deliverable-local user guide skeleton. The procedure is documentation-oriented and does not edit `docs/user_guide/index.md` or move any artifact to `ISSUED`.

## Prerequisites

| Prerequisite | Requirement |
|---|---|
| Sealed context | Confirm `DEL-11-01`, `PKG-11`, SOW-033, OBJ-001, and OBJ-011 from `_CONTEXT.md` and the registers. |
| Governing sources | Read the local references listed in `_REFERENCES.md`, especially `INIT.md`, `docs/DIRECTIVE.md`, `docs/CONTRACT.md`, `docs/TYPES.md`, `docs/SPEC.md`, and `docs/IP_AND_DATA_BOUNDARY.md`. |
| Data boundary | Do not introduce protected standards data, proprietary commercial data, or private user/project data into public setup artifacts. |
| Professional boundary | Do not certify, endorse, approve, seal, authenticate, or declare engineering code compliance. |
| Write scope | Write only inside this deliverable folder. |

## Steps

1. Confirm the deliverable identity and current lifecycle state in `_STATUS.md`.
2. Draft the four-document kit:
   - `Datasheet.md` captures identity, source basis, boundaries, and the guide outline.
   - `Specification.md` captures guide skeleton requirements, exclusions, and acceptance checks.
   - `Guidance.md` captures interpretation principles, trade-offs, and prohibited overclaims.
   - `Procedure.md` captures production and verification steps.
3. Check that the guide outline covers project setup, project creation, centerline modeling, mechanics solving, user rule checks, results review, reports, limitations, troubleshooting, and glossary/status vocabulary.
4. Check that guide language preserves these separations:
   - mechanics solve vs user rule check;
   - user rule check vs human professional acceptance;
   - open mechanics vs protected standards data;
   - global centerline analysis vs local FEA handoff.
5. Mark unresolved product behavior as `TBD`; do not invent installation commands, UI behavior, external formats, solver-library choices, rule expression grammar, or project container behavior.
6. Generate semantic matrix and lensing artifacts as setup evidence.
7. Refresh dependency artifacts with conservative, evidence-cited rows.
8. Run local validation checks and record warnings in `_run_records`.

## Verification

| Check | Command or review method | Pass condition |
|---|---|---|
| Four-document kit | `tools/validation/check_four_documents.sh <deliverable-folder>` | All four documents are present. |
| Required guide sections | Review `Datasheet.md` Construction and `Specification.md` UG-REQ-001. | Required user guide section slots are present. |
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
