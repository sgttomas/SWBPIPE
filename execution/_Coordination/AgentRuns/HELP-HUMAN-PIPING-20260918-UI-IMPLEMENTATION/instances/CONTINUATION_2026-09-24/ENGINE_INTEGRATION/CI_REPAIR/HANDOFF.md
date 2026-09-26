# Qualification handoff — 2026-09-25

Owner requested a different-session handoff. Stop implementation and preserve the candidate. PR905 stays draft at e65001ad50072d3399bf204da02055b0f360353a; do not merge. This handoff branch adds only the preserved historical-preimage repair, four leaf lockfile repairs and their evidence. It is a checkpoint, not a qualified merge candidate.

Independent source-coverage precheck is CLEAR for all322 maintained paths at e650: [_run_records/readiness_precheck/RETURN.md](_run_records/readiness_precheck/RETURN.md). It explicitly does NOT cover the subsequent four lockfile changes. That narrow dependency/native-input-impact review is still required; it was interrupted by a worker usage limit. Existing package versions/checksums are preserved, all39 current offline locked fetches pass, and83 tests pass in the three affected test-bearing crates. The fourth crate builds with zero registered tests. Full tests and hosted numerical execution remain pending.

The historical conflict-capture repair has its own completed independent backcheck. Do not repeat that review or modify the conflict-marker criterion. Validate the actual committed complete PR diff with the existing conflict validator.

The original source workflow finished with accessibility and shard1 passing, numerical failing at locked fetch, shard4 failing five browser cases, and shards2/3 cancelled. Five browser failures require diagnosis/repair before final qualification:

- ui-foundation.spec.ts:677 dark comfortable and dark compact1280x800: center covered by result-filter-summary (two cases).
- gui-workflow-validation.spec.ts:107 compact: expected whole label “Solve job state: failed”, locator received “failed”.
- r2-smoke.spec.ts:338 compact: expected unit-policy text N*m/rad,N/m; observed results=none, rows=0 after backend refusal.
- r2-smoke.spec.ts:1680 compact: expected RULE_INPUTS_INCOMPLETE in issues-home; actual diagnostics differed.

These are observations, not pre-decided test-only repairs. Preserve intended behavior and investigate fixture/app semantics; do not weaken protected tests or use cancelled shards as passes. The complete failed log is [_run_records/source-e650-failed.log](_run_records/source-e650-failed.log). Source run36139859999 attempt1, governance run36139859994; PR state capture is [_run_records/pr-handoff.json](_run_records/pr-handoff.json). No successful CI surface4 binding or clean DEC-025 sweep exists for this candidate.

Next: review the four lockfile changes and actual native build dependency impact; diagnose/repair the five browser failures with relevant independent review; update PR905 to the resulting frozen revision; obtain actual-head required CI and successful full source coverage; bind surface4 to that actual run; then run the clean registered DEC-025 sweep and final independent readiness review before merge. The preserved runner is an environment-specific execution helper; update and verify its candidate/path/binding arguments. General UI continuation remains deferred; these affected workflows are part of solver integration qualification.
