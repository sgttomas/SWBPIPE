# T1 wire addendum: activating `load-reference-source-1`, and the joined record shape

This addendum supplements the wire of record without changing any earlier file:

- `CP2_WIRE.md`;
- `CP2_WIRE_ADDENDUM_1.md`;
- `CP2_WIRE_ADDENDUM_2.md`;
- `CP3_WIRE_ADDENDUM.md`;
- `CP4_WIRE_ADDENDUM.md`, with `CP4_REVIEW_DISPOSITION.md`.

Paths are WORKING_ROOT-relative.

## 1. Activation (D1)

ROOT and the owner decided on 2026-09-26 (`OWNER_T1_DECISIONS_2026-09-26.md`, D1) that `openpipestress.result_semantics/0.3.0/load-reference-source-1` is activated in the T1 PR, together with its profile `resolved_straight_load_state_source_v1` and receipt policy `LOAD-REFERENCE-SOURCE-1`. The condition is that its readers pass independent review (`T1_WAVE1_REVIEW_A`). The owner approved the stress-neutral packager edit.

**Downstream consumers** (commit `bfef71b19`):

- the Rust evidence reader, `core/reporting/result_export/src/load_reference_source.rs`;
- the Python evidence reader, `core/analysis_runs/load_reference_source.py`;
- dispatch, derivative and AnalysisRun registration;
- the three 0.3 carrier schemas;
- the stress-neutral 0.3 packager.

Earlier records say the identity is "reserved and inactive". That wording now holds only up to this activation. The frozen load-reference-1 table still lists the joined ID under `reserved_inactive_successors`, because its bytes stay frozen. Readers do not use that list for dispatch.

**One changed expectation.** A load-reference-1 envelope relabelled with the joined ID and keeping the load-reference-1 profile used to be refused as an unsupported producer contract. It is now refused at the formulation basis. It is still refused, identically in both languages (`T1_WAVE1_RULINGS.md` §6).

## 2. Joined per-case record shape (corrects ADDENDUM_2 §5.3)

ADDENDUM_2 §5.3 says `contract_evidence.load_reference_states[i]` "keeps the ADDENDUM_1 §3 shape". That is not exact for a joined envelope:

- **`solve.recovery_method`** is `retained_source_blocks_exact_v1` for a selected case, and the ordinary method for any other case.
- **`source_recovery`** is `{status: "selected", method: "retained_source_blocks_exact_v1"}` for a selected case, and `{status: "not_joined", code: "LOAD_STATE_SOURCE_RECOVERY_NOT_JOINED"}` otherwise. At least one case is selected.
- **`contract_evidence.exact_cases[i]`** takes the physics-source-1 exact-case shape (`recovery_method` and the retained `pipe_stress_extrema` alternatives), not the physics-1 shape. `recovery_method` agrees with the case's record. The readers check this, not the schemas, because it spans two arrays.

For these reasons the frozen `schemas/load_reference_state.schema.json` still describes only the load-reference-1 shape and is unchanged. The joined shapes are the new `LoadReferenceSource*` `$defs` in `schemas/results.v0.3.schema.yaml`. The AnalysisRun and stress-neutral carriers reach those `$defs` by cross-carrier `$ref`.

## 3. Joined AnalysisRun 0.3

The joined AnalysisRun record mirrors physics-source-1: it carries `source_block_recovery` and `contract_evidence`, with `load_reference_states`. `analysis_run.v0.3` `properties.contract_evidence` is an `anyOf` of the existing physical-evidence definition and the joined definition. That is the only non-append pointer change, and every earlier document keeps its outcome.

## 4. Numerical standing

Joined results are `needs_recompute` in T1. They are never numerically eligible (ROOT decision, `T1_WAVE1_RULINGS.md` §7).

- **Where it is decided.** One declared early return in `numerical_use_standing_with_context` (Rust) and `numerical_use_standing` (Python), after the existing source validation.
- **Why.** Invocation-based eligibility needs a reader-side re-derivation of the resolved case. That is open work for T3.
- **Effect on closure.** It does not block M10, M16 or M29. They are delivered and qualified on the ordinary `load-reference-1` route.
