RV11: run records of the independent review of slice K2b (PR #1040, head 087b3a088), 2026-09-28.
Mac, aarch64-apple-darwin, rustc and cargo 1.97.1 (toolchain.txt). RUSTUP_TOOLCHAIN=1.97.1 RUSTUP_AUTO_INSTALL=0
CARGO_INCREMENTAL=0 --offline --locked RUST_TEST_THREADS=4; -j 8 for single jobs, -j 4 for mutants (at most three
at once). Default dev profile (opt-level 0) throughout; no profile was added. Everything built came from git-archive
copies under <wt>/scratch/rv11, with targets under <wt>/rv11-target (mutant targets deleted after each run).
Nothing was built or written in <wt>/k2b; GEN-8 ran there read-only (gen8.txt).

Files:
- merge_check.txt: the head and PR state after a fetch; the merge 087b3a088 adds exactly main's delta and nothing
  else, touches no slice file, and has an empty remerge diff.
- head_tests/: FK, SD and NI in full, and PP s11f_site_test and formation_check_runtime, on the head archive.
- callers/: a lexer scan (comments, strings and #[cfg(test)] items removed; test files skipped) for every new K2b
  name, on the head and (as a control) the base.
- b0probe/: I10's b = 0 Debug probe source, rebuilt at opt-level 0 against archives of eb52114e9 and 087b3a088,
  with the comparison against I10's recorded hash lists.
- probes/: the reviewer's probe module (added only to a scratch copy of the head, as a child of k2b_tests.rs) and
  its consolidated log (probe_final.log, one run of all five probes), and a binary64 boundary check in Python.
  F-A2 is finding RV11-1; F-B is RV11-2; F-C and F-D are the forced-b invariance grids. Earlier development runs
  of the same probes (one with a wrong reference DOF in F-A2) are superseded by probe_final.log and not kept.
- mutations/: NONE first, then 7 of I10's mutants re-run and 4 of mine (mutate_rv11.py.txt), from clean archives
  of the head. MUTANTS.txt has the kills; logs/ has every run.
- records_checks.txt: SHA256SUMS, path and model-identifier scans, RETURN section 2's counts, the suites claim
  against the logs, the T9 records, and git diff --check.
- gen8.txt: GEN-8 on the head.
- build_records.py.txt: the assembler of this folder.
