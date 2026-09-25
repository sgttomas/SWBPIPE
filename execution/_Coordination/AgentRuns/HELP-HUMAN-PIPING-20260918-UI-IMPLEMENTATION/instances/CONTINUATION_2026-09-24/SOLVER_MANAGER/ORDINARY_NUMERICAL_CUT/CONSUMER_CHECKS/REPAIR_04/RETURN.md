# Final targeted consumer verification

The one repaired long App case passed, followed by TypeScript build checking. The targeted selection explicitly skipped the other 218 App cases. It does not claim to rerun those cases. The preceding eight-file run passed 297 cases; only the remaining named test case and its imports changed afterward, with independent source backcheck covering that applicability correction.

Both final stages used unchanged source/manifests/locks and the eight current WASM artifacts. All 110 source files still match FINAL_SOURCE_FREEZE.json and complete patch c47e5626354de9ba5c672cdcb16b25b323606cf1cbd41831e80af02dc629041f. `_run_records` retains the exact commands, inputs, logs, driver and final source check. The JSDOM canvas limitation messages remain in raw output; assertions passed, and this is not a browser/native rendering witness.

The original 240/251 run, later 297/298 run, review findings and repairs remain preserved. Current affected-case coverage is supported by the prior 297 passes plus the final repaired-case pass, with final TypeScript pass. Complete-cut review and registered clean-candidate/native checks remain separate.
