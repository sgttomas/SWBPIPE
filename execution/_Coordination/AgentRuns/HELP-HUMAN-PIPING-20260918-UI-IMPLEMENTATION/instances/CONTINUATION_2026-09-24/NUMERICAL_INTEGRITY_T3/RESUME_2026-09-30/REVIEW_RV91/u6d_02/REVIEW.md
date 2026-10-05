# RV91 confirmation: I67's U6d repair round (02)

RV91 is a TASK (Type 2) dispatched directly by ROOT (HELP_HUMAN, Agent 0). ROOT is the return path, and nothing was delegated. This confirms the round-01 findings (`R/REVIEW_RV91/u6d_01/REVIEW.md`, sha256 `67c7155f…`) against the repair round, with my round-01 context.

- **Request:** ROOT's message of 2026-10-04 to RV91 (confirm items 1–5); RR "RV91 on U6d: PASS; the U6d repair round granted…", "RV88 on U6a, U6c, U6b (and U6d)…" and "U6d repair round verified and committed".
- **Read:** I67's `R/I67/u6d_typescript_02/RETURN.md`; RV88's `R/REVIEW_RV88/u6d_01/REVIEW.md` (its N-3).
- **Candidate:** `968adb44fe` on `codex/piping-f2a-carriers-ts-20261004`, parent `9555b6ffc2`. 4 files, all U6d's: `analysisRunCompatibility.ts`, `retainedPrecisionStanding.ts` and their two test files (+108/−12).
- **Host:** as round 01. `git archive` lanes under `WT/rv91/` (cand2; mut2, an APFS clone; the round-01 merge lane updated). `node_modules` linked, WASM copied (same hashes), nothing installed or built, no Cargo, nothing native, solver or DEC-025, Git reads only. The memory guard (PID 5387) was running. `TMPDIR` was `WT/scratch/rv91_u6d/tmp`. `_run_records/basis.txt`.
- **When:** 2026-10-04, about 11:45Z to 12:00Z.

## Verdict: CONFIRMED

**Counts: 0 BLOCKING · 0 SHOULD-FIX · 1 NOTE** (new, test strength only; it does not reopen SF-1).

| Item | Result |
|---|---|
| 1. SF-1 | **Confirmed.** `buildAnalysisRunV02` refuses a `retained_precision` member (object, null, `{}`) and W1 token rows (first or last row) with `ANALYSIS_LEGACY_SOURCE_DOWNGRADE_FORBIDDEN`, on both legacy schema versions (0.1.0 and 0.2.0). |
| 2. N-3 | **Confirmed.** RV08 and RV09, and RV03 and RV04 too, are killed by committed tests alone. |
| 3. N-4 and RV88 N-3 | **Confirmed.** "Selected cases" appears only for a validated registration, and the refused-this-session text is truthful. |
| 4. The F1 pin | **Confirmed TS-local.** The shared file is unchanged (`952e39bf…`), with no `declared_differences` section on `968adb44fe` or on the carriers tip `924c6284cb`. |
| 5. Nothing else changed | **Confirmed.** Vitest 3,431/3,431 and `tsc` clean. All 3,417 tests of `9555b6ffc2` keep their outcome. My sweep is identical on all 69 existing-identity envelopes; the 17 successors change only in the intended panel text. |

## 1. SF-1: the historical v0.2 builder

**The code** (analysisRunCompatibility.ts:84): after its unchanged first refusal, `buildAnalysisRunV02` throws `ANALYSIS_LEGACY_SOURCE_DOWNGRADE_FORBIDDEN` when `retainedPrecisionDowngrade(result)`. That is the dispatch guard: a `retained_precision` member of any value, or any row carrying the W1 token. Its successor exemption is unreachable here, because a source with a producer is already refused at :82. The code is Python's twin (U6b's `compatibility.py:85–87` list).

**My probe** (`_run_records/r2_probe.json` `sf1`). I built legacy shapes from the milestone (producer, quality, formulation, evidence, receipt and tokens removed) at **both** 0.1.0 and 0.2.0:

| Source | Result |
|---|---|
| Control | builds, without a receipt |
| Receipt member: object, `null` or `{}` | `ANALYSIS_LEGACY_SOURCE_DOWNGRADE_FORBIDDEN` |
| W1 token on the first row, or on the last row | `ANALYSIS_LEGACY_SOURCE_DOWNGRADE_FORBIDDEN` |
| Another method string on a row | builds, as in Rust's row guard, which matches the W1 token only |
| The successor itself | `HISTORICAL_ANALYSIS_SOURCE_UNSUPPORTED` (unchanged) |

I67's tests (retainedPrecisionAnalysisRun.test.ts:160–) cover the object, null and first-row token forms on a 0.1.0 fixture, a control, and the unchanged refusal.

## 2. N-3: committed tests kill RV08 and RV09

`_run_records/mutants_r2.*`. These ran in the mutant lane against the **committed tests only**: I67's 3 U6d files and 8 related files, with no RV91 probe. The control passes 310/310.

