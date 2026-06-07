# Procedure: DEL-13-03 Constraint validation engine

## Purpose

This procedure describes how to verify the current DEL-13-03 validation artifact set and how later bounded changes should proceed without exceeding the grounded scope. It is not authority for engineering values, protected standards content, release readiness, or professional acceptance claims.

## Prerequisites

| Prerequisite | Basis |
|---|---|
| Deliverable scope and identity loaded from `_CONTEXT.md` and `execution/_Decomposition/SOFTWARE_DECOMP.md`. | Four-documents source hierarchy. |
| Approved local dependency mirror available as `Dependencies.csv`. | `_DEPENDENCIES.md`; `Dependencies.csv`. |
| Architecture basis constraints applied only as applicable dispatch context. | `_CONTEXT.md` Architecture Basis Injection. |
| No hidden owner standards, protected code requirements, proprietary values, or protected tables used as public defaults. | `docs/CONTRACT.md`; `docs/IP_AND_DATA_BOUNDARY.md`. |
| Current validation module and focused tests available. | `core/constraints/validation/engine.py`; `core/constraints/validation/__init__.py`; `tests/test_constraint_validation.py`. |
| Remaining `TBD` items are limited to deferred integration, presentation, engineering, release, and human-acceptance decisions. | `_STATUS.md`; `MEMORY.md`; current evidence refresh. |

## Steps

1. Confirm the current work is scoped to DEL-13-03 and SOW-068.
   - Evidence: `_CONTEXT.md`; `execution/_Decomposition/SOFTWARE_DECOMP.md` DEL-13-03 and SOW-068 rows.

2. Inspect the implemented validation surface.
   - Module: `core/constraints/validation/engine.py`.
   - Package export surface: `core/constraints/validation/__init__.py`.
   - Focused tests and invented fixtures: `tests/test_constraint_validation.py`.

3. Load only available design-knowledge and constraint inputs supplied by accepted schemas or test fixtures.
   - Current API: `validate_constraint_envelope(constraint_envelope, design_knowledge_envelope=None)`.
   - If required schema or fixture details are unavailable, record `TBD` or emit diagnostics rather than inventing fields.

4. Validate the SOW-068 categories when supporting input data exists:
   - connectivity;
   - route conflicts;
   - clearance conflicts;
   - support-zone conflicts;
   - slope/drain/vent conflicts;
   - missing required data.

5. Emit deterministic validation diagnostics for observed findings.
   - Current diagnostic dict fields: `code`, `severity`, `class`, `affected_references`, `source_references`, `message`, and `remediation`.
   - `ValidationResult.to_dict()` also reports `has_blocking_findings`.
   - Messages must preserve source/provenance visibility where available and must not become hidden report prose or agent-only text.

6. Treat missing required data as explicit findings.
   - Do not supply engineering defaults, code-specific values, owner-standard criteria, or proprietary values.

7. Enforce public/private and professional-boundary checks on fixtures, messages, and documentation.
   - Reject or quarantine suspected protected content according to `docs/IP_AND_DATA_BOUNDARY.md`.
   - Do not emit software-generated professional approval, code-compliance, certification, sealing, or authentication statuses.

8. Maintain validation diagnostics tests.
   - Current coverage: deterministic output behavior, represented SOW-068 category handling, missing-data findings, unresolved references, provenance visibility, unit metadata and dimension checks, and boundary protection.
   - Public documentation examples remain `TBD` unless invented or otherwise permitted examples are separately reviewed.

9. Preserve approved DAG-006 dependency evidence.
   - Do not retire, delete, reclassify, or normalize current ACTIVE local dependency rows during this evidence-refresh workflow.

## Verification

| Check | Expected result |
|---|---|
| Scope check | Only files authorized by the active bounded brief are changed; product-code changes remain scoped to DEL-13-03 when a product-code brief authorizes them. |
| Category check | SOW-068 categories are represented in requirements and tests. |
| Determinism check | Repeated validation over deep-copied inputs yields identical `diagnostic_dicts(...)` output. |
| Missing-data check | Required missing values produce explicit findings rather than defaults. |
| Provenance check | Findings retain or reference input provenance through `source_references` where available. |
| Boundary check | No protected standards text, owner defaults, proprietary values, or professional-approval/compliance statuses are introduced. |
| Dependency check | Current dependency evidence remains ACTIVE and unchanged in `Dependencies.csv`; historical dependency IDs are preserved where present under the DAG-006 coordination basis. |
| Syntax check | `python3 -m py_compile core/constraints/validation/engine.py tests/test_constraint_validation.py` completes successfully. |
| Focused test check | `python3 tests/test_constraint_validation.py` completes successfully. |

## Records

- Constraint validation module: `core/constraints/validation/engine.py`.
- Package export surface: `core/constraints/validation/__init__.py`.
- Validation diagnostics tests and invented fixtures: `tests/test_constraint_validation.py`.
- Test fixture provenance/review records: required before public examples are relied on outside executable test evidence.
- `TBD` list for localization, full geometric conflict solving, owner criteria/rules, GUI presentation, physical-to-analytical transformation, runtime integration, release readiness, and human acceptance.
- Local dependency validation result from `tools/validation/validate_dependencies_schema.py` when `Dependencies.csv` exists.
