# Procedure: DEL-13-04 Physical-to-analytical transformation contract

**Generated:** 2026-05-03
**Status:** Initial draft from four-documents P1/P2
**Source posture:** Operational steps describe how to produce/use the contract without inventing unsupported implementation details.

## Purpose

This procedure defines the conservative workflow for producing and checking the DEL-13-04 physical-to-analytical transformation contract. The workflow keeps the physical model as source of truth, derives solver-ready analytical representation, records transformation warnings, and preserves traceability without introducing protected data, hidden defaults, or professional compliance claims.

## Prerequisites

1. Confirm current deliverable context:
   - `_CONTEXT.md` identifies DEL-13-04, PKG-13, BACKEND_FEATURE_SLICE, SOW-066, OBJ-014.
   - `_STATUS.md` is in a state that permits setup drafting before overwriting local production documents.
2. Confirm governing references:
   - `_REFERENCES.md`
   - `execution/_Decomposition/SOFTWARE_DECOMP.md`
   - `docs/_Registers/Deliverables.csv`
   - `docs/_Registers/ScopeLedger.csv`
   - `docs/_Registers/ContextBudgetQA.csv`
   - `docs/CONTRACT.md`
   - `docs/SPEC.md`
   - `docs/TYPES.md`
   - `docs/IP_AND_DATA_BOUNDARY.md`
   - `INIT.md`
3. Treat `Dependencies.csv` as the approved DAG-002 mirror/evidence surface. Preserve approved rows as ACTIVE; do not retire, delete, or reclassify them during this setup workflow.
4. Use only source-cleared or invented/public-permissive data in examples and tests. Suspected protected content must be quarantined and escalated under project policy.

## Steps

### 1. Establish Transform Boundary

1. Identify the physical model as the source-of-truth input.
2. Identify the analytical model as a derived solver-ready output.
3. Confirm the default target mechanics boundary is the 3D centerline/frame model.
4. Record any physical data category whose analytical representation is unsupported as `TBD` unless a source explicitly defines its handling.

### 2. Define Contract Inputs

1. Declare required input surfaces from source-grounded context:
   - canonical model/domain schema;
   - design knowledge and provenance;
   - constraint entities and constraint validation;
   - support/restraint semantics;
   - primitive load semantics;
   - diagnostics/result-envelope conventions.
2. For each unit-bearing input, require explicit unit metadata unless the field is explicitly dimensionless.
3. For each input whose source, provenance, or units are missing, define a diagnostic/warning path instead of a silent default.
4. Leave exact field names and schema references `TBD` unless fixed by an accessible source.

### 3. Define Contract Outputs

1. Specify the derived analytical model output at the contract level.
2. Specify warning/diagnostic output for non-representable physical data, missing solve-required data, weak provenance, unresolved assumptions, or protected/private-data risk where applicable.
3. Specify traceability links from physical inputs to:
   - analytical output records;
   - warnings/diagnostics;
   - omissions;
   - assumptions;
   - unresolved `TBD` items.
4. Do not encode compliance, approval, certification, or professional reliance status in transform outputs.

### 4. Define Determinism Rules

1. Define deterministic behavior in terms of equivalent input model, units, transform contract version, and configuration.
2. Require repeatable analytical output and repeatable warning/diagnostic output for the same basis.
3. Record sorting, canonicalization, hash, or stable-reference rules as `TBD` unless fixed by the applicable architecture-basis source.

### 5. Define Warning Tests

1. Create tests for transformation warnings required by SOW-066.
2. Include negative cases for at least:
   - missing solve-required physical input;
   - physical data that lacks a solver-ready analytical representation;
   - missing or ambiguous unit metadata on a unit-bearing value.
3. Use invented or permissive fixtures only.
4. Verify warnings remain diagnostic and do not become compliance or professional acceptance claims.

### 6. Review Data Boundary

1. Scan the contract and tests for protected standards text, protected tables, code-specific defaults, copied formulas, proprietary catalog values, owner standards, or private project data.
2. Mark uncertain content `TBD` or `protected_suspected` according to policy.
3. Escalate suspected protected content; do not normalize it into public examples.

## Verification

| Check | Expected evidence |
|---|---|
| Four-doc consistency | `Datasheet.md`, `Specification.md`, `Guidance.md`, and `Procedure.md` use the same DEL ID, package, scope, objective, and terminology. |
| Requirement coverage | Specification requirements trace to SOW-066, OBJ-014, project invariants, or local DAG mirror rows. |
| Determinism | Tests or planned tests cover repeated transform behavior. |
| Warning behavior | Tests or planned tests cover warnings for non-representable physical data. |
| Traceability | Contract records physical-to-analytical links or explicit unresolved gaps. |
| Unit safety | Missing/ambiguous unit metadata produces findings, not defaults. |
| Data boundary | No protected/private/proprietary engineering content is introduced. |
| Professional boundary | Outputs avoid certification, approval, compliance, or professional reliance claims. |
| Dependency preservation | Existing DAG-002 rows in `Dependencies.csv` remain ACTIVE and unchanged during setup. |

## Records

Maintain the following records in the deliverable folder:

- `Datasheet.md`
- `Specification.md`
- `Guidance.md`
- `Procedure.md`
- `_SEMANTIC.md`
- `_SEMANTIC_LENSING.md`
- `_STATUS.md`
- `Dependencies.csv`
- `_DEPENDENCIES.md`

Implementation records remain `TBD` until a sealed Type 2 implementation task defines write scope and acceptance criteria.
