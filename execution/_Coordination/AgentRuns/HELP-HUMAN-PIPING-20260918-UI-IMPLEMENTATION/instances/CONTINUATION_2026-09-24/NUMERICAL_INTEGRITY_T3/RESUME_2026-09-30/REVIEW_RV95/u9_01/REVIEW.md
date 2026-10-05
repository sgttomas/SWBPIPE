# RV95: fresh independent complete review of the F2a D1 milestone PR (#1082)

**Reviewer:** RV95, TASK (Type 2), dispatched directly by ROOT (HELP_HUMAN, Agent 0). ROOT is the return path. No descendants. I wrote none of the code under review and am not RV94.

**Brief:** `R/BRIEFS/RV95_U9_PR_REVIEW.md` (NUM `1217af76f3`), read in full, with the repository root AGENTS.md (merge policy), `R/I61/u9_plan_01/PLAN.md`, and RR from "Step 4 planned: decisions and dispatch" to the end (NUM `d069ab3ccf`), especially "U9 planned and ruled: the D1 milestone PR" and "U9 cut".

**Placeholders.** WT = the t3 workspace; NUM = the records/integration worktree; P = `projects/chirality-piping`; PP = P/core/product_physics; RS = P/core/reporting/result_export; PY = P/core/analysis_runs; TS = P/apps/desktop/src; T3 = P/execution/…/NUMERICAL_INTEGRITY_T3; R = T3/RESUME_2026-09-30; RR = T3/ROOT_RULINGS_V1.md; PKG = T3/IMPLEMENTATION/F2A_D1. Line numbers are at the current head `6d8f8a82b2` unless stated.

## The candidate and how it moved during the review