| Mutant | Result | Killed by |
|---|---|---|
| RV08: binding selects the successor by route, not producer id | killed | "the successor's own header requires…" (the nine header edits, rows refused `RULE_QUANTITY_NOT_COVERED`; :508) |
| RV09: the fingerprint is taken after the reader's await | killed | "registration binds the bytes captured before the reader's await…" (:270, both modes) |
| RV03: registration keeps the caller's live invocation | killed | "the registered invocation is a private copy…" (:285) |
| RV04: the registered invocation is handed out by reference | killed | as above |
| RVb3 (new): the v0.2 guard removed | killed | the SF-1 tests |
| RVb4 (new): the case count leaks into the refused text | killed | the standing-text tests |
| RVb5 (new): the case count leaks into the unvalidated text | killed | the standing-text tests and the N-4 two-case test |
| RVb1 (new): the v0.2 guard is applied to schema 0.1.0 only | **survives** | killed by RV91's probe (see N-1) |
| RVb2 (new): the v0.2 token is checked on the first row only | **survives** | killed by RV91's probe (see N-1) |

## 3. N-4 and RV88 N-3: the standing text

retainedPrecisionStanding.ts:195–203 now returns early for an unvalidated or refused outcome, and computes the case count only on the validated branch. My probe (`r2_probe.json` `texts_*`, both modes) checks both the function and the rendered ResultsPanel.

| State | Text |
|---|---|
| Registered | "…validated by the retained-precision reader against the actual invocation for these exact bytes. Selected cases: 1 of 1. …" |
| Two-case corpus statement (`two_case_facade_after_certificate_synthetic`), after registration | "Selected cases: 1 of 2." |
| Two-case statement before registration | no count |
| Unregistered (a copy) | "…not validated for these exact bytes in this session (saved, reference or copied data never register); needs recompute. …" with **no count** |
| Refused in this session: an edited row | "…refused by the retained-precision reader (`RETAINED_PRECISION_RECEIPT_MISMATCH`); unsupported, values shown for inspection only. …" |
| Refused in this session: the foreign mode | the same text, with `RETAINED_PRECISION_INVOCATION_MISMATCH` |

- The refused texts carry **no count** and no "historical". They name the reader's actual code, which is also the standing finding.
- "values shown for inspection only" is accurate for a delivery refused in this session: it gets no registration, no binding, no export and no rule check.
- No text names the stop-rule bound or enclosure.

## 4. The F1 pin

**What the pin asserts.** "the declared parity difference F1 (pinned in TS only)" (retainedPrecisionIntegration.test.tsx:515–) runs in both modes, for an edited covered row and for `numerical_quality` rewritten to `checks_passed`:
- the reader refuses with `RETAINED_PRECISION_RECEIPT_MISMATCH`;
- `retainedPrecisionStanding` and `numericalResultStanding` read `needs_recompute`/`RETAINED_PRECISION_VALIDATION_REQUIRED`, not eligible;
- binding refuses with `RULE_QUANTITY_NOT_COVERED`.

These are exactly round 01's observations (`u6d_01`, extra cases), and the comment cites the ruling.

**The shared file is unchanged.** `retained_precision_carrier_cases.json` at `968adb44fe` is byte-identical (`952e39bf…`), and neither `968adb44fe` nor `924c6284cb` has a `declared_differences` section. So the switch to I66's shared section is still to come, as ROOT says.

## 5. Nothing else changed

**Suites** (`_run_records/*_summary.txt`, `compare_r1_r2.txt`).
- `968adb44fe`: Vitest **3,431/3,431** (138 files), and `tsc` exits 0 with empty output. This equals ROOT's run.
- Against `9555b6ffc2` (round 01, 3,417/3,417): 0 tests missing, 0 outcome changes, and 14 new tests (9 integration, 5 AnalysisRun).
- **Merge preview:** the carriers tip `924c6284cb` plus U6d's files at `968adb44fe` (no file overlap) gives **3,443/3,443**, `tsc` clean.

**My sweep** (`sweep_compare_r2.txt`; the same test and the same fixed inputs as round 01, `9555b6ffc2` against `968adb44fe`).
- All 86 envelopes were compared.
- All 69 existing-identity envelopes are identical on every outcome, including AnalysisRun v0.2, registration through mocked IPC, the panel render and reopen.
- The 17 successors differ **only** in the rendered standing text, which is the intended N-4 removal of "Selected cases: 1 of 1." from the unregistered text. Every other successor outcome is identical, including dispatch, standing findings, binding, notices, output refusal, AnalysisRun v0.3 and reopen.

## Finding

### N-1 (NOTE, new): the SF-1 tests pin one legacy schema version and the first row only

- **Where:** retainedPrecisionAnalysisRun.test.ts:160–; the guard at analysisRunCompatibility.ts:84.
- **Evidence:** RVb1 (the guard applied to 0.1.0 only) and RVb2 (the token checked on the first row only) survive the committed tests, because the tests use a 0.1.0 fixture with the token on row 0. My probe kills both: it uses a 0.2.0 legacy shape and a last-row token (`probe_vs_mutants.json`).
- **Impact: none today.** The product code is correct. It calls the shared dispatch guard, and the any-row behaviour is pinned elsewhere (I67's N04, `some` to `every`, is killed in I67's runs; I did not rerun it).
- **Remedy (optional, test only, any later round):** add a 0.2.0 legacy shape and a non-first token row to the SF-1 tests.

## Records

`_run_records/` (placeholder paths only; SHA256SUMS covers this folder):
- `basis.txt`;
- the candidate and merge suite summaries, and the per-test comparison;
- the sweep comparison and the compressed sweep output;
- the round-2 probe, its output and log;
- the round-2 mutant runner, results and log;
- the probe-versus-mutant check.
