---
schema: chirality-deliverable-sow/v1
deliverable_id: DEL-16-06
package_id: PKG-16
decomposition_basis: projects/chirality-piping/execution/_Decomposition/SOFTWARE_DECOMP.md@sha256:c78301c67df5729d65c57963e6a915339049e3ffaa12d13961ee201445a9b984
project_scope_refs: [SOW-069,SOW-070]
package_objective_refs: [OBJ-015]
---

# Scope of Work — DEL-16-06

**Application basis:** SCA-011 Group 2 amendment; audited poststate acceptance is recorded in the SCA-011 Group 3 closure. Basis: `projects/chirality-piping/execution/_Decomposition/SOFTWARE_DECOMP.md@sha256:c78301c67df5729d65c57963e6a915339049e3ffaa12d13961ee201445a9b984`.

## Purpose and Objective Traceability

Apply only explicitly accepted, contract-valid structured operations to their current model basis through the sole DEC-020 operation engine; preserve rejected-state nonmutation and hand actual changed-state outcomes to audit and persistence owners.

- **OUT-001** — Verified contract for DEL-16-06-REQ-001.
- **OUT-002** — Verified contract for DEL-16-06-REQ-002.
- **OUT-003** — Verified contract for DEL-16-06-REQ-003.
- **OUT-004** — Verified contract for DEL-16-06-REQ-004.

## Deliverable Definition — Ontology

### CLM-001 — DEL-16-06-REQ-001

Apply only an explicitly accepted, schema/constraint-valid, previewed structured operation whose current model hash matches its accepted basis.

### CLM-002 — DEL-16-06-REQ-002

Preserve rejected, invalid, blocked and stale-basis state without mutation or success-shaped receipts.

### CLM-003 — DEL-16-06-REQ-003

Use the sole DEC-020 operation engine with equivalent native/wasm behavior for supported operations; preserve owned schema/taxonomy semantics.

### CLM-004 — DEL-16-06-REQ-004

Return deterministic changed-state/result evidence for the accepted change and hand actual outcomes to DEL-16-03 audit and DEL-02-05 persistence without creating another mutation route.

## Completion and Reliance Basis — Epistemology

Each requirement needs named candidate-bound evidence. Existing code, route visibility or historical tests alone do not establish current conformance.

- **AC-001** — CLM-001 is supported by the corresponding witness, including invalid/unsupported paths and declared limitations.
- **AC-002** — CLM-002 is supported by the corresponding witness, including invalid/unsupported paths and declared limitations.
- **AC-003** — CLM-003 is supported by the corresponding witness, including invalid/unsupported paths and declared limitations.
- **AC-004** — CLM-004 is supported by the corresponding witness, including invalid/unsupported paths and declared limitations.

## Production and Verification Method — Praxeology

- **VER-001** — Invoke the product route with valid acceptance, schema-invalid envelope, blocking validation, absent acceptance and stale hash; only the accepted-current case may mutate.
- **VER-002** — Compare before/after accepted-state identity for each rejected path and ensure truthful failure/unchanged-state evidence reaches callers.
- **VER-003** — Run the same supported-operation corpus through native and wasm routes and compare contract-visible outcomes; Python preview tests alone do not establish product application conformance.
- **VER-004** — Trace operation/accepted-basis/result identity through application, receipt and save/reopen; distinguish pre-application acceptance from actual post-application outcome and retain persistence-policy residuals.

## Governing Values and Decisions — Axiology

DEL-16-01 owns schema/taxonomy, DEL-16-02 validation/preview, DEL-16-03 acceptance/audit policy, DEL-02-05 persistence and DEL-16-04 professional-boundary controls. A user actor-type string is not identity evidence. Retention, actor policy and unsupported-operation decisions remain open; no live agent/provider activation.

Apply existing schema, units, provenance, protected-content and professional-boundary contracts. Source integration, internal checking, engineering acceptance and release remain distinct. Standard claim fence applies (F-PIP-2; DEC-081 claims taxonomy).

## Output and Evaluation Matrix

| Output | Objective refs | Requirement/claim refs | Acceptance refs | Verification refs | Evidence expectation |
|---|---|---|---|---|---|
| OUT-001 | OBJ-015 | CLM-001 | AC-001 | VER-001 | Named claim-bound contract witness with valid/invalid cases and candidate identity |
| OUT-002 | OBJ-015 | CLM-002 | AC-002 | VER-002 | Named claim-bound contract witness with valid/invalid cases and candidate identity |
| OUT-003 | OBJ-015 | CLM-003 | AC-003 | VER-003 | Named claim-bound contract witness with valid/invalid cases and candidate identity |
| OUT-004 | OBJ-015 | CLM-004 | AC-004 | VER-004 | Named claim-bound contract witness with valid/invalid cases and candidate identity |

## Delivery commitments and evidence limits

These clauses retain open delivery duties, owner decisions, and bounded evidence limits. Their keys link to the finite source-retirement account. They do not assert completion, lift a hold, change an accepted scope boundary, or select execution work. The governing requirements and cited decisions control future implementation and acceptance.

- **DEL-16-06:1** — Follow the SCA-011 accepted closure handoff for any explicitly retained derivative obligations; existing product/lifecycle holds remain.
- **DEL-16-06:2** — Inventory current implementation evidence against the four claim hooks in ScopeOfWork.md; retain missing, partial and unobserved behavior explicitly. No product or native verification was performed by candidate drafting.
- **DEL-16-06:3** — Preserve all engineering, privacy, professional, lifecycle and release holds named by the accepted instruments.
