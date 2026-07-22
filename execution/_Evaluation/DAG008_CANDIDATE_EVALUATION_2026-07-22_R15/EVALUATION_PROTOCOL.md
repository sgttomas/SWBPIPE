# Evaluation Protocol — DAG-008 candidate specification

**Evaluation ID:** `DAG008_CANDIDATE_EVALUATION_2026-07-22_R15`

**Requested by:** HELP_HUMAN

**Manager:** EVALUATION

**Frozen Git base:** `aeace2ac39cb0039f2076dadcfce980c9e327a86`

**Additional frozen subject state:** exactly six uncommitted PROJECT_SETUP dependency-refresh paths for DEL-08-01 and DEL-10-05.

**Execution root:** `projects/chirality-piping/execution`

**Accepted upstream graph:** approved `execution/_DAG/DAG-007/`

**Accepted advisory basis:** `execution/_Evaluation/DEPENDENCY_READINESS_AUDIT_2026-07-21_R15/`

**Write target:** this evaluation directory only. No write to `_DAG`, deliverable registers, lifecycle/status, decomposition, receipts, product, pointers, or Git state is authorized.

## Questions

1. Do the two refreshed local registers reconcile all 13 R15 stale-satisfaction findings without changing dependency meaning, scope, direction, type, required maturity, decomposition truth, or lifecycle truth?
2. What exact immutable `DAG-008` proposal file state follows from approved DAG-007 plus those refreshed authoritative local rows?
3. Does the candidate pass canonical schema, strict graph, JSON, topology, cycle/SCC, uniqueness, count, and satisfaction-enumeration checks?
4. What deterministic source-to-target application can CHANGE perform without interpreting evaluation prose?

## Accepted toolbelt and work graph

- Deterministic CSV/JSON comparison and hashing.
- `projects/chirality-piping/tools/validation/validate_dependencies_schema.py`.
- `tools/coordination/audit_dag.py --canonical --strict`.
- Independent topology/SCC/count/status recomputation.
- Two read-only Agent 2 audits: exact 13-edge semantic cross-check and candidate-schema/count cross-check.
- EVALUATION fan-in and quarantined candidate materialization.

No scoring rubric is selected. Child returns must be validated for basis, scope, required fields, contradictions, and no-write compliance before synthesis.

## Candidate boundary

- `bundle/DAG-008/` is a derivative proposal package, not active dependency authority.
- Its `APPROVAL_RECORD.md` remains a placeholder and its local `_LATEST.md` says proposed.
- Root `execution/_DAG/_LATEST.md` remains untouched.
- DEL-08-01 and DEL-10-05 do not become selectable from this evaluation; selectability can change only after owner acceptance of the successor and separate authorization/effect of the root pointer move.
- Any detected change in dependency meaning, scope, or decomposition truth blocks this evaluation and routes to SOFTWARE_DECOMP.

## Expected outputs

- `bundle/DAG-008/` complete candidate package.
- `EDGE_DISPOSITIONS.csv` with all 13 rows exactly once.
- `APPLICATION_SPEC.csv` with exact source, target, action, and SHA-256.
- Preserved read-only child returns.
- `FINDINGS.csv`, `EVALUATION_REPORT.md`, `VALIDATION.md`, `HANDOFF.md`, and a run-level manifest.

## Decision points

1. EVALUATION may close only if all 13 changes are closure-currency-only and all candidate validations pass.
2. CHANGE may materialize the candidate only from `APPLICATION_SPEC.csv` and only without activating it.
3. The owner separately decides DAG-008 acceptance and root pointer authorization.
4. D-45 is presented at the same owner touchpoint as a separate decision and is not decided by this evaluation.
