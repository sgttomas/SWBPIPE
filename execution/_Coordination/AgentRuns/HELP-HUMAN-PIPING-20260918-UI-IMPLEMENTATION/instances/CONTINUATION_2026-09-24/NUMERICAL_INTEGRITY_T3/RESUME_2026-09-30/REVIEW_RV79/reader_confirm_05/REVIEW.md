# RV79 confirmation review 05: the Python reader at READER `63355a91d2` (snapshot 07e), the final scoped check

RV79 is a TASK (Type 2), resumed by ROOT (HELP_HUMAN, Agent 0) for the final scoped check before reader acceptance (workflow §3). ROOT is the return path. RV79 wrote none of the repairs and did not delegate. All evidence below comes from RV79's own probes and mutants on the new head.

- **Run:** first tool call 2026-10-04T02:19:57Z; report frozen about 02:37Z, inside the 30-minute box. The memory guard (PID 5387) was running.
- **Limits held:** no Git writes or index operations (Git reads used `GIT_OPTIONAL_LOCKS=0`); no install, new tooling, Cargo, native, solver or DEC-025 job.
- **Copies:** four `git archive` copies in WT/rv79: two mutant copies and one probe copy of the head, plus one of the previous head for comparison. All are deleted afterwards.
- **Paths** use the placeholders WT, P, NUM, R, T3 and VENV.

## Verdict: FAIL

| Severity | Count |
|---|---|
| BLOCKING | 1 |
| SHOULD-FIX | 0 |
| NOTE | 2 |

**All three scoped items pass:**
- **S1:** M29 is killed by the three G0 pins.
- **D34:** −0 in both enum/const integer fields fails G2 ENCODING, and +0 behaves exactly as at `abcb16fd27`.
- **The diff:** the Python reader changes only by D34's G2 check. The harness change follows the 07e format rule. Nothing is weakened.
- **Regressions:** all 48 earlier RV79 probes, the E1 probes and the round-04 boolean and −0 probes still agree with their rulings.

**The FAIL is a new BLOCKING finding, X1, outside the stated scope.** I found it while building the D34 controls, and it is **pre-existing**: present at every head I have reviewed.

An unavailable attempt's PublicFailure kind is checked against its stage record only in one direction: first failed stage → allowed errors, over the first eight stages. So when no pipeline stage failed, Python accepts errors such as `g5a`, `observable`, `proof`, `values` and `numeric` that contradict the recorded stages.

This is the same class as RV79-B1c and C1, which D4d and D19 closed for `preparation` and `native` only. ROOT decides whether X1 blocks acceptance or is carried; I rate it BLOCKING by that precedent.

## Candidate

| Item | Value |
|---|---|
| READER head | `63355a91d24dce246f2b85ee4601825824fd1dac` (07e); NUM at `5c5d2a15c4` |
| `P/core/analysis_runs/retained_precision.py` | `3333142b4ca20108deef3ea371f101e21516049255cfac31d59c0dac741a8af5` (was `031334e29f`) |
| `P/tests/test_retained_precision_contract.py` | `e8f22d8bc7e0551cd722b928b13a8c76ee5339035752fd153079c1096fff180c` |
| Corpus 07e / schema | `bbca15d94055227da364ce4b8a7223d1350ac5b85b1219c94b1405b43c56340a` (15 / 263 / 22) / `07951edacf` (unchanged) |
| Diffs since `abcb16fd27` | reader 28 lines (sha256 `91f3036ea7`); tests 76 lines (`bcb8feadd9`) |

**Baseline:** the brief's pytest command gave **365 passed** (`baseline_pytest.log`).

## 1. S1: M29 is killed

- **The mutant set:** `rv79_mutants_05.py` holds the earlier 29 mutants plus three aimed at D34. M26 was re-targeted to the new G2 line.
- **M29** (`_integral` truncating non-integral floats) is **killed by exactly the three new shared pins:** `g0_receipt_version_non_integral`, `g0_case_limit_non_integral` and `g0_invocation_limit_non_integral` (`mutants_05_d32_d34.json`).
- **My round-04 vector** (`m29_vector_on_63355.txt`) still gives G0 in the reviewed reader and G1 under M29, for all three fields.

