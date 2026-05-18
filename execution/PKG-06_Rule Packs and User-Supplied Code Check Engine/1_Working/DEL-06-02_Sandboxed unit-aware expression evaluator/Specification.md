# Specification: DEL-06-02 Sandboxed unit-aware expression evaluator

## Scope

This setup specification bounds the future backend evaluator for user-defined rule-pack expressions. It covers sandboxing, unit awareness, variable binding, deterministic diagnostics, and public/private data boundaries.

It does not implement an evaluator module, create evaluator tests, choose an expression grammar or library as final, invent numerical tolerances, include protected formulas, ship code-specific rule content, or claim professional code compliance.

## Requirements

| ReqID | Requirement | Source |
|---|---|---|
| REQ-06-02-001 | The future evaluator shall be sandboxed and incapable of arbitrary code execution. | SOW-045; OPS-K-RULE-2 |
| REQ-06-02-002 | The evaluator shall remain declarative: rule packs define expressions and checks, not executable programs. | SOW-045; OI-006 |
| REQ-06-02-003 | All expression inputs, intermediate quantities where represented, comparisons, and outputs shall be unit-aware and dimensionally checked. | OPS-K-UNIT-1 |
| REQ-06-02-004 | Missing variables, missing required rule-check values, unit mismatches, invalid references, and unsupported expression forms shall produce explicit findings rather than silent defaults. | OPS-K-DATA-2; `docs/SPEC.md` section 7 |
| REQ-06-02-005 | Variable binding shall be limited to declared rule-pack variables, solver result fields, and user-supplied design-basis inputs allowed by the sealed schema/interface contract. | `docs/SPEC.md` section 6; AB-00-02; AB-00-07 |
| REQ-06-02-006 | The exact expression grammar, parser, and evaluator library remain TBD and require a future human architecture decision before implementation. | OI-006; sealed brief |
| REQ-06-02-007 | Public artifacts shall not include protected standards text, tables, code-derived formulas, material allowables, SIF/flexibility data, proprietary vendor data, owner standards, or private rule packs. | OPS-K-IP-1; OPS-K-IP-3; `docs/IP_AND_DATA_BOUNDARY.md` |
| REQ-06-02-008 | Rule-pack evaluation results shall not be represented as automatic code compliance, certification, sealing, approval, or professional reliance. | OPS-K-AUTH-1; OPS-K-MECH-2; `docs/TYPES.md` section 4 |
| REQ-06-02-009 | Diagnostics and result envelopes shall preserve mechanics solved, user-rule checked, incomplete data, and human-approved state separation. | AB-00-03; AB-00-06 |
| REQ-06-02-010 | Adapters and plugins shall not bypass sandboxing, unit checks, provenance checks, diagnostics, or public/private data boundaries. | AB-00-07 |
| REQ-06-02-011 | Future tests shall cover unsafe expression rejection, dimension mismatch, missing binding, deterministic diagnostic emission, protected-content boundaries for public examples, and plugin/adapter bypass attempts. | `docs/VALIDATION_STRATEGY.md`; AB-00-08 |

## Standards

No external protected standard text is introduced by this setup. Governing local standards are the project bootstrap, invariant catalog, data-boundary policy, analysis-status vocabulary, sealed architecture-basis rows in `_CONTEXT.md`, and the decomposition/register rows for DEL-06-02 and SOW-045.

## Open Decisions

| Decision | Current status | Required handling |
|---|---|---|
| Expression grammar and parser/evaluator library | TBD | Do not implement until a future sealed brief records the human architecture decision. |
| Quantity representation and unit algebra integration point | TBD | Coordinate with the unit-system contract before implementation. |
| Final diagnostic code taxonomy | TBD | Use explicit finding classes in setup; define final codes in implementation scope. |
| Numerical tolerances for comparisons | TBD | Do not invent tolerances in setup artifacts. |
| Variable namespace and result-field binding contract | TBD | Coordinate with rule-pack schema and result-envelope contracts. |

## Verification

| Requirement | Verification approach |
|---|---|
| REQ-06-02-001 | Security tests demonstrating that expressions cannot invoke filesystem, network, process, reflection, imports, or host-language execution paths. |
| REQ-06-02-002 | Schema/evaluator tests proving accepted rule definitions are declarative expressions and checks only. |
| REQ-06-02-003 | Unit tests for compatible dimensions, incompatible dimensions, comparison dimensions, and explicit unit metadata propagation. |
| REQ-06-02-004 | Negative tests for missing variables, invalid references, unsupported operators, and missing user rule inputs. |
| REQ-06-02-005 | Binding tests using only declared variables and result fields from the sealed interface. |
| REQ-06-02-006 | Review gate check confirming grammar/library selection is recorded before implementation. |
| REQ-06-02-007 | Protected-content and private-data review for public rule-pack examples and evaluator fixtures. |
| REQ-06-02-008 | Status/result tests confirming no automatic `CODE_COMPLIANT` or professional approval state is emitted. |
| REQ-06-02-009 | Result-envelope inspection for diagnostics, warnings, provenance, and state separation. |
| REQ-06-02-010 | Adapter/plugin boundary tests proving no bypass path reaches the evaluator without validation. |
| REQ-06-02-011 | CI or release-gate inclusion of deterministic evaluator verification. |

## Documentation

Required setup artifacts are `Datasheet.md`, `Specification.md`, `Guidance.md`, `Procedure.md`, `_SEMANTIC.md`, `_SEMANTIC_LENSING.md`, `Dependencies.csv`, `_DEPENDENCIES.md`, `_STATUS.md`, and `_run_records/`.

Future implementation documentation should add the selected grammar/library decision record, evaluator threat model, unit-binding contract, diagnostic taxonomy, and test evidence. Those artifacts are outside this setup session.
