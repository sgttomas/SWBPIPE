# Datasheet: DEL-06-03 Required-input completeness checker

## Identification

| Field | Value |
|---|---|
| Deliverable ID | DEL-06-03 |
| Deliverable name | Required-input completeness checker |
| Package ID | PKG-06 |
| Package name | Rule Packs and User-Supplied Code Check Engine |
| Deliverable type | BACKEND_FEATURE_SLICE |
| Scope item | SOW-004 |
| Supported objectives | OBJ-002, OBJ-005 |
| Context envelope | M |
| Current production mode | Setup/document production only |

## Attributes

| Attribute | Source-grounded value |
|---|---|
| Primary behavior | Identify missing rule-pack-required user inputs and prevent rule-check status from advancing when those inputs are absent. |
| Protected-data boundary | The checker must not bundle protected standards text, code tables, allowables, SIF/flexibility factors, proprietary formulas, or code-specific defaults. |
| User-data boundary | Code-specific and project-specific values remain user-supplied or privately imported data with provenance. |
| Status vocabulary | The relevant analysis state is `RULE_INPUTS_INCOMPLETE`; software must not emit automatic `CODE_COMPLIANT` status. |
| Warning class | Missing rule-pack input is a `RULE_CHECK_BLOCKING` condition, distinct from solve-blocking physical data. |
| Professional boundary | Checker output is a software finding; professional acceptance remains outside solver/rule code. |

## Conditions

| Condition | Handling |
|---|---|
| Required code/project input missing | Emit an explicit missing-input finding and block user-rule-check completion. |
| Required physical input missing | Keep classified separately as solve-blocking, not rule-check-blocking. |
| Input exists but provenance is absent or weak | Surface provenance warning; do not silently accept as reliable. |
| Rule pack requests protected bundled data | Treat as data-boundary/IP issue and escalate rather than fill a default. |
| Completeness cannot be determined | Mark as `TBD` or equivalent unresolved finding; do not infer engineering values. |

## Construction

The setup artifact defines the future feature boundary only. It does not create executable completeness rules, code formulas, material allowables, design-code tables, or checker implementation files.

The future implementation is expected to consume a rule-pack schema contract from DEL-06-01, analysis-status semantics from DEL-05-04, and diagnostics/result-envelope constraints from the architecture basis. Those dependencies are recorded as information-flow dependencies, not as schedule decisions.

## References

| Source | Used for |
|---|---|
| `INIT.md` | Bootstrap boundaries for open mechanics, user rule checks, and professional responsibility. |
| `AGENTS.md` | TASK dispatch boundary and sealed deliverable discipline. |
| `docs/CONTRACT.md` | Invariants OPS-K-DATA, OPS-K-RULE, OPS-K-IP, OPS-K-AGENT, and professional authority constraints. |
| `docs/DIRECTIVE.md` | No silent defaults, private code data, and human authority principles. |
| `docs/TYPES.md` | Analysis statuses, epistemic labels, provenance labels, and rule-pack vocabulary. |
| `docs/SPEC.md` | Rule-pack object, required inputs, evaluator constraints, warning classes, report notices, and acceptance semantics. |
| `docs/IP_AND_DATA_BOUNDARY.md` | Public/private data rules, quarantine expectations, and private rule-pack handling. |
| `docs/VALIDATION_STRATEGY.md` | Rule-pack missing-input test expectation. |
| `docs/_Decomposition/SOFTWARE_DECOMP.md` | PKG-06 and DEL-06-03 decomposition context. |
| `docs/_Registers/Deliverables.csv` | Machine-readable deliverable row. |
| `docs/_Registers/ScopeLedger.csv` | SOW-004 scope mapping. |
