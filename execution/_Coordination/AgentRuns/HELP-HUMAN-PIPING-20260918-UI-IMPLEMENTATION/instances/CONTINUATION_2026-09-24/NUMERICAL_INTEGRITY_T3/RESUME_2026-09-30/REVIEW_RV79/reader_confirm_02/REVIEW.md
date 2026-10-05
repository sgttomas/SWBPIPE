# RV79 confirmation review: the Python reader repairs at READER `b36739112a`

RV79 is a TASK (Type 2), resumed by ROOT (HELP_HUMAN, Agent 0) under `BRIEFS/RV78_RV81_CONFIRMATION.md`. ROOT is the return path. RV79 wrote none of the repairs and did not delegate. All evidence below comes from RV79's own probes, mutants and oracle, rerun on the new head, not from the author's tests.

- **Run:** first tool call 2026-10-04T00:26:09Z; report frozen about 00:44Z, about 18 minutes into the 90-minute box. The memory guard (PID 5387) was running.
- **Limits held:** no Git writes or index operations (Git reads used `GIT_OPTIONAL_LOCKS=0`); no install, new tooling, Cargo, native, solver or DEC-025 job.
- **Copies:** two `git archive` copies of the head in WT/rv79, one for mutants and one for probes. Both are deleted afterwards.
- **Paths** use the placeholders WT, P, NUM, R, T3 and VENV.

## Verdict: FAIL

| Severity | Count |
|---|---|
| BLOCKING | 1 |
| SHOULD-FIX | 3 |
| NOTE | 4 |

Every original RV79 finding is now fixed, or superseded by a decision. The one partial exception is the B2d residual (C2). All 27 original probes give the ruled outcome. The arithmetic oracle again shows 0 mismatches, and 17 of the 22 mutants are killed, up from 12.

The FAIL is one new BLOCKING finding (C1). Python implements D4c as written: a case that *claims* `prepared_product_failure` must own that attempt. The converse is not enforced. An unavailable C3 attempt whose case names a C2 cause instead is accepted, and that also skips the whole D4d reason table. So RV79-B1c's false accept (a `preparation` error with a selected Run) reappears by changing a single field. This is a gap in D4c's scope rather than a defect against its text, so ROOT must extend D4c (S06:25–30), probably for all three readers.

## Candidate

| Item | Value |
|---|---|
| READER head | `b36739112a48da40cd22a7a3bdf7691bf2ad4404` (07a); NUM at `c57496275b` |
| `P/core/analysis_runs/retained_precision.py` | `9669b88359f0f40756d0d27d0aa503f2ddd35ce4ac1174363671e12b33c8611e` (was `55736ea65a`) |
| `P/tests/test_retained_precision_contract.py` | `2d2ca008c9cd24f0bb6e70dfcf85c5e65e42b7c20212d3fae1112bda2030a65b` |
| `P/tests/test_retained_precision_schema.py` | `90bbd5660c504d3344ce19756d2b2250f80c0bd5d93b3ec123e5abf10523e504` (unchanged) |
| Corpus 07a / schema | `a6fa398731c35245baca098e322a9846399c7535b2d4cd882a5058010de456c9` (15 / 236 / 19) / `07951edacfedd410c153929ee75bb5bada15dbd222369ec63240c678b233b61c` |
| Python diff since `6b607fd01f` | `git diff 6b607fd01f b36739112a -- P/core/analysis_runs/retained_precision.py`: 488 lines, sha256 `d9e7abd9ca` (bulk copy in WT/scratch) |

**Baseline:** the brief's pytest command gave **328 passed** (`baseline_pytest.log`). The helpers and VENV are unchanged from reader_review_01.

## Dispositions of the reader_review_01 findings

The probe ids refer to `probes_02.jsonl`. Each probe is the reader_review_01 edit, rerun unchanged and compared with the ruled outcome.

