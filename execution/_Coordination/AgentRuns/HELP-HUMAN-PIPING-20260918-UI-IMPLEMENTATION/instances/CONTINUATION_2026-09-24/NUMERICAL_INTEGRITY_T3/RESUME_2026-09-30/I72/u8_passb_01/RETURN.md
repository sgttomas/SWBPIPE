# I72 U8-4: the fail-closed Pass B on the U8 head (u8_passb_01)

**Brief:** `BRIEFS/I72_U8_PASS_B.md` (sha256 `adfc130d…`), under `BRIEFS/U8_COMMON.md` (`3146c3e6…`). I hold the Pass B role that I65 held.

**Basis:** U8 head **`bd6b4be2c33cc64edf3e273bc126083872d03e24`** (tree `f8655453…`) on `codex/piping-f2a-u8-20261005`. It sits on NUM `b1e2d7741e`, whose maintained source equals main `c1bfc460fc`.
- I extracted it read-only with `git archive` (`GIT_OPTIONAL_LOCKS=0`) into WT/scratch/i72_u8/basis, without `execution/`.
- It equals the tree blob for blob: 2,952 of 2,952. That is F′'s 2,950 plus the two new L = 0 fixtures.
- **The bases are the same as `u4_g7_06`'s:** Pass A `ba1faa1c…` and the registered `0c7827b6ad`.

## The tool

I ran u4_g7_06's fail-closed **`g7_pass.sh`**, the tool that builds, as ROOT confirmed.

**What it reads, unchanged:**
- its tools, rules, references and the 11-entry `delta_reviewed.json` from `R/I65/u4_g7_06/_run_records/` (`SHA256SUMS` 72 of 72 OK before the run);
- Pass A's references from `R/I65/u4_g7_01/` (78 of 78 OK).

**What I changed.** My copies of `g7_pass.sh` and `run_cargo_a.sh` are in `_run_records/`. They differ from I65's only by a path retarget, plus header lines that state it (`_run_records/tools_diff_vs_u4_g7_06.txt`):
- outputs go to WT/scratch/i72_u8/;
- the cargo targets go to WT/targets/i72-u8/;
- item 5 calls the retargeted `run_cargo_a.sh`.

Every gate, every early stop and the verdict logic are I65's.

