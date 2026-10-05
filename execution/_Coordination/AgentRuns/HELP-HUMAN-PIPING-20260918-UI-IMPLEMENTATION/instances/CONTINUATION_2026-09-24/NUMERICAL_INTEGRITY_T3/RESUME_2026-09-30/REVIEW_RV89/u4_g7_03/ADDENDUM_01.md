# RV89 G9a, addendum 01: the frozen-head Pass B (F = `20dd3d929d`)

**Reviewer:** RV89, TASK (Type 2), dispatched directly by ROOT. No descendants. This follows this folder's REVIEW.md, which confirmed `6b9bb19a5f` plus `92a5a9da1c`'s `integer` hunk.

**Candidate:**
- I65's frozen-head run, `R/I65/u4_g7_06/` (RETURN.md `b4022b8b…`; `runs/frozen/`), on PR #1082's frozen head **F = `20dd3d929d2a8b6e51671023b2f1eaa74f364a88`**.
- Its 11-entry `delta_reviewed.json`.

**Method:** reading, and Pass B's own gate checks. **No cargo build**, as ROOT asked.
- My `git archive` extract of F's projects/chirality-piping (without `execution/`) matches the tree file for file: 2,950 of 2,950, with nothing extra.
- I ran `delta_inventory2.py`, `g7_linemap.py`, `statics_list.py` and the `entry`, `premise` and `forms` gates on that extract.
- The gates that need a build (`law`, `text`, `noncand`, `controls`, `outcomes`) I ran on I65's recorded outputs. Those outputs were written by the run itself, 17:31–17:35 MDT (the N-4 clearing). I also compared them with my own registered build of `6b9bb19a5f`.

## Verdict: **PASS**

| | Count |
|---|---|
| BLOCKING | 0 |
| SHOULD-FIX | 0 |
| NOTE | 0 |

**No registered build of F is needed.** Since my last full confirmation, the source changed only by comments and docs, one non-qualification test file and CI. I65's build of F gives outcomes, a record, witnesses and challenge peaks identical to my own build of `6b9bb19a5f`.

## 1. The fingerprints match my own diff of `ba1faa1c..20dd3d929d`

My run of `delta_inventory2.py` (u4_g7_06, unchanged since u4_g7_05) on my extract, with I65's `pass_frozen` edges, gives **35 files and 86 rows**. They are **row-for-row equal to I65's `runs/frozen/delta_inventory.json`** in file, lines, class and fingerprint.

- **With an empty reviewed table it stops (5), naming exactly the 11 hunks that need an entry.** With I65's table it passes. I65's `reviewed_match.json` shows 11 of 11 matched.
- **The 11 hunks and their entries:**
  - **my N-1 deletion half,** `source_blocks.rs` after :55, fingerprint **`2718103130462590a4379a0c336fc369cb0267826de894b3cce2040241f2e585`**, as I gave it;
  - the three S-1 doc hunks: `retained_memory_law_tests.rs:2–4`, `retained_memory_witness_tests.rs:6` and `tests/retained_memory_challenge.rs:6–10` (`qualification-test`, "doc comment only");
  - `retained_memory_law_tests.rs:1238`;
  - grant 2's three `cfg-test-stmt` hooks;
  - the U7 flag (`retained_precision.rs:4267–4269`);
  - the two `lib.rs` `#[doc]` lines (:2235, :2254).

## 2. The new hunks are classified correctly

I read every hunk of `92a5a9da1c..F` in the D1 crates. Its three commits are `6d8f8a82b2`, `35d8ae59a7` and the freeze:

