# Rotational-gap unit containment: independent review

**Verdict: no actionable finding in the frozen two-file delta. Cleared for the queued bounded postfix checks.** No postfix Rust execution was performed or observed in this review; this is not full-module or Current/product qualification.

Reviewed sources:

- core/product_physics/src/validation.rs: d146409ae7dcc8ae0f1e6bb05446b9dd91efd318465d12ce9883109b47214616.
- core/product_physics/src/lib.rs: d50f289e61807599a2b6f271e949a8304e8b97c58200ca1fd05a5e0b03fb514b.

The baseline-test patch, production repair patch and test-expectation refinement each exactly match their preserved before/after bytes. The sole production change is the pre-normalization guard; the product lib delta is confined to the new test. [Static checks and baseline summary](_run_records/STATIC_CHECKS.json), [source snapshots](_run_records/reviewed_sources), [scope check](_run_records/scope_check.json) and [input hashes](_run_records/INPUT_HASHES.json) retain the reviewed boundary.

Same independent TASK Type2 /root/solver_manager/kernel_review, parent /root/solver_manager, delegated-harness-native, unchanged gpt-6-astra xhigh dispatch. Full Root/TASK/Piping and software-code-review bodies were read; prior LOOP_INIT basis remains recorded. [Origins](_run_records/ORIGINS.json) records actual roots, hashes and limits. No descendants, source edit, Cargo/heavy test, Node, native/browser or Git mutation. Writes are confined to REVIEW evidence.

## Findings-free source conclusions

1. **Correct boundary and ordering.** validation.rs lines 77–92 inspect the immutable submitted model, parse the nonlinear-support DOF, and emit NONLINEAR_ROTATIONAL_GAP_UNSUPPORTED only for behavior "gap" and a parsed nontranslational DOF. The entrypoint calls validation at lib.rs:1120 and returns a blocked envelope at 1140–1142, before shared-section resolution and normalize_model_units at 1149. The current gap conversion at 4864–4874 otherwise always uses Dimension::Length. The guard therefore prevents a converted metre value from becoming a rotational stop.

2. **No alias escape or expansion.** The guard and construction use the same parse_dof. All accepted rotation spellings—RX/Rx/rx, RY/Ry/ry, RZ/Rz/rz—map to nontranslational enum values and are caught. All nine translational spellings remain outside the guard. Unrecognized support tokens still follow existing rejection paths. The separate parse_direction accepts rotation_x/y/z as moment-load aliases; parse_dof does not accept them as support aliases. Neither parser changed.

3. **Other behavior is preserved.** Only the exact supported "gap" behavior token is targeted, matching the builder's behavior dispatch. OneWay aliases, LiftOff and friction do not enter the guard. Unsupported behavior spellings do not acquire capability through it. Valid M35 moment-load processing is separate and unchanged. No stiffness, force, gap value, angle conversion or contact law was added or altered.

4. **Diagnostic identity.** The diagnostic uses a fixed prefix/suffix around the exact support ID, without lossy normalization. Distinct valid support IDs therefore produce distinct IDs. Duplicate support IDs remain independently rejected by validate_ids. The blocker retains the support and authored DOF references. No new diagnostic-ID collision was identified.

5. **Records remain editable.** This is solve-readiness validation over an immutable model reference. The early return occurs before unit mutation; no DTO, editing operation, schema, persistence or import rule changed. The record is neither coerced into an angular input nor silently repaired. The new test retains the accepted_model_state_mutated=false assertion.

## Baseline and test correction

The actual retained baseline log contains 36 observations and a failed test exit, not a passing containment result:

| Baseline group | Count | Observed behavior |
|---|---:|---|
| RX/RY/RZ with m or mm, both modes | 12 | MECHANICS_SOLVED with rotation 5e-5 rad: unsafe length-as-angle admission |
| RX/RY/RZ with rad, both modes | 6 | Already rejected with UNIT_CONVERSION_UNAVAILABLE |
| rotation_x/y/z as support tokens, all three units/both modes | 18 | Already rejected: DOF-invalid for m/mm, unit-conversion-unavailable for rad |

The new initial test over-required the new diagnostic even for already unsupported tokens. The refinement preserves their actual existing blocking codes while continuing to require NONLINEAR_ROTATIONAL_GAP_UNSUPPORTED for the parsed RX/RY/RZ cases. Every case still requires blocking severity, non-solved mechanics, and no accepted-model mutation. No existing protected criterion, supported token or numerical tolerance was relaxed.

After the repair, the source routes all eighteen parsed uppercase rotational cases—including rad—to the new early blocker. The remaining eighteen unsupported-token cases retain their old rejection routes. Lower/mixed-case rotational aliases are covered by the shared parser/enum guard; they are not additional executed cases in the 36-case baseline.

The retained baseline log and before/after manifests were read, including its failure. Its prior solved observations are preserved; no repaired pass is inferred from the source change.

## Bounded next checks and limits

Run on the reviewed hashes:

- unit_contract_rotational_gap_cannot_consume_a_length_as_an_angle.
- gap_preview_closes_to_explicit_clearance_through_dense_loop, retaining its existing expectations.
- input_contract_authored_moment_axes_reach_live_solve, retaining its existing moment aliases and criteria.

These queued checks supply the missing postfix observations. The source-only isolation supports no new interception of OneWay/friction paths; it does not claim broad runtime qualification of those families, authoring/persistence UI, the rest of product mechanics, or Current status. Angular-gap implementation remains unsupported pending its own qualified unit/recovery contract.

