# I61 addendum: RV77 fixes before acceptance

**Status: DONE, uncommitted in WT/f2a-coverage for ROOT (branch head `e0fc33b4f7`).**
- **RV77-S1:** I adopted RV77's tests into the two fenced test files. Its mutants R1, R6 and
  R10 are now killed by the committed suite, as are M1–M6 again. The NONE control passes.
- **RV77-N3:** the comment is corrected, and the actual behaviour is now asserted.
- **RV77-N2:** the accounting correction is below.

Only those two test files changed. No source change was made: final_case.rs and
retained_receipt.rs are byte-identical to `e0fc33b4f7`.

This was the same TASK under ROOT's grant (NUM `059e876617`). It started
2026-10-03T20:42:02Z, and the freeze runs ran from 20:45:50Z to 20:56:04Z. There were no Git writes or index
operations, the memory guard (PID 5387) was running, and only one cargo job ran at a time with
`--locked --offline`, `CARGO_BUILD_JOBS=4` and `RUST_TEST_THREADS=2`.

## Host toolchain (ROOT's interim ruling)

Xcode was updated on this host at about 20:39Z and its licence is not accepted, so `xcrun`
exits 69 and test binaries fail to link. The first two runs of this grant (focused FK and PP,
20:42:58Z) failed to link for that reason; their logs were overwritten by the re-runs. Every
later cargo command in this grant set `DEVELOPER_DIR=/Library/Developer/CommandLineTools`
(Apple clang 21.0.0, already installed) in that command's own environment:
- both focused runs (re-run);
- all mutant and NONE runs;
- both freeze runs.

ROOT later ruled to permit this, as an interim measure for local tests only. I did not accept
the licence or change any system setting. Only linking changes; the compiled crates are
unchanged, and the incremental re-link rebuilt only the test crates.

## Changed files (in WT/f2a-coverage)

| File | Before (`e0fc33b4f7`) sha256 / lines | After sha256 / lines | +/− |
|---|---|---|---|
| PP/retained_product_tests.rs | be2579221d4530bcfa1cd02bd34742a96ca6c9c71d7b9608a9ad59946a4bb491 / 3131 | 7df075d7a033b1431d142b8d122351a215c5566d1fbf0b0e05eb0f65e8d54985 / 3191 | +61 −1 |
| P/core/solver/frame_kernel/tests/retained_k4/product_final_case_tests.rs | 9e0ca9219254de392445954cbf5b5aaed8aa160484c60acf09a94d89a1340cfa / 634 | 7f112382a30e7631445c19ef7b3e0bb9dfa635a08c15eb3e74fa9216814d93b3 / 783 | +149 −0 |

These two files are unchanged:
- final_case.rs: 3986919726e962b52eaa730eb4cc76311fb79ff6dee6e075aa75b4a4d639def7
- retained_receipt.rs: 55dc8cb50526104186c944f9265f54395c3dc206e70fa7c43be2fc2982dee517

### What was added

- **RV77-S1, frame_kernel.** `tests/rv77_enum.rs` is appended verbatim to the end of
  product_final_case_tests.rs as `mod rv77_enum { … }`. Its sha256 is
  7cc8d74bcc652bfe49394e3ac1eabee53240a82b40f4808a4eeb1e7aacf80634, equal to RV77's
  SHA256SUMS. Its `use super::*;` resolves through this test module to final_case. It adds
  three tests:
  - the genuine domain;
  - the full payload domain, which pins the positive-floor refusal at L = 0;
  - the input-derived force/moment necessity.