| Hunk | Content (read) | Class |
|---|---|---|
| `lib.rs:2176` | the `///` doc of `RetainedPreviewOutput` | `no-code` |
| `retained_memory.rs:2757` | the `///` doc of `RetainedHeadlessContext` | `no-code` |
| `retained_wire.rs:6–7`, `:14` | the `//!` module doc and a `//` comment; `#![allow(dead_code)]` is unchanged | `no-code` |
| `retained_memory_law_tests.rs:2–4` | the `//!` module doc | `qualification-test`, reviewed: doc only. Confirmed |
| `retained_memory_witness_tests.rs:6` | one `//!` line | `qualification-test`, reviewed: doc only. Confirmed |
| `tests/retained_memory_challenge.rs:6–10` | five `//!` lines. The bound constants that `challenge_bounds_are_the_profile` reads are unchanged, and that test passes in I65's law run (42/0) | `qualification-test`, reviewed: doc only. Confirmed |
| `retained_wire_tests.rs` (`6d8f8a82b2`) | the ordinary-bytes pin is asserted only when `cfg!(all(target_arch = "aarch64", target_os = "macos"))`. That is the registered target, so the pin is unchanged where it matters; the equalities still run everywhere. Not a qualification test | `test` |
| `tools/ci/e2e_plan.py`, `tests/test_ci_e2e_plan.py` | CI policy in Python | `not-d1` |

- **result_export is unchanged between `92a5a9da1c` and F** (`git diff --quiet`). So the `integer` and PR1080 hunks are exactly the ones I measured, and stay `unreachable` (`edge_zero`).
- **The line map** `ba1faa1c..F` exits 0, with 0 rules moved and 0 unmapped.
- **The three premise pins** are as reviewed (premise gate 0).
- **No statics** are added or removed.
- **FORMS** equals the regeneration from G7's tree.

## 3. The exit-6 delta is exactly the six tests

- **I65's frozen PP outcomes:**
  - against Pass A's reference, the outcomes gate gives 6, with exactly six added `ok` lines: the five `retained_facade_tests::u3g2_*` and D-U6-5's carrier test;
  - against **my own registered `6b9bb19a5f` run**, they are **identical**: PP 705 passed, 1 failed (t13), 10 ignored, so `u1_ordinary_bytes_unchanged_under_capture` still passes on the registered target.
- **runner/headless:** outcomes gate 0, and identical to my run.
- **Witnesses:** 9 of 9, with output lines identical to my run.
- **The challenge** passes, with peaks 3,541,898 / 2,252,863 B.
- **The law log** gives 42 passed, 0 failed, and its printed record is identical to my `6b9bb19a5f` record. **The maxima are 0.8881 / 0.8929 M, unchanged.**
- **On I65's outputs:** TEXT gate 0 (D 14,734, identical to Pass A), §11 gate 0 (410), controls gate 0 (12/12).
- **I65's verdict** is `DELTAS TO READ exit=6`, and its only non-zero gate is `pp_outcomes:6`.

## 4. The entry is unchanged

- **The entry gate** on F's `retained_memory.rs` against `git show 0c7827b6ad`: equal, threshold `4_026_531_840`.
- **The law gate** on I65's frozen law log: the compiled identity, all 14 reviewed inputs and the layouts equal the entry, 0 failed, and the registered tests ran.

## Execution record

- **Who.** RV89, TASK (Type 2) under ROOT. No descendants.
- **When.** 2026-10-04, about 17:37–17:45 MDT, within the 45-minute box.
- **No cargo build.** I ran no cargo, no native, solver-at-scale or DEC-025 job, and no install.
- **Git.** Git reads, `git archive` and `git show` only, with `GIT_OPTIONAL_LOCKS=0`. Nothing was written to the system temp directory.
- **Other agents' files.** I65's scratch (`pass_frozen`, its logs) and records were only read.
- **Writes.** This addendum, `evidence/addendum_01/` (`gate_checks.txt`, `delta_inventory.txt`, `delta_inventory_rv89.json`) and SHA256SUMS. Also the extract WT/rv89_pr/frozen, deleted after this addendum, and WT/scratch/rv89_u4_g7_01/frozen/. Machine paths in the evidence are replaced by `WT` and `R`.
