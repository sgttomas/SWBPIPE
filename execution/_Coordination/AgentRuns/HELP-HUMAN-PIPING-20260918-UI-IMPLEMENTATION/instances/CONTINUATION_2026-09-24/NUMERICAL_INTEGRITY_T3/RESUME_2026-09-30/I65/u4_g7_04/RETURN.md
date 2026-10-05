# I65 U4 G7: Pass B on the U7 head (slice Q), u4_g7_04

**Basis:** U7 head `cfda60403f1047fea9bb6500ff789efb4bf02e1d` (tree `b8296c5b…`), the read-only extract at WT/scratch/i65_u4_g7_01/u7/. The request was ROOT's "U7 slice Q", citing RR "U7 slice F part 1 returned…", point 1.

**Tools:** u4_g7_03's, unchanged except for the `REC` path (`runs/tools_diff_vs_u4_g7_03.txt`) and three new entries in the reviewed table, `delta_reviewed.json` (now 6 entries).

## Verdict: `DELTAS TO READ`, exit 6. The one delta is the same six PP tests as at the final basis

The run is `runs/u7/`.

| Gate | Code | Result |
|---|---|---|
| tree | 0 | 2,950 of 2,950 blobs equal `cfda60403f` |
| entry | 0 | **byte-identical to `0c7827b6ad`'s:** identity, inputs, layouts, threshold 4_026_531_840 |
| law | 0 | identity, inputs and layouts equal; 42 passed, 0 failed; the registered tests ran |
| statics | 0 | none added or removed |
| linemap | 0 | 0 rules moved; every hunk is line-neutral |
| premise | 0 | `:4252`, `:4253` and `:4305` are untouched |
| text_run | 0 | the TEXT chain ran |
| delta | 0 | 26 files, 38 rows; every one classified, and every one that needs an entry is reviewed |
| text | 0 | complete; D 14,734; every row and output identical to Pass A |
| forms | 0 | equal to regeneration from G7's tree |
| noncand_run / noncand | 0 / 0 | the 410 |
| controls_run / controls | 0 / 0 | 12 of 12 |
| **pp_outcomes** | **6** | **+6 tests, all `ok`:** grant 2's five `retained_facade_tests::u3g2_*` and D-U6-5's carrier test, the same six as at `7f07a2f7b4` |
| runner_outcomes | 0 | 85 passed, 2 failed, identical |
| witnesses / challenge | 0 / 0 | 9 of 9, with outcomes identical to the final basis / peaks 3,541,898 / 2,252,863 B |

**Against the final basis `7f07a2f7b4`** (`runs/u7/vs_final_basis.txt`), these are identical:
- the PP and runner/headless outcome lists;
- the witness outcomes.

`retained_wire_tests::u1_milestone_successor_both_modes` passes, with its assertion flipped to "eligible with the actual invocation (U7)". PP is 705 passed, 1 failed (t13), 10 ignored.

**The maxima are unchanged:** 0.8881 M sparse and 0.8929 M dense.

## The deltas read (`runs/u7/delta_inventory.json`)

**From Pass A's basis `ba1faa1c`:**
- **U7's own production-file hunks, all line-neutral:**

| Hunk | Tool class | Status |
|---|---|---|
| `retained_precision.rs:4267–4269` | `item` (the const) | **reviewed as D1-live** |
| `lib.rs:2235`, `:2254` | `item` (an inline `#[doc]` on an unchanged signature) | **reviewed as doc only** |
| `lib.rs:3156` | `no-code` | automatic |
| `semantic_contract.rs:581–582` | `no-code` | automatic |

- **Unchanged from the final basis:** grant 2's three `cfg-test-stmt` hunks (reviewed), 11 `test` hunks, and the `generated` T17_V4 line.
- **`retained_wire_tests.rs`:** `test`. It is not one of the three qualification-test files.
- **The other 18 files are `not-d1`:** 10 TypeScript, 2 `core/analysis_runs` Python, 2 `result_export/tests/*.rs`, 2 fixtures (neither embedded by D1 code), and 2 Python tests.

**The first run** (`runs/u7a/`) stopped as designed: exit 5, on the three unreviewed `item` hunks (the flag const and the two `#[doc]` lines). I entered them as ROOT directed:
- **The flag** is D1-live, on I66's evidence (`R/I66/u7_slice_f_01/RETURN.md` §3). I re-read it at `cfda60403f`:
  - the single token `false` → `true` on `:4269`;
  - the constant is read only at `:4309` (grep);
  - every newly evaluated operation only borrows: `is_some`, `Index<&str>`, `==` on `&str`, the `list` slice, `text` → `&str`, `matches!`;
  - I66's counting allocator gives identical counts and bytes in 12 pairs;
  - PP's precommit uses only `Ok`/`Err`;
  - no rule key or pin falls on an edited line.
- **`lib.rs:2235` and `:2254`** are doc only, slice P's inline `#[doc = "…"]` attributes.

No other hunk needed an entry, and nothing moved.

**A count note:** ROOT's message says 24 files. `git diff --stat 7f07a2f7b4 cfda60403f` outside `execution/` gives 22; the tool's 26 is measured from `ba1faa1c` and includes grant 2's files.

## Execution

- I65, TASK, no descendants; 2026-10-04.
- Memguard PID 5387 was running; one cargo job at a time; `--locked --offline`.
- No Git writes, and no source changes.
- **Writes:**
  - this folder;
  - WT/scratch/i65_u4_g7_01/: `pass_u7a`, `pass_u7` and logs;
  - WT/targets/i65_g7/.
- Placeholder paths only. `SHA256SUMS` covers this folder.
