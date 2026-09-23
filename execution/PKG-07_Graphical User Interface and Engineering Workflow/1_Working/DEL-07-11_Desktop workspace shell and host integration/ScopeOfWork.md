---
schema: chirality-deliverable-sow/v1
deliverable_id: DEL-07-11
package_id: PKG-07
decomposition_basis: projects/chirality-piping/execution/_Decomposition/SOFTWARE_DECOMP.md@sha256:c78301c67df5729d65c57963e6a915339049e3ffaa12d13961ee201445a9b984
project_scope_refs: [SOW-078]
package_objective_refs: [OBJ-006,OBJ-010,OBJ-015]
---

# Scope of Work — DEL-07-11

**Application basis:** SCA-011 Group 2 amendment; audited poststate acceptance is recorded in the SCA-011 Group 3 closure. Basis: `projects/chirality-piping/execution/_Decomposition/SOFTWARE_DECOMP.md@sha256:c78301c67df5729d65c57963e6a915339049e3ffaa12d13961ee201445a9b984`.

## Purpose and Objective Traceability

Provide the desktop workspace shell and native-host boundary for project/session navigation, shared selection/history coordination, panel mounting, jobs and explicit file operations, preserving durable/transient separation and owned service routes.

- **OUT-001** — Verified contract for DEL-07-11-REQ-001.
- **OUT-002** — Verified contract for DEL-07-11-REQ-002.
- **OUT-003** — Verified contract for DEL-07-11-REQ-003.
- **OUT-004** — Verified contract for DEL-07-11-REQ-004.

## Deliverable Definition — Ontology

### CLM-001 — DEL-07-11-REQ-001

Dispatch shell navigation, panel mounting, jobs and explicit file operations to their owning services while preserving source/result identity.

### CLM-002 — DEL-07-11-REQ-002

Coordinate shared selection and session history while separating transient session state from durable state stored by DEL-02-05 and run records owned by DEL-14-02.

### CLM-003 — DEL-07-11-REQ-003

Preserve native-host security configuration implementation, user-controlled private-data boundaries, cancellation, interruption and recoverable session behavior.

### CLM-004 — DEL-07-11-REQ-004

Conform common focus, keyboard and accessibility behavior to DEL-07-06 while feature panels retain their semantics.

## Completion and Reliance Basis — Epistemology

Each requirement needs named candidate-bound evidence. Existing code, route visibility or historical tests alone do not establish current conformance.

- **AC-001** — CLM-001 is supported by the corresponding witness, including invalid/unsupported paths and declared limitations.
- **AC-002** — CLM-002 is supported by the corresponding witness, including invalid/unsupported paths and declared limitations.
- **AC-003** — CLM-003 is supported by the corresponding witness, including invalid/unsupported paths and declared limitations.
- **AC-004** — CLM-004 is supported by the corresponding witness, including invalid/unsupported paths and declared limitations.

## Production and Verification Method — Praxeology

- **VER-001** — Exercise each native command/event and mounted route, including failed or denied actions; assert the shell cannot bypass owning service validation.
- **VER-002** — Save/reopen and undo/redo witnesses compare durable model/run identity and transient selection; superseded or failed responses cannot corrupt current session state.
- **VER-003** — Native-host witness covers menus/events, cancelled jobs, interrupted/recovered sessions and denied/private paths; browser-only evidence does not establish native behavior.
- **VER-004** — Exercise focus transfer, keyboard navigation and accessibility interfaces under the common contract; independent practitioner/usability holds remain distinct.

## Governing Values and Decisions — Axiology

DEL-07-08 retains operation review/apply interaction and ledger presentation; DEL-16-06 owns application execution; DEL-07-07 owns solve-progress/diagnostic UX. DEL-00-03/05/07 constrain architecture. Shell hosting transfers no feature semantics, persistence policy or security-policy authority.

Apply existing schema, units, provenance, protected-content and professional-boundary contracts. Source integration, internal checking, engineering acceptance and release remain distinct. Standard claim fence applies (F-PIP-2; DEC-081 claims taxonomy).

## Output and Evaluation Matrix

| Output | Objective refs | Requirement/claim refs | Acceptance refs | Verification refs | Evidence expectation |
|---|---|---|---|---|---|
| OUT-001 | OBJ-006 OBJ-010 OBJ-015 | CLM-001 | AC-001 | VER-001 | Named claim-bound contract witness with valid/invalid cases and candidate identity |
| OUT-002 | OBJ-006 OBJ-010 OBJ-015 | CLM-002 | AC-002 | VER-002 | Named claim-bound contract witness with valid/invalid cases and candidate identity |
| OUT-003 | OBJ-006 OBJ-010 OBJ-015 | CLM-003 | AC-003 | VER-003 | Named claim-bound contract witness with valid/invalid cases and candidate identity |
| OUT-004 | OBJ-006 OBJ-010 OBJ-015 | CLM-004 | AC-004 | VER-004 | Named claim-bound contract witness with valid/invalid cases and candidate identity |

## Delivery commitments and evidence limits

These clauses retain open delivery duties, owner decisions, and bounded evidence limits. Their keys link to the finite source-retirement account. They do not assert completion, lift a hold, change an accepted scope boundary, or select execution work. The governing requirements and cited decisions control future implementation and acceptance.

- **DEL-07-11:1** — Follow the SCA-011 accepted closure handoff for any explicitly retained derivative obligations; existing product/lifecycle holds remain.
- **DEL-07-11:2** — Inventory current implementation evidence against the four claim hooks in ScopeOfWork.md; retain missing, partial and unobserved behavior explicitly. No product or native verification was performed by candidate drafting.
- **DEL-07-11:3** — Preserve all engineering, privacy, professional, lifecycle and release holds named by the accepted instruments.
