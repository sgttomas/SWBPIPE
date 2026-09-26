# T1 wave 1: manager rulings made during execution

These are the T1 WORKING_ITEMS manager's technical rulings within the D1–D4 authority (`OWNER_T1_DECISIONS_2026-09-26.md`). They were sent to the TASKs by message.

## 1. T1_WP1_JOINED_SCHEMAS

**(1) Test edit granted.** The TASK may make one minimal edit to `tests/test_load_reference_schema.py::test_carrier_branches_are_appended_after_the_existing_methods`, and nothing else in that file.

- The test pins the exact branch counts, so any appended branch fails it.
- After the edit, the five existing oneOf entries and the load-reference-1 anyOf and SemanticContract entries must stay exactly as pinned, at their current positions. The test then asserts that the joined branch comes after them.
- Nothing the test protects is weakened: "appended after the existing methods" still holds, and is checked.

**(2) New definitions stay in the carrier files.** The joined `contract_evidence` has exact cases of the physics-source-1 shape, and records whose `solve` and `source_recovery` may be `selected`. It is therefore not an instance of `load_reference_state.schema.json#/$defs/LoadReferenceContractEvidence`.

- New `LoadReferenceSource*` definitions go in the carrier files. They reuse the load-reference and physics-source sub-definitions by `$ref`.
- Neither `load_reference_state.schema.json` nor `physics_source_recovery.schema.json` changes.

**(3) One non-append pointer in the AnalysisRun schema.** The joined AnalysisRun 0.3 mirrors physics-source-1, so `source_block_recovery` and `contract_evidence` are required.

- This needs exactly one non-append pointer change: `analysis_run.v0.3` `properties.contract_evidence` goes from `$ref` to an `anyOf` of the existing definition and the joined definition.
- Every old document must keep its outcome, and the TASK must show that it does.

## 2. T1_WP1_JOINED_READERS

The joined AnalysisRun records follow ruling 1(3). Per-case record shape:

- a selected record has `solve.recovery_method` = `retained_source_blocks_exact_v1` and `source_recovery` = `{status: selected, method: retained_source_blocks_exact_v1}`;
- the other records are ordinary and `not_joined`.

## 3. T1_WP6_STATIC_CASES

WP6 follows WP5's manifest and selector conventions (WP5 heads-up, relayed by the manager):

- manifest paths are relative to WORKING_ROOT;
- the runner input's `preview_model` equals the product request;
- `required_scalar_rows` counts only the positive assertions;
- evidence selectors address `contract_evidence.load_reference_states` by record kind and key;
- negative assertions carry their wrong values in the reference file's `wrong_values`;
- run-level binding lives in a separate run-selection record;
- candidates stay pending until the independent freeze.

## 4. ROOT boundary tightening for the T0R overlap (2026-09-26)

WP1_JOINED_READERS keeps out of the source-blocks readers entirely (`result_export/src/source_blocks.rs`, `analysis_runs/source_blocks.py`).

In `semantic_contract.rs` it only adds the `load-reference-source-1` identity to the enumerations and dispatch. It changes no Current, rule or export standing function in either language. Any change it already needed there is declared to ROOT before integration.

`_T1_COMMON.md` now records this reservation for future briefs, together with T0R's static rule that precision-1 is never a fresh identity. T1's identities join that rule when T1 rebases after T0R.

## 5. T1_WP3_OPERATIONS follow-up

The TASK asked four design questions. Rulings:

- **Q1: inbound-reference refusals are added.** An operation may not orphan an `analysis_state` or member reference. This covers replacing or removing configurations or laws, and deleting a pipe, support, material or primitive load that such a reference names. Pre-0.4 and 0.3.0 outcomes stay byte-identical.
- **Q2:** `not_present` is accepted as the inverse after-value.
- **Q3:** the stricter refusal of duplicated keys is accepted.
- **Q4:** the other rules stay with the product's solve-time diagnostics.
