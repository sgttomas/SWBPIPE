# RV79 confirmation review 03: the Python reader at READER `a894d9d0ba` (snapshot 07c)

RV79 is a TASK (Type 2), resumed by ROOT (HELP_HUMAN, Agent 0) for a scoped round 03 (workflow §3). ROOT is the return path. RV79 wrote none of the repairs and did not delegate. All evidence below comes from RV79's own probes, mutants and oracle on the new head.

- **Run:** first tool call 2026-10-04T01:10:07Z; report frozen about 01:27Z, inside the 60-minute box. The memory guard (PID 5387) was running.
- **Limits held:** no Git writes or index operations (Git reads used `GIT_OPTIONAL_LOCKS=0`); no install, new tooling, Cargo, native, solver or DEC-025 job.
- **Copies:** two `git archive` copies of the head in WT/rv79 (main and probe), deleted afterwards.
- **Paths** use the placeholders WT, P, NUM, R, T3 and VENV.
- **Out of scope:** T1 and T2 from I61's experiment.

## Verdict: FAIL

| Severity | Count |
|---|---|
| BLOCKING | 1 |
| SHOULD-FIX | 0 |
| NOTE | 4 |

**All four confirm_02 findings are fixed**, and so are the outstanding NOTE items N-c (D30) and C4/M21 (D26). Every one of RV79's 48 probes from rounds 01–03 gives the outcome the rulings now require, 18 of the 22 mutants are killed, and the oracle again shows 0 mismatches.

**The FAIL is one new BLOCKING finding, E1, introduced by implementing D25.**
- Python now accepts integral floats at G2.
- Its G1 integrity guards and G3 coverage guards still test `type(x) is int`, so an index written as `0.0` silently skips them.
- Under D25, `0.0` is the same JSON number as `0` and has the same canonical receipt hash.
- With a forged `source_identity_sha256`, the receipt therefore **passes every gate**, on a selected case.

## Candidate

| Item | Value |
|---|---|
| READER head | `a894d9d0bacc98deb0adcab215d1bc9a91ea4373` (07c); NUM at `19065f4828` |
| `P/core/analysis_runs/retained_precision.py` | `59e5b1cc985be4a915e530a57f27d9bfb05672db4afe4ce77d21de7d8712f465` (was `9669b88359`) |
| `P/tests/test_retained_precision_contract.py` | `bd356816669f15c1e715e7d8c824acced5b1ddc95cedafffefcc917ff6a5fa2b` |
| Corpus 07c / schema | `d33667719e777cd6d6881e207bf359b6f897bdb2f24094781943d44b68ea4b38` (15 / 254 / 19) / `07951edacf` (unchanged) |
| Python diff `b36739112a..a894d9d0ba` | 94 lines, sha256 `058fbdddbb`. Reproduce with `git diff b36739112a a894d9d0ba -- P/core/analysis_runs/retained_precision.py`. |

**Baseline:** the brief's pytest command gave **348 passed** (`baseline_pytest.log`).

## Dispositions of the confirm_02 findings

The probe ids refer to `compare_r03.jsonl`, which merges every RV79 probe with the ruled outcome.

| Finding | Disposition | Evidence on `a894d9d0ba` |
|---|---|---|
| **C1** (BLOCKING), the converse of D4c | **Fixed (D19)** | All four `probes_03` cases give G5 PRODUCT_ATTEMPT at `_g5_products` 918, including `R6c_via_C2cause_preparation_error_selected_run`. So B1c is now closed under every cause. The Ready direction is covered by the reader-local `test_d19_unavailable_attempt_needs_its_cause_and_ready_needs_receipt_failure`: `receipt_failure` is accepted and `unavailable_precondition` rejected. |
| **C2** (SHOULD-FIX), D5b evidence | **Fixed (D21, three indicators)** | `D5b_vbuild_on_escalating_failed_verification` and the new `D21_summary_on_escalating_failed_verification` both give G5 ATTEMPT (620). The control is still rejected. |
| **C3** (SHOULD-FIX), selected case with no C3 attempt | **Fixed (D20)** | `selected_case_without_c3_attempt`: G5 PRODUCT_ATTEMPT (908, class-2 case pass, after the ordinary pass) |
| **C4** (SHOULD-FIX), M21 | **Fixed (D26)** | M21 is now killed by the shared pin `g5a_sanity_margin_between_2m40_and_2m39`. The vector rerun gives the reviewed reader G5a SCALE (1285) and the M21 reader G8 (`m21_vector_on_a894.txt`). |
| N-a: harness rehash uses reader functions | Unchanged; RV78's area | — |
| N-b: deferred-base survivors | Unchanged | M05, M12 and M17 (below) |
| N-c: native error `run_ref` on a nonselected Run | **Fixed (D30)** by a reader-local test | `test_native_run_ref_on_nonselected_run_d30` |
| N-d: `rcond_label` in class 2 | Recorded (D3) | — |

