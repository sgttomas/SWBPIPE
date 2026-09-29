RV11 delta check of K2b's fix commits (bf4647c21, 14f9b093f, f385a8bc8 on 087b3a088), 2026-09-28.
Same host, toolchain and limits as the first review (../toolchain.txt): opt-level 0, -j 8 single jobs, -j 4 mutants
(three at once), RUST_TEST_THREADS=4. Everything built came from git-archive copies under <wt>/scratch/rv11, targets
under <wt>/rv11-target (mutant targets deleted after each run). <wt>/k2b was used read-only for the index check and
GEN-8 only.

Files:
- commits_check.txt: the three commits, the paths each changes (all in K2b's write set), and no empty blobs.
- index_check.txt: I10's disclosed `git add -N`, reset: no trace in <wt>/k2b's index, working tree or reflog.
- head_tests/: FK, SD and NI in full and PP s11f_site_test and formation_check_runtime on the f385a8bc8 archive.
- probes/: the probe module re-based on f385a8bc8 (its F-A2 and F-D use the kernel's Result-returning functions; F-E
  and F-S are new), with delta_probe.log (F-A, F-A2, F-B, F-C, F-D, F-E) and delta_probe_spring.log (F-S).
- b0probe/: I10's b = 0 probe source rebuilt against eb52114e9, 98b1723b1 and f385a8bc8; the three lists and the
  comparison.
- mutations/: NONE first, then I10's four new mutants, RV11's two census mutants, and four new RV11 mutants.
- callers/: the lexer scan with the two new names, on f385a8bc8.
- records_checks.txt and gen8.txt: the candidate's records at f385a8bc8, and GEN-8 in <wt>/k2b.
