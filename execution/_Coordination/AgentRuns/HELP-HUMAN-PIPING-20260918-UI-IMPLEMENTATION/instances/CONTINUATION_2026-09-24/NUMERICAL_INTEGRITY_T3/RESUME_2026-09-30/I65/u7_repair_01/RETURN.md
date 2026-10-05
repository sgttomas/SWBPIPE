# I65: the U9 decision 6 comment repair in `retained_memory.rs` (u7_repair_01)

**Basis:** RR "U9 planned and ruled: the D1 milestone PR" (NUM `13ed75d2ae`), decision 6, and ROOT's request.

**Where:** WT/f2a-u7 at `ffe65ef203`. The work is **uncommitted**. Only `projects/chirality-piping/core/product_physics/src/retained_memory.rs` is touched; I61's and I66's files were not.

## The change: comment-only, line-neutral (`_run_records/comment.diff`)

**Lines 6–8** (the module header). Before, it said no production profile was registered and no permit could be constructed. It now reads:
> One production profile is registered (`REGISTERED_PROFILES`: the dev/test identity, M = 4,026,531,840 B under D-7; G6/G7). Every other build is Stale and keeps the ordinary route; a refusal keeps every fact and the first failing clause.

**Line 2742** (the `ProfileStatus` doc; added at ROOT's acceptance). It now reads:
> `Missing` when no profile is registered (none since G6);

Before, it said "`Missing` while no profile is registered (until G6)".

**Line 2847** (`admission`). Before, it said "with none registered (until G6), never". It now reads:
> …the whole law selected: the one registered dev/test profile; any other build is Stale.

## Checks

| Check | Result |
|---|---|
| Line count | 3,076 before and after; the edits replace lines 6–8, 2742 and 2847 one for one |
| PP compiles | On an isolated copy (`git archive ffe65ef203` of core, fixtures and schemas, plus the edited file, so the concurrent edits are excluded):<br>– `cargo build --lib` finishes;<br>– `cargo test --lib retained_memory` gives 42 passed, 0 failed, 9 ignored, including the source-reading law tests (`compile.txt`).<br>All checks were re-run after the line 2742 edit, with the same results |
| Rule keys | Pass B's line map (u4_g7_04 tools, `ba1faa1c` → `ffe65ef203`, exit 0) leaves two rule keys in this file, `:91:next` and `:83`, both callgraph rules. **Neither is on an edited line** (6–8, 2742, 2847). The premise pins are in `retained_precision.rs`, which is untouched (`rule_keys_check.json`) |
| FORMS | The GENERATED PROFILE block (`:1051–2300`) is untouched. The FORMS gate (the file equals its regeneration from G7's `profile_tree.json`) gives code 0 (`forms_gate.json`) |

## Execution

- I65, TASK, no descendants; 2026-10-04.
- Memguard PID 5387 was running; one cargo job; `--locked --offline`; targets in WT/targets/i65_u7_repair.
- No Git writes. Scratch is in WT/scratch/i65_u7_repair_01.
- Placeholder paths only. `SHA256SUMS` covers this folder.
