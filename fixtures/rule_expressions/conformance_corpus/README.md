# Rule Expression Conformance Corpus (TP-C1-GRAMMAR-001)

Blessed golden corpus for the frozen rule-pack expression grammar
(**DEC-022** / D-02 Option A), modeled on the operation contract corpus
precedent (`fixtures/model_operations/contract_corpus/`). Every
`case_*.json` file is executed by
`core/rules/expression_evaluator/tests/conformance_corpus.rs` under plain
`cargo test` in that crate directory and must reproduce its recorded
expected outcome exactly. **This corpus is the freeze artifact for grammar
v1.0.0**: a grammar change without a corresponding corpus extension fails
review, and any semantic change requires a `grammar_version` bump per the
DEC-022 versioning rule (minor = additive only; breaking = new major +
recorded human ruling).

## Provenance (IP policy)

| Field | Value |
|---|---|
| data_origin | All expressions, variable ids, tables, magnitudes, and unit references are **invented values** authored for this repository (e.g. a lookup table mapping `{10 -> 1.5, 20 -> 2.5, 40 -> 3.5}` in invented units). |
| provenance_class | `PUBLIC_DOMAIN_OR_ORIGINAL` |
| contributor_certification | Nothing in this corpus derives from protected standards content, vendor-proprietary data, or real project data. No code-specific allowables, SIFs, flexibility factors, or load-combination defaults appear anywhere. |
| engineering_status | Non-engineering test data; no release-readiness, professional-approval, certification, sealing, or code-compliance claim. |
| review_status | pending human review |

## Case file format

| Field | Meaning |
|---|---|
| `case_id` / `description` | Stable id (must equal the file name minus `.json`) and human description. |
| `declared_grammar_version` | The grammar version the simulated rule pack declares; accepting cases must declare the frozen version (`1.0.0`). |
| `expression` | Expression AST (see node encoding below). |
| `bindings` | Variable bindings: `variable_id`, `source` (`rule_pack_required_input` \| `user_supplied_value` \| `solver_result_field`), and `quantity` (object or `null` for declared-but-unsupplied). |
| `required_variable_ids` | Required-input ids for the completeness check. |
| `statuses` | Input `AnalysisStatus` tokens (snake_case). |
| `expected.value` | `null` when blocked, else `{"kind":"boolean","value":...}` or `{"kind":"quantity","value":...,"dimension":...,"unit_ref":...}` compared by exact `f64` equality on canonical outputs (corpus values are chosen to be exact in binary floating point; tolerance-bearing comparisons wait on the D-04 ruling). |
| `expected.findings` | Ordered list of `{code, subject_id}` records — diagnostic ordering determinism is itself under test. Messages are prose and excluded. |
| `expected.statuses` | Resulting status list (e.g. `human_approved_for_project` input maps to `human_review_required`). |
| `expected.source_variable_ids` | Sorted, deduplicated variable ids the evaluation touched. |

### Expression node encoding

```
{"node":"literal","quantity":{...}}
{"node":"variable_ref","variable_id":"..."}
{"node":"unary","operator":"negate|abs|not","operand":{...}}
{"node":"binary","operator":"add|subtract|multiply|divide","left":{...},"right":{...}}
{"node":"compare","operator":"less_than|less_than_or_equal|greater_than|greater_than_or_equal|equal|not_equal","left":{...},"right":{...}}
{"node":"logical","operator":"and|or","left":{...},"right":{...}}
{"node":"select","condition":{...},"then":{...},"else":{...}}
{"node":"aggregate","function":"min|max","operands":[{...}]}
{"node":"interpolate","table":{...},"argument":{...}}
{"node":"lookup","table":{...},"mode":"exact|step","argument":{...}}
{"node":"unsupported_form","form_id":"..."}
{"node":"unsafe_host_access","request":"..."}
```

