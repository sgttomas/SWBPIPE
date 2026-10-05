# I65: Pass B's no-build gates on the refrozen head F′ (u9_refreeze_01)

**Basis:** **F′ = `5488136a193ef8921bd88c33a8124647c4cb352d`** (tree `1b41c07e…`), against the same bases as u4_g7_06: Pass A `ba1faa1c…` and registered `0c7827b6ad`.

**No cargo or rustc was run,** because DEC-025 is running on the host. The tool is `_run_records/g7_pass_nobuild.sh`: u4_g7_06's `g7_pass.sh` with every build step removed, a fresh `git archive F′` straight into `pass_refreeze/work`, and `price_delta` reading F's recorded law record. The header states each difference.

**Outputs:** WT/scratch/i65_u4_g7_01/pass_refreeze/, a new tag.

**F → F′:** outside `execution/`, exactly one file changes, `core/solver/nonlinear_integration/src/s11k_tests.rs` (`F_to_Fprime_files.txt`).

## Gate codes on F′ (`runs/refreeze/GATES.txt`): all 0

| Gate | Code | Against u4_g7_06 on F |
|---|---|---|
| tree | 0 | 2,950 of 2,950 blobs on a fresh `git archive` copy. The listing differs from F's in exactly one line: the `s11k_tests.rs` blob (`73210fc6` → `0e3b884b`) |
| entry | 0 | byte-identical to `0c7827b6ad`'s (threshold 4_026_531_840); gate output identical |
| statics | 0 | identical (`s11k_tests.rs` is a `_tests.rs` file, outside the statics list) |
| linemap | 0 | identical except the `new` revision id. No rule moved; `s11k_tests.rs` is a test file and outside the map |
| premise | 0 | identical; the three pins are as reviewed |
| text_run / text | 0 / 0 | **every TEXT-chain output byte-identical to F's:** the call graph and its audit, edges, lexicon, the regenerated inventory, all four TEXT runs, T07, T08, T25, producer, ordinary, composite, `profile_tree.json` and the summary (D 14,734) |
| delta | 0 | **87 rows against F's 86. Exactly one row is added, `core/solver/nonlinear_integration/src/s11k_tests.rs`, class `test` ("a test file":** `nonlinear_integration/src` is in `crate_dirs.txt`, and the file ends `_tests.rs`). Every other row is identical in file, lines, class, fingerprint and reviewed entry. **All 11 reviewed entries still match** |
| forms | 0 | identical |
| noncand_run / noncand | 0 / 0 | the 410; the non-candidate dump and the comparison are byte-identical |
| controls_run / controls | 0 / 0 | 12 of 12; the output is byte-identical |
| price_delta (not a gate) | — | byte-identical, computed from F's law record |

`runs/refreeze/rows_vs_F.txt` lists the 60 outputs compared byte for byte (all identical) and each difference, read.

## Gates not run (they build), and why they cannot change between F and F′

**Not run:**
- **law:** the compiled identity, the reviewed inputs and the layouts, and the registered law tests;
- **PP and runner/headless outcomes;**
- **the nine witnesses;**
- **the challenge.**

**Why they cannot change:**
- **What these builds compile is the same at F and F′.** The only changed source file, `s11k_tests.rs`, is declared `#[cfg(test)] mod s11k_tests;` in `nonlinear_integration/src/lib.rs`. A dependency is always built without `cfg(test)`, so the file enters only `nonlinear_integration`'s own test target, and none of the gated builds compiles it:
  - PP's lib, its lib test binary and its integration tests (the challenge);
  - runner/headless;
  - the witnesses.
- **No `nonlinear_integration` test belongs to the gated suites.** None of the 716 PP outcome lines on F names `nonlinear_integration` or `s11k`. Neither `Cargo.toml` (PP, runner/headless) is a workspace with members.
- **The identity inputs are unchanged.** The 14 reviewed inputs have the same blob at `0c7827b6ad`, F and F′ (`reviewed_inputs_blobs.txt`). The compiled identity depends only on the toolchain, target, profile and flags, which are unchanged on this host. The reader layouts come from `result_export`'s types, whose source is unchanged.
- So F's results stand for F′: law 0; PP 705/1/10 with the six added tests; runner 85/2; witnesses 9 of 9; challenge peaks 3,541,898 / 2,252,863 B.
- **The full Pass B verdict on F′ would be exit 6, with the same six-test delta.**

## For ROOT

- **Nothing needs a ruling from the gates:** every no-build gate is 0, and the only row added is the expected `test` row.
- **What is inferred:** the build gates on F′ come from the argument above, not from a run. If ROOT wants them measured, `g7_pass.sh` (u4_g7_06) on F′ under a new tag runs them once the host is free.

## Execution

- I65, TASK, no descendants; 2026-10-05 UTC.
- Memguard PID 5387 was running. **No cargo or rustc.**
- No Git writes: `archive`, `ls-tree`, `diff`, `show` and `rev-parse` were run as reads with `GIT_OPTIONAL_LOCKS=0`. No source changes.
- **Writes:**
  - this folder;
  - WT/scratch/i65_u4_g7_01/pass_refreeze/ only.
- Placeholder paths only. `SHA256SUMS` covers this folder.
