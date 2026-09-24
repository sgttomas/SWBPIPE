# Async unit readiness regression repair

Only `apps/desktop/src/App.test.tsx` changed in the first-correctness checkout, based on `b9dda2addcccb2adb45f618140425aec533314c0`. Seven assertion sites now wait for their exact operation ID or enabled Queue state using ordinary `waitFor`. No timeout was increased; no assertion, model/apply/target value, operation identity or support-deletion guard was removed. Product code and the separate numerical branch remain unchanged.

The focused baseline reproduced all nine sweep failures with two passing neighboring controls. The earliest divergence is the expected asynchronous unit-validation state: old tests read the preview or clicked Queue before the validated intent existed. The support-deletion case consequently had zero queued operations; its downstream apply lookup was not a deletion-guard failure. Waiting for validated readiness repairs the tests without bypassing the product's pending gate.

The repaired selection ran 11 App cases plus four unchanged deferred unit-validation controls: all 15 passed. The deferred controls cover pending/unavailable responses, stale replies and A→B→A invalidation. TypeScript passed. Raw command/output/source bindings are in `_run_records`; existing jsdom canvas warnings remain recorded and do not establish native rendering proof. The original failed sweep log and summary are preserved and hash-referenced in `_run_records/CONTEXT.json`.

Source identity is in `CHECKED_SOURCE.json`. Fresh independent review is recorded under `review/`. ROOT owns the full clean DEC-025/CI rerun, Git integration, and later transfer of this App-test-only patch to the numerical branch. No Git mutation, package installation or broader test rerun was performed here.
