# I90 B1-SR-RS, repair round 1: RV113's S-1, N-1 and N-3 (tests only)

TASK (Type 2), I90 (I-RS), for ROOT (HELP_HUMAN, Agent 0), who is the return path. I made no delegation. 2026-10-07 UTC.

**The basis:**
- ROOT's message dispatching this round;
- RR "RV113 passes SR-RS with S-1; an SR-RS repair round queued; test binaries run under the lock", rulings 1, 2 and 4;
- RR "I92's SR-TS verified; …", ruling 5;
- RV113's review, `R/REVIEW_RV113/rvr_sr_rs_01/REVIEW.md`, sha256 `09140cb99d031c0b08d5e0e752581187a7b6b381fc6e6bdfaff46a0548404dda` (verified). I took S-1, N-1 and N-3's probe shapes and mutant definitions from its `evidence/harness/gen_probes.py` and `evidence/mutants/mutant_schema_RS.diff`.
- For N-3 and S-1, I mirrored I92's TS rows at `7e5c40f3d5` and `7e47e51b5d`.

N-2 is kept as ruled. N-4 goes to PR-B1's package text, and N-5 needs no action, so neither is in this round.

**State.** Done. One test-only commit on `codex/piping-t3-b1-r-20261007`: **`b5cb7faaeb`** over `cc81e78801`. RS and `source_blocks.rs` are unchanged (`git diff cc81e78801..b5cb7faaeb` touches only `RE/tests/retained_precision_contract.rs`, +78/−3). I needed no RS `src` change. Not pushed.

**Placeholders:** WT, NUM, P, PP, RE, RS, T, R as in RETURN.md.

## 1. The changes (`RE/tests/retained_precision_contract.rs`)

| Finding | Change | Kills |
|---|---|---|
| **S-1** (SHOULD-FIX): the `not_required` rule's `product_attempt_ref` null conjunct was pinned by no test | In `b1_g5_not_required_admits_a_w2_published_case`: 07j's must-pass entry without its removal of `product_attempts/1` (the test asserts there is exactly one such removal), with `cases/1/product_attempt_ref` = 1. The `not_required` case names its own attempt, so G3 passes, and RS refuses it at **G5 `ATTEMPT_MISMATCH`**, by the rule itself (G5's ordinary class runs before D19's `PRODUCT_ATTEMPT`). The existing other-case variant still stops at G3. The doc says both | M20 |
| **N-1**: (4a) had no positive witness at `validate` | A new test, `b1_d38_4a_native_failure_with_a_run_is_admitted`, after RV113's `a4_*` shape (below) | M02 |
| **N-3**: P4's "published" (not "triggered") was unpinned | In `b1_g8_parity_rows_p2_to_p4_for_every_case`: on the dense 07j statement, one parity row on a `not_required` case whose W2 was triggered and **failed** (`{kind: failed, trigger: evaluation/range, failure: not_engaged}`, so b = 0) is **admitted**. This is I92's TS row. The doc says P4 reads published | M11 |

**N-1's witness.** No shared base has a non-selected native Run (the contract test's D30 note). So on F_BASE:
- case 1's Run is made idle in its ready group: no records or attempts, no charge, `cache_after` = `cache_before`, and the terminal refused `ledger_unavailable`, as a CasePrep failure leaves it;
- the call's after-value and `charged` drop to the Run's `invocation_before`;
- the case reads `kernel_refused` (`kernel`), with cause `prepared_product_failure` naming attempt 1;
- attempt 1 keeps `run_ref` 1 (asserted), with proof null, preparation completed, native failed and later stages not entered.

