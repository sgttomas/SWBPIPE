# TASK brief — T1_WP6_STATIC_CASES (VP-STATIC load/reference-state cases: authoring only)

Read `_T1_COMMON.md` first. Return folder: `LSI/T1_WP6_STATIC_CASES/`.

## Assignment

Author the VP-STATIC case package for three families, from the existing independent analytical references in `core/product_physics/tests/fixtures/load_reference_states/reference_cases.json` (read its `README.md`). The families are support motion (M10), reference temperatures (M16) and cold spring (M29).

This brief is **authoring only**:

- no comparison is run;
- no observed output is recorded;
- no reference or criterion is admitted.

An independent freeze check follows before any run.

**Cases.** Take them from `T1_PLAN.md` §2, WP6:

| Family | Reference cases |
|---|---|
| Support motion | `prescribed_translation_two_bar`, `prescribed_translation_all_fixed`, `prescribed_rotation_all_fixed`, `prescribed_rotation_free_tip` |
| Reference temperatures | `thermal_datum_ratio`, `coefficient_definition`, `constant_alpha_interval`, `shared_material_serial_companion` (the practical companion of the analytical-only `shared_material_parallel`; record that relationship), `multi_segment_free_length`, `temperature_unit_identity` |
| Cold spring | `signed_fit_states`, `persistent_source_once` |

Where a reference case holds several states (for example cold, hot, return and released), each state is its own qualification case or its own load case, whichever the product can represent. State which.

**For each case:**

1. An invented 0.4.0 product request (`<case>.preview_request.json`) that represents the reference's geometry, supports, materials, states and loads. Use the OD 0.20 m / wall 0.01 m companions the reference file provides, and state every representability limit. Where the product cannot represent a reference exactly, say so and exclude the case with the reason; never approximate silently.
2. A runner input (`<case>.runner_input.json`) in the `openpipestress-runner solve` request format of `validation/qualification/fixtures/first_static/*.runner_input.candidate.json`.
3. Candidate files:
   - selectors (`openpipestress.first_static_selector_candidate/1`, `producer_contract` `openpipestress.result_semantics/0.3.0/load-reference-1`);
   - reference values (`openpipestress.qualification_reference_values/1`, `readiness` pending independent review);
   - criteria (`tolerance_profile`, `profile_status` `draft_pending_independent_review`).
4. Reference values are computed **at test time from `reference_cases.json`** by a small generator script you commit in the package. Take `value` or `exact`; do not hand-copy constants. Record the generator and its output hash.
5. Include all six reactions and governing locations where the reference defines them. Turn the reference's `wrong_result_discriminators` into negative assertions, in a format you specify.
6. **Criteria.** Propose them per assertion family, citing the reference README's consumer rule: `|observed − expected| ≤ 1e-9·|expected|`, with dimension-aware zero-reference handling. They stay candidates; no new or relaxed tolerance.

**Manifest.** Write `validation/qualification/fixtures/load_reference/MANIFEST.json` in the format the parallel T1_WP5_HARNESS_ADAPTER brief defines:

```
{"format": "openpipestress.load_reference_qualification_manifest/1",
 "cases": [{"case_id", "runner_input": {path, sha256}, "product_request": {path, sha256},
            "reference": {path, sha256}, "selectors": {path, sha256}, "criteria": {path, sha256},
            "analytical_reference": {"path": "core/product_physics/tests/fixtures/load_reference_states/reference_cases.json", "sha256", "case_key"},
            "modes": ["sparse_interactive", "dense_scrutiny"], "required_scalar_rows": N}]}
```

Also write a `README.md` and a `PROVENANCE.json`.

**Selectors.** Selectors address result rows by the load-reference-1 table signatures, and, where needed, fields of `contract_evidence.load_reference_states`. To learn the exact row IDs, kinds and metadata the product emits, you **may** run the producer example on your own requests, for **identification only**: `cargo run --example physics_source_connected -- <sparse|dense> <request>` in `core/product_physics`. Record in the package that no observed value is used as a reference or criterion, and do not commit observed values. If a request is refused by the product, report the refusal. Do not change the request to hide a product defect; that goes to the manager.

## Write boundary

- `validation/qualification/fixtures/load_reference/**` (new).
- Your return folder.

## Checks

- Every manifest hash matches.
- Every request parses and every runner input is well-formed.
- The generator reproduces the reference values byte for byte.
- Every selector names a row kind present in the load-reference-1 table.
- For each case, list the assertions and why each is required.
