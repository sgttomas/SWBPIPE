RV21: run records of the independent review of slice V-K (PR #1057, head 3fd1baff335e7f62a78095499c83488c6ad387d8).
The review is REVIEW/VK_REVIEW.md. Placeholders: <wt> is the T3 worktree root on the owner's Mac, <scratch> the session
scratchpad, <home> the home directory. Machine paths in logs were replaced by these placeholders before commit.

Copies and targets (all deleted after the review):
  <wt>/rv21/head   git archive of 3fd1baff3 (projects/chirality-piping, with execution/): VR suite, probes, mutants,
                   the seeded-fault subset (its FK copy carried RV21's two own faults: scripts/rv21_own_faults.diff.txt),
                   gen_vk_cases.py --check, the adapter check and the floor derivation
  <wt>/rv21/clean  git archive of 3fd1baff3 without execution/ (as CI): FK's full suite
  <wt>/rv21/base   main 0f5d8c7b4's FK plus A0's patch (patch -p1): the base for vk_own_fk.diff.txt
  targets: <wt>/rv21-target (VR, no features), <wt>/rv21-mut-target (VR --features seeded-faults),
           <wt>/rv21-fk-target (FK)

Files:
  toolchain.txt                 rustc/cargo/python, host, environment
  a0_check.txt                  A0: patch-ids, removed/added line classification, the facade
  vk_own_fk.diff.txt            diff -ru of (main 0f5d8c7b4 + A0) FK against the head's FK: V-K's own change
  records_checks.txt            SHA256SUMS verification and set equality, machine-path scan, retained_api and manifest
                                scans, CI discovery, B's summary regeneration, PR checks (read only)
  suites/vr_build.log, vr_suite.log      VR: fresh build, 44 of 44 (cargo test, no features)
  suites/fk_suite.log           FK's full suite on the clean archive, FK_SEEDED_FAULT unset: 402 passed
  suites/runner_tests.log       runner/test_vk_scale_runner.py: 8 of 8
  suites/gen_check.log          gen_vk_cases.py --check: 16 of 16 OK
  suites/ci_numerical_vr_excerpt.txt     VR's step in the dispatch's numerical job (read with gh api)
  probes/rv21_probes_run.log    rv21_probes (value, outcome and constructed probes) and rv21_engine (225,405 vectors)
  probes/rv21_probes_o6.log     O6's full failure list
  probes/engine_vectors.sha256.txt       the vectors' sha256 (file not committed; scripts/rv21_engine_vectors.py.txt)
  probes/adapter_check.log      scripts/rv21_adapter_check.py.txt: 213 cases, 0 problems
  probes/floor_rederive.log     scripts/rv21_floor.py.txt: 46 / 3 / 2, set-equal to the committed lists
  probes/proposed_tests_fixcheck.txt     RV21-2's proposed tests: pass on the head, kill RV21-H4 and RV21-H6
  faults/fault_matrix.txt, logs/         the seeded-fault subset and RV21-A, RV21-B (scripts/run_faults.sh.txt)
  mutants/harness_mutants.txt, logs/     RV21-H1..H6 (scripts/run_harness_mutants.py.txt)
  guard/                        RV21-1: the probe self-test fails on the head, passes with feature_guard_fix.diff.txt
  scripts/                      every script and test RV21 wrote, as .txt

Commands (from VR's directory in the archive unless stated; env as in toolchain.txt):
  cargo test --offline --locked -j 4
  cargo test --offline --locked -j 4 --test rv21_probes --test rv21_engine -- --nocapture   (RV21_VECTORS=<vectors>)
  FK_SEEDED_FAULT=<id> cargo test --offline --locked -j 4 --features seeded-faults --lib --test lane --test parity
      --test adapter --test files --test engine --test rcm --test invariance --test feature_guard --test scale
  python3 -B cases/gen_vk_cases.py --check          (from cases/)
  python3 -B -m unittest runner/test_vk_scale_runner.py
  python3 -B rv21_adapter_check.py <archive>/projects/chirality-piping
  python3 -B rv21_floor.py <archive>/projects/chirality-piping
  cargo test --offline --locked -j 4 --no-fail-fast  (from FK's directory in the clean archive)

SHA256SUMS covers every file in this folder except itself.
