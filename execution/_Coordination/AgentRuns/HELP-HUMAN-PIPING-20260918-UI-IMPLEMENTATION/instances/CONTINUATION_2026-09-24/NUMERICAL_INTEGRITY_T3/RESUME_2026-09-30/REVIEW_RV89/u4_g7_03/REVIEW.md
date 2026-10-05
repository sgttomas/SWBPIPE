# RV89: U9 G9a, the full U4 Pass B on the F2a D1 milestone PR (#1082)

**Reviewer:** RV89, TASK (Type 2), dispatched directly by ROOT. No descendants. This continues RV89's G7 reviews (`u4_g7_01`, `u4_g7_02`).

**Candidate:**
- **I65's run:** `R/I65/u4_g7_05/` (RETURN.md `ec93eb99…`; `runs/pr/`, `runs/pr1/`), on the Pass B basis **`6b9bb19a5f`**: the snapshot of U = `2e03d7cc25`, plus main `5fdc5ab601`, plus the package.
- **The new entry:** one reviewed entry (`retained_memory_law_tests.rs:1238`).
- **ROOT's addition:** the PR head's moves to `fd3cbebb42` and then **`92a5a9da1c`**. The latter adds a two-line reorder in `source_blocks.rs` `fn integer`.

**My copy:** WT/rv89_pr/base, a `git archive` of `6b9bb19a5f`. It matches the tree file for file: 2,950 of 2,950, with no extra file under projects/chirality-piping.

**Oracles:** RV89's own, with builds kept modest (`CARGO_BUILD_JOBS=2`).
- My registered build of `6b9bb19a5f`: law tests, all nine witnesses, the challenge, PP, runner/headless and my 71-input sweep.
- **Copy-only counters** on `source_blocks::validate_in` (calls and loop rows) and on `fn integer`.
- **A counting global allocator,** over all threads, around:
  - the milestone's Direct entry;
  - `semantic_contract::for_source` on four source-blocks-1 documents (n05 and multicase, both modes).
  
  Each was run as committed and with the hunk set back to its parent.
- **Pass B's tools,** run by me: the `entry` and `outcomes` gates, and `delta_inventory2.py` on `6b9bb19a5f` and on `92a5a9da1c` (`evidence/`).

## Verdict: **PASS**

| | Count |
|---|---|
| BLOCKING | 0 |
| SHOULD-FIX | 0 |
| NOTE | 1 |

**My findings:**
- **PR1080 cannot be reached on D1. Even where it is reached, it adds no allocation and no text.**
- **The new reviewed entry is right.**
- **The exit-6 delta is exactly the six tests,** the entry is unchanged byte for byte, and the maxima are unchanged.

**On the moved PR head:**
- **`fd3cbebb42` needs no rerun:** it touches only CI tooling and its Python test.
- **`92a5a9da1c`'s `integer` reorder** cannot be reached on D1 and is allocation-identical, so it needs no rerun now. ROOT's mechanical Pass B on the frozen head suffices. **But that rerun will stop with exit 5** on the deletion half of the reorder unless one reviewed entry is added first (N-1). I confirm what that entry should say.

## 1. PR1080 (`5b4f31766c`, `source_blocks.rs:1055–1058` in `validate_in`)

**By reading** (my copy):
- **The callers.** `validate_in` (:673) is called by `source_blocks::validate` (:670) and `physics_source::validate` (:1131). Both are called only from `semantic_contract::for_source`, in its source-blocks-1 and physics-source-1 branches (:464 and the physics-source branch), and from `numerical_use_standing_with_context` (:772–774).
- **The D1 path.** On D1, PP calls result_export only at lib.rs:3161 (`retained_precision::validate`). That reaches `for_source` only on `project(source)`, whose `producer.semantic_contract_id` is **assigned** the preview-physics-1 literal (retained_precision.rs:4252–4253, the premise pins). So `for_source` takes only the preview-physics branch.
  - That is G4's `edge_zero` cut, `for_source → source_blocks.rs:669:validate`.
  - `numerical_use_standing_with_context` is not on the D1 graph.
- **On I65's lexical graph** (`pass_pr/edges_pb.json` and its `edge_zero` rules):
  - `validate_in` is reached but **not live**.
  - The only live `source_blocks` functions are `domain_hash` and `text`.
