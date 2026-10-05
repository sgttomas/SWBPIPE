# I65 U4 G7: Pass B hygiene, RV89 ADDENDUM_01 N-4 and N-5 (u4_g7_03)

**Basis.** ROOT's message after RR "RV93 on U3 grant 2: PASS; RV89 confirms the final basis re-qualified; U7 planned and ruled" (NUM `4d6fa4e6f9`). This is records tooling only; G7 changed no source. The tools are in `_run_records/`, copied from u4_g7_02 and changed as follows.

## N-4: a reused tag can pass on stale outputs (`g7_pass.sh`)

- **Clearing.** At the start, `pass_<tag>/` is removed whole, and so are this tag's cargo logs and outcome lists, by exact name. The tag must match `[A-Za-z0-9_]+`.
- **New gates,** each non-zero → 6:
  - `text_run`, the TEXT chain's exit;
  - `noncand_run`, the §11 run's exit;
  - `controls_run`, the controls run's exit.
- **The delta tool.** A crash (for example, missing edges) is now 6. With no inventory, the pass stops there.

**Controls** (`pass_b_controls_n4.sh` → `runs/n4_controls/`). Each runs a *mutated copy* of the script: one step is forced to fail, the item 5 cargo stage is cut, and `pass_<tag>/` is pre-filled with the previous final run's outputs plus a sentinel, as a reused tag leaves them.

| Mutated copy | Result |
|---|---|
| New script, TEXT chain fails | exit 6, `text_run:6` |
| New script, §11 run fails | exit 6, `noncand_run:6` |
| New script, controls run fails | exit 6, `controls_run:6` |
| u4_g7_02's script, TEXT chain fails | **`VERDICT PASS exit=0`**: it read the stale outputs, and the sentinel survives |

In every new-script run the sentinel is gone. The last row shows N-4 was a real bug before the fix. 11 of 11 controls pass.

## N-5: test-file changes never stop the pass (`delta_inventory2.py`)

**New class `qualification-test`** for every hunk in:
- `retained_memory_law_tests.rs`;
- `retained_memory_witness_tests.rs`;
- `tests/retained_memory_challenge.rs`.

Each needs a reviewed entry. An unreviewed one alone is **exit 6**, a delta to read, and the pass continues. An unreviewed `live`, `item` or `cfg-test-stmt` hunk is still exit 5.

**Control** (`runs/n5_control/`): the real G6 repair delta, `2bb81ec1ea` → `b43378d90a`, which has 8 hunks in the law and challenge files.

| Reviewed table | Exit |
|---|---|
| empty | 5 (one non-test hunk is unreviewed) |
| every non-qualification hunk | **6** (only the 8 qualification-test hunks are unreviewed) |
| everything | 0 |
| u4_g7_02's tool, with the non-qualification table | 0 (before the fix) |

In this control, reachability comes from the final basis's graph, so some `retained_memory.rs` functions are mislabelled as off the D1 graph. Only the qualification-test class is under test.

## The self-test on the final basis `7f07a2f7b4` (`runs/selftest_final/`)

**Run with the reused tag `final`: `VERDICT DELTAS TO READ exit=6`, as before.**
- The only non-zero gate is `pp_outcomes`. It shows exactly the six added tests, all `ok`: grant 2's five `u3g2_*` and D-U6-5's carrier test.
- Every other gate is 0, including the new `text_run`, `noncand_run` and `controls_run`. The delta tool finds no qualification-test hunk at this basis.

**A slip during this work:** my first self-test run cleared its own stdout log. The early glob `pass_<tag>_*` matched it. The run's result was the same exit 6 (its VERDICT.txt survived). I narrowed the clearing to exact names and re-ran; the re-run is the recorded one.

**The earlier controls,** with the u4_g7_03 tools: 33 of 33 (`runs/pass_b_controls.out.txt`).

## Execution

- I65, TASK, no descendants; 2026-10-04.
- Memguard PID 5387 was running; one cargo job at a time; `--locked --offline`.
- No Git writes (`GIT_OPTIONAL_LOCKS=0`), and no source changes.
- **Writes:**
  - this folder;
  - WT/scratch/i65_u4_g7_01/: `pass_final`, `pass_n4_*`, `ctl3`, `ctl_n4`, `ctlB3` and logs;
  - WT/targets/i65_g7/.
- Placeholder paths only. `SHA256SUMS` covers this folder.
