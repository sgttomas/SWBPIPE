# Specification: DEL-05-02 Load-case algebra engine

## Scope

This setup specifies the boundary for a backend load-case algebra deliverable covering unit-aware user-defined combinations and result-state subtraction/ranging. It does not implement an algebra engine, define code-specific combinations, choose an expression grammar/library, bundle protected rule data, or claim code compliance.

## Requirements

| ReqID | Requirement | Source |
|---|---|---|
| REQ-05-02-001 | The deliverable shall preserve unit-aware algebra for user-defined combinations. | SOW-014 |
| REQ-05-02-002 | Combination expressions shall reject or report dimensionally incompatible operations as explicit findings. | OPS-K-UNIT-1; OPS-K-DATA-2 |
| REQ-05-02-003 | Result-state subtraction and ranging shall remain mechanics-result operations and shall not imply code compliance. | DEL-05-02 description; OPS-K-MECH-2 |
| REQ-05-02-004 | Code-specific load combinations shall be supplied by user rule packs, not by bundled public defaults. | SOW-014 notes; OPS-K-DATA-1 |
| REQ-05-02-005 | The algebra surface shall not require arbitrary executable rules. | OPS-K-RULE-2 |
| REQ-05-02-006 | Missing primitive load cases, referenced result states, or rule-required inputs shall be reported explicitly, not silently defaulted. | OPS-K-DATA-2 |
| REQ-05-02-007 | Command/query/job result envelopes shall preserve mechanics solved, user-rule checked, and human-approved state separation. | AB-00-03 |
| REQ-05-02-008 | Diagnostics produced by the algebra layer shall carry provenance-compatible envelope fields when applicable. | AB-00-06 |
| REQ-05-02-009 | Deterministic tests shall cover valid unit-compatible combinations, invalid dimensional mixes, missing operands, subtraction/ranging, and no-default rule-pack boundaries before release use. | OPS-K-SOLVER-1; AB-00-08 |

## Standards

No external protected standard text is introduced by this setup. Governing local standards are the project invariant catalog, the sealed architecture-basis rows AB-00-01, AB-00-02, AB-00-03, AB-00-06, and AB-00-08, and the decomposition/register rows listed in `_CONTEXT.md`.

## Verification

| Requirement | Verification approach |
|---|---|
| REQ-05-02-001 | Unit tests for algebraic combination of compatible load/result terms. |
| REQ-05-02-002 | Negative tests for incompatible dimensions and expected diagnostics. |
| REQ-05-02-003 | Result-state tests showing subtraction/ranging produces mechanics outputs only. |
| REQ-05-02-004 | Protected-content/data-boundary review showing no bundled code-specific defaults. |
| REQ-05-02-005 | Security or evaluator-boundary test showing no arbitrary execution path is required. |
| REQ-05-02-006 | Missing operand/input tests with explicit findings. |
| REQ-05-02-007 | Result-envelope inspection for state separation. |
| REQ-05-02-008 | Diagnostic envelope inspection for provenance-compatible fields. |
| REQ-05-02-009 | CI or release-gate inclusion of deterministic expression tests. |

## Documentation

Required setup artifacts are `Datasheet.md`, `Specification.md`, `Guidance.md`, `Procedure.md`, `_SEMANTIC.md`, `_SEMANTIC_LENSING.md`, `Dependencies.csv`, `_DEPENDENCIES.md`, `_STATUS.md`, and `_run_records/`.

