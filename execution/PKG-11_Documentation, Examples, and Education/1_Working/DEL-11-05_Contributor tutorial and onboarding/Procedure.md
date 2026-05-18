# Procedure: DEL-11-05 Contributor tutorial and onboarding

## Purpose

This procedure defines how to produce and verify the contributor onboarding tutorial draft for DEL-11-05 inside the deliverable working folder.

## Prerequisites

- The contributor has read the assigned brief and confirmed the write scope is only this deliverable folder.
- `_CONTEXT.md`, `_REFERENCES.md`, `_DEPENDENCIES.md`, `_STATUS.md`, and register rows for DEL-11-05 and SOW-033 are available.
- Governing references listed in `_REFERENCES.md` are accessible for reading.
- No protected standards examples, proprietary vendor data, private rule packs, owner standards, or commercial software examples are imported.
- The current lifecycle state allows setup refresh.

## Steps

1. Confirm deliverable identity.
   - Read `_CONTEXT.md`.
   - Verify `Deliverable ID: DEL-11-05`, `Package ID: PKG-11`, `Type: DOC_UPDATE`, `Scope Coverage: SOW-033`, and `Objective Support: OBJ-001; OBJ-002`.

2. Confirm source authority.
   - Read `INIT.md`, `AGENTS.md`, `docs/CONTRACT.md`, `docs/TYPES.md`, `docs/SPEC.md`, `docs/AGENTIC_DEVELOPMENT_WORKFLOW.md`, `docs/IP_AND_DATA_BOUNDARY.md`, and the decomposition/register rows listed in `_REFERENCES.md`.
   - Use source locations in the document kit where requirements or constraints are stated.

3. Draft the contributor tutorial concept in the four-document kit.
   - Put descriptive identity, boundaries, and references in `Datasheet.md`.
   - Put normative onboarding requirements, invariants, and acceptance criteria in `Specification.md`.
   - Put rationale, trade-offs, safe/unsafe examples, and source-boundary advice in `Guidance.md`.
   - Put contributor execution and validation steps in this `Procedure.md`.

4. Preserve hard boundaries.
   - Do not edit repo-level `CONTRIBUTING`, `docs/AGENTIC_DEVELOPMENT_WORKFLOW.md`, source code, examples, schemas, or documentation outside this deliverable.
   - Do not introduce protected standards text, copied standards tables, code-derived formulas, commercial software examples, proprietary vendor data, private rule packs, owner standards, or company design bases.
   - Do not claim software, agents, contributors, or maintainers certify, approve, seal, authenticate, or declare engineering code compliance.

5. Generate setup artifacts in sequence.
   - Run `four-documents` with `RUN_PASSES=P1_P2`.
   - Run `semantic-matrix-build`.
   - Run `lens-register`.
   - Run `four-documents` with `RUN_PASSES=P3_ONLY`.
   - Run `dependency-extract`.

6. Validate setup gates.
   - Run `tools/validation/check_min_viable_fileset.sh <deliverable>`.
   - Run `tools/validation/check_four_documents.sh <deliverable>`.
   - Run `python3 tools/validation/validate_dependencies_schema.py <deliverable>/Dependencies.csv`.
   - Run `python3 tools/validation/validate_enum.py LIFECYCLE_STATE SEMANTIC_READY`.
   - Scan the local artifacts for protected-data risk and certification/compliance-claim language.

7. Set status only after gates pass.
   - If all setup gates pass, update `_STATUS.md` to `SEMANTIC_READY`.
   - If any setup gate fails, leave status below `SEMANTIC_READY`, record the failure in `_run_records`, and surface open issues.

8. Return the handoff.
   - Report status, changed paths, validation commands/results, warnings, and open issues.

## Verification

The setup is valid when all required files exist, dependency schema validation passes, the semantic lens and lensing register exist, run records are present, no out-of-scope file is modified, and `_STATUS.md` is set to `SEMANTIC_READY` only after those checks pass.

## Records

- `Datasheet.md`
- `Specification.md`
- `Guidance.md`
- `Procedure.md`
- `_SEMANTIC.md`
- `_SEMANTIC_LENSING.md`
- `Dependencies.csv`
- `_DEPENDENCIES.md`
- `_STATUS.md`
- `_run_records/TASK_RUN_*.md`
