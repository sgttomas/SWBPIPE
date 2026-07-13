# Procedure: DEL-12-01 Local-first storage and private data paths

## Purpose

This procedure describes how to maintain the DEL-12-01 storage-boundary artifact set, align it with implementation evidence, and guide future runtime storage work without crossing the private-data, protected-data, cloud-service-default, secret-handling, or authority-claim boundaries.

## Prerequisites

| Prerequisite | Required State |
|---|---|
| Sealed deliverable context | DEL-12-01, PKG-12, SOW-029, OBJ-010, explicit write scope |
| Governance sources | `INIT.md`, `AGENTS.md`, `docs/CONTRACT.md`, `docs/TYPES.md`, `docs/SPEC.md`, and decomposition/register rows read |
| Architecture basis | AB-00-04 persistence baseline and related AB-00 items injected as constraints, not copied wholesale |
| Scope boundary | No edits outside this deliverable folder |
| Protected/private data boundary | No real private values, credentials, protected standards content, or proprietary data introduced |
| Current metadata-guard evidence | 2026-06-07 run records identify `core/security/local_first_storage/`, `docs/security/local_first_storage_policy.md`, and `tests/security/test_local_first_storage_policy.py` as metadata-only guard evidence |

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
| 10 | For later readiness-evidence alignment, replace stale setup-only language only where run records, product docs, code, or tests prove new evidence exists. | The four documents distinguish current metadata-only guard evidence from deferred runtime storage and approval decisions. |

## Future Implementation Procedure

When a later implementation task is authorized, it should:

1. Preserve the symbolic path classes unless a human-approved design decision replaces them.
2. Select OS-specific roots and/or physical project packaging only through the appropriate architecture or storage deliverable.
3. Keep private data outside default public repository paths.
4. Preserve the existing metadata-only guard tests and add repo-leakage, path-resolution, serialization round-trip, provenance, migration, and report/export boundary tests for any future runtime storage behavior.
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
| June 7 metadata-guard evidence | Confirm `core/security/local_first_storage/`, `docs/security/local_first_storage_policy.md`, `tests/security/test_local_first_storage_policy.py`, `TASK_RUN_2026-06-07_0140.md`, and package fan-in evidence exist. | Present; evidence is metadata-only and side-effect-free. |
| Boundary scan | Search deliverable files for real secrets, hidden cloud defaults, protected standards content, approval claims, professional claims, code-compliance claims, or security-certification claims. | No disallowed content found. |
| Lifecycle status | Read `_STATUS.md` without editing it. | This alignment does not promote lifecycle state; later lifecycle transitions require human-gated workflow authority. |

## Records

The original setup workflow left these records in the deliverable folder:

- four production documents;
- `_SEMANTIC.md`;
- `_SEMANTIC_LENSING.md`;
- `Dependencies.csv`;
- `_DEPENDENCIES.md`;
- `_run_records/*`;
- `_STATUS.md`.

The 2026-06-07 readiness evidence also cites product-level artifacts outside this deliverable folder:

- `docs/security/local_first_storage_policy.md`;
- `core/security/local_first_storage/`;
- `tests/security/test_local_first_storage_policy.py`.

Do not move any artifact to `ISSUED` during setup or readiness-evidence alignment. Do not use this procedure to approve runtime storage, cloud behavior, encryption/key management, real secret storage, professional reliance, code compliance, security certification, or release readiness.

For PDU-036, classify the transform trace-gap fixture as adjacent verification evidence only. Re-run LFSP-REQ-011 only when the actual runtime path, package/migration, and report/export owners supply their bounded implementations and fixtures.
