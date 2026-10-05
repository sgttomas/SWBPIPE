# RV80 — confirmation round 04: the Rust reader on snapshot 07d

RV80 is a TASK (Type 2) dispatched by ROOT (HELP_HUMAN) for a scoped confirmation of the 07d delta (workflow §3). ROOT is the return path. RV80 resumes as the Rust reviewer, did not write the repairs, and had no descendants.

- **Candidate:** READER `abcb16fd27d7c3ccd019f2533eb261d4c564fdc7`, archived into `WT/rv80/`. NUM is at `83732c5677`.
- **Files under review:**

  | File | sha256 |
  |---|---|
  | `retained_precision.rs` | `bb713adcebc6c14db1f4c531e54b3da2e7027210b9cf501408c613b0a957ba7d` |
  | tests | `37cf97b4ab5a…` |
  | `lib.rs` (unchanged) | `375b07313518…` |
  | corpus 07d | `12da125d9d` (15 cases, 259 mutations, 21 must-pass) |

  All match I63's RETURN_07D.
- **Diff reviewed:** `a894d9d0ba..abcb16fd27`, which touches only `retained_precision.rs` (+61/−3) and its test file (+266/−12).
- **Run window:** 2026-10-03 19:46–20:09 MDT (WT/rv80 deleted at 20:09), inside the 45-minute box. Memory guard PID 5387 running.
- **Host:** default toolchain only. Every command ran under `env -u DEVELOPER_DIR`, and the mutant script strips the variable (`MUTANTS.py:100`). `CARGO_TARGET_DIR=WT/targets/rv80`, one Cargo job at a time.
- **No** Git writes, installs, new tooling or native jobs.

## Verdict

**PASS — 0 BLOCKING, 0 SHOULD-FIX, 2 NOTE.**

- Both confirm_03 notes are disposed of as ruled, and RV80's own probe and mutant confirm each.
- The D31–D33 changes add or replace checks; none weakens one.
- The D32 normalization touches only receipt numbers, leaves −0 and booleans unconverted (both are rejected earlier), and converts nothing outside ±(2^53−1).
- **Baseline** 53/53; **oracle** 0 mismatches over 8,238 vectors; **mutants** 44 of 52 killed, every survivor explained below.

## confirm_03 notes: dispositions

| Note | Decision | Disposition | RV80 evidence on `abcb16fd27` |
|---|---|---|---|
| RV80-N1: a `verification_estimate` reason on a translation/rotation row | D33 | **Fixed.** In the D28 block, a `verification_estimate` reason must name a force or moment row (RS:1036–1040, G5 ATTEMPT); `charge` stays unrestricted. | PR16 (the node-1 UX translation row) now gives **G5 ATTEMPT** (was Ok). PR12, PR13, PR17 and PR18 are unchanged. M51 (D33 dropped) is killed by `d33_…` and the 07d slice. |
| RV80-N2: D21's last-slot conjunct unpinned | (test) | **Fixed.** `d21_last_slot_verification_shared_build` (tests:2919) builds a Ceiling-shaped run with no fresh attempt after the escalating v512 failure. | **M39 is now killed**, by that test only. |

PR1–PR15 give the same outcomes as in confirm_02 and confirm_03 (`PROBES.json`).

## The D32 normalization (`integral_receipt`, RS:251–291)

Each question in the dispatch, checked against the code and probes:

- **Only receipt numbers are touched.** Both the float scan and `normalize` operate on `source["retained_precision"]` only (RS:283–289). The rows, diagnostics, quality and invocation are untouched. In this schema, every receipt number is a U or I32: the only other `number` type is `RawRow.value`, which sits outside the receipt.
  - **One caveat:** mutant M50 widens normalization to the whole statement, and it survives. The receipt-only scope is therefore correct by code reading but not pinned by any test (NOTE N1).
