# RV78 confirmation 05: the final scoped check on snapshot 07e

RV78 is a TASK (Type 2), resumed by ROOT (HELP_HUMAN) for the final scoped check before reader acceptance (workflow §3). ROOT is the return path, and RV78 did not delegate.

- **Candidate:** READER `63355a91d24dce246f2b85ee4601825824fd1dac`. The previous head was `abcb16fd27`. NUM is at `5c5d2a15c4`.
- **Shared files:** corpus `bbca15d940…` (15 cases, 263 mutations, 22 must-pass entries). The schema is unchanged.
- **Run window:** 2026-10-03, 20:20:01 to about 20:27 local, inside the 30-minute box. Nothing in scope is unfinished. The memory guard (PID 5387) was running.
- **Limits held:** no Git writes, no install, no new tooling, and no native, solver or DEC-025 job.

## Verdict: PASS on the four scoped items; 2 SHOULD-FIX found outside the delta

**Counts:** 0 BLOCKING, 2 SHOULD-FIX, 0 NOTE.

| Scoped item | Result |
|---|---|
| 1. Joint parity on 07e | **300 of 300** entries agree in all three readers and match their expectations, using RV78's own edit, rehash and JCS code that follows `format_rule`. |
| 2. D34 probes | −0 at a U counter, at two indexes, at `G5aError.quantity_kind` and at `constructor_counts.directional_springs` gives **G2 ENCODING in every reader**. |
| 3. RV78-N1 (rehash rule) | **Fixed.** All three harnesses follow the stated `format_rule`. |
| 4. New entries | All 5 are contract-faithful, and RV78's reading agrees with the corpus on each. |

The two SHOULD-FIX findings came from the D34 control probes. They concern result-error consistency on unavailable attempts. **Neither can reach eligibility**, since both concern unavailable attempts. Under ROOT's rule, every SHOULD-FIX is repaired before acceptance, unless ROOT rules otherwise.

## Findings

| ID | Severity | Where | Evidence | Remedy |
|---|---|---|---|---|
| S1 | SHOULD-FIX | P/core/analysis_runs/retained_precision.py:852–866 (`_g5_typed`). TypeScript checks this at retainedPrecision.ts:737–738, and Rust also rejects. | **Python accepts an `observable` or `g5a` result error whose check never failed.** On F′, attempt 1's result was set to `g5a{sanity}` (probe Y1) or to `observable{storage}` (probe Y2), with stages and checks unchanged (observables and g5a not_entered). Python **passes**; Rust and TypeScript give G5 PRODUCT_ATTEMPT. A single defect with a reader divergence and no shared pin. Native basis: `Observable` and `G5a` are returned only after those stages ran and recorded their errors (PP retained_product.rs:3529–3543). C3:279–284: "Observable/G5a require their actual captured cause". With the check failed but the stage not entered (Y5), all three readers reject. The fully consistent shape (Y3) passes in all three. | Python requires an `observable`/`g5a` result to equal a failed `checks.observables`/`checks.g5a`, as TypeScript does. Add Y1 and Y2 as shared pins (G5 PRODUCT_ATTEMPT). |
| S2 | SHOULD-FIX (ROOT to rule) | All three readers (P9 class) | **A `numeric` result error with observables and G5a not entered is accepted by all three readers** (probe Y4). Natively, `Numeric` is returned only at PP:3543, after Observables (:3529) and G5a (:3532) were entered and both checked without error, and after a passed certificate (`full_case_passed` false with no observable/g5a error). So a `numeric` error requires the certificate passed and both observables and g5a completed and passed. The consistent shape (Y6) passes in all three readers, so the rule is pinnable without changing any positive entry. | ROOT rules the P9 extension: a `numeric` error requires certificate passed, and observables and g5a completed and passed, at G5 PRODUCT_ATTEMPT. Then all readers add it, with Y4 as a shared pin. |

## 1. Joint parity on 07e

| Reader | Command result |
|---|---|
| Python | pytest: 365 passed |
| Rust | default toolchain, no `DEVELOPER_DIR`: 56 passed |
| TypeScript | vitest 428/428; tsc exits 0 |

RV78's rehash follows SHARED_SNAPSHOT_07E `format_rule`:
- strict integral indexes (booleans, non-integral values, −0 and negatives are not indexes);
- unresolved references are skipped;
- the preparation hash is recomputed only when every member is prepared;
- source identities are recomputed for selected cases only;
- the order is preparation → source identity → publication → receipt → `after_rehash`.

