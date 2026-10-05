# I63 return: Rust reader, review repair 07, phase 2 (snapshot 07)

I63 is a TASK (Type 2). ROOT (HELP_HUMAN) granted this work directly in the session as phase 2 of `BRIEFS/I62_I64_REVIEW_REPAIR_07.md`. The basis is the rulings from "Checkpoint A: rulings on the native facts for snapshot 07" to "Snapshot 07 installed; Python repaired; phase 2 granted" (NUM `b2dbfcfacb`). I63 had no descendants.

- **Run:** first tool call 2026-10-04T00:09:46Z; freeze about 00:17Z, inside the 90-minute box. The memory guard (PID 5387) was running.
- **Limits held:** no Git writes or index operations; no install, new tooling, solver, native or DEC-025 job; one Cargo job at a time, each under a 1,200 s wall.
- **Toolchain:** the default, with no `DEVELOPER_DIR`.
- **Basis files:** READER at `04ea067b5c`, with corpus `90f6e4ed9b`, schema `07951edacf` and the Python reference `dddac2fa96`.
- **Paths** use the brief's placeholders.
- **Status: one stop item.** One 07 expectation is contested and was **not adjusted**; see "Stop". Everything else meets the bar. **Not accepted; eligibility still held** (`IMPLEMENTATION_COMPLETE = false`).

## Changed files (READER, inside the fence)

Paths are under P/core/reporting/result_export.

| File | Before (phase 1, `dd684fcbeb`) | After |
|---|---|---|
| src/retained_precision.rs | b3cbb8afd178… | a0973ae5455ce15a12d4717b49d4af1d3796a24e3023c7d88c0788d02ecf99de (169071 B) |
| tests/retained_precision_contract.rs | 79836bb241a0… | 226b9f25af5cb8470ad1c25cb96cc6f88ce43c808bb4528181cecc39f16d2998 (92385 B) |
| src/lib.rs | 375b073135… | unchanged |

## What changed

**D8, mirroring Python's `_accounting_rules` and `_fault_owner`.** R1′–R4 run in `accounting_rules`, at G5 WORK in class 4, deferred after every attempt's association checks and the typed pass:
- **R1′:** `adapter.fault` must be null, and no object `{kind:"accounting", event}` may appear.
- **R2′:** no `lost: true`, and no OperationalError `{kind:"accounting"}`. Those are located as Python locates them: a non-ready MemberOperational error, a CaptureError `prepared_arithmetic` cause, or a G5aError `operational`/`arithmetic` cause, in the result or a failed check.
- **R3′:** every fault-bearing cause, in all three spellings (`work_accounting{fault}`, a nested `stop/work_accounting`, a view `work{fault}`), must have its fault in **its owner's** emitted statuses. The owner (`fault_owner`) is:
  - the member's PreparationWork, for a PreparedMember error or the preparation section;
  - the lane's LaneWork, for a lane error;
  - the values completion, for a values cause;
  - otherwise the ProofTrace.
  
  A cause with no owner fails. This replaces 06d's attempt-wide join.
- **R4:** a SectionError `accounting`, on a PreparedMember or as the preparation section (owned by the last member), needs a non-exact status in that member's PreparationWork.

**D8 kernel scope:** a `work_accounting` stop or reason anywhere in a Run, a build or a group preparation fails G5 ATTEMPT at the start of class 1, ahead of the deferred native WORK checks.

**D9:** no reader code was needed. The closed-shape walker reads the 07 schema directly, which pins the removed Refusal variants and the unavailable-case `source_ref` branches at G1.

