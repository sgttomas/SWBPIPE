# RV80 — confirmation round 03: the Rust reader on snapshot 07c

RV80 is a TASK (Type 2), dispatched by ROOT (HELP_HUMAN) for a scoped confirmation round (workflow §3). ROOT is the return path. RV80 resumes as the Rust reviewer of `reader_review_01` and `reader_confirm_02`. It did not write any of the repairs and had no descendants. T1 and T2 from I61's experiment are out of scope.

- **Candidate:** READER `a894d9d0bacc98deb0adcab215d1bc9a91ea4373`, built from a `git archive` into `WT/rv80/`. NUM is at `19065f4828`.
- **Files under review:**

  | File | sha256 |
  |---|---|
  | `retained_precision.rs` | `abacc74664d67ce34887cf86481f3ff12090bab2b40689e20e297a9f9fc2c80f` |
  | tests | `c984f49a54b6…` |
  | `lib.rs` | `375b07313518…` (unchanged) |
  | corpus 07c | `d33667719e` (15 cases, 254 mutations, 19 must-pass entries) |

  These match I63's RETURN_07B and RETURN_07C.
- **Diff reviewed:** `b36739112a..a894d9d0ba`. The source changed +105/−52; the test file changed +242/−5.
- **Run window:** 2026-10-03 19:10–19:29 MDT (WT/rv80 deleted at 19:29), inside the 60-minute box. Memory guard PID 5387 was running throughout.
- **Host:** the default toolchain only, `CARGO_TARGET_DIR=WT/targets/rv80`, `CARGO_BUILD_JOBS=4`, `RUST_TEST_THREADS=2`, `CARGO_NET_OFFLINE=true`, `--locked --offline`, one Cargo job at a time.
  - Every command ran under `env -u DEVELOPER_DIR`.
  - The mutant script now strips `DEVELOPER_DIR` from the child environment instead of setting it (`MUTANTS.py:92`).
- **No** Git writes, installs, new tooling, or native jobs.

## Verdict

**PASS — 0 BLOCKING, 0 SHOULD-FIX, 3 NOTE.**

- **Findings:** each of the four confirm_02 findings (C1–C4) is fixed as its decision (D27–D30) states, and confirmed by RV80's own probes on this head.
- **Diff:** the factored `reason_table` keeps every rule of the block it replaces. No check is removed or weakened. The only removed source conditions are each replaced by an equal or stricter check, as D20, D22 and D27 require.
- **Tests:** baseline 47/47.
- **Arithmetic:** the oracle finds 0 mismatches over 8,238 vectors.
- **Mutants:** 38 of 44 are killed. Every survivor is explained below.

## confirm_02 findings: dispositions

| Finding | Decision | Disposition | RV80 evidence on `a894d9d0ba` |
|---|---|---|---|
| RV80-C1: the idle Run rule read the WORK-derived running meter | D27 | **Fixed.** The condition is kept, but now reads the recorded `invocation_before` (RS:920–923). | PR14 now gives **G5 WORK** (was ATTEMPT). PR15, the shared idle pin, still gives G5 ATTEMPT. M36 (revert to `current`) is killed by `d27_…`, the shared pin and the 07b slice. |
| RV80-C2: D5d covered only `stop_rule` | D28 | **Fixed** for all four quantity-bearing reasons (RS:972–990, with `space == "attempt"`). | PR12 (`verification_estimate`, quantity not in the layout) now gives G5 ATTEMPT (was Ok). The new PR17 (`charge`) and PR18 (`publication_enclosure`) give G5 ATTEMPT. PR13 (`stop_rule`) is unchanged. M37 (narrowed back) is killed. See NOTE N1 on the kind domain. |
| RV80-C3: M13 survived (native `run_ref` on a nonselected Run) | D30 | **Fixed.** `d30_native_run_ref_on_nonselected_run` uses the factored `reason_table`. | M13 is **killed** by `d30_…`. PR1 and PR5 still give G5 PRODUCT_ATTEMPT. |
| RV80-C4: an empty body inventory | D29 | **Fixed and generalized:** every CaseSource needs a non-empty `body_membership` at G3 (RS:662–666). The roster-specific check (RS:736–745) remains. | PR11 gives G3 COVERAGE. M38 (general check dropped) is killed by `d29_empty_body_inventory_without_roster`. |

