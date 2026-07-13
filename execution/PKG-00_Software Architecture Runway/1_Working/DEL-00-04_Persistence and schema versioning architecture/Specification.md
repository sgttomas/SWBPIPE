# Specification: DEL-00-04 Persistence and schema versioning architecture

## Normative Scope
This specification governs only `DEL-00-04` inside `PKG-00 - Software Architecture Runway`. It defines architecture documentation obligations and acceptance evidence. It does not authorize implementation work in `PKG-01` through `PKG-12`.

## Requirements
| ID | Requirement | Evidence |
|---|---|---|
| REQ-04-01 | Define persistence as deterministic, versioned, unit-aware, and provenance-preserving. | Acceptance review |
| REQ-04-02 | Require every persisted artifact to declare schema version and migration status. | Acceptance review |
| REQ-04-03 | Define canonicalization as a prerequisite for reproducible hashes and audit manifests while leaving the concrete algorithm TBD. | Acceptance review |
| REQ-04-04 | Require round-trip checks for models, units, loads, rule-pack references, diagnostics, and provenance metadata. | Acceptance review |
| REQ-04-05 | Separate public schemas from private/user-owned code data and proprietary libraries. | Human review |

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
- Downstream consumers may use this deliverable as AB-00-04 architecture-basis context only through sealed briefs and governed review/dispatch surfaces; this does not make PKG-00 ISSUED.
- DAG-007 is approved graph authority for relationship context; lifecycle state remains deliverable-local, and PKG-00 architecture-basis rows are not implementation or release-readiness claims.

## Human Review Gate
Human review must decide whether the architecture content is sufficient to support later PKG-01 through PKG-12 planning. `SEMANTIC_READY` means prepared for review; it does not mean accepted or issued.

## D-41 R5 T7 PDU-054 current declaration

Earlier setup-era statements on this surface are retained as historical setup context where applicable; this section is the active current-state declaration. Persistence versioning, migration, canonical round-trip behavior, and storage-boundary evidence now exist in the implemented slice. Current upstream authority is SOFTWARE_DECOMP revision 0.8 with DAG-007 coordination; provider expansion and any still-recorded policy choices remain residual work.
