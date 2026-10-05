# RV78 confirmation 06: the D37-scoped check on snapshot 07f

RV78 is a TASK (Type 2), resumed by ROOT (HELP_HUMAN) for the D37-scoped check, the last before reader acceptance. ROOT is the return path, and RV78 did not delegate.

- **Candidate:** READER `85905e95e9d5a97224236d8ef817718a044fcd92`. The previous head was `63355a91d2`. NUM is at `d3155b3539`.
- **Corpus:** `2cae6d6823…` (15 cases, 268 mutations, 22 must-pass entries). The schema is unchanged.
- **Run window:** 2026-10-03, 20:49:29 to about 20:56 local, inside the 30-minute box. Nothing in scope is unfinished. The memory guard (PID 5387) was running.
- **Limits held:** no Git writes, no install, no new tooling, and no native, solver or DEC-025 job.

## Verdict: PASS

**Counts:** 0 BLOCKING, 0 SHOULD-FIX, 0 NOTE. There are no new findings to triage under D36.

| Scoped item | Result |
|---|---|
| 1. Joint parity on 07f | **305 of 305** entries agree in all three readers and match their expectations. |
| 2. D37 probes | **All 17 gated probes** give their D37 outcome in all three readers. That covers Y1, Y2, Y4 and Y6, RV79's five X1 pins (in the parity run), and consistent and inconsistent records across the error kinds. |
| 3. The `capture` (a) record | All three readers **reject** it at G5 PRODUCT_ATTEMPT. They agree. It is tracked under D36. |
| 4. Weakening | **None.** Python and Rust replace the old one-direction mapping with strictly stronger whole-record tables; TypeScript removes no line. |

## 1. Joint parity on 07f

| Reader | Command result |
|---|---|
| Python | pytest: 371 passed |
| Rust | default toolchain: 58 passed |
| TypeScript | vitest 436/436; tsc exits 0 |

RV78 used its own edit, rehash and JCS code, following the 07e `format_rule` (PARITY_TABLE.json):
- **305 of 305** entries match in all three readers, with 0 disagreements;
- RV78's prepared inputs equal the Python harness on all 290 entries;
- the base hashes are reproduced;
- the invariant checks of the 15 cases and 22 must-pass entries found nothing, and all 7 p512 floor entries match.

**The corpus delta from 07e** (RV78's own diff):
- exactly 5 mutations added: RV79's X1 pins;
- nothing changed or removed;
- the cases and must-pass entries are unchanged;
- `g5a_error_with_g5a_not_entered`, `observable_error_with_observables_not_entered` and `numeric_error_with_checks_not_entered` carry RV78's Y1, Y2 and Y4 edits.

## 2. D37 probes in all three readers (PROBES_CONFIRM_06.json)

All probes are on F′ unless marked P′. Each row lists the expected outcome; Python, Rust and TypeScript all matched it.

| Error kind | Consistent record (expected PASS) | Inconsistent record (expected G5 PRODUCT_ATTEMPT) |
|---|---|---|
| `g5a` | Y3: observables completed and passed; G5a failed | Y1: G5a not entered; `K_g5a_with_observables_failed` |
| `observable` | `K_observable_consistent`: observables failed; G5a completed and passed | Y2: observables not entered; `K_observable_g5a_not_entered` |
| `numeric` | Y6: all ten stages completed; checks passed | Y4: checks not entered; `K_numeric_with_g5a_failed` |
| `capture` | (b): native completed, no proof; (c): the F′ base itself; (d): all ten completed and passed | `K_capture_with_proof_start_failed` |
| `proof` | (a), (c): must-pass `lane_k_failed`, `cert_failed_before_summary` (parity) | `K_proof_all_completed_passed`; corpus `proof_error_with_certificate_passed` |
| `values` | must-pass `values_failed_separate_completion` (parity) | `K_values_with_maxima_failed`; corpus `values_error_with_values_completed` |
| `abandoned` | (a), (b), (c): must-pass `maxima_abandoned`, `aliases_abandoned`, `bind_rows_abandoned` (parity) | `K_abandoned_with_certificate_completed` |
| `preparation` | the P′ base (parity) | `K_preparation_with_preparation_completed` (P′) |
| `native` | no faithful base: there is no nonselected-Run base (D30, deferred) | `K_native_error_on_unavailable_without_run` (P′); corpus `native_error_with_selected_run` |

## 3. The `capture` (a) record (D36 tracked, not gating)

**How the record was built.** RV78 built it on F′ case 1 as a `solve_native` failure before any Run is recorded:
- case 1 has `run: null` and the attempt has `run_ref: null`;
- stages are preparation completed, native failed, the rest not entered;
- `proof: null` and the result is `capture{storage}`;
- the reason is `source_unavailable/preparation`, per the S06 row "capture, with no Run";
- run 1 is removed from the call, execution order, meter and group.

**Result:** Python, Rust and TypeScript all give **G5 PRODUCT_ATTEMPT**. The readers agree on the outcome. RV78 did not isolate which check fires in each reader; per ROOT, Rust and Python require a Run whenever native was entered. The outcome waits on the D36 ruling on the Run representation, taken together with the producer serializer.

## 4. Weakening: the readers' D37 diffs since `63355a91d2`

| Reader | Lines removed / added | Change | Weakened? |
|---|---|---|---|
| Python | 7 removed, 23 added | The removed `_g5_typed` one-direction mapping (first failed stage → allowed kinds) is replaced by `ERROR_STAGE_RECORDS`. That table whitelists the complete ten-stage tuple per error kind and matches I62's table row for row: capture (a)–(d), proof (a)–(c) with the five observables/G5a outcomes, and abandoned (a)–(c). | No. Every record the old mapping rejected is still rejected. The only newly rejected shape is `capture` with a failed preparation, which the native table assigns to `preparation`. |
| Rust | 13 removed, 59 added | The removed P9 `STAGE8` mapping is replaced by `error_stages`. It keeps the old first-failed rule for each kind and adds the both-directions conditions. | No |
| TypeScript | 0 removed, 21 added | Additions only. | No |

## 5. Method and evidence

**Commands:** as in earlier rounds, on the default toolchain without `DEVELOPER_DIR`.

**Disclosed:**
- **`node_modules`:** linked to READER's link target.
- **WASM assets:** READER's prebuilt `public/wasm-engine` and `public/self-weight-engine` were **copied**, not built. Hashes are unchanged.
- **Probe test files:** RV78's probe test files existed only in `WT/rv78/`, which is deleted.

**Evidence in this folder:**
- **PARITY_TABLE.json:** 305 rows.
- **PROBES_CONFIRM_06.json:** 18 probes with outcomes.
- **INDEPENDENT_CHECKS_06.json.**
- **SHA256SUMS.**

**Scripts** are in `WT/scratch/rv78_reader_confirm6/`. They are unchanged from round 05, apart from the new probe builder.

## 6. For ROOT

Nothing in scope remains open. The `capture` (a) outcome is uniform across the three readers and stays with the D36 ruling.
