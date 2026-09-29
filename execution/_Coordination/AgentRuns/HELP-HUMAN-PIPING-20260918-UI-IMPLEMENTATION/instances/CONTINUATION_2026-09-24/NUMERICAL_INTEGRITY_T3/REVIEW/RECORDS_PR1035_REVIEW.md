# RV9: independent review of records PR #1035

**Verdict: PASS.** There are no BLOCKING findings. There are 3 SHOULD-FIX findings and 9 NOTEs.
- **Every headline number and every merge fact I checked holds.** That covers the PR, merge, head and base SHAs, the CI run ids, conclusions and heads, the DEC-025 summaries, the T9 counts, the 40 leaves, the libm table, the 764 gate rows and the suites summary. No placeholder SHA remains.
- **The rulings are intact.** ROOT_RULINGS_V1 changes by 10 in-place annotations, each keeping the original text verbatim and adding a pointer, plus four appended K1 sections. There is no silent rewrite.
- **The SHOULD-FIX findings are about currency and standing guidance, not about false records:**
  - S1: stale present-tense status in the work graph;
  - S2: the operating notes tell the next ROOT to force-push;
  - S3: the owner's Mac DEC-025 gate decision lives only inside one merge record.

## Reviewer, brief and basis

- **Reviewer:** RV9, a Type 2 TASK dispatched directly by ROOT (HELP_HUMAN). I wrote none of these records.
  - I made no Git writes and did not delegate.
  - I ran no cargo, npm or test suite. The only test run was GEN-8.
- **Read:** Root `AGENTS.md`, `agents/AGENT_TASK.md`, `TASK_BRIEFS/_COMMON.md`, and `.agents/skills/chirality-change/SKILL.md` (for S2).
- **Candidate:**
  - `origin/codex/piping-numerical-integrity-20260926` = `59660791a01e67ca8c3add3535cfe64914d15ce4` after a fetch. This is PR #1035's head per GitHub, and `<wt>/numerics` HEAD.
  - Base: main `eb52114e9`, which is also the merge base.
  - Reviewed: the complete diff `git diff origin/main...59660791a0`, 195 files (190 added, 5 modified).
