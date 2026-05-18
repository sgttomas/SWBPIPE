# Procedure: DEL-12-01 Local-first storage and private data paths

## Purpose

This procedure describes how to maintain the DEL-12-01 storage-boundary artifact set during setup and how future implementation work should convert it into code or tests without crossing the private-data, protected-data, or cloud-assumption boundaries.

## Prerequisites

| Prerequisite | Required State |
|---|---|
| Sealed deliverable context | DEL-12-01, PKG-12, SOW-029, OBJ-010, explicit write scope |
| Governance sources | `INIT.md`, `AGENTS.md`, `docs/CONTRACT.md`, `docs/TYPES.md`, `docs/SPEC.md`, and decomposition/register rows read |
| Architecture basis | AB-00-04 persistence baseline and related AB-00 items injected as constraints, not copied wholesale |
| Scope boundary | No edits outside this deliverable folder |
| Protected/private data boundary | No real private values, credentials, protected standards content, or proprietary data introduced |

## Steps

| Step | Action | Output |
|---|---|---|
| 1 | Confirm DEL-12-01 identity, scope, objective, invariants, and write scope. | `_CONTEXT.md` remains the scope anchor. |
| 2 | Classify storage-relevant private data classes: project models, private rule packs, material data, component data, owner/code data, report exports. | `Datasheet.md` private data class table. |
| 3 | Define symbolic path classes rather than physical directories or package/container choices. | `Datasheet.md` symbolic path class table. |
| 4 | Translate local-first and private-boundary constraints into requirements. | `Specification.md` LFSP requirements and verification table. |
| 5 | Record implementation guidance, trade-offs, and open issues without deciding the physical container. | `Guidance.md` principles, trade-offs, open issues, and conflict table. |
| 6 | Build semantic matrix and lensing artifacts after the four documents exist. | `_SEMANTIC.md` and `_SEMANTIC_LENSING.md`. |
| 7 | Apply P3 lensing by surfacing warranted TBDs or gaps in the four documents only when source evidence supports the edit. | Open issues and verification gaps are visible. |
| 8 | Extract dependency register rows for anchors and explicit execution information flow. | `Dependencies.csv` and `_DEPENDENCIES.md`. |
| 9 | Run validation checks and update `_STATUS.md` to `SEMANTIC_READY` only if setup gates pass. | Final status and run records. |

## Future Implementation Procedure

When a later implementation task is authorized, it should:

1. Preserve the symbolic path classes unless a human-approved design decision replaces them.
2. Select OS-specific roots and/or physical project packaging only through the appropriate architecture or storage deliverable.
3. Keep private data outside default public repository paths.
4. Add repo-leakage, path-resolution, serialization round-trip, provenance, migration, and report/export boundary tests.
5. Verify no plugin, adapter, import, export, or private library path bypasses validation, units, provenance, diagnostics, sandboxing, or public/private boundary controls.
6. Preserve `TBD` or warning status for unresolved storage decisions.

## Verification

| Check | Method | Expected Result |
|---|---|---|
| Four-document presence | Confirm `Datasheet.md`, `Specification.md`, `Guidance.md`, and `Procedure.md` exist. | Present. |
| Dependency schema | Run `python3 tools/validation/validate_dependencies_schema.py <deliverable>/Dependencies.csv`. | Valid v3.1 schema. |
| Enum spot checks | Run `python3 tools/validation/validate_enum.py` against dependency enum values used in the register. | Valid enums. |
| Semantic audit | Confirm `_SEMANTIC.md` contains `Audit: PASS`. | PASS. |
| Lensing coverage | Count `_SEMANTIC_LENSING.md` lens rows for matrices A, B, C, F, D, X, and E. | 96 rows. |
| Boundary scan | Search deliverable files for real secrets, hidden cloud defaults, protected standards content, or certification claims. | No disallowed content found. |
| Lifecycle status | Read `_STATUS.md`. | `Current State: SEMANTIC_READY` only after the checks pass. |

## Records

The setup run shall leave these records in the deliverable folder:

- four production documents;
- `_SEMANTIC.md`;
- `_SEMANTIC_LENSING.md`;
- `Dependencies.csv`;
- `_DEPENDENCIES.md`;
- `_run_records/*`;
- `_STATUS.md`.

Do not move any artifact to `ISSUED` during this setup run.
