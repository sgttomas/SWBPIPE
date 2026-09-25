# Rotational-gap containment runtime backcheck

**Bounded backcheck passed.** The actual postfix evidence satisfies the reviewed containment checks with no source or lock change. No unresolved finding remains for this bounded fix.

The run used the exact reviewed validation hash d146409ae7dcc8ae0f1e6bb05446b9dd91efd318465d12ce9883109b47214616 and product-lib hash d50f289e61807599a2b6f271e949a8304e8b97c58200ca1fd05a5e0b03fb514b. BEFORE, AFTER and STAGES agree; all three stages exited zero. I independently checked all 37 recorded source/manifest/lock hashes and the recursive closure of 13 local manifests plus their Rust source files. They match the retained run basis and current bytes.

| Actual check | Result |
|---|---|
| Rotational-gap regression | 1 test passed, all 36 case observations present |
| Existing translational-gap positive | 1 test passed |
| Existing authored moment-axis/alias positive | 1 test passed |

Each run reports 196 filtered tests; these are focused results, not a broader suite pass.

The keyed baseline/postfix comparison confirms:

- The 12 RX/RY/RZ length-input cases that previously solved at 5e-5 rad now emit NONLINEAR_ROTATIONAL_GAP_UNSUPPORTED, MODEL_INCOMPLETE and no rotation result.
- The six parsed RX/RY/RZ rad cases now use the same explicit rotational-gap blocker before conversion.
- All eighteen rotation_x/y/z support-token cases preserve their previous rejection output exactly: twelve DOF-invalid length cases and six unit-conversion-unavailable rad cases.
- All 36 postfix cases are non-solved with no rotation output. The passed frozen test also executes blocking-severity and accepted_model_state_mutated=false assertions for each case.

The original failed baseline, initial test and diagnostic-expectation refinement remain preserved. No relaxed solve, numerical tolerance or angular capability was introduced. The supported translational gap and valid moment-load aliases still pass their existing checks.

[Raw backcheck](_run_records/RUNTIME_BACKCHECK.json) contains input/log hashes, all 36 keyed comparisons, source checks and provenance; the [read-only script](_run_records/runtime_backcheck.py) reproduces the backcheck without executing Rust. The prior [source review](RETURN.md) remains the implementation review basis.

Executor: the same independent TASK /root/solver_manager/kernel_review under /root/solver_manager, delegated-harness-native, unchanged gpt-6-astra xhigh dispatch and instruction basis. No descendants, source edits, Cargo/build/native/browser/Node execution or Git mutation occurred in this backcheck. Only this report and raw provenance were written.

This verifies the containment fix and the named regressions. It does not establish Current status, general rotational-gap/angular capability, native editing/persistence behavior, or whole-product qualification.

