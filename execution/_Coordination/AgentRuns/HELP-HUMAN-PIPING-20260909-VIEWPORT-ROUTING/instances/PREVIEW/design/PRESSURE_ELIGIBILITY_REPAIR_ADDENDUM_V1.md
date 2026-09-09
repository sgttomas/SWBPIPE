# Pressure eligibility repair addendum V1

Status: `FROZEN`
Basis: committed source `533332349a4607eee561d4ef90fb05a62d86519e`
Verdict: confirmed bounded producer defect; repair may proceed independently of pressure/Poisson/schema redesign

## Source proof and reachability

`core/product_physics/src/lib.rs:6618-6624` admits a pressure-thrust input only when both `load.category == "pressure"` and `load.dimension == "pressure"`. `pressure_for_pipe` at `:9319-9333` instead sums every element-targeted record with `dimension == "pressure"`, without the category condition, and feeds that scalar to stress recovery.

The mismatch is reachable. Upstream `build_load_case_primitive_loads` at `:5146-5221` parses category and dimension independently and does not validate a category/dimension pairing. Supported non-pressure categories such as `hydrotest` therefore accept `dimension=pressure`. Both straight and general uniform-load paths deliberately exclude every pressure-dimensioned primitive (`:6296-6301`, `:6450-6455`), so such a record creates no ordinary distributed load and no pressure thrust, yet current `pressure_for_pipe` creates hoop/longitudinal pressure stress. This is a phantom stress-only pressure state, not merely an unreachable malformed-input case.

## Minimal repair

In `core/product_physics/src/lib.rs`, make stress pressure eligibility identical to thrust pressure eligibility:

```text
category == "pressure"
and dimension == "pressure"
and target is the same resolved element
```

Prefer one private predicate used by both `build_pressure_thrust_loads` and `pressure_for_pipe` so the two paths cannot drift again. Keep the existing sum of all eligible genuine pressure records on the element. Do not add a schema field, diagnostic, topology inference, Poisson behavior, result kind, or public quantity.

## Focused oracles

Add co-located product-physics tests using a supported non-pressure category so the case reaches the solve:

1. Mutate an otherwise valid element pressure record to `category="hydrotest"`, retaining `dimension="pressure"`. Require the same solve status the current independent parser permits, no pressure-thrust displacement/reaction change, and no `pipe_section_pressure_hoop_stress` or `pipe_section_pressure_longitudinal_stress` endpoint/station rows from that record.
2. Keep `category="pressure", dimension="pressure"` and require the existing genuine-pressure result leaves to remain unchanged.
3. Put two genuine pressure records on the same pipe and require their signed magnitudes to sum once in thrust and stress recovery. Use unequal values so last-wins and duplicate-count mutations fail.
4. Put one genuine pressure record plus one supported non-pressure/pressure-dimension record on the same pipe. Require only the genuine record to contribute.

If a later change adds category/dimension-pair validation and blocks the mixed record before solve, replace oracle 1 with the blocking diagnostic at that boundary. Under the reviewed committed source it is not blocked, so the current repair must test the reachable solve behavior.

## Fence and interaction

Exact source fence: `core/product_physics/src/lib.rs` and its co-located tests. Regenerate a fixture only if a focused existing fixture actually contains the mixed-category condition; no current evidence requires fixture or schema changes.

This repair composes with the endpoint section-cut change already in progress: pressure eligibility determines whether pressure stress components exist; endpoint/station action-to-cut conversion determines the signs of force-derived stress components. Neither repair changes raw endpoint-action rows.
