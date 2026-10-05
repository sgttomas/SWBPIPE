| mode | form | Python | Rust | TypeScript | expected |
|---|---|---|---|---|---|
| sparse_interactive | milestone with its invocation | numerically_eligible | numerically_eligible | numerically_eligible | slice A oracle: match |
| sparse_interactive | milestone without it | needs_recompute | needs_recompute | needs_recompute (RETAINED_PRECISION_NOT_NUMERICALLY_ELIGIBLE) | slice A oracle: match |
| sparse_interactive | D-U7-4 (a) invocation_without_native_capture | numerically_eligible | numerically_eligible | needs_recompute (RETAINED_PRECISION_NATIVE_CAPTURE_REQUIRED) | case file D-U7-4 (declared): match |
| sparse_interactive | D-U7-4 (b) stale_current_model_same_case_ids | numerically_eligible | numerically_eligible | needs_recompute (RETAINED_PRECISION_NATIVE_CAPTURE_REQUIRED) | case file D-U7-4 (declared): match |
| sparse_interactive | TS job IPC, with its invocation | — | — | numerically_eligible | slice A oracle: match |
| dense_scrutiny | milestone with its invocation | numerically_eligible | numerically_eligible | numerically_eligible | slice A oracle: match |
| dense_scrutiny | milestone without it | needs_recompute | needs_recompute | needs_recompute (RETAINED_PRECISION_NOT_NUMERICALLY_ELIGIBLE) | slice A oracle: match |
| dense_scrutiny | D-U7-4 (a) invocation_without_native_capture | numerically_eligible | numerically_eligible | needs_recompute (RETAINED_PRECISION_NATIVE_CAPTURE_REQUIRED) | case file D-U7-4 (declared): match |
| dense_scrutiny | D-U7-4 (b) stale_current_model_same_case_ids | numerically_eligible | numerically_eligible | needs_recompute (RETAINED_PRECISION_NATIVE_CAPTURE_REQUIRED) | case file D-U7-4 (declared): match |
| dense_scrutiny | TS job IPC, with its invocation | — | — | numerically_eligible | slice A oracle: match |

| mode | Python withheld (with / without invocation) | Rust | TypeScript | TypeScript D-U7-4 (a) / (b) |
|---|---|---|---|---|
| sparse_interactive | [69] / [97] | [69] / [97] | [69] / [97] | [97] / [97] |
| dense_scrutiny | [69] / [97] | [69] / [97] | [69] / [97] | [97] / [97] |

Reader-level `numerical_eligible`: true with the invocation and false without it, in all three languages and both modes: True.

Mismatches: none