Quantities: `{"value":N,"dimension":"<snake_case Dimension>","unit_ref":"..."}`
(optional `unit_required` / `dimension_check_required`, default `true`).
Tables: `{"table_id","argument_dimension","argument_unit_ref",
"result_dimension","result_unit_ref","rows":[{"argument":N,"result":N}]}` with
strictly increasing arguments.

The JSON reader lives in the test harness only; the evaluator itself never
parses text (the AST is the sole canonical form; a writable text syntax is
deferred to ruling D-02b).

## Coverage floor (enforced by the runner, not just documented)

- Every grammar feature has at least one accepting case: literal,
  variable_ref, all three unary operators, all four binary operators, all six
  comparisons, both logical operators, select, min, max, interpolate, and
  both lookup modes.
- Every relevant `FindingCode` has at least one blocking case, including the
  out-of-range table blocks (no extrapolation, no clamping), unrepresentable
  and ambiguous dimension products/quotients, and grammar-version blocking.
  `non_finite_input` is excluded (finite JSON cannot encode NaN/inf) and is
  pinned by in-crate unit tests instead.
- The corpus must not shrink below 60 cases, accepting cases must declare the
  frozen grammar version, and every case must evaluate identically on re-run
  (determinism check).

Checksum/grammar-version binding evidence lives in the sibling
`../checksum_binding/` corpus, executed by `core/rules/rule_pack_lifecycle`.

## Bless workflow

Expected outcomes were hand-derived from the frozen semantics and verified by
running the evaluator (no auto-bless mode exists). To extend the corpus: add
a case file with hand-derived expectations, run `cargo test` in
`core/rules/expression_evaluator`, and investigate any mismatch before
changing either side — never edit an expectation just to make a failing case
pass without understanding which side changed. Ambiguous divergences move to
a `_quarantine/` subfolder (excluded by the runner, which only reads
top-level `case_*.json`) with a `TBD` note for human ruling.

## Case inventory (69 cases)

