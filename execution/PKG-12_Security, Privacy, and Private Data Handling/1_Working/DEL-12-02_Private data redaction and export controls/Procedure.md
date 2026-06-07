# Procedure: DEL-12-02 Private data redaction and export controls

## Purpose

This procedure describes how to maintain the DEL-12-02 redaction/export-control artifact set after setup and how implementation work should wire the current metadata-only helper, schema contract, focused tests, and security documentation into product report/export behavior without crossing private-data, protected-data, or professional-authority boundaries.

## Prerequisites

| Prerequisite | Required State |
|---|---|
| Sealed deliverable context | DEL-12-02, PKG-12, SOW-040, OBJ-010, explicit write scope |
| Governance sources | `INIT.md`, `AGENTS.md`, `docs/CONTRACT.md`, `docs/TYPES.md`, `docs/SPEC.md`, `docs/IP_AND_DATA_BOUNDARY.md`, and decomposition/register rows read |
| Architecture basis | AB-00-01, AB-00-02, AB-00-03, AB-00-04, AB-00-06, AB-00-07, and AB-00-08 injected as constraints, not copied wholesale |
| Scope boundary | No edits outside this deliverable folder |
| Protected/private data boundary | No real private values, credentials, protected standards content, proprietary report templates, or legal/compliance claims introduced |
| Current evidence basis | June 7 evidence exists for `schemas/redaction_export_controls.schema.yaml`, `core/security/redaction/`, `tests/security/test_redaction_export_controls.py`, `docs/security/redaction_export_controls.md`, and `TASK_RUN_2026-06-07_0935_redaction-export-hardening.md` |

## Steps

| Step | Action | Output |
|---|---|---|
| 1 | Confirm DEL-12-02 identity, scope, objective, invariants, acceptance criteria, and write scope. | `_CONTEXT.md` remains the scope anchor. |
| 2 | Classify export risk surfaces: report preview/export, shared model export, public templates/examples, CLI/API export, adapter/plugin export, and downstream-tool handoff. | `Datasheet.md` attributes and conditions. |
| 3 | Classify sensitive value groups: project data, rule-pack details, material/component/library values, owner/company design basis, protected standards content, proprietary/vendor data, and private report-template content. | `Datasheet.md` and `Guidance.md` private/protected value sections. |
| 4 | Define setup-level redaction mode and export context vocabulary without selecting a concrete config schema. | `Datasheet.md` redaction configuration contract. |
| 5 | Translate redaction/export safeguards into requirements and verification expectations. | `Specification.md` REXC requirements and verification table. |
| 6 | Record implementation guidance, trade-offs, open issues, and human-ruling conflicts. | `Guidance.md` principles, trade-offs, open issues, and conflict table. |
| 7 | Build semantic matrix and lensing artifacts after the four documents exist. | `_SEMANTIC.md` and `_SEMANTIC_LENSING.md`. |
| 8 | Apply P3 lensing by surfacing warranted TBDs, verification gaps, or conflicts only when source evidence supports the edit. | Open issues and verification gaps remain visible. |
| 9 | Extract dependency register rows for anchors and explicit execution information flow. | `Dependencies.csv` and `_DEPENDENCIES.md`. |
| 10 | During the historical setup workflow, run validation checks before any authorized lifecycle update. | Final setup run records. |
| 11 | During readiness-evidence alignment, replace stale setup-only language where current run evidence proves schema/helper/docs/tests exist. | Four-document kit reflects current evidence without lifecycle promotion. |
| 12 | Keep unresolved runtime report/export integration, destructive quarantine movement, legal review, cloud exception workflow, storage roots, UI/CLI/public transport/export-format choices, and approval choices as explicit `TBD` deferrals. | No hidden dependency closure or overclaim. |
| 13 | Refresh dependency and review evidence only where cited upstream statuses, reviews, or run records warrant the change. | `Dependencies.csv`, `_DEPENDENCIES.md`, `_REVIEW.md`, and `Review_Findings.csv` remain evidence-first. |

## Future Implementation Procedure

When a later implementation task is authorized, it should:

1. Wire the existing redaction/export-control schema into a project-local persisted profile store once storage roots and migration behavior are authorized.
2. Add GUI/CLI/API controls that require explicit export context selection before shareable export.
3. Route report generation, model export, adapter/plugin export, and downstream-tool handoff through the same redaction and diagnostic checks.
4. Emit `IP_BOUNDARY_WARNING` and related diagnostics with code, class, severity, source, affected object, message, remediation, and provenance.
5. Preserve unit awareness, provenance, privacy/redistribution status, rule-pack checksums, model/report hashes, and report limitations.
6. Ensure redaction affects only export/report representations and never mutates authoritative project models or private libraries.
7. Extend the focused helper/schema tests into runtime integration tests for public/shared redaction, local-private override, unknown provenance, protected-content linting, adapter/plugin no-bypass behavior, manifest preservation, and source non-mutation.
8. Define quarantine review routing without destructive movement unless a maintainer-approved quarantine workflow is authorized.
9. Preserve professional-boundary notices and avoid certification, sealing, approval, authentication, or code-compliance claims.

## Verification

| Check | Method | Expected Result |
|---|---|---|
| Four-document presence | Confirm `Datasheet.md`, `Specification.md`, `Guidance.md`, and `Procedure.md` exist. | Present. |
| Default sections | Confirm Datasheet has Identification/Attributes/Conditions/Construction/References; Specification has Scope/Requirements/Standards/Verification/Documentation; Guidance has Purpose/Principles/Considerations/Trade-offs/Examples; Procedure has Purpose/Prerequisites/Steps/Verification/Records. | Present. |
| Dependency schema | Run `python3 tools/validation/validate_dependencies_schema.py <deliverable>/Dependencies.csv`. | Valid v3.1 schema. |
| Current implementation evidence | Confirm `schemas/redaction_export_controls.schema.yaml`, `core/security/redaction/`, `tests/security/test_redaction_export_controls.py`, and `docs/security/redaction_export_controls.md` exist. | Present as June 7 evidence. |
| Focused redaction tests | Review `TASK_RUN_2026-06-07_0935_redaction-export-hardening.md` and package fan-in validation. | Focused redaction/local-first tests passed in June 7 evidence. |
| Semantic audit | Confirm `_SEMANTIC.md` contains no `MatrixError`/`MATRIX_ERROR` and no algebra/operator leaks in final result tables. | PASS. |
| Lensing coverage | Count `_SEMANTIC_LENSING.md` lens rows for matrices A, B, C, F, D, X, and E. | 96 required rows. |
| Boundary scan | Search deliverable files for real secrets, protected standards content, real private project values, cloud-operation assumptions, or certification/compliance claims. | No disallowed content found. |
| Lifecycle status | Read `_STATUS.md` without editing it. | Current state remains `IN_PROGRESS`; this procedure does not authorize promotion. |

## Records

The deliverable evidence surface includes these records in the deliverable folder:

- four production documents;
- `_SEMANTIC.md`;
- `_SEMANTIC_LENSING.md`;
- `Dependencies.csv`;
- `_DEPENDENCIES.md`;
- `_run_records/*`;
- `_STATUS.md`.

Do not move any artifact to `ISSUED` during a readiness-evidence alignment run.

Readiness-evidence alignments must also leave product code, schemas, tests, DAG artifacts, coordination files, approval records, release files, package registers, and lifecycle files unchanged unless a separate authorized brief grants that write scope.
