# Datasheet: DEL-07-01 3D viewport and centerline editor

## Identification

| Field | Value |
|---|---|
| Deliverable ID | DEL-07-01 |
| Deliverable name | 3D viewport and centerline editor |
| Package ID | PKG-07 |
| Package name | Graphical User Interface and Engineering Workflow |
| Deliverable type | UX_UI_SLICE |
| Context envelope | L |
| Current execution mode | Setup/document production only |
| Write boundary | This deliverable folder only |

## Attributes

| Attribute | Value | Source |
|---|---|---|
| Primary scope item | SOW-020: GUI shall provide a 3D centerline modeler with model tree and piping component visualization | `docs/_Registers/ScopeLedger.csv` row `SOW-020`; `_CONTEXT.md` |
| Supported objective | OBJ-006: GUI workflow makes model creation, missing data, results, and assumptions visible | `docs/_Decomposition/SOFTWARE_DECOMP.md` section 5; `_CONTEXT.md` |
| Future artifact class | GUI viewport and interaction tests | `docs/_Registers/Deliverables.csv` row `DEL-07-01`; `_CONTEXT.md` |
| Runtime/UI baseline | Tauri 2 desktop shell, TypeScript/React/Vite GUI, Three.js viewport | `_CONTEXT.md` Architecture Basis Injection; `docs/_Decomposition/SOFTWARE_DECOMP.md` `DEC-009` |
| Unresolved implementation choices | Exact dependency versions, component library, state-management library, and platform release matrix remain `TBD` | `_CONTEXT.md`; `docs/_Decomposition/SOFTWARE_DECOMP.md` `DEC-012` |
| State/editing basis | Durable project state is separate from transient viewport, selection, and session state; mutations route through application-service commands | `docs/_Decomposition/SOFTWARE_DECOMP.md` `AB-00-05` |
| Diagnostic basis | User-facing warnings and result-envelope diagnostics use explicit classes and do not claim code compliance | `docs/_Decomposition/SOFTWARE_DECOMP.md` `AB-00-06`; `docs/SPEC.md` section 7 |
| Data boundary posture | Code-specific values, protected standards data, manufacturer/private data, and professional approval remain user/private or human-owned | `docs/CONTRACT.md` `OPS-K-DATA-1`, `OPS-K-IP-1`, `OPS-K-AUTH-1`; `docs/IP_AND_DATA_BOUNDARY.md` |

## Conditions

| Condition | Required handling |
|---|---|
| A viewport interaction would require a physical property, component dimension, SIF, flexibility factor, allowable, or code-specific value | Request user-supplied/provenanced data or mark the value `TBD`; do not introduce public defaults |
| A future component glyph needs protected dimensional tables or vendor catalog data | Stop and route through the protected-data/provenance review path |
| The viewport can create geometry but required solve data is missing | Surface an explicit diagnostic or missing-data state; do not hide the gap |
| A rule-pack or code-check status is implied by viewport color, label, or state | Preserve mechanics-solve, user-rule-check, and human-approval separation |
| A future implementation needs to choose a component or state-management library | Record the decision through the architecture decision path; this setup deliverable does not finalize it |
| A change would edit GUI source, package manifests, repo-level docs, or tests during this setup session | Stop; those paths are outside this sealed write scope |

## Construction

This setup artifact frames a future GUI slice for a 3D centerline viewport and editor. A conforming future implementation should keep these surfaces distinct:

| Surface | Setup expectation |
|---|---|
| Viewport rendering | Render pipe centerlines, bend arcs, branch symbols, and simple piping component symbols using the accepted Three.js viewport baseline |
| Centerline editing | Support creation and editing of nodes and pipe runs through service-command mutations, not direct uncontrolled durable-state edits |
| Selection and identity | Preserve stable model-entity identity for coordination with the model tree and property inspector |
| Units and coordinates | Treat coordinates and editable quantities as unit-aware model data; reject incompatible units through domain/service validation |
| Diagnostics | Carry missing-data, provenance, assumption, nonlinear, and IP-boundary warnings as explicit diagnostics |
| Undo/redo | Scope undo/redo to reversible model edits and preserve diagnostics/result-envelope integrity |
| Test evidence | Use GUI-layer tests appropriate to the accepted Vitest/Playwright baseline once implementation is authorized |

This deliverable does not implement product UI in this setup session. It also does not supply engineering component data, rule-pack values, or professional acceptance records.

## References

| Reference | Used for |
|---|---|
| `_CONTEXT.md` | Deliverable identity, package, scope, objectives, accepted decomposition revision, and architecture basis injection |
| `_REFERENCES.md` | Local source inventory |
| `INIT.md` | Bootstrap boundaries for protected data, missing values, and professional reliance |
| `AGENTS.md` | Type 2 dispatch and write-scope rule |
| `docs/CONTRACT.md` | Invariants for protected content, user-supplied code data, units, diagnostics, privacy, IP, and agent limits |
| `docs/DIRECTIVE.md` | Founding intent, centerline-first model, no silent defaults, and stop rules |
| `docs/TYPES.md` | UX_UI_SLICE type, analysis-status vocabulary, centerline model, and domain object vocabulary |
| `docs/SPEC.md` | GUI requirements, warning classes, architecture layers, and acceptance semantics |
| `docs/PRD.md` | Functional requirements FR-003, FR-013, and GUI requirements section 14 |
| `docs/IP_AND_DATA_BOUNDARY.md` | Public/private data, provenance, and protected-content policy |
| `docs/_Decomposition/SOFTWARE_DECOMP.md` | PKG-07 and DEL-07-01 decomposition context plus SCA-001 architecture basis |
| `docs/_Registers/Deliverables.csv` | Deliverable register row |
| `docs/_Registers/ScopeLedger.csv` | Scope mapping for SOW-020 |