**New-decision probes** (`probes_r03.jsonl`, each agreeing with its ruling):
- D28: a `verification_estimate` or `charge` reason with a foreign quantity, and a stop rule naming another body or another kind, each give G5 ATTEMPT (607).
- D23: a noncanonical `kernel_member` gives G3 (1606).
- D29: an empty body inventory gives G3 (1592), alone and together with an empty coverage roster (that is my old PR2, now G3).
- D25: `17.0` is accepted with identical classifications; `17.5` fails G2.

## Diff review `b36739112a..a894d9d0ba`

| Change | Lines | Assessment |
|---|---|---|
| D25: G2 accepts integral floats again (`type in (int, float)`, with integrality and range kept) | 173 | As ruled. **But the integer guards elsewhere were not updated:** E1. |
| D28: every attempt reason carrying a quantity must resolve to a layout row with the same body and kind | 603–607 | Correct. The schema requires `body` and `kind` on all four reason tags that carry a quantity (Reason oneOf[4] and [6]), so the check cannot crash on a valid receipt. |
| D21: three pieces of evidence that the pass ran | 618–620 | Correct |
| D20: the attempt reference moves out of D6b into the class-2 case pass | 867; 907–908 | Correct, with the D17 order kept |
| D19: both directions, per attempt, before D4b–D4e; the D4d table then applies to every unavailable attempt | 916–921 | Correct |
| D29: a non-empty body inventory per CaseSource, plus the roster guard | 1592; 1619 | Correct |
| D23: id sequence compared with `kernel_member` | 1606 | Correct, but skipped for a float `source_ref` (E1) |

Apart from the ruled D25 relaxation, **no check was removed or weakened**. The only regression is E1, an interaction of that relaxation with unchanged integer-only guards.

## New findings

