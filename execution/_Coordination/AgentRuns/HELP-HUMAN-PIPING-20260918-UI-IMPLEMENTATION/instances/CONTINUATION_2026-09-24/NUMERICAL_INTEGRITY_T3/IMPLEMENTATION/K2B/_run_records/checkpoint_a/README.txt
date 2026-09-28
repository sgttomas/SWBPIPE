K2b checkpoint A and the targeted re-run before checkpoint B (I10, 2026-09-28; Mac, aarch64-apple-darwin).

Which run each file is:
- ni_first.log, ni_k2b_1.log, ni_k2b_2.log, pp_targeted_1.log: development runs during checkpoint A.
  Their failures are in-progress states fixed before the checkpoint-A handback. For example,
  pp_targeted_1.log is the PP s11f rule-6 test failing before the PP site-test rows were added.
- a_k2b_summary.log: the checkpoint-A run of the two NI formation-range tests, on the checkpoint-A
  tree (committed as 6ce4d694b). It holds the LEF-large worst ratios and the reach results that
  went into the checkpoint-A stop (ROOT's rulings A-C, a95adb540).
- run_a.sh.txt, a_fk.log, a_sd.log, a_ni.log: the full FK, SD and NI crates. These files hold the
  last run of run_a.sh. That run was on the tree after the rulings (the checkpoint-B candidate,
  committed as 70828d4d6), before the suites. The logs of the checkpoint-A run itself were
  overwritten by it. Counts: FK 193, SD 30, NI 111 + 4 doc tests; no failures; no warnings.
- The checkpoint-A PP targeted run (s11f_site_test 11, formation_check_runtime 5,
  k2a_formation_range_runtime 3) was not kept as a file. The same tests pass in checkpoint B's
  suites (checkpoint_b/suites/logs/008_core_product_physics_Cargo.toml.log) and in checkpoint C's
  NONE control (mutations/logs/NONE.log).
