# RV95 addendum 01: same-reviewer confirmation on the frozen head F

**Reviewer:** RV95, TASK (Type 2), dispatched directly by ROOT (HELP_HUMAN, Agent 0); ROOT is the return path; no descendants. This confirms my own findings from `REVIEW.md` (sha256 `2918e063…`, unchanged) on the frozen head, and the gate evidence available now.

**Candidate:** **F = `20dd3d929d2a8b6e51671023b2f1eaa74f364a88`**, the PR #1082 head (checked with `gh pr view`), on M `5fdc5ab601`. Since my review head `6d8f8a82b2`:
- `35d8ae59a7` carries S-1 (NUM `bb3d766379`) and N-3 (NUM `c5adc16384`);
- `20dd3d929d` holds the regenerated package (S-2; I61 `R/I61/u9_freeze_01/`, SHA256SUMS 9/9 OK).

Placeholders as in `REVIEW.md`. **Basis:** RR through "Owner decisions at the freeze" (NUM `759f8508bc`).

**Host:**
- A fresh `git archive` of F (P without `execution/`, plus PKG) was placed in `WT/rv95/`; 26,842 of 26,842 blobs verified. Targets went to `WT/targets/rv95/`.
- One cargo job only: PP registered, all targets, 23:31–23:33Z. After it, the Python CI tests and one mutant ran.
- **I ran no build or test after ROOT's host note** (the Mac baseline and DEC-025 run on a quiet host). Everything after it was read-only: records and Git reads with `GIT_OPTIONAL_LOCKS=0`.
- No Git writes. The memory guard (PID 5387) was running and its log has no KILLED line. The copies and targets are deleted.

## Verdict: **CONFIRMED** (S-1, S-2 and N-3 repaired; F's non-execution tree equals `35d8ae59a7`'s; the package states the truth)

| | Count |
|---|---|
| BLOCKING | 0 |
| SHOULD-FIX | 0 |
| New NOTE | 4 (A-1 to A-4; none needs a change to F) |

## 1. S-1: the six stale texts (`35d8ae59a7`)

**CONFIRMED.**

**Each new text is true against the code at F:**
- **`lib.rs:2176`** (public `RetainedPreviewOutput`) says a production permit exists only for a D1 Direct call in the one registered build. That is `admit` with `REGISTERED_PROFILES`; the Headless entry is refused at D1.0 and every other build is Stale.
- **`retained_memory.rs:2757`** (public `RetainedHeadlessContext`) says Headless is refused at D1.0 (D-2) even in the registered build. My earlier registered sweep agrees: 70 of 70 Headless rows give `(Registered, Some("D1.0"))` with exact bytes.
- **`retained_wire.rs:6–7`, `:14`** say the serializer is reached in production only behind the capture permit (`retained_w1`). That is its one non-test call site, `lib.rs` `retained_w1` → `serialize_frozen`.
- **The law, witness and challenge headers** match their bodies:
  - the law tests construct no profile, and `admit_grants_a_permit_for_the_milestone_in_the_registered_build` uses the production entry;
  - the witnesses enter `retained_w1` directly;
  - the challenge's bound is `MAX_PHASE_BYTES` when permitted and `W1_PHASE_BYTES` otherwise, as its new header says.

**The edits are comment- and doc-only and line-neutral:**
- every changed `.rs` line in `6d8f8a82b2..35d8ae59a7` is a `//`, `///` or `//!` line;
- all 7 files keep their line counts (24,333 / 3,076 / 1,400 / 247 / 1,991 / 134 / 495);
- the `retained_memory.rs` edit (`:2757`) lies outside the GENERATED PROFILE block (`:1051–2300`), and `copies/g5_profile.py profile_tree.json` regenerates `retained_memory.rs` byte for byte at F.

**My registered PP run on F** gives 705 ok / 1 failed (t13) / 9 ignored. That is test-by-test identical to my `6d8f8a82b2` run, apart from the absent scratch sweep harness (pristine copy).
- It includes `challenge_bounds_are_the_profile` (which reads the edited challenge file), `retained_direct_peak_is_within_the_profiles_ordinary_span` and `legacy_native_and_typed_call_graphs_cannot_acquire_retained_admission` (whose `lib.rs` section spans the edited `:2176`).
- The live successors are again `ac6986b0…` (sparse) and `6cd1d249…` (dense).
- `lib.rs:117` ("production-unreachable until U3") is left as is; my review called it historically accurate.

## 2. N-3: the policy pin (`35d8ae59a7`)

**CONFIRMED.**
- `test_ci_e2e_plan.py:68` asserts that `ci.NUMERICAL_APP_INPUTS == {DESKTOP + 'src-tauri/src/lib.rs'}`.
- My mutant C2 (the set emptied, `test_ci_e2e_plan.py` run alone) is now **killed**, by `test_numerical_policy_is_independent_and_whole_pr`.
- `test_ci_numerical.py`, `test_ci_e2e_plan.py` and `test_analysis_run_compatibility.py` give 105 passed on F.

