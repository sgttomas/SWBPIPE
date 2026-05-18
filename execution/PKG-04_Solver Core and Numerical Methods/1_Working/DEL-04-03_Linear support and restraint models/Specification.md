# Specification: DEL-04-03 Linear support and restraint models

## Scope

This deliverable defines the setup contract for linear support and restraint models in PKG-04. It covers anchors, guides, line stops, vertical supports, springs, and imposed displacement boundary data as listed in SOW-011.

Out of scope:

- Nonlinear active-set behavior such as one-way restraints, lift-off, gaps, and friction, assigned to DEL-04-04.
- Solver kernel implementation, sparse solve selection, and performance harness work outside the local support-model contract.
- Code compliance, certification, professional approval, or bundled protected standards/vendor values.

## Requirements

| ID | Requirement | Source |
|---|---|---|
| DEL-04-03-R01 | The support model shall cover the SOW-011 families: anchors, guides, line stops, vertical supports, springs, and imposed displacement boundary data. | `docs/_Registers/ScopeLedger.csv` row SOW-011 |
| DEL-04-03-R02 | Linear support data shall be expressible against the 3D centerline/frame model and the six node degrees of freedom. | `docs/CONTRACT.md` OPS-K-MECH-1; `docs/SPEC.md` section 4.1 |
| DEL-04-03-R03 | The support model shall keep mechanics solving separate from user-rule evaluation and human professional compliance judgment. | `docs/CONTRACT.md` OPS-K-MECH-2 |
| DEL-04-03-R04 | Support stiffnesses, imposed displacements, directions, and related numerical data shall be unit-aware and dimensionally checked. | `docs/CONTRACT.md` OPS-K-UNIT-1 |
| DEL-04-03-R05 | Missing solve-required support data shall be reported as explicit findings or diagnostics, never silently filled by defaults. | `docs/CONTRACT.md` OPS-K-DATA-2 |
| DEL-04-03-R06 | Public support examples and test fixtures shall avoid protected standards text, protected tables, copied formulas, proprietary vendor data, and unsupported default values. | `docs/CONTRACT.md` OPS-K-IP-1 |
| DEL-04-03-R07 | Support diagnostics/result envelopes shall preserve the AB-00-06 fields: code, class, severity, source, affected object, message, remediation, and provenance. | `docs/_Decomposition/SOFTWARE_DECOMP.md` AB-00-06 |
| DEL-04-03-R08 | The support model shall preserve module boundaries and inward dependencies toward domain contracts; adapters/plugins must not bypass validation, diagnostics, or governance. | `docs/_Decomposition/SOFTWARE_DECOMP.md` AB-00-02 |
| DEL-04-03-R09 | Solver-facing support changes shall be covered by deterministic verification tests before release use. | `docs/CONTRACT.md` OPS-K-SOLVER-1; `docs/_Decomposition/SOFTWARE_DECOMP.md` AB-00-08 |
| DEL-04-03-R10 | Unknown coordinate conventions, stiffness representation details, and exact solver-library integration points shall remain `TBD` until resolved by an implementation brief or human decision. | `docs/CONTRACT.md` OPS-K-AGENT-1; `_CONTEXT.md` Still TBD |
| DEL-04-03-R11 | Setup wording shall not imply a selected rigid-restraint numerical method, constraint-elimination strategy, penalty method, or solver assembly algorithm. | Human hard stop; `docs/_Decomposition/SOFTWARE_DECOMP.md` AB-00-01 |

## Standards

- Public project invariants in `docs/CONTRACT.md` apply, especially OPS-K-MECH-1, OPS-K-MECH-2, OPS-K-UNIT-1, OPS-K-SOLVER-1, OPS-K-DATA-2, OPS-K-AGENT-1..4, and OPS-K-IP-1.
- SCA-001 architecture basis IDs AB-00-01, AB-00-02, AB-00-03, AB-00-06, and AB-00-08 apply as dispatch constraints.
- External piping design-code clauses, support stiffness defaults, and vendor support catalogs are not accessible sources for this setup kit; any such requirement is `TBD` and must be user-supplied or rights-cleared before use.

## Verification

| Requirement | Verification approach |
|---|---|
| R01, R02 | Schema/model review confirms all SOW-011 support families can be represented without leaving the six-DOF centerline model. |
| R03 | Result/status review confirms mechanics-solved state is not represented as rule-pass or professional approval. |
| R04 | Unit tests cover dimensional validation for stiffness and imposed displacement fields. |
| R05, R07 | Negative tests omit required support data and verify explicit structured diagnostics. |
| R06 | Protected-content/provenance review checks public examples and tests. |
| R08 | Architecture review checks dependency direction and no-bypass behavior. |
| R09 | Linear restraint tests are deterministic for fixed model, units, solver version, and inputs. |
| R10 | Open implementation choices are tracked as `TBD` or decision-record inputs, not hidden assumptions. |
| R11 | Review confirms setup artifacts do not prescribe solver implementation or invented support stiffness/defaults. |

## Documentation

The implementation brief for this deliverable should produce or update:

- support model artifact;
- linear restraint tests;
- diagnostics notes for missing/invalid support data;
- decision records for any resolved coordinate convention, stiffness representation, or solver-library integration choice.
