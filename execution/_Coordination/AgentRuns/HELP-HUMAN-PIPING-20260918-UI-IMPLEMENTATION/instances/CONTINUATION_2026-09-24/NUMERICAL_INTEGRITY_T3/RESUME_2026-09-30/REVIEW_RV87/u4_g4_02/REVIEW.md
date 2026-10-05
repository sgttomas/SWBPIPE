# RV87 confirmation: G4 S-2 (and S-1, N-1, N-2) in U4 G5 part 2

**Reviewer:** RV87, TASK (Type 2), by ROOT's direction. No descendants.
**Candidate:** `R/I65/u4_g5_01/part2/`. Its 63-entry SHA256SUMS verifies OK.
- The text run (`_run_records/text_p2/`) is at NUM `1e323058f3`.
- The generated profile is in `PP/retained_memory.rs` at `cba3e9fda7`, on the memory branch.
- Source was read from a `git archive` of `1e323058f3` and a `git show` of `cba3e9fda7`.

## Verdict: **NOT CONFIRMED** (S-2 only)

| | Count |
|---|---|
| BLOCKING | 0 |
| SHOULD-FIX | 1 |
| NOTE | 4 |

| Item | Result |
|---|---|
| **1. The 16 sites are exactly the result-id copies at risk** | **No.**<br>– **The 16 are real.** Each copies a `ResultItem` id, and each is now priced at the 1,024-B class: clones at 1,024, formats at +896.<br>– **The set is incomplete.** 18 more result-id copies are still priced at 128 B, or carry a 128-B id inside a format: 8 in the 1,024-B class and 10 bounded by their own templates (SF-1).<br>– **The 17th changed row,** `lib.rs:13322`, has multiplicity 0 |
| **2. +71.9 MB is the right delta** | **Yes, for the 16 sites.**<br>– The row-by-row diff against the rebase run gives exactly +71,882,496 B, plus carry 3's +16,080 B.<br>– The branch totals are TAV_W +53,136,336 B and TAV_X +49,158,096 B.<br>– **It is not S-2's full delta.** The 18 residual sites add up to +32,373,120 B to the whole TAV: up to +5,858,688 B on W, and about 32–36 MB on X |
| **3. S-1, N-1(a)–(e) and N-2 are in the generated profile** | **Yes.**<br>– RV87's own G4 model, run at the part-2 text atoms, reproduces the tree's T16 P1–P4, STAGED, NOTICE, NOTICE_moving, BODY, SUCC and T17 V2/V4 forms on every stride atom.<br>– The only differences are constants of at most 745 B. They come from RV87's exact selected-diagnostic strings; the tree's classes are larger (N-3).<br>– V4 also carries the N-2 walkers, matching their stated bound exactly.<br>– **All 47 generated Rust `FORMS` equal `profile_tree.json`** |

**What it does to the maximum.**
- The in-build W3 maximum (dense) rises to **at most 3,586,398,906 B = 0.8907 M**, still 37,479,750 B under 0.9 M. Sparse is ≤ 0.8858 M.
- X1 stays near 0.83 M.
- **No ruling moves.**

## Findings

