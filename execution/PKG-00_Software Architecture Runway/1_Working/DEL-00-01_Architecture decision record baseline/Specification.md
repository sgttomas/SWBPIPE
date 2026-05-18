# Specification: DEL-00-01 Architecture decision record baseline

## Normative Scope
This specification governs only `DEL-00-01` inside `PKG-00 - Software Architecture Runway`. It defines architecture documentation obligations and acceptance evidence. It does not authorize implementation work in `PKG-01` through `PKG-12`.

## Requirements
| ID | Requirement | Evidence |
|---|---|---|
| REQ-01-01 | Define an ADR index structure that can list proposed, accepted, superseded, and rejected decisions. | Acceptance review |
| REQ-01-02 | Define an ADR template with fields for context, options, decision status, consequences, source references, and human authority. | Acceptance review |
| REQ-01-03 | Record stack/runtime/framework/library/package choices as TBD unless a human ruling is cited. | Acceptance review |
| REQ-01-04 | Require every accepted architecture decision to identify affected packages and reconsideration triggers. | Acceptance review |
| REQ-01-05 | Keep all architecture decisions code-neutral and free of protected standards data. | Human review |

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
- Upstream authority is the v0.3 decomposition and registers, not inferred cross-deliverable dependencies.
- Downstream consumers may use this deliverable as architecture-review input only after human acceptance.
- The deferred Full DAG must not be used to compute blocked/unblocked states for this deliverable.

## Human Review Gate
Human review must decide whether the architecture content is sufficient to support later PKG-01 through PKG-12 planning. `SEMANTIC_READY` means prepared for review; it does not mean accepted or issued.
