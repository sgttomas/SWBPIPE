# N2 fan-in — launch brief

- Parent: PROJECT_SETUP R23.
- Objective: independently validate all five consumer patches against the frozen
  branch basis, two-part evidence, exact delta, schemas/IDs, raw-row guards,
  constrained index deltas, and path fences.
- Writes: R23 control records only; no consumer repair authority.
- Acceptance: PASS only if every frozen check passes; otherwise stop without
  repair. Stop before EVALUATION/DAG-009.