## 3. F's tree and S-2: the package

**F's non-execution tree equals `35d8ae59a7`'s.** `git diff --quiet 35d8ae59a7 20dd3d929d -- . ':!P/execution'` is true. F's diff against `35d8ae59a7` is exactly 4 PKG files (`CHANGE_RECORD.md`, `PR_BODY.md`, `SHA256SUMS`, `source_equality.py`).

**Source equality on F**, my own run with `--pr 20dd3d929d --int bb3d766379 --main 5fdc5ab601`: **5/5 PASS**.
- |S| = 139, and 137 paths are identical in blob and mode.
- `compatibility.py` has one conflict, its rule holds, and it equals blob `767da340…`.
- `source_blocks.rs` is a clean merge and equals blob `e8aadb41…`.
- The execution files are exactly the 10 package files, and their SHA256SUMS verify.

**The generalized check 3 is correct.** I read its diff:
- a clean merge must equal the PR's blob;
- a conflict needs a `MERGE_RULES` entry with the exact conflict count and main's side inside the integration side;
- modes are compared.

**My four negative controls all fail as designed** (`evidence/confirm_01/neg_controls.txt`):
1. a stale PR head;
2. a conflict without a rule;
3. a rule whose main-side text is absent;
4. a PR without the `source_blocks.rs` merge.

**Citations on F:** 368 resolved, 0 ambiguous, 0 unresolved, PASS.

**Every claim in the package that I can check holds at F:**

| Claim | Check |
|---|---|
| 139 files, 59 A / 80 M, 11,944,314 B, +204,659 / −897 against M | recomputed: equal |
| Package: 10 files, 192,326 B; SHA256SUMS `c8893d64…` | equal; 9/9 OK |
| 137 identical to NUM `bb3d766379`; two recorded merges | equal (check 2, check 3). `c5adc16384` is an ancestor of `bb3d766379`, which is on `origin/codex/piping-numerical-integrity-20260926` |
| `source_blocks.rs` "in S because of the 32-bit bound; at the cut it was main's version" | true: U does not touch it; the cut's blob is M's |
| The post-cut commits (§3) with their NUM and PR SHAs and their reviews | true. "Five source commits" counts NUM's five; the PR carries them in four commits, as PR_BODY says |
| U1 pin scoped to the registered target (§3; PR_BODY) | true (`ORDINARY_PINNED_TARGET`); stated in both |
| N-4 on the activation checklist; N-5 to T6; N-6 as a re-qualification obligation; N-7 "counts within D1's caps" | present in §4, §5 and PR_BODY |
| The GENERATED PROFILE block regenerates at the source head | regenerated byte for byte at F |
| Gate results in §7 (G5, G6, G7, G8, G9a) | match their records (§4 below) |

**The live PR body** equals `PR_BODY.md` without its title line (apart from one trailing newline). No placeholders remain, and the package has no machine paths.

**S-2 is CONFIRMED.**

## 4. Gate evidence available now

