RV17 delta check of PR #1052 at 6fa422979 (one commit after f183e1fa9: tests and records only), 2026-09-29.

Host: as the review (toolchain.txt), under ROOT's delta rule: one cargo job of mine at a time, -j 4,
RUST_TEST_THREADS=4, the memory guard running (unchanged: two start lines). Every run used a clean git archive of
6fa422979 (projects/chirality-piping without execution/) under <wt>/rv17/<run>/ and a target under
<wt>/rv17-target/<run>, both deleted after the run. Placeholders as in ../README.txt.

Files:
- records_delta.txt (collect_delta_records.sh.txt): the commit's files (one test file under core, +337 -9; no product
  or solver file), F1b's SHA256SUMS at 6fa422979 (470/470), trailing whitespace as disclosed (24 files), 0 GEN-8
  machine-path hits, and I13's three requests equal to RV17's as parsed JSON.
- platform_check.txt (platform_check.sh.txt): the two new full-envelope pins inspected for functions of unspecified
  precision and for magnitudes with more than one nonzero component.
- mutants/: the runner (run_mutant3.sh.txt; run_mutant2.sh with the head and the log folder changed), the queue
  (queue4.sh.txt), MUTANTS.txt and the four logs. D-NONE is the control; D-F1B-M2 uses I13's mutate_f1b.py and
  D-RV17-M1 and D-RV17-M2 RV17's mutate_rv17.py (both byte-identical to the copies in I13's review_fixes_rv17/).
- assemble_delta.py.txt: the assembler of this folder, which also regenerated ../SHA256SUMS after checking that
  every file of the review's original record set is unchanged.