Per-entry results (PARITY_TABLE.json):
- **300 of 300** entries match in all three readers: 15 cases, 263 mutations and 22 must-pass entries. Disagreements: 0.
- **Python harness:** RV78's prepared inputs equal it on all 285 entries.
- **Hashes:** all 15 base hashes are reproduced.
- **Independent checks:** the invariant checks of the 15 cases and 22 must-pass entries found nothing, and all 7 p512 floor entries match.

## 2. D34 probes (PROBES_CONFIRM_05.json)

| Probe | Python / Rust / TypeScript |
|---|---|
| `Z1` U counter `records[0].corrections = -0.0` | G2 / G2 / G2 |
| `Z2` index `run.origin.call = -0.0` | G2 / G2 / G2 |
| `Z2b` index `product_attempt_ref = -0.0` | G2 / G2 / G2 |
| `Z3` enum `G5aError.quantity_kind = -0.0` (F′ attempt 1) | G2 / G2 / G2 |
| `Z4` const `source_decline.constructor_counts.directional_springs = -0.0` (P′) | G2 / G2 / G2 |

**About the inputs.** RV78's prepared JSON writes `-0.0`, so Rust and TypeScript parse a real −0. RV78's JCS hashes −0 as `0`, so the receipt hashes are valid and only G2 can reject.

**Controls with `+0`:**
- Z4c passes in all three readers.
- Z3c exposes S1: Python passes it, Rust and TypeScript give G5 PRODUCT_ATTEMPT.

## 3. RV78-N1: the rehash rule in all three harnesses

| Harness | How it follows `format_rule` |
|---|---|
| Python | `_rehash_ref`: a number, never a boolean (`type in (int, float)`), finite, integral, ≥ 0 and not −0, and it must resolve. Otherwise it is skipped. The preparation hash requires every member prepared; source identity is for selected cases only. |
| Rust | `index()`: `as_f64` (so booleans are not indexes), finite, ≥ 0, integral and not −0. An out-of-range attempt gives `Null` and is skipped; the source goes through `get()`. Same preparation and selected conditions. |
| TypeScript | `rehashRef`: `typeof number`, `Number.isInteger`, ≥ 0, `!Object.is(-0)` and in range. It now requires every member prepared (this closes the round-04 divergence) and is selected-only. Its edit paths also refuse non-index array keys. |

Each harness has a reader-local test of the rule. **No harness check is weakened**: the removed lines are the replaced `int()` indexing and direct indexing.

## 4. New entries (NEW_ENTRIES_REVIEW_05.json)

| Entry | Expected | RV78 reading |
|---|---|---|
| `g0_receipt_version_non_integral` (1.5) | G0 | Agrees (D2, D32). Correct rejection. |
| `g0_case_limit_non_integral` | G0 | Agrees (D2). Correct rejection. |
| `g0_invocation_limit_non_integral` | G0 | Agrees (D2). Correct rejection. |
| `verification_estimate_names_rotation_row` | G5 ATTEMPT | Agrees (D33; FK verify.rs:880). The quantity resolves to a rotation row with matching body and kind, so only D33 fails. |
| `verification_estimate_names_moment_row` (must-pass) | pass | Agrees. The quantity resolves to a moment row. This is an emittable shape. |

## 5. Method and evidence

**Commands:** as in earlier rounds, on the default toolchain without `DEVELOPER_DIR`.

**Disclosed:**
- **`node_modules`:** linked to READER's link target.
- **WASM assets:** READER's prebuilt `public/wasm-engine` and `public/self-weight-engine` were **copied**, not built. Hashes are unchanged from round 04.
- **Probe test files:** RV78's probe test files existed only in `WT/rv78/`, which is deleted.

**Evidence in this folder:**
- **PARITY_TABLE.json:** 300 rows.
- **PROBES_CONFIRM_05.json:** the D34 probes, Y1–Y6 and the controls.
- **NEW_ENTRIES_REVIEW_05.json.**
- **INDEPENDENT_CHECKS_05.json.**
- **SHA256SUMS.**

**Scripts** are in `WT/scratch/rv78_reader_confirm5/`. The only change from round 04 is in `rv78_prepare.py`: source identities are now recomputed for selected cases only, per `format_rule`.

## 6. For ROOT

1. **S1:** Python aligns, and Y1 and Y2 become shared pins.
2. **S2:** rule on the `numeric`-error stage requirement, then pin Y4.

Both concern unavailable attempts only, with no path to eligibility. ROOT decides whether they gate acceptance or follow it.
