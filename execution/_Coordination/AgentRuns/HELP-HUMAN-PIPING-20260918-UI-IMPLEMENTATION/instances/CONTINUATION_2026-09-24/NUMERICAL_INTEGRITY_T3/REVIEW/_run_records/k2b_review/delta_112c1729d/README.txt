RV11 second delta check of K2b's commits f9a15fbd6, 85ae94b41 and 112c1729d (on f385a8bc8), 2026-09-28.
Same host, toolchain and limits as before (../toolchain.txt): opt-level 0, -j 8 single jobs, -j 4 mutants (three at
once), RUST_TEST_THREADS=4. Builds from git-archive copies under <wt>/scratch/rv11; targets under <wt>/rv11-target
(mutant targets deleted after each run). <wt>/k2b used read-only for the state check and GEN-8 only.

Files:
- commits_check.txt: the three commits, the paths each changes (all in K2b's write set), no pre-K2b body changed.
- state_check.txt: <wt>/k2b at 112c1729d, clean, index equal to HEAD's tree.
- head_tests/: FK, SD and NI in full, and PP s11f_site_test and formation_check_runtime, on the 112c1729d archive.
- probes/: the probe module re-based on 112c1729d (F-S2 and F-R added) and delta2_probe_spring.log (F-S, F-S2, F-R).
- b0probe/: I10's b = 0 probe rebuilt against 112c1729d; its list and the comparison with the earlier lists.
- mutations/: NONE first; RV11's three survivors of the first delta check re-run (mutate_rv11d.py, unchanged) and two
  new RV11 mutants (mutate_rv11d2.py).
- records_checks.txt, gen8.txt: the candidate's records at 112c1729d, and GEN-8 in <wt>/k2b.
