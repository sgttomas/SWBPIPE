# RV98: independent confirmation of I72's Pass B on the U8 head (u8_passb_01)

**Reviewer:** RV98, TASK (Type 2), dispatched directly by ROOT (HELP_HUMAN). No descendants. A fresh instance; I wrote none of the work. I hold the confirmation role RV89 held for Pass B.

**Brief:** `BRIEFS/RV98_U8_PASS_B_CONFIRM.md` (sha256 `3f830a98…`), under `BRIEFS/U8_COMMON.md`.

**Candidate:**
- **I72's record:** `R/I72/u8_passb_01/` (RETURN.md `18017da1…`; `SHA256SUMS` 47 of 47 OK), with I72's scratch `WT/scratch/i72_u8/` (`pass_u8/` without `work/`, `logs/`, `run.out.txt`).
- **The U8 head:** **`bd6b4be2c33cc64edf3e273bc126083872d03e24`** (tree `f8655453…`) on `codex/piping-f2a-u8-20261005`. Its base is NUM `b1e2d7741e`, whose tree outside `execution/` equals main `c1bfc460fc` (`git diff` empty).
- **The bases:** Pass A `ba1faa1c…` and the registered `0c7827b6ad`, as for `u4_g7_06`.

**Method:** read-only, with **no cargo or rustc**.
- I made my own `git archive` extract of the U8 head's `projects/chirality-piping` (without `execution/`). It has 2,952 files, equal to the tree's 2,952 blobs.
- **I re-ran every Pass B gate that does not build,** on my extract, with u4_g7_06's own tools, rules, references and 11-entry reviewed table, read in place. My driver is `evidence/scripts/rv98_g7_nobuild.sh`. It is I72's copy with the three cargo steps removed. `finish()`, the verdict line and exit code, the early stops, `text_summary` and the delta-tool failure handling are all kept (`evidence/tool/rv98_driver_vs_u4_g7_06.diff`).
- **The five build gates** (law, PP and runner outcomes, witnesses, challenge) I evaluated with the same checks on I72's recorded build outputs. I also ran `pass_checks.py` directly on I72's recorded outputs.
- I compared every output byte for byte with I72's run, with u4_g7_06's on F (`pass_frozen`) and with u9_refreeze_01's on F′ (`pass_refreeze`).

## Verdict: **PASS. I72's Pass B is confirmed.**

| | Count |
|---|---|
| BLOCKING | 0 |
| SHOULD-FIX | 0 |
| NOTE | 2 |

- **The verdict vector reproduces.** My re-run gives `VERDICT DELTAS TO READ exit=6`, with every gate 0 except `pp_outcomes:6`. That is I72's vector and u4_g7_06's on F.
- **My outputs equal I72's:** 95 of 96 common files are byte-identical, and 116 of 117 when `rr/` is included. The one difference is `VERDICT.txt`, by its tag. `cargo_summary.txt` exists only in I72's run, which built.
- **The delta has no production-class row.** My inventory is row-for-row equal to I72's: 46 files and 97 rows. Against F′, 10 rows are added and none is removed or changed.
- **I72's item 1 is confirmed from the source alone; no build is needed.** The two L = 0 fixtures are embedded only by a `#[test]` fn in a module file that is compiled only under `cfg(test)`. They are not production rows. N-1 refines the class name.
- **All 11 reviewed entries match,** with the same rows and entries as at F and F′. The maxima, the entry, the law record, the witnesses and the challenge are all unchanged from F.

## 1. The tool: unchanged in substance (`evidence/tool/i72_tool_diff_check.txt`)

**My diff** of I72's `g7_pass.sh` and `run_cargo_a.sh` against `R/I65/u4_g7_06/_run_records/` has 32 changed lines. Those are exactly the changed lines in I72's `tools_diff_vs_u4_g7_06.txt`. They are:
- **header comments** stating the retarget;
- **`O` and `G7L`:** output to `WT/scratch/i72_u8/`;
- **the cargo `--target-dir`s:** `WT/targets/i72-u8/`;
- **`REC_I72`,** used only so that item 5 calls the retargeted `run_cargo_a.sh`.

**Unchanged:**
- every `gate` call and its arguments;
- `finish()` and the verdict;
- the early stops after `tree`, `entry`, `law`, `linemap`, `premise` and delta = 5;
- `text_summary`;
- the delta-tool failure mapping and the no-inventory stop;
- N-4's emptying of `pass_<tag>/` and of the tag's logs;
- `guard()`;
- the flags, timeouts and outcome extraction in `run_cargo_a.sh`.