| ID | Sev | Where (at `1e323058f3`) | Evidence | Remedy |
|---|---|---|---|---|
| **SF-1** | SHOULD-FIX | `_run_records/text_args.g4.json`: the result-id rule `^(row\|result)\.id$`, plus `result_ref\|^r\.id$`. Sites are listed below | **S-2 is incomplete.**<br>– The anchored rule misses result-id copies whose receiver is not literally `row` or `result`.<br>– It also misses result-id locals and parameters that are bounded by their own (longer) format templates.<br>– **Residual:** +32,373,120 B (whole TAV, at the run's `mult × max(8, 2·size)` convention). Of that, up to +5,858,688 B is on W | Extend the rule, or add `site_size` rows, for the 18 sites. Rerun TEXT and regenerate the profile record, which `profile_in_build_record` pins |
| N-1 | NOTE | `PP/retained_product.rs:280`, `:282`, `:287`, `:290`; `PP/lib.rs:1916` | **A sibling class, diagnostic-id copies, is also priced low.** `record.id.clone()` and `d.id.clone()` copy diagnostic ids (≤ L_DIAGID = 2,330 B) at the 128-B class. Multiplicity is 1–4, so this is ≤ 40 KB. The entity-ref copies of existing rows (`lib.rs:2816`, `PP/preview_physics.rs:690`) are multiplicity 1 | Fold in with SF-1 |
| N-2 | NOTE | `text_budget.py` pricing | **Why 71.9 MB, not RV87's 9.75 MB.**<br>– The run prices every site, exact clones included, at 2 × size.<br>– RV87's G4 estimate used exact capacity for six sites. At the run's convention those six are 22,740,480 B.<br>– The other 10 sites add 49,142,016 B.<br>– Total: 71,882,496 B. Both figures are consistent | None |
| N-3 | NOTE | `profile_tree.json` forms T16_P1–P4, SUCC and T17_V2_hash | **Small constant differences in the tree, none at the maximum.**<br>– The tree keeps the G4 classes for the selected diagnostic (code 26, message 300, keys 6 × 16): up to 745 B above RV87's exact sizes.<br>– T16 P3 and P4 do not carry the 64-B `publication`/`receipt` hex Strings, which are under 128 B. P2 is the maximum, so this does not matter | None needed |
| N-4 | NOTE | `text_p2/sens_p2.summary.json` against the rebase run | **TAV_X counts some ordinary-run sites twice, so whole-run X estimates are lower bounds.**<br>– TAV_X's S-2 delta (49,158,096 B) exceeds the whole-run delta minus the W1-only `retained_product.rs` sites (41,577,936 B) by 7,580,160 B. That is two multiplicity-2,115 sites counted twice.<br>– So RV87's X residual is a lower bound (about 32–36 MB); X1 keeps about 280 MB of margin.<br>– W's delta reconciles exactly: whole minus `rows.rs` = 53,136,336 B | None |

### SF-1: the 18 residual sites

**Class A/B: `ResultItem` ids copied under other names, 1,024-B class** (+29,668,352 B):

| Site | Mult | Copy | Δ | Branch |
|---|---|---|---|---|
| `PP/lib.rs:5592` | 1,760 | `matched[0].id.clone()`, where `matched: Vec<&ResultItem>` (`source_row_bindings`, called beside `qualify_source_case_rows` at `:5489–5491`) | 3,153,920 | W, X |
| `PP/source_receipt/rows.rs:390` | 96 | `primary[&s.functional_indices[c]].id.clone()`, where `primary: &BTreeMap<usize, ResultItem>` (`:332`) | 172,032 | X |
| `…/rows.rs:533` | 960 | `vec![primary[&function].id.clone()]` | 1,720,320 | X |
| `…/rows.rs:308` | 3,875 | `format!("row semantic/value mismatch: {}", actual.id)` | 6,944,000 | X |
| `…/rows.rs:625` | 2,115 | `format!("required derived row missing: {}", d.row.id)` | 3,790,080 | X |
| `…/rows.rs:590`, `:592` | 1,760 each | `binding.result_id.clone()`, the `FunctionalRowBinding.result_id` built at `lib.rs:5592` | 3,153,920 each | X |
| `PP/source_receipt.rs:1016` | 4,230 | `r.result_id.clone()`, the `RowTreatment.result_id` (`finalize_for`, `:867`) | 7,580,160 | X |

**Class C: result-id locals and parameters, bounded by their own templates** (+2,704,768 B, both branches):

| Site | Mult | Copy | Template bound | Δ |
|---|---|---|---|---|
| `lib.rs:13044` | 1,344 | `id: id.to_string()` (`append_endpoint_stress_result`) | ids from `:12881`, up to 872 B | 1,999,872 |
| `lib.rs:13074` | 1,536 | `id: id.to_string()` (station stress) | ≤ 292 | 503,808 |
| `lib.rs:11617` | 768 | `id: id.to_string()` (endpoint force) | ≤ 158 | 46,080 |
| `lib.rs:11649` | 1,152 | `id: id.to_string()` (station force) | ≤ 173 | 103,680 |
| `lib.rs:4624`, `:5276`, `:5281`, `:5314`, `:5329`, `:5359` | 32 each | `result_id.clone()` | 140–322 | 51,328 together |

**How the set was found.**
- RV87 scanned every positive-multiplicity site under 1,024 B whose source line names an id or a result-ref field: 435 lines.
- Each candidate receiver was classified by reading its type.
- Model, node, support, load, material and case ids were excluded. They are in the 128-B class.

**The 10 sites I65 added are genuine `ResultItem` ids:**
- `retained_product.rs:2239` and `:2434` (`row: &ResultItem`, `validate_final_metadata`);
- `rows.rs:294`, `:522`, `:608`, `:610`, `:631` and `:633`;
- `preview_physics.rs:517` (`row = maximum_row(..)`) and `:690`.

## Execution

**Who.** RV87, TASK under ROOT.

**Memory guard.** PID 5387 was running at start and at seal.

**Git reads only, all with `GIT_OPTIONAL_LOCKS=0`:** `rev-parse`, `log`, `diff --stat`, `archive`, `show`.

**Not run.** No Cargo, install, new tooling, solver, native or DEC-025 job.

**Writes.**
- `R/REVIEW_RV87/u4_g4_02/`: this file, SHA256SUMS and `_run_records/`.
- `WT/scratch/rv87_u4_g4_02/`: the source snapshot and scan scratch.

**Commands** (from `_run_records/`):
- `python3 rv87_s2_confirm.py <part2 _run_records/text_p2>`;
- `python3 rv87_profile_check.py <R/REVIEW_RV87/u4_g4_01/_run_records/rv87_g4.py> <part2 _run_records> <retained_memory.rs at cba3e9fda7>`.
