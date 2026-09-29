RV14 delta check 2 of K5 at head babcf5e65 (a4378835c on 28517eaaa, plus the merge of main b37331092), 2026-09-29.
Built from clean git-archive copies of babcf5e65 under <wt>/scratch/rv14, targets under <wt>/rv14-target (deleted
afterwards); ROOT released the host: two cargo jobs at once, -j 8, RUST_TEST_THREADS=4, rustc 1.97.1, --offline --locked.

Files:
- records_checks_delta2.txt: the head after a fetch, PR state and hosted checks at that time, a4378835c's files, the
  merge checks, K5's SHA256SUMS, the new test file, check()'s P arm, RETURN's D2 record, ROOT's ruling, the RV14-M5
  patch, git diff --check, and the path, model and user-name scans.
- mutations/: rv14_run_mutant_delta2.sh.txt, batch_delta2.log, MUTANTS_DELTA2.txt, and the NONE and RV14-M5 logs
  (the 4,226-case probe output kept as its sha256 only).
- memguard_delta2.txt.
