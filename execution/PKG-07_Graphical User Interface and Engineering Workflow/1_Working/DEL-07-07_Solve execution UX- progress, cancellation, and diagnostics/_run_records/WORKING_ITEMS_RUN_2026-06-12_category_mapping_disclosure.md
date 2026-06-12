---
run-id: WORKING_ITEMS_RUN_2026-06-12_category_mapping_disclosure
timestamp: 2026-06-12T08:35:00-06:00
persona: WORKING_ITEMS
tranche-id: TP-R2VERIFY-FIX-001
package-id: PKG-07
deliverable-id: DEL-07-07
run-status: SUCCESS
authority:
  - plans/VERIFICATION_2026-06-12_r2_exit_chain.md finding F-1
  - docs/CONTRACT.md no-silent-defaults posture
write-scope:
  - core/product_physics/src/lib.rs
  - execution/PKG-07_Graphical User Interface and Engineering Workflow/1_Working/DEL-07-07_Solve execution UX- progress, cancellation, and diagnostics/MEMORY.md
  - execution/PKG-07_Graphical User Interface and Engineering Workflow/1_Working/DEL-07-07_Solve execution UX- progress, cancellation, and diagnostics/_run_records/**
  - plans/PLAN_COMPLETION_LOG.md
---

# WORKING_ITEMS Run - TP-R2VERIFY-FIX-001

## Scope

Repair of verification finding F-1: the A12 adapter coerced operation-authored
load categories (`concentrated_force`/`concentrated_moment` → `occasional`,
`distributed_force` → `weight`) in `parse_category` with no in-product
disclosure — a silent vocabulary collapse assigning stress-practice-meaningful
classification the user did not choose.

## Changes

- Added `authored_category_preview_mapping` beside `parse_category` naming the
  three coerced vocabulary values and their preview categories.
- The load-construction path now emits a per-load `warning` diagnostic
  `LOAD_CATEGORY_PREVIEW_MAPPED` ("…the preview classification is not a
  user-selected engineering classification") with the load and load-case ids
  in `affected_refs`. Mechanical behavior is unchanged; native preview
  categories emit nothing.
- Extended `operation_authored_primitive_categories_map_to_preview_mechanics`
  to pin the diagnostic code, `warning` severity, message content, affected
  refs, unchanged `MECHANICS_SOLVED` status, and the native-category negative
  case.

## Validation

- `cargo test --manifest-path core/product_physics/Cargo.toml` — PASS, 25/25.
- `cargo fmt --manifest-path core/product_physics/Cargo.toml --check` — PASS.
- Five-surface DEC-025 sweep at session closeout HEAD (summary committed with
  the closeout evidence commit).

## Boundary review

No mechanical solve change, no schema change, no lifecycle, release,
professional, certification, sealing, authentication, or code-compliance
claim. The diagnostic makes an existing mapping visible; it does not validate
the mapping as an engineering classification.

## Handoffs

- When combination-basis semantics or Phase D component depth make category
  mechanically load-bearing, revisit whether authored categories should be
  carried first-class through `PrimitiveLoadCategory` instead of mapped.
- Desktop diagnostics panel renders `warning` diagnostics generically; no UI
  change was needed. Playwright spec extension is not applicable because
  browser mode cannot solve edited models (recorded per the H4 exception
  pattern).
