RV24's review of KF2 (PR #1060, head f2b8c85a2d40d9b86e14a1b8b90274444d01a4e0, base main 78f55f927), 2026-09-29 (host clock).
The review is T3/REVIEW/KF2_REVIEW.md. Everything here was produced from the reviewer's own clean `git archive` copies of the
head and the base (projects/chirality-piping, execution/ excluded) under <wt>/rv24/, with the reviewer's probe copy and one clean
copy of `core` per mutant under <wt>/rv24-mut/ and targets under <wt>/rv24-target/ (all deleted at the end). Toolchain: rustc/cargo
1.97.1 (RUSTUP_TOOLCHAIN=1.97.1, RUSTUP_AUTO_INSTALL=0, CARGO_INCREMENTAL=0, --offline --locked, -j 4, RUST_TEST_THREADS=2, one
cargo job of the reviewer's at a time), Python 3.13 (standard library). The memory guard ran throughout (no KILLED line).
I20's uncommitted gate files in <wt>/scratch/i20/b/ were read, never written. Paths are written <wt>, <scratch>, <home>, <tmp>.

scripts/    make_old_fns.sh (extracts base 78f55f927 FK/structural.rs:2152-2215 from the base's bytes and renames the two functions;
            its output is rv24_old_fns.rs, the oracle), rv24_probe.rs (the differential harness: mounted in the reviewer's copy of
            the head as `#[cfg(test)] mod rv24_probe;` at the end of structural.rs; tests rv24_*), rv24_mutants.py (17 edits: NONE,
            six of I20's with I20's edit text, ten of the reviewer's), rv24_run_mutants.sh (the mutation driver), rv24_runs_diff.py
            (structural diff of B's two uncommitted part-1 runs.jsonl), rv24_reach_scan.py (lexer scan for the witness's callers),
            rv24_gate_sample.py (20-run sample of part 1 on base, head and instrumented head), rv24_gate_instr.py (the instrumented
            head over part 1), rv24_instrumentation.diff (the one-line stderr instrumentation, reviewer's copy only),
            probe_*_Cargo.toml (P1's probe manifests; main.rs is KF2 B's committed probe_main.rs.txt, cd1052f7...),
            rv24_built_order_search.py / rv24_built_order_scan.py (built-like edge pairs: verdict flips under a coupling swap),
            rv24_screen_check.py (the arithmetic of plan section 7.2). All as .txt.
probes/     harness_debug.out, harness_release.out (the rv24_ tests with their tallies), cost_release.out (I20's T6 and the
            reviewer's cost scaling test, release, with uptime before and after).
mutations/  summary.txt (verdict per mutant, split into I20/FK tests and RV24 tests), driver.log, and per mutant <id>.log (edit
            diff, test output, site table), <id>.build.log and <id>.rv24.txt (the harness tallies).
suites/     fk_full_suite_head_debug.log (FK's full suite on the pristine head copy: 415 passed, 1 ignored).
checks/     reference_copies_and_scope.txt (I20's reference copies against the base bytes; the PR's scope), b_records.txt (KF2
            folder integrity, T9 lists, the runs.jsonl hashes, part-1 dense/sparse disagreements, part 2), runs_jsonl_diff.out,
            reach_scan.out, built_order_search.out, screen_check.out.
gate/       probes_and_inputs.txt (probe binary and request hashes), build_probe_{base,head,instr}.log, sample_part1.out (20 runs x
            3 probes), instr_part1_candidate.jsonl and .summary.txt (860 runs), instr_n10_chain_rot.jsonl and .summary.txt.

To re-run the harness: copy scripts/rv24_probe.rs.txt and scripts/rv24_old_fns.rs.txt (or regenerate the latter with
make_old_fns.sh) into FK/src/structural/ of a head copy as rv24_probe.rs and rv24_old_fns.rs, append
`#[cfg(test)]\nmod rv24_probe;` to FK/src/structural.rs, and run `cargo test --lib rv24_ -- --nocapture` (add
`--release -- --ignored rv24_cost_scaling_release` for the cost observation).

confirm_1c7558df5/  RV24's confirmation of head 1c7558df5 (KF2_REVIEW.md, "Confirmation at 1c7558df5"): scope.txt (the delta
            f2b8c85a2..1c7558df5 and file hashes), mutations/ (NONE, RV24-M1, M4b and M5 on the committed tests only, from the
            reviewer's archive of 1c7558df5; driver.log, per-mutant logs and build logs), rv24c_run_mutants.sh.txt (the driver),
            fk_full_suite_debug.log (FK at 1c7558df5: 417 passed, 1 ignored), n3_count.txt (RV24-N3's count against B's index and
            the instrumented part-1 run), i20_records_check.txt (I20's mutant texts and KF2's SHA256SUMS at 1c7558df5).
