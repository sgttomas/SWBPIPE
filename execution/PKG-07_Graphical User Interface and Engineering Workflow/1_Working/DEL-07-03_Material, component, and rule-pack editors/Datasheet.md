# Datasheet: DEL-07-03 Material, component, and rule-pack editors

## Identification

| Field | Value |
|---|---|
| Deliverable ID | DEL-07-03 |
| Package ID | PKG-07 |
| Package | Graphical User Interface and Engineering Workflow |
| Type | UX_UI_SLICE |
| Current setup scope | Document/setup production only; no GUI source implementation |
| Register scope item | SOW-021 |
| Supported objective | OBJ-006 |
| Context envelope | L |
| Context risk | WATCH - confirm scope and split if editor scope expands |

Sources: `_CONTEXT.md` sections "Scope Coverage", "Objective Support", "Context Envelope"; `docs/_Registers/Deliverables.csv` row `DEL-07-03`; `docs/_Registers/ScopeLedger.csv` row `SOW-021`.

## Attributes

| Attribute | Source-grounded value |
|---|---|
| Product surface | GUI editor workflow for materials, sections, components, load cases, supports, rule packs, and private libraries. |
| Deliverable focus | Private material editors, component editors, and rule-pack reference editors. |
| Related editor surfaces | Section, load-case, support/restraint, and private-library editing are part of SOW-021 and must remain visible in the workflow model even if later split. |
| Anticipated artifacts | Editor panels; validation UI tests. |
| GUI runtime baseline | Rust core/application services; Tauri 2 desktop shell; TypeScript/React/Vite GUI where GUI-facing. Exact dependency versions and component/state-management libraries remain TBD. |
| Mutation route | GUI mutations route through application-service commands; durable project state is separate from transient session, selection, and job-progress state. |
| Diagnostic route | Editor validation must surface result-envelope diagnostics and warning classes rather than silently normalizing missing or private data. |
| Data boundary | Code-specific and proprietary material, component, allowable, SIF/flexibility, and rule-pack values are user-supplied or private, not public defaults. |

Sources: `docs/SPEC.md` sections 1, 3, 6, 7; `docs/_Decomposition/SOFTWARE_DECOMP.md` sections 4, 8; `_CONTEXT.md` section "Architecture Basis Injection"; `docs/CONTRACT.md` invariants `OPS-K-DATA-1`, `OPS-K-DATA-2`, `OPS-K-DATA-3`, `OPS-K-RULE-3`, `OPS-K-PRIV-1`.

## Conditions

| Condition | Required posture |
|---|---|
| Protected standards data | Do not include protected standards text, tables, examples, code-derived formulas, material allowables, SIF/flexibility tables, protected dimensional tables, or proprietary commercial data. |
| Missing data | Missing solve-required or rule-check-required values are explicit findings, never silent defaults. |
| Provenance | Materials, components, SIFs, flexibility factors, allowables, and rule-pack values carry provenance fields. |
| Units | Editor inputs, validation, imports, exports, and rule-pack references must be unit-aware and dimensionally checked. |
| Rule packs | Rule packs are versioned, checksummed, source-noted, marked public/private, and evaluated by sandboxed unit-aware logic outside this GUI setup deliverable. |
| Professional boundary | UI copy and workflow state must not claim certification, sealing, approval, authentication, or engineering code compliance for reliance. |
| Scope split risk | Multiple editors remain in one GUI domain for setup, but later implementation may require split work if the surface expands. |

Sources: `docs/CONTRACT.md` invariants `OPS-K-IP-1`, `OPS-K-DATA-1`, `OPS-K-DATA-2`, `OPS-K-DATA-3`, `OPS-K-UNIT-1`, `OPS-K-RULE-1`, `OPS-K-RULE-2`, `OPS-K-RULE-3`, `OPS-K-AUTH-1`; `docs/DIRECTIVE.md` sections 2, 3, 5; `docs/_Registers/ContextBudgetQA.csv` row `DEL-07-03`.

## Construction

The editor workflow is a future GUI slice, but this setup pass records the intended construction boundaries:

| Work surface | Construction note |
|---|---|
| Material editor | Presents unit-bearing material fields and provenance/source status without shipping public protected allowables or proprietary material libraries. |
| Section editor | Presents user-entered or lawfully imported section dimensions/properties with provenance and unit checks; no protected dimensional defaults. |
| Component editor | Presents component type, geometry, user modifiers, manufacturer/user data references, and provenance/private status. |
| Load-case editor | Presents primitive and user-defined load case inputs with unit-aware validation; code-specific combinations remain user/rule-pack supplied. |
| Support/restraint editor | Presents support type, directions, stiffness/gap/friction fields, and active-state result hooks without hiding nonlinear uncertainty. |
| Rule-pack reference editor | Presents rule-pack identity, version, checksum, source notice, redistribution status, required inputs, and missing-input findings. |
| Private library editor | Presents private/local library references and provenance/redistribution status while preserving local-first/private-data posture. |
| Validation UX | Surfaces `SOLVE_BLOCKING`, `RULE_CHECK_BLOCKING`, `PROVENANCE_WARNING`, `ASSUMPTION_WARNING`, `NONLINEAR_WARNING`, and `IP_BOUNDARY_WARNING` classes when applicable. |

Sources: `docs/SPEC.md` sections 3, 6, 7; `docs/TYPES.md` sections 4, 7, 8; `docs/_Decomposition/SOFTWARE_DECOMP.md` architecture basis rows `AB-00-03`, `AB-00-05`, `AB-00-06`, `AB-00-07`.

## References

- `_CONTEXT.md` - sealed deliverable identity, architecture basis injection, and write-scope context.
- `_REFERENCES.md` - preparation reference index.
- `INIT.md` - bootstrap and project boundaries.
- `AGENTS.md` - OpenPipeStress agent index and dispatch rule.
- `docs/DIRECTIVE.md` - founding intent, product boundaries, and stop rules.
- `docs/CONTRACT.md` - invariant catalog.
- `docs/TYPES.md` - statuses, data provenance labels, and domain object vocabulary.
- `docs/SPEC.md` - architecture, domain object, rule-pack, GUI, report, and validation requirements.
- `docs/AGENTIC_DEVELOPMENT_WORKFLOW.md` - Type 2 execution and deliverable document kit.
- `docs/_Decomposition/SOFTWARE_DECOMP.md` - accepted v0.4 decomposition basis.
- `docs/_Registers/Deliverables.csv`, `ScopeLedger.csv`, `ContextBudgetQA.csv` - machine-readable deliverable, scope, and context records.
