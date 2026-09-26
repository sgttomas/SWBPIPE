# TASK brief — CP4_REVIEW (independent review of checkpoint 4)

- **Role:** TASK (Type 2), acting as a fresh-context **non-author** reviewer. You wrote none of the reviewed bytes. You do not delegate.
- **Manager:** the T1 WORKING_ITEMS manager (load/reference states) that requested this TASK. Report to it by `SendMessage`.
- **Candidate:** branch `codex/piping-load-states-20260925` in `/home/user/wt/loadstate`, at the records commit that the manager names in its spawn request (the candidate head). Review the frozen diff `a68326039..<head>`. `a68326039` is ROOT's merge of main into the CP3 branch.
- **Paths:** relative to WORKING_ROOT = `projects/chirality-piping/`. `LSI` is `execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/LOAD_STATE_IMPLEMENTATION`.
- **No writes** outside your return folder `LSI/REVIEW_CHECKPOINT_4/`. Make no Git writes. Never edit the reviewed bytes; run probes and mutants on a scratch copy only.

## What was decided (the basis you review against)

- `LSI/REVIEW_CHECKPOINT_3/RETURN.md`: the CP3 review, with SF-1 and N-1 to N-5.
- `LSI/REVIEW_CHECKPOINT_3/ROOT_DISPOSITION.md`: ROOT's selections. These are decisions; review whether they are implemented, not whether they were wise. Report a concrete defect in a selection separately.
- ROOT's later confirmation that the implemented SF-1 approach is acceptable: reserve the replay budget before selection, and republish every case ordinary under `load-reference-1` when a join cannot finalize. ROOT asked that it be in this review's scope.
- ROOT's disposition of the readers' design question, recorded in `LSI/CHECKPOINT_4.md` §2 (N-2): the readers are not tightened to the canonical 2^53 profile.
- Constraints:
  - pre-0.4 bytes and meanings unchanged;
  - `load-reference-source-1` and `resolved_straight_load_state_source_v1` stay reserved and inactive;
  - the physics-source-1 composite `SOURCE_BLOCKS_FINALIZATION_FAILED` behaviour is out of scope (T3) and must be unchanged;
  - no library or code-rule data;
  - no weakened protected criterion.

## What to review

1. **SF-1** (`core/product_physics/src/lib.rs`, `src/source_recovery.rs`, `src/source_receipt/load_state_fallback_tests.rs`; `LSI/CP4_WIRE_ADDENDUM.md` §1).
   - Can a 0.4.0 invocation still lose ordinary results when a selected join cannot finalize?
   - Can the fallback publish anything other than the ordinary route: a stale selected projection, a receipt, a wrong label or a wrong record?
   - Is the replay reservation sound? Is replay really bounded by the live charge?
   - Is the ledger ever refunded or double-debited in a way that matters?
   - Is pre-0.4 behaviour byte-identical?
   - Are the diagnostics and records truthful?
   - Can the fallback recurse or loop?
   - Are the tests adequate, including the reviewer's two scenarios?
2. **N-1** (`src/source_receipt/load_state_join_tests.rs`; `LSI/CP4_JOIN_TESTS/RETURN.md`). Do the tests assert what they claim, with independent expectations? Are K5 and K7–K10 really killed? Reproduce the mutant runs for at least K8 and K9 on a scratch copy. Does the K9 route mirror stay faithful?
3. **N-2 and N-3** (`core/reporting/result_export/src/load_reference.rs`, `core/analysis_runs/load_reference_evidence.py`, the shared case file and both harnesses; `LSI/CP4_READERS/RETURN.md`).
   - Are the two readers identical in rule, order and code?
   - Does any case diverge between languages?
   - Is the binary64 equality handling right?
   - Is any tightening beyond the disposition present?
   - Do the frozen fixtures still pass?
4. **N-5**: formatting only.
5. **N-4 and the records** (`LSI/CHECKPOINT_4.md`, `LSI/CP4_WIRE_ADDENDUM.md`, the session3 `_run_records`). Are the hashes and claims accurate? Were earlier records left unrewritten? Are machine paths kept out of committed records?
6. **`LSI/T1_PLAN.md`**: read-only sanity. It is a plan, not a mergeable slice. Note anything that misstates the current state or a constraint.

## Checks you may run

Use `CARGO_TARGET_DIR=/home/user/wt/ls-review-target` (delete when done), `-j 2`, `+1.97.1 --locked --offline`, and the venv `/home/user/dec025-venv`. About 15 GB of disk is shared, so keep one scratch copy at a time.

- the product_physics crate (expect 415 passed, 1 ignored);
- result_export (64);
- the named pytest files `tests/test_load_reference_readers.py`, `tests/test_load_reference_schema.py` and the stress-neutral suites (expect 957 passed, 1 skipped);
- `_run_records/session3/cp4_regen_compare.py`;
- your own probes and mutants on a scratch copy.

## Return

Write `LSI/REVIEW_CHECKPOINT_4/RETURN.md`, with commands and logs under `LSI/REVIEW_CHECKPOINT_4/_run_records/`, and no absolute machine paths in RETURN.md. It should contain:

- the verdict: CLEAR, FINDINGS or BLOCKING;
- a findings table: severity (blocking, should-fix, note), location, concrete failure scenario, suggested repair;
- what you verified and how;
- limits.

Then send a `SendMessage` to the manager with the verdict and the path.
