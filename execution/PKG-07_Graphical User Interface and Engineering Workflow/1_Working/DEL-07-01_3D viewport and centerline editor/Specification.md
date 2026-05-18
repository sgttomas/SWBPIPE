# Specification: DEL-07-01 3D viewport and centerline editor

## Scope

This deliverable defines setup documentation for the future 3D viewport and centerline editor slice. It is limited to the local `DEL-07-01` working folder and does not create or modify GUI source files, interaction tests, package manifests, dependency manifests, or repo-level documentation.

The deliverable covers:

- the future viewport/editor responsibilities for nodes, pipe runs, bends, and simple component symbols;
- the accepted runtime/UI and architecture-basis constraints applicable to this GUI slice;
- data-boundary, unit-safety, diagnostic, privacy, and professional-responsibility constraints;
- setup evidence required before future implementation work begins.

The deliverable excludes:

- implementation of React, Tauri, Three.js, or state-management code;
- final selection of unresolved component/state libraries or exact dependency versions;
- model tree, property inspector, editor panels, solve execution UX, or results viewer behavior owned by adjacent PKG-07 deliverables;
- any protected standards data, code-specific defaults, proprietary component dimensions, or professional approval workflow.

## Requirements

| Requirement ID | Requirement | Source |
|---|---|---|
| DEL-07-01-REQ-01 | The future implementation shall provide a 3D centerline viewport/editor for creating and editing nodes, pipe runs, bends, and simple piping component symbols. | `docs/_Registers/Deliverables.csv` row `DEL-07-01`; `docs/PRD.md` FR-003 and FR-013 |
| DEL-07-01-REQ-02 | The viewport shall represent the visual categories listed for the 3D viewport where applicable to this slice: pipe centerlines, bend arcs, branch symbols, valves, flanges, reducers, expansion joints, supports, labels, load vectors, deformed shapes, reaction arrows, and stress-ratio color maps. Items beyond initial centerline editing may remain deferred to adjacent result/component slices. | `docs/PRD.md` section 14.2; `_CONTEXT.md` description |
| DEL-07-01-REQ-03 | The setup deliverable shall preserve the accepted runtime baseline of Tauri 2, TypeScript/React/Vite, and Three.js while keeping exact dependency versions and component/state libraries `TBD`. | `_CONTEXT.md` Architecture Basis Injection; `docs/_Decomposition/SOFTWARE_DECOMP.md` `DEC-009`, `DEC-012` |
| DEL-07-01-REQ-04 | Viewport model mutations shall route through application-service commands and shall keep durable project state separate from transient viewport, selection, camera, and interaction state. | `docs/_Decomposition/SOFTWARE_DECOMP.md` `AB-00-03`, `AB-00-05` |
| DEL-07-01-REQ-05 | Coordinates, dimensions, component references, loads, and editable quantities exposed through the viewport shall be unit-aware and validated by domain/service contracts rather than silently coerced. | `docs/CONTRACT.md` `OPS-K-UNIT-1`; `docs/SPEC.md` sections 1 and 3 |
| DEL-07-01-REQ-06 | The viewport/editor shall not supply protected standards data, code-specific defaults, component dimensional tables, SIF/flexibility values, allowables, or proprietary manufacturer data. Missing values shall remain explicit findings or `TBD`. | `docs/CONTRACT.md` `OPS-K-IP-1`, `OPS-K-DATA-1`, `OPS-K-DATA-2`; `docs/IP_AND_DATA_BOUNDARY.md` sections 2-6 |
| DEL-07-01-REQ-07 | Viewport diagnostics shall preserve warning classes and result-envelope boundaries, including solve-blocking, rule-check-blocking, provenance, assumption, nonlinear, and IP-boundary warnings. | `docs/SPEC.md` section 7; `docs/_Decomposition/SOFTWARE_DECOMP.md` `AB-00-06` |
| DEL-07-01-REQ-08 | Visual status, labels, colors, and interaction states shall not claim certification, sealing, approval, authentication, or engineering code compliance for reliance. | `docs/CONTRACT.md` `OPS-K-AUTH-1`; `docs/TYPES.md` sections 4 and 6 |
| DEL-07-01-REQ-09 | Future tests for the implemented slice shall use the accepted GUI test baseline and verify viewport rendering, selection/editing commands, missing-data visibility, and no-protected-data defaults. Exact test harness details beyond the baseline remain `TBD`. | `docs/_Decomposition/SOFTWARE_DECOMP.md` `AB-00-08`; `docs/PRD.md` section 21 |
| DEL-07-01-REQ-10 | This setup run shall write only deliverable-local setup artifacts and shall not move any artifact to `ISSUED`. | `AGENTS.md` dispatch rule; `docs/AGENTIC_DEVELOPMENT_WORKFLOW.md` section 4 |

## Standards

No external engineering code, standards-body text, dimensional table, component catalog, allowable table, SIF/flexibility table, or proprietary commercial data is used as an authority for this setup deliverable.

| Standard or governing source | Status |
|---|---|
| OpenPipeStress governance and invariant documents | Accessible local governing source |
| PRD GUI requirements | Accessible local product source |
| SCA-001 architecture basis | Accepted downstream dispatch basis, not `ISSUED` product implementation |
| Exact GUI dependency versions | `TBD`; future implementation decision |
| Component and state-management libraries | `TBD`; future implementation decision |
| Engineering code values and component catalogs | User/private or legally imported only; not supplied by this deliverable |

## Verification

| Verification ID | Verifies | Method |
|---|---|---|
| DEL-07-01-VER-01 | Four-document kit exists locally | Run `tools/validation/check_four_documents.sh` on this deliverable folder |
| DEL-07-01-VER-02 | Semantic setup artifacts exist | Confirm `_SEMANTIC.md`, `_SEMANTIC_LENSING.md`, and `_run_records/*` exist locally |
| DEL-07-01-VER-03 | Dependency register is schema-valid | Run `python3 tools/validation/validate_dependencies_schema.py` on local `Dependencies.csv` |
| DEL-07-01-VER-04 | Dependency enums are canonical | Run `python3 tools/validation/validate_enum.py` for emitted enum values |
| DEL-07-01-VER-05 | Setup stayed inside the assigned write scope | Check git status and changed paths for this deliverable folder only |
| DEL-07-01-VER-06 | Protected-data and professional boundaries are visible | Review the setup documents for no-protected-content, no-silent-default, and no-certification language |
| DEL-07-01-VER-07 | Unresolved implementation choices remain unresolved | Confirm exact dependency versions and component/state libraries are marked `TBD` rather than finalized |

## Documentation

Required setup artifacts for this deliverable are:

- `Datasheet.md`
- `Specification.md`
- `Guidance.md`
- `Procedure.md`
- `_SEMANTIC.md`
- `_SEMANTIC_LENSING.md`
- `Dependencies.csv`
- `_DEPENDENCIES.md`
- `_run_records/*`
- `_STATUS.md`

Future implementation artifacts such as GUI source and interaction tests remain outside this session's write scope.
