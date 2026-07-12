# Procedure: DEL-03-04 Branch connection component model fields

## Purpose

Describe the evidence-reconciliation procedure for maintaining DEL-03-04
branch connection component model documentation against the implemented
component schema, invented fixture, and schema test evidence without exceeding
the authorized data boundary.

## Prerequisites

- Use the sealed DEL-03-04 brief and deliverable-local context.
- Apply SOW-008 and OBJ-004.
- Apply architecture basis AB-00-01, AB-00-02, AB-00-04, AB-00-06, AB-00-07, and AB-00-08.
- Apply CONTRACT invariants OPS-K-IP-1, OPS-K-IP-3, OPS-K-DATA-1, OPS-K-DATA-2, OPS-K-DATA-3, OPS-K-UNIT-1, OPS-K-RULE-1, OPS-K-MECH-1, and OPS-K-AGENT-1 through OPS-K-AGENT-4.
- Read `schemas/component.schema.yaml`,
  `fixtures/component/invented_component_library_valid.json`, and
  `tests/test_component_section_schema.py` as current implementation evidence.
- Treat absent engineering source data, policy decisions, dependency
  satisfaction, human dispositions, and lifecycle acceptance as `TBD`.

## Steps

1. Confirm the write scope is limited to authorized DEL-03-04 narrative files
   and the new reconciliation run record.
2. Read `_CONTEXT.md`, `_REFERENCES.md`, `_DEPENDENCIES.md`, the relevant decomposition/register rows, and applicable CONTRACT invariants.
3. Reconcile the four active docs and `MEMORY.md` against the implemented
   branch evidence in the component schema, invented component fixture, schema
   test, and local review findings.
4. Record implemented branch component field names where the schema now defines
   them: `branch_run_size`, `branch_header_size`,
   `branch_connection_angle`, `branch_connection_type`,
   `branch_reinforcement_area`, `branch_reinforcement_reference`,
   `branch_geometry_source_reference`, `sif_user_value`, and
   `flexibility_factor_user_value`.
5. Preserve unresolved `TBD`s for public branch source catalogs, fixture-value
   policy, standard-specific interpretation, concrete import formats,
   dependency satisfaction, human review dispositions, and lifecycle state.
6. Do not edit `Review_Findings.csv`; keep its findings conceptually
   `TECHNICALLY_ADDRESSED_PENDING_HUMAN` with `HumanDisposition=TBD`.
7. Run relevant validation when feasible: `python3 -m pytest
   tests/test_component_section_schema.py` and a stale-language search scoped to
   `Datasheet.md`, `Specification.md`, `Guidance.md`, and `Procedure.md`.
8. Do not introduce protected tables, formulas, examples, dimensional data,
   material allowables, or certification/compliance claims.

## Verification

- Four-document kit exists and preserves Datasheet, Specification, Guidance, and Procedure sections.
- Requirements trace to local context, decomposition/register rows, CONTRACT invariants, schema evidence, fixture evidence, and test evidence.
- Implemented branch schema slots are no longer described as absent or future-only.
- Unknown policy/source/dependency/human/lifecycle gates are still `TBD`; assumptions are labeled.
- No protected branch/SIF/flexibility tables, formulas, or examples are present.
- `python3 -m pytest tests/test_component_section_schema.py` passes when the
  local test environment is available.
- A stale-language search over the four active docs shows no remaining
  setup-only or future-only implementation language that contradicts the
  schema/fixture/test evidence.

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
- `_run_records/TASK_RUN_2026-06-05_DEL-03-04_evidence-reconciliation.md`
## D-41 R5 T3 PDU-019 Check

Run strict ComponentRecord schema negatives for privacy classification and unknown payload keys. Route evidence to REVIEW without changing its outcome.
