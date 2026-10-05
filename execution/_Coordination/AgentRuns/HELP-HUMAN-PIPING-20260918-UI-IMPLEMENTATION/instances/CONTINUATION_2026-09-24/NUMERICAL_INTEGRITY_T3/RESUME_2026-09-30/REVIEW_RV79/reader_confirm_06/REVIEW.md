# RV79 confirmation review 06: D37 in the Python reader at READER `85905e95e9` (snapshot 07f)

RV79 is a TASK (Type 2), resumed by ROOT (HELP_HUMAN, Agent 0) for the D37-scoped check (workflow §3). ROOT is the return path. RV79 wrote none of the repairs and did not delegate. All evidence below comes from RV79's own probes and mutants on the new head. New findings are triaged under D36.

- **Run:** first tool call 2026-10-04T02:49:27Z; report frozen about 02:59Z, inside the 30-minute box. The memory guard (PID 5387) was running.
- **Limits held:** no Git writes or index operations (Git reads used `GIT_OPTIONAL_LOCKS=0`); no install, new tooling, Cargo, native, solver or DEC-025 job.
- **Copies:** two `git archive` copies of the head in WT/rv79, deleted afterwards.
- **Paths** use the placeholders WT, P, NUM, R, T3 and VENV.

## Verdict: PASS

| Severity | Count |
|---|---|
| BLOCKING | 0 |
| SHOULD-FIX | 0 |
| NOTE | 2 (tracked under D36, not gating) |

**X1 is fixed by D37:**
- All five of my X1 receipts now fail G5 PRODUCT_ATTEMPT at the D37 check (868), in class 3. The base control still passes.
- Over every error kind against every stage record the class-2 rules admit, `_g5_typed` agrees exactly with my own independent encoding of the native sequence: 250 pairs, 0 mismatches. Its `ERROR_STAGE_RECORDS` is set-equal to my table for every kind.
- **No weakening:** the new rule accepts no pair the removed one-direction rule rejected, and it tightens 110.

**Mutants:** of 7 mutants aimed at D37, 5 are killed. The 2 survivors expose a test that takes its expectation from the reader's own table (N1). That is a tracked test gap; the reader is correct.

## Candidate

| Item | Value |
|---|---|
| READER head | `85905e95e9d5a97224236d8ef817718a044fcd92` (07f); NUM at `d3155b3539` |
| `P/core/analysis_runs/retained_precision.py` | `d77008e24fe1775c9def1e6b838fb535fb3155f1df3f427a227d3d990433228e` (was `3333142b4c`) |
| `P/tests/test_retained_precision_contract.py` | `0485b86198e6e78dc1686ea157e93a8b284c0b649c964ec1a540a6d042aa653b` |
| Corpus 07f | `2cae6d68231f945e21f883e4b7d4dd7ed0c53e533b7ca7740c2cf402c50fdabe` (15 / 268 / 22); schema unchanged |
| Diffs since `63355a91d2` | reader 45 lines (sha256 `e71547fe64`); tests 43 lines (`76e998a57c`) |

**Baseline:** the brief's pytest command gave **371 passed** (`baseline_pytest.log`).

## 1. X1's disposition, and the full error-kind table

**The five X1 receipts** (`R/REVIEW_RV79/reader_confirm_05/rv79_probes_error_stage.py`, unchanged; `probes_error_stage_on_85905.jsonl`), each a single-field edit of F′ case 1's error:

| Error | `63355a91d2` | `85905e95e9` |
|---|---|---|
| `g5a`, with G5a not entered | PASS | **G5 PRODUCT_ATTEMPT** (868) |
| `observable`, with observables not entered | PASS | **G5 PRODUCT_ATTEMPT** (868) |
| `proof`, with the certificate passed | PASS | **G5 PRODUCT_ATTEMPT** (868) |
| `values`, with values completed | PASS | **G5 PRODUCT_ATTEMPT** (868) |
| `numeric`, with observables and G5a not entered | PASS | **G5 PRODUCT_ATTEMPT** (868) |
| control: the base `capture{storage}` | PASS | PASS |

The raise path is `_g5_products` → the typed loop (951) → `_g5_typed` (868). That is class 3, after every attempt's association checks, as D37 requires.

**The full table, both directions** (`rv79_d37_table.py`, `d37_table_run.jsonl`):
- **The record space:** every stage record the class-2 rules admit, which is 25 records. The first eight stages form a completed prefix followed by at most one failed or not-entered stage. Observables and G5a are both not entered, or both entered with each completed or failed, and only when the certificate was entered.
- **The kinds:** each of the 9 error kinds plus an unknown kind, giving 250 pairs.
- **The expectation:** my own encoding of the native sequence, written from my reading of PP/retained_product.rs 3136–3290 and 3456–3549 and not from the reader's table:
  - `preparation`: preparation failed.
  - `native`: through preparation; native failed.
  - `capture`: one of four records — native failed before any Run; through native; through certificate with observables and G5a not entered; all ten completed.
  - `proof`: proof_start failed; or projection failed; or certificate failed, with observables and G5a either not entered or both checked.
  - `values`: values failed.
  - `abandoned`: maxima failed; or aliases failed; or through aliases with nothing failed (`bind_rows_view`).
  - `numeric`: all ten completed.
  - `observable`: observables failed, with G5a checked.
  - `g5a`: through observables; G5a failed.
