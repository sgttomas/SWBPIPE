RV20: run records of the independent review of slice KF1 (PR #1056, head 1854911d1, base main 8cca91701), 2026-09-29.
Mac, aarch64-apple-darwin, rustc and cargo 1.97.1 (toolchain.txt). RUSTUP_TOOLCHAIN=1.97.1 RUSTUP_AUTO_INSTALL=0
CARGO_INCREMENTAL=0 --offline --locked, -j 4, RUST_TEST_THREADS=2, one cargo job at a time (I17 was building V-K).
Everything built came from `git archive 1854911d1` copies under <wt>/rv20 with targets under <wt>/rv20-target:
- head: the FK tree with its sibling crates (core/solver and core/loads/load_case_algebra, which FK's integration
  tests include), unmodified;
- probe: the FK tree plus RV20's test-only additions (probes/probe_adaptive.diff.txt, probes/rv20_probe_tests.rs.txt);
- one fresh copy and target per mutant, per hook-check variant and per fix-check variant, each deleted after its run.
<wt>/kf1 was used read-only (git read commands and file reads); nothing was built or written there.
Copies and targets were deleted at the end.

Placeholders: <wt> is the T3 worktrees root, <home> the home directory, <scratch> the session scratchpad. Machine paths
in the logs were replaced with these when this folder was assembled.

Files:
- revisions.txt: PR #1056's head and base (read-only gh), the first-parent history, the merge base, the empty piping
  diff ab02ee3a6..8cca91701, the files the PR changes outside KF1's records, the `pub` scan, retained's visibility, the
  scan for the tracker, the set or the retained module outside FK, GEN's files unchanged, the site-table diff, the
  sha256 of the four code files, the hosted checks.
- records_checks.txt: KF1's SHA256SUMS against the committed bytes (35 of 35, set-equal), GEN-8's
  MACHINE_ABS_PATH_RE and a user-name scan over the PR's 40 files (0 hits), the reference copy against K4's code
  (verbatim), no ExtremeTracker left and the construction sites.
- suites/fk_full.log: the non-test build (no warning) and FK's full suite on the head copy (401 passed, 0 failed).
- suites/kf1_tests.log: KF1's 7 tests with --nocapture (coverage, sizes, the 129 "moved" lines at T = 1, none at 512).
- suites/hook_check.log: a non-test probe naming `tracker_hook` fails to compile (E0433); the same probe naming
  the non-test `tracker_rows` compiles.
- probes/probe_tests.log: RV20's four probe tests (probes/rv20_probe_tests.rs.txt) on the probe copy:
  arbitrary keys and collapse schedules against K4's code and an oracle, with J1-J3 after every row; the tracker set
  with tiny caps; the RuleTest order; the key-stream replay that derives the collapse pins.
- probes/probe_adaptive.diff.txt: the probe copy's test-only changes to adaptive.rs (a key override under
  #[cfg(test)], a log of the stop rule's offers, the probe module).
- mutations/: mutation_table.txt, mutants.jsonl, logs/ (NONE first), and fixcheck_rv20_1.log (RV20-1's proposed
  assertion: passes on the head, kills RV20-M4).
- scripts/: RV20's scripts, as .txt.
- toolchain.txt.
