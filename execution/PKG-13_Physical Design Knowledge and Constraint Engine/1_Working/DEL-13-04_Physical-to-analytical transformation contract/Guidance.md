# Guidance: DEL-13-04 Physical-to-analytical transformation contract

**Generated:** 2026-05-03
**Status:** Initial draft from four-documents P1/P2
**Source posture:** Directional guidance is source-grounded where possible; unsupported implementation advice is marked `TBD` or omitted.

## Purpose

DEL-13-04 is the bridge from design authoring to solver execution. Its purpose is to define how a richer physical source-of-truth model becomes a solver-ready analytical model while preserving warnings, omissions, assumptions, and traceability when information cannot cross that boundary exactly.

Sources: `_CONTEXT.md` Scope Detail and Context Envelope; `execution/_Decomposition/SOFTWARE_DECOMP.md` DEL-13-04 and OBJ-014.

## Principles

| Principle | Guidance | Source |
|---|---|---|
| Preserve source-of-truth role | Treat the physical model as the editable source-of-truth and the analytical model as a derived view. Do not overwrite physical design context during transformation. | `docs/SPEC.md` section 3; `docs/TYPES.md` `ModelRole` |
| Be deterministic | The same physical input and contract configuration should yield the same analytical output and warning set. | SOW-066; `docs/CONTRACT.md` OPS-K-SOLVER-1 |
| Warn rather than hide loss | When physical data cannot be represented analytically, expose the loss as a warning/diagnostic with traceability. | SOW-066; `docs/TYPES.md` `Diagnostic` |
| Keep units explicit | Unit-bearing values crossing the boundary need explicit unit metadata; missing or ambiguous units are findings. | `docs/SPEC.md` section 4; `docs/CONTRACT.md` OPS-K-UNIT-1 |
| Avoid silent engineering defaults | Missing solve-required values must not be silently supplied. | `docs/CONTRACT.md` OPS-K-DATA-2 |
| Stay code-neutral | Do not introduce protected code data, proprietary values, code-specific public defaults, SIF/flexibility tables, or compliance claims. | `docs/CONTRACT.md`; `docs/IP_AND_DATA_BOUNDARY.md` |
| Target global frame mechanics | The default analytical target is the project's 3D centerline/frame mechanics boundary, not routine shell/solid FEA. | `docs/CONTRACT.md` OPS-K-MECH-1; `INIT.md` |

## Considerations

### Transform-Loss Classes

The sources require warnings for physical design data that cannot be represented analytically, but they do not define a final transform-loss taxonomy. Open issue OI-012 explicitly says physical-to-analytical transformation loss classes need technical architecture detail before implementation briefs are sealed.

Guidance: keep loss classes `TBD` until an implementation task reads the relevant upstream contracts and receives an approved taxonomy. Candidate categories must not be invented in this setup document.

### Upstream Contract Surfaces

`Dependencies.csv` identifies prerequisite surfaces for canonical domain model schema, design knowledge, constraints, constraint validation, frame stiffness, supports/restraints, primitive loads, diagnostics/result envelopes, API boundaries, persistence/schema versioning, and layered tests. Those rows are evidence of coordination dependencies, not permission to copy or reinterpret sibling deliverable content.

Guidance: implementation work should consume the approved upstream contracts through explicit sealed scope and should not use this setup pass to resolve sibling details.

### Diagnostic Placement

The project vocabulary distinguishes diagnostics and warning classes from professional compliance outcomes. Missing data warnings include `SOLVE_BLOCKING`, `PROVENANCE_WARNING`, `ASSUMPTION_WARNING`, and other classes in `docs/SPEC.md`; exact transform warning code names remain `TBD`.

Guidance: transformation findings should be deterministic, source-linked, and reviewable. They should not state that the model is compliant or professionally accepted.

### Data Boundary

Transformation may touch component, material, section, support, load, and rule-pack references. Public artifacts must not supply protected or proprietary values for those surfaces. Unknown or suspected protected sources require review/quarantine rather than normalization into public defaults.

Guidance: tests should use invented or permitted fixtures and should include provenance where data matters.

## Trade-offs

| Trade-off | Conservative direction |
|---|---|
| Rich physical data vs. solver-ready idealization | Preserve physical richness through traceability and warnings rather than forcing all data into solver elements. |
| Convenience defaults vs. explicit findings | Prefer explicit `TBD`, warning, or diagnostic records over hidden defaults. |
| Contract specificity vs. unsupported invention | Specify behavior proven by SOW, decomposition, and project invariants; leave schema fields, warning taxonomy, and module paths `TBD` when sources do not fix them. |
| Analytical target breadth vs. global mechanics scope | Keep routine transformation aimed at 3D centerline/frame analysis. Treat shell/solid FEA as a separate handoff path. |

## Examples

No source-grounded transform examples are locally available for DEL-13-04. Example physical models, transform cases, warning examples, and fixtures are `TBD` and must be invented/public-permissive or otherwise source-cleared before use.

## Source Access Gaps

| Gap | Impact | Handling |
|---|---|---|
| PRD v0.2 Section8.3 / FR-MOD-007 not locally available through `_REFERENCES.md` | Cannot derive PRD-specific transform clauses beyond SOW-066 wording. | Keep PRD-specific particulars `TBD`. |
| Exact upstream contract details not read from sibling DEL folders in this workflow | Cannot define final input/output schema fields or warning taxonomy here. | Use local DAG mirror as dependency evidence only; defer exact details to sealed implementation. |
| OI-012 unresolved architecture detail for loss classes | Transform-loss taxonomy cannot be finalized here. | Record `TBD`; require later architecture/detail task. |