| Finding | Disposition | Evidence on `b36739112a` |
|---|---|---|
| B1a source.preparation back-reference | **Fixed** (D4a) | R4 and R4b: G5 PRODUCT_ATTEMPT at `_g5_products` 916 |
| B1b ordinary material basis | **Fixed** (D4b) | R5: G5 PRODUCT_ATTEMPT (910) |
| B1c preparation error with a selected Run | **Fixed when the case claims ppf** (D4d); **bypassed otherwise**, see C1 | R6c: G5 PRODUCT_ATTEMPT (983). The same receipt with a C2 cause passes (`probes_03.jsonl`). |
| B1d run_ref null with a case Run | **Fixed** (D4e) | P_run_ref: G5 PRODUCT_ATTEMPT (912) |
| B1e native error run_ref | **Fixed in code** (D4d, 984–986) | The selected-Run case gives PRODUCT_ATTEMPT (986). The nonselected-Run mismatch has no base and is deferred. |
| B2a phantom group | **Fixed** (D5e) | R8: G5 ATTEMPT (705) |
| B2b candidate with a verification summary | **Fixed** (D5a) | T3: G5 ATTEMPT (611) |
| B2c stop-rule foreign quantity | **Fixed** (D5d) | T2: G5 ATTEMPT (605) |
| B2d escalating failed verification with pass evidence | **Partly fixed** (D5b); see C2 | T1 (`verification_lme` 1): G5 ATTEMPT (616). A verification shared build as the pass evidence still passes. |
| B2e orphan retained diagnostic | **Fixed** (D7) | G4 DIAGNOSTIC (1620) |
| B2f unsourced complete old inventory | **Fixed** (D1 as corrected) | R3b: G3 COVERAGE (1595). The G8 branch is at 1505–1506. |
| B2g published W2 with a zero exponent | **Fixed in code** (D6c, 879) | Reader-logic test `test_ordinary_pass_d6`; no base, deferred |
| B2h `verification_failed` with a completed verification | **Fixed** (D5c) | N13: G5 ATTEMPT (480) |
| S1 G3 placement | **Fixed** (D1) | R1a/b/c: G3 (1608); R3: G3 (1592) |
| S2 within-G5 order | **Fixed** (D3, D17) | D1–D3 dual defects: G5 ATTEMPT (855–864) |
| S3 G5b labelled G5a | **Fixed** (D18; the D10 fallback is phase-labelled) | A1 and A2: G5b SECTION (1310) |
| S4 G0 thresholds | **Fixed** (D2) | G0_case_limit_threshold and D2_receipt_version_true: G0 |
| S5 tests that did not kill implemented rules | **Fixed, except M21** (D13); see C4 | M06, M09, M11, M14, M18 and M22 are now killed. The harness still rehashes with reader functions (N-a). |
| N1 order inside native G5 | **Superseded** (D3 class-1 convention) | `_g5_native` defers native WORK (566–570). The dual-defect behaviour is pinned by the shared corpus and a reader test. |
| N2 untyped G0 | **Fixed** (D2) | N2_producer_list: G0 (1531); AttributeError has joined the fail-closed fallback |
| N3 parity rule 2 / empty inventory | **Superseded** (D1 correction) | PR2: G5a, as ruled |
| N4 open readings | **Superseded** (D6a, D6b, D6c) | T4a passes (D6a: no names-the-case rule). T4b, T4c and T4c2: G5 ATTEMPT. |
| N5 integral float | **Fixed** (D10) | D10_integral_float_counter: G2 ENCODING (173) |
| N6 deferred-base survivors | **Unchanged; accepted** | M05, M12 and M17 still survive (below) |
| N7 dead recheck | **Removed** (D15: the author's choice) | N7_selected_source_ref_foreign is still rejected (G5 ATTEMPT, 586), so no hole opened |

## New findings

| ID | Sev | Where | Evidence | Remedy |
|---|---|---|---|---|
| **C1** | BLOCKING | `_g5_products` 903–904 (D4c), 977–994 (D4d table) | **The converse of D4c is not enforced.** Python checks D4d and the S06 code/phase table only when the case's cause is `prepared_product_failure`. Four probes pass (`probes_03.jsonl`):<br>(1) F′ case 1, whose unavailable C3 attempt failed `capture{storage}` after its certificate, with the case cause relabelled `receipt_failure`;<br>(2) the same with `unavailable_precondition`;<br>(3) **RV79-B1c's receipt again** (a `preparation` error with a selected Run) under a `receipt_failure` cause;<br>(4) P′ case 1 (preparation failure) with an `unavailable_precondition` cause.<br>S06:25–30 makes `prepared_product_failure` the outer cause for an actual unavailable C3 attempt: "Existing C2 cause branches remain unchanged for outcomes **without** an actual C3 product attempt". `receipt_failure` belongs only to a *Ready* attempt whose later encoding failed (S06, the receipt_failure paragraph after the table). | Have ROOT extend D4c: an attempt whose result is unavailable requires its case to be unavailable with `{prepared_product_failure, product_attempt_ref: this attempt}`, and a Ready attempt in an unavailable case requires `receipt_failure`. Apply D4d by the attempt's own error, whatever the case cause. G5 PRODUCT_ATTEMPT, class 2. Add a shared pin for each direction. RV80 and RV81 should check their readers; RV81-B2 showed that the ordinary pass also skips this cause. |
| **C2** | SHOULD-FIX | 613–616 (D5b) | **D5b counts only `verification_lme` as evidence that the pass ran.** Natively the verification pass obtains the `v{P}` shared build inside `verify_precision` (adaptive.rs:4261, 4286). A failed verification *solve* returns before that (4556–4590), so it can have no verification shared build. Probe `D5b_vbuild_on_escalating_failed_verification` adds a consistent `v256` build (id 5, cache_after, shared stages and charges all updated) to the escalating Condition failure on the `verification_failure_skip_synthetic` base, and **Python accepts it**. The control variant (the same build, a non-escalating stop) is rejected. TypeScript rejects this (I64 06d RETURN, item 1). | Also require `verification_shared_build_ref is None` and `verification_shared_lme == 0` on a failed verification record with an escalating stop. Add a shared pin from the probe's edits. |
| **C3** | SHOULD-FIX | 861–864 | **A code changed in the move.** The "selected case has a product attempt" check was G5 PRODUCT_ATTEMPT at 6b607fd01f (old 1453). It now shares a `fail` with D6b in `_g5_ordinary` and reports **ATTEMPT**. Probe `selected_case_without_c3_attempt` (`probes_04.jsonl`: case 0 of `two_case_synthetic` left without its attempt, everything else re-indexed and rehashed) reaches it: G5 ATTEMPT at 863. A missing C3 case→attempt reference is a C3 association (C3:165, 304; D16 "PRODUCT_ATTEMPT for C3 references"), or the G3 case→attempt bijection (C3:302). No shared entry pins it, so readers may differ. | Split the check: `product_attempt_ref is not None` with PRODUCT_ATTEMPT in the association pass (or G3, as ROOT rules); keep the D6b quality rule as ATTEMPT. Pin it. |
| **C4** | SHOULD-FIX | 1271–1272; tests | **M21 survives, but a vector exists.** I62 gives the reason as "no faithful vector". A negative mutation needs only a faithful *base*. `rv79_m21_vector.py` sets E = S/(1+1.5·2^-40) per kind on `ordinary_prepared_synthetic`. It also lowers the echoed member stiffnesses, because that base's G5a lower test binds above S. The reviewed reader reports G5a SCALE at the sanity line (1272); the M21 reader passes sanity and fails later at G8 (`m21_vector_run.txt`). D13 asks for tests that kill surviving mutants wherever the rule is implemented. | Add the vector as a reader-local test (a shared entry is optional, since it carries a second defect behind the first). |
| N-a | NOTE | test harness 92–114 | The shared harness still rehashes with `rp._hash`, `rp._preparation_payload` and `rp._source_hash`. That is unchanged and is RV78's area. RV78's independent rehash in the parity run covers it. | None for Python |
| N-b | NOTE | — | M05 (N17 overshoot), M12 (a budget failure is never cached) and M17 (L = 0) still survive. Each needs a deferred base (≥20B/60B of work, a budget-failed build, or a producer-confirmed memberless node). | Carried with the deferred witnesses |
| N-c | NOTE | 984–986 | D4d's native-error checks (nonselected Run, `run_ref` equal to the Run) are present, but only the selected-Run branch is reachable on a base. A nonselected-Run receipt is deferred (RETURN_B1). | Pin when a nonselected-Run base exists |
| N-d | NOTE | 861–864 | The `rcond_label` check is now in class 2 (D3), not with the native selected summaries. I had suggested class 1; D3 rules class 2 and this is recorded, not challenged. | None |

## The Python diff since `6b607fd01f`: regressions and weakened checks

I read the whole diff (`git diff`, 488 lines). Every removed check has a replacement:

| Removed at `6b607fd01f` | Now |
|---|---|
| G5 run-id contiguity and `execution_order` (old 491–492) | G3 at 1606–1608. It is equivalent: it requires position k = run id k over exactly the cases with a Run, and run origin owner and source stay in class 1 (D1). |
| G5 complete-old length check (old 780) | G3 at 1589–1597, plus G8 at 1505–1506 |
| The trailing per-case block (old 1446–1454) | `_g5_ordinary` 853–864: D6a, `not_attempted`, report, not_required, D6b, rcond |
| The selected-case source-hash recheck | Removed (N7, D15). G1 at 1559–1560 still checks it; the probe shows no hole. |
| The immediate `_at(..., WORK)` build lookups | `_ref` plus deferred WORK (D16, settled reading 4), with dependent checks skipped |
| G8 `next(... strict bracket)` StopIteration | An explicit `need(bracket is not None)` (1375–1376) |

**No check was weakened.** The only behavioural regressions are C3 (one code changed) and M13, which now survives. M13 is an **equivalent** mutant, not a lost test: the new D6c trigger check (876–877) compares the trigger's error with `initial.get("error")`. That is None for a `report` or `not_attempted` initial, and the schema requires the trigger's error. So the W2-precondition rule (874) is subsumed, and `w2_published_without_initial_failure` keeps its G5 ATTEMPT from the D6c line.

**Decisions checked against their text:**
- **Implemented as written:** D1 (with its correction), D2 (with its correction), D3, D4a–e, D5a, D5c–e, D6a–d, D7, D8 (R1′–R4 and kernel scope, `_accounting_rules` 246–293 and 574–576), D10 (corrected), D16, D17 and D18.
- **D5b:** implemented narrowly (C2).
- **D4c:** implemented as written, but the decision's scope leaves the C1 hole.
- **Not re-derived in this box:** the D8 owner table (`_fault_owner` 211–224) against I62's checkpoint-A tabulation, beyond reading that it follows the four owners named in the ruling.

## Mutation testing (the same 22 mutants, re-targeted)

`rv79_mutants_02.py` uses the reader_review_01 edits. M04 and M20 follow the renamed R3 and R2 lines, and every edit applied exactly once. Each run starts from the pristine bytes `9669b88359` and restores them afterwards (verified). Results are in `mutants_02.json`.

| Result | Mutants |
|---|---|
| **Killed (17)** | M01, M02, M03, M04, M06, M07, M08, M09, M10, M11, M14, M15, M16, M18, M19, M20, M22 |
| Survived: deferred base (3) | M05 (N17), M12 (budget-failure caching), M17 (L = 0) |
| Survived: equivalent (1) | M13: subsumed by D6c (see above) |
| Survived: genuine gap (1) | M21: a separating vector exists (C4) |

## Arithmetic

`R/REVIEW_RV79/reader_review_01/rv79_oracle.py` (sha256 `c568e1ba18`), rerun on the new head, gives 0 mismatches on all 20 checks (`oracle_run_02.txt`). The diff does not touch the arithmetic helpers.

## For ROOT to rule on

1. **C1:** extend D4c to the converse (S06:25–30). An unavailable C3 attempt requires a `prepared_product_failure` cause naming it, a Ready attempt in an unavailable case requires `receipt_failure`, and D4d then applies by the attempt's error. This likely affects all three readers.
2. **C3:** the code and gate for a selected case with no C3 attempt: PRODUCT_ATTEMPT (C3:165, D16) or G3 (C3:302 bijection). Not ATTEMPT.
3. **C2:** confirm that the verification shared build is pass evidence under D5b, as TypeScript reads it.

## Evidence (this folder; SHA256SUMS covers every file)

| File | Content |
|---|---|
| `baseline_pytest.log` | the brief's command on the head: 328 passed |
| `rv79_probes_02.py`, `probes_02.jsonl` | the 27 reader_review_01 probes with their ruled expectations, plus 7 new probes (D5b v-build and its control, N7, D4d, D10, D2 and N2) |
| `rv79_probes_03.py`, `probes_03.jsonl` | C1: four converse-D4c probes |
| `rv79_probes_04.py`, `probes_04.jsonl` | C3: a selected case without its C3 attempt |
| `rv79_m21_vector.py`, `m21_vector_run.txt` | C4: the vector that separates the 1+2^-40 and 1+2^-39 sanity factors |
| `rv79_mutants_02.py`, `mutants_02.json` | 22 mutants and their outcomes |
| `oracle_run_02.txt` | the oracle rerun |

**Rerun:** from a fresh `git archive b36739112a` of READER (P/core, tests, fixtures, schemas), with both BIN variables set, run each script with `VENV/bin/python` from P. `rv79_mutants_02.py` takes `<log_dir> VENV/bin/python`. Bulk logs are in WT/scratch/rv79_reader_confirm/.

## Basis read (sha256)

| sha256 | File |
|---|---|
| 1ef5394f0abcd8b2a98af5c4349c19aa9fda6e3c9dca36a81de40a7d3b40919d | R/BRIEFS/RV78_RV81_CONFIRMATION.md |
| c6d2e3c790f7f6d9e312011b9070ba1112a809fcbd19e4ae06b46e32850e395c | T3/ROOT_RULINGS_V1.md (7952 to the end: D1–D18, checkpoint A and the settled readings) |
| c7d7fe253bac1b635d81623970f9f4b8845df6a0351a8ed40549dd109de6c3f2 | R/I62/review_repair_07/RETURN_B1.md |
| 473b8a8ae05e271ea9601bec81714213a61b863586e5f5707d208f854e431d47 | R/I62/review_repair_07/RETURN_B2.md |
| de3cdfd8da4f1537562d979bcfcb2b2d1a5054649702b0f1920bde218b68d227 | R/I62/review_repair_07/RETURN_07A.md |
| 031b2a150545df4d80da8d8078b956e98760e0df757a75f98bd39b914dec42ab | R/I52/reader_contract_seams_06/ADDENDUM.md (1–40) |

The native citations are at NUM `c57496275b`: adaptive.rs:4261, 4286 and 4556–4611.
