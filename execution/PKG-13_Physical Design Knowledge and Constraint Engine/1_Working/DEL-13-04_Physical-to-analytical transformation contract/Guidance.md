# Guidance: DEL-13-04 Physical-to-analytical transformation contract

**Generated:** 2026-05-03
**Status:** Evidence refresh applied 2026-06-07
**Source posture:** Directional guidance is source-grounded where possible; unsupported implementation advice is marked `TBD` or omitted.

## Purpose

DEL-13-04 is the bridge from design authoring to solver execution. Its implemented purpose is to derive a deterministic `analytical_solver_model` from a schema-shaped physical source-of-truth model while preserving diagnostics, omissions, assumptions, and traceability when information cannot cross that boundary exactly.

Sources: `_CONTEXT.md` Scope Detail and Context Envelope; `execution/_Decomposition/SOFTWARE_DECOMP.md` DEL-13-04 and OBJ-014; `core/model_transform/physical_to_analytical/contract.py`; `tests/test_physical_to_analytical_transform.py`.

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

The sources require warnings for physical design data that cannot be represented analytically, and current implementation evidence supplies concrete diagnostic behavior. The transform emits deterministic `PTA-*` diagnostics for current coverage, including missing source/model role/coordinate data, missing required fields, missing or `TBD` unit metadata, noncanonical quantity dimensions, unsupported components/supports/elements, and unresolved references. The internal adapter emits deterministic `ASBA-*` diagnostics for strict solver-boundary DTO coverage, including missing properties, invalid `y_reference`, unsupported/noncanonical load semantics, unresolved targets, wrong dimensions, nonfinite quantities, unordered uniform-load spans, and wrong model role.

Guidance: treat those `PTA-*` and `ASBA-*` codes as current implementation/test evidence, not as the final release transform-loss taxonomy. The final taxonomy, release thresholds, and user-facing severity policy remain `TBD` until a governed architecture or release-readiness decision accepts them.

### Upstream Contract Surfaces

`Dependencies.csv` identifies prerequisite surfaces for canonical domain model schema, design knowledge, constraints, constraint validation, frame stiffness, supports/restraints, primitive loads, diagnostics/result envelopes, API boundaries, persistence/schema versioning, and layered tests. Those rows are evidence of coordination dependencies, not permission to copy or reinterpret sibling deliverable content.

Current upstream refresh evidence confirms DEL-13-02 has an implemented constraint entity/provenance schema at `schemas/constraint.schema.json`, and DEL-13-03 has an implemented validation engine at `core/constraints/validation/engine.py`. DEL-13-04 currently carries `design_knowledge_refs` and `constraint_refs` through the derived analytical model, but it does not invoke the validation engine, resolve owner criteria, or create a GUI/runtime/API integration path.

Guidance: implementation work should consume approved upstream contracts through explicit sealed scope and should not use this evidence refresh to reclassify dependencies, expand schema vocabulary, or infer owner/code criteria.

### Diagnostic Placement

The project vocabulary distinguishes diagnostics and warning classes from professional compliance outcomes. Current transform diagnostics use classes such as `SOLVE_BLOCKING`, `ASSUMPTION_WARNING`, and `NONLINEAR_WARNING`; adapter diagnostics classify blocking findings as `SOLVE_BLOCKING` and other adapter findings as diagnostic evidence only.

Guidance: transformation and adapter findings should be deterministic, source-linked, and reviewable. They should not state that the model is compliant, professionally accepted, release-ready, or externally proven.

### Data Boundary

Transformation may touch component, material, section, support, load, constraint, and design-knowledge references. Public artifacts must not supply protected or proprietary values for those surfaces. Unknown or suspected protected sources require review/quarantine rather than normalization into public defaults.

Guidance: tests should use invented or permitted fixtures and should include provenance where data matters.

## Trade-offs

| Trade-off | Conservative direction |
|---|---|
| Rich physical data vs. solver-ready idealization | Preserve physical richness through traceability and warnings rather than forcing all data into solver elements. |
| Convenience defaults vs. explicit findings | Prefer explicit `TBD`, warning, or diagnostic records over hidden defaults. |
| Contract specificity vs. unsupported invention | Specify behavior proven by SOW, decomposition, project invariants, current implementation, and focused tests; leave release taxonomy, external prover behavior, GUI/runtime/API integration, persistence/handoff readiness, and human/professional acceptance `TBD` where evidence does not fix them. |
| Analytical target breadth vs. global mechanics scope | Keep routine transformation aimed at 3D centerline/frame analysis. Treat shell/solid FEA as a separate handoff path. |

## Examples

Executable invented examples are available in `fixtures/domain/invented_physical_source_of_truth_model.json`, `tests/test_physical_to_analytical_transform.py`, and `tests/test_analytical_solver_boundary_adapter.py`. They are public-permissive test evidence, not owner/project examples or professional design examples. Publication-grade examples, owner/project examples, and any examples containing code/owner criteria remain `TBD` pending source clearance and human acceptance.

## Source Access Gaps

| Gap | Impact | Handling |
|---|---|---|
| PRD v0.2 Section8.3 / FR-MOD-007 not locally available through `_REFERENCES.md` | Cannot derive PRD-specific transform clauses beyond SOW-066 wording. | Keep PRD-specific particulars `TBD`. |
| Broader upstream integration beyond current DEL-13-02/DEL-13-03 refresh evidence | Cannot claim full runtime constraint-validation consumption, GUI presentation, public API behavior, or persisted handoff readiness. | Keep those integration surfaces `TBD` until bounded implementation evidence exists. |
| OI-012 unresolved architecture detail for loss classes | Transform-loss taxonomy cannot be finalized here. | Record `TBD`; require later architecture/detail task. |