- **−0 is still rejected.** Normalization skips −0 (`!(x == 0.0 && x.is_sign_negative())`). G2 runs first and already rejects −0 through `uint()` and the I32 encoding. PR20 (`source_ref: -0.0`) gives **G2 ENCODING_MISMATCH**. M46, which lets normalization convert −0, survives only because nothing past G2 ever sees a −0.
- **Booleans are still rejected.** `normalize` acts only on `Value::Number`, and `uint()` uses `as_f64`, which is `None` for a boolean.
  - PR21 (`source_ref: false`) gives **G1** (a closed shape: not a number).
  - PR23 (`receipt_version: true`) gives **G0**, per D2's wrong-type rule.
- **Nothing outside ±(2^53−1) is converted.** The guard `x.abs() <= SAFE as f64` leaves such values as written. They never reach G2 or normalization: the checked canonical JSON refuses unsafe numbers when the receipt hash is computed, so they fail at G1.
  - PR25 (`case_charge: 1e16`) and PR26 (`9007199254740993`) both give **G1 RECEIPT_MISMATCH**. RV80's harness writes an empty hash where canonical JSON refuses the statement.
  - If G1 were bypassed, G2's `uint`/I32 checks would reject these values.
  - M47 (range guard dropped) survives for the same reason as M46.

  This matches ROOT's earlier record that unsafe integers are refused before any gate (snapshot-04 ruling).
- **Value semantics now hold where they matter.**
  - PR19 (`source_ref: 0.0`), PR22 (`receipt_version: 1.0`), PR24 (`case_limit: 20000000000.0`) and PR28 (float-written attempt owner index, quality index and `product_attempt_ref`) all validate with eligibility held.
  - PR27 (the forged source identity under `source_ref: 0.0`, using an after-rehash edit) gives **G1**.
  - The G0 checks (RS:525, 539) use `uint()`. Mutants M48/M49, which restore the host-type tests, are killed by `d32_…` and the must-pass entry. M45 (normalization disabled) is killed by three tests.
- **Placement.** Normalization runs after G0–G2, which validate the statement as written (RS:4202, 4254). Every later gate reads the normalized copy. G7 projects away `retained_precision`, so the base validator sees exactly the original envelope. Canonical hashing renders `17.0` and `17` identically, so G1 is unaffected.

## Repair diff review (`a894d9d0ba..abcb16fd27`)

**Source.** The three removed lines are:
- the G0 `receipt_version` host-type test, replaced by `uint()`;
- the G0 limits `as_u64`, replaced by `uint()`;
- the G8 model-version rule `0.2.0|0.3.0`, replaced by `0.1.0|0.2.0|0.3.0` (D31, RS:3384; 0.4.0 stays excluded).

Each is replaced by its decision's check; none is weakened. The additions are D33 (one ATTEMPT conjunct) and `integral_receipt` with its two call sites.

**Tests.** The removed lines are the old harness `as_u64` indexing, now value-indexed per D32, and the 254/19 count assertions, now 259/21. The additions are `d31_…`, `d32_…` (two tests), `d33_…`, the D21 last-slot test and the 07d slice.

**Fail-closed.** `IMPLEMENTATION_COMPLETE = false`, and the eligibility conjunction is unchanged. M01 is killed.

## Mutation testing (52 single-edit mutants)

`MUTANTS.py` runs M01–M44 from round 03 (M31 re-expressed for the `uint()` G0 test), plus eight new mutants:

| ID | Edit |
|---|---|
| M45 | normalization disabled |
| M46 | −0 converted |
| M47 | range guard dropped |
| M48 | `receipt_version` host-type test restored |
| M49 | limits host-type test restored |
| M50 | normalization widened to the whole statement |
| M51 | D33 dropped |
| M52 | D31 reverted |

Each mutant starts from the pristine source (`bb713adc`), which is restored and re-hashed after every run (01:48–02:06Z).

**Totals:** 44 killed, 8 survived.