**Already in place from phase 1, now pinned by 07:** D16 (a dangling build reference is deferred WORK; other class-1 references are ATTEMPT); D17 (ordinary references come before C3 association in class 2; Rust's `g5_ordinary` already runs before `g5_products`); the settled readings (`receipt_version` exactly 1; an absent receipt or body fails G0; computation faults are deferred WORK).

**Harness and tests:**
- `rehash` skips an entry that removes `retained_precision` or its body, as the 07 format specifies for the G0 pins.
- Counts are now 235 mutations and 19 must-pass entries.
- New slice test `snapshot_07_mutation_outcomes` (178..235).
- The `reader_logic::accounting` hook returns `[R1′, R2′, R3′, R4]`.
- New reader-local tests:
  - R3′ with `both` against its owner; a fault emitted outside the owner does not count;
  - R2′ operational accounting;
  - R4 exact versus non-exact member work;
  - `d8_kernel_scope_work_accounting_anywhere`: a build reason and a record outcome give ATTEMPT, ahead of WORK;
  - `d8_accounting_rules_on_shared_bases`.

## Commands and results

Every run used the 06d command and environment variables, with the default toolchain and no `DEVELOPER_DIR`.

| Run | State | Result |
|---|---|---|
| run1 | baseline: phase-1 reader on 07 | rehash panicked on the G0 receipt-removal pins; count assertions failed |
| run2 | counts and robust rehash | five 07 mismatches: four D8 entries admitted, and `g5b_zero_section_area` |
| run3 | D8 | one mismatch left: `g5b_zero_section_area` |
| run4 (**final, full command**) | plus the D8 tests | **33 passed, 2 failed.** Both failures are this one contested entry: `shared_rehashed_first_failure_mutations` and the 07 slice tally |
| run5 | `--nocapture snapshot_0 shared_must_pass` | all outcomes captured |

**Against the bar:**
- **Mutations: 234 of 235** match their expected first gate and code (per reader for G7). The one exception is `g5b_zero_section_area`.
- **must_pass:** all 19 validate with the base case's classifications.
- **Cases:** all 15 validate with their expected classifications.
- **Every other test passes.**
- **Tables:** `OUTCOMES_07.json`, all 235 mutations in corpus order and all 19 must-pass entries, with the mismatch flagged.

## Stop: a contested expectation, not adjusted

**`g5b_zero_section_area`** sets a section's `area` to +0 in both the source and the Selection section terms.
- **Expected:** G5b `RETAINED_PRECISION_SCALE_MISMATCH` (source decision D10, from RV79-S3). D10 was ruled for Python only.
- **Rust:** G5b `RETAINED_PRECISION_SECTION_MISMATCH`, at the G5b section-echo check, which requires every echoed term (area, section_modulus, length, axial and torsional stiffness) to equal the source **and be positive** (RS:3011). That check is inherited (I59) and runs before the stress-row scales.
- **Python:** echoes equality only (PY:1309). Its stress-scale division by the zero area (PY:1330) then raises ZeroDivisionError. Only the **fail-closed catch-all** (PY:1639) maps that to the G5b phase code SCALE. Under D16 that catch-all is a fallback, not an adopted check.

**Contract.** C1's G5b row lists "section truth" with the "adopted scale/section mismatch codes". A zero section term is a section-truth defect; natively it is impossible, since prepared conversions require normal values ≥ 2^-1022 (G5 P5). So I read SECTION_MISMATCH as at least as faithful, and the expectation rests on Python's fallback.

**Options for ROOT:**
- **(a)** Expect G5b SECTION for all readers. Python and TypeScript would add positivity to their section echo.
- **(b)** Keep SCALE. Rust would drop the positivity requirement and let `finite_check` on the stress scale report SCALE. A zero length or stiffness would then go unchecked at G5b when no row divides by it, and fall to G8.

I recommend (a), but I made neither change.

## Remaining known differences

**From Python `04ea067b5c`:**
1. **The family behind the stop.** Rust requires positive section terms at the G5b echo; Python requires only equality. For a zero area or modulus, Python reaches SCALE through its fallback. For a zero length or stiffness, Python passes G5b and fails later; Rust gives G5b SECTION.
2. **Fail-closed fallback codes:** Rust has no catch-all; Python's maps to the phase's code. This is unreachable apart from item 1, and accepted by ROOT.
3. **G7:** per-language base codes, by design (`expected_by_reader`).

**What this covers.** It is a targeted comparison of the D1–D17 code paths, the D8 rules and the kernel scope, which I mirrored line by line. It is not an exhaustive comparison of the two files. On all 235 shared mutations, 19 must-pass entries and 15 cases, the only disagreement is item 1.

**From TypeScript:** not compared at its new head. I64 is adopting 07 at the same time, so its 07 outcomes are I64's to report.

## Files read (sha256)

| sha256 | File |
|---|---|
| 4a6d516510afe9c768e655aa29c23ebb0126f07edc3af140f1f10f8a73c6fd23 | T3/ROOT_RULINGS_V1.md at NUM `b2dbfcfacb` (checkpoint A, D16, settled readings, D17, B2 and the phase-2 grant) |
| 946a75ccd47c743c641312905f13a3308c08ea874dfdf7e5787d591ebc5ab766 | R/I62/review_repair_07/CHECKPOINT_A.md (D8 shape table, owner scopes, rebase) |
| 82072054ecde2ba6b55582c19764ba31ecc616f3188470fe2a7abef8cc515c54 | R/I62/review_repair_07/SHARED_SNAPSHOT_07.json (format, schema changes, new entries, deferrals) |
| c7d7fe253bac1b635d81623970f9f4b8845df6a0351a8ed40549dd109de6c3f2 | R/I62/review_repair_07/RETURN_B1.md (verified via SHA256SUMS; skimmed) |
| 473b8a8ae05e271ea9601bec81714213a61b863586e5f5707d208f854e431d47 | R/I62/review_repair_07/RETURN_B2.md (verified via SHA256SUMS; skimmed) |
| dddac2fa96adc1e148cab4e305739cf7055873f515051b5151dc4ccf56bdf752 | READER/P/core/analysis_runs/retained_precision.py (`_accounting_rules`, `_fault_owner`, `_statuses`, kernel scope, the G5b section/scale lines and the fallback; read only) |

## Bulk (WT/scratch/i63_review_repair_07_p2/)

| sha256 | bytes | file |
|---|---|---|
| 650ac4ba37bc97006d24353a39278fe0cf38232aa0ea21b0882a63d48543e23b | 9764 | I63_07P2_DELTA_src.diff |
| 591e66fd5e985eed48d317e910c1fee038b9ef1acb742ef3ee43911ccbd9a2b6 | 7454 | I63_07P2_DELTA_test.diff |
| da784e82ebdd288d940e4ef929b3aab59b4472fff2a790931793a71e8a3fac8d | 5254 | run1_baseline.log |
| 3da2dff554eb1382abf0897d5f2a35caaec429b84bb3ecff90b18aee1bb36647 | 18377 | run2.log |
| e6ee6b4696cbb561d74c4971952e8c09a7ec17b348c85a434d0633c2e29e975c | 18011 | run3.log |
| c802ed9a2f09b6041b8de7d434e877f0ca5b8ce8cb1bdc2c079dc9d05228d45d | 18112 | run4.log (final) |
| 2dd70a333ad8d2e26ec82ee40b71bdfc2c0bbdd52185574af05742975fa8ba96 | 61232 | run5_outcomes.log |
| b3cbb8afd17850bba1af1149603e8f77a1b1f889ca6912ff72a34bac27b61c0e | 163517 | before/retained_precision.rs |
| 79836bb241a0f56da2419bffce13f4901d0d0212adc000bbb891ce19040fe3ce | 88734 | before/retained_precision_contract.rs |
| 375b07313518a0cdd09e317b83b9ca7536acef5c80fd4cd0a94920fd8bf2486d | 66051 | before/lib.rs |
| a0973ae5455ce15a12d4717b49d4af1d3796a24e3023c7d88c0788d02ecf99de | 169071 | final/retained_precision.rs |
| 226b9f25af5cb8470ad1c25cb96cc6f88ce43c808bb4528181cecc39f16d2998 | 92385 | final/retained_precision_contract.rs |

The run*.exit and run*_start.txt files hold the exit codes and UTC bounds.
