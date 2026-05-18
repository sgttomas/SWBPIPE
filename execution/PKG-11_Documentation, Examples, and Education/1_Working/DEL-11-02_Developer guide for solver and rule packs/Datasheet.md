# Datasheet: DEL-11-02 Developer guide for solver and rule packs

## Identification

| Field | Value |
|---|---|
| Deliverable ID | DEL-11-02 |
| Package ID | PKG-11 |
| Package | Documentation, Examples, and Education |
| Type | DOC_UPDATE |
| Primary anticipated artifact | `docs/developer_guide/index.md` |
| Setup artifact location | `execution/PKG-11_Documentation, Examples, and Education/1_Working/DEL-11-02_Developer guide for solver and rule packs/` |
| Scope item | SOW-033 |
| Objectives | OBJ-001, OBJ-002 |
| Context envelope | M |
| Current setup status | Prepared for semantic setup and dependency extraction |

## Attributes

| Attribute | Setup value |
|---|---|
| Documentation audience | Solver contributors, rule-pack contributors, reviewers, and maintainers. |
| Guide subject | Developer-facing explanation of solver architecture, rule-pack schema expectations, test discipline, and contribution boundaries. |
| Architecture basis | AB-00-01, AB-00-02, AB-00-06, AB-00-07, AB-00-08 from SOFTWARE_DECOMP revision 0.4. |
| Runtime baseline referenced | Rust core/application services; schema-first command/query/job result envelopes; JSON Schema 2020-12; Cargo/Vitest/Playwright/validation/protected-content gates where applicable. |
| Solver boundary | The guide must present the solver as open mechanics for a 3D centerline/frame model, not as a code-compliance authority. |
| Rule-pack boundary | The guide must present rule packs as user-supplied/private design-basis artifacts with provenance, checksums, and redistribution status. |
| Contribution boundary | Contributors may improve mechanics, schemas, tests, docs, and permissively sourced/invented examples; they must not add protected standards data, private project data, or misleading approval claims. |
| Professional boundary | Software output and agent artifacts are drafts/decision support until accepted by human review; they do not certify, seal, approve, authenticate, or declare engineering compliance. |

## Conditions

| Condition | Requirement for this deliverable |
|---|---|
| Protected content | Do not copy standards-body text, tables, examples, protected formulas, protected dimensional tables, material allowables, SIF/flexibility tables, or proprietary commercial data. |
| Examples | Any example shown in the future guide must be invented, non-code, and marked as educational only. |
| Missing values | Unknown implementation details, engineering values, tolerances, expression grammar choices, numerical solver choices, and CI thresholds remain `TBD` unless a human-approved source resolves them. |
| Unit handling | Guide requirements must reinforce unit-aware model data, rule expressions, imports, exports, diagnostics, and tests. |
| Diagnostics | Guide requirements must use structured diagnostics and result-envelope language without implying compliance approval. |
| No-bypass baseline | Plugins, adapters, rule-pack tooling, and public APIs cannot bypass units, provenance, diagnostics, sandboxing, report controls, or data-boundary checks. |

## Construction

The future developer guide should be organized around these content groups:

| Content group | Required coverage |
|---|---|
| Architecture map | Layer responsibilities for GUI/application services/domain core/solver/rules/reports/adapters, with no-bypass constraints. |
| Solver architecture | Centerline/frame model, six degree-of-freedom node model, straight elements, component interfaces, loads, stress recovery, nonlinear support status, deterministic diagnostics, and test obligations. |
| Rule-pack architecture | Schema metadata, required inputs, user-supplied variables, checks, provenance, redistribution status, checksum handling, sandboxing, unit checks, and incomplete-input behavior. |
| Test discipline | Unit tests, schema tests, solver benchmarks, stress recovery regression, nonlinear convergence traces, rule-pack evaluator tests, report reproducibility tests, protected-content/provenance gates, and review evidence. |
| Contribution boundaries | What contributors may add, what must be quarantined, how to label assumptions/TBDs, and when to escalate to human/legal/professional review. |
| Review and acceptance | How developer changes move from draft evidence through review without claiming engineering reliance. |

## References

| Source | Use |
|---|---|
| `INIT.md` | Bootstrap boundaries: open mechanics, private standards data, rule checks vs professional approval, centerline analysis vs local FEA. |
| `AGENTS.md` | TASK dispatch boundaries and documentation package role. |
| `docs/DIRECTIVE.md` | Founding intent, stop rules, non-negotiable product principles. |
| `docs/CONTRACT.md` | Invariants for IP, data, rule packs, units, privacy, authority, and agent boundaries. |
| `docs/TYPES.md` | Canonical statuses, terms, provenance labels, and domain-object vocabulary. |
| `docs/SPEC.md` | Architecture, solver, rule-pack, report, and V&V baseline for guide content. |
| `docs/IP_AND_DATA_BOUNDARY.md` | Public/private data rules and quarantine behavior. |
| `docs/VALIDATION_STRATEGY.md` | Verification and validation families and release gate expectations. |
| `docs/_Decomposition/SOFTWARE_DECOMP.md` | Scope, objectives, package context, architecture basis, and open issues. |
| `docs/_Registers/Deliverables.csv` | DEL-11-02 identity and anticipated artifact. |
| `docs/_Registers/ScopeLedger.csv` | SOW-033 scope mapping and no-protected-examples note. |

