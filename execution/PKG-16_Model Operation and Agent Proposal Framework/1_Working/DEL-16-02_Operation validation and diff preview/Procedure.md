# Procedure: DEL-16-02 Operation validation and diff preview

## Purpose

This procedure describes a conservative production workflow for creating and using the DEL-16-02 artifacts: operation validator, diff preview service, and validation tests. It is a setup artifact, not implementation evidence.

## Prerequisites

| Prerequisite | Source | Status |
|---|---|---|
| DEL-16-01 Structured model operation schema | `Dependencies.csv` row `DAG-002-E0827` | Approved upstream dependency; exact schema fields TBD. |
| DEL-13-03 Constraint validation engine | `Dependencies.csv` row `DAG-002-E0828` | Approved upstream dependency; exact API TBD. |
| DEL-14-03 Model-state comparison engine | `Dependencies.csv` row `DAG-002-E0829` | Approved upstream dependency; exact preview integration TBD. |
| DEL-14-05 Comparison mapping, tolerance, and export contracts | `Dependencies.csv` row `DAG-002-E0830` | Approved upstream dependency; default tolerances TBD. |
| DEL-04-06 Solver diagnostics and singularity detection | `Dependencies.csv` row `DAG-002-E0831` | Approved upstream dependency; diagnostic mapping TBD. |
| Architecture basis AB-00-01, AB-00-02, AB-00-03, AB-00-04, AB-00-06, AB-00-07, AB-00-08 | `_CONTEXT.md#Architecture Basis Injection` | Dispatchable context constraints; not implementation evidence. |

## Steps

1. Confirm the proposed edit is represented as a structured model operation.
   - If the operation schema is unavailable, record `TBD` rather than inventing fields.
   - Source: SOW-069; DEL-16-01 dependency.

2. Run schema validation against the approved operation schema.
   - Accept valid schema-conformant operations for the next validation stage.
   - Return structured diagnostics for schema failures.
   - Exact schema path and validator API are TBD.

3. Run constraint validation against the available design-knowledge and constraint-validation contracts.
   - Preserve validation messages and provenance.
   - Treat invalid findings as blockers before controlled application when supported by the constraint contract.
   - Exact constraint engine interface is TBD.

4. Generate a deterministic diff preview before application.
   - Use stable model-state comparison and mapping/tolerance contracts when available.
   - Keep preview payload fields and tolerance defaults TBD until upstream contracts define them.

5. Block invalid operations before application.
   - If schema or constraint validation fails, do not hand the operation to the controlled application path.
   - Preserve diagnostics and preview status without professional approval or code-compliance claims.

6. Hand valid, previewed operations to the later controlled application/user-acceptance workflow.
   - This deliverable does not implement hidden mutation or autonomous engineering acceptance.
   - User acceptance and operation audit trail are owned by DEL-16-03.

## Verification

| Check | Evidence to retain |
|---|---|
| Structured-operation input check | Test case showing direct mutation input is rejected or not accepted as application input. |
| Schema validation check | Passing and failing operation fixtures with structured diagnostics. |
| Constraint validation check | Constraint failure fixture with blocked application handoff. |
| Diff determinism check | Repeated preview output comparison for the same operation/model basis. |
| No-apply-on-invalid check | Test proving invalid operations do not mutate accepted state or enter controlled application. |
| Professional-boundary check | Test or review proving diagnostics/previews do not emit approval, certification, sealing, authentication, or code-compliance status. |

## Records

- Operation validator implementation notes: TBD.
- Diff preview service contract: TBD.
- Validation test cases and fixture inventory: required artifact.
- Diagnostic/result-envelope mapping notes: TBD.
- Dependency preservation record: existing `Dependencies.csv` approved DAG-002 rows remain ACTIVE and unchanged by this setup workflow.