| ID | Sev | Where | Evidence | Remedy |
|---|---|---|---|---|
| **E1** | BLOCKING | `_validate_draft` 1572 (G1 source identity), 1578 (G1 preparation hash), 1606 (G3 D23), 1617 (G3 coverage roster) | **Integer-only guards skip integrity and coverage checks for integral-float references (a D25 regression).** Each probe edits the parsed base and then re-hashes only the receipt. Under D25 the receipt hash is unchanged by `0` → `0.0`. Results (`probes_d25.jsonl`):<br>(1) **A forged `source_identity_sha256` on a selected case, with `source_ref: 0.0`, PASSES every gate** (standing `needs_recompute`, so eligible once the hold is lifted). The control with `source_ref: 0` fails G1 (1573). The G5 recheck that would have caught it was removed in round 02 (N7, D15).<br>(2) A forged preparation hash with `attempt_ref: 1.0` (the source identity re-derived so only the preparation check can catch it): **PASS without an invocation**, G8 PREPARATION with one. The control gives G1 (1581).<br>(3) A noncanonical `kernel_member` (D23) or a foreign coverage body, with the attempt's `source_ref: 0.0`: G5a SCALE instead of G3 COVERAGE.<br>(4) `source_ref: 0.0` alone passes with identical classifications, which is correct under D25. | Treat any value that passed G2 as an index: use a helper such as `_index(x)` (`type(x) in (int, float)`, finite and integral) and `int(x)` at 1572, 1578, 1606 and 1617. Simpler and more robust: normalize integral floats to int once, in the `deepcopy` snapshot after G2, so no later guard sees a float. Add reader-local tests for (1)–(3). A shared pin needs a format step that re-hashes only the receipt after an index edit, or a harness that indexes by value. |
| N-e | NOTE | test harness 106, 111 | The Python harness indexes lists with the reference (`body["sources"][case["source_ref"]]`), so a shared entry whose reference is an integral float would crash the harness instead of reaching the reader. It is not D25-ready, and no current entry uses such a value. | Index by value in the harness (RV78/I62). |
| N-f | NOTE | — (Rust, RV80's area) | D25 rules that readers validate values. If Rust's typed (serde) parse refuses `0.0` for a U field, then the valid `float_ref_alone` receipt is accepted by Python (and, by D25, TypeScript) but rejected by Rust. I did not run Rust. | RV78 or RV80 to confirm, and pin a valid integral-float receipt. |
| N-a, N-b | NOTE | — | As in confirm_02: the harness rehashes with reader functions; M05, M12 and M17 need deferred bases. | — |

## Mutation testing

The same 22 mutants (`rv79_mutants_03.py`) were run; every run started from the pristine bytes `59e5b1cc98`, which were restored and verified afterwards. M10 was re-targeted to the new roster line: the D29 guard is kept and only the ordering is dropped. Results are in `mutants_03.json`.

| Result | Mutants |
|---|---|
| **Killed (18)** | M01–M04, M06–M11, M14–M16, M18–M22 (M21 newly, by the D26 pin) |
| Survived: deferred base (3) | M05 (N17 overshoot), M12 (budget-failure caching), M17 (L = 0) |
| Survived: equivalent (1) | M13: the D6c trigger check (881) subsumes the W2-precondition rule |

## Arithmetic

`R/REVIEW_RV79/reader_review_01/rv79_oracle.py` (`c568e1ba18`) gives 0 mismatches on all 20 checks (`oracle_run_03.txt`). The diff does not touch the helpers.

## For ROOT

- **E1:** a repair of Python's integer guards in line with D25 (no new contract reading needed), with reader-local tests. Consider a shared pin format that keeps the receipt hash valid across an integral-float index edit.
- **N-f:** confirm Rust's handling of integral-float U values under D25.

## Evidence (this folder; SHA256SUMS covers every file)

| File | Content |
|---|---|
| `baseline_pytest.log` | 348 passed |
| `probes_02_on_a894.jsonl`, `probes_03_on_a894.jsonl`, `probes_04_on_a894.jsonl` | the confirm_02 scripts (`R/REVIEW_RV79/reader_confirm_02/rv79_probes_0{2,3,4}.py`, unchanged; sha256 `416f3e6a45`, `ee65c84404`, `f002a102a3`) rerun on this head |
| `rv79_probes_r03.py`, `probes_r03.jsonl` | the D21, D23, D25, D28 and D29 probes |
| `rv79_compare_r03.py`, `compare_r03.jsonl` | all 48 probes against the current rulings: 48/48 agree |
| `rv79_probes_d25.py`, `probes_d25.jsonl` | E1 |
| `m21_vector_on_a894.txt` | the confirm_02 M21 vector script (`28923ea4cc`) rerun on this head |
| `rv79_mutants_03.py`, `mutants_03.json` | 22 mutants |
| `oracle_run_03.txt` | the oracle rerun |

**Rerun:** from a fresh `git archive a894d9d0ba` of READER (P/core, tests, fixtures, schemas), with both BIN variables set, run each script with `VENV/bin/python` from P. `rv79_compare_r03.py` takes the folder holding the jsonl files. Bulk logs are in WT/scratch/rv79_reader_confirm03/.

## Basis read (sha256)

| sha256 | File |
|---|---|
| 2e62fcf7955b80e941a9da8e7101a29cb9d865da00ccfa185d6abbce55c2813c | T3/ROOT_RULINGS_V1.md (8273–8415: D19–D30, 07b, 07c, "All readers on 07c") |
| ddf4e17b58a79dab3ff4c157e92a0126c9cc3c9ee708aee337d2b1b272e518d9 | R/I62/review_repair_07/RETURN_07B.md |
| 271fb79f9cbbe6bcc8668978b703dbe59b7c38c65ebaed371f12c67c49286a08 | R/I62/review_repair_07/RETURN_07C.md |