**My earlier findings** (S1, S2, N1–N8 from review_01) all still hold on this head: PR1–PR10 give the same outcomes as in confirm_02 (`PROBES.json`).

## Repair diff review (`b36739112a..a894d9d0ba`)

**Removed source lines, each replaced by an equal or stricter check:**
- **The inline S06 reason-table block → `reason_table(c, a)`** (RS:1881–1930, called at RS:2234 under the same guard). RV80 compared the two side by side, and they are identical:
  - the `preparation`, `native`, both `capture` and default arms;
  - the code/phase equality;
  - the observable/G5a check equality;
  - `st`, `proof`, `e` and `run` bind to the same values (`a.stages`, `a.proof`, `a.result.error`, `c.run`).

  The call site keeps its preceding `pf(cause.product_attempt_ref == ai && result unavailable)`.
- **G3 `if !a.source_ref.is_null() { at(...)? }` → `if let Ok(s) = at(...)` (D22,** RS:698–701, 736–737). A null reference is skipped as before. A dangling one now skips the source-dependent G3 checks and reaches G5 PRODUCT_ATTEMPT through `g5_products`' own `at(...)`. M43, which restores the G3 failure, is killed.
- **`current >= invocation_limit` → recorded `invocation_before >= invocation_limit` (D27).**
- **`g5_ordinary`'s `&& !c.product_attempt_ref.is_null()` → the D20 case pass in `g5_products`** (RS:1943–1952, PRODUCT_ATTEMPT in class 2, after the ordinary checks per D17). M42 is killed.

**Added checks:**
- **D19 (RS:2207–2221)** is in both halves. The unavailable half is killed by M40 and the Ready half by M41.
- **D21:** the shared build ref must be null on an escalating failed verification (RS:964–969), on top of phase 1's null `verification` and `verification_lme` 0. The third indicator (the null summary) was already present; M44 is killed.
- **D28 and D29,** as above.
- **D24:** harness only (`after_rehash`).
- **D25:** no code change; `uint` reads parsed values.

**Hooks (D14):**
- `reader_logic::reason_table` (RS:1613–1616) is pure. It returns `Result<(), ValidationError>`, with no Validation and no path to eligibility.

**Tests:**
- The only removed lines are the 236-count assertion and its comment (now 254).
- The new tests are `d19`–`d30`, plus the 07b and 07c slices.

**Fail-closed:**
- `IMPLEMENTATION_COMPLETE = false` (RS:4141), and the eligibility conjunction is unchanged.
- M01 is killed by three tests.

## Mutation testing (44 single-edit mutants)

**Method:**
- `MUTANTS.py` (with WT as its argument) runs M01–M44.
- M01–M35 are confirm_02's set: M13 and M19 are re-indented for `reason_table`, and M23 is re-expressed with D21's added conjunct.
- M36–M44 target this round: the D27 revert, D28 narrowing, the D29 general check, D21 vbuild, both halves of D19, D20, the D22 revert, and D21's third indicator.
- Each mutant starts from the pristine source (`abacc746`), which is restored and re-hashed afterwards.

**Totals:** 38 killed, 6 survived.

| Survivor | Edit | Reason |
|---|---|---|
| M02 | N17: drop `!case_over` | **No base,** deferred: a Budget terminal needs ≥20B/60B of work. |
| M06 | Relative-class `>=` → `>` | **No base:** no equality row. |
| M19 | D4d: drop the nonselected requirement | **Equivalent:** the `refused` arm (RS:1902) rejects a selected Run with the same code. |
| M27 | D1: drop the captured_prefix G5 references | **Equivalent:** the G5 stage, D4e and Ready rules give the same PRODUCT_ATTEMPT code. |
| M35 | D1: drop the origin-owner conjunct | **Equivalent for single defects:** the source-owner conjuncts give the same ATTEMPT. |
| M39 | D21: drop `verification_shared_build_ref.is_null()` at RS:969 | **Unpinned** (NOTE N2). The replay's own check (RS:1187) catches a shared build only when a fresh attempt follows (`if ai + 1 < attempts.len()`, RS:1168). The D21 test and shared pin both use the V base, where an attempt follows. A last-slot escalating failed verification (a p256 candidate whose v512 fails, leading to the Ceiling) is caught only at RS:969, and no test reaches it. |

