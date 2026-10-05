# I65 U4 G9a: the full Pass B on the F2a D1 milestone PR head (u4_g7_05)

**Basis:** PR #1082's head **`6b9bb19a5f66143101b6364757ecdfa12da455ef`** (tree `ca6d08ef…`), cut from main `5fdc5ab601`. I extracted it read-only with `git archive` (`GIT_OPTIONAL_LOCKS=0`) to WT/scratch/i65_u4_g7_05/pr/. It equals the tree blob for blob: 2,950 of 2,950. The request was ROOT's "U9 G9a" (U9 decision 4).

**Tools:** u4_g7_04's, unchanged except for `REC` (`runs/tools_diff_vs_u4_g7_04.txt`) and one new reviewed entry (7 entries in all). Run outputs are under WT/scratch/i65_u4_g7_01/pass_<tag>, as the tool writes them.

## Verdict: `DELTAS TO READ`, exit 6. The one delta is the same six added PP tests

The run is `runs/pr/`.

| Gate | Code | Result |
|---|---|---|
| tree | 0 | 2,950 of 2,950 |
| entry | 0 | **byte-identical to `0c7827b6ad`'s:** identity, inputs, layouts, threshold 4_026_531_840 |
| law | 0 | the compiled identity, **all 14 reviewed-input hashes** and the layouts equal the entry; 42 passed, 0 failed; the registered tests ran |
| statics | 0 | none added or removed |
| linemap | 0 | 7 changed production files; 0 rules moved (PR1080's +3 lines at `source_blocks.rs:1055` are after every rule key in that file) |
| premise | 0 | `:4252`, `:4253` and `:4305` are as reviewed |
| text_run / text | 0 / 0 | complete; D 14,734; every row and output identical to Pass A |
| delta | 0 | 30 files, 75 rows, every one classified, and every one that needs an entry is reviewed |
| forms | 0 | equal to regeneration from G7's tree |
| noncand_run / noncand | 0 / 0 | the 410 |
| controls_run / controls | 0 / 0 | 12 of 12 |
| **pp_outcomes** | **6** | **+6, all `ok`:** grant 2's five `retained_facade_tests::u3g2_*` and D-U6-5's carrier test, the same six as before |
| runner_outcomes | 0 | 85 passed, 2 failed, identical |
| witnesses / challenge | 0 / 0 | 9 of 9 / peaks 3,541,898 / 2,252,863 B |

**Against the U7 head run** (u4_g7_04 `runs/u7`), these are identical (`runs/pr/vs_u7_head.txt`):
- the PP and runner/headless outcome lists;
- the witness outcomes.

PP is 705 passed, 1 failed (t13), 10 ignored. **The maxima are unchanged:** 0.8881 M sparse and 0.8929 M dense.

## The deltas read (`runs/pr/delta_inventory.json`; from Pass A's basis `ba1faa1c`)

| Class | Rows | What |
|---|---|---|
| not-d1 | 20 | 10 TypeScript files; 2 `core/analysis_runs` Python files, including `compatibility.py` (PR1078 resolved); 3 `result_export/tests/*.rs`, including PR1080's `tests/source_blocks.rs`; 2 fixtures; 3 Python tests |
| no-code | 31 | comments and doc lines, including `2e03d7cc25`'s citation rewording and `fc575c7e56`'s comments: `lib.rs` 4, `retained_memory.rs` 5, reader 21, `semantic_contract.rs` 1 |
| unreachable | 5 | `semantic_contract.rs` `classification_summary`/`_from` (4 hunks, `8c84e7ae14`: not on the lexical D1 graph); **PR1080's `source_blocks.rs:1055–1058`** (see below) |
| item | 3 | reviewed: the U7 flag (D1-live, I66 evidence) and `lib.rs:2235`/`:2254` (doc only), unchanged from u4_g7_04 |
| cfg-test-stmt | 3 | reviewed: grant 2's three hooks |
| test | 11 | `#[cfg(test)]` `lib.rs` hunks, `grant2.rs`, `retained_facade_tests.rs`, `retained_wire_tests.rs` |
| generated | 1 | the T17_V4 line (FORMS gate) |
| qualification-test | 1 | **`retained_memory_law_tests.rs:1238`: reviewed this run** |

**PR1080 (`5b4f31766c`), `source_blocks.rs:1055–1058`, inside `validate_in`.** The tool classes it as not live on D1 (`runs/pr/pr1080_reachability.txt`).
- **Why it is dead.** `validate_in`'s only callers are `source_blocks::validate` and `physics_source::validate`. On D1 they are reached only from `semantic_contract::for_source`, through G4's `edge_zero` rules. Those rules hold because the reader's projection carries the literal preview-physics-1 id (the premise pins), so `for_source` never takes the source-blocks or physics-source arm. Their other caller, `numerical_use_standing_with_context`, is not on the D1 graph.
- PP calls `result_export` only through `retained_precision::validate` (lib.rs:3161).
- **Even if it were reached, nothing changes in allocation or text.**
  - The loop now walks the receipt's `rows` (a `&Vec<Value>`) instead of the `treatments` `HashMap` (still built for the coverage check), and reads `id` by `text(&treatment["result_id"])?`, which returns `&str`.
  - Neither walk allocates.
  - `text`'s error path (`"SOURCE_BLOCKS_STRING".into()`) cannot newly fire: the `treatments` collection just above already ran `text` on every row's `result_id`.
- So no stop.

**`retained_memory_law_tests.rs:1238`** (`2e03d7cc25`, flagged by I61). One `///` doc line of `not_attempted_examples` changes its citation from `PP/lib.rs:3918` to ``lib.rs `solve_load_case_observed` ``.
- No code changes: the examples, the assertions, `PINNED_RECORD` and the registered tests are as before. The law gate passes 42 of 42.
- **Entered as "doc comment only"** in `delta_reviewed.json`.
- The first run (`runs/pr1/`) gave `delta:6`, as designed (N-5), and continued to the same final gates.

**Stop conditions, all clear:**
- PR1080 is not live on D1, and changes no allocation or text;
- the entry and the 14 reviewed inputs are unchanged;
- no premise pin moved.

## Execution

- I65, TASK, no descendants; 2026-10-04.
- Memguard PID 5387 was running, and my cargo jobs ran strictly one at a time (the pass is sequential; no other cargo process was running at its start). `--locked --offline`.
- No Git writes: `git archive`, `ls-tree`, `diff` and `show` were run as reads with `GIT_OPTIONAL_LOCKS=0`. No source changes.
- **Writes:**
  - this folder;
  - WT/scratch/i65_u4_g7_05/ (the extract, `s/`, logs);
  - WT/scratch/i65_u4_g7_01/pass_pr1 and `pass_pr`;
  - WT/targets/i65_g7/.
- Placeholder paths only. `SHA256SUMS` covers this folder.
