# Datasheet: DEL-07-05 Results viewer

## Identification

| Field | Value |
|---|---|
| Deliverable ID | DEL-07-05 |
| Deliverable name | Results viewer |
| Package ID | PKG-07 |
| Package name | Graphical User Interface and Engineering Workflow |
| Deliverable type | UX_UI_SLICE |
| Scope item | SOW-023 |
| Objectives | OBJ-006, OBJ-007 |
| Context envelope | L |
| Current source basis | `_CONTEXT.md`; `docs/_Decomposition/SOFTWARE_DECOMP.md` revision 0.4; `docs/_Registers/Deliverables.csv`; `docs/_Registers/ScopeLedger.csv`; `docs/CONTRACT.md`; `docs/SPEC.md`; `docs/TYPES.md`; `docs/DIRECTIVE.md` |

## Attributes

| Attribute | Setup value | Source |
|---|---|---|
| Intended GUI surface | Results viewer for tabular and graphical review | `_CONTEXT.md`; `docs/SPEC.md` section 7 |
| Result categories in scope | Displacements, rotations, forces, moments, restraint reactions, equipment loads, stresses, and ratios | SOW-023 in `docs/_Registers/ScopeLedger.csv`; `docs/SPEC.md` section 8 |
| Architectural route | GUI reads schema-first command/query/job result envelopes through application services | `_CONTEXT.md` Architecture Basis Injection |
| Unit behavior | Result values must remain unit-aware and dimensionally checked | OPS-K-UNIT-1 in `docs/CONTRACT.md`; `docs/DIRECTIVE.md` section 3 |
| Diagnostics behavior | Missing data, assumptions, provenance, nonlinear/convergence issues, and IP boundary warnings must remain visible | `docs/SPEC.md` section 7; OPS-K-DATA-2 and OPS-K-AUTH-1 in `docs/CONTRACT.md` |
| Rule-pack ratio behavior | Ratios are displayed only when a user-supplied rule pack and required inputs support the check; incomplete rule input remains an explicit finding | SOW-023 note; OPS-K-DATA-1/2 and OPS-K-RULE-1/2/3 in `docs/CONTRACT.md` |
| Report/export relationship | Viewer content should remain reproducible and handoff-ready for reports and structured result exports | OBJ-007; SOW-046; `docs/SPEC.md` section 8 |
| Protected-data posture | No protected standards text, protected tables, copied formulas, proprietary thresholds, or certification claims are introduced | OPS-K-IP-1/2/3 and OPS-K-AUTH-1 in `docs/CONTRACT.md`; `docs/DIRECTIVE.md` sections 3-5 |

## Conditions

| Condition | Status |
|---|---|
| This setup produces GUI planning documents, not GUI source code | FACT |
| Exact UI component library, state library, and visual layout details | TBD |
| Exact result-envelope schema fields for each displayed category | TBD, owned by schema/result-envelope implementation deliverables |
| Exact stress-ratio formulas, thresholds, allowables, or code categories | Out of scope for public defaults; user/rule-pack supplied |
| Professional acceptance or code compliance status | Out of software authority; human review remains required |

## Construction

The results viewer deliverable should be framed as a review surface over already-produced mechanical, diagnostic, stress, reaction, equipment-load, and user-rule-check result envelopes. It should not compute solver mechanics, recover stresses, evaluate rule packs, define protected code criteria, or export final reports directly unless a later sealed implementation brief expands scope.

The viewer setup must preserve these visible boundaries:

- mechanics solved is distinct from rule-pack checked;
- rule-pack pass/fail is distinct from professional approval;
- missing solve-required or rule-check-required data is a displayed finding;
- every numerical value remains unit-aware;
- result displays carry warnings, assumptions, and provenance hooks;
- public examples and templates use no protected standards data or proprietary engineering values.

## References

- `INIT.md` - bootstrap boundaries and unknown-value rule.
- `AGENTS.md` - Type 2 sealed dispatch rule.
- `docs/CONTRACT.md` - invariants OPS-K-DATA-1/2/3, OPS-K-UNIT-1, OPS-K-RULE-1/2/3, OPS-K-AUTH-1, OPS-K-IP-1/2/3, OPS-K-PRIV-1/2, OPS-K-AGENT-1..4.
- `docs/SPEC.md` - architecture, GUI, warnings, reporting, acceptance semantics.
- `docs/TYPES.md` - analysis-status vocabulary and Result/Report object boundary.
- `docs/DIRECTIVE.md` - product principles, stop rules, and professional boundary.
- `docs/_Decomposition/SOFTWARE_DECOMP.md` - package, objective, and architecture-basis context.
- `docs/_Registers/Deliverables.csv` - DEL-07-05 row.
- `docs/_Registers/ScopeLedger.csv` - SOW-023 row.