| Gate | Record (seal) | Ran on | Confirmation |
|---|---|---|---|
| **G5, T9** | `R/I61/u9_g5g6_01/` (84/84 OK; RETURN `37f7938d…`) | M; C `6b9bb19a5f`; C2 `92a5a9da1c` | 112/112 identical. +2 milestone outputs equal the ordinary value-route bytes, whose compact sha256 are U1's protected pins `9c7ec1a1…` / `21ca629c…`. Extra corpus 16/16. C2 = C. **Confirmed** |
| **G6, part 1** | same | M; C; C2 | gate_check PASS on each side (764 evaluated, 332 trusted, 0 trusted breaches); 884/884 identical; 0 heap-cap aborts. **Confirmed** |
| **G6, part 2** | same | M; C2 | 24 executions in 45.4–52.8 s (limit 1,800 s); envelopes identical and equal to KF2's. Run 3 had 0 foreign samples. **Confirmed** (see A-4) |
| **G5/G6 carry-over** to `35d8ae59a7` (ROOT's ruling) | RR "U9 G5 (T9) and G6 (both-entry) pass…" | — | **The premise holds.** From C2 to `35d8ae59a7`, the non-test production `.rs` files changed (`lib.rs`, `retained_memory.rs`, `retained_wire.rs`) change 0 non-comment lines and keep their line counts, so panic locations are unchanged too. The other changes are `#[cfg(test)]` modules, an integration test and a Python test, which the release-built T9 harness and probe do not compile. F = `35d8ae59a7` outside execution |
| **G7, src-tauri** | `WT/scratch/u9_g7/` (M.log, C.log, outcomes; `run.sh`) | M; C | M 116 / C 116; the outcome lists are byte-identical (sha256 `4d3df8cf…` both); `diff.txt` empty. **Confirmed on C**; see A-3 for its carry-over to F |
| **G8, controls, sweep, callers** | `R/I61/u9_g8_01/` (14/14 OK; RETURN `90103096…`) | C | Pressure refused at D1.5 / D1.3 with exact bytes and no notice; n05/n06 `Coexistence` exact; 324-output sweep `9a74ff16…` / `0e2db8b8…`; no product caller. My own sweep at C2 and my PHYS-R4 sample at `6d8f8a82b2` agree. **Confirmed** |
| **G9a, full Pass B** | I65 `R/I65/u4_g7_05/` (101/101); RV89 `R/REVIEW_RV89/u4_g7_03/` (15/15) | C | **Confirmed** (unchanged from my review) |
| **Frozen-head Pass B** | I65 `R/I65/u4_g7_06/` (72/72 OK; RETURN `b4022b8b…`) | F (2,950/2,950 blobs) | Exit 6 on the six tests only. Entry byte-identical to `0c7827b6ad`'s. Law 42/0. Maxima 0.8881 / 0.8929 M. The 11 reviewed entries matched 11 of 11; the S-1 non-test hunks class `no-code` and `6d8f8a82b2` classes `test`. **Exists and reads as expected; RV89's confirmation is pending** |
| **GEN-8** | RR "The freeze…" (ROOT: 1 passed on the placement) | F placement | ROOT's; not rerun by me (it needs the full checkout) |

**Pending, for my final read-only confirmation:**
- **Hosted CI on F** (run 37243941694 and siblings). At 23:35Z, selection, harness, pec and the App checks had passed; the numerical cargo suite and the four remainder shards were pending.
- **The full-SHA dispatch.**
- **The Mac baseline and DEC-025.**
- **RV89 on `u4_g7_06`.**
- **The native witness (G10)** is recorded as outstanding on the owner's Mac by the owner's decision (RR "Owner decisions at the freeze"). It will not be run before the merge.

## New NOTEs

| # | Where | Evidence | Remedy |
|---|---|---|---|
| A-1 | PKG `CHANGE_RECORD.md` §1 "Not included"; `PR_BODY.md` "Records committed" | **The records-not-brought count is the plan's.** "5,677 execution-record files (191.4 MB)" is the delta at NUM `fd5032f6cb` (191.4 MiB). At the named integration head `bb3d766379` it is 6,103 files, 213.3 MB (203.4 MiB). The statement that they are not brought stays true. | None for F. Give the count at the merge, with its basis, in the post-merge record. |
| A-2 | PKG `CHANGE_RECORD.md` §3, "the U1 pin" row | **"The successor pins are platform-stable"** rests on two targets: aarch64 macOS (here), and x86_64 Linux as ROOT read run 37240946900. There, `u1_milestone_successor_both_modes`, which is not gated by `registered()`, passed. I did not open that run's artifact. §5's N-6 obligation states the general case correctly. | None for F. In the post-merge record, prefer "identical on the two targets observed". |
| A-3 | `WT/scratch/u9_g7/C/` | **G7's tree was reused after the run.** `C/…/result_export/src/source_blocks.rs` was modified at 22:35:12Z, 13 minutes after the G7 run ended (`C.log` 22:21:59Z), and now holds the `92a5a9da1c` reorder. The logs and outcomes predate the edit and stand for C. G7 ran on C. ROOT's carry-over ruling names G5 and G6, not G7. From C to F, the only non-comment production change in src-tauri's dependency closure is that reorder, which is identical on 64-bit by construction. | Add G7 to the carry-over ruling in the merge record (C → F, by construction). Note that the scratch tree is no longer a pristine copy of C, or re-extract it if it is kept as evidence. |
| A-4 | `R/I61/u9_g5g6_01/` part 2, run 1 | **Disclosure.** I61 records that run 1 of G6 part 2 overlapped "RV95/RV89 cargo test, pytest and vitest". My review's chains were among them. I61 detected it and reran, and run 3 is clean (0 of 73 samples). No result relies on run 1. | None. |

## Records

`evidence/confirm_01/` (placeholder paths only) holds:
- `source_equality_F_vs_bb3d766379.txt` and `.json`;
- `neg_controls.txt` and the two control diffs;
- `check_citations_F.txt`;
- `pp_reg_full_F.outcomes`;
- `live_successors_F_sha256.txt`;
- `mutant_C2_F.json`;
- `py_ci_F.outcomes`;
- `chain4.sh`.

`SHA256SUMS` covers the whole `u9_01` folder; `REVIEW.md` is unchanged. **Cleanup done:** `WT/rv95/` and `WT/targets/rv95/` are deleted. Scratch logs remain in `WT/scratch/rv95_u9_01/`.