- **RV77-S1, product_physics.** `tests/rv77_pp.rs` is appended verbatim to retained_product_tests.rs
  as `mod rv77_pp { … }` (sha256 24290692e4fd9c18e04861590fad8ab5c9a7810a07286c71572be902911f89e3,
  equal to RV77's). It pins:
  - null refused on a passed G5a;
  - each of the seven prerequisite stages;
  - certificate entry;
  - the requirement for `capture.source`.
- **RV77-N3.** At retained_product_tests.rs:3126 the comment now states the actual behaviour.
  A proof projected against a different selected owner with identical public facts is *not*
  refused by the seam (the I57 §5 custody limit). The test now asserts `view.is_ok()`, so a
  later structural binding has to flip it deliberately.

## Runs

| Run | Result |
|---|---|
| frame_kernel `--lib -- i61_ rv77_` | 6 passed: RV77_GENUINE cases=589824 p512=393216 estimate≠charge=157696 refusals=0 mismatches=0; RV77_PAYLOAD_SPEC accepted=9288 refused=64440; RV77_INPUT_DERIVED_FM breaks=336 (equal to RV77's ENUMERATION) |
| product_physics `--lib -- i61_ rv77_` | 5 passed (`I61_FOREIGN_OWNER … ok=true` and RV77_PP_STAGE_RULES ok). The compiler warnings are the same five pre-existing dead-code warnings. |
| **Freeze:** frame_kernel `--lib` | 480 passed, 0 failed, 1 ignored (585 s). That is 477 plus the 3 RV77 tests. |
| **Freeze:** product_physics `--lib` | 456 passed, 1 failed, 1 ignored. The only failure is `s11g_tests::t13_committed_fallback_uz_is_byte_identical`, the known Mac failure. 456 is 455 plus the RV77 test. The compiler warnings are the same five pre-existing ones. |

## Mutants

Every mutant run used `--lib -- i61_ rv77_` on its crate. Each patch was applied with `patch -p1`
and reversed afterwards, and the restore was sha256-verified against all four frozen files after
every run.

| Id | Patch | Result | Killing tests |
|---|---|---|---|
| NONE (fk, pp) | — | pass (6, 5) | — |
| R1 floor refusal only when L≠0 | REVIEW_RV77/…/mutants/R1_floor_refusal_only_when_L_nonzero.diff | KILLED | `rv77_enum::rv77_full_payload_domain_matches_independent_spec` |
| R6 null allowed on passed G5a | …/R6_null_allowed_on_passed_g5a.diff | KILLED | `rv77_pp::rv77_stage_rules_null_and_non_null_prerequisites` |
| R10 source presence not required | …/R10_source_presence_not_required.diff | KILLED | `rv77_pp::rv77_stage_rules_null_and_non_null_prerequisites` |
| M1 adapter copy source | I61 `_run_records/mutants/M1…diff` | KILLED | i61 certificate-prefixes, i61 failure-prefixes, rv77 stage-rules |
| M2 empty as all-false | M2…diff | KILLED | i61 failure-prefixes, i61 certificate-prefixes |
| M3 null allowed on Ready | M3…diff | KILLED | i61 certificate-prefixes |
| M4 drop one body | M4…diff | KILLED | all four i61 PP tests |
| M5 p512 charge := estimate | M5…diff | KILLED | i61 synthetic; rv77 full-payload; rv77 genuine-domain (now killed by the enumeration as well) |
| M6 accept flag mismatch | M6…diff | KILLED | i61 rederivation-refuses |

## Corrections to the RETURN's accounting (RV77-N2) and extent wording (RV77-N1)

The RETURN's "Accounting" section is corrected as follows. The sizes and `size_of` charges it
states are unchanged and verified by RV77.

1. **Unmetered work.** The RETURN called the per-body node scans and extent arithmetic "reads,
   not copies". That is inaccurate. They are unmetered compute: no owner meters them, and
   `TraceCopyWork` counts copies only. Per body, `coverage_facts` performs:
   - one pass over the whole bound layout;
   - two passes over all source nodes (one to count, one to copy; only the copied
     coordinates are charged, as `[f64;3]` events);
   - two linear scans, for the `resolution_scale` and floor lookups;
   - the 9 binary64 operations of `adaptive::body_extent` (3 sub, 3 mul, 2 add, 1 sqrt),
     plus the min/max comparisons;
   - the Boolean rederivation.
   
   None of this is metered. Over all bodies the scans are O(bodies × (layout rows + nodes)).
   Whether projection compute is metered is left to the receipt-transaction design.
2. **Double count.** The 16 B `SummaryCoverage` charged in local `TraceCosts`
   (retained_receipt.rs:108) is already inside the `PreparedAttemptView` charged at the start of
   `project`. It is a conservative double count, not an omission.
3. **Extent wording (N1).** The extent bit-equality compares two values from the same `CasePrep`
   (`prep.source` and `prep.extents`). It guards against a regression in `CasePrep::with`; it
   does not bind the proof to the owner.

No allowance, LME, byte permit or M is claimed.

## Logs (WT/scratch/i61_coverage_producer_01/rv77/)

| sha256 | bytes | path |
|---|---|---|
| cf3d4e05c1a3fac76ed75aab527247f470975e9f3c1b9e361ecd53886aa776d5 | 3880 | WT/scratch/i61_coverage_producer_01/rv77/30_fk_focus.log |
| 5810fafde9296cbdfd34e086db4cf95a9f97e7df4d44ba0a99473b52bb66c611 | 1334328 | WT/scratch/i61_coverage_producer_01/rv77/31_pp_focus.log |
| e92eddd0388768de8d0711d9feb2551f88c1cb97cc03d68e7d2f81c433f76511 | 24 | WT/scratch/i61_coverage_producer_01/rv77/40_freeze_fk_lib.exit |
| 712dd6ea33c03eb3775a09d104881de26b80e178e2d205e1677d357401379944 | 52044 | WT/scratch/i61_coverage_producer_01/rv77/40_freeze_fk_lib.log |
| 5ac55823ba0aa44c2057a0ddd52fac62f3c545f8825dec2e2eef54d683547722 | 21 | WT/scratch/i61_coverage_producer_01/rv77/40_freeze_start.txt |
| d4ece73ab02574bc1b1f0793e265bf474ffd7d29b085622218356bba1ba9d1ae | 21 | WT/scratch/i61_coverage_producer_01/rv77/41_freeze_end.txt |
| 57c09c8c19e4d87534b359bd3408c4a3cee1b46400b492c51a557d9db37f174d | 25 | WT/scratch/i61_coverage_producer_01/rv77/41_freeze_pp_lib.exit |
| 80917540b87b46a8c9605b89b4a8bc42a2501e3e0a643411bb2e526d5a9962b0 | 47487 | WT/scratch/i61_coverage_producer_01/rv77/41_freeze_pp_lib.log |
| 3f5fd37c4a951939ae8655cfe908175b7ff5e764083ac6277b3dd8e26713893e | 618 | WT/scratch/i61_coverage_producer_01/rv77/frozen_source.sha256 |
| a56aa9a277c464f7feae313ec640d5a0ec92aaaf03a786fe685bdb0ebc3f7665 | 1079950 | WT/scratch/i61_coverage_producer_01/rv77/mut_M1_adapter_copy_source.log |
| 2c9c7dfde9cd7a5c584fa1c2b6128a42e31713fd86c1eabd15180d3e86131bda | 5387 | WT/scratch/i61_coverage_producer_01/rv77/mut_M2_empty_as_all_false.log |
| 4553f388bbc49d0a550bf32963fb677650c0890e1268bb84bdd6b73fec25e42f | 4868 | WT/scratch/i61_coverage_producer_01/rv77/mut_M3_null_allowed_on_ready.log |
| 4faad946b8caf0fda0295d4757d6fdd033556923000f75e526042d1c917c4f77 | 342669 | WT/scratch/i61_coverage_producer_01/rv77/mut_M4_drop_one_body.log |
| 10fc319e491a86de6624ec2a7646e093ec35a8d0961cffa9b655a79504c02920 | 4328 | WT/scratch/i61_coverage_producer_01/rv77/mut_M5_p512_charge_is_estimate.log |
| 75bf65838accb7e8775c2d8fe18dec1ffec4b8a15db2d8bddf535b0ddb54c773 | 2374 | WT/scratch/i61_coverage_producer_01/rv77/mut_M6_accept_flag_mismatch.log |
| 405678da77337b94a0e900658a54c94d31ab0d5fe5dc2bb7c92b2fbc7b11ac2b | 1366 | WT/scratch/i61_coverage_producer_01/rv77/mut_NONE_fk.log |
| aaf1abeb9f0bd386d812198c6655d3995bcd11f46c03502ea9bef95510ee9700 | 3980 | WT/scratch/i61_coverage_producer_01/rv77/mut_NONE_pp.log |
| 5d6228e87c82d96ab80c26574f5be979879e177eff75b14fb35fb67c28db493d | 2423 | WT/scratch/i61_coverage_producer_01/rv77/mut_R1.log |
| c1b6b9e8543369cce6e2494d0f8ef2a890b7f3ddc60260af8c3a7697e8b3f166 | 4698 | WT/scratch/i61_coverage_producer_01/rv77/mut_R10.log |
| 432d8f3e9ea20a1a433875351d2fbbd048cc0a72a65f209290a5434b3dac8655 | 6958 | WT/scratch/i61_coverage_producer_01/rv77/mut_R6.log |
| 44c70f9b5ed0b7d626999690891319fa8d3477322576b8382ce6e3e3f65faedc | 1187 | WT/scratch/i61_coverage_producer_01/rv77/run_patch_mutant.sh |
