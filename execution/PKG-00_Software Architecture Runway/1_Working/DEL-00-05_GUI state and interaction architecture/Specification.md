# Specification: DEL-00-05 GUI state and interaction architecture

## Normative Scope
This specification governs only `DEL-00-05` inside `PKG-00 - Software Architecture Runway`. It defines architecture documentation obligations and acceptance evidence. It does not authorize implementation work in `PKG-01` through `PKG-12`.

## Requirements
| ID | Requirement | Evidence |
|---|---|---|
| REQ-05-01 | Separate durable project state from transient session, viewport, selection, and job-progress state. | Acceptance review |
| REQ-05-02 | Route model-changing GUI actions through application-service commands rather than direct domain mutation. | Acceptance review |
| REQ-05-03 | Require undo/redo records to be scoped to reversible model edits and to preserve diagnostics when an edit changes solve readiness. | Acceptance review |
| REQ-05-04 | Define framework-agnostic selection and interaction architecture; model-tree and property-inspector behavior is delegated to `DEL-07-02`, which consumes this architecture basis. | Acceptance review; D-41 `DEC-074` O2 delegation record |
| REQ-05-05 | Require GUI warnings to preserve solve-blocking, rule-check-blocking, provenance, assumption, nonlinear, and IP-boundary classes. | Human review |

## Acceptance Criteria
- Datasheet.md, Specification.md, Guidance.md, and Procedure.md exist and cite the deliverable identity.
- All scope statements remain limited to PKG-00 architecture-runway work.
- TBD decisions are visible and routed to human ruling rather than silently selected.
- No implementation code, GUI screens, schemas, tests, or production package work are created.
- No protected standards/code data or proprietary engineering values are introduced.
- The semantic lens and lensing register exist for review and do not claim engineering authority.

## Required Invariants
- `OPS-K-IP-1`: Public artifacts must not contain protected standards text, tables, figures, copied formulas, material allowables, SIF/flexibility tables, protected dimensional tables, or proprietary commercial data.
- `OPS-K-DATA-2`: Missing solve-required or rule-check-required values remain explicit findings, never silent defaults.
- `OPS-K-AUTH-1`: Software and agents must not claim to certify, seal, approve, authenticate, or declare engineering code compliance for reliance.
- `OPS-K-MECH-1`: Global analysis architecture remains a 3D centerline/frame model; local FEA is a handoff path.
- `OPS-K-AGENT-1`: Unknown engineering or architecture facts become `TBD`.
- `OPS-K-AGENT-3`: Type 2 execution stays within sealed deliverable scope.

## Interface Commitments
- Upstream authority is `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.7, the SCA-001/SCA-003/SCA-004 architecture-basis records, and approved `execution/_DAG/DAG-007/` graph authority.
- Downstream consumers may use this deliverable as AB-00-05 architecture-basis context only through sealed briefs and governed review/dispatch surfaces; this does not make PKG-00 ISSUED.
- `DEL-07-02` is the accepted delegated owner of model-tree and property-inspector behavior under D-41 `DEC-074` O2. `DEL-00-05` retains ownership of the cross-cutting GUI state and interaction architecture; the delegation does not move implementation or authorize behavior beyond `DEL-07-02`'s accepted scope.
- DAG-007 is approved graph authority for relationship context; lifecycle state remains deliverable-local, and PKG-00 architecture-basis rows are not implementation or release-readiness claims.

## Human Review Gate
Human review must decide whether the architecture content is sufficient to support later PKG-01 through PKG-12 planning. `SEMANTIC_READY` means prepared for review; it does not mean accepted or issued.

## D-41 R5 T7 PDU-054 current declaration

Earlier setup-era statements on this surface are retained as historical setup context where applicable; this section is the active current-state declaration. The GUI state boundary now has command-backed selection and operation flows in the implemented slice. Current upstream authority is SOFTWARE_DECOMP revision 0.8 with DAG-007 coordination; this declaration does not claim completion of the full GUI product surface.