It checks:
- **(4a)** with a native error naming Run 1: admitted, not eligible;
- **(4a)** with a capture error beside the Run (`reason_table`'s capture-with-Run arm): admitted, not eligible;
- **(4b)'s reason** (`source_unavailable`, `preparation`) on this Run-bearing shape: G5 `PRODUCT_ATTEMPT`.

## 2. Evidence

### 2.1 Mutants (`_run_records/repair_01/mutants/`)

RV113's definitions, applied as exact text edits to a scratch archive of `b5cb7faaeb` (`WT/scratch/i90_b1_sr_rs/repair_01/mut`). Each mutant ran one cargo job through `WT/tools/t3_cargo.sh`: `test --locked --offline --no-fail-fast --test retained_precision_contract --lib`. The bytes were restored after each, and the script checks the restoration (`mutants_r1.py`, `results.jsonl`, one log each).

Each run covers 96 tests.

| Mutant (RV113's) | Result | Killed by (the assertion's message) |
|---|---|---|
| **M20**: `not_required` without `product_attempt_ref` null | **killed** | `b1_g5_not_required_admits_a_w2_published_case`: "product_attempt_ref naming the case's own attempt: got G5 `PRODUCT_ATTEMPT`" (it falls through to D19) |
| **M02**: (4b)'s branch with "or" (a Run-bearing native failure routed to (4b)) | **killed** | `b1_d38_4a_native_failure_with_a_run_is_admitted`: both (4a) rows "got G5 `PRODUCT_ATTEMPT` want admitted" |
| **M11**: P4 stricter (a parity row only beside W2 `not_triggered`) | **killed** | `b1_g8_parity_rows_p2_to_p4_for_every_case`: "one parity row on a not_required case whose W2 failed: got G8 `PREPARATION` want admitted" |

At `cc81e78801` all three survived RE's tests (RV113 §8).

### 2.2 The RE suite against `cc81e78801` (`_run_records/repair_01/suites/`)

RE's whole suite at `b5cb7faaeb`, `cargo test --locked --offline --no-fail-fast -- --nocapture` through `t3_cargo.sh`, compared test by test with RETURN's run at `cc81e78801`:

- `cc81e78801`: 186 ok. `b5cb7faaeb`: **187 ok**, 0 failed.
- **The one difference is the added test** `b1_d38_4a_native_failure_with_a_run_is_admitted` (ok). Every other test has the same name and outcome. S-1's and N-3's checks are added rows inside two existing tests.
- Compiler warnings: the same lines.

### 2.3 The census

The same `census.py`, comparing I1's run with `b5cb7faaeb`'s: **`CENSUS mutations 294 (base lines 294, aligned lines 294); must_pass 28 (base 28 true, aligned 28 true); changes 0`** (`census_r1.out`).

### 2.4 The c = 1 pins

At `b5cb7faaeb`, PP's three publishing pin tests pass (3 ok), with their output variables set. The six successor documents are byte-identical to RETURN's head run and to I1's:
- milestone: `ac6986b0…dca59dc` (sparse) and `6cd1d249…ad95c9b5` (dense);
- L = 0: `93c6c865…eb350876` (sparse) and `dbb3d477…63b0ac88` (dense).

That is expected, since PP compiles RS's unchanged production code; the test file is not compiled into PP. `compare_pins_r1.out` has the comparison.

## 3. Host

- Every cargo ran through `WT/tools/t3_cargo.sh` (`--locked --offline`). I ran no test binary directly, so nothing heavy ran outside the lock.
- My jobs this round are in `_run_records/repair_01/cargo_jobs_i90_r1.log`:
  - the B1 tests (`r1_b1tests`);
  - the RE suite;
  - the pins;
  - the three mutants.
- Each was one job with one wait, ending when its process was gone. The B1-tests job waited about 15 minutes for the lock, past my foreground call's limit, so its wait continued in the background until the job ended. None of my waits remain, and I killed no job.
- The mutant target (`WT/targets/i90-b1-sr-rs/mut-r1`) and the archive are deleted. TMPDIR was in scratch.
- No record folder is named `build`, every write used an absolute path, and this folder has placeholder paths only.

## 4. For ROOT

1. S-1, N-1 and N-3 are closed in RS by tests alone. RS's production code is unchanged since RETURN (`d7c76de43f`), so the census, the c = 1 identity and the D38 list stand as RV113 confirmed them.
2. For SC, unchanged from ruling 1: 07n's "`product_attempt_ref` set non-null on case B" uses case B's own attempt, so that it reaches G5, as this test does on 07j.
