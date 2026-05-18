# Datasheet: DEL-06-02 Sandboxed unit-aware expression evaluator

## Identification

| Field | Value | Source |
|---|---|---|
| Deliverable ID | DEL-06-02 | `_CONTEXT.md` |
| Package ID | PKG-06 | `_CONTEXT.md` |
| Package | Rule Packs and User-Supplied Code Check Engine | `_CONTEXT.md` |
| Type | BACKEND_FEATURE_SLICE | `docs/_Registers/Deliverables.csv` row DEL-06-02 |
| Scope item | SOW-045 | `docs/_Registers/ScopeLedger.csv` row SOW-045 |
| Objective | OBJ-005 | `docs/_Decomposition/SOFTWARE_DECOMP.md` objective map |
| Context envelope | L | `_CONTEXT.md` |
| Setup status | Setup documents and semantic/dependency artifacts only | Human sealed brief |

## Attributes

| Attribute | Value | Source |
|---|---|---|
| Primary purpose | Bound a future evaluator that can evaluate user-defined rule-pack expressions against solver results and user-owned design bases. | SOW-045; OBJ-005 |
| Required posture | Sandboxed, unit-aware, deterministic, and declarative. | `docs/SPEC.md` section 6; OPS-K-RULE-2; OPS-K-UNIT-1 |
| Arbitrary executable code | Not permitted. | OPS-K-RULE-2 |
| Unit and dimensional checks | Required for formulas, values, imports, exports, and evaluator results. | OPS-K-UNIT-1 |
| Missing required values | Explicit findings, never silent defaults. | OPS-K-DATA-2 |
| Expression grammar/library | TBD. No final grammar or library is selected by this setup run. | OI-006; sealed brief |
| Protected formulas or code-derived values | Excluded from public setup artifacts. | OPS-K-IP-1; OPS-K-IP-3; `docs/IP_AND_DATA_BOUNDARY.md` |
| Professional approval | Outside evaluator authority. User-rule checked status is not code compliance. | OPS-K-MECH-2; OPS-K-AUTH-1; `docs/TYPES.md` section 4 |

## Conditions

| Condition | Handling |
|---|---|
| Public repository boundary | Public artifacts may describe schemas, sandbox behavior, invented examples, and tests. They must not embed protected standards text, tables, proprietary formulas, material allowables, SIF/flexibility tables, or owner design-basis content. |
| User-owned rule basis | User rule packs may contain private or licensed formulas and values in user-controlled paths. Public deliverables only define the safe evaluation boundary. |
| Determinism | Future implementation must produce repeatable results and diagnostics for the same inputs, units, evaluator version, and rule-pack content. Exact numerical tolerances remain TBD. |
| Diagnostics | Future outputs should use result-envelope diagnostics with code, class, severity, source, affected object, message, remediation, and provenance where applicable. |
| Plugin and adapter boundary | Adapters and plugins must not bypass sandboxing, unit validation, provenance checks, diagnostics, or public/private data controls. |

## Construction

This setup run produces deliverable-local planning and readiness artifacts only:

- `Datasheet.md`, `Specification.md`, `Guidance.md`, and `Procedure.md`.
- `_SEMANTIC.md` and `_SEMANTIC_LENSING.md` for semantic readiness.
- `Dependencies.csv` and `_DEPENDENCIES.md` for dependency extraction.
- `_run_records/` evidence for each required setup step.

The anticipated implementation artifacts remain future work:

- `rule evaluator module` - not produced in this setup run.
- `evaluator tests` - not produced in this setup run.

## References

| Source | Use |
|---|---|
| `INIT.md` | Bootstrap boundaries and stop rules. |
| `AGENTS.md` | TASK dispatch discipline and write-scope rule. |
| `docs/CONTRACT.md` | Invariants OPS-K-RULE-2, OPS-K-UNIT-1, OPS-K-DATA-2, OPS-K-PRIV, OPS-K-IP, OPS-K-AGENT-1..4. |
| `docs/SPEC.md` | Rule-pack evaluator section, diagnostic classes, reporting, and V&V expectations. |
| `docs/TYPES.md` | Rule-pack check, user-rule checked status, human review boundary, and data provenance vocabulary. |
| `docs/IP_AND_DATA_BOUNDARY.md` | Public/private rule-pack and protected-data boundary. |
| `docs/VALIDATION_STRATEGY.md` | Rule-pack evaluator verification families. |
| `docs/_Decomposition/SOFTWARE_DECOMP.md` | PKG-06, DEL-06-02, AB-00 basis rows, OI-006. |
| `docs/_Registers/Deliverables.csv` | Deliverable identity and risk note. |
| `docs/_Registers/ScopeLedger.csv` | SOW-045 source row. |
| `docs/_Registers/ContextBudgetQA.csv` | WATCH risk and split-if-expanded note. |
