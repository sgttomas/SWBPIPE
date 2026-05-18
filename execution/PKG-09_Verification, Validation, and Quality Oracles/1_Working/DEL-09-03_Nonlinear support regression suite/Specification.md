# Specification: DEL-09-03 Nonlinear support regression suite

## Scope

This deliverable specifies the setup surface for a future nonlinear support regression suite covering active-set, gap, friction, lift-off convergence, and regression cases for OpenPipeStress.

In scope for this setup pass:

- document the regression-suite purpose, boundaries, dependencies, and acceptance surface;
- preserve public/original/permissive benchmark-source constraints;
- identify diagnostics and result-envelope information the future tests must observe;
- record `TBD` items that depend on nonlinear solver maturity.

Out of scope for this setup pass:

- implementing nonlinear solver behavior;
- creating benchmark source files or regression tests;
- copying protected standards examples, code text, tables, formulas, or proprietary commercial cases;
- inventing final numerical convergence tolerances;
- asserting certification, professional approval, or code compliance.

## Requirements

| ID | Requirement | Source basis | Verification approach |
|---|---|---|---|
| REQ-09-03-001 | The future suite shall cover active-set, gap, friction, lift-off convergence, and regression behavior. | DEL-09-03 register description; SOW-026; `docs/SPEC.md` section 4.4 and 9. | Confirm case catalog maps each behavior category to at least one source-qualified case once implementation begins. |
| REQ-09-03-002 | Benchmark and regression case sources shall be public, original, or permissively licensed, with provenance recorded before public contribution. | SOW-026 note; OPS-K-IP-1/2/3; OPS-K-DATA-1/2/3. | Review case-source records and reject or quarantine suspected protected or proprietary content. |
| REQ-09-03-003 | The suite shall not bundle protected standards data, protected examples, code-derived tables, proprietary component values, or commercial benchmark cases without redistribution rights. | `INIT.md`; `docs/DIRECTIVE.md`; OPS-K-IP-1/2/3. | Protected-content/provenance review before any case enters the public repository. |
| REQ-09-03-004 | Nonlinear support regression checks shall observe diagnostic/result-envelope fields for active/inactive state, gaps, lift-off, friction state, convergence tolerance, iteration count, and non-convergence warnings when those fields exist. | `docs/SPEC.md` section 4.4; OPS-K-SOLVER-2; architecture basis AB-00-06. | Schema and test review against the diagnostics/result-envelope contract. |
| REQ-09-03-005 | Regression comparisons shall be deterministic for the same model, units, solver version, and applicable rule-pack inputs. | `docs/SPEC.md` section 4.5; OBJ-008. | Repeat-run checks once solver and runner support exist. |
| REQ-09-03-006 | All case definitions, inputs, and expected observations shall be unit-aware and dimensionally checkable. | OPS-K-UNIT-1; `docs/SPEC.md` sections 3, 4.5, and 9. | Unit/schema validation before case acceptance. |
| REQ-09-03-007 | Final numerical convergence tolerances and pass/fail thresholds shall remain `TBD` until nonlinear solver maturity provides evidence for defensible values. | DEL-09-03 risk note; OPS-K-AGENT-1. | Human review of solver evidence and tolerance proposal before implementation. |
| REQ-09-03-008 | Regression outcomes shall be reported as software verification evidence only and shall not claim code compliance, certification, sealing, approval, or professional reliance. | OPS-K-AUTH-1; `docs/TYPES.md` analysis-status vocabulary; `docs/DIRECTIVE.md` section 4.2. | Review report text and test labels for prohibited authority claims. |

## Standards

No external engineering code text or protected standard is imported by this setup deliverable.

| Standard or governance source | Status |
|---|---|
| OpenPipeStress invariant catalog | Applicable through `docs/CONTRACT.md`. |
| OpenPipeStress technical specification | Applicable through `docs/SPEC.md`. |
| External protected design codes | Not used as public source material; any future user-private use remains outside this public setup artifact. |

## Acceptance Criteria

This setup deliverable is acceptable when:

- `Datasheet.md`, `Specification.md`, `Guidance.md`, and `Procedure.md` exist and remain within the deliverable write scope;
- `_SEMANTIC.md` is generated and audited without final-cell algebra/operator leaks;
- `_SEMANTIC_LENSING.md` covers matrices A, B, C, F, D, X, and E and records warranted gaps without inventing content;
- `Dependencies.csv` is parseable with the v3.1 dependency schema;
- `_DEPENDENCIES.md` summarizes active anchors, constraints, warnings, and run history;
- `_STATUS.md` records `SEMANTIC_READY` only after the setup sequence completes;
- no benchmark source files, regression tests, solver code, protected examples, or final convergence tolerances are introduced.

## Deferred Implementation Decisions

| Decision Slot | Current setup disposition | Required future evidence |
|---|---|---|
| Nonlinear convergence tolerances | `TBD`; not invented in setup. | Nonlinear solver maturity evidence, repeat-run behavior, and human-reviewed tolerance proposal. |
| Diagnostic/result-envelope field names | `TBD`; categories are named but exact schema fields are not fixed here. | Applicable schema/solver contract for active state, gap/lift-off, friction state, iteration count, tolerance basis, and non-convergence warnings. |
| Release-gate command names | `TBD`; no implementation commands are defined in setup. | Actual regression runner, CI entry points, and validation command names from later implementation work. |

## Documentation

Required setup artifacts:

- `Datasheet.md`
- `Specification.md`
- `Guidance.md`
- `Procedure.md`
- `_SEMANTIC.md`
- `_SEMANTIC_LENSING.md`
- `Dependencies.csv`
- `_DEPENDENCIES.md`
- `_run_records/TASK_RUN_*.md`
- `_STATUS.md`
