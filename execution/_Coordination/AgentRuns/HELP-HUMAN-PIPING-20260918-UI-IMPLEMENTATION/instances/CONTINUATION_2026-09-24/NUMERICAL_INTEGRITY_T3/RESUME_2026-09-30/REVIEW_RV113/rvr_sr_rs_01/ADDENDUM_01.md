# RV113 (RV-R), addendum 01: confirmation of SR-RS's repair round 1

TASK (Type 2), RV113, for ROOT (HELP_HUMAN, Agent 0), who is the return path. I made no delegation. 2026-10-07 UTC.

**What I was asked to confirm** (the coordinator's message; RR "RV113 passes SR-RS with S-1; …", rulings 1, 2 and 4):
- my mutants M20 (S-1), M02 (N-1) and M11 (N-3) now die at assertions;
- no RS `src` changed;
- the suite delta is only the added test.

**The repair.**
- **The commit:** `codex/piping-t3-b1-r-20261007` at `b5cb7faaeb55e4553b64c75b49c6f9dbef77ac64`, one commit over `cc81e78801`.
- **I90's record:** `R/I90/b1_sr_rs_01/REPAIR_01.md`, sha256 `db26e375c0f6d1c0d148c3e7c08d41448fcbf911afa5a5ddeaf353f84ac444c5` (verified); SHA256SUMS.repair_01 23 of 23 OK.
- **My copies:** two `git archive` copies of `b5cb7faaeb` (P without `execution/`) in `WT/rv113/rsr-{head,mut}`, with targets `WT/targets/rv113-{rsr,rsrmut}`.
- **The mutants:** I applied my own mutant schema (`evidence/harness/make_mutants.py`, unchanged since my review) to the second copy.

**Placeholders:** WT, NUM, P, RE, RS, T and R as in my review.

## CONFIRMED

| Check | Result | Evidence |
|---|---|---|
| No RS `src` changed | **Yes.** `git diff cc81e78801 b5cb7faaeb` touches one file, `RE/tests/retained_precision_contract.rs` (+78/−3). In the archive, every RE `src` file is byte-identical to `cc81e78801`; that includes RS (`c6965da0…780d0f5`) and `source_blocks.rs` (`e388416b…0a76a13e`) | `addendum_01/STATIC.txt` |
| **M20** dies at an assertion | **Killed** by `b1_g5_not_required_admits_a_w2_published_case`. The message: "product_attempt_ref naming the case's own attempt: got G5 `PRODUCT_ATTEMPT_MISMATCH`" | `addendum_01/rsr_mut_M20.log` |
| **M02** dies at an assertion | **Killed** by the new `b1_d38_4a_native_failure_with_a_run_is_admitted`. Both (4a) rows fail: "got G5 `PRODUCT_ATTEMPT_MISMATCH` want admitted false" | `addendum_01/rsr_mut_M02.log` |
| **M11** dies at an assertion | **Killed** by `b1_g8_parity_rows_p2_to_p4_for_every_case`. The message: "one parity row on a not_required case whose W2 failed: got G8 `PREPARATION_MISMATCH` want admitted true" | `addendum_01/rsr_mut_M11.log` |
| The control (`RV113_MUT` unset) | All 26 lib and all 70 contract tests pass | `addendum_01/rsr_mut_NONE.log` |
| The suite delta | **Only the added test.** RE's whole suite at `b5cb7faaeb` gives 187 ok and 0 failed. Compared test by test with my run at `cc81e78801`, the one difference is `b1_d38_4a_native_failure_with_a_run_is_admitted` (new, ok); my two harness tests were absent from this pristine copy. S-1's and N-3's checks are rows inside existing tests | `addendum_01/SUITE_RE_REPAIR.json`, `rsr_re_head.log` |
| The census and the c = 1 pins | **Unchanged by construction.** Every RE `src` file is byte-identical, and the corpus is unchanged. My census (0 changes) and my pin comparison (14 documents) depend only on those bytes, so I did not rerun them. I90's reports agree | — |

Each mutant ran as one cargo job through `WT/tools/t3_cargo.sh` (`test --locked --offline --no-fail-fast --test retained_precision_contract --lib`), as did the suite (`test --locked --offline --no-fail-fast`). Their lock holds are in `addendum_01/cargo_jobs_rsr.log`. The kills match I90's REPAIR_01 §2.1, which ran the same three definitions.

S-1, N-1 and N-3 are closed in RS. N-2 is kept (RR ruling 3), N-4 goes to PR-B1's package text, and N-5 needs no action.

## Host

- Every cargo job went through the lock. No test binary was run directly.
- Each background job had one wait, and it ended with the job. I killed no job.
- I deleted the copies and targets afterwards. No Git writes. Absolute paths for every write. Placeholder paths only.