| File | Expected outcome | Blocking codes |
|---|---|---|
| `case_01_accept_logical_and_range_check.json` | boolean | — |
| `case_02_accept_logical_or.json` | boolean | — |
| `case_03_accept_logical_not.json` | boolean | — |
| `case_04_accept_compare_equal_and_not_equal.json` | boolean | — |
| `case_05_accept_arithmetic_add_negate.json` | quantity | — |
| `case_06_accept_arithmetic_subtract_abs.json` | quantity | — |
| `case_07_accept_select_then_branch.json` | quantity | — |
| `case_08_accept_select_else_branch.json` | quantity | — |
| `case_09_accept_select_boolean_branches.json` | boolean | — |
| `case_10_accept_min_of_three.json` | quantity | — |
| `case_11_accept_max_of_three.json` | quantity | — |
| `case_12_accept_min_single_operand.json` | quantity | — |
| `case_13_accept_interpolate_midpoint.json` | quantity | — |
| `case_14_accept_interpolate_row_hit.json` | quantity | — |
| `case_15_accept_interpolate_lower_bound.json` | quantity | — |
| `case_16_accept_lookup_exact_match.json` | quantity | — |
| `case_17_accept_lookup_step_between_rows.json` | quantity | — |
| `case_18_accept_lookup_step_upper_bound.json` | quantity | — |
| `case_19_accept_multiply_force_length_moment.json` | quantity | — |
| `case_20_accept_multiply_length_length_area.json` | quantity | — |
| `case_21_accept_multiply_stress_section_modulus_moment.json` | quantity | — |
| `case_22_accept_multiply_thermal_strain_dimensionless.json` | quantity | — |
| `case_23_accept_multiply_dimensionless_scaling.json` | quantity | — |
| `case_24_accept_divide_moment_length_force.json` | quantity | — |
| `case_25_accept_divide_same_dimension_ratio.json` | quantity | — |
| `case_26_accept_divide_strain_by_temperature_interval.json` | quantity | — |
| `case_27_accept_divide_by_dimensionless.json` | quantity | — |
| `case_28_accept_nested_stress_utilization_check.json` | boolean | — |
| `case_29_accept_select_with_interpolate_branch.json` | quantity | — |
| `case_30_block_grammar_version_unsupported.json` | blocked | `unsupported_grammar_version` |
| `case_31_block_grammar_version_malformed.json` | blocked | `unsupported_grammar_version` |
| `case_32_block_grammar_version_missing.json` | blocked | `unsupported_grammar_version` |
| `case_33_block_logical_operand_type_mismatch.json` | blocked | `type_mismatch` |
| `case_34_block_not_of_quantity.json` | blocked | `type_mismatch` |
| `case_35_block_select_condition_not_boolean.json` | blocked | `type_mismatch` |
| `case_36_block_select_branch_dimension_mismatch.json` | blocked | `dimension_mismatch` |
| `case_37_block_select_branch_unit_mismatch.json` | blocked | `unit_mismatch` |
| `case_38_block_select_mixed_branch_types.json` | blocked | `type_mismatch` |
| `case_39_block_min_dimension_mismatch.json` | blocked | `dimension_mismatch` |
| `case_40_block_max_unit_mismatch.json` | blocked | `unit_mismatch` |
| `case_41_block_min_empty_operands.json` | blocked | `unsupported_expression_form` |
| `case_42_block_min_boolean_operand.json` | blocked | `type_mismatch` |
| `case_43_block_abs_of_boolean.json` | blocked | `type_mismatch` |
| `case_44_block_interpolate_below_range.json` | blocked | `table_out_of_range` |
| `case_45_block_interpolate_above_range.json` | blocked | `table_out_of_range` |
| `case_46_block_lookup_step_below_range.json` | blocked | `table_out_of_range` |
| `case_47_block_lookup_exact_above_range.json` | blocked | `table_out_of_range` |
| `case_48_block_lookup_exact_key_not_found.json` | blocked | `table_key_not_found` |
| `case_49_block_table_not_monotone.json` | blocked | `table_malformed` |
| `case_50_block_interpolate_single_row_table.json` | blocked | `table_malformed` |
| `case_51_block_lookup_empty_table.json` | blocked | `table_malformed` |
| `case_52_block_table_missing_unit_metadata.json` | blocked | `table_malformed` |
| `case_53_block_interpolate_argument_dimension_mismatch.json` | blocked | `dimension_mismatch` |
| `case_54_block_interpolate_argument_unit_mismatch.json` | blocked | `unit_mismatch` |
| `case_55_block_multiply_unrepresentable_product.json` | blocked | `unsupported_expression_form` |
| `case_56_block_divide_ambiguous_quotient.json` | blocked | `unsupported_expression_form` |
| `case_57_block_divide_unrepresentable_quotient.json` | blocked | `unsupported_expression_form` |
| `case_58_block_divide_by_zero.json` | blocked | `division_by_zero` |
| `case_59_block_unsafe_host_access.json` | blocked | `unsafe_construct` |
| `case_60_block_unsupported_form.json` | blocked | `unsupported_expression_form` |
| `case_61_block_missing_variable.json` | blocked | `missing_variable` |
| `case_62_block_missing_required_value.json` | blocked | `missing_required_value` |
| `case_63_block_duplicate_binding.json` | blocked | `duplicate_binding` |
| `case_64_block_invalid_empty_reference.json` | blocked | `invalid_reference` |
| `case_65_block_unit_metadata_missing.json` | blocked | `unit_metadata_missing`, `missing_variable` |
| `case_66_block_status_boundary_violation.json` | blocked | `status_boundary_violation` |
| `case_67_block_compare_dimension_mismatch.json` | blocked | `dimension_mismatch` |
| `case_68_block_compare_unit_mismatch.json` | blocked | `unit_mismatch` |
| `case_69_block_add_dimension_mismatch.json` | blocked | `dimension_mismatch` |