| Survivor | Reason |
|---|---|
| M02 (N17 scope) | **No base** (≥20B/60B work; deferred). |
| M06 (relative boundary) | **No base:** no equality row. |
| M19 (D4d nonselected requirement) | **Equivalent:** the `refused` arm rejects a selected Run with the same code. |
| M27 (D1 captured_prefix G5 references) | **Equivalent:** the G5 stage, D4e and Ready rules give the same code. |
| M35 (D1 origin-owner conjunct) | **Equivalent for single defects:** the source-owner conjuncts give the same ATTEMPT. |
| M46 (normalization converts −0) | **Equivalent:** G2 rejects −0 before normalization (PR20). |
| M47 (range guard dropped) | **Equivalent:** canonical hashing refuses unsafe numbers at G1 (PR25, PR26), and G2 would refuse them anyway. |
| M50 (normalization widened to the whole statement) | **Unpinned scope** (NOTE N1). No corpus entry is sensitive to rows written as integral floats, because rows are read by value. |

**Killed:** M01, M03–M05, M07–M18, M20–M26, M28–M34, M36–M45, M48, M49, M51 and M52. M39 is newly killed by the D21 last-slot test, and M51/M52 are killed by the D33 and D31 tests.

## New findings

| ID | Severity | Location | Finding | Remedy |
|---|---|---|---|---|
| RV80-N1 | NOTE | RS:283–289 | **The receipt-only scope of `integral_receipt` is correct by code, but not pinned:** M50 (normalize the whole statement) survives. RV80 found no current consumer for which rewriting a row's `17.0` as `17` changes an outcome, so this is defence of intent rather than a live gap. | Optional: a reader-local test that the projected base envelope (G7 input) is byte-identical for a statement whose receipt contains integral floats. |
| RV80-N2 | NOTE | RS:262–268 | **The −0 and range guards inside `normalize` are defence in depth.** −0 fails G2 first, and out-of-range numbers fail G1 first, so M46 and M47 are equivalent. This is recorded so that a later reordering of the gates keeps the guards. | None. |

## Commands and evidence

| Run | Command (from WT/rv80, default toolchain, `env -u DEVELOPER_DIR`) | Result |
|---|---|---|
| Baseline | `cargo test --locked --offline --manifest-path WT/rv80/P/core/reporting/result_export/Cargo.toml --test retained_precision_contract -- --test-threads=2` | 53 passed (`BASELINE_TESTS.txt`) |
| Oracle | same, with `--test rv80_oracle` and `RV80_VECTORS` | 0 mismatches over 8,238 (`ORACLE_RESULT.txt`) |
| Probes | same, with `--test rv80_probes` | PR1–PR28 (`PROBES.json`, `PROBES_HARNESS.rs`) |
| Mutants | `python3 MUTANTS.py WT` | 44 killed, 8 survived (`MUTANTS.json`) |

- **Probe harness changes this round:** it indexes by integral value (as D32 requires), supports after-rehash edits, and writes an empty hash where canonical JSON refuses the statement. Its first run panicked on PR25 for exactly that reason; it was rerun after the fix.
- **Reviewer instruments:** `rv80_oracle.rs` and `rv80_probes.rs` were added only to RV80's archive copy.
- **Locations:** bulk logs are in `WT/scratch/rv80_reader_confirm4/`. WT/rv80 is deleted at the end; WT/targets/rv80 is kept.

## Files read

| Identity | File |
|---|---|
| NUM `83732c5677` | T3/ROOT_RULINGS_V1.md, from "T1 and T2: I61's analysis and the rulings" to the end (D31–D33; T1 owner-confirmed) |
| NUM `83732c5677` | R/I63/review_repair_07/RETURN_07D.md |
| READER `abcb16fd27` | the Rust source and tests (full diff from `a894d9d0ba`); the schema (number-typed fields) |

## Open for ROOT

Nothing. N1 and N2 are optional.
