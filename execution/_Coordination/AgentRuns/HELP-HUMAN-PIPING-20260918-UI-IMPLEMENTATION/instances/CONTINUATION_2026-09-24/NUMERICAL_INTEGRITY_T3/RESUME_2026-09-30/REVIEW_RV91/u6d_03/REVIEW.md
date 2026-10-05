# RV91 confirmation: I67's U6d follow-up (round 03)

RV91 is a TASK (Type 2) dispatched directly by ROOT (HELP_HUMAN, Agent 0). ROOT is the return path, and nothing was delegated. This is a short confirmation made with my round-01 and round-02 context (`R/REVIEW_RV91/u6d_01/`, `u6d_02/`).

- **Request:** ROOT's message of 2026-10-04 (items 1–4); RR "I66's U6 repair round committed; U6d merged…" and "U6d follow-up (round 03) verified and committed…".
- **Read:** I67's `R/I67/u6d_typescript_03/RETURN.md`.
- **Candidate:** `76477534f6` on `codex/piping-f2a-carriers-20261004`, parent `52052ece61` (the U6d merge). It changes 3 files:
  - `knownSemanticLimitations.ts`, the only product file;
  - two U6d test files.
  
  The shared case file is unchanged by the round (`21b158de…`, format v2).
- **Host:** as before. `git archive` lanes `WT/rv91/{cand3, base3}` and an APFS-clone mutant lane `mut3`. `node_modules` is linked and the WASM copied (same hashes); nothing was installed or built. No Cargo, nothing native, solver or DEC-025; Git reads only. The memory guard (PID 5387) was running, and `TMPDIR` was `WT/scratch/rv91_u6d/tmp`. `_run_records/basis.txt`.
- **When:** 2026-10-04, about 12:40Z to 13:05Z.

## Verdict: CONFIRMED

**Counts: 0 BLOCKING · 0 SHOULD-FIX · 0 NOTE.**

| Item | Result |
|---|---|
| 1. My round-02 N-1 | **Confirmed.** The SF-1 tests now run on both legacy shapes, 0.1.0 and 0.2.0, with a token on the last row only. **Both surviving mutants are now killed by committed tests:** RVb1 (0.1.0-only) by the 0.2.0 object test, and RVb2 (first-row-only) by the last-row token test. |
| 2. Case format v2 and `declared_differences` | **Confirmed.** TS pins `I66-U6-CARRIER-CASES-v2`, 20 cases and 4 fixtures, and reads F1 and F2 (and F-U6b-2 and F5) from the section, which holds exactly the four ruled entries. Every `typescript` expectation equals what I observed independently in rounds 01–02, and my own v2 consumer agrees on all 20 cases and all 8 (entry, fixture) pairs. |
| 3. The unit table | **Confirmed.** The nine entries equal Rust's `derivative::si_unit` exactly. An unknown unit (11 probed, including `toString`, `constructor` and `__proto__`) claims no bound and is labelled `N_RP_NOT_COVERED`, as Rust's `None` branch does. The only product change is this table and `retainedAbsoluteNotice`. |
| 4. No regressions | **Confirmed.** Vitest **3,466/3,466** and `tsc` clean, as in ROOT's run. My sweep against `52052ece61` is identical on all 86 envelopes. |

## 1. N-1: the SF-1 tests

`_run_records/mutants_r3.*`. The mutants ran against the **committed tests only**: I67's 3 U6d files and 8 related files, with no RV91 probe. The control passes 333/333.

| Mutant | Result | Killed by |
|---|---|---|
| RVb1: the v0.2 guard is applied to schema 0.1.0 only | killed | "a 0.2.0 legacy-shaped source carrying a retained_precision object is refused…" |
| RVb2: the v0.2 token is checked on the first row only | killed | "a 0.1.0 legacy-shaped source carrying a W1 token on the last row only is refused…" |
| RV08 (round-01 regression check) | killed | the header-edit test |
| RV09 (round-01 regression check) | killed | the edit-during-await test |
| RVc1 (new): an unknown unit prints b in the published unit (the pre-round-03 behaviour) | killed | "the bound is printed upward with three significant digits" |

The 12 SF-1 tests cover:
- **both shapes:** a control that builds without a receipt;
- **refused:** an object, `{}`, `null`, a first-row token and a last-row-only token, each with `ANALYSIS_LEGACY_SOURCE_DOWNGRADE_FORBIDDEN`.

My round-02 probe (both shapes, first and last row, another method string, the successor itself) still passes unchanged on `76477534f6` (`r2_probe_on_r3.*`), and so do its standing-text checks.

## 2. Case format v2 and the declared differences

