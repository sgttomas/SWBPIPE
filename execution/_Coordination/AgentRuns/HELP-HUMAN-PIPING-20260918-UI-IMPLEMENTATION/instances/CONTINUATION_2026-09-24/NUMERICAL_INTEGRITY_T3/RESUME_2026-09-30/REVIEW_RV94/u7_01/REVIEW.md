# RV94: independent review of U7, the eligibility switch-on

**Reviewer:** RV94, TASK (Type 2), dispatched directly by ROOT (HELP_HUMAN, Agent 0). ROOT is the return path. No descendants. I wrote none of the code under review.

**Brief:** `R/BRIEFS/RV94_U7_SWITCH_REVIEW.md` (NUM `8f2674570b`), read in full, with the repository's root AGENTS.md and the rulings from "…U7 planned and ruled" to the end of ROOT_RULINGS_V1.

**Candidate:** `ffe65ef203` on `codex/piping-f2a-u7-20261004`, against the U7 base `3f5fca3010`. The code diff (`git diff 3f5fca3010 ffe65ef203 -- . ':(exclude)*/execution/*'`) is 22 files, +947 / −223:
- slice T `0ca5449c87` (TS: N-2 live-capture binding, N-5 panel gates, the N-8 token pin);
- slice P `12a849a7bd` (N-6 `#[doc]` on `into_parts()` / `successor()`, line-neutral);
- slice F `cfda60403f` (the three flags, 07i, case file v4 with D-U7-4, the D-U7-6 and blocked-envelope scope sentences, the pins);
- `e5e1693ceb` (TS `classificationSummary` withheld fails closed without eligible standing);
- slice L `ffe65ef203` (07j: `not_required_second_case_checks_passed`, two count pins).

**Placeholders:** WT = the t3 workspace; P = `projects/chirality-piping`; PY = P/core/analysis_runs; RS = P/core/reporting/result_export; TS = P/apps/desktop/src; PP = P/core/product_physics/src; R = NUM's RESUME_2026-09-30. Line numbers are at `ffe65ef203`.

**Copies and host.**
- Fresh `git archive` copies (P without `execution/`) of `ffe65ef203` (`WT/rv94/cand`, plus a pristine copy and three mutant lanes) and of `3f5fca3010` (`WT/rv94/base`). Untracked `node_modules` symlinks; the prebuilt WASM copied from WT/f2a-u7's ignored `public/` (hashes equal I67's runtime record). Nothing built for WASM.
- Targets in `WT/targets/rv94/` (registered and Stale PP builds in separate target dirs); logs and scripts in `WT/scratch/rv94_u7_01/`.
- Default toolchain (rustc 1.97.1 `8bab26f4f`), `--locked --offline`, `CARGO_BUILD_JOBS=4`, `RUST_TEST_THREADS=2`, one cargo job at a time; Python 3.13.14 (the repository venv) with `TMPDIR` and `--basetemp` under scratch and `-p no:cacheprovider`; Node 24.18.0, Vitest 4.1.10, tsc. The memory guard (PID 5387) was checked before every job.
- No Git writes or index operations (reads with `GIT_OPTIONAL_LOCKS=0`), no installs, no new tooling, nothing native, solver-at-scale or DEC-025, nothing written to WT/f2a-u7. Every job's `TMPDIR` (and pytest's `--basetemp`) was under scratch, and every log was redirected there. **Disclosure:** the agent harness itself keeps a few lines of each background command's stdout in its own task files under the system temp directory; no job output, log or temporary file of mine went there. Copies and targets deleted afterwards (§Records).