**How it ran: one job under the T3 cargo lock.**
- `launch.sh` runs `/usr/bin/lockf -k WT/guard/cargo_job.lock /bin/bash i72_locked_job.sh <basis> <rev> u8`.
- The wrapper logs WAIT, START and END to `WT/guard/cargo_jobs.log`, in `t3_cargo_run.sh`'s format: WAIT 13:03:11Z, START 13:04:49Z, END 13:12:35Z, rc=6.
- It never calls `t3_cargo.sh`.
- **Inside the lock:**
  - `--locked --offline`, `CARGO_BUILD_JOBS=4`, `RUST_TEST_THREADS=2` (1 for the witnesses and the challenge, as in I65's runner), RUSTFLAGS unset;
  - rustc 1.97.1 `8bab26f4f68e`, aarch64-apple-darwin;
  - a fresh target directory.

## Verdict: `DELTAS TO READ`, exit 6, as expected: the same verdict and the same single non-zero gate as u4_g7_06

The run is `_run_records/runs/u8/`. The verdict line, verbatim:

`VERDICT DELTAS TO READ exit=6 basis=bd6b4be2c33cc64edf3e273bc126083872d03e24 tag=u8 overlay=none gates=tree:0 entry:0 law:0 statics:0 linemap:0 premise:0 text_run:0 delta:0 text:0 forms:0 noncand_run:0 noncand:0 controls_run:0 controls:0 pp_outcomes:6 runner_outcomes:0 witnesses:0 challenge:0`

| Gate | Code | Result | Against u4_g7_06 (F) / u9_refreeze_01 (F′) |
|---|---|---|---|
| tree | 0 | 2,952 of 2,952, 0 mismatched, 0 extra | F 2,950 / F′ 2,950. The listing against F′ differs in exactly 15 lines: 13 changed blobs and 2 added |
| entry | 0 | **byte-identical to `0c7827b6ad`'s,** M = `4_026_531_840` | gate output identical to both |
| law | 0 | identity, the 14 reviewed inputs and the layouts equal the entry; 42 passed, 0 failed; the registered tests ran | **the law record is identical to F's, line for line:** identity, inputs, layouts, profile, phases, atoms and budgets. Only the summary's "filtered out" moves from 502 to 505, which is U8's three tests. F′ did not run it |
| statics | 0 | none added or removed | identical |
| linemap | 0 | no rule moved | identical except the `new` revision id |
| premise | 0 | `:4252`, `:4253` and `:4305` are as reviewed | identical |
| text_run / text | 0 / 0 | complete; **D 14,734**; every row and output identical to Pass A | **all 34 TEXT-chain outputs byte-identical** to F's and F′'s |
| delta | 0 | 46 files, 97 rows, every one classified; every one that needs an entry is reviewed | F 35 / 86, F′ 36 / 87 (below) |
| forms | 0 | equal to regeneration from G7's tree | identical |
| noncand_run / noncand | 0 / 0 | the 410 | identical |
| controls_run / controls | 0 / 0 | 12 of 12 | all 25 controls outputs byte-identical |
| **pp_outcomes** | **6** | **+9 against Pass A's reference, all `ok`:** u4_g7_06's six and U8's three | **against F, +3, all `ok`, none removed:** `u8_real_input_fallbacks_append_one_notice`, `u8_l0_isolated_node_publishes_pinned_successor`, `u8_d_u6_5_l0_fixtures_are_the_live_successors` |
| runner_outcomes | 0 | 85 passed, 2 failed (the known `load_reference` pair), identical | identical to F |
| witnesses / challenge | 0 / 0 | 9 of 9 / peaks 3,541,898 and 2,252,863 B | **every witness and challenge line identical to F's** |

- **PP** is 708 passed, 1 failed (the known Mac `s11g_tests::t13`), 10 ignored. I68's U8-1 return reported the same counts.
- **The maxima are unchanged:** 0.8881 M sparse and 0.8929 M dense. `price_delta` is byte-identical to F's.
- **Every other gate is 0, as the brief expects.**

## The delta rows (`runs/u8/delta_inventory.json`, `u8_rows.json`, `delta_rows_vs_F_and_Fprime.txt`)

**Against F′ (87 rows),** 10 rows are added and none is removed or changed in file, lines, class, fingerprint or reason. **Against F (86 rows),** the same 10 are added, plus F′'s `s11k_tests.rs` (`test`).

### U8's own seven files (`b1e2d7741e..bd6b4be2c3`, the whole U8 diff)

| File | U8 change | Class | Row |
|---|---|---|---|
| `PP/src/retained_facade_tests.rs` (`2a1229b5…`) | +244 / −0 | `test`: a test file | existing, unchanged |
| `core/reporting/result_export/tests/retained_precision_contract.rs` | +23 / −6 | `not-d1`: not in a D1 crate's `src` | existing, unchanged |
| `tests/test_retained_precision_contract.py` | +68 / −7 | `not-d1` | existing, unchanged |
| `apps/desktop/src/features/results/retainedPrecision.test.ts` | +45 / −0 | `not-d1` | existing, unchanged |
| `fixtures/results/retained_precision_cases.json` (07k `482449bf…` → 07l `5ac13296…`) | +16,139 / −1 | `not-d1`: not embedded by D1 production code | existing, unchanged |
| `fixtures/results/retained_precision_l0_successor_sparse_interactive.json` (`93c6c865…`) | new | **`unreachable`**: embedded only by fns not live on D1. The embedder is `retained_facade_tests.rs:1051`, in the `#[test]` fn at `:1049` | **added** |
| `fixtures/results/retained_precision_l0_successor_dense_scrutiny.json` (`dbb3d477…`) | new | **`unreachable`**, the same fn, `:1052` | **added** |

**No production-class row.**
- **No U8 row needs a reviewed entry.** None is `live`, `item`, `cfg-test-stmt`, `generated`, `data`, `data-live`, `no-code` or `qualification-test`.
- **No hunk touches a production file.**
- **The fixtures' `unreachable` label is the tool's, and its content is test-only.** I read it as a test row, not a stop:
  - **The embedder is test code.** `retained_facade_tests.rs` is declared only as `#[cfg(test)] mod retained_facade_tests;` (`PP/src/lib.rs:128–129`), and the embedding fn is a `#[test]`.
  - **Why the tool says `unreachable`.** Its embedder scan recognises `#[cfg(test)]` only inside the embedding file. A fixture that a `cfg(test)` module *file* embeds therefore falls to `unreachable`, which needs no entry and does not stop the pass.
  - **The build artefacts confirm it** (`runs/u8/fixture_embedding_check.txt`). Each fixture's whole text, which `include_str!` embeds contiguously, is present in PP's lib test binary only. It is absent from:
    - PP's non-test rlib, in both targets;
    - all 22 PP integration-test binaries;
    - all 10 runner/headless binaries.
  - This reading is for ROOT (below).
- **The corpus (I69's note 6).** The only D1-crate `src` embedder is `result_export`'s `#[cfg(test)] mod u6e_reader_round_tests` (`retained_precision.rs:4349–4353`). The tool's span check recognises that module, so the corpus classes `not-d1`.
  - `d37` and `cases[0]` are unchanged from F′ through 07l to the U8 head (`runs/u8/corpus_pins_check.txt`: `d37` `226bd5e1…`, `cases[0]` `93e48c55…`).
  - The corpus bytes appear in none of the gated artefacts.

### The other eight added rows come from main, not U8: all `not-d1` documentation

- **The files:**
  - `AGENTS.md`, `README.md`;
  - `docs/AGENTIC_DEVELOPMENT_WORKFLOW.md`, `docs/README.md`, `docs/contributor_guide/index.md`;
  - `loop/LOOP_INIT.md`, `loop/WORKPLAN_2026-09-19_piping_loop.md`;
  - `validation/portability_policy.json`.
- **Where they come from:** main's #1084, #1092 and #1094 (`git log c1bfc460fc --not 5488136a19`). NUM absorbed them before U8's base `b1e2d7741e`.
- **None is embedded by a D1 crate.** I grepped every `crate_dirs.txt` source for these names.

## The 11 reviewed entries: 11 of 11 matched (`runs/u8/reviewed_match.json`)

| Entry | Matched row |
|---|---|
| Grant 2's three `cfg-test-stmt` | `lib.rs:2379`, `lib.rs:3005`, `retained_product.rs:3244` |
| The two `lib.rs` `#[doc]` lines (doc only) | `:2235`, `:2254` |
| The U7 flag (D1-live, I66's evidence) | `retained_precision.rs:4267–4269` |
| `:1238` (doc comment only) | `retained_memory_law_tests.rs:1238` |
| RV89 N-1 (`27181031…e585`) | `source_blocks.rs` "after 55" |
| The three S-1 qualification-test doc hunks | `retained_memory_law_tests.rs:2–4`, `retained_memory_witness_tests.rs:6`, `tests/retained_memory_challenge.rs:6–10` |

**The reviewed rows are identical to F's and F′'s** in fingerprint and entry.

## Comparison with u4_g7_06 (F) and u9_refreeze_01 (F′), byte for byte

The tool is `compare_outputs.py`, run against I65's scratch outputs. It skips `work/`, `tmp/`, `rr/` and `logs/`.

**Against F's `pass_frozen`** (`runs/u8/outputs_vs_frozen.txt`): **87 of 97 files are identical.** That covers:
- all 34 `sens_pb/` TEXT-chain outputs and all 25 controls outputs;
- the edges, lexicon, call graph and audit;
- the §11 dump and comparison, the statics and `price_delta`;
- `D.txt` and `verdict.tsv`;
- the `entry`, `statics`, `premise`, `text`, `forms`, `noncand`, `controls` and `runner_outcomes` gate files.

**The ten that differ, each read:**

| File | The difference |
|---|---|
| `VERDICT.txt` | basis and tag only |
| `tree_blobs.tsv`, `gate_tree.json` | the listing, 2,952 against 2,950 |
| `linemap.out.json`, `rules/g7_linemap.out.json` | the `new` revision only |
| `delta.out.txt`, `delta_inventory.json` | the 11 added rows above |
| `gate_law.json` | 505 against 502 filtered out, and the time |
| `gate_pp_outcomes.json` | 719 against 716 lines |
| `cargo_summary.txt` | PP 708 against 705 passed, the filtered counts (555 against 552) and times |

**Against F′'s `pass_refreeze`** (`outputs_vs_refreeze.txt`): **85 identical.**
- **Seven differ:** six outputs carry the F′→U8 file changes (`tree_blobs`, `gate_tree`, the two linemap outputs, `delta.out` and `delta_inventory`), and `verdict.tsv` carries the five build gates that the no-build run lacked.
- **Five exist only here:** the build-gate outputs.
- **Four exist only there:** I65's no-build notes.

**u9_refreeze_01's build-gate argument is now measured, not inferred.** On the U8 head, law, PP, runner/headless, the nine witnesses and the challenge equal F's results, apart from U8's three added `ok` tests.

## For ROOT

1. **To rule: the two L = 0 fixtures' class.** The tool labels them `unreachable` ("embedded only by fns not live on D1"), not `test`.
   - I read them as test rows, not a production-class stop. The embedder is a `#[test]` fn in a `#[cfg(test)]`-only module file, and the compiled artefacts carry the fixture text only in PP's lib test binary.
   - The tool is unchanged. Making its embedder scan apply its existing `test_module_file` check would label these rows `test`, but that would be a tool change for a later Pass B, not for this one.
   - RV98 can confirm the reading.
2. **For information: eight `not-d1` documentation rows are in the delta but not in U8.** They are main's #1084, #1092 and #1094, already on main `c1bfc460fc`.
3. **For RV98: the driver is a path-retargeted copy.** The only diff is `tools_diff_vs_u4_g7_06.txt`. The reviewed table, rules, references and checks are read from I65's u4_g7_06 folder itself.

Nothing else needs a ruling. No stop condition arose.

## Execution

- **Who and when.** I72, TASK, no descendants; 2026-10-06 UTC.
- **Host.** Memguard PID 5387 was running. The whole pass was one job under the T3 cargo lock, and I ran no other cargo or rustc command. No DEC-025, native or solver-at-scale job, and no install.
- **Git.** Reads only, with `GIT_OPTIONAL_LOCKS=0`: `archive`, `ls-tree`, `show`, `diff`, `rev-parse`, `log`, `merge-base`. No source changes. `WT/f2a-u8` is clean at `bd6b4be2c3`.
- **Writes:**
  - this folder;
  - WT/scratch/i72_u8/;
  - WT/targets/i72-u8/;
  - three lines in WT/guard/cargo_jobs.log.
- **Cleanup.**
  - **Deleted:** the target (3.7 GB), the basis extract and the pass's source copy.
  - **Kept for RV98:** WT/scratch/i72_u8/ with `pass_u8/` (without `work/`), `logs/` and `run.out.txt`, 23 MB in all.
- **The write guard** accepted every write into NUM.
- **Records.**
  - **Supporting scripts** in `_run_records/`: `post_run.py` gathered `runs/u8/`, `compare_outputs.py`, `fixture_embedding_check.py` and `corpus_pins_check.py`.
  - Placeholder paths only (`WT`, `R`, `P`, `PP`, `NUM`, `~`). `SHA256SUMS` covers every other file in this folder.