## 2. D34: −0 anywhere in the receipt

`rv79_probes_d34.py` was run on this head and on `abcb16fd27`, through the shared harness:

| Edit | `abcb16fd27` | `63355a91d2` |
|---|---|---|
| `G5aError.quantity_kind` = −0.0 (on F′ case 1) | PASS | **G2 ENCODING** (1611) |
| `quantity_kind` = 0 / 0.0 / 1 | PASS / PASS / PASS | PASS / PASS / PASS (unchanged) |
| `constructor_counts.directional_springs` = −0.0 (P′ case 1 `source_decline`) | PASS | **G2 ENCODING** (1611) |
| `directional_springs` = 0 / 0.0 | PASS / PASS | PASS / PASS (unchanged) |
| U `corrections` = −0.0 / 0.0 / 0 | G2 / PASS / PASS | G2 / PASS / PASS (unchanged: the U encoding rejects −0 first) |
| A U inside a list (`adapter.counts[0]`) = −0.0 / 0.0 | G2 / PASS | G2 / PASS (unchanged) |
| A **results row** value = −0.0 (outside the receipt) | G5 PRODUCT_ATTEMPT | G5 PRODUCT_ATTEMPT. Unchanged: D34 is receipt-only, and the later failure is the projection-outcome bit check. |

**The order on line 1611:** `_encoding`, then `_need(not _negative_zero(receipt))`, then `_normalize_integrals`. `_negative_zero` (560–565) walks dicts and lists and flags only a float equal to −0, so ints, strings and signed Bits strings are unaffected.

**Mutants aimed at D34 (all killed by `test_negative_zero_anywhere_in_the_receipt_d34`):**
- M30: the check removed;
- M31: `_negative_zero` not descending into lists;
- M32: the check moved after the normalization, which would erase −0 first.

(At unit level, `_encoding` alone still accepts −0 on the enum/const specs, as in round 04. D34 is the separate receipt-wide check that follows it.)

## 3. Diff review `abcb16fd27..63355a91d2`

| File | Change | Assessment |
|---|---|---|
| Reader | `_negative_zero` (560–565); the G2 line now runs encoding, then D34, then D32 normalization (1611) | Correct, as ruled. Nothing removed except the old G2 call, which the extended call replaces. |
| Harness | `_rehash_ref` (07e format rule): an index is a JSON number, never a boolean, finite, integral, ≥ 0 and not −0. Anything else is skipped. Preparation hashes are recomputed only for a resolving, fully prepared attempt; then selected source identities, publication, receipt and `after_rehash`. | Correct. It replaces the round-04 `int()` truncation, which would have taken 0.5 and `true` as indexes. Pinned by `test_rehash_index_rule_07e`. |
| Tests | the D34 test, the index-rule test, and the counts (15/263/22) | Correct |

**Mutants overall:**
- **Killed: 25 of 32.** That is 18 of the earlier 22, plus M23, M26, M27, M29, M30, M31 and M32.
- **Survivors, as before:** M05, M12 and M17 (deferred bases); M13, M24, M25 and M28 (equivalent, for the reasons in confirm_04).

**Regression reruns:**
- `compare_r05.jsonl`: all 48 earlier probes agree.
- `probes_d25_on_63355.jsonl`: every E1 probe agrees.
- `probes_r04_on_63355.jsonl`: booleans fail G0 or G1, and −0 in U fields fails G2, as in round 04.

## New finding (outside the scoped items)

