---
doc_id: DEL-10-01-PROCEDURE
doc_kind: deliverable.procedure
status: draft
created: 2026-04-30
deliverable_id: DEL-10-01
package_id: PKG-10
---

# Procedure: Public API and Plugin Boundary

## Purpose

Use this procedure to produce, review, and later refine the DEL-10-01 public API/plugin boundary without crossing into product implementation, external transport selection, plugin runtime implementation, repository-level API files, source code, package manifests, or protected/private data.

## Prerequisites

- The sealed DEL-10-01 context is available in `_CONTEXT.md`.
- Governing documents and registers named in `_REFERENCES.md` have been read.
- Applicable architecture basis IDs AB-00-02, AB-00-03, AB-00-04, AB-00-06, AB-00-07, and AB-00-08 are treated as dispatch constraints, not copied as full PKG-00 authority.
- Current setup write scope is restricted to this deliverable folder.
- Public transport, plugin runtime, external format list, rule expression grammar/library, code-generation tooling, and repository-level `api/openapi.yaml` placement remain TBD.

## Steps

1. Confirm identity and scope:
   - Verify the deliverable is `DEL-10-01` under `PKG-10`.
   - Confirm scope item SOW-030 and objective OBJ-009.
   - Confirm no edit target is outside this deliverable folder.
2. Inventory public API operation families:
   - Model creation/import/export.
   - Load-case definition.
   - Solve execution.
   - Result extraction.
   - Rule-pack evaluation.
   - Report generation.
   - Validation-test execution.
3. Classify operation semantics:
   - Mutating operations become commands.
   - Read-only operations become queries.
   - Long-running solve/report/export/validation operations become jobs with progress, cancellation, reproducibility metadata, and final result envelopes.
4. Define mandatory envelope slots at concept level:
   - Schema/envelope version.
   - Operation family and command/query/job classification.
   - Units and dimensional metadata where applicable.
   - Provenance and redistribution/private-public metadata where applicable.
   - Diagnostics with code/class/severity/source/affected object/message/remediation/provenance where applicable.
   - Result status using project analysis-status vocabulary.
   - Reproducibility metadata such as payload hash, model hash, solver version, rule-pack checksum, and input manifest references where applicable.
5. Define no-bypass rules:
   - Adapters/plugins cannot bypass schema validation, unit checks, provenance checks, public/private data controls, diagnostics, rule sandboxing, result envelopes, report controls, or protected-content gates.
   - Direct writes into domain core, solver state, storage internals, private libraries, or reports are not authorized by this boundary.
6. Define plugin manifest concepts without approving concrete fields:
   - Plugin identity/version.
   - Declared extension point.
   - Compatible schema/envelope version.
   - Requested capabilities.
   - Private-data access posture.
   - Diagnostics compatibility.
   - Review/status metadata.
   - Rule-pack hook posture where applicable.
7. Preserve unresolved decisions:
   - Mark public transport, endpoint syntax, plugin runtime, permission taxonomy, import/export formats, code generation, and repository-level API file layout as `TBD`.
   - Do not choose defaults without human/project authority evidence.
8. Check protected-data and authority boundaries:
   - Confirm no protected standards text, tables, copied formulas, proprietary vendor data, private project data, or private rule-pack content is embedded.
   - Confirm no wording claims certification, sealing, approval, endorsement, or code compliance.
9. Produce setup artifacts:
   - Four-document kit.
   - `_SEMANTIC.md`.
   - `_SEMANTIC_LENSING.md`.
   - `Dependencies.csv`.
   - `_DEPENDENCIES.md`.
   - `_run_records/*`.
   - `_STATUS.md` with `SEMANTIC_READY` only after setup gates pass.

## Verification

| Check | Expected result |
|---|---|
| Write-scope check | Only files in the DEL-10-01 folder changed. |
| Four-document check | Datasheet, Specification, Guidance, and Procedure exist with default sections. |
| Boundary check | No transport, plugin runtime, source code, package manifest, repository-level API file, or external format implementation is introduced. |
| Protected-data check | No protected standards content, proprietary commercial data, or private project/rule/component/material data is introduced. |
| Professional-boundary check | No certification, sealing, approval, endorsement, or code-compliance claim appears. |
| Semantic check | `_SEMANTIC.md` and `_SEMANTIC_LENSING.md` exist and preserve lens-not-authority separation. |
| Dependency check | `Dependencies.csv` validates against v3.1 schema and canonical enum values. |

## Records

- `Datasheet.md`
- `Specification.md`
- `Guidance.md`
- `Procedure.md`
- `_SEMANTIC.md`
- `_SEMANTIC_LENSING.md`
- `Dependencies.csv`
- `_DEPENDENCIES.md`
- `_run_records/TASK_RUN_*.md`
- `_STATUS.md`

## Completion Condition

The setup sequence is complete when the required setup artifacts exist, dependency validation passes, semantic/lensing artifacts are internally consistent, unresolved decisions remain visible as `TBD`, protected-data/professional-boundary checks are clean, and `_STATUS.md` records `SEMANTIC_READY` without any `ISSUED` transition.