- **The result:** 0 mismatches, both directions. The per-kind accepted counts are 1, 1, 4, 7, 1, 3, 1, 2, 1 and 0 (the unknown kind). `ERROR_STAGE_RECORDS` is set-equal to my table for every kind, and holds no record outside the class-2 space.

## 2. Diff review `63355a91d2..85905e95e9`

| File | Change | Assessment |
|---|---|---|
| Reader | The one-direction mapping (old 865–871) is replaced by `fail(record in ERROR_STAGE_RECORDS[kind])` (868), with the table built by `_error_stage_records()` (871–891). The check-wrapper rule (P9, 858–862) is unchanged. | Correct (above). **No weakening:** `rv79_d37_old_vs_new.py` reruns the removed rule over the same 225 kind × record pairs. The new rule accepts nothing the old one rejected, and it tightens 110 pairs, including `capture` with a failed preparation stage (`d37_old_vs_new.json`). |
| Tests | `test_error_kind_agrees_with_stage_record_d37`, the five shared X1 pins, and the counts (15/268/22) | Present. See N1 for the test's self-reference. |

Unchanged as ruled: stage ⇔ check consistency (class 2), and the D4d Run rules. The capture (a) Run representation is ROOT's tracked item under D36, so I did not re-examine it.

## 3. Mutants aimed at D37

`rv79_mutants_06.py`; results in `mutants_06.json`. Each run started from the pristine `d77008e24f`, which was restored and verified afterwards.

| Mutant | Result | Killed by |
|---|---|---|
| M33 D37 check removed | **killed** | 7 tests, including the X1 shared pins |
| M34 `observable` loses its (F, F) record | **killed** | the D37 test (hard-coded assertion) |
| M36 `proof` loses its unchecked (N, N) certificate-failure record | **killed** | 5 tests (the `cert_failed_before_summary` family) |
| M38 `numeric` widened to observables and G5a not entered | **killed** | the D37 test and `numeric_error_with_checks_not_entered` |
| M39 `g5a` widened to G5a not entered | **killed** | the D37 test and `g5a_error_with_g5a_not_entered` |
| M35 `abandoned` widened to after a passed certificate (through certificate, observables and G5a not entered) | **survives** | N1 |
| M37 `values` widened to values completed, aliases not entered | **survives** | N1 |

## Findings (triaged under D36)

| ID | Sev | Where | Evidence | Remedy |
|---|---|---|---|---|
| N1 | NOTE (tracked, not gating: test only, unavailable paths) | `test_error_kind_agrees_with_stage_record_d37` | The test takes its "allowed" set from `rp.ERROR_STAGE_RECORDS`, the reader's own table, and iterates only that table's union. It hard-codes only the `g5a`, `observable`, `numeric` and `preparation` rows and one `capture`/`proof` fact. A widening of `abandoned`, `values`, `proof`, `capture` or `native` therefore passes it unless a shared pin covers the exact record: M35 and M37 survive. The reader's table itself is correct, as my independent 250-pair check shows. | In the next reader round, write the expected table literally in the test, independent of `rp`, and iterate the full 25-record class-2 space (as `rv79_d37_table.py` does). |
| N2 | NOTE | — | Carried: the equivalent and deferred-base survivors from earlier rounds (M05, M12, M13, M17, M24, M25, M28) and N-a (the harness rehashes with reader functions). None was re-run here; the scope is D37. | — |

## Evidence (this folder; SHA256SUMS covers every file)

| File | Content |
|---|---|
| `baseline_pytest.log` | 371 passed |
| `probes_error_stage_on_85905.jsonl` | the five X1 receipts and the control (script in reader_confirm_05, unchanged, `295da467c7`) |
| `rv79_d37_table.py`, `d37_table_run.jsonl` | 250 kind × record pairs against RV79's independent native table; per-kind set equality |
| `rv79_d37_old_vs_new.py`, `d37_old_vs_new.json` | the weakening check against the removed rule |
| `rv79_mutants_06.py`, `mutants_06.json` | M33–M39 |

**Rerun:** from a fresh `git archive 85905e95e9` of READER (P/core, tests, fixtures, schemas), with both BIN variables set, run each script with `VENV/bin/python` from P. Bulk logs are in WT/scratch/rv79_reader_confirm06/.

## Basis read (sha256 prefix)

- T3/ROOT_RULINGS_V1.md (`ffb45af8df`), from "RV78 final check (confirm 05)" to the end (D35–D37, 07f);
- R/I62/review_repair_07/RETURN_07F.md (`5ef4b15c11`), with its native table;
- PP/retained_product.rs at NUM `d3155b3539`, 3456–3549 (read by RV79).