- **If it were reached:**
  - The new walk iterates the receipt's `rows`, a borrowed slice, instead of the `treatments` map, which is still built just above for the coverage check.
  - It reads the id with `text(..)?`, which returns `&str`.
  - Its error path cannot newly fire, because building `treatments` has already run `text` on every row's `result_id`.

**By measurement** (`evidence/source_blocks_probes.txt`):
- **On D1:** across the milestone's Direct entry in both modes, which publishes the successor, `validate_in` has **0 calls and 0 loop rows**.
- **Across my whole 71-input sweep** (all five routes): **0 and 0**.
- **Off D1, where it is reached** (`for_source` on n05 and multicase source-blocks-1 documents, both modes, two rounds):
  - each call enters `validate_in` once and walks 79, 80, 158 or 160 rows, returning `Ok("0.3.0")`;
  - **the allocation counts and bytes are identical with the new loop and the old** in all 8 pairs. For example, n05 sparse gives 24,043 allocations / 2,757,837 B, and multicase dense 48,012 / 5,501,917 B.

**Confirmed: not reachable on D1. Where it is reached, it is allocation-identical and adds no text.**

## 2. The new reviewed entry (`retained_memory_law_tests.rs:1238`, qualification-test)

- **The hunk** (`2e03d7cc25`) is −1/+1 inside the `///` doc comment of `not_attempted_examples`. It changes the citation from `PP/lib.rs:3918` to ``lib.rs `solve_load_case_observed` ``.
- No code, example, assertion, `PINNED_RECORD` or registered test changes.
- My law tests give 42 passed, 0 failed, with outcomes and the printed record identical to the U7 head.
- **With an empty table,** `delta_inventory2.py` flags exactly this hunk as `qualification-test`, alongside the six earlier stop hunks. **With I65's table it passes.**

**Confirmed: doc comment only.**

**The other delta rows check out:**
- `classification_summary` and `_from` (`8c84e7ae14`, 4 hunks) are `unreachable`: they are not on the D1 graph, and neither PP nor `validate` calls them.
- There are 31 `no-code` rows, including `2e03d7cc25`'s and `fc575c7e56`'s comment rewording in `lib.rs`, `retained_memory.rs` (doc lines outside the generated block and the entry) and the reader.
- There are 20 `not-d1` rows.

## 3. The run

**My registered build of `6b9bb19a5f`** (`evidence/pr_run.txt`):
- **PP:** 705 passed, 1 failed (t13), 10 ignored. The outcome list is **identical to my U7-head run** (`cfda60403f`).
- **Against Pass A's reference,** the outcomes gate gives 6, with exactly six added `ok` lines: the five `retained_facade_tests::u3g2_*` and D-U6-5's carrier test.
- **Identical to the U7 head:**
  - runner/headless (85 passed, 2 failed);
  - the law outcomes (42 passed, 0 failed, 9 ignored);
  - the printed record;
  - all nine witnesses' and the challenge's output, with peaks 3,541,898 / 2,252,863 B.
- **The maxima:** 0.8881 M sparse (3,575,778,286 B) and 0.8929 M dense (3,595,488,734 B), unchanged.
- **My sweeps of `6b9bb19a5f`,** with counters, and with `92a5a9da1c`'s `integer` in place, are **both byte-identical to the G6R registered sweep** (sha256 `25cce1e1…ccbb`).
- **The entry:** the entry gate on my copy against `git show 0c7827b6ad` gives equal, threshold `4_026_531_840`.

**I65's run** (`runs/pr/VERDICT.txt`): `DELTAS TO READ exit=6`, and its only non-zero gate is `pp_outcomes:6`.

**Confirmed: the exit-6 delta is exactly the six tests, the entry is unchanged byte for byte, and the maxima are unchanged.**

## 4. The moved PR head

**`fd3cbebb42`** changes only `tools/ci/e2e_plan.py` and `tests/test_ci_e2e_plan.py`. That is CI policy in Python, outside the D1 crates, not a reviewed input and not embedded; the delta tool classes both `not-d1`. **I agree it needs no rerun.**

