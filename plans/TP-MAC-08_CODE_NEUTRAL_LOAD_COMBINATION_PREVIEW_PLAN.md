---
doc_id: TP-MAC-08
doc_kind: implementation.plan
status: implemented
created: 2026-05-10
closed: 2026-05-10
---

# TP-MAC-08 Code-Neutral Load Combination Preview

## Purpose

Add the first governed load-combination preview slice: explicit user-defined
linear mechanics combinations over solved preview load cases.

TP-MAC-08 bridges the preview model to the existing
`core/loads/load_case_algebra` crate without introducing code defaults,
protected rule criteria, compliance checks, or professional acceptance
behavior.

## Scope

- Add optional preview model input
  `combinations: [{ id, label, basis, terms, provenance }]`.
- Support only `basis: "mechanics"` in this tranche.
- Solve each `load_cases[]` record independently.
- Preserve legacy unqualified result IDs for the first/default load case.
- Emit non-default load-case result rows as
  `result:loadcase:{load_case_suffix}:{base_result_tail}`.
- Emit combination rows as
  `result:combination:{combination_suffix}:{base_result_tail}`.
- Add `basis_ref` to emitted result rows for load-case and combination
  auditability.
- Add `source_result_refs` to combination result rows.
- Combine matching scalar displacement, reaction, force, moment, and stress
  component rows using explicit user factors through
  `open_pipe_stress_load_case_algebra::evaluate_linear_combination`.
- Skip `open_formula_stress_summary` rows for combinations with an explicit
  diagnostic.
- Label combined force, moment, and stress rows with metadata basis
  `explicit_user_linear_combination`.
- Update invented fixtures, schema metadata enum, analysis-run/report selected
  refs, persistence run-history refs, desktop display, and tests.

## Boundaries

- No code-specific public default combinations or factors.
- No rule-pack checks, protected criteria, allowables, SIF/flexibility tables,
  owner design-basis data, or private project data.
- No combination editor, final CLI syntax, desktop save/open UX, durable
  storage workflow, or external execution.
- No pressure-to-frame conversion, range envelopes, result-state subtraction,
  expression language, arbitrary station sweeps, shear recovery, equivalent or
  principal stress.
- Combination rows do not alter summary maxima; summary fields remain tied to
  the default preview solve.
- Use invented or explicitly cleared data only.

## Diagnostics

Structured diagnostics are emitted for:

- missing combination provenance;
- duplicate or empty IDs;
- unsupported combination basis;
- empty terms;
- non-finite factors;
- unknown load cases;
- skipped `open_formula_stress_summary` combination rows.

## Acceptance Criteria

- The invented preview model includes a second invented load case and one
  explicit mechanics-basis combination.
- The product-preview solve emits default unqualified load-case rows,
  qualified second-load-case rows, and qualified combination rows.
- Combination rows carry `basis_ref`, `source_result_refs`, and
  `explicit_user_linear_combination` metadata where applicable.
- TP-MAC-07 midspan labels remain intact; endpoint-pair display remains
  endpoint-only for midspan rows.
- Desktop result detail displays explicit combination basis and source result
  refs.
- The mechanics gap ledger marks only explicit mechanics-basis combinations
  implemented while code/rule combinations remain deferred.
- Analysis-run/report/persistence hashes and refs include combination rows.

## Verification

- `cargo test --manifest-path core/loads/load_case_algebra/Cargo.toml`
- `cargo test --manifest-path core/product_physics/Cargo.toml`
- `npm run generate:product-preview-mechanics`
- `python3 -m pytest tests/product_preview/test_product_preview_service.py tests/test_analysis_run_records.py tests/test_results_schema.py tests/test_project_persistence_service.py`
- `python3 tests/test_results_schema.py`
- `npm test --workspace apps/desktop`
- `npm run build --workspace apps/desktop`
- Browser smoke of the desktop preview combination row and report load-basis
  refs at `http://127.0.0.1:5173/`
- `git diff --check`

## Closeout

TP-MAC-08 is implemented for the current product-preview workflow.

Delivered:

- independent per-load-case solves with legacy first-load-case IDs preserved;
- qualified additional load-case result rows;
- explicit user mechanics combination result rows through the existing
  load-case algebra crate;
- load-case and combination `basis_ref` audit metadata;
- combination `source_result_refs`;
- blocking validation and runtime diagnostics for invalid combination inputs;
- stress-summary skip diagnostics for unsupported combination summary rows;
- updated invented mechanics and persistence fixtures with hashes;
- desktop result/detail/report support for explicit combination rows;
- mechanics gap ledger marking explicit mechanics-only combinations
  implemented while code/rule combinations remain deferred.

The desktop build no longer emits the prior Vite chunk-size warning after lazy
fixture loading and vendor chunk splitting. Code/rule combinations, protected
checks, pressure-to-frame conversion, final CLI syntax, desktop save/open UX,
durable storage workflow, external execution, release/professional claims, and
professional acceptance remain deferred.
