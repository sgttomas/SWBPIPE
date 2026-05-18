# Datasheet: DEL-05-02 Load-case algebra engine

## Identification

| Field | Value |
|---|---|
| Deliverable ID | DEL-05-02 |
| Name | Load-case algebra engine |
| Package | PKG-05 Loads, Load Cases, and Stress Recovery |
| Type | BACKEND_FEATURE_SLICE |
| Scope items | SOW-014 |
| Objectives | OBJ-003, OBJ-005 |
| Context envelope | M |

## Attributes

| Attribute | Setup value | Source |
|---|---|---|
| Primary subject | Unit-aware algebra for user-defined load-case combinations and result-state subtraction/ranging. | `_CONTEXT.md`; `docs/_Decomposition/SOFTWARE_DECOMP.md` row DEL-05-02 |
| Mechanics boundary | Combination results remain mechanics solver/result states; code acceptability is evaluated by user rule packs. | `docs/CONTRACT.md` OPS-K-MECH-2 |
| Unit discipline | All load-case expressions and result operations must be unit-aware and dimensionally checked. | `docs/CONTRACT.md` OPS-K-UNIT-1 |
| Data boundary | Code-specific combinations, allowables, and project-specific rule content are user-supplied or privately imported, not bundled defaults. | `docs/CONTRACT.md` OPS-K-DATA-1; SOW-014 notes |
| Rule-pack boundary | Rule packs supply code-specific combinations; they must not execute arbitrary code. | SOW-014; OPS-K-RULE-2 |
| Expression grammar/library | TBD. Do not invent an evaluator, syntax, or code-specific defaults during setup. | `_CONTEXT.md` Still TBD |

## Conditions

- Missing combination inputs or rule-check-required values must become explicit findings rather than silent defaults. Source: OPS-K-DATA-2.
- Solver/rule changes require deterministic verification before release. Source: OPS-K-SOLVER-1; AB-00-08.
- Result envelopes must preserve mechanics-solved, user-rule-checked, and human-approved distinctions. Source: AB-00-03.
- Diagnostics and result envelopes must carry code, class, severity, source, affected object, message, remediation, and provenance where applicable. Source: AB-00-06.

## Construction

| Artifact | Description | Setup status |
|---|---|---|
| Combination engine | Backend algebra surface for user-defined combinations and result-state subtraction/ranging. | Anticipated; no implementation in this setup |
| Expression tests | Tests for unit/dimension compatibility, subtraction/ranging behavior, and invalid-expression diagnostics. | Anticipated; fixtures TBD |
| Rule-pack handoff | Boundary allowing user rule packs to supply code-specific combinations without bundling defaults. | Required boundary; schema/evaluator details TBD |
| Diagnostics | Explicit findings for dimensional mismatch, missing inputs, invalid references, and unsupported operations. | Required in principle; codes TBD |

## References

- `_CONTEXT.md`
- `_REFERENCES.md`
- `docs/_Decomposition/SOFTWARE_DECOMP.md` revision 0.4
- `docs/_Registers/Deliverables.csv` row DEL-05-02
- `docs/_Registers/ScopeLedger.csv` row SOW-014
- `docs/_Registers/ContextBudgetQA.csv` row DEL-05-02
- `docs/CONTRACT.md`

