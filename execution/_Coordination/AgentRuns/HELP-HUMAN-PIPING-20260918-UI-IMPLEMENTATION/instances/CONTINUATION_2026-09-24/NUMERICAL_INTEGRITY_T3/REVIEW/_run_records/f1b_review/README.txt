RV17: run records of the independent full-diff review of slice F1b (PR #1052), 2026-09-28/29.
Final head reviewed: f183e1fa9 (the brief named 9aeed9c22; ROOT moved the head during the review to c4879c496, the
pressure_thrust_load pin 07bed2638 and a merge of main 1cdeae2c1 with K5, and then to f183e1fa9, records only). P/core is
identical at c4879c496 and f183e1fa9; product_physics/src is identical at 130445db2, 9aeed9c22 and c4879c496
(revisions.txt).

Host: Mac, aarch64-apple-darwin, rustc and cargo 1.97.1 (toolchain.txt). RUSTUP_TOOLCHAIN=1.97.1
RUSTUP_AUTO_INSTALL=0 CARGO_INCREMENTAL=0 --offline --locked; at most two cargo jobs of mine at once, -j 4 to -j 8,
RUST_TEST_THREADS=4 (1 for the probes). Everything I built came from git archives of 9aeed9c22 or c4879c496
(projects/chirality-piping without execution/) under <wt>/rv17/<run>/, with targets under <wt>/rv17-target/<run>;
all deleted after the review. The largest model I formed has 4,000 members (C-SPARSE, sparse mode, in the PP
suite); no dense matrix at 10,000 or more members was formed. The memory guard log was unchanged (memguard.txt).

Placeholders: <wt> is the T3 worktrees root, <repo> the outer worktree, <home> the home directory, <scratch> the
session scratchpad, <tmp> a temporary directory. build_records.py.txt replaced machine paths with these, stripped
trailing whitespace and ended every text file with one newline when it assembled this folder, and checked that no
machine path remains. The envelopes under coex/envelopes/ and probes/envelopes/ are kept byte for byte (one JSON
line, no final newline); envelopes_sha256.txt lists them.

Files:
- revisions.txt (collect_revisions.sh.txt): the commits, the slice's numstat, both merges checked against main's
  delta with an empty remerge diff, the pin commit's files, and product_physics/src identical across the heads.
- k5m_checks.txt: the spot-verification of F1b's RETURN addendum 2 (f183e1fa9): its gate part 1 against G1's base and
  against the 130445db2 gate, its uncommitted runs.jsonl hash, its T9 hashes against the calibration, its suites.
- records_checks.txt (collect_records.sh.txt): F1b's SHA256SUMS at 9aeed9c22 (356/356), c4879c496 (379/379) and
  f183e1fa9 (449/449), the trailing-whitespace disclosure checked file by file, the path and model-identifier scans,
  and the uncommitted raw gate files: the five hashes in uncommitted_sha256.txt and gate2/SHA256SUMS verified in full
  (3,456 files).
- gate/: c3_check (gate part 1 partitioned independently from the committed tables: C3 832 runs, 0 differences; C1
  28; C2 24), raw_full_check (116 raw full envelopes against the committed table), c1_references (the 14 published
  C1 runs against R1's exact references with P1's predicate, trusted or not, with main's same-family Sensitive runs),
  thin_b_exact (my own exact-rational oracle for RF-RANGE-THIN-B).
- coex/: the coexistence constructions CX-A to CX-G (D10 step 4), their requests, the runs through the recorded main
  and candidate probes (coex_summary.txt, coex2_summary.txt, coex_runs_*.jsonl), the head build's envelopes compared
  with both (head_compare.txt), and main's four selected-and-published CX-F/CX-G envelopes.
- mutants/: the runner (run_mutant.sh for 9aeed9c22, run_mutant2.sh for c4879c496), the queues, my patcher
  mutate_rv17.py, the provenance of I13's reused patchers, MUTANTS.txt, every log, and the NI kill-site comparison
  with I13's record (compare_ni_sites.out). Run ids prefixed H- ran on c4879c496; the others on 9aeed9c22. An
  old-head RV17-M2 run was stopped when the head moved and left no result line.
- probes/: the scratch probe tests (never committed to the product; rv17_probe_mem.rs was written but not needed,
  because RV17-M3 was killed by the suite), their requests, probe_compare.txt (main, the clean trees and the
  mutants, byte for byte), and the six envelopes the findings cite.
- toolchain.txt, memguard.txt, build_records.py.txt, SHA256SUMS.
