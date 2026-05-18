# Procedure: DEL-15-04 External prover boundary metadata

## Purpose

Define and check the external-prover boundary metadata deliverable without implementing product code in this setup pass. The procedure describes how a later implementation task should produce external reference fields and boundary validation tests while preserving the professional reliance and data-boundary constraints already present in the sources.

## Prerequisites

| Prerequisite | Source / Status |
|---|---|
| Deliverable context for DEL-15-04 | `_CONTEXT.md` |
| Scope item SOW-075 and objectives OBJ-017/OBJ-018 | `docs/_Registers/ScopeLedger.csv`; `execution/_Decomposition/SOFTWARE_DECOMP.md` |
| Applicable invariants | `docs/CONTRACT.md`, especially OPS-K-IP-1 through OPS-K-IP-3, OPS-K-DATA-1 through OPS-K-DATA-3, OPS-K-AUTH-1 through OPS-K-AUTH-2, OPS-K-UNIT-1, OPS-K-GOV-3, and OPS-K-AGENT-1 through OPS-K-AGENT-4 |
| Status vocabulary and prohibited automatic statuses | `docs/TYPES.md` section 4 |
| Data and protected-content policy | `docs/IP_AND_DATA_BOUNDARY.md` |
| Approved local dependency mirror | `_DEPENDENCIES.md`; `Dependencies.csv` |
| Implementation write scope | TBD; this setup workflow does not authorize product-code edits |

## Steps

1. Confirm the implementation task is explicitly scoped to DEL-15-04 and has write authority for the intended schema/tests.
2. Read `_CONTEXT.md`, `_REFERENCES.md`, `Specification.md`, and the approved local dependency mirror before drafting any schema or tests.
3. Define only descriptive external-prover metadata categories supported by the sources: names, tags, notes, external references, attachments, and diagnostic comparison/handoff links.
4. Mark the exact schema path, field names, required/optional cardinality, attachment storage behavior, and external artifact hash treatment as `TBD` until resolved by the implementation task or human ruling.
5. Add or update schema fields so that they cannot be interpreted as automatic professional approval, certification, sealing, authentication, code compliance, or formal prover lifecycle state.
6. If a human acceptance reference is in scope, represent it only as an external, human-owned, reviewed-payload hash-bound record. Do not create it automatically.
7. Add boundary validation tests for allowed flexible metadata.
8. Add negative boundary validation tests for prohibited authority/status terms and formal prover lifecycle behavior.
9. Add protected-content/private-data review checks for any fixtures or example metadata.
10. Record unresolved design choices as `TBD`, `ASSUMPTION`, or `PROPOSAL`; do not turn them into requirements without source support.

## Verification

| Check | Expected Result |
|---|---|
| Deliverable identity check | Files and tests reference DEL-15-04 / PKG-15 consistently. |
| Flexible metadata check | Names, tags, notes, external references, attachments, and comparison/handoff links can be represented. |
| Status-boundary check | Automatic approval, certification, code-compliance, sealing, authentication, professional reliance, and formal prover lifecycle states are absent or rejected. |
| Human acceptance check | Any human acceptance reference is external, human-owned, and hash-bound; otherwise it is `TBD` or absent. |
| Data-boundary check | Fixtures/examples contain no protected standards text, proprietary values, private project data, unauthorized commercial examples, real secrets, or private rule-pack payloads. |
| Dependency check | Upstream dependency context from `Dependencies.csv` is preserved; approved DAG-002 mirror rows are not deleted, retired, or reclassified by this setup pass. |

## Records

The following records should result from a future implementation task:

- external reference fields or schema changes;
- boundary validation tests;
- evidence that prohibited status/authority terms are blocked;
- evidence that public fixtures/examples passed protected-content/private-data review;
- explicit `TBD` list for unresolved schema path, field names, cardinality, attachment storage, external artifact hashing, and commercial-tool parser behavior.

This setup pass produces only the four-document kit, semantic lens, semantic lensing register, dependency validation result, and status transitions allowed by the setup skills.
