# Procedure: DEL-13-03 Constraint validation engine

## Purpose

This procedure describes how later sealed execution should produce and verify the DEL-13-03 artifact set without exceeding the currently grounded scope. It is not product code and does not authorize engineering values, protected standards content, or professional acceptance claims.

## Prerequisites

| Prerequisite | Basis |
|---|---|
| Deliverable scope and identity loaded from `_CONTEXT.md` and `execution/_Decomposition/SOFTWARE_DECOMP.md`. | Four-documents source hierarchy. |
| Approved local dependency mirror available as `Dependencies.csv`. | `_DEPENDENCIES.md`; `Dependencies.csv`. |
| Architecture basis constraints applied only as applicable dispatch context. | `_CONTEXT.md` Architecture Basis Injection. |
| No hidden owner standards, protected code requirements, proprietary values, or protected tables used as public defaults. | `docs/CONTRACT.md`; `docs/IP_AND_DATA_BOUNDARY.md`. |
| Exact constraint diagnostic schema, module path, fixture shape, and message ID scheme resolved or explicitly marked `TBD`. | Accessible sources do not define these details. |

## Steps

1. Confirm the current work is scoped to DEL-13-03 and SOW-068.
   - Evidence: `_CONTEXT.md`; `execution/_Decomposition/SOFTWARE_DECOMP.md` DEL-13-03 and SOW-068 rows.

2. Load only available design-knowledge and constraint inputs supplied by accepted schemas or test fixtures.
   - ASSUMPTION: These inputs will come from upstream DEL-13-01 and DEL-13-02 outputs, but this setup pass does not inspect those sibling folders.
   - If required schema or fixture details are unavailable, record `TBD` rather than inventing fields.

3. Validate the SOW-068 categories when supporting input data exists:
   - connectivity;
   - route conflicts;
   - clearance conflicts;
   - support-zone conflicts;
   - slope/drain/vent conflicts;
   - missing required data.

4. Emit deterministic validation messages for observed findings.
   - The exact diagnostic record shape is TBD.
   - Messages must preserve source/provenance visibility where available and must not become hidden report prose or agent-only text.

5. Treat missing required data as explicit findings.
   - Do not supply engineering defaults, code-specific values, owner-standard criteria, or proprietary values.

6. Enforce public/private and professional-boundary checks on fixtures, messages, and documentation.
   - Reject or quarantine suspected protected content according to `docs/IP_AND_DATA_BOUNDARY.md`.
   - Do not emit software-generated professional approval, code-compliance, certification, sealing, or authentication statuses.

7. Add validation diagnostics tests.
   - Required coverage: deterministic output behavior, SOW-068 category handling, missing-data findings, provenance visibility, and boundary protection.
   - Exact fixtures and test harness details remain TBD unless resolved by a sealed implementation brief or human ruling.

8. Preserve approved DAG-002 dependency mirror rows.
   - Do not retire, delete, reclassify, or normalize approved ACTIVE mirror rows during this setup workflow.

## Verification

| Check | Expected result |
|---|---|
| Scope check | Only DEL-13-03 production artifacts are changed. |
| Category check | SOW-068 categories are represented in requirements and tests. |
| Determinism check | Repeated validation over identical inputs yields the same messages under the accepted comparison basis. Exact basis TBD. |
| Missing-data check | Required missing values produce explicit findings rather than defaults. |
| Provenance check | Findings retain or reference input provenance where available. Exact fields TBD. |
| Boundary check | No protected standards text, owner defaults, proprietary values, or professional-approval/compliance statuses are introduced. |
| Dependency check | Approved DAG-002 mirror rows remain ACTIVE and unchanged in `Dependencies.csv`. |

## Records

- Constraint validation module: required, path TBD.
- Validation diagnostics tests: required, paths TBD.
- Test fixture provenance/review records: required before public examples are relied on.
- `TBD` list for unresolved schema, module, diagnostic, severity, fixture, and comparison details.
- Local dependency validation result from `tools/validation/validate_dependencies_schema.py` when `Dependencies.csv` exists.