**My oracles (the implementers' oracle and tests were only run, never used as expectations):**
- `rv94_oracle.py`, stdlib only, written from C1 WIRE_CONTRACT.md:160 and :162, D2 DESIGN.md §4.9.4 (the standing table) and §4.9.9 (withheld), D-U6-1 and D-U7-4. Rules: reader `numerical_eligible` = gates pass ∧ invocation ∧ `MECHANICS_SOLVED` ∧ every case `selected|not_required`; carrier token = `unsupported` if a gate fails, else `numerically_eligible` iff the reader rule ∧ requested refs equal the receipt's case order ∧ every `not_required` case ordinarily eligible (per-case `checks_passed`, `passive_model_basis`, `represented_equations_retained`, `accuracy_evidence ∈ {not_claimed, reference_verified}`, non-empty evidence refs resolving to unique ids); summary Current iff that token, with the invocation's own cases, is eligible; TS standing additionally requires the live native capture. The gate outcome is an input: the shared expectation, or for my hostile not_required variants the U7 base reader's outcome (the flag reaches no gate).
- `rv94_gen.py`: 420 concrete inputs built once and read by all three languages (the committed `apply_entry` helper does only the mechanical edit-and-rehash): 15 bases and 24 must-pass entries, each with and without the invocation; 277 mutations; the 20 carrier cases; the 6 declared standing forms; 12 milestone variants (with, without, requested empty / other / duplicated / wrong ref type); 16 hostile milestone variants (mode swapped, node moved, materials dropped, `{}` invocation, three blocked statuses made hash-consistent, an unrehashed row edit); 11 hostile variants of the 07j entry (requested refs omitting or reversing the not_required case, six non-ordinary quality edits, a foreign quality basis_ref, no invocation, and the entry re-bound to the desktop-shaped invocation `{model, materials: []}`).
- Dumpers of my own in each language (`rv94_dump_py.py`, `zz_rv94_dump.rs`, `zzRV94Dump.test.tsx`), run on base and candidate; the TS dumper also delivers the live successors and the 07j entry through mocked direct and job IPC with my hostile TS variants.
- PP: I61's committed-harness sweep (`zz_i61_u3g2_sweep.rs`, 324 outputs) in four builds; the registered milestone tests with `I61_U3G2_OUT`; U5's pinned script on the live bytes with both readers.
- 30 mutants of my own, and the implementers' mutant programmes re-pointed to my lanes.

## Verdict: **PASS**

| | Count |
|---|---|
| BLOCKING | 0 |
| SHOULD-FIX | 1 |
| NOTE | 4 |

**The headline:**
- **Exactly the eligible set changes, in all three languages.** Over 420 inputs, Python, Rust and TypeScript (standing seam with and without a live capture) match my oracle on every input in both lanes: **0 mismatches**. Base→candidate, exactly 52 inputs change, only in eligibility fields (reader `numerical_eligible`/`standing`, the carrier token, `withheld`). The token becomes `numerically_eligible` on exactly 36 inputs: 13 bases, 14 must-pass entries (07j's included), the 2 carrier `…:invocation` cases, the 4 D-U7-4 forms, the 2 live milestones with their invocation, and my 07j re-bound to the desktop request shape. All 277 mutations stay `unsupported`; every gate, code, publication hash and classification is identical base→candidate in every language.
- **The three languages agree** on every input apart from the declared D-U7-4 standing difference and pre-existing G7 base-code classes. **One undeclared post-U7 difference: the summary's `withheld`** (S-1).
- **No published byte changes.** PP's 324-output sweep is byte-identical to the base: registered `9a74ff16…` (2 successor rows) and Stale `0e2db8b8…`, in my own four builds. The candidate's registered Direct entry writes `ac6986b0…` / `6cd1d249…`, U1's pins, and `u1_milestone_successor_both_modes` passes with its flipped pin.
- **TS hostile variants all fail closed:** a stale model, a case-ids-only model, the caller's model mutated after capture, a registration without capture: `needs_recompute` / `NATIVE_CAPTURE_REQUIRED`; bytes mutated after registration: `VALIDATION_REQUIRED`; a content-equal clone of the captured model and the job path: eligible. The panels refuse a live eligible successor by their explicit gate only.
- **U5 reproduces:** the base reader on the live bytes gives U5's report and log byte for byte; the candidate differs only in `reader.numerical_eligible` and `reader.standing`, per mode.
- **Mutants:** my 30: 29 killed by assertion, 1 equivalent (M09). The implementers' re-run: I67 slice F 23/24 (C02 equivalent; **C04 now killed**), I67 withheld fix 3/3, I67 slice T 135 of 136 (S28 no longer applies: its line was rewritten by `e5e1693ceb`, and W03 is the same mutation on the new line, killed), I66 15 of 17 (C2 and C5 equivalent), and I67's Python/Rust scope fences as designed.

## Findings

| # | Sev | Where (`ffe65ef203`) | Evidence | Remedy |
|---|---|---|---|---|
| S-1 | SHOULD-FIX | `P/fixtures/results/retained_precision_carrier_cases.json:784–787` (D-U7-4); TS `features/results/retainedPrecisionStanding.ts:203–206`; PY `compatibility.py:358–361`; RS `semantic_contract.rs:672–687` | **After U7 the per-case summary's `withheld` differs across languages on inputs no declared entry names.** On both D-U7-4 forms (both modes), TS gives the not-Current count **97** (`e5e1693ceb`), Python and Rust **69** (Current), because their summary is Current whenever the token with the invocation's own cases is eligible, and they have no capture or current-model input. The same split occurs whenever the requested refs differ from the invocation's cases (`ms:*:inv:req_other`: PY/RS [69], TS [97]; 07j with the not_required case omitted or reordered: PY/RS [1, 0], TS [73, 0]). Before U7 every language gave 97, so the difference is new with the switch. The case file's own note says the declared list holds "the only ruled differences … any other difference is a defect", and D-U7-4's entry has only `standing` forms. TS's 97 is pinned (I67's 4 tests); Python's and Rust's 69 on these forms is not. It fails closed in TS and nothing displays `withheld` (N-4), so it is not blocking. | Declare it: extend D-U7-4's description to name the summary, and add `summary` forms with the invocation (a vocabulary value for a Current summary is needed, since v3's `by_validated_class` means not-Current), with Python and Rust asserting their side. Or ROOT records the declaration by ruling and routes the forms to U8. Aligning Python/Rust to take the caller's requested refs is a carrier API change and belongs with public activation (N-4). |
| N-1 | NOTE | `P/apps/desktop/src-tauri/src/lib.rs:2739–2759` (`run_rule_checks`), `:2794–2836` (`qualify_rule_mechanics_with_context`) | **A consumer missing from slice A's inventory (and PLAN §1.3).** The desktop backend's rule-check gate calls RS `numerical_use_standing_with_context(envelope, model refs, invocation)` with the `sourceBlockInvocation` TS supplies. After U7 it returns `Ok` for a successor with its actual invocation (my Rust dump: the token is `numerically_eligible` on exactly the oracle's set), and row binding then follows the validated classes. No Tauri test exercises a successor (its context tests at `:5081–5130` cover source-blocks and physics). Unreachable in product today: TS `ruleCheckService.ts:122–135` requires `hasNativeMechanicsInvocation` and eligible standing first, and no native path delivers a successor (F-1). I did not build the Tauri crate (a native app build). Also unlisted but unaffected: `resultExportAdapter.ts:173` and `StressNeutralExportPanel.tsx:502` (the shared refusal throws first), `LoadReferenceStatesBlock.tsx:66` (load/reference routes only), `HistoricalRunContext.tsx:283` (saved bytes never register). | Add the Tauri gate to the public-activation checklist (D-U7-1) with a backend successor test then. No U7 change. |
| N-2 | NOTE | TS `features/results/retainedPrecision.ts:1307` ("eligibility as in C1:160"); TS test helper `retainedPrecision.test.ts:288` (`c160`) | **The reader's own eligibility is necessary, not sufficient, and the TS docstring says otherwise.** My two gate-passing 07j variants with a non-ordinary `not_required` case (`evidence_refs` empty; an unresolved ref) read `numerical_eligible: true` (Python and TS `standing: "eligible"`) in all three readers, while all three carriers read `needs_recompute`, exactly as D-U6-1 and D2 §4.9.4 split it and as my oracle predicts. Requested refs and the not_required conjunct of C1:160 live in the carriers, not the reader. No product code reads the reader's eligibility directly (grep: only the three carriers), so nothing misbehaves. The PY and RS reader texts state the reader rule correctly. | Reword the TS docstring as PY/RS do, adding that standing comes from the carriers (D2 §4.9.4); optionally rename `c160`. Optionally add one of these statements to the shared corpus (reader eligible, carrier `needs_recompute`), so the carriers' not_required conjunct is exercised on a real statement and not only through seams. |
| N-3 | NOTE | Scope sentence, `retained_precision_carrier_cases.json:4`; TS reader G7 (`retainedPrecision.ts:1324–1325`) | **A pre-existing G7 code class not covered by the scope.** A 07j statement whose not_required quality has an out-of-vocabulary `accuracy_evidence`, `structural_status` or `model_matrix_fidelity` is refused at G7 by all three readers, but Python and Rust report `SOURCE_NUMERICAL_CASE_INVALID` and TS `SOURCE_PRODUCER_CONTRACT_UNSUPPORTED` (TS's G7 dispatch check fires first). Identical at the base, so not U7's. "G7 parity compares the reader's (gate, code)" overstates parity for this class, as it did for blocked envelopes before U7 declared them. | Route to U8 or wider F2a: a scope clause naming the class and each code, or align TS's G7 code. |
| N-4 | NOTE | PY `compatibility.py:358–361`, RS `semantic_contract.rs:672–687` (ROOT's note 4) | **Confirmed, and no consumer displays it.** With requested refs other than the invocation's cases, Python and Rust read the standing `needs_recompute` and the summary Current (69). Python and Rust have no non-test caller of `classification_summary`; TS's one product caller (`knownSemanticLimitations.ts:168`) passes no model and shows only the absolute and not-covered counts, never `withheld`. | Before public activation, bind the Python/Rust summary's Current flag to the caller's requested refs (as TS now does through its standing), or document that it is relative to the invocation's own cases. See S-1 for the cross-language declaration. |

## 1. Exactly the eligible set changes

**Oracle comparison** (`evidence/compare/cmp_{py,rs,ts}.txt`; dumps in `evidence/dumps/`):

| Language | Inputs | Mismatches with my oracle (base / candidate) | Changed base→candidate | Token eligible (candidate) |
|---|---|---|---|---|
| Python | 420 | 0 / 0 | 52 | 36 |
| Rust | 420 | 0 / 0 | 52 | 36 |
| TS reader | 420 | 0 / 0 | 52 (reader fields) | — |
| TS seam, live capture held | 418 (the 2 wrong-ref-type variants are inexpressible: TS's requested refs are always load-case refs) | 0 / 0 | 36 (token, status, `withheld`) | 36 |
| TS seam, no live capture | 418 | 0 / 0 | 36 (finding `NOT_NUMERICALLY_ELIGIBLE` → `NATIVE_CAPTURE_REQUIRED` only) | 0 |

- **Changed but not eligible (16):** the reader becomes eligible and the token stays `needs_recompute`: requested refs empty / other / duplicated / wrong type / omitting or reversing the not_required case, and the two non-ordinary not_required statements. The summary follows the oracle in each (Current only where the token with the invocation's own cases is eligible).
- **Unchanged:** every no-invocation input; every mutation (277, all `unsupported`, each with the corpus's first failure apart from Rust's pinned per-reader G7 code on `g7_maximum_off_enclosure`); every refused carrier case; every hostile invocation (mode swapped, node moved, materials dropped: G8 `INVOCATION_MISMATCH`; `{}`: G3) and every hash-consistent blocked statement (G7).
- **Gates and classes:** in each language the reader's gate, code, `invocation_bound`, publication hash, class counts and class digest, and the summary's class counts, are identical base→candidate on all 420 inputs. Only eligibility fields move.
- **The suites** (candidate; base in brackets): Python retained (contract, schema, carriers) 460 passed [457: +1 for 07j's entry, +2 for I66's solved-status pin]; `result_export` 169 ok [168]; Vitest 3,542 passed in 138 files [3,494], tsc clean [clean]; PP registered 705 ok, 1 failed (t13, known), 10 ignored (one is my appended scratch sweep test) [identical, test by test]; PP Stale identical to registered; runner/headless 85 ok, 2 failed (the known `load_reference` pair) [identical]. Vitest: no test changes outcome; the 20 base-only titles are describe-block renames, each mapped to a candidate title (`evidence/suites/ts_base_vs_cand_titles.json`); 48 tests are new.

## 2. The three languages agree; D-U7-4

- **Candidate, Python vs Rust vs TS reader:** identical acceptance, gate, `invocation_bound`, `numerical_eligible` and publication hash on all 420 inputs. Codes differ only in Rust's pinned G7 codes (blocked envelope `…_BLOCKED_ENVELOPE`, declared by the U7 scope clause; `g7_maximum_off_enclosure` `…_EXTREMA_BOUNDS`, the 06b per-reader G7 entry) and in N-3's class (pre-existing).
- **Tokens:** Python and Rust tokens are identical on all 420; TS's seam token with a live capture equals them on all 418 inputs TS can express.
- **D-U7-4 is truthful.** Through mocked IPC (`evidence/dumps/ts_cand.jsonl.ipc.jsonl`), with the live successors: registered without capture and a stale same-ids model both read `needs_recompute` / `RETAINED_PRECISION_NATIVE_CAPTURE_REQUIRED` in TS, while Python and Rust read `numerically_eligible` with the invocation argument. Every TS consumer of eligible standing does require `hasNativeMechanicsInvocation` (session Current `resultsSessionState.ts:76`; rule checks `ruleCheckService.ts:122`; both T6 panels), as the entry says. **It is pinned:** TS's structure test writes the TS side literally; Python and Rust assert `numerically_eligible` on both forms, read `capture` and `current_model_edits` explicitly, and reject unknown fields (I66's D3–D6 and I67's D03–D11, A01–A02 re-run, all killed).
- **Not declared:** the summary difference (S-1).

## 3. No published byte changes

| Build | Base `3f5fca3010` | Candidate `ffe65ef203` |
|---|---|---|
| Registered sweep (324 outputs) | `9a74ff16d42c2b5a…` | `9a74ff16d42c2b5a…` (2 `successor` rows, `Some((Registered, None))`) |
| Stale sweep | `0e2db8b89745fdf1…` | `0e2db8b89745fdf1…` (no successor) |

Both equal I66's and I61's recorded hashes. The candidate's registered `u3g2_direct_entry_publishes_the_pinned_successor` wrote `ac6986b0…` (sparse) and `6cd1d249…` (dense), byte-identical to U1's pins and to the carrier fixtures my 420 inputs use; `u1_milestone_successor_both_modes` passes with `invocation_bound && numerical_eligible` (133 retained-filtered lib tests ok, 9 ignored). The RS flag is read only at `retained_precision.rs:4309`; PP's precommit (`lib.rs:3161`) uses only `Ok`/`Err`.

## 4. Consumers

- **T6 panels.** On a live, eligible successor (direct and job, both modes, and 07j re-bound), every other conjunct of `liveStressBinding` holds (live capture, current contract, eligible standing) and it returns `null`: only the gate `loadReferenceOutputRefusal(result) !== null` closes it. Removing either panel's gate is killed (my M27, M28; I67's G01–G03). The stress-neutral panel now shows the shared refusal text for a successor; load/reference routes show the same text as before.
- **TS rule checks.** `retainedPrecisionInvocation` is the captured invocation exactly when standing is eligible, and `null` on every hostile variant.
- **TS summaries.** `withheld` is 69 only when TS standing is eligible, 97 otherwise, and `[]` without a valid registration, per the oracle (S-1 for the cross-language side).
- **Packager and D-U6-9.** `P/core/handoff/stress_neutral/package_v0_3.py` and `PY/records.py` are unchanged and read no eligibility. On the candidate's live successor, the packager's dispatch gives the successor id, which `SUPPORTED_METHODS` excludes (`package_v0_3.py:405–406`, `SN-SOURCE-METHOD-UNSUPPORTED`), and the 0.1.0 wrapper refuses with `ANALYSIS_RETAINED_PRECISION_DOWNGRADE_FORBIDDEN` (`records.py:90–96`).
- **Runner binding stays inert.** `result_envelope_binding.rs:258` would accept a successor with its invocation, but the only retained headless route takes `into_parts()` (`runner/headless/src/lib.rs:774`), which drops a successor, and its admission refuses W1.
- **The derivative, AnalysisRun builders, row binding and transports** read classes or receipts only; their dump fields and suites are unchanged.
- **Inventory completeness** (ROOT's question): slice A's pin inventory plus I67's one addition is complete for the committed suites, since every suite passes and no outcome changed except by mapped renames. The consumer inventory missed the Tauri rule-check gate (N-1) and four unaffected TS consumers.

## 5. D-U7-6 and stale claims

- The scope sentence (asserted in all three languages), Python's reader docstring ("Hashes bind the supplied statements; they do not establish producer origin (D-U7-6)"), RS's existing `retained_precision.rs:4271`, and TS's standing-text tail ("…it does not establish which producer made them") all say eligibility is a property of the statement and its invocation. No text I found claims producer origin.
- **Stale "held" claims are gone.** A search of P outside `execution/` for "until U7", "U7 owns", "eligibility stays/remains/is held/off", "held withheld", "intentionally rejects" and "never numerically eligible" finds only non-retained identities (load/reference T1, transport) and the historical `loop/LOOP_RECEIPTS.md`. The 9 corpus `qualification` strings now read "the public reader accepts it (D-U6-1); its eligibility is its expected value (U7)".
- N-2's TS docstring is an overstatement, not a held claim.

## 6. Nothing weakened

- **Pins:** I read every changed test line. Each removed assertion is replaced by the same assertion with its 07i/07j or oracle value; the TS structure, scope and v4 checks gain assertions. No test is deleted: Vitest shows 0 status changes and 20 mapped renames; Python's retained outcomes differ from the base only by the two renamed `test_standing_…` cases and three additions; Rust `result_export` only by two renames and one addition; PP and runner outcomes are identical test by test.
- **Readers:** the reader diffs are the flag token and comments/docstrings only. The carriers gain TS's live-capture conjunct and the withheld condition (both stricter); the panel gate moves from `isLoadReferenceRoute` to the shared refusal (a superset). No gate is relaxed.
- **Line-neutrality:** RS `retained_precision.rs` 4,402 and `semantic_contract.rs` 855 lines, base = candidate; PP `lib.rs` 24,333 = base; hunks `-4264,9 +4264,9`, `-578,8 +578,8`, `-2232,7`, `-2251,7`, `-3153,7`. The premise pins `:4252`, `:4253`, `:4305` are byte-identical. (Pass B itself is RV89's.)
- **N-6 docs** are accurate: `into_publication()` returns the successor when present, else the ordinary envelope; `into_parts()` drops it.

## 7. Mutants

**My 30** (`evidence/mutants/mutants_rv94_{py,rs,ts}.json`; one exact edit each in a lane checked pristine before and after; a kill is a failing test, never a compile or load error):

| Area | Python | Rust | TypeScript |
|---|---|---|---|
| Flag reverted alone | M01 killed | M11 killed | M21 killed |
| Reader: no invocation conjunct | M02 killed | M12 killed | M22 killed |
| Reader: case-status conjunct dropped entirely | (I66 C3, re-run: killed) | (I66 C6, re-run: killed) | (I67 C03, re-run: killed) |
| Reader: `selected` only (C04's narrowing) | M03 killed | M13 killed | M23 killed |
| Reader: admits `unavailable` | M04 killed | M17 killed | M36 killed |
| Reader: no `MECHANICS_SOLVED` | **M09 survives (equivalent)** | (I66 C5) | (I67 C02) |
| Carrier: no requested-refs check | M05 killed | M14 killed | M33 killed |
| Carrier: no not_required ordinary eligibility | M06 killed | M15 killed | M34 killed |
| Summary Current from the reader's flag alone | M07 killed | M16 killed | — |
| N-2: null live passes / live check removed / always live / live ignores the model | — | — | M24, M25, M26, M37 killed |
| N-5: stress-neutral gate / result-export gate removed | — | — | M27, M28 killed |
| Status string equals the token / refused token read as `needs_recompute` (token vs status comparison) | — | — | M30, M35 killed |
| Withheld ignores the live capture | — | — | M32 killed |

**M09 is equivalent** (ROOT's first note, I66's claim): a hash-consistent non-solved milestone is refused at G7 by every reader, with evidence (`…_EVIDENCE_INVALID` / Rust `…_BLOCKED_ENVELOPE`) and without it (Python probe: `SOURCE_PREVIEW_PHYSICS_EVIDENCE_REQUIRED`), so no gate-passing statement is unsolved; my Python dump with M09 applied is byte-identical to the candidate's on all 420 inputs. The carriers keep their own independent `MECHANICS_SOLVED` conjunct.

**The implementers' mutants, re-run in my lanes** (`evidence/impl_mutants/`: their programmes with only the lane, pristine-source, target, output and temp paths re-pointed):

| Programme | Result |
|---|---|
| I67 slice F (`mutants_f.py`, 24) | 23 killed by assertion; **C02** survives (equivalent, as M09); **C04 is killed** by `must pass: not_required_second_case_checks_passed` (07j closes I61's C04) |
| I67 withheld fix (`mutants_f2.py`, W01–W03) | 3 / 3 killed |
| I67 slice T (`mutants_r5.py`, 136: B01–B05, G01–G03, T01s–T03s, Q01–Q02 and the 123 U6d mutants re-pointed) | Control 407 passed. The 13 U7 mutants: 13 / 13 killed by assertion. The other 123: 122 killed by assertion; **S28 does not apply** at this head (its line was rewritten by `e5e1693ceb`; W03 is the same mutation there, killed) |
| I66 slice F (`mutants.py`, 17: F1–F2, C1–C8, D1–D7; data mutants run in both phases) | Python phase 11 of 12 killed, **C2** survives (equivalent); Rust phase 11 of 12 killed, **C5** survives (equivalent); D1–D7 killed in both languages |
| I67 scope fences (`pyrs_mutants.py`) | Control passes; P1 (clause removed) and P4 (D-U7-6 removed) killed in both; P2 (Python's code misstated) killed in Python only and P3 (Rust's) in Rust only, as designed |
| I61 C04 (`mutants_c04.py`) | Not re-run as a script: its three edits are my M03, M13 and M23, all killed on 07j |

## ROOT's notes

1. **I66's equivalent mutant:** confirmed (§7).
2. **Summary vs standing with other requested refs:** confirmed in Python and Rust; no consumer displays it (N-4); the cross-language side is S-1.
3. **TS's IPC harness refusing 07j at G8 is a harness limit:** confirmed. The corpus invocation is `{request: {model}, solver_mode}`; the desktop always captures `{request: {model, materials: []}, solver_mode}` (`previewService.ts:108`), so G8's invocation hash cannot match. Re-binding 07j to the desktop shape (one invocation edit, rehashed) makes it pass through mocked direct and job IPC, stand `numerically_eligible` / `integrity_checked` with `withheld` [1, 0], and fail closed on every hostile TS variant.
4. **Slice A's inventory:** pins complete with I67's addition; consumers missed the Tauri gate (N-1).
5. **U5 on live bytes:** with the base reader, the pinned `u5_compare.py` (`df4684d3…`) on RV86's extract reproduces U5's report (`b8546c97…`) and log (`271eeeee…`) byte for byte; with the candidate reader the report (`c4d0fc45…`, equal to I61's slice-L report) differs only at `modes[0..1].summary.reader.numerical_eligible` (false→true) and `.standing` (needs_recompute→eligible).
6. **The `liveStressBinding` export:** acceptable. Its comment (`StressNeutralExportPanel.tsx:83`) says why; it is a pure function of its arguments that returns the checked text or `null`, so exporting it grants nothing; it has no product caller outside the panel, and the module already exports non-component helpers. It is the seam that shows the gate is the only thing closing the binding (§4).

## Limits

- The Python 24-file sweep was not re-run (it needs `examples/` and `execution/` handoff fixtures outside my archive); the diff touches no Python file outside the reader, its carrier docstring and the retained tests, which I ran.
- The Tauri crate was not built (N-1).
- G7 Pass B on the U7 head is RV89's (taken as an input, PASS).

## For ROOT to rule

1. **S-1:** declare the post-U7 summary difference (D-U7-4's description plus summary forms, or a ruling), or direct an alignment.
2. **N-1:** whether the Tauri rule-check gate joins the public-activation checklist.
3. **N-3:** route the G7 code class to U8 or wider F2a.

## Records

`evidence/` (placeholder paths only): the generator, oracle, dumpers, comparers and mutant runner; the base and candidate dumps for each language (the 55 MB input file is not kept: `inputs_sha256.txt` and the generator reproduce it); the TS IPC scenario dumps; the comparison outputs; my mutant results and the re-run implementers' results; the four sweep TSVs and the two live successor hashes; the U5 reports and logs; suite outcome lists. `SHA256SUMS` covers this folder. **Cleanup done:** `WT/rv94/` (all copies and lanes) and `WT/targets/rv94/` are deleted; the scratch logs in `WT/scratch/rv94_u7_01/` remain (the 55 MB input file and temp folders removed).
