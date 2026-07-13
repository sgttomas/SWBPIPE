# Datasheet: DEL-00-03 Application service command-query-job model

## Identity
| Field | Value |
|---|---|
| Deliverable ID | DEL-00-03 |
| Package ID | PKG-00 |
| Package | Software Architecture Runway |
| Type | API_CONTRACT |
| Lifecycle target | SEMANTIC_READY before downstream package-level work proceeds |
| Scope items | SOW-058, SOW-062 |
| Objectives | OBJ-013 |
| Anticipated artifacts | docs/architecture/application_services.md; service interface contracts |
| Write boundary | Deliverable-local document kit and semantic artifacts only |

## Purpose
Command, query, job, cancellation, progress, and transaction model connecting user-facing workflows to the domain core.

## Scope Boundary
This deliverable defines service contracts and flow rules only; it does not implement commands, queues, solvers, storage, CLI handlers, or GUI actions.

## Architecture Roles
- Command contract
- query contract
- background job contract
- transaction boundary
- progress and cancellation contract

## Required Source Basis
- `INIT.md` for project bootstrap and data-boundary constraints.
- `docs/CONTRACT.md` for invariants that must be preserved.
- `docs/SPEC.md` for target software layers and architecture vocabulary.
- `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.7 for package and deliverable authority.
- `docs/_Registers/Deliverables.csv` row DEL-00-03 for scope identity.
- `docs/_Registers/ScopeLedger.csv` rows SOW-058, SOW-062 for scope mapping.
- `execution/_Coordination/_COORDINATION.md` for approved DAG-007 graph authority and `SEMANTIC_READY` architecture-basis handling.

## TBD and Human-Ruling Slots
- TBD: Concrete service interface language
- TBD: job runner implementation
- TBD: transaction persistence mechanism
- TBD: cancellation token API shape

## Outputs Expected From This Deliverable
- docs/architecture/application_services.md
- service interface contracts

## Boundary Confirmation
- No product implementation code is authorized by this deliverable.
- No protected standards text, standards tables, code-derived formulas, proprietary values, or vendor-private data are introduced.
- Architecture outputs remain draft/proposal material until accepted by the human project authority.

## D-41 R5 T7 PDU-054 current declaration

Earlier setup-era statements on this surface are retained as historical setup context where applicable; this section is the active current-state declaration. The command-query-job contract and application-service seams are implemented and evidenced by their current schemas, modules, and tests. Current upstream authority is SOFTWARE_DECOMP revision 0.8 with DAG-007 coordination; broader application integration remains bounded by the recorded residuals.
