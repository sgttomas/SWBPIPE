# RET-01 long-ID / tight-budget source witness

This is a source-derived reproduction recipe, **not an executed Rust result**.

Use the existing adapter retention test's adjacent_input and independent expected source setup. Rename g1 to a 4,096-byte nonempty support ID in both nonlinear_supports and initial_states. No stiffness, load, gap, DOF, state or criterion changes. The support constructors and integration validation have no ID-length ceiling; the retained state copy reservation already counts the resulting decision and work-label bytes. Run this fixture only when the parent grants the test lane.

For its 18 DOFs, 2 decisions and 36 projection entries, current replay_retained_against reserves:

32 + 8*18 + 24*2 + 24*36 = **1,088 units**.

That number does not depend on support-ID or work-label length. summaries_match then compares, among other fields:

- self.work against work_snapshot, including the label "gap operands " plus the 4,096-byte ID (at least 4,109 bytes);
- self.decisions against decision_sources, including the 4,096-byte ID.

Thus at least 8,205 dynamic bytes from these two fields alone are compared after a 1,088-unit fixed reservation, without a size-sensitive reservation for them. Other compared fields add more work. This violates the implementation's declared conservative replay comparison reservation; it is not a timing or hardware instruction-count assertion.

Call replay_retained_against with work_limit=1,088. On a valid retained report the existing source accepts that fixed prefix, executes summaries_match, and calls the child replay with zero remaining budget. The child rejects its initial 16-unit reservation. Predicted current result: Budget, charged=1,088, rejected=16. A failed final result therefore does not cure the preceding uncharged comparison work.

Repair expectation: before summaries_match, reserve a checked conservative estimate covering all compared dynamic decision/work/source/summary data, including nested attempt labels/errors. For the long-ID fixture that estimate must grow with the actual retained payload. A 1,088-unit limit should reject that reservation before the comparison: charged=0, rejected greater than1,088. A sufficient total limit should still replay successfully; one unit below the observed successful reservation must reject without resetting the child budget.

Retain the actual short-ID/long-ID results and charges. Do not change exact signs, source ownership, projection limits, contact law or the original fixture's mechanical values.

