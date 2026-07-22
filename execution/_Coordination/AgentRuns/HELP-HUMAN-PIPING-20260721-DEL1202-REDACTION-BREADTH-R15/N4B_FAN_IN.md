# WORKING_ITEMS Fan-in — N4B immutable remediation attempt 2

**Verdict:** `ACCEPT_FOR_FRESH_N5B_REVIEW`

- N4B return and mandatory TASK record are `SUCCESS`.
- The five N5 findings have focused tests: Vitest 42, Rust 5, runner
  subprocess/contract 8, and full headless crate 44—all pass.
- All eight N4B registered/H4 evidence files are `PASS`; project pytest 520,
  desktop 482 plus build, harness 311 plus self-check, H4 source 2/2 and dist
  1/1.
- Attempt-1 sweep is unchanged at SHA-256
  `10fbc3c4e54b69df2856cb5dd42240dc87b35d4c3762df9664e6036cf7b3cd63`
  and is superseded/non-acceptance evidence.
- Attempt-2's sole sweep is
  `SWEEP_20260722T061039Z_0c066652cd52-dirty.json`, SHA-256
  `d1620f2f25264400be6a68e6931982c13b7c17fe52a14381fe70c673f045ca72`,
  and is the only acceptance-eligible sweep.
- Protected/release paths remain unchanged and `git diff --check` passes.
- No deliverable state, receipt, final deliverable run record, or Git closeout
  appears.

Fresh N5B must independently verify all five repairs and the complete adopted
behavior. This fan-in does not release W3.