**Killed:** M01, M03–M05, M07–M18, M20–M26, M28–M34, M36–M38 and M40–M44. M13 (D30) is now killed. M08 is now also killed by `d27_…`.

## New findings

| ID | Severity | Location | Finding | Remedy |
|---|---|---|---|---|
| RV80-N1 | NOTE | RS:976–990 | **D28 admits a `verification_estimate` reason on a translation or rotation row.** It matches a layout row by quantity, body and kind (D28's text), but natively an estimate rejection exists only for force/moment rows. The report's `w` is `Some` only for `Kind::Force | Kind::Moment` (`FK/verify.rs:878–882`), and the estimate loop skips `None` (`FK/adaptive.rs:2385–2392`). **PR16:** a p512-ladder rejection with `verification_estimate` on the existing node-1 UX translation row validates Ok. By contrast, `charge` can name displacement rows natively (`FK/verify.rs:1150–1186`), so no kind limit applies to it. Low impact: a rejected attempt's reason cannot affect a selected value. | Optionally require kind ∈ {force, moment} for `verification_estimate` reasons in all readers, with one shared pin. |
| RV80-N2 | NOTE | RS:964–969; tests | **D21's generalized conjunct is unpinned where no fresh attempt follows** (M39 survives; see above). | Add a reader-local test on a Ceiling-shaped run whose last attempt is a verification-failed p256 with a shared build. The Ceiling base is deferred, so this is reader-local only. |
| RV80-N3 | NOTE | RS:1613–1616 | D14: one more hidden hook, `reason_table`. It is pure and has no path to eligibility. | Recorded for the public-activation review. |

## Commands and evidence

| Run | Command (from WT/rv80, default toolchain, `env -u DEVELOPER_DIR`) | Result |
|---|---|---|
| Baseline | `cargo test --locked --offline --manifest-path WT/rv80/P/core/reporting/result_export/Cargo.toml --test retained_precision_contract -- --test-threads=2` | 47 passed (`BASELINE_TESTS.txt`) |
| Oracle | same, with `--test rv80_oracle` and `RV80_VECTORS` | 0 mismatches over 8,238 (`ORACLE_RESULT.txt`) |
| Probes | same, with `--test rv80_probes` | PR1–PR18 (`PROBES.json`, `PROBES_HARNESS.rs`) |
| Mutants | `python3 MUTANTS.py WT` | 38 killed, 6 survived (`MUTANTS.json`), 01:12–01:26Z |

**Reviewer instruments:** `rv80_oracle.rs` and `rv80_probes.rs` were added only to RV80's archive copy.

**Locations:**
- bulk logs: `WT/scratch/rv80_reader_confirm3/`;
- WT/rv80: deleted at the end;
- WT/targets/rv80: kept.

## Files read

| Identity | File |
|---|---|
| NUM `19065f4828` | T3/ROOT_RULINGS_V1.md, from "Confirmation findings: disposition D19–D26 and the repair round" through "All readers on 07c" (D19–D30) |
| NUM `19065f4828` | R/I63/review_repair_07/RETURN_07B.md, RETURN_07C.md |
| READER `a894d9d0ba` | the Rust source and tests (full diff from `b36739112a`) |
| NUM | FK/adaptive.rs:2309–2440 (estimate/charge rejections); FK/verify.rs:876–884, 1140–1203 (`w`, `charge`, `w_plus`) |

## Open for ROOT

Nothing blocking. Two optional items:
1. **N1:** whether D28 should also restrict `verification_estimate` reasons to force/moment kinds.
2. **N2:** a reader-local test for D21's last-slot case.
