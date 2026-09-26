# TASK brief — T1_WP6_FREEZE (independent freeze check of the VP-STATIC package)

Read `_T1_COMMON.md` first. Return folder: `LSI/T1_WP6_FREEZE/`.

You are a **fresh-context, non-author** checker. You wrote none of the package and none of the analytical references. You delegate nothing.

## Candidate

- The package is `validation/qualification/fixtures/load_reference/` at commit `91ec30630` or later. It contains `MANIFEST.json`, `README.md`, `PROVENANCE.json`, `generate_reference_values.py` and 14 cases × 5 files.
- The author's return is `LSI/T1_WP6_STATIC_CASES/RETURN.md`, and its §7 lists the freeze items.
- The independent references are `core/product_physics/tests/fixtures/load_reference_states/reference_cases.json` and its `README.md`.
- The harness conventions are in `LSI/T1_WP5_HARNESS_ADAPTER/RETURN.md` §3.

## What you decide

This is the HARNESS_CUT "freeze before comparisons". For each case, and each assertion family, decide **ADMIT**, **ADMIT WITH CHANGES** (say exactly what) or **STRIKE**, with reasons.

1. **Representation.** Does each 0.4.0 request represent its reference's geometry, supports, materials, states and loads? Consider units, signs, the OD 0.20 m / wall 0.01 m companion, installation and operating temperatures, and fit. List every invented input and whether it changes what the reference asserts.
2. **Reference values.** Rerun `generate_reference_values.py --check`, and verify byte reproduction. Independently recompute at least one assertion per case from the reference's `exact` values and its declared transforms: negate, `one_plus`, the As/area ratio and m→mm. Do not use the generator's code path for this.
3. **Selectors.** Is each selector the right physical quantity: kind, entity, frame, location, sign convention and definition? Check the derived mappings in RETURN §7.3: end_i axial = −N; the fixed-rotation element end actions; N = `combined_rhs`; root Fx = −`two_distinct_equal_actions_rhs`; datum stretch = 1 + dilation or 1 + integral.
4. **Equilibrium zeros (148).** For each support, is a zero component truly implied by the reference's topology and loads? Strike any that is not.
5. **Criteria.** The relative criterion is 1e-9 for nonzero references, per the reference README consumer rule. The harness classifier is `|Δ| ≤ max(abs, rel·max(|obs|,|exp|))`. Are the `zero_scale:<tag>` absolutes (1e-9 × a named same-dimension reference magnitude) appropriate? Is each named magnitude the right scale? No criterion may be looser than the protected product tests (`load_reference_state_runtime.rs`, `close()`) for the same quantity.
6. **Negatives.** Is each wrong value the reference's discriminator, correctly transformed, and is it outside the tolerance of the correct value?
7. **Gaps and exclusions.** Is excluding `shared_material_parallel` right? Are the unscored gaps honest, and none of them hiding a required quantity?

## Rules

- You must **not** run the product, the runner, or any comparison against observed values. You judge the package against the references and physics only.
- You may run the package's own `check_package.py` in its no-producer mode, if it has one, and the generator.
- Make no edits to the package. Record every change you require in your return as an exact list: file, JSON pointer, old → new, reason. The manager applies it.

## Write boundary

Only `LSI/T1_WP6_FREEZE/`.

## Return

`LSI/T1_WP6_FREEZE/RETURN.md` must contain:

- a per-case decision table;
- the exact change list;
- your independent recomputations;
- the struck assertions;
- open questions;
- an overall verdict: FREEZE (admit as changed) or NOT FREEZABLE (why).

Then `SendMessage` the manager.