**My own consumer** (`zzRV91R3.test.tsx`, `r3_probe.json`), independent of I67's harness.
- **The 20 cases.** The fixtures are sha-checked. Edits are applied literally. A case with an invocation is captured through mocked IPC.
  - **All 20 match** `expected_standing` and `expected_dispatch`, and none is eligible.
  - The 6 new cases are the token on the last row only, a `{}` member and a `null` member, on the raw legacy 0.1.0 and preview-physics-1 fixtures. All 6 read `unsupported` with `RETAINED_PRECISION_DOWNGRADE_FORBIDDEN`.
  - Unedited, the raw fixtures read `legacy` and `preview_physics`.
- **The declared differences.** The ids are exactly the four ruled entries, and each has exactly `rust`, `python` and `typescript` expectations. The `typescript` expectations hold on both milestones:

| Entry | TS expectation | Observed (round 03) | My earlier evidence |
|---|---|---|---|
| I67-F1, unregistered invalid statement | `needs_recompute`, `RETAINED_PRECISION_VALIDATION_REQUIRED` | equal; not eligible | round 01, extra cases (`u6d_01` N-1) |
| I67-F2, display-only binding precheck | `every_row:RULE_QUANTITY_NOT_COVERED`, notice `N_RP_UNVALIDATED` | equal: all 98/99 rows refused, and every precheck notice is `N_RP_UNVALIDATED` | round 01: unregistered TS refuses 98/99 rows, and the precheck shows `N_RP_UNVALIDATED` |
| F-U6b-2, Python refuses transport | `ok` | equal: header route, then `validateRetainedPrecisionTransport` | new in this round |
| F5, refused-statement binding | `every_row:RULE_QUANTITY_NOT_COVERED`, `N_RP_UNVALIDATED` | equal | round 01 (refused statements have no classes) |

**I67's committed consumer** asserts the same entries. It also checks `eligible: false` for every entry and keeps the extra `numerical_quality` → `checks_passed` input under F1. The retired TS-local F1 pin asserted that the reader refuses the edited statement. That is still asserted by the shared `edited_row` cases (dispatch `RETAINED_PRECISION_RECEIPT_MISMATCH`) and by the extra F1 input, so nothing is lost.

## 3. The unit table

`knownSemanticLimitations.ts`: `RETAINED_SI_UNIT` is `{m: m, mm: m, rad: rad, N: N, kN: N, N*m: N*m, kN*m: N*m, Pa: Pa, MPa: Pa}`.
- **It matches Rust exactly:** these are the same nine entries as `derivative.rs:28–37` `si_unit`. I checked each mapping programmatically.
- **Unknown units:** `retainedAbsoluteNotice` reads the table with `Object.hasOwn` and returns `N_RP_NOT_COVERED` for any other unit. I probed `degC`, `mode_code`, `unitless`, `""`, `toString`, `constructor`, `__proto__`, `MN`, `m/s`, `N.m` and `N·m`. Every one claims no bound, matching Rust's `None => RETAINED_NOT_COVERED` branch.
- **Known units:** each prints the upward b in its SI unit (for example, ±1.09e−19 for 2⁻⁶³).
- **The rest of the diff** to `knownSemanticLimitations.ts` is doc comments. Nothing else in product code changed (diffstat: 3 files, 1 product).

## 4. No regressions

**Suites** (`*_summary.txt`, `compare_base3_cand3.txt`).

| Lane | Tests | `tsc` |
|---|---|---|
| Base `52052ece61` | 3,449 run: 3,442 passed, 7 failed | 0 |
| Candidate `76477534f6` | **3,466/3,466** | 0 |

- **The base's 7 failures are exactly the expected ones:** the v1 format pin and the 6 new v2 cases.
- **Against base:**
  - every test whose title is kept keeps its outcome (0 changes);
  - 29 titles are retired or renamed: the four 0.1.0 SF-1 tests, the four TS-local F1 pins, the v1 format pin, and the 20 parity cases under the old block title;
  - 46 titles are new and all pass: 12 SF-1 tests, 11 declared-difference tests, the format and raw-guard tests, and the 21-test parity block (the 20 cases plus a coverage check).

**My sweep** (`sweep_compare_r3.txt`; the round-01 sweep test on one fixed input tree, base `52052ece61` against `76477534f6`): **86 envelopes in 72 files, identical on every outcome.** That is 69 existing-identity and 17 successor envelopes, on 25 outcomes each plus the guard probe.

## Records

`_run_records/` (placeholder paths only; SHA256SUMS covers this folder):
- basis;
- base and candidate suite summaries, and the per-test comparison;
- both sweep outputs and their comparison;
- RV91's round-3 probe (with its output and log) and the round-2 probe rerun;
- the round-3 mutant runner, results and log.

**One probe fix, disclosed:** my unit probe's first run failed on my own harness, because an object literal cannot hold a `__proto__` key. It was rerun with a `Map`. The product's result is `N_RP_NOT_COVERED`.
