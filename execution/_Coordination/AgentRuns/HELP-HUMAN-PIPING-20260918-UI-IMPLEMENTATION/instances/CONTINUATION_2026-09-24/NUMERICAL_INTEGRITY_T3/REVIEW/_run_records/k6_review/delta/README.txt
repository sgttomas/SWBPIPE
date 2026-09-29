RV18 delta check of PR #1053 at cd325c1fe (one commit after ae3320b5a: I15's fixes for RV18's review), 2026-09-29.
Same host rules as the base review (../README.txt, ../toolchain.txt): git archive copies of cd325c1fe under <wt>/rv18,
targets under <wt>/rv18-target and one fresh target per mutant, all deleted afterwards; at most two cargo jobs of RV18's
own, -j 4, RUST_TEST_THREADS=2; <wt>/k6 used read-only (git read commands and GEN-8). No observation run above 100
members and no dense run at 1,000 members or more. The N6 probe computed counts (O(nnz), about 158 MB of heap) for
four CONT n10000 variants under a 512 MiB heap cap, and every variant was refused before any lane entry.

Files:
- revisions.txt: the PR head and hosted checks, the commit, the delta's files by kind, the poll loop's verbatim move,
  the is_cont_n10000 call sites.
- records_checks.txt: both SHA256SUMS at the head, the M5-log lines as scrubbed, GEN-8's regex over the PR's 1,145
  files, identifier scans, the memory guard log.
- gen8.txt: GEN-8 at cd325c1fe in <wt>/k6 (1 passed; the working tree clean before and after).
- suites/: H's cargo test (suite_delta_NONE.log, the Rust NONE control), the pytest wrapper and the two DEC-050/053
  pins (pytest_delta.log), --smoke with the release binary (smoke_delta.log, smoke_delta_summary.txt).
- mutations/: mutation_table.txt and logs/ (controls first).
- probes/: probe_signals.out (SIGTERM and SIGINT to a running runner; the SIGTERM handler before, during and after
  launch(); a caller's handler; a worker thread), probe_n6.out and probe_n6_method.txt (the content-keyed predicate).
- scripts/: RV18's run scripts and signal probe, and I15's own_mutants.py as committed, as .txt.
Synthetic test paths that appear in failing assertions are marked <synthetic-...-path>, and machine paths are
replaced with <wt>, <repo>, <VENV>, <home>, <scratch> and <tmp>.