| Head | Commit | What | Reviewed here |
|---|---|---|---|
| C (cut) | `6b9bb19a5f` on M `5fdc5ab601` | `5a0461661f` source snapshot (136 files from U `2e03d7cc25`, `compatibility.py` per U9 decision 3) + `6b9bb19a5f` evidence package (10 files, 184,282 B) | complete diff against M |
| +1 | `fd3cbebb42` (NUM `c7bc3fd54e`) | CI numerical-selection policy: `NUMERICAL_APP_INPUTS` | in full (ROOT's relay 1) |
| +2 | `92a5a9da1c` (NUM `7ff569a55c`) | `source_blocks::integer` bound before `usize::try_from` (wasm32) | in full (ROOT's relay 2) |
| +3 | `6d8f8a82b2` (NUM `d069ab3ccf`) | U1's protected ordinary-bytes pin gated to aarch64-macos | in full (ROOT's relay 3) |

The PR head at the end of my review is **`6d8f8a82b27547c070fd81e4d345eb1208b67ba4`** (verified with `gh pr view`); hosted CI's numerical cargo suite on it was still pending when I last read it (see §5). My copy was a `git archive` of each head (P without `execution/`, plus PKG), advanced file by file and verified blob-for-blob against the head's tree (26,832 files; the only difference is my appended scratch sweep module, below).

## Host and method

- `git archive` copies in `WT/rv95/` (PR) and `WT/rv95_main/` (M), targets in `WT/targets/rv95/` (registered), `WT/targets/rv95-stale/` (Stale: `RUSTFLAGS=--cfg=rv95_stale`) and `WT/targets/rv95-wasm/` (wasm32 `cargo check`), logs and scripts in `WT/scratch/rv95_u9_01/`.
- Default toolchain (rustc 1.97.1 `8bab26f4f68e`, aarch64-apple-darwin), `--locked --offline`, `CARGO_BUILD_JOBS=4`, `RUST_TEST_THREADS=2`, one cargo job at a time (my own jobs chained). Python 3.13.14 (repository venv) with `TMPDIR` and `--basetemp` under scratch and `-p no:cacheprovider`; Node 24 vitest and tsc with an untracked `node_modules` symlink and the prebuilt WASM copied from `WT/f2a-u7` (hashes in `evidence/`). The memory guard (PID 5387) was checked before every job; its log has no KILLED line at all.
- No Git writes or index operations (reads with `GIT_OPTIONAL_LOCKS=0`), no installs, nothing native, solver-at-scale or DEC-025, nothing written to WT/f2a-pr or other agents' scratch. **Disclosure:** the agent harness keeps a few lines of each background command's stdout in its own task files under the system temp directory; no job output, log or temporary file of mine went there.
- **The scratch sweep harness.** For the 324-output sweep I appended I61's `zz_i61_u3g2_sweep.rs` (R/I61/u3_grant2_01/_run_records, sha256 `6b181a61…`) as `#[cfg(test)] mod zz_i61_u3g2_sweep;` at the end of PP `lib.rs` in my copy, as RV94 did. `lib.rs` is not a reviewed input, so the build stays Registered (asserted by the tests). It adds one ignored test to the PP counts.

## Verdict: **PASS**

| | Count |
|---|---|
| BLOCKING | 0 |
| SHOULD-FIX | 2 |
| NOTE | 7 |

**The headline:**
- **Source equality holds at the cut** (my own commands and `source_equality.py` 5/5 on `6b9bb19a5f` against U `2e03d7cc25`); the one resolution (`compatibility.py`) is exactly main's two call-site lines; the second main overlap introduced later (`source_blocks.rs`, `92a5a9da1c`) is a clean three-way merge that I reproduced byte for byte. No execution-records bulk is in the PR; the package is exactly the 10 listed files.
- **Every changed file maps to a review** (ledger, §2). The hunks no earlier review covered — the citation rewording `2e03d7cc25`, the `compatibility.py` resolution, U3 grant 1d (never diff-reviewed; N-1) and the three post-cut commits — I read in full. They are correct.
- **The milestone holds on every head I built.** In the registered build the actual Direct entry publishes U1's pinned successors (sparse `ac6986b0…`, dense `6cd1d249…`), byte-identical to the committed carrier fixtures; the 324-output sweep is registered `9a74ff16…` / Stale `0e2db8b8…` at `6b9bb19a5f` and at `92a5a9da1c`, so neither main's PR1080 nor the 32-bit fix moves a published byte. Python, Rust and TS readers agree on the live successors: eligible with the invocation, `needs_recompute` without, G8 `INVOCATION_MISMATCH` with the mode swapped; TS stands eligible only with the live capture.
- **The three post-cut fixes are right** (§4.3–4.5): the CI-policy fix is the correct place (the guard belongs in PP); the wasm32 fix is exact on 64-bit and refuses identically on 32-bit, and no F2a runtime path is reachable from the wasm exports; gating the U1 dense pin to the pinned target is right and leaves no PR claim untrue across targets.
- **Two SHOULD-FIX items remain before the freeze:** six more stale "no permit / production-unreachable" texts that U9 decision 6's list missed, two of them public rustdoc (S-1); and the evidence package and PR body, which describe the cut head, not the current one (S-2; ROOT already plans the regeneration).

## Findings

| # | Sev | Where (`6d8f8a82b2`) | Evidence | Remedy |
|---|---|---|---|---|
| S-1 | SHOULD-FIX | PP `src/lib.rs:2176`; `src/retained_memory.rs:2757`; `src/retained_wire.rs:6–7`, `:14`; `src/retained_memory_law_tests.rs:2–4`; `src/retained_memory_witness_tests.rs:6`; `tests/retained_memory_challenge.rs:6` (and, milder, `lib.rs:117`) | **Stale pre-registration statements that U9 decision 6's six-place list missed.** Since `0c7827b6ad` a production permit exists for D1 Direct calls in the registered build, and the serializer is reached through it. Still in maintained text: the rustdoc of the **public** type `RetainedPreviewOutput` says "No production permit exists." (two items above the repaired `:2185`); the rustdoc of the **public** `RetainedHeadlessContext` says "No production profile exists;"; `retained_wire.rs`'s module doc says "Production-unreachable: no public entrypoint calls it; U3 installs it behind the capture permit", and `:14` "Production-unreachable until U3 installs it…"; the law tests' header says "No profile and no permit is constructed here (decision 7): the registered-identity list stays empty", while the same file holds `admit_grants_a_permit_for_the_milestone_in_the_registered_build`; the witness tests' header says "No permit exists (decision 7)"; the challenge header says "(no permit exists, so the ordinary span runs …)" although its body already bounds the permitted case by `MAX_PHASE_BYTES`. `lib.rs:117` ("production-unreachable until U3") is historical and accurate as a temporal statement. None changes behaviour; they understate what the PR makes public, which the brief's item 3 and decision 6 ask to correct. | The decision-6 treatment: a comment-only, line-neutral repair before the freeze (I61: `lib.rs`, `retained_wire.rs`, the challenge; I65: the `retained_memory*` files), classified by the frozen-head Pass B; RV95 confirms. If ROOT prefers, the three test-file headers may be ruled acceptable and only the two public rustdocs and the `retained_wire.rs` module doc repaired. |
| S-2 | SHOULD-FIX | PKG `CHANGE_RECORD.md` §1, §2, §3, §6; `PR_BODY.md` "Source and packaging" (and the live PR body); PKG `source_equality.py` check 3 | **The package describes the cut, not the current head.** At `6d8f8a82b2`: S has **139** files (59 A, 80 M; 11,943,997 B), not 136; five S files differ from U `2e03d7cc25` (`compatibility.py`, `source_blocks.rs`, `e2e_plan.py`, `test_ci_e2e_plan.py`, `retained_wire_tests.rs`; the reviewed maintained source is now NUM `d069ab3ccf`'s); "Main's PR1080 … touches no file in this PR" is false since `92a5a9da1c`; `source_equality.py --int d069ab3ccf` FAILs checks 2, 3 and 5 on `source_blocks.rs`, because check 3 knows only `compatibility.py`. The three post-cut commits and their review are not in §3. Separately, §1's "11,811,048 B" is `e543c3d8f3`'s byte total; U's is 11,813,578 B and the cut's 11,813,616 B (line counts +204,652/−896 are right). | At the freeze, as ROOT has ruled: generalize check 3 to every S file main also changed (I reproduced `source_blocks.rs`'s (NUM, B, M) merge byte for byte, §1), refresh counts, bytes, the reviewed head and the PR1080 sentence, list `fd3cbebb42`, `92a5a9da1c` and `6d8f8a82b2` with their review, add the limit that U1's protected ordinary-bytes pin applies on the registered target only, reseal, rerun GEN-8 and `check_citations.py`. RV95 confirms the regenerated package. |
| N-1 | NOTE | `8abb5274a9` (U3 grant 1d): PP `lib.rs` +82/−7, `retained_facade_tests.rs` +50 | **The one unit commit no fresh reviewer diff-reviewed.** RR "U3 grant 1d (test-only) committed" planned RV85's confirmation of T1/U2 in grant 2's review, but grant 2 went to RV93 against `0c7827b6ad`, which already contained 1d; RV89 used it as a base. I read it in full: every `lib.rs` line is `#[cfg(test)]`, inside `#[cfg(test)] mod retained_tests_hooks`, or doc text; the production `carry_test_hooks` is unchanged. The non-test runner binaries carry 0 `retained_tests_hooks` symbols (positive control: PP's lib-test binary, 309). CHANGE_RECORD's "—" for 1d is truthful; PR_BODY's "Every unit had a fresh independent review" is slightly broad. | None required. Optionally say "every unit except U3's test-only grant 1d, read by RV95" in the regenerated body. |
| N-2 | NOTE | `retained_memory.rs:2576`; `PY/retained_precision.py:649`; `P/fixtures/results/retained_precision_carrier_cases.json` (`ruling` strings); test comments naming "I66 u6b", "I66 post-U6f", "I63 audit", "I62 checkpoint-B" | **Citation residues outside `check_citations.py`'s patterns.** Two bare code-line numbers survive the rewording: `` `solve_load_case_observed`, :3918, :4011, :4042, :4098 `` (lib.rs lines) and "set only by verify_precision, 4333" (adaptive.rs). Both are accurate at this head. Agent-run references without a `_NN` suffix ("I67 u6d RETURN F1/F2" in two carrier-case `ruling` strings, and the four comment mentions) are not indexed. The check's own claim ("every record … citation the PR adds resolves") is therefore a little broader than what it scans. | Optional: reword at the next touch of those files, or extend the check's patterns. No change needed for U9. |
| N-3 | NOTE | `P/tests/test_ci_e2e_plan.py:66–70` (`fd3cbebb42`) | **The new policy pin is vacuous for an empty set.** Mutant C2 (`NUMERICAL_APP_INPUTS = set()`) survives `test_ci_e2e_plan.py`; the suite as a whole kills it through `test_ci_numerical`'s real-include test (C1). | Optional: assert the set's exact content (or non-emptiness) in the pin. |
| N-4 | NOTE | RS `retained_precision.rs` (about 30 `u(..) as usize` index casts, e.g. `:947`, `:1188`, `:2228`, `:3653`, `:3878`) | **32-bit readiness (ROOT's relay 2).** The wasm32 crates use PP only for its self-weight helpers, types and constants; no wasm export reaches a preview solve, a retained entry or the Rust reader, so no F2a runtime path runs on a 32-bit target today, and the registered identity is aarch64 only. The Rust reader's index casts would truncate on 32-bit if it were ever reached there. | Record as a precondition: any future 32-bit consumer of the Rust reader or of W1 needs its own review (and `try_from` on those indices). |
| N-5 | NOTE | RS `src/source_blocks.rs:57` | **The 2^53−1 integer bound is untested in `result_export`** (main's pre-existing gap; the post-cut fix reorders exactly this line). Mutant S1 (bound removed) survives the whole `result_export` suite. | Optional, main-owned: a source-blocks fixture with an integer above 2^53−1 expecting `SOURCE_BLOCKS_INTEGER`. |
| N-6 | NOTE | PP `src/retained_wire_tests.rs:19–22`, `:69–71` (`6d8f8a82b2`) | **The gated U1 pin (ROOT's relay 3).** Gating is the right fix; a per-target Linux pin is not advisable (§4.5). The gate is target-only (`aarch64` + `macos`), broader than the registered identity, which is fine for a regression anchor. G1 (dense digest altered) is killed on this Mac, so the gate is live here; G2 (gate forced off) passes, so the other-target branch holds. | Optional: print the observed digest on other targets, so a Linux drift is visible in CI logs. For wider F2a: the retained product values also use platform `hypot` for support magnitudes (`frame_kernel/…/product_certificate/final_case.rs:1847`, `:1850`; the PP guard at `retained_product.rs:2176`), so registering any further identity (release, another target) must re-establish the milestone's bytes and verdicts on that identity, not inherit them. |
| N-7 | NOTE | `PR_BODY.md` "What is public" | The body's D1 list omits "capped counts" and abbreviates the identity (no opt-level 0, debug assertions, panic=unwind); CHANGE_RECORD §3/§4 carry both. Not untrue. | Optional: add "capped counts" in the regenerated body. |

## 1. Source equality

**`source_equality.py` on the cut** (`--pr 6b9bb19a5f --int 2e03d7cc25 --main 5fdc5ab601`): 5/5 PASS (`evidence/equality/source_equality_cut_6b9bb19a5f.txt` and `.json`); B = merge-base = `381be775ae`; |S| = 136.

**My own commands** (outputs in `evidence/`):
1. `git diff --name-only B U -- . ':!P/execution' ':!execution'` equals `git diff --name-only M C -- . ':!P/execution'` as sorted lists (136; 59 A, 77 M; all under P).
2. `git diff --exit-code U C -- <S minus compatibility.py>`: no content difference; `git ls-tree` modes equal (all 100644). The only raw difference is `compatibility.py` `322c58f1e2 → 767da34027`.
3. `git diff U C -- compatibility.py` is exactly two hunks, `:698` and `:703`, each main's `_same_canonical(…)` call site from PR1078 (`1a0bfe2b71`); U already carries the helper (`:254`, body byte-identical to main's) and main's import line. **Resolution correct.**
4. `git diff --name-only M C -- P/execution` is exactly PKG's 10 files; `shasum -c` on PKG: 9/9 OK; sizes sum to 184,282 B. No other execution path, no root file, no `.github` file is in the PR.
5. NUM `4c876ac7e8` has the same maintained source as U (`git diff U 4c876ac7e8 -- . ':!P/execution'` empty).

**Main since B** (87 commits): 4 non-execution files under P (`compatibility.py` and its test, PR1078; `source_blocks.rs` and its test, PR1080); the rest are App v4 records, governance-harness and workflow files and Root `AGENTS.md`. Main touched none of PP's 14 reviewed inputs.

**After the cut.** `fd3cbebb42` and `6d8f8a82b2` change only S-or-tooling files carried byte-identically from NUM (`c7bc3fd54e`, `d069ab3ccf`). `92a5a9da1c`'s `source_blocks.rs` equals `git merge-file -p` of (NUM `7ff569a55c`, B, M) byte for byte, with no conflict: NUM's change swaps two lines in `integer`, main's PR1080 changes `validate_in`. `source_equality.py` at `6d8f8a82b2 --int d069ab3ccf` reports checks 1 and 4 PASS and 2, 3, 5 FAIL on `source_blocks.rs` only (S-2).

**Citations.** `check_citations.py --base M --head C`: **368 resolved, 0 ambiguous, 0 unresolved, PASS** (65 record/RR occurrences, 41 distinct, 0 unused entries; 289 design-document citations; 14 pinned code lines). The pinned NUM commit `cfcdb5997a` is on `origin/codex/piping-numerical-integrity-20260926`, so the URLs resolve. No machine path in any added line or in PKG. The residues are N-2. At `6d8f8a82b2` the check still gives 368 / 0 / 0, PASS.

## 2. The complete diff and the ledger

`evidence/ledger.tsv` has one row per S path at `6d8f8a82b2` (139 rows): status against M, the last commit on U, the full chain of non-merge commits B..U touching it with the review that covered each, and the post-U commits. Every commit in the 99-commit B..U chain is mapped; none is unmapped. **No merge in B..U has a combined-diff hunk on S** (`git diff-tree --cc` on all 35 merges: 0), so every merged blob is the union of reviewed sides.

| Unit (commits) | Review (reviewed commit) | Paths whose chain includes it |
|---|---|---|
| Checked work, kernel helpers, origins, coefficients, bridge, source residual, product certificate, selected material, supports (`fdae294643` … `8104a4fedd`) | RV51, RV53, RV54/55, RV56/57, RV58, RV60/61, RV62/63, RV65 (accepted at each component's commit; RR "Checked-work source accepted…", "Accept native origin…", etc.) | RV51 38; RV53 5; RV54/55 14; RV56/57 11; RV58 8; RV60/61 12; RV62/63 5; RV65 6 |
| Prepared producer (`430bc4f798`, `c79a1c293d`, `922db9dce3`) | RV67/RV68 (`922db9dce3`) | 21 |
| Caller separation, census (`24af17c470`) | RV72 | 6 |
| Prepared trace, prior cause (`7018513af3`, `fc23cff95f`) | RV74 | 14 |
| Summary-coverage seam (`c618675e84`, `fa225abded`) | RV77 | 6 |
| Readers, schema, corpus to 07f (the "WIP (unaccepted)" chain to `85905e95e9`) | RV78–RV81, accepted at `85905e95e9`, fanned in at `c15e64b756` | 108 |
| U1/U2 (`59a5de2032`, `b54caba7ab`) | RV82 (`b54caba7ab`) | 15 |
| U3 1, 1b, 1c (`bee3dc07ca`, `4b31bbf23a`, `886bef131a`) | RV85 | 21 |
| U3 1d (`8abb5274a9`) | **none as a diff** (N-1); read by RV95 | 2 |
| U3 grant 2, D-U6-5 (`664f8df7b7`, `f71478696b`) | RV93 and addendum | 6 |
| U4 G5 p1/p2, G6, pre-registration, registration, G7 (`1e323058f3` … `7f07a2f7b4`) | RV89 (each), RV87 (G6, TEXT); registration = the reviewed `registration.diff`, tree-equal (RR "Registration applied…") | 28 |
| U6a–U6f and repairs (`844448112f` … `5f8d6291b8`) | RV88, RV90, RV91, RV92 (+ addenda) | RV88 20; RV88/RV91 15; RV90 7 (+4 confirm); RV91 confirm 7; RV88 confirm 7; RV92 10 |
| U7 slices T, P, F, fix, L (`0ca5449c87` … `ffe65ef203`) | RV94 (`ffe65ef203`) | 31 |
| U7 repair round (`fc575c7e56`, `8c84e7ae14`, `e543c3d8f3`) | RV94 addendum (`e543c3d8f3`) | 17 |
| Citation rewording (`2e03d7cc25`) | **RV95** (§3) | 8 |
| `compatibility.py` resolution (cut) | **RV95** (§1) | 1 |
| `fd3cbebb42`, `92a5a9da1c`, `6d8f8a82b2` | **RV95** (§4) | 4 |

**What I read myself, in full:** `2e03d7cc25` (8 files, 91 lines; every reworded symbol exists at this head except that `ordinary_report` is a method in `retained_product.rs` called at `lib.rs:4543`, which the comment's "in lib.rs" fairly describes); the `compatibility.py` resolution; `8abb5274a9`; the three post-cut commits; and PP `lib.rs`'s whole diff against M (the dispatch, the permitted run on the reserved stack, the notice reservation, W1 and every observer hook): with `product = None` every hook is skipped, so the ordinary route's arithmetic is untouched, and the `ptr::eq` ordinal lookups under `product.is_some()` index the same `temperature_points` vector, so their `expect`s cannot fire. The rest of the diff was reviewed by the ledger's reviews; I spot-read the seams the plan names (§6).

**`#[cfg(test)]` containment.** Every hook call site in production files is `#[cfg(test)]`-gated (`lib.rs:2379`, `:2933–2934`, `:3005`, `:3125–3126`, `:3138–3139`, `:3146–3147`, `:3153–3154`, `:3159–3160`; `retained_product.rs:3244`), the hook module is `#[cfg(test)]`, and the `test_*` helpers in `retained_product.rs` and `retained_wire.rs` sit in `#[cfg(test)]` items. `nm`: 0 `retained_tests_hooks` symbols in the non-test runner binaries, which do contain `retained_w1`.

## 3. Scope truthfulness

**What is public, as stated** (PR body, CHANGE_RECORD §4): the Direct entry publishes the M03-INTEGRITY-MP-v2 successor only for D1 requests in the registered dev/test build with M = 4,026,531,840 B; Headless refused at D1.0; three-language eligibility and the carriers. All of it matches the code and my runs:
- the registered entry (`REGISTERED_PROFILES`) is byte-identical to `0c7827b6ad`'s (sha256 of the block equal), `threshold_bytes: 4_026_531_840`; my sha256 of the 14 reviewed inputs at the PR head equals the registered reviewed-input text 14/14; PP's `Cargo.lock` blob `833cbda4…` is unchanged from `0c7827b6ad` through the head;
- the sweep's Direct rows: registered 64 exact, 4 notice (the two `rejected_stress_range` requests, Preparation fallbacks after W1 ran), 2 successor (the milestone); Stale 70 exact, `Stale`, refused at D1.1; Headless 70 exact, refused at D1.0;
- no product caller: outside PP's tests and the runner's test, the only references to the retained entries are the runner's `run_preview_model_value_with_retained_headless` (`runner/headless/src/lib.rs:740–774`, Headless only, `into_parts()`), which nothing outside its test calls, and `retained_memory.rs:2988` inside `#[cfg(test)] mod tests`.

**What stays closed** is stated in both places (public activation with its six-item checklist, Stale builds and hosted CI, any supported-machine M, U8, wider F2a, S-I, F2b, F3, the owner-held items), consistent with D-U7-1, U9 decision 1 and the plan's §3.2. No maintained text claims more than is public. **The six decision-6 repairs** (`fc575c7e56`) are true against the code and line-neutral; six further stale texts remain (S-1).

## 4. Main's interaction and the post-cut commits

### 4.1 PR1078 (`compatibility.py`)
U's helper and main's are byte-identical, so the resolution keeps one helper and takes main's two call sites (§1). Main's 86 new test lines run against it: `tests/test_analysis_run_compatibility.py` passes in my runs (selected suite, below). Mutant P1 (main's `contract_evidence` call site reverted to `!=`) is killed by main's own `test_copy_checks_refuse_a_boolean_for_an_integer`; P2 (the retained copy compared with `!=`) is killed by the retained carrier tests.

### 4.2 PR1080 (`source_blocks.rs`)
A receipt-order walk in `validate_in`; accepted and refused sets unchanged. On the F2a routes it moves no byte: my sweeps on the cut and on `92a5a9da1c` are registered `9a74ff16…` and Stale `0e2db8b8…`, equal to I66's, RV94's and I61's. `result_export`'s suite is 172/0, exactly RV94's confirmation outcomes plus main's new `actual_pre_repair_stress_range_first_failure_is_stable` (test-by-test comparison). The full Pass B on the cut (I65 `R/I65/u4_g7_05/`, RV89 `R/REVIEW_RV89/u4_g7_03/`) classes it unreachable on D1; I have not re-done that classification.

### 4.3 `fd3cbebb42`: the CI numerical-selection policy
**Correct, and the guard belongs in PP.** `numerical_input` is consumed only at `e2e_plan.py:199` (`numerical_required`), so adding `apps/desktop/src-tauri/src/lib.rs` there makes a change to that file run the crate suite, which contains PP's `legacy_native_and_typed_call_graphs_cannot_acquire_retained_admission`. Moving the guard into the src-tauri crate would take it out of both hosted CI and DEC-025 (WG:41: the src-tauri suite runs in neither), and the guard is load-bearing: RR "U4 plan: decisions…" closes the native-context term of the admission law "by construction" partly on this test. Keeping a separate set (not `NUMERICAL_EVIDENCE_INPUTS`) is right for the stated reason. My run: `test_ci_numerical.py` + `test_ci_e2e_plan.py` + `test_analysis_run_compatibility.py` 105 passed; on the cut, the same CI test fails exactly as hosted CI did (selected suite 1 failed, 1,494 passed, 11 skipped). Mutants: C1 killed; C2 survives (N-3).

### 4.4 `92a5a9da1c`: the 32-bit bound
**Correct.** On 64-bit, `try_from` never fails, so the two filters commute and the result is identical; on 32-bit, a value above `usize::MAX` but within 2^53−1 now refuses with the same `SOURCE_BLOCKS_INTEGER`. Control: restoring main's order in my copy makes `cargo check --target wasm32-unknown-unknown` fail with `literal out of range for usize` at `source_blocks.rs:57`, as hosted CI did; with the fix, `result_export`, PP, `self_weight_wasm` and `operation_applier` all check for wasm32 `--locked --offline` with only the warnings native builds also emit.
**Other 32-bit hazards:** compile-time hazards are excluded by those checks (const-eval and overflowing literals are deny-level). For runtime: the wasm crates import from PP only `self_weight::*`, `PreviewModel`, `AnalysisStateInput`, `ExpansionLawInput`, `ReferenceConfigurationInput` and `LOAD_STATE_MODEL_VERSION`; no wasm export calls a preview solve, a retained entry or the Rust reader. So, contrary to the relay's phrasing, wasm does not run PP's ordinary preview path, and no F2a code executes on a 32-bit target today. Latent hazards for a future 32-bit consumer are N-4. Behaviour on 64-bit is unchanged: PP registered sweep `9a74ff16…`, Stale `0e2db8b8…`, live successors `ac6986b0…` / `6cd1d249…`, `result_export` 172/0 at `92a5a9da1c`.

### 4.5 `6d8f8a82b2`: the U1 dense ordinary-bytes pin
**Gating is the right fix; a per-target pin is not.** The test's claims — the Direct entry's ordinary envelope equals the plain bytes, and the captured run equals the plain bytes — stay asserted on every target; only the absolute regression anchor is limited to the target it was recorded on. `frame_kernel` has no dependencies (no BLAS or runtime-dispatched SIMD), so a dense-only difference most plausibly comes from platform math on the dense path (for example `hypot` in `frame_kernel/src/rigid_body.rs:52`, `:108`, which differs in the last ulp between libms); I did not establish the cause. A Linux pin would then tie CI to a runner image and libm version. On this Mac: G1 (digest altered) is killed, G2 (gate forced off) passes; the U1 lib tests pass.
**No PR claim depends on cross-target identity of dense ordinary bytes.** Every Stale claim is a same-build equality (exact ordinary bytes, compared within one build); every registered claim (successor bytes, U5, T9, both-entry, the profile's maxima) is stated and evidenced on the registered Mac only; the committed successor fixtures are validated by deterministic readers on any target, and D-U6-5's live comparison runs only when registered. The successor pins in `retained_facade_tests.rs:15–16` and `retained_wire_tests.rs:30–31` are asserted only when `registered()`. The other F2a crates in the numerical suite add no output-hash pins (I checked the changed tests of `performance_harness`, `numerical_robustness` and `runner/headless`; `frame_kernel`'s vectors come from exact-rational generators). Linux results for crates after PP are still to come from hosted CI.

## 5. Gate evidence (U9 decision 12)

At this review, ROOT has not yet sent the freeze-time evidence; this section records what exists and is to be completed in my confirmation at the freeze.
- **Hosted CI.** Cut `6b9bb19a5f` (run 37239539032): Select source coverage FAILED (`test_real_rust_literal_includes_require_numerical`), reproduced by me; fixed by `fd3cbebb42`. Run on `fd3cbebb42`: wasm build failed in the four remainder shards (ROOT's record); fixed by `92a5a9da1c`. Run on `92a5a9da1c`: numerical suite failed on the U1 pin (ROOT's relay); fixed by `6d8f8a82b2`. On `6d8f8a82b2` (run 37241444863 and siblings), at my last read: selection, harness, pec and App checks and all four remainder shards passed (the wasm build now succeeds); the numerical cargo suite was still pending. **The full-SHA dispatch has not run.**
- **G7 src-tauri (ROOT):** RR "U9 G7 and G8 pass…": M 116 / C 116, per-test identical, on the cut. Not rerun for the post-cut commits (none touches src-tauri or its lock).
- **G8 and G9b (I61, `R/I61/u9_g8_01/`, SHA256SUMS 14/14 OK, RETURN `90103096…`):** on the cut. My own sample agrees (§6).
- **G9a (I65 `R/I65/u4_g7_05/`, 101/101 OK, RETURN `ec93eb99…`; RV89 `R/REVIEW_RV89/u4_g7_03/`, 15/15 OK, REVIEW `3bcc025a…`):** on the cut, PASS; RV89 ruled `fd3cbebb42` and `92a5a9da1c` need only the mechanical frozen-head rerun, with one added reviewed entry (RV89 N-1). `6d8f8a82b2` is a test-only edit in a D1 crate's test file; it should be in that rerun's delta classification.
- **Pending, ROOT's:** T9, both-entry (I61 under grant, `R/I61/u9_g5g6_01/` not yet returned), the Mac baseline and DEC-025, GEN-8 on the frozen head, the native witness.

## 6. My own spot checks

| Check | Head | Result |
|---|---|---|
| PP registered, all targets, `--no-fail-fast` | cut; final | 705 ok / 1 failed (t13, Mac known) / 10 ignored (one is the scratch sweep); test-by-test identical to RV94's confirmation run, and identical between the cut and `6d8f8a82b2` |
| PP Stale (`RUSTFLAGS=--cfg=rv95_stale`) | cut; final | identical outcomes to registered, on both heads |
| `result_export` registered and Stale | cut; `92a5a9da1c` | 172 / 0 / 0 each (= RV94 + main's PR1080 test) |
| runner/headless registered and Stale (cut), registered (final) | cut; final | 85 ok / 2 failed (the known `load_reference` pair); identical to RV94's, on both heads |
| The milestone through the actual Direct entry, registered, both modes | cut; `92a5a9da1c` | U1's pins: sparse `ac6986b0…`, dense `6cd1d249…`; byte-identical to `P/fixtures/results/retained_precision_milestone_successor_*.json` |
| 324-output sweep (I61's harness), registered / Stale | cut; `92a5a9da1c` | `9a74ff16…` (2 successor rows) / `0e2db8b8…` (0 successor rows) |
| Readers on the live successors (Python, Rust, TS) | `92a5a9da1c` live bytes | with invocation: `invocation_bound`, `numerical_eligible`, publication `26ef4435…` (sparse, 98 classes) / `487c4cf2…` (dense, 99); without: `needs_recompute`; mode swapped: G8 `RETAINED_PRECISION_INVOCATION_MISMATCH`. Identical in all three |
| TS standing seam | same | registered without live capture: `needs_recompute` / `RETAINED_PRECISION_NATIVE_CAPTURE_REQUIRED`; with it: `numerically_eligible`; another model object: `needs_recompute` |
| PHYS-R4 pair through the Direct entry (G8 c sample) | `6d8f8a82b2` | registered and Stale, both modes: pressure gives the exact refusal bytes (`MODEL_INCOMPLETE`), no successor, no notice; no-pressure `MECHANICS_SOLVED`, exact ordinary bytes, no notice |
| G8 named PP tests (f1b_w2 PHYS-R4 pair, pressure_membrane_range, the U3g2 no-W1 and fallback tests) | cut; final | pass, registered and Stale |
| Python selected suites (retained contract, schema, carriers; the 4 changed schema/consumer tests; main's compatibility test; the 2 CI policy tests) | cut | 1,494 passed, 11 skipped, 1 failed (the CI policy test hosted CI also failed); on `92a5a9da1c` the CI and compatibility tests 105/0 |
| vitest, tsc | cut (no TS change since) | 3,552 / 3,552 in 490 suites; tsc clean |
| wasm32 `cargo check` (result_export, PP, two wasm crates) | `92a5a9da1c` | clean apart from native-identical warnings; pre-fix control fails at `:57` |

**Mutants** (`evidence/mutants.json`; one exact edit each, asserted unique, restored and hash-verified; the copy re-verified pristine against the head's tree afterwards):

| # | Seam | Result (killing test) |
|---|---|---|
| R1 | PP W1: precommit validation result ignored | killed (`u3g2_direct_entry_w1_fallbacks_append_one_notice`) |
| R2 | PP admission: D1.3 pressure-contract clause removed | killed (`law_tests::every_family_clause_refuses_with_its_fact`) |
| R3 | PP: N1 notice not appended on fallback | killed (`u3g2_direct_entry_w1_fallbacks_append_one_notice`) |
| R4 | Rust reader: G8 invocation binding skipped | killed (`u6a_shared_carrier_cases_rust`, …) |
| P1 | Python carrier: main's call site reverted (PR1078 interplay) | killed (main's `test_copy_checks_refuse_a_boolean_for_an_integer`) |
| P2 | Python carrier: retained copy compared with `!=` | killed (`test_analysis_run_carries_the_complete_receipt…`) |
| T1 | TS standing: live-capture conjunct removed (D-U7-4) | killed (vitest) |
| C1 | CI policy: `NUMERICAL_APP_INPUTS` emptied | killed (`test_real_rust_literal_includes_require_numerical`) |
| C2 | the same, `test_ci_e2e_plan.py` alone | survives (N-3) |
| S1 | `source_blocks::integer` bound removed | survives (N-5, main-owned) |
| G1 | U1 dense pin digest altered | killed (gate live on this Mac) |
| G2 | U1 pin gate forced off | passes, as designed (the other-target branch is sound here) |

## Limits

- I did not run the full Python `tests/` directory: seven of its files launch cargo themselves; DEC-025's pytest is ROOT's. The Tauri crate was not built (G7 is ROOT's). Pass B's TEXT classification is I65's and RV89's, taken as inputs.
- My TS runs used the prebuilt WASM; the PR's own wasm build is exercised by hosted CI and DEC-025.
- Linux behaviour beyond what hosted CI reports is not observed here.

## For ROOT to rule

1. **S-1:** repair the six further stale texts before the freeze (decision 6's class), or rule which may stay.
2. **S-2:** the package regeneration at the freeze, as already planned, including the post-cut commits, `source_blocks.rs`'s merge in check 3, and the gated-pin limit in CHANGE_RECORD.
3. Whether the frozen-head Pass B rerun's delta classification includes `6d8f8a82b2` (test-only, PP test file), alongside RV89 N-1's entry.

## Records

`evidence/` (placeholder paths only): the ledger; my source-equality and citation outputs; the three-way merge checks; the sweep TSVs and hashes; live-successor hashes; the three readers' dumps; suite outcome lists; the mutant results; and the scripts (`cargo_run.sh`, `chain1–3.sh`, `run_py.sh`, `run_ts.sh`, `rv95_mutants.py`, the reader dumpers and the PHYS-R4 sample). `SHA256SUMS` covers this folder. **Cleanup done:** `WT/rv95/`, `WT/rv95_main/` and `WT/targets/rv95{,-stale,-wasm}/` are deleted; the scratch logs in `WT/scratch/rv95_u9_01/` remain (temp folders removed). For the same-reviewer confirmation I will rebuild from a fresh `git archive` of the repaired head.
