# ROOT disposition of the checkpoint-3 review

HELP_HUMAN (ROOT), 2026-09-25. Before this review returned, the owner directed that current work be brought to completion and then paused. So these dispositions are recorded for the next session, not implemented here. [RETURN.md](RETURN.md) is the independent review of `485cc2ed0`. It found no blocking defect.

- **SF-1: selected but not finalizable.** ROOT's technical selection: an invocation whose selected join cannot finalize must not lose its ordinary results.
  - Publish the `load-reference-1` envelope with every case on its ordinary route. Mark each case `source_recovery` `not_joined`/unavailable and keep the attempt diagnostic. Numerical qualification still governs Current eligibility.
  - Reserve the captured-replay work budget before selecting the join, so a case is selected only when replay can complete.
  - Pin both of the reviewer's scenarios (budget exhaustion; composite with a sensitive pressure case) with tests.
  - Correct `CHECKPOINT_3.md:43` and `CP2_WIRE_ADDENDUM_2.md:102` through a new addendum, not by rewriting them.
  - Pre-0.4 bytes and meanings stay unchanged. The inherited physics-source-1 composite `Err("SOURCE_BLOCKS_FINALIZATION_FAILED")` behaviour in PR905 fails closed and is recorded as a connected open finding in the work graph. It is not changed in the PR905 merge candidate.
- **N-1:** add committed tests equivalent to the reviewer's probes P1–P12, so mutants K5 and K7–K10 are killed by maintained tests.
- **N-2:** the Python reader accepts any finite integer literal as a number, matching Rust. Add shared mutation cases.
- **N-3:** tighten both readers identically to `CP3_WIRE_ADDENDUM.md` §1: adjacent segment indices, `integration_interval` with `start_k < end_k`, no duplicate entries. Add shared mutation cases.
- **N-4:** correct the stale hashes in a CHECKPOINT_4 record. Record that ROOT edited `CP3_READERS/RETURN.md` to remove two machine paths before committing `485cc2ed0`, and that the manager's 17-mutant run used pre-final bytes. The reviewer's rerun of all 17 on frozen bytes killed every one.
- **N-5:** run `rustfmt` over the new result_export files (a stable rustfmt is available) as a formatting-only change.