**What the copy reads:**
- `REC` is still `I65/u4_g7_06/_run_records` and `REC_A` is still `I65/u4_g7_01/_run_records`. Their `SHA256SUMS` are 72 of 72 and 78 of 78 OK, and their files are clean against NUM's HEAD.
- The tool files' mtimes (13:02–13:03Z) precede the run's lock START (13:04:49Z). I65's tools date from 2026-10-04.

**The run was one job under the T3 lock.** `WT/guard/cargo_jobs.log` shows WAIT 13:03:11Z, then START 13:04:49Z, immediately after the previous job's END. I72's END is 13:12:35Z with rc=6, and the next job STARTs at 13:12:36Z. **No other job overlapped.**

**The build compiled the checked copy.** Every `Compiling open_pipe_stress_product_physics` line (law, PP, runner) names `WT/scratch/i72_u8/pass_u8/work/…`. That is the copy that the tree gate had checked against the revision. The target was fresh: the law build compiled 37 crates.

**Confirmed: path retargets only. No gate weakened.**

## 2. The gate outputs (`evidence/run/`, `evidence/cmp/`, `evidence/build/`)

**My re-run on my own extract** (`evidence/run/run.out.txt`):

| Gate | Code | Result |
|---|---|---|
| tree | 0 | 2,952 listed, 0 mismatched, 0 extra |
| entry | 0 | equal to `git show 0c7827b6ad`'s, threshold `4_026_531_840` |
| law (I72's log, my entry) | 0 | identity, inputs and layouts equal; 42 passed, 0 failed; the registered tests ran |
| statics | 0 | none added or removed |
| linemap | 0 | exit 0 |
| premise | 0 | 3 pins as reviewed |
| text_run / text | 0 / 0 | D 14,734, no issues against Pass A |
| delta | 0 | 46 files, 97 rows: PASS |
| forms | 0 | equal to regeneration |
| noncand_run / noncand | 0 / 0 | 410, none new, none absent |
| controls_run / controls | 0 / 0 | 12 of 12 |
| pp_outcomes (I72's list) | **6** | +9 against Pass A, all `ok` |
| runner_outcomes (I72's list) | 0 | 87 lines, identical |
| witnesses / challenge (I72's logs) | 0 / 0 | 9 of 9 / 1 |

**`pass_checks.py` run directly on I72's recorded outputs** (`build/gate_checks_on_i72_outputs.txt`) gives the same codes:
- text 0 (D 14,734), statics 0, noncand 0, controls 0 (12/12);
- law 0, pp_outcomes 6, runner_outcomes 0 against Pass A and against F;
- premise 0, entry 0, forms 0;
- `noncand_compare.py` reproduces I72's comparison byte for byte.

**I72's outputs against F's `pass_frozen`** (`cmp/i72_vs_frozen.txt`): **87 of 97 identical, 10 differ.** These are exactly I72's ten, and I read each:

| File | Difference (read) |
|---|---|
| `VERDICT.txt` | basis and tag only; the gate vector is the same |
| `tree_blobs.tsv`, `gate_tree.json` | 2,952 against 2,950 listed. Against F, 16 changed or added blobs: F′'s `s11k_tests.rs`, plus the 15 below |
| `linemap.out.json`, `rules/g7_linemap.out.json` | the `new` revision id only |
| `delta.out.txt`, `delta_inventory.json` | the 11 rows added over F (§3) |
| `gate_law.json` | "502 filtered out" becomes "505" (U8's three tests), plus the time. Identity, inputs, layouts and registered tests are equal |
| `gate_pp_outcomes.json` | 716 becomes 719 lines; the 3 added lines are U8's tests, all `ok` |
| `cargo_summary.txt` | PP 705 becomes 708 passed; "552 filtered out" becomes "555"; times. Every `I65_G5_WITNESS` and `I65_G5_CHALLENGE` line is equal |

**I72's outputs against F′'s `pass_refreeze`** (`cmp/i72_vs_refreeze.txt`): **85 identical, 7 differ, 5 only in I72's run, 4 only in F′'s.**
- **The 7 that differ:** the six file-carrying outputs (`tree_blobs`, `gate_tree`, the two linemap outputs, `delta.out`, `delta_inventory`), plus `verdict.tsv`. F′'s `verdict.tsv` lacks the five build gates.
- **The 15-line tree difference against F′** is exactly `git diff 5488136a19 bd6b4be2c3` outside `execution/`:
  - 13 changed blobs: main's 8 documentation and policy files, plus U8's 5 changed files;
  - 2 added: the L = 0 fixtures.
- **Only in I72's run:** `VERDICT.txt`, `cargo_summary.txt`, and the gate files for law and the two outcome lists.
- **Only in F′'s:** I65's no-build notes (`F_to_Fprime_files.txt`, `GATES.txt`, `reviewed_inputs_blobs.txt`, `run.out`).
- **All 34 TEXT-chain files (`sens_pb/`) and all 25 controls files (`ctl/`)** are identical to F's and F′'s, and to mine.

**The build outputs against F** (`build/law_outcomes_vs_F.txt`, `build/witness_compare.txt`):
- **The law log:** all 268 `I65_G5_` lines are identical to F's, and so are the law-test outcome lines. Normalising timings and the filtered count, the logs are identical.
- **The maxima:**

  | Mode | Bytes | Fraction of M |
  |---|---|---|
  | sparse | 3,575,778,286 | 0.8881 |
  | dense | 3,595,488,734 | 0.8929 |

  `price_delta` is byte-identical to F's.
- **The identity:** rustc 1.97.1 `8bab26f4f68e`, aarch64-apple-darwin.
- **PP:** 708 ok, 1 FAILED (the known Mac `s11g_tests::t13`), 10 ignored (719 lines).
  - **Against F:** exactly 3 added lines, all `ok`: `u8_d_u6_5_l0_fixtures_are_the_live_successors`, `u8_l0_isolated_node_publishes_pinned_successor` and `u8_real_input_fallbacks_append_one_notice`. None removed.
  - **Against Pass A:** u4_g7_06's six plus these three.
- **runner/headless:** identical to F and to Pass A. 85 passed, 2 failed: the known `load_reference` pair.
- **Witnesses:** all 9 logs equal F's after normalisation (timings, paths, warnings).
- **The challenge:** equal to F's. The 34 `I65_G5_WITNESS` and `I65_G5_CHALLENGE` lines are identical, with peaks 3,541,898 and 2,252,863 B.

**Confirmed: every difference is explained by the U8 and main file changes, U8's three added `ok` tests, or the run's identity (revision, tag, times).**

## 3. My delta inventory, `ba1faa1c..bd6b4be2c3` (`evidence/delta/`, `run/delta_inventory.json`)

**My `delta_inventory2.py` run** on my extract, with edges and loop bounds from my own TEXT chain, gives **46 files and 97 rows**:

| Class | Rows |
|---|---|
| not-d1 | 30 |
| no-code | 35 |
| item | 4 |
| cfg-test-stmt | 3 |
| test | 12 |
| generated | 1 |
| qualification-test | 4 |
| unreachable | 8 |

- **Against I72's:** 0 rows differ.
- **Against F′'s 87:** 10 added, 0 removed or changed (`rows_rv98_vs_Fprime.txt`).
- **Against F's 86:** 11 added (the same 10, plus F′'s `s11k_tests.rs` `test` row), 0 removed or changed.

**The 10 rows added over F′:**
- **8 `not-d1` documentation and policy files from main**, which reached NUM before U8's base: `AGENTS.md`, `README.md`, `docs/AGENTIC_DEVELOPMENT_WORKFLOW.md`, `docs/README.md`, `docs/contributor_guide/index.md`, `loop/LOOP_INIT.md`, `loop/WORKPLAN_2026-09-19_piping_loop.md` and `validation/portability_policy.json`.
  - `git log c1bfc460fc --not 5488136a19` on these paths gives exactly #1084 (`f506f3e2de`), #1092 (`87661be164`) and #1094 (`7ba1181d43`).
  - The tool found no D1 embedder for any of them.
- **2 L = 0 fixtures**, labelled `unreachable` by the tool. §4 shows they are not production rows.

**U8's five other changed files keep their existing, unchanged rows:**

| File | Row |
|---|---|
| `retained_facade_tests.rs` | `test`, "a test file" |
| `RE/tests/retained_precision_contract.rs` | `not-d1` |
| `tests/test_retained_precision_contract.py` | `not-d1` |
| `retainedPrecision.test.ts` | `not-d1` |
| the corpus `retained_precision_cases.json` | `not-d1`, "not embedded by D1 production code" |

- **The corpus's only Rust embedder** is `RE/src/retained_precision.rs:4353`. It sits inside the `#[cfg(test)] mod u6e_reader_round_tests` span (:4349–4402), which the tool's in-file check recognises (`delta/tmf_check.out.txt`). The other Rust reference to the corpus is the integration test `RE/tests/retained_precision_contract.rs`, outside `src`.
- **The U8 diff** (`b1e2d7741e..bd6b4be2c3`, numstat) is exactly these 7 files: `retained_facade_tests.rs` +244/−0, the two fixtures, the corpus +16,139/−1, and the three reader tests. **The only file under a D1 crate's `src` that changed from F′ to U8** is `retained_facade_tests.rs` (`build/build_inputs.txt`).

**The reviewed table:**
- **With an empty table** the tool stops (exit 5), naming exactly the same 11 hunks as at F and F′: 7 `item`/`cfg-test-stmt` and 4 `qualification-test` (`delta/empty_table_delta.out.txt`).
- **With I65's table it passes.** The 11 rows carrying an entry match the table's 11 fingerprints one for one. They are equal to F's and F′'s in file, lines, class and entry (`delta/reviewed_check.txt`). Among them is RV89 N-1's `27181031…e585` at `source_blocks.rs` "after 55". I72's `reviewed_match.json` equals F's.

**Confirmed: no production-class row is added; all 11 reviewed entries match.**

## 4. I72's item 1: the L = 0 fixtures' class (`delta/tmf_check.out.txt`, `delta/rows_tool_vs_diag.txt`)

**Confirmed from the source. No build artefact is needed.**

**The only embedder.** `git grep` over the whole U8 head (outside `execution/`) finds exactly two `include_str!` sites for `retained_precision_l0_successor_*`: `PP/src/retained_facade_tests.rs:1051–1052`.
- They are inside `const FIXTURES` in `#[test] fn u8_d_u6_5_l0_fixtures_are_the_live_successors` (:1048–1071).
- The other references are a `std::fs::write` in the same file's `#[test] fn u8_l0_isolated_node_publishes_pinned_successor` (:1039), the corpus's `fixture` strings, and a Python test assertion. None is compiled into a D1 crate.

**The module is compiled only under `cfg(test)`.**
- `retained_facade_tests` is declared exactly once: `#[cfg(test)] mod retained_facade_tests;` at `PP/src/lib.rs:128–129`.
- **No other path pulls the file in.** No `#[path]` attribute and no `include!` names it, and no other crate declares it. PP's `Cargo.toml` has no `[features]`. No `.cargo/config` exists in the repository, in the build copy's ancestor folders or in `~/.cargo`. The registered build runs with RUSTFLAGS unset.
- **So the file is loaded only when PP's lib is compiled as its own unit-test harness.** An out-of-line module behind a false `cfg` is never loaded, so its `include_str!` is never expanded.
- **Every other artefact links PP's lib built without `cfg(test)`,** so none can carry the fixture text: the rlib, the 22 integration-test binaries (including the challenge), the examples and runner/headless.
- **I72's artefact scan agrees** (`runs/u8/fixture_embedding_check.txt`): each fixture's whole text appears in PP's lib test binary only. The targets have since been deleted, so I could not re-scan them, and the source argument does not need them.

**Why the tool says `unreachable`.**
- `delta_inventory2.py`'s data scan (:118–139) excludes an `include_str!` site only when it lies inside an in-file `#[cfg(test)]` item span. `retained_facade_tests.rs` has no in-file `cfg(test)` span; its `cfg(test)` is on the declaration in `lib.rs`.
- So the test fn is counted as an embedder. That fn is not on the D1 graph, which gives `unreachable`, a class that needs no entry.
- **The tool's own `test_module_file()`,** run unchanged on my extract, returns **True** for `retained_facade_tests.rs`. Every `mod retained_facade_tests;` declaration lies in `#[cfg(test)]`. The data scan simply does not consult it.

**What the tool would say if it knew.** I ran a diagnostic copy (`scripts/delta_inventory2_diag.diff`; memoised; not a gate) that skips test files and `cfg(test)` module files as embedders.
- **Exactly these two rows change,** from `unreachable` to **`not-d1`, "not embedded by D1 production code"**: the class the corpus gets. All other 95 rows are unchanged.

**So:**
- **The rows are not production-class.** No D1 production code embeds the fixtures, and they cannot enter any registered artefact.
- **No stop.** `unreachable` needs no entry, and the pass's verdict does not depend on the label.
- **I72's reading stands, with one refinement** (N-1): in the tool's own taxonomy the class is `not-d1`, not `test`.

## 5. The build-gate argument (brief item 4)

I72 ran the build, so item 4's inference is not needed: the build gates were measured. The argument would hold in any case (`build/build_inputs.txt`):
- **F′ → U8 head, outside `execution/`:** no PP or reader production file changes. The only D1-crate `src` change is the test file `retained_facade_tests.rs`. No `Cargo.lock`, `Cargo.toml` or `build.rs` changes; the only such files changed are records under `execution/`, which no build reads.
- **The 14 reviewed inputs** (PP's `Cargo.lock`, the schemas and the semantic-contract fixtures) have the same blob at `0c7827b6ad`, F, F′ and the U8 head.
- **The measured build agrees.** Law, PP, runner/headless, the nine witnesses and the challenge equal F's results, apart from U8's three added `ok` tests (§2).

## Findings

| ID | Sev | Where | Evidence | Remedy |
|---|---|---|---|---|
| N-1 | NOTE | `R/I72/u8_passb_01/RETURN.md`, "For ROOT" item 1, and the row class of the two L = 0 fixtures | **I72's item 1 is confirmed:** the rows are test-only, not production-class (§4). One refinement. The RETURN says making the embedder scan apply `test_module_file` "would label these rows `test`". In the tool's taxonomy it labels them **`not-d1`** ("not embedded by D1 production code"), as for the corpus, because the `test` class applies only to `.rs` files and hunks. Measured: my diagnostic copy changes exactly these two rows to `not-d1`, and no other | ROOT's ruling can record the two rows as **not-D1: embedded only by a `cfg(test)`-only module, test-only**. No change for this pass. If a later Pass B teaches the tool this case:<br>– it is a tool change, reviewed on its own;<br>– it removes embedders, so its review should check `test_module_file`'s premise;<br>– the tool's `cfg(test)` matcher also accepts `cfg(any(test, …))`. One such module exists, frame_kernel's `seeded` (feature `mutation-controls`, which no product manifest enables) |
| N-2 | NOTE | `R/I72/u8_passb_01/_run_records/runs/u8/law_record.txt` against `R/I65/u4_g7_06/_run_records/runs/frozen/law_record.txt` | I72's record file keeps only the `I65_G5_` lines, so it lacks F's final `test result:` summary line (274 lines against 275). The RETURN's "only the summary's 'filtered out' moves" describes the law log, not the record file. The 268 `I65_G5_` lines are identical, and so are the full law logs after normalising timings and the filtered count. The summary is in `gate_law.json` | None needed. Read the law summary from `gate_law.json` or the log |

## For ROOT

- **The one ruling:** confirm the two L = 0 fixture rows as test-only, not production-class. I recommend recording them as not-D1 (N-1). The provisional reading in "U8 Pass B returned; RV98 dispatched" is correct.
- **No registered build is needed** to settle anything.
- **Nothing blocks U8's freeze** from Pass B's side. The pre-freeze full suite remains ROOT's.

## Execution record

- **Who.** RV98, TASK (Type 2) under ROOT. No descendants, and no agent or subagent tool used.
- **When.** 2026-10-06, about 13:13–13:42Z, within the 2-hour box.
- **Host.**
  - **No cargo or rustc** was run, and no native, solver-at-scale or DEC-025 job.
  - Memguard (PID 5387) was running and checked by the driver.
  - The Python gate tools ran outside the T3 cargo lock, since they build nothing.
- **Git.** Reads only, with `GIT_OPTIONAL_LOCKS=0`: `archive`, `ls-tree`, `rev-parse`, `show`, `diff`, `log` and `grep`. No Git writes, and no installs.
- **Other agents' files** were only read: I72's records and scratch, I65's records and scratch (`pass_frozen`, `pass_refreeze`) and RV87's non-candidate reference. `WT/f2a-u8` is clean at `bd6b4be2c3`.
- **Writes:**
  - this folder (`REVIEW.md`, `evidence/`, `SHA256SUMS`);
  - `WT/scratch/rv98_u8_01/`: the extract `basis/`, the run's `pass_rv98/` and working files.
- **Cleanup.** The extract and the run's source copy (`pass_rv98/work`, 260 MB together) are deleted. The run's outputs (`pass_rv98/` without `work/`) and working files, 17 MB, are kept for ROOT; the periodic cleanup may remove them. The evidence here keeps every output the findings rely on.
- **The write guard** accepted the writes into NUM.
- **Paths.** Machine paths in the evidence are replaced by `WT` and `R`.

**Evidence** (`evidence/`):

| Folder | Contents |
|---|---|
| `tool/` | the driver diffs and I72's tool check |
| `run/` | my re-run's verdict, gate files and delta inventory |
| `cmp/` | the byte-for-byte comparisons |
| `delta/` | row comparisons, the empty-table run, the reviewed check, the `test_module_file` check and the diagnostic |
| `build/` | the build inputs, the law, outcome and witness comparisons, and the gate checks on I72's outputs |
| `scripts/` | the tools I wrote |
