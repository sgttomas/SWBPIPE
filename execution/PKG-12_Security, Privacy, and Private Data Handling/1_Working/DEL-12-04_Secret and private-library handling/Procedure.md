# Procedure: DEL-12-04 Secret and private-library handling

## Purpose

Use this procedure to produce or review the DEL-12-04 setup artifacts and to guide later implementation of private library registry and secret handling tests. The procedure is local-first, source-bound, and limited to the assigned deliverable.

## Prerequisites

- Read `_CONTEXT.md`, `_REFERENCES.md`, `_DEPENDENCIES.md`, and `_STATUS.md`.
- Read `INIT.md`, `AGENTS.md`, `docs/CONTRACT.md`, `docs/_Decomposition/SOFTWARE_DECOMP.md` revision 0.4, and the relevant register rows for `DEL-12-04`, `SOW-040`, `SOW-029`, and `OBJ-010`.
- Confirm write scope is limited to this deliverable folder.
- Confirm no real secrets, real private libraries, protected standards text, proprietary values, credential examples, or private project data are needed.
- Treat exact secret provider, encrypted-storage default, physical project package/container, permission grant storage, and public API transport as `TBD` unless a later human-approved brief resolves them.

Sources: TASK brief; `_CONTEXT.md`; `docs/DIRECTIVE.md` section 5; `docs/_Decomposition/SOFTWARE_DECOMP.md` section 8.

## Steps

1. Establish the private asset boundary.
   - Use SOW-029 and SOW-040 to identify protected assets: private rule packs, private material data, private component data, private library references, project models, private paths, and credential references.
   - Verify that no cloud assumption is introduced.

2. Define the registry metadata surface.
   - Record privacy classification, provenance/source summary, redistribution status, review status, checksum status, path reference, credential reference if needed, and default transmission posture.
   - Mark exact schema file placement and storage-provider mechanics as `TBD` unless separately approved.

3. Define secret-reference behavior.
   - Persist only opaque credential references or non-sensitive test markers.
   - Reject or flag artifacts that appear to contain usable credential material.
   - Do not include credential-like examples in docs, fixtures, reports, or run records.

4. Define boundary controls.
   - For import, export, report, bug-report, telemetry, plugin, adapter, and public contribution paths, require explicit privacy/provenance checks.
   - Default to warning, redaction, omission, denied access, quarantine, or human review when classification is missing or suspicious.

5. Define tests.
   - Add tests for metadata completeness, no-secret persistence, redaction/export behavior, telemetry exclusion, denied-by-default plugin access, and quarantine routing.
   - Use non-sensitive sentinel markers only.

6. Check no-bypass constraints.
   - Confirm adapters/plugins cannot skip schema validation, units, provenance, privacy, protected-content screening, diagnostics, checksums, rule sandboxing, or report controls.

7. Record unresolved human rulings.
   - If provider selection, encryption default, permission grant persistence, or transport behavior is required, record it as `TBD` for a future human-approved implementation brief.

8. Finalize deliverable-local setup artifacts.
   - Ensure four documents exist and keep default sections.
   - Ensure `_SEMANTIC.md`, `_SEMANTIC_LENSING.md`, `Dependencies.csv`, and `_DEPENDENCIES.md` are refreshed.
   - Ensure `_STATUS.md` remains `SEMANTIC_READY`, not `ISSUED`.
   - Add phase run records under `_run_records`.

## Verification

| Check | Pass condition |
|---|---|
| Scope containment | All changed files are inside the DEL-12-04 folder. |
| Four-document sections | Datasheet, Specification, Guidance, and Procedure preserve their default sections. |
| Source boundary | No real secrets, real private libraries, private project data, protected standards content, or usable credential examples are present. |
| Semantic readiness | `_SEMANTIC.md` has complete final result tables without matrix errors or operator leakage in final cells. |
| Lensing coverage | `_SEMANTIC_LENSING.md` includes A/B/C/F/D/X/E coverage. |
| Dependency schema | `python3 tools/validation/validate_dependencies_schema.py <ScopePath>/Dependencies.csv` returns valid. |
| Status | `_STATUS.md` Current State is `SEMANTIC_READY`, not `ISSUED`. |

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
- `_run_records/TASK_RUN_2026-04-30_1430_four-documents-P1_P2.md`
- `_run_records/TASK_RUN_2026-04-30_1431_semantic-matrix-build.md`
- `_run_records/TASK_RUN_2026-04-30_1432_lens-register.md`
- `_run_records/TASK_RUN_2026-04-30_1433_four-documents-P3_ONLY.md`
- `_run_records/TASK_RUN_2026-04-30_1434_dependency-extract.md`
- `_run_records/TASK_RUN_2026-04-30_1435_validation.md`
