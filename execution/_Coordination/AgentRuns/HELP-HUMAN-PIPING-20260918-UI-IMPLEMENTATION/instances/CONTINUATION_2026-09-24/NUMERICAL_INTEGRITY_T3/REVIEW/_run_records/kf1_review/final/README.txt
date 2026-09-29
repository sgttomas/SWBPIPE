RV20: confirmation of KF1's head 66adfede4 (PR #1056, parent 1854911d1), 2026-09-29, at ROOT's request.
Mac, aarch64-apple-darwin, rustc and cargo 1.97.1. RUSTUP_TOOLCHAIN=1.97.1 RUSTUP_AUTO_INSTALL=0 CARGO_INCREMENTAL=0
--offline --locked, -j 4, RUST_TEST_THREADS=2, one cargo job at a time (a DEC-025 sweep may have been running).
NONE, RV20-M4 and RV20-M5 each ran from a clean `git archive 66adfede4` of the FK tree with a fresh target under
<wt>/rv20c-target, deleted after its run; <wt>/rv20c is deleted. <wt>/kf1 was used read-only.

Files:
- records_checks.txt: the head's parent; the 11 files changed since 1854911d1 (none under src/); KF1's SHA256SUMS at the
  head (42 of 42, set-equal); GEN-8's MACHINE_ABS_PATH_RE and a user-name scan over the PR's 47 files (0 hits); I18's
  rv20/mutants.py and mutants.jsonl (the same M4 edit, built from a THROWAWAY constant, and the same M5 edit).
- mutants.jsonl, logs/: NONE passes 8 of 8 (coverage, sizes and shared-cap lines equal to the review's run; 0 "moved"
  lines at T = 512, 129 at T = 1); RV20-M4 killed by kf1_the_shared_cap_bounds_a_calls_trackers_together ("round 0: the
  set's work 1032854 < K4's 3413670"); RV20-M5 killed by kf1_the_stop_rules_trackers_finish_in_k4s_map_order
  ("assertion failed: RuleTest::Estimate < RuleTest::Charge").
- rustfmt.txt: the changed test file is rustfmt-clean.
- confirm_mutants.py.txt: the script.
