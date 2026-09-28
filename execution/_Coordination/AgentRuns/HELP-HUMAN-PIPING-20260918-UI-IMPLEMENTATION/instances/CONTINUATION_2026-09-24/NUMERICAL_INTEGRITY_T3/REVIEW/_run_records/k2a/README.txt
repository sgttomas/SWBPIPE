RV7 (K2a review) run records. <scratch> = the reviewer's scratch directory; <wt> = the worktree root.
Builds: only from `git archive 80290ce98` (candidate) and `git archive 5ae22926e` (main, for the skew product probe
only), toolchain 1.97.1, RUSTUP_AUTO_INSTALL=0 CARGO_INCREMENTAL=0 --offline --locked, CARGO_TARGET_DIR=<wt>/rv7-target
(deleted after the slot). The probe tests rv7_rotated_m03.rs and rv7_skew_runtime.rs existed only in the scratch copies.

Runs, in order (SUMMARY.txt, SUMMARY2.txt, SUMMARY3.txt):
 slot 1 (rv7_run_slot.sh.txt):
   fk_base.log       - FAILED TO COMPILE the RV7 probe (ambiguous float literal); the K2a tests were not reached. Superseded.
   diag.log          - diagnostics k2a unit test: pass.
   mut_*.log         - 11 FK mutants on k2a_checked_formation (patches: rv7_mutants.py.txt), one at a time, fresh copy each.
   pp.log, pp_main_skew.log - FAILED to build: the archive lacked P/fixtures (include_str!). Superseded.
 slot 2 (rv7_run_slot2.sh.txt, first form):
   fk_base2.log      - K2a tests 10/10 pass; the RV7 probe panicked: 2f64.powi(-1030) evaluates to 0 (1/inf), so E = 0. Superseded.
   pp2.log           - PP k2a_formation_range_runtime 3/3 pass on 80290ce98 (the RV7 skew probe in it had E = 0: void).
   pp_main_skew2.log - interrupted by RV7 (same E = 0 defect). Void.
 slot 3 (rv7_run_slot2.sh.txt, final form; subnormal powers built exactly with from_bits):
   fk_base3.log      - RV7 kernel probe with FK's real transform_roundoff: the figures in K2A_REVIEW.md section 3.
   pp3.log           - RV7 skew product probe on the candidate: K2a refuses by name (24 runs).
   pp_main_skew3.log - RV7 skew product probe on main 5ae22926e: 24 runs, all NUMERICAL_INTEGRITY_UNRESOLVED.
Standard-library Python (no product code): rv7_transform_roundoff_replica (main's transform_roundoff replicated),
rv7_reach_recompute (reach_zero/reach_lef and the section 5.2 constants), rv7_skew_exact_ref (Fraction solve),
rv7_site_kinds (the non-normal kind each per-site row produces), rv7_match_scan (lexer scan for FrameKernelError matches).
