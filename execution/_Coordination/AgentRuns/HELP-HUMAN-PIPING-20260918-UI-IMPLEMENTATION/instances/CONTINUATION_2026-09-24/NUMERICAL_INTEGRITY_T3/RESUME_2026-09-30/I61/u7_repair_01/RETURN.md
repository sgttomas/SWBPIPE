# I61 RETURN: U9 decision 6, the stale "no permit" comments (PP, comment-only)

**Done:** five comment lines in two PP files now state the registered state truthfully. Every change is comment-only and line-neutral, and both files keep their line counts:
- `lib.rs`: 24,333 lines;
- `retained_facade_tests.rs`: 827 lines.

PP compiles (`cargo test --lib --no-run`), and no Pass B rule key falls on an edited line. The work is **uncommitted** in WT/f2a-u7, on top of `ffe65ef203`.

**Who:** I61 (TASK, Type 2), dispatched directly by ROOT. **When:** 2026-10-04, 20:29Z to 20:36Z.

**What was not touched:**
- `retained_memory.rs`, I65's, which shows as modified in WT/f2a-u7 from I65's parallel work;
- I66's Python and Rust summary files.

**Host:** no Git writes; Git reads used `GIT_OPTIONAL_LOCKS=0`. The memory guard (PID 5387) was running. There was one Cargo job, `--locked --offline`, `CARGO_BUILD_JOBS=4`, on my own target.

## 1. The edits (before and after)

The paths are under P/core/product_physics/src.

| File:line | Before | After |
|---|---|---|
| `lib.rs:2185` (doc on `RetainedPublication`) | ``/// Without a permit (until U4 G6) it is always `Ordinary`.`` | ``/// Without a permit it is always `Ordinary`, as in every Stale (unregistered) build.`` |
| `lib.rs:2286` (comment in `run_linear_static_preview_value_dispatch`, G-A) | `    // below is unchanged. No permit exists until U4 G5 (decision 7).` | `    // below is unchanged. Only D1 Direct calls in the registered build get a permit.` |
| `lib.rs:2919` (doc on `permitted_dispatch`) | `/// Unreachable until U4 G5 adds a registered profile.` | `/// Only D1 Direct calls in the registered dev/test build reach it (M = 4,026,531,840 B, D-7).` |
| `retained_facade_tests.rs:1` | `//! I61 U3 grants 1 and 1b: the facade's W1 phases through the private driver. No permit` | `//! I61 U3 grants 1 and 1b: the facade's W1 phases through the private driver. A permit` |
| `retained_facade_tests.rs:2` | `//! exists until U4 G5 and decision 7 forbids a test permit, so these tests enter` | `//! needs the registered build, and decision 7 forbids a test permit, so these tests enter` |

**Together the three `lib.rs` lines state the ruled scope:**
- permits go only to in-domain (D1) Direct calls (`:2286`);
- only in the registered dev/test build, with M = 4,026,531,840 B under D-7 (`:2919`);
- every other build is Stale, and its publication stays `Ordinary`, the ordinary route (`:2185`).

Each line is true on its own. Basis: RR "Registration applied; M = 4,026,531,840 B selected under D-7…" (RR:10547–10552) and D1.0's Headless refusal.

**Two departures from the line list:**
1. **`retained_facade_tests.rs:1` is also edited.** The stale sentence ("No permit / exists until U4 G5") starts at the end of line 1, so line 2 cannot be made true alone. Lines 3–4 are unchanged.
2. **`lib.rs:2919` is 94 characters.** The file's other doc lines are at most 90. The ruled text needs both M and D-7, and the line must stay one line to keep the edit line-neutral. Dropping ", D-7" would make it 89, if ROOT prefers.

**Byte identity apart from the edited lines** (`git diff -U0` against `ffe65ef203`): exactly the five hunks above, each replacing one line with one line.
- `lib.rs`: before blob `7de691aeb5`, sha256 `bd74e080…`; after blob `0e483d161e`, sha256 `dd1b852e315f491f61f5c7cf75bf211644a66c133c864aa477d6be0af5e25b63`.
- `retained_facade_tests.rs`: before blob `e7ce92fa88`, sha256 `bf68e7ec…`; after blob `d31d65a77b`, sha256 `14d9ab461ec9ac9193ad3a486a0e59ce5cc69bf602f2eb0597291f2f7e517ae5`.

## 2. Checks

**Line counts:** unchanged, `wc -l` 24,333 and 827, before and after.

**Compilation:**
- **Method.** `git archive ffe65ef203` gave a clean copy of P's core, schemas and fixtures in WT/scratch/i61_u7_repair_01/tree. Only my two edited files were copied in. This isolates my change from I65's and I66's work in progress in the shared worktree; `retained_memory.rs` in the copy equals `ffe65ef203`'s blob (sha256 `5861b83a…`).
- **Command,** run in `core/product_physics`:
  ```
  env -u RUSTFLAGS CARGO_TARGET_DIR=WT/targets/i61-u7r CARGO_BUILD_JOBS=4 \
    cargo test --lib --no-run --locked --offline
  ```
  It ran from 20:33:36Z to 20:33:51Z and **exited 0**, in the default (registered-identity) debug profile.
- **Warnings:** 7 PP lib-test warnings and 1 result_export warning. All are pre-existing dead-code and parenthesis warnings at other lines (`lib.rs:10320`, `retained_memory.rs:1607–1608`, `source_recovery.rs`, `source_blocks.rs:433`). **None names an edited line.**

**Rule keys (reasoned from I65's Pass B rule files at `R/I65/u4_g7_04/_run_records/`):**
- **The keys read:** every `lib.rs:N` key in `chain/callgraph_rules.g4.json`, `chain/loop_bounds.g4.json`, `chain/text_args.g4.json`, `chain/sens.py`, `premise_pins.json` and `delta_reviewed.json`. That is 492 keys that are PP's or ambiguous; 78 keys naming another crate's `lib.rs` were excluded.
- **The mapping:** each key was carried from Pass A's basis (tree `ba1faa1c…`) to `ffe65ef203` with `g7_linemap.py`'s offset rule, over the 13 `lib.rs` hunks.
- **Results:**
  - **no key maps onto `lib.rs` 2185, 2286 or 2919, or within 3 lines of them;**
  - no raw key carries those numbers either;
  - four keys sit inside basis-to-head hunks and are left unmapped (2235, 2254, 2379, 3005). They are slice Q's reviewed doc entries and grant 2's `cfg(test)` fragment lines, already handled by Pass B, and none is near an edit.
- **`retained_facade_tests.rs`** is a `_tests.rs` file. `g7_linemap.py` excludes such files, and no rule file names it.
- Every edit is one line for one line, so no key's offset moves.

**Line-neutral and comment-only.** Pass B should classify the three `lib.rs` hunks as comment-only, as at slice Q, where "the comment-only and test hunks classify on their own".

## 3. Files

- **The edits:** WT/f2a-u7, `P/core/product_physics/src/lib.rs` and `src/retained_facade_tests.rs`. Uncommitted, for ROOT to verify and commit.
- **Scratch:** WT/scratch/i61_u7_repair_01/ (the archive copy and `build.log`); target WT/targets/i61-u7r. Kept, not pruned.
- **This record:** RETURN.md and SHA256SUMS, with placeholders only and no machine paths.
