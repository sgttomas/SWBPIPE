RV18: run records of the independent full-diff review of slice K6 (PR #1053, head ae3320b5a, base main 59cb20073), 2026-09-29.
Mac, aarch64-apple-darwin, rustc and cargo 1.97.1 (toolchain.txt). RUSTUP_TOOLCHAIN=1.97.1 RUSTUP_AUTO_INSTALL=0
CARGO_INCREMENTAL=0 --offline --locked, -j 4, RUST_TEST_THREADS=2, at most two cargo jobs of RV18's own (I12 was building).
Everything built came from `git archive ae3320b5a` copies under <wt>/rv18 (the head) with the target <wt>/rv18-target;
each mutant had its own fresh archive and target under <wt>/rv18, deleted after its run. <wt>/k6 was used read-only
(git read commands and GEN-8); nothing was built or written there. No observation run above 100 members and no dense run
at 1,000 members or more was made: the binary ran only at 10 and 100 members, the DEC-053 nine, --smoke, --counts-only
at 100 members or fewer, --emit-model (every model, O(n)) and --list-models.

Placeholders: <wt> is the T3 worktrees root, <repo> the outer worktree, <VENV> the repository venv, <home> the home
directory, <scratch> the session scratchpad, <tmp> a temporary directory. Machine paths in the logs were replaced with
these when this folder was assembled; the assembly checks that GEN-8's MACHINE_ABS_PATH_RE finds nothing here.

Files:
- revisions.txt: the PR and its checks (read-only gh), the first-parent history and the branch base, the two merges
  (remerge diff, main-delta equality, files outside the write set), the D commit, the Scope 8 path scan, H/src/lib.rs's
  diff, the lock's new packages (no source or checksum line), the dependency-closure scan, the provenance copy against
  H/Cargo.lock at main, the Rust sources changed since B's binary commit, the runner's hash per slot.
- records_checks.txt: both SHA256SUMS, the packet and D-table regeneration, GEN-8's regex over the PR's files,
  identifier scans, the M5-log lines (redacted), the memory guard log.
- gen8.txt: GEN-8 at the head in <wt>/k6 (1 passed; the working tree clean before and after).
- suites/: H's cargo test from the archive (suite_head.log), the non-test and release builds (no warning), the pytest
  wrapper and the two DEC-050/053 pins (pytest_head.log), --smoke (smoke.log).
- checks/: lane_gap_ceiling.txt (lane-id ratios, the product gap, the ceiling repeats), crosscheck.out (RV18's own check of the 24 RF-LARGE models against references.py's model_json, with
  y_reference and the sha256 list), counts_oracle.out (RV18's own RCM and skyline port against counts.jsonl, 21
  models), records_check.out (parity counts, product counts against counts.jsonl, ascent and admission arithmetic, F1b
  ratios, fits, N10, RSS/footprint), digest_check.out (the parity lines re-derived from the recorded digests; summary
  peaks against stage peaks), cont_identity_quadratic.txt, py_model_hashes.txt, smoke_summary.txt.
- probes/: probe_term.out (SIGTERM to the runner leaves the wrapper and child running), probe_stops.out (the binary's
  two time stops act), probe_m3_m4.out (RV18-M3 and M4 change the recorded peaks).
- mutations/: mutation_table.txt and logs/ (controls first).
- scripts/: RV18's scripts, as .txt.