- **Main moved during the review** to `7535bd7e1` (PR #1033). It touches only `projects/chirality-app-v4/`, so there is no overlap with this PR. `git merge-tree` merges the candidate cleanly. The review stands for `59660791a0`.
- **PR #1035's own checks:** Desktop E2E (source mode), harness, pec and the selection jobs pass; the rest are skipped.
- **Out of scope:** two untracked briefs now sit in `<wt>/numerics` (`TASK_BRIEFS/I9_M03_SKEW_PIN.md` and `I10_K2B_IMPLEMENTATION.md`). They are not in the candidate, and I did not review or touch them.

## 1. Scope: records only (PASS)

- **All 195 paths are under `projects/chirality-piping/execution/`:** 194 under `T3/`, plus the work graph.
- **File types:** 16 `.md`, 93 `.txt`, 73 `.log`, 3 `.json`, 3 `.tsv` and 7 `SHA256SUMS`. Code in the run records is committed as `.rs.txt`, `.py.txt` and `.sh.txt`.
- **No product code, test, fixture, schema or tool changes.**

## 2. Hygiene (PASS, with N9)

- **GEN-8:** `<VENV>/bin/python -m pytest tools/practitioner_harness/test_live_baseline.py -k gen8`, run from `<wt>/numerics` at the candidate: **1 passed**.
  - It passed again after this review's files were written (`_run_records/records_pr1035/gen8.txt`).
- **Machine paths:** the 56,504 added lines contain no home-directory, temp, system-private, tool-install or tilde path. The only root-anchored strings are the python3 shebang lines of the committed scripts.
- **Model identifiers:** none in any added or changed file. The only hit is the product test name `…bind_model_identity`.
  - Commit trailers carry the host's attribution, as `HANDOFF_2026-09-28_TO_LOCAL.md` §6 prescribes. They are commit metadata, not files.
- **SHA256SUMS:** every SHA256SUMS in a new or changed folder verifies. Each lists exactly the folder's tracked files, with none missing and none extra.

  | Folder | Entries |
  |---|---|
  | `IMPLEMENTATION/K1_MERGE/` | 10/10 |
  | `IMPLEMENTATION/K2A_MERGE/` | 4/4 |
  | `IMPLEMENTATION/KD5_MERGE/` | 5/5 (`ADDENDUM_1.md`'s entry updated) |
  | `PLATFORM_CALIBRATION_MAC/` | 22/22 |
  | `REVIEW/_run_records/k1_review/` | 51/51 |
  | `REVIEW/_run_records/k1_review_delta/` | 34/34 |
  | `REVIEW/_run_records/k2a/` | 55/55 |

- **The review documents' hashes match their citations:**
  - `K1_REVIEW.md` hashes `e7b9809d…` at `369dc2f16` and at the head, as K1_MERGE cites;
  - `K1_REVIEW_DELTA.md` hashes `878e8893…` at `a44084695` and at the head.

## 3. Rulings integrity (PASS)

- **`ROOT_RULINGS_V1.md`:** lines 1–849 of main survive verbatim except lines 745, 748, 774, 801, 806, 808, 823, 833, 834 and 843.
  - Each of those lines is main's text plus one bracketed pointer. At 801 and 843 the pointer sits mid-line, and removing it gives main's line exactly.
  - Lines 850–898 are four new dated K1 sections.
  - The pointers resolve:
    - "K2a product reach: correction (ROOT)" (:758) and "correction 3" (:841) exist;
    - `IMPLEMENTATION/K2A/RETURN_ADDENDUM_1.md` is on main (`aad23e82d`).
  - K2A_REVIEW §8.3 asked for exactly the pointers at :801, :806, :823, :833, :834 and :843. The PR adds those, plus the ruling-1, cost-clause and captured-entry-only pointers.
- **`ROOT_RULINGS_V2.md`:** unchanged.
- **The other in-place edits** follow the same form:
  - `TASK_BRIEFS/I6_K2A_IMPLEMENTATION.md`: two bracketed supersession pointers, to addenda 2 and 3, which exist at :97 and :117.
  - `KD5_MERGE/ADDENDUM_1.md`: one added bullet. The lines it cites, `KD5/RETURN.md:630` and `KD5/CHANGE_RECORD.md:113`, do carry the withdrawn 15–20% figure.

## 4. Merge records (PASS)

The facts below are in `_run_records/records_pr1035/github.txt`.

### K2A_MERGE/RECORD.md

- **PR #1032:** MERGED 2026-09-28T04:26:55Z as `f12e068761de…`. The head is `aad23e82dd5d…` and the merge's second parent is `aad23e82d`.
- **The first parent is `06069225`, not `649162522`.** Main moved by PR #1031, which touches only `projects/chirality-app-v4/`. The record's "main `649162522`" is the main merged into K2a at `79c0d320b`, whose parents are `80290ce98` and `649162522`, and that is true.
- **Chain:**
  - `80290ce98`'s parent is `5ae22926e`;
  - `79c0d320b` → `aad23e82d` is 13 files, additions only;
  - FK `tests/k2a_checked_formation.rs` +135 and `tests/k2a/rf_range_models.rs` +19, all else under `IMPLEMENTATION/K2A/`.
- **CI:** every run's conclusion is success, on the stated head.

  | Run | Workflow | Event | Head |
  |---|---|---|---|
  | 36372299519 | Piping Desktop E2E | workflow_dispatch (log shows `target_base` `649162522e06…`) | `79c0d320b` |
  | 36372300496 | Piping Desktop E2E | pull_request | `79c0d320b` |
  | 36376818888 | Piping Desktop E2E | pull_request (jobs include the Numerical cargo suite, remainders 1–4 and Desktop E2E) | `aad23e82d` |
  | 36376818884 | governance-harness | pull_request | `aad23e82d` |
  | 36376818928 | pec-tests | pull_request | `aad23e82d` |

- **DEC-025** (`dec025/SWEEP_20260928T030440Z_79c0d320b880.json`):
  - commit `79c0d320b`, `working_tree_dirty: false`, only `sandboxed`, overall pass;
  - all four surfaces pass, with exit codes 0 / 0 / 0,0 / 0;
  - the log shows 39 manifests, pytest 3023 passed and 32 skipped, and vitest 134 files and 2822/2822;
  - the window matches `meta.txt`.

### K1_MERGE/RECORD.md

- **PR #1034:** MERGED 2026-09-28T05:31:50Z as `eb52114e919582aa…`, with parents `f12e06876` and `b6f1724b4`. The head is `b6f1724b4af5…` and the base is `f12e06876`.
- **The chain table matches `git log --first-parent 134eefc24..b6f1724b4` commit for commit.**
- **CI:**
  - 36380608240 is a pull_request run, success, on `b6f1724b4`. Its Numerical cargo suite covers 39 distinct manifests.
  - 36380617536 is a workflow_dispatch run, success, on `b6f1724b4`, with `target_base` `f12e068761de…`.
  - "19 checks passing and 0 failing" is true: 19 check runs succeeded on `b6f1724b4` and 5 were skipped. The PR rollup shows 12 because it de-duplicates by name.
- **T9 "all on the candidate head":** true in substance.
  - The run used `3b86b111f` plus the interaction test copied over it. Outside `execution/`, the only difference from `b6f1724b4` is that file, whose committed hash `cf241452…` equals RETURN's.
  - The base and candidate hashes in `K1/_run_records/combined/t9/` both equal the Mac-native main hashes in `PLATFORM_CALIBRATION_MAC/t9/`.
- **The DEC-025 account matches `dec025/`:**
  - commit `b6f1724b4`, `working_tree_dirty: false`, only `sandboxed`, **overall fail**;
  - cargo exits 101 at `core/product_physics` on `s11g_tests::t13_committed_fallback_uz_is_byte_identical` ("SparseInteractive: committed bytes changed");
  - pytest, vitest and the build are `not_run`;
  - `surfaces_2_3_5.sh.txt` runs the tool's own commands for surfaces 2, 3 and 5. `run_evidence_sweep.py` numbers Playwright 4 in the full plan, so "surfaces 2, 3 and 5" is right even though the partial JSON orders the build 4th.
  - Those surfaces all exit 0: pytest 3023 passed, 32 skipped, 130 subtests; vitest 134 files and 2822/2822; the build completed. These equal F1a's Linux counts (`F1A_MERGE/RECORD.md:28–29`).
  - The full cargo surface is `K1/_run_records/combined/suites/suites_compare.txt` on main. Only the three platform tests fail, their failure blocks hash-identical to the base; 34 tests are added, and 0 changed or removed.
- **The force push is stated plainly.** It happened outside the standing grant, without the owner's explicit authorization, and was disclosed afterwards. The pre-reshape commits `d08b0efc7`, `9d4ba0e17`, `19925122b` and `3513fd8ab` are all on `origin/codex/piping-k1-wip-20260928`.
- **The skew M03 pin miss is stated plainly** ("ROOT's miss"). The timeline holds:
  - RV8's full review at `369dc2f16`, 04:17Z;
  - the K2a merge record at `435a26971`, 04:29Z;
  - the K1 merge at 05:31:50Z.

## 5. The platform calibration (PASS on every headline; N4 and N5)

I re-derived these from the committed files (`_run_records/records_pr1035/rv9_checks.py.txt`, output in `checks.stdout.txt`):
- **T9 against F1A's Linux candidate hashes:**
  - native **100/112**;
  - correctly rounded replay **112/112**.
  - The committed sorted copy equals F1A's record, and F1A's equals K-D5's four hash lists.
- **Leaves:** `t9/platform_differences.txt` has 12 outputs and **40 leaves** (the header counts sum to 40, and there are 40 leaf lines): 12 across the ten hypot outputs, 21 dense and 7 sparse. Its prose claims match the lines, except as noted in N5.
- **libm:** 334 hypot, 6 exp and 10 expm1 distinct calls.
  - I computed the correctly rounded values independently (120-digit Decimal, nearest binary64 with ties to even). They agree with the committed table on all 350 calls.
  - macOS differs on 16 hypot, 1 expm1 and 0 exp.
- **Gate:** `gate/result_part1_mac.json` and K-D5's Linux `combined/gate/part1_result.json`:
  - equal **as multisets on every field of all 764 rows**, with no duplicate rows;
  - not equal in order (the record says so);
  - every summary field equal: 764 evaluated, 328 trusted, 0 breach triples, PASS.
- **Other gate facts:**
  - `gen_out_sha256.txt` lists `cases.json` plus 222 requests;
  - Linux's largest successful peak is 3.534 GiB;
  - the 24 n10000 runs are refused near 6 GiB.
- **Suites:**
  - `per_manifest_first_pass.txt` has 39 manifests, 37 exiting 0, with product_physics and headless failing;
  - the no-fail-fast excerpts give 522 passed and 1 failed over 17 targets, and 82 passed and 2 failed over 8 targets, as `suites/SUMMARY.md` says;
  - the attribution rows match the T9 leaves (`fallback_uz` sparse, one moment magnitude; `connected` sparse, two force magnitudes; `connected` dense, one).
- **Main `649162522`'s piping tree:** it equals `134eefc24`'s on `core`, `fixtures`, `validation` and `schemas` (empty diff).

## 6. Work graph (PASS on the required items; S1, N2 and N3)

- **K2a:** merged (PR1032, `f12e06876`), with RV7 NOT PASS at `79c0d320b` (B1), then PASS at `aad23e82d`. The graph's figures match K2A_MERGE: the 888-run gate with 0 trusted breaches and 0 standing changes, and T9 112/112.
- **K1:** merged (PR1034, `eb52114e9`), with RV8 PASS, CI green, Mac T9 112/112, and DEC-025 "under the owner's Mac-sweep decision".
- **Next safe action:** K2b, then F1b.
- **Skew M03 pin: open.** It was not taken by K1 (ROOT's miss) and now goes to K5 or a tests-only follow-up before K2b.
- **The skew-pin case figures** (2^-1030…2^-1050 accepted with up to 1.19e-7 relative error; refused at 2^-1055 and in S6a) match `REVIEW/K2A_REVIEW.md` §3 (B1 table, :63–70). The N1 input-validation example also matches (:143).

## 7. Other checks

- **Consistent across records:**
  - K1_MERGE's reviewer renumbering matches `a90e7699b`;
  - the handoff's §7 matches K1_MERGE's "no separate T3 manager on the Mac";
  - ROOT's rule that K1 cannot merge before K2a held (04:26:55Z, then 05:31:50Z).
- **`a1029d7da`** is the only cited SHA that does not resolve (N7). Every other SHA cited in both merge records resolves to a commit.

## Findings

| ID | Severity | Site | Evidence | Resolution |
|---|---|---|---|---|
| S1 | SHOULD-FIX | `WORK_GRAPH.md` T3 row, status cell; and the next-safe-action bullet (:62) | **Stale status presented as current.** (1) The T3 cell says "K1 merged as PR1034 …", but further on the same cell says "K1 (I8; …) **is in implementation** from `134eefc24`, spawned **while K2a is still in implementation** …", added by this PR at the spawn and never updated. (2) The next-safe-action bullet is headed "updated 2026-09-27" but now carries 2026-09-28 content. It still says "Active now: the T3 WORKING_ITEMS manager with its TASKs", against `K1_MERGE/RECORD.md:8` ("There was no separate T3 manager on the Mac"). | Put the K1 spawn sentence in the past tense, or mark it in place as superseded by the merge sentence. Update the bullet's date and its "Active now" clause to the Mac arrangement (ROOT dispatching TASKs directly). |
| S2 | SHOULD-FIX | `OPERATING_NOTES_FOR_LOCAL_ROOT.md:64` (§6) | The notes tell the next ROOT: "Then restart the designated branch from main and **force-push it**, since it is your own branch." `.agents/skills/chirality-change/SKILL.md:15` puts a force push outside the standing grant and requires separate explicit authorization. `K1_MERGE/RECORD.md:27` attributes the unauthorized K1 force push to "the cloud operating notes' practice". Merged, the notes would carry that instruction on main as guidance, and the next step after this PR (restarting numerics from main) is exactly the case it covers. | Add an in-place bracketed correction at :64: a force push, including restarting a designated branch, needs the owner's separate explicit authorization (chirality-change `SKILL.md`; `K1_MERGE/RECORD.md`, "History disclosure"). Otherwise ask first, or restart without overwriting remote state. Best done in this PR, before the post-merge restart. |
| S3 | SHOULD-FIX | `K1_MERGE/RECORD.md` "The owner's decision (2026-09-28)"; `HANDOFF_2026-09-28_TO_LOCAL.md` §4 "Gates for a slice PR" | **An unrecorded supersession of a gate.** The owner's decision for Mac-run slices is a standing rule, and K2b is next. It says: the cargo surface passes if its only failures are the three platform tests, identical to Mac main; pytest, vitest and the build must pass; PR Linux CI supplies the clean cargo run. It is recorded only inside one slice's merge record, which the work graph cites. It is not in `ROOT_RULINGS_V1.md` or `OWNER_DIRECTION.md`, and the handoff's gate list still reads "a clean DEC-025 sandboxed sweep" with no pointer. | Transcribe the decision, with its scope and conditions, as a dated entry in `ROOT_RULINGS_V1.md` (as an owner decision relayed by ROOT) or in `OWNER_DIRECTION.md`. Add an in-place pointer at the handoff's DEC-025 gate bullet, and cite the entry from K1_MERGE. |
| N1 | NOTE | `ROOT_RULINGS_V1.md`; `K2A_MERGE/RECORD.md` "Gates" | ROOT's dispositions of RV7's findings live only in K2A_MERGE: B1, S1–S3 and N1–N5 answered by a records-only addendum; N2 closed by per-site rows ("ROOT's revised ruling"); the `79c0d320b` sweep and E2E standing for `aad23e82d` ("owner-endorsed"). The new axis-aligned pointers in V1 have no dated V1 entry. The pointers are well formed, were requested by K2A_REVIEW §8.3, and fit correction 3 ruling 4. | Optional: one dated V1 paragraph recording these dispositions and the pointer pass. |
| N2 | NOTE | `WORK_GRAPH.md` T3 row, skew-pin text | The graph restates K2a product-reach figures (2^-1030…2^-1050, 1.19e-7, 2^-1055, S6a). Correction 3 ruling 4 says "Rulings and the work graph cite it by section and do not restate its figures." The figures match `K2A_REVIEW.md` §3 (:63–70) today, and the graph already restated some on main. | Optional: replace the figures with a citation of `REVIEW/K2A_REVIEW.md` §3 (B1) and `K2A/RETURN_ADDENDUM_1.md` §1. |
| N3 | NOTE | `WORK_GRAPH.md` T3 row | "reached main via PR1029 at `a90e89dcf` (2026-09-28)": GitHub gives mergedAt 2026-09-27T23:45:15Z. | Change the date to 2026-09-27. |
| N4 | NOTE | `PLATFORM_CALIBRATION_MAC/RECORD.md` §2 and §4 | (a) "All 884 runs … agree on ok/ERR" rests on the **uncommitted** `runs_part1_mac.jsonl`, hash-cited in `gate/uncommitted_sha256.txt`. I found it at `<wt>/scratch/calib/gate/`; its sha256 matches `ef8993f4…`. Under the Linux driver's `probe.run.ok` semantics it gives 0 mismatches (794 ok, 90 ERR). The claim holds. (b) "Available memory stayed at 96% throughout": the same file records 96% at the end of 733 runs and **95%** at 151, sampled only at run ends. "It never fired" has no committed evidence. | Optional: say "95–96% at each run's end". Keep the scratch file while the claim cites it. |
| N5 | NOTE | `PLATFORM_CALIBRATION_MAC/RECORD.md` §1, summary bullets | The summary is incomplete against `t9/platform_differences.txt`. The sparse `coefficient_definition` output also differs in the M03 residual-row diagnostic message (denominator `2090073.3883936002` against `2090073.3883936`). The expm1 bullet names three fields, but four leaves move (also `load_reference_states[2].contributions[2].value`). The 40-leaf count and the committed file are correct. | Optional wording fix. |
| N6 | NOTE | `K1_MERGE/RECORD.md` "What ran", item 1 | (a) "it recorded … the cargo manifests after product_physics, as not run": the JSON records the three later surfaces as `not_run` but has no per-manifest record; those manifests simply never ran. (b) `CARGO_BUILD_JOBS=8` and `RUST_TEST_THREADS=4` for the sweep invocation are not in committed evidence; `surfaces_2_3_5.sh.txt` shows them only for surfaces 2, 3 and 5. (c) The pre-sanitization hashes (`82eaa69b…`, and K2A_MERGE's `97228e02…`) cannot be re-derived from the committed, sanitized JSONs (`1e66ed9d…`, `016574a9…`). Both records disclose this. | Optional wording fix for (a). No action for (b) or (c). |
| N7 | NOTE | `K2A_MERGE/RECORD.md` "Gates" | `a1029d7da` (RV7's re-run tree) does not resolve in the repository; K2A_REVIEW §8.1 calls it a local commit. `rerun_a1029d7da/sources_sha256.txt` gives FK `src/lib.rs` `7622e7cc…` and `tests/k2a_checked_formation.rs` `0fa172eb…`, and both equal the files at `aad23e82d`. | Optional: "(a local commit; its FK files hash-equal `aad23e82d`'s)". |
| N8 | NOTE | `K2A_MERGE/RECORD.md` "Findings routed" | This routes the skew pin to K1 with no pointer to K1_MERGE's "not taken" section, which is in the same PR. The work graph carries the open status. | Optional in-place pointer. |
| N9 | NOTE | `REVIEW/_run_records/k2a/logs/SUMMARY.txt`, `rerun_a1029d7da/SUMMARY.txt` and 15 `.log` files | `git diff --check` reports 19 trailing-whitespace lines and 15 blank lines at EOF, all in raw, hash-bound run records. Nothing else in the diff is flagged. | Leave them; editing would break the hashes. Sanitize before hashing next time, as K1's records did. |

## What I did not do

- I did not build or run any product code, suite, gate or T9. Everything in §5 is re-derived from committed files, plus one hash-cited uncommitted file (N4).
- I did not re-review the K1 or K2a code, or re-run the reviewers' mutations. The merge records' review claims were checked against the review documents and their hashes only.
- I did not verify the pre-sanitization sweep hashes, the host memory guard, or the Mac env caps (N4, N6).
- I did not review the untracked I9 and I10 briefs in the worktree.
- I made no Git writes. My only writes are this file and `_run_records/records_pr1035/`, all uncommitted.

## Evidence

`REVIEW/_run_records/records_pr1035/`, with its own `SHA256SUMS`:
- `rv9_checks.py.txt`: the read-only re-derivation script (standard-library Python, run from the repository root at `59660791a0`);
- `checks.stdout.txt`: its output;
- `github.txt`: the GitHub and git facts for §4;
- `gen8.txt`: the GEN-8 run.
