# I63 return: Rust reader on snapshot 07a

I63 is a TASK (Type 2). ROOT (HELP_HUMAN) granted this short adoption directly in the session. The basis is ruling D18 (NUM `253ac9404e`): at G5b, each echoed section term must equal the source's and be positive, otherwise SECTION_MISMATCH; D10 is corrected in place. I63 had no descendants.

- **Run:** first tool call 2026-10-04T00:23:15Z; freeze about 00:25Z, inside the 20-minute box. The memory guard (PID 5387) was running.
- **Limits held:** no Git writes or index operations; no install, new tooling or native job.
- **Toolchain:** the default, with no `DEVELOPER_DIR`.
- **Basis files:** READER at `a491db2f4c`. NUM was at `1e8f73c3de` when hashed.
- **Paths** use the brief's placeholders.
- **Status:** 07a passes in full. **Not accepted; eligibility still held** (`IMPLEMENTATION_COMPLETE = false`).

## Inputs verified

All five shared files match SHARED_SNAPSHOT_07A (`f5f03ac034`), whose I62 SHA256SUMS verify.

| File | sha256 prefix |
|---|---|
| corpus | `a6fa398731` |
| schema | `07951edacf` |
| definition | `3e0779a45a` |
| preview table | `c74742ce6a` |
| results yaml | `4585a45fcf` |

**Counts:** 15 cases, 236 mutations and 19 must-pass entries.

## Changed files (READER, inside the fence)

Paths are under P/core/reporting/result_export.

| File | Before (phase 2) | After |
|---|---|---|
| src/retained_precision.rs | a0973ae5455c… | **unchanged.** D18 was already Rust's behaviour (the positive section echo at RS:3011) |
| tests/retained_precision_contract.rs | 226b9f25af5c… | e1ef9da64055dc088b0fbc2d0d21c4346d42152c0153f0c0def0b9def90a7d77 (92404 B) |
| src/lib.rs | 375b073135… | unchanged |

**The test changes:**
- the slice helper asserts 236 mutations;
- the 07 slice covers 178..236;
- its tally now has G5b SECTION_MISMATCH 2 (`g5b_zero_section_area` and the new `g5b_zero_section_length`) in place of G5b SCALE 1.

## Commands and results

Both runs used the 06d command and environment variables, with the default toolchain and no `DEVELOPER_DIR`.

| Run | State | Result |
|---|---|---|
| run1 (**final, full command**) | final bytes | **35 passed, 0 failed** |
| run2 | `--nocapture snapshot_0 shared_must_pass` | 10 passed; outcomes captured |

**Against the bar:**
- **Mutations:** all 236 match their expected first gate and code, using `expected_by_reader.rust` for G7.
- **must_pass:** all 19 validate with the base case's classifications.
- **Cases:** all 15 validate with their expected classifications.
- **Every other test passes.**
- **Tables:** `OUTCOMES_07A.json`, all 236 mutations in corpus order and all 19 must-pass entries.

## Remaining known differences

**None known on 07a.** The D18 family is resolved: all three readers now require positive section terms at the G5b echo.

The only differences that remain are by design or unreachable:
- G7 base codes per language;
- the fail-closed fallback codes, which Rust does not have. D18 removes the one input that reached the Python fallback.

The basis is phase 2's targeted comparison with Python; it is not exhaustive. I have not compared TypeScript at its head; I64 reports that TypeScript passes 07a.

## Files read (sha256)

| sha256 | File |
|---|---|
| e001726a4c17e44a7aef93b75701c9377a9fa32fc852abeed4d952f04b6e5c3c | T3/ROOT_RULINGS_V1.md at NUM `1e8f73c3de` (the D18 / 07a sections) |
| f5f03ac034f2… | R/I62/review_repair_07/SHARED_SNAPSHOT_07A.json (files, counts) |

## Bulk (WT/scratch/i63_review_repair_07a/)

| sha256 | bytes | file |
|---|---|---|
| 0391fda4af58c1e712ff35d5c904f6432936f51dbc8f446429269d10a6122a77 | 1805 | I63_07A_DELTA_test.diff |
| 9a36a777063a44a4d8436c974e46a8e7bf7c79eae9fa31dedaca7b8b3317d392 | 2648 | run1.log (final) |
| 961392cce345ff868b949ec00aef360a3e26e4c0df5e505c31f063a10d289dab | 61083 | run2_outcomes.log |
| 226b9f25af5cb8470ad1c25cb96cc6f88ce43c808bb4528181cecc39f16d2998 | 92385 | before/retained_precision_contract.rs |
| a0973ae5455ce15a12d4717b49d4af1d3796a24e3023c7d88c0788d02ecf99de | 169071 | final/retained_precision.rs (= before) |
| e1ef9da64055dc088b0fbc2d0d21c4346d42152c0153f0c0def0b9def90a7d77 | 92404 | final/retained_precision_contract.rs |
