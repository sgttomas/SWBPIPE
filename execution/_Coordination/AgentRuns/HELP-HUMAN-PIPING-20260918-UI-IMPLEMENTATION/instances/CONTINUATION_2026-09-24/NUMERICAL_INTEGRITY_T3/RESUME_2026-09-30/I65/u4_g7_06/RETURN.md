# I65 U4: Pass B on PR #1082's frozen head F (u4_g7_06)

**Basis:** **F = `20dd3d929d2a8b6e51671023b2f1eaa74f364a88`** (tree `3754502b…`). I extracted it read-only with `git archive` (`GIT_OPTIONAL_LOCKS=0`) to WT/scratch/i65_u9_frozen/F/. It equals the tree blob for blob: 2,950 of 2,950.

**Tools:** the prepared `u9_repair_01/_run_records/pass_b/`, unchanged except for `REC` (`runs/tools_diff_vs_u9_repair_01.txt`), with the same 11 reviewed entries.

## Verdict: `DELTAS TO READ`, exit 6, as expected

The run is `runs/frozen/`.

| Gate | Code | Result |
|---|---|---|
| tree | 0 | 2,950 of 2,950 |
| entry | 0 | **byte-identical to `0c7827b6ad`'s,** threshold M = 4_026_531_840 |
| law | 0 | identity, the 14 reviewed-input hashes and the layouts equal the entry; 42 passed, 0 failed; the registered tests ran |
| statics | 0 | none added or removed |
| linemap | 0 | no rule moved |
| premise | 0 | `:4252`, `:4253` and `:4305` are as reviewed |
| text_run / text | 0 / 0 | complete; D 14,734; every row and output identical to Pass A |
| delta | 0 | 35 files, 86 rows; every one classified, and every one that needs an entry is reviewed |
| forms | 0 | equal to regeneration from G7's tree |
| noncand_run / noncand | 0 / 0 | the 410 |
| controls_run / controls | 0 / 0 | 12 of 12 |
| **pp_outcomes** | **6** | **+6, all `ok`:** grant 2's five `retained_facade_tests::u3g2_*` and D-U6-5's carrier test (the only outcome delta) |
| runner_outcomes | 0 | 85 passed, 2 failed, identical |
| witnesses / challenge | 0 / 0 | 9 of 9 / peaks 3,541,898 / 2,252,863 B |

- **Against the PR-head run** (u4_g7_05 `runs/pr`), these are identical (`vs_pr_head.txt`): the PP and runner/headless outcome lists, and the witness outcomes.
- PP is 705 passed, 1 failed (t13), 10 ignored.
- **The maxima are unchanged:** 0.8881 M sparse and 0.8929 M dense.

## The reviewed entries: 11 of 11 matched (`runs/frozen/reviewed_match.json`)

| Entry | Matched row |
|---|---|
| RV89 N-1 (`27181031…e585`) | `source_blocks.rs` "after 55" (`item`), the deletion half of `fn integer`'s reorder; its re-added `:57` and PR1080's `:1055–1058` are `unreachable` by themselves |
| The three S-1 qualification-test doc hunks | `retained_memory_law_tests.rs:2–4` (`9b81ac2e…`), `retained_memory_witness_tests.rs:6` (`4198b681…`), `tests/retained_memory_challenge.rs:6–10` (`eb06a586…`) |
| `:1238` | `71e1fff2…` |
| The rest | grant 2's three `cfg-test-stmt`, the U7 flag (D1-live, I66's evidence), and `lib.rs:2235`/`:2254` (doc only) |

**Other classifications:**
- **The other S-1 edits** (`lib.rs:2176`, `retained_memory.rs:2757`, `retained_wire.rs:6–7` and `:14`) class `no-code` by themselves.
- **`6d8f8a82b2`'s `retained_wire_tests.rs`** classes `test` ("a test file").
- **Two files are new against the PR-head run,** both `not-d1`: `tools/ci/e2e_plan.py` and `tests/test_ci_e2e_plan.py`, from main's CI input pin. That makes 22 `not-d1` in all.

## Execution

- I65, TASK, no descendants; 2026-10-04.
- Memguard PID 5387 was running. My cargo jobs were strictly sequential, and no other cargo process was running when the pass started. `--locked --offline`.
- No Git writes, and no source changes.
- **Writes:**
  - this folder;
  - WT/scratch/i65_u9_frozen/;
  - WT/scratch/i65_u4_g7_01/pass_frozen;
  - WT/targets/i65_g7/.
- Placeholder paths only. `SHA256SUMS` covers this folder.
