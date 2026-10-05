# I65: the RV95 S-1 repair (six stale pre-registration texts) and the frozen-head Pass B, prepared (u9_repair_01)

**Basis:**
- RV95's complete PR review (`R/REVIEW_RV95/u9_01/REVIEW.md`, S-1);
- ROOT's assignment, which includes `lib.rs:2176` and `retained_wire.rs` for this comment-only repair;
- RV89 u4_g7_03 N-1.

**Where:** a fresh scratch copy, `git archive d069ab3ccf` of `projects/chirality-piping` without `execution/`. It equals the tree blob for blob (2,950 of 2,950). No Git writes. **ROOT applies the files.**

**Deliverables:**
- `edited/projects/chirality-piping/core/product_physics/…`: the six files;
- `u9_repair.diff`: 86 lines, sha256 `f608cb2b…`.

## The edits: comment and doc only, line-neutral

The new texts are true at the PR head:
- one registered dev/test profile (M = 4,026,531,840 B, D-7);
- permits only for D1 Direct calls in that build;
- every other build Stale, keeping the ordinary route;
- the serializer reached on the permitted path (`retained_w1` → `retained_wire::serialize_frozen`, lib.rs:3144).

| File:lines | Was | Now |
|---|---|---|
| `lib.rs:2176` (public `RetainedPreviewOutput`) | "No production permit exists." | "A production permit exists only for a D1 Direct call in the one registered (dev/test) build; any other build is Stale and keeps the ordinary route." |
| `retained_memory.rs:2757` (public `RetainedHeadlessContext`) | "No production profile exists;" | "Headless is refused at D1.0 (D-2), even in the registered build;" (`:2758`, "these roots only supply facts and cannot authorize…", is unchanged and still true) |
| `retained_wire.rs:6–7` | "Production-unreachable: no public entrypoint calls it; U3 installs it behind the capture permit." | "In production it is reached only behind the capture permit (U3, `retained_w1`): D1 Direct calls in the registered build." |
| `retained_wire.rs:14` | "Production-unreachable until U3 installs it…" | "In production, reached only behind the capture permit (U3): D1 Direct calls in the registered build (G6)." |
| `retained_memory_law_tests.rs:2–4` | "No profile and no permit is constructed here … the registered-identity list stays empty…" | "The tests construct no profile (decision 7): the one registered entry is the production one, a permit comes only from `admit` in the registered build, and a check that needs another matched build passes its inputs to the pure function." |
| `retained_memory_witness_tests.rs:6` | "No permit exists (decision 7), so the private driver" | "The witnesses mint no permit (decision 7): the private driver" |
| `tests/retained_memory_challenge.rs:6–10` | "(no permit exists, so the ordinary span runs …) … each measured peak … at or below … W1 phase" | The span now includes the W1 phases when the registered build permits the call. A permitted run's peak (the milestone, registered build) is bounded by E_mov,max, any other by the in-build W1 phase, as the body does (`MAX_PHASE_BYTES` / `W1_PHASE_BYTES`). |

RV95's "milder" `lib.rs:117` is not in ROOT's list and is untouched. RV95 calls it historical and accurate.

## Checks (`_run_records/`)

| Check | Result |
|---|---|
| Line counts | unchanged in all six files (24,333 / 3,076 / 1,991 / 1,400 / 247 / 134) |
| Comment or doc only, mechanically | `comment_only_check.py`: every differing line is a `//`, `///` or `//!` line on both sides, at the same position; no other line differs (`comment_only.json`) |
| PP compiles | On the edited copy: `cargo build --lib` finishes; `cargo test --lib retained_memory` gives 42 passed, 0 failed (the source-reading law tests included); `cargo test --test retained_memory_challenge` passes (`compile.txt`) |
| Rule keys | Pass B's line map (`ba1faa1c` → `d069ab3ccf`, exit 0). Of 612 rule references whose basename matches an edited file (any crate's `lib.rs` included, conservatively), **none is on an edited line** (`rule_keys_check.json`). The premise pins are as reviewed (`premise_gate.json`) |
| FORMS | The generated block is untouched; the FORMS gate gives code 0 on the edited `retained_memory.rs` (`forms_gate.json`) |

## The frozen-head Pass B, prepared, not run (`_run_records/pass_b/`)

**Tools:** u4_g7_05's, with `REC` pointing here. The two control scripts are carried unchanged and still name their own record folders.

**`delta_reviewed.json` grows from 7 to 11 entries.**
- **RV89 N-1:** fingerprint `2718103130462590a4379a0c336fc369cb0267826de894b3cce2040241f2e585`. It is `source_blocks.rs` `fn integer` (:54), the deletion half of `92a5a9da1c`'s two-line reorder (re-added at :57). Unreachable on D1 (`edge_zero`; 0 calls measured), allocation-identical (RV89 u4_g7_03 §4). The text is RV89's.
- **The S-1 hunks in qualification-test files, as doc comment only,** fingerprinted as the tool computes them, from `ba1faa1c` to the edited files (`qualification_hunks.json`):
  - `retained_memory_law_tests.rs:2–4`: `9b81ac2e…`;
  - `retained_memory_witness_tests.rs:6`: `4198b681…`;
  - `tests/retained_memory_challenge.rs:6–10`: `eb06a586…`.
  
  The same computation reproduces the existing `:1238` entry's fingerprint (`71e1fff2…`), which checks the method.
- **Only valid as applied.** These fingerprints hold only if the hunks land exactly as in `u9_repair.diff`. If ROOT changes the text, the rerun stops with exit 6 on them, failing closed.
- The other S-1 hunks (`lib.rs:2176`, `retained_memory.rs:2757`, `retained_wire.rs`) are comments in production files. The tool classes them `no-code` by itself, with no entry.

**`6d8f8a82b2` (`retained_wire_tests.rs`):** it classifies as `test` ("a test file").

**Preview at NUM's head `d069ab3ccf`, without the S-1 edits** (`delta_preview_d069ab3ccf.*`: `delta_inventory2.py` with this table, reachability from the PR-head run's graph):
- **PASS**, exit 0;
- `source_blocks.rs` "after 55" is the reviewed `item`, and `:57` is `unreachable` (`integer`, reached only through `edge_zero`).

**To run, once ROOT sends the frozen head and extract:** `I65_T=<WT> bash pass_b/g7_pass.sh <extract> <rev> <tag>`.

## Execution

- I65, TASK, no descendants; 2026-10-04.
- Memguard PID 5387 was running, with one cargo job at a time. `--locked --offline`; targets in WT/targets/i65_u9_repair.
- No Git writes: `archive`, `ls-tree`, `diff` and `show` were run as reads with `GIT_OPTIONAL_LOCKS=0`. No file outside the scratch copy and this folder was edited.
- Scratch is in WT/scratch/i65_u9_repair_01.
- Placeholder paths only. `SHA256SUMS` covers this folder.