**`92a5a9da1c`**, `source_blocks.rs` `fn integer` (:54–58): the 2^53−1 `filter` now runs on the `u64` before `usize::try_from`. The same two lines are reordered, and the line count is unchanged.
- **Unreachable on D1:**
  - `integer`'s callers are `validate_in`, `source_plan`, `supports` and `summary`, all reached only through the same `edge_zero` cut (not live on I65's graph).
  - **Measured:** `integer` has **0 calls** on the milestone's Direct entry in both modes and across my whole sweep, run with `92a5a9da1c`'s `source_blocks.rs` in my copy.
- **Allocation-identical and text-free:**
  - `as_u64`, `filter`, `try_from` and `ok_or_else` allocate only on the error path, with the same error text as before.
  - On 64-bit both orders give the same result.
  - **Measured off D1:** `integer` is called 72 or 139 times per `for_source`. The allocation counts and bytes are identical between the new order and the old in all 8 pairs.
- **The rerun:** no Pass B rerun is needed now. ROOT's mechanical rerun on the frozen head suffices, with N-1's entry added first.

## Finding

| ID | Sev | Where | Evidence | Remedy |
|---|---|---|---|---|
| N-1 | NOTE (for the frozen-head rerun) | `u4_g7_05/_run_records/delta_inventory2.py:186` (`… and False`: a deletion-only hunk takes no enclosing fn from the old side) | **The mechanical Pass B will stop on the moved head.** `git diff -U0` shows `integer`'s reorder as two hunks: the old `.and_then(|n| usize::try_from(n).ok())` line deleted after :55, and re-added at :57.<br>– The added half is classed `unreachable` (`integer`, reached only through `edge_zero`).<br>– The deletion half (`after 55`, −1/+0) has no new lines, so it falls to `item` and needs a reviewed entry.<br>– My run of `delta_inventory2.py` for `ba1faa1c..92a5a9da1c` gives **exit 5**, `STOP: 1 hunk(s) need a reviewed entry`, with fingerprint **`2718103130462590a4379a0c336fc369cb0267826de894b3cce2040241f2e585`**.<br>– This fails closed and is not a defect. The content is confirmed in §4 | Before the frozen-head rerun, add a reviewed entry for that fingerprint: "`source_blocks.rs` `fn integer` (:54), the deletion half of `92a5a9da1c`'s two-line reorder; the line is re-added at :57. Unreachable on D1 (`edge_zero`; 0 calls measured), allocation-identical (RV89 u4_g7_03 §4)". Optionally, let deletion-only hunks take their enclosing fn from the old side |

## Execution record

- **Who.** RV89, TASK (Type 2) under ROOT. No descendants.
- **When.** 2026-10-04, about 16:33–16:50 MDT, within the 1.5-hour box.
- **Memory guard.** `memguard.sh` PID 5387 was running, and every cargo job checked it.
- **Cargo.** The default toolchain, `--locked --offline`, **`CARGO_BUILD_JOBS=2`** (kept modest while I61 runs gates), `RUST_TEST_THREADS=2` (1 for the witnesses, the challenge and the probes), and `TMPDIR` in scratch. One cargo job at a time, with targets in WT/targets/rv89_pr/.
- **Instrumentation.** The counters, the probes, the old-loop and old-order variants, and `92a5a9da1c`'s `source_blocks.rs` were in my copy only, for their runs. `source_blocks.rs` and `lib.rs` were restored from `6b9bb19a5f` with `git archive` and checked equal to it.
- **Not run.** No Git writes or index operations (Git reads used `GIT_OPTIONAL_LOCKS=0`), no installs, no native, solver-at-scale or DEC-025 jobs, and nothing in the system temp directory. I65's scratch (`pass_pr`) and records were only read.
- **Writes.** Only this folder, WT/rv89_pr/, WT/targets/rv89_pr/ and WT/scratch/rv89_u4_g7_01/pr/. Machine paths in the evidence are replaced by `WT` and `R`.
- **Copies.** WT/rv89_pr and WT/targets/rv89_pr are deleted after this report.
- **Evidence** (`evidence/`):
  - `pr_run.txt`: the run comparison, the entry and outcomes gates, and the delta inventory on `6b9bb19a5f` and `92a5a9da1c`;
  - `source_blocks_probes.txt`: PR1080 and `integer`, as committed and as their parents, with the counting sweeps;
  - `outcomes/`;
  - `scripts/`.
