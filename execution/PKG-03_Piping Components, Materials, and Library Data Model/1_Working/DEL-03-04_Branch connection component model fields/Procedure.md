# Procedure: DEL-03-04 Branch connection component model fields

## Purpose

Describe the setup-level procedure for producing and later using the branch connection component model deliverable without exceeding the authorized data boundary.

## Prerequisites

- Use the sealed DEL-03-04 brief and deliverable-local context.
- Apply SOW-008 and OBJ-004.
- Apply architecture basis AB-00-01, AB-00-02, AB-00-04, AB-00-06, AB-00-07, and AB-00-08.
- Apply CONTRACT invariants OPS-K-IP-1, OPS-K-IP-3, OPS-K-DATA-1, OPS-K-DATA-2, OPS-K-DATA-3, OPS-K-UNIT-1, OPS-K-RULE-1, OPS-K-MECH-1, and OPS-K-AGENT-1 through OPS-K-AGENT-4.
- Treat absent engineering source data as `TBD`.

## Steps

1. Confirm the working scope is only the DEL-03-04 deliverable folder for setup evidence.
2. Read `_CONTEXT.md`, `_REFERENCES.md`, `_DEPENDENCIES.md`, the relevant decomposition/register rows, and applicable CONTRACT invariants.
3. Define branch component field groups only at the non-protected category level: geometry, reinforcement, user SIF, user flexibility, local data, source/provenance, units, and redistribution metadata.
4. Record unknown exact field names, specialized local-check fields, and code-specific interpretation as `TBD`.
5. Specify validation expectations for missing data, unit checks, provenance, public/private data boundary, and protected-content exclusion.
6. For the later implementation pass, create the branch component model and validation tests only within the write scope authorized by that future brief.
7. Do not introduce protected tables, formulas, examples, dimensional data, material allowables, or certification/compliance claims.

## Verification

- Four-document kit exists and preserves Datasheet, Specification, Guidance, and Procedure sections.
- Requirements trace to local context, decomposition/register rows, and CONTRACT invariants.
- Unknowns are `TBD`; assumptions are labeled.
- No protected branch/SIF/flexibility tables, formulas, or examples are present.
- Dependency register validates against v3.1 schema after extraction.

## Records

- `Datasheet.md`
- `Specification.md`
- `Guidance.md`
- `Procedure.md`
- `_SEMANTIC.md`
- `_SEMANTIC_LENSING.md`
- `Dependencies.csv`
- `_DEPENDENCIES.md`
- `_run_records/TASK_RUN_*.md`
