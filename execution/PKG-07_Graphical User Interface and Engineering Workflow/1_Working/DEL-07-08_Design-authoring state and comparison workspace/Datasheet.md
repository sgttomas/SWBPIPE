# Datasheet: DEL-07-08 Design-authoring state and comparison workspace

## Identification

| Field | Value | Source |
|---|---|---|
| Deliverable ID | DEL-07-08 | `_CONTEXT.md` / Context header |
| Name | Design-authoring state and comparison workspace | `_CONTEXT.md` / Context header |
| Package | PKG-07 Graphical User Interface and Engineering Workflow | `_CONTEXT.md` / Package Reference |
| Type | UX_UI_SLICE | `_CONTEXT.md` / Type |
| Scope item | SOW-076 | `_CONTEXT.md` / Scope Coverage |
| Objectives | OBJ-015, OBJ-016 | `_CONTEXT.md` / Objective Support |
| Context envelope | L | `_CONTEXT.md` / Context Envelope |
| Context risk | WATCH; confirm scope and split if it expands | `_CONTEXT.md` / Context Budget QA |

## Attributes

| Attribute | Current value |
|---|---|
| Intended GUI surfaces | Design knowledge panel; operation diff review; state/run browser; comparison overlays. Source: `_CONTEXT.md` / Anticipated Artifacts. |
| Scope statement | The GUI shall support design-authoring and comparison workflows, including design knowledge panels, constraint/warning panels, state/run browsers, comparison tables, and graphical comparison overlays. Source: `execution/_Decomposition/SOFTWARE_DECOMP.md` / SOW-076. |
| GUI-facing stack basis | Rust core/application services, Tauri 2 desktop shell where GUI-facing, TypeScript/React/Vite GUI where GUI-facing, and Three.js viewport where 3D viewport-facing. Source: `_CONTEXT.md` / Architecture Basis Injection. |
| Exact dependency versions | TBD. Source: `_CONTEXT.md` / Architecture Basis Injection / Still TBD. |
| Mutation boundary | GUI mutations route through application-service commands; durable project state remains separated from transient session, viewport, selection, and job-progress state. Source: `execution/_Decomposition/SOFTWARE_DECOMP.md` / AB-00-05. |
| Diagnostic boundary | Diagnostics and result envelopes carry code, class, severity, source, affected object, message, remediation, and provenance; outputs must not claim certification or compliance. Source: `execution/_Decomposition/SOFTWARE_DECOMP.md` / AB-00-06. |
| Data boundary | Missing solve-required or rule-check-required values are explicit findings and not silent defaults. Source: `docs/CONTRACT.md` / Invariant index. |
| Professional boundary | Software and agents must not claim to certify, seal, approve, authenticate, or declare engineering code compliance for reliance. Source: `docs/CONTRACT.md` / Invariant index. |

## Conditions

| Condition | Status |
|---|---|
| Upstream dependency mirror | `Dependencies.csv` contains 21 rows; all are ACTIVE approved DAG-002 mirror rows. Source: `Dependencies.csv` and `_DEPENDENCIES.md` / Generated Dependency Register. |
| Architecture basis dependencies | AB-00-01, AB-00-02, AB-00-03, AB-00-05, AB-00-06, AB-00-07, and AB-00-08 apply as dispatchable constraints. Source: `_CONTEXT.md` / Architecture Basis Injection. |
| GUI predecessor dependencies | DEL-07-01, DEL-07-02, DEL-07-04, and DEL-07-05 are declared upstream GUI foundations. Source: `Dependencies.csv` rows DAG-002-E0840 through DAG-002-E0843. |
| Design/constraint dependencies | DEL-13-01, DEL-13-03, and DEL-13-04 are declared upstream design-knowledge, constraint-validation, and transformation inputs. Source: `Dependencies.csv` rows DAG-002-E0844 through DAG-002-E0846. |
| State/comparison dependencies | DEL-14-01, DEL-14-03, DEL-14-04, and DEL-14-05 are declared upstream state and comparison inputs. Source: `Dependencies.csv` rows DAG-002-E0847 through DAG-002-E0850. |
| Operation workflow dependencies | DEL-16-01, DEL-16-02, and DEL-16-03 are declared upstream operation schema, diff preview, and audit inputs. Source: `Dependencies.csv` rows DAG-002-E0851 through DAG-002-E0853. |
| Frontend implementation evidence | TBD; this folder contains setup documents only at this workflow stage. Source: local folder inspection. |

## Construction

This deliverable is a GUI workflow slice. The expected construction surface is a workspace that composes design knowledge, warnings, operation/diff review, state/run browsing, comparison tables, and graphical comparison overlays without owning the backend schemas or engines that produce those records.

The workspace must consume upstream contracts rather than invent placeholder semantics. The approved DAG-002 edge review states that DEL-07-08 should consume existing GUI foundations plus initial design, transform, comparison, and operation contracts. Source: `execution/_DAG/DAG-002/DAG-002_EdgeDispositionReview.md` / DAG2-RD-015.

Implementation details not supported by local evidence remain TBD, including concrete component hierarchy, state-management library, route names, data-fetching library, visual encoding rules, keyboard shortcuts, and Playwright coverage thresholds.

## References

| Reference | Use in this datasheet |
|---|---|
| `_CONTEXT.md` | Deliverable identity, scope, objectives, package, architecture basis, context budget. |
| `_REFERENCES.md` | Local reference index. |
| `_DEPENDENCIES.md` and `Dependencies.csv` | Approved local DAG-002 mirror and dependency evidence surface. |
| `execution/_Decomposition/SOFTWARE_DECOMP.md` | SOW-076, OBJ-015, OBJ-016, PKG-07 deliverable row, architecture-basis constraints. |
| `docs/CONTRACT.md` | Invariants for data boundary, no silent defaults, professional boundary, and agent drafting limits. |
| `docs/SPEC.md` | GUI warning classes, application-service boundaries, persistence/hash boundaries, result/export/report boundary constraints. |
| `docs/TYPES.md` | UX_UI_SLICE type and relevant vocabulary for viewport sessions, diagnostics, states, reports, and professional boundary. |
| `docs/IP_AND_DATA_BOUNDARY.md` | Public/private data and protected-content boundary. |
