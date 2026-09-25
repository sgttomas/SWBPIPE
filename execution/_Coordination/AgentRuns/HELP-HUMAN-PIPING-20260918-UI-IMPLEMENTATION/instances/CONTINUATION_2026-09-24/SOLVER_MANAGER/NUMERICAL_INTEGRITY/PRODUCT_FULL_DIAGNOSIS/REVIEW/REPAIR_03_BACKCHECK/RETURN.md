# Repair 03 backcheck — clear for affected rerun

No actionable blocker. The complete frozen change replaces exactly two positional assertions with exact support-ID lookups in `unsupported_gap_inspection_preserves_mixed_computation_without_qualification`. Missing IDs still fail, and the expected gap Active / friction Sliding states are unchanged. Reversing those two replacements recovers the entire previous source byte-for-byte. No production logic, fixture, tolerance, convergence criterion or other assertion changed; product remains unchanged.

The retained focused log at `SOLVER_FIRST_CHECKS/_run_records/five_nonlinear_inspection/cargo.log` reports two passes and one failure at the old friction assertion: Inactive versus Sliding. The classifier's `states.sort_by` at nonlinear_supports lib.rs:491 and the integration assignment from `active_set.states` at nonlinear_integration lib.rs:673 establish final ordering friction/gap/one-way. The old index 2 selected one-way. Exact ID lookup correctly repairs that test association without weakening its physical expectation.

The live nonlinear candidate matches SHA256 `ba45af590ab1bdb0ad4d58037c4ce2e52270ac08730386fbd081e755f85b2041`, from the previously reviewed `572d0ed16d6811d2cc0b93c26d2610ac480a56f56ac2225572510749ee96d739`. Live product remains `646b648b1931009f357f76b96d10f80d32e98369bd76920f61638136d8925e1a`. The preserved diff reconstructs exactly and explicit-path scope validation passed.

Source clearance extends to this test-only successor for the already granted focused/full affected rerun. This reviewer executed no Cargo/build/native/npm/Git or source tests; the prior positive test stopped before its remaining assertions and before later loop modes, so neither the repaired positive test nor a candidate runtime pass is inferred. Prior REVIEW/RETURN.md's production findings and outstanding verification limits continue to apply.

Same independent reviewer `/root/solver_manager/mixed_inspection_review`, TASK Type 2 under `/root/solver_manager`, native followup with no descendants. Runtime origins and hashes are in `_run_records/ORIGINS_AND_CHECKS.json`; this report uses repository-relative source paths.
