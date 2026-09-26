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

## 6. Further WP1 and WP5 rulings

**One declared standing edit (WP1 readers).** ROOT accepted one early return of `needs_recompute` for `load-reference-source-1`, at the top of `semantic_contract::numerical_use_standing_with_context` (Rust) and the single equivalent early return in the Python peer.

- Why: it is conservative and has the same outcome as the fall-through. An admitted joined envelope always has a selected case whose ordinary attempt was sensitive or rejected, so it can never be `checks_passed`.
- Nothing else in any standing function changes.
- The two standing edits are combined by hand at the T0R/T1 merge.

**Shared-case expectation (WP1 readers).** In `load_reference_mutations.json`, case `TABLE-id-reserved-source-successor` (an LR envelope relabelled with the joined ID, keeping the LR profile) previously expected `SOURCE_PRODUCER_CONTRACT_UNSUPPORTED`, which encoded "reserved, inactive". D1 activates the ID, so the case is now refused at the formulation basis (`SOURCE_FORMULATION_BASIS_UNSUPPORTED`), identically in both languages. It is still refused. Only that expectation changes.

**Schema test pin (WP1 schemas, ruling 4).** One minimal edit in `tests/test_source_block_schema_contract.py`: the joined branch (index 5) uses `PhysicsSourceResultSet`, like physics-source-1 (index 3). The pin on branches 0–4 stays exact.

**WP5 negatives.** Negative assertions are keyed on the assertion ID, and each (selector quantity, wrong value) pair must be unique. This lets each reference discriminator have its own negative. Positive selectors stay unique.

## 7. ROOT decision: joined results are not numerically eligible in T1 (2026-09-26)

In T1, joined (`load-reference-source-1`) results stay `needs_recompute`. The readers accept and display them, but never grant numerical eligibility.

Invocation-based eligibility is **open work mapped to T3**, not T6. It needs a reader-side re-derivation of the resolved case, in both languages and captured like physics-source-1: thermal laws, fit and material selection. That is retained-source recovery qualification, which belongs to T3 (general retained-source recovery, M03/M34).

This does not block closing M10, M16 and M29. Their required capability is correct imposed motion, per-element thermal and reference states, and cold spring. It is delivered and qualified on the ordinary `load-reference-1` route, through the VP-STATIC comparisons and the native witnesses. Today every route withholds sensitive cases from Current, and qualifying them is M03 numerical work. The closure reassessment must state this boundary.

ROOT made this decision within its delegated correctness authority.

## 8. WP1 readers: two placement details

- **Standing early return.** It sits immediately after the existing `for_source` / `_source_contract` validation, not before it. A refused joined envelope therefore stays `unsupported`, as the shared cases pin.
- **Ruling B, dispatch only.** Only the dispatch expectation changed. The direct load-reference-1 validator still accepts the relabelled case, because it leaves the header to dispatch, so its expectation stays "accept".

## 9. Pre-review shakedown of the VP-STATIC run (not evidence)

**What was run.** Before REVIEW_A returned, the manager ran `_run_records/session4/t1_vp_static_run.py` in scratch at `203396e4d` to shake out the run. The runner was built from that commit (sha256 `df6dca9c…`). The reader binding cited only the CP3 and CP4 reviews.

**Why it is not evidence.** The reader module changed in WP1 after those reviews, so this run is not evidence and is not retained. The scratch output is deleted.

**What it showed.** In both modes:

- 12 of the 14 admitted cases solved with `checks_passed` standing;
- all 441 of their required assertions matched;
- the other two cases (`coefficient_definition`, `multi_segment_free_length`) emit 14.1 MB and 21.9 MB of runner output.

The adapter accepts an output limit of up to 64 MiB, but its capture admission is capped at the gate's 8 MiB. That is an adapter defect, and the repair went to the WP5 author. No fixture, reference or criterion changes. The run script now uses the adapter's maximum 64 MiB output limit.

## 10. REVIEW_B dispositions (with ROOT's direction, 2026-09-26)

REVIEW_B's verdict was FINDINGS, with nothing blocking (`T1_WAVE1_REVIEW_B/RETURN.md`).

| Finding | Disposition |
|---|---|
| **F1** | Must fix. `Model/pressure_profile` may not rewrite a 0.4.0 model. Sent to the WP3 author: a targeted refusal, with a test, and pre-0.4 outcomes unchanged. |
| **F2** | Close as in Q1: replacing `temperature_points` may not orphan an `exact_point` `point_ref`. Sent to the WP3 author. |
| **F3** | The adapter's runner-output admission limit. The WP5 author is repairing it; REVIEW_B backchecks. |
| **F4** | Fixed by the manager in `cad59d01e`. The generator writes the admitted rule review text; README and PROVENANCE are at the admitted state, and `author_package.py` is marked superseded for R1. An admitted-state successor checker (`_run_records/session4/check_package_admitted.py`) replaces the authoring-time pending invariants: 4535 passed, 0 failed. |
| **F5** | Kill the surviving mutants with tests, or record why one is equivalent. Adapter survivors went to the WP5 author, applier survivors to the WP3 author. |
| **F6** | Assert the dense value, and state the equal-length assumption or use the general form. Sent to the WP3 author. |
| **F7** | Document that the operation-level inverse cannot restore an explicit-null prior. The product refuses explicit null, and desktop Undo is checkpoint-based. Sent to the WP3 author. |
| **F8** | Fix the stale pin comment. Sent to the WP5 author. |
| **F9** | Treat 0.4.0 exact-pressure models as the exact profile in `validate_temperature_points`, so no spurious warnings appear. Sent to the WP3 author. |

## 11. WP4 headless: joined canonical document (wave 2, manager ruling)

- **Question (T1_WP4_HEADLESS).** The brief's item 2 said "the canonical document selects the joined branch". Through `result_envelope_binding`, a `load-reference-source-1` solve produces no canonical document and no `QualifiedPreviewEvidence`. The recorded reason is `CURRENT_NUMERICAL_INTEGRITY_NEEDS_RECOMPUTE`: the binding builds only numerically eligible results, and §7 makes joined results never eligible in T1.
- **Ruling.** Characterize this as designed; no seam is added.
  - Assert no document and no proof, with the exact unavailability text.
  - Assert the receipt is bound to the actual invocation and validates through the joined reader.
  - Assert `needs_recompute` in both readers.
  - Assert reparsed receipt bytes cannot produce a document.
  - Emit the joined artifacts without a document.
  - The committed joined carriers remain the ones from `derive_document` in `load_reference_source_contract.rs`.
  - A binding route for joined results is T3 work, together with eligibility.
- **Pre-0.4 controls.** A 0.3.0 document that carries 0.4.0 keys is blocked with `LOAD_STATE_CONTRACT_VERSION_MISMATCH`. Its identity follows its own version and contract: `physics-1` on the exact contract, `preview-physics-1` otherwise. It never carries a T1 identity.