| ID | Sev | Where | Evidence | Remedy |
|---|---|---|---|---|
| **X1** | BLOCKING (by the B1c/C1 precedent; ROOT to rule) | `_g5_typed` 854–872; `_g5_products` reason table | **The error→stage direction is unchecked for most PublicFailure kinds.** Base F′ case 1 has every pipeline stage through the certificate completed, the certificate check passed, observables and G5a not entered, and error `capture{storage}`. Replacing only the error gives **PASS** for each of these (`probes_error_stage.jsonl`):<br>• `g5a{sanity}` with G5a not entered;<br>• `observable{…}` with observables not entered;<br>• `proof{storage}` with the certificate passed;<br>• `values{…}` with values completed;<br>• `numeric{null}` with observables and G5a not entered.<br>Natively (PP retained_product.rs 3469–3543): `Observable`, `G5a` and `Numeric` are returned only after the certificate completed *and* the observables and G5a stages were entered and checked (3529–3543). `Proof` comes from a failing proof-start, projection or certificate stage (3469, 3473, 3515). `Values` comes from a failing values stage (3481). `Abandoned` comes from a maxima or aliases failure (3476–3491). So none of the five receipts can be emitted. S06's table row for these kinds requires "actual respective … errors" and states "Existing stage consistency still applies"; C3:196–201 and 279–287 say the same. `_g5_typed` checks only first-failed-stage → allowed errors, over the first eight stages, and never looks at observables or G5a. D4d added the converse only for `preparation` and `native`. | Have ROOT extend D4d's converse to every kind, at G5 PRODUCT_ATTEMPT in class 3:<br>• `proof` ⇒ the first failed stage is proof_start, projection or certificate;<br>• `values` ⇒ values failed;<br>• `abandoned` ⇒ maxima or aliases failed;<br>• `observable` ⇒ the observables stage and check failed;<br>• `g5a` ⇒ the G5a stage and check failed, with observables passed;<br>• `numeric` ⇒ observables and G5a completed and passed, and the certificate check not passed (or the numeric pass false, as the stage record carries it).<br>Add shared pins on F′ (single-field edits, as in the probe). RV80 and RV81 should check their readers; this is likely shared. |
| N1 | NOTE | — | M24, M25 and M28 remain equivalent, as in confirm_04. M05, M12 and M17 need deferred bases. | — |
| N2 | NOTE | — | Carried: N-a, the harness rehashes with the reader's own functions; RV78's independent rehash covers it. | — |

## For ROOT

- **X1:** extend the D4d converse to `proof`, `values`, `abandoned`, `observable`, `g5a` and `numeric`, and decide whether it blocks acceptance now. It is pre-existing, and it concerns unavailable attempts only, so it cannot make a case eligible. But it is the same false-accept class as B1c and C1, which were treated as BLOCKING.

## Evidence (this folder; SHA256SUMS covers every file)

| File | Content |
|---|---|
| `baseline_pytest.log` | 365 passed |
| `rv79_probes_d34.py`, `probes_d34_probe.jsonl`, `probes_d34_prev.jsonl` | D34 and its +0 controls, on this head and the previous one |
| `rv79_probes_error_stage.py`, `probes_error_stage.jsonl` | X1 |
| `rv79_mutants_05.py`, `mutants_05_d32_d34.json`, `mutants_05_earlier22.json` | 32 mutants, run in two parallel sets on two identical copies |
| `m29_vector_on_63355.txt` | the confirm_04 M29 vector (`R/REVIEW_RV79/reader_confirm_04/rv79_m29_vector.py`) rerun |
| `compare_r05.jsonl`, `probes_d25_on_63355.jsonl`, `probes_r04_on_63355.jsonl` | regression reruns of the earlier probe scripts (unchanged, in reader_confirm_02, _03 and _04) |

**Rerun:** from a fresh `git archive 63355a91d2` of READER (P/core, tests, fixtures, schemas), with both BIN variables set, run each script with `VENV/bin/python` from P. For the D34 comparison, also archive `abcb16fd27`. Bulk logs are in WT/scratch/rv79_reader_confirm05/.

## Basis read (sha256 prefix)

- T3/ROOT_RULINGS_V1.md (`2c0f474eb1`), from "RV79 confirmation 04" to the end;
- R/I62/review_repair_07/RETURN_07E.md (`aceaee9b3d`);
- native code at NUM `5c5d2a15c4`: PP/retained_product.rs 3335–3336 and 3469–3543.
