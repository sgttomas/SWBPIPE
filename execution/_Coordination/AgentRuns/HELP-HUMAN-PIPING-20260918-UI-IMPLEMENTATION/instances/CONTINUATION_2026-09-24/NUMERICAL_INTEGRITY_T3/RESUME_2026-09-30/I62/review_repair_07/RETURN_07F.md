# I62 review repair 07f: D37, the error kind against the stage record

**Basis:** the rulings D35–D37 (NUM `c8e97c6df1`, from "RV78 final check (confirm 05)" to the end).

**Starting state:** READER at `63355a91d2`.

**Run window:** 2026-10-04T02:35:32Z (first tool call) to the 02:42:38Z freeze of SHARED_SNAPSHOT_07F, inside the 60-minute box. The memory guard (PID 5387) was running.

**Limits held:** no Git writes, no Cargo. Writes are within the fence.

**No stop condition was reached.** The native code agrees with D37's text, and D35 is a special case of the table below. The change only tightens a check: in particular, `capture` with a failed preparation stage, which the old one-direction rule admitted, is no longer accepted.

## 1. The native table (read first)

**Source:** PP/retained_product.rs at NUM `c8e97c6df1`. Its sha256 is `d07383fc02`, byte-identical to `652ad0cc1f`.

**Stage transitions** (PP/retained_receipt.rs:45–55):
- `enter` sets *entered*.
- `completed` sets *completed*.
- `checked(stage, i, passed)` sets the stage *completed* or *failed*, and the check *passed* or *failed*.
- `fail_entered`, called on every error return (PP:3257, 3290, 3555), turns each *entered* stage into *failed*.

So a public record holds only *not entered*, *completed* or *failed*. **Stage order:** preparation, native, proof_start, projection, maxima, values, aliases, certificate, observables, g5a.

**Every stage record each error kind can carry** ("through X" means every stage up to and including X is completed, and every later stage is not entered unless named):

| Error kind | Native site | Stage record | First failed / terminal stage | Checks |
|---|---|---|---|---|
| `preparation` | `prepare_owned_case`: enter at PP:3140; error → `fail_entered` at PP:3257 (FailureRef::Preparation) | preparation **failed**; all others not entered | preparation | none (no proof) |
| `native` | `solve_native`: enter at PP:3276; nonselected outcome after the Run is recorded (PP:3286–3288) → `fail_entered` at PP:3290 | through preparation; native **failed** | native | none |
| `capture` (a) | `solve_native` before any Run is recorded (PP:3279–3285) → PP:3290 | through preparation; native **failed** (no Run) | native | none |
| `capture` (b) | `project_candidate` before ProofStart: consumed attempt, owner, outcome, `bind_rows`, `prepared_specs`, MapWrite (PP:3461–3466) | through native; nothing failed | native (terminal) | none |
| `capture` (c) | after a passed certificate (checked passed at PP:3494): frozen value owner (PP:3518–3520) or verdict copy (PP:3522–3525) | through certificate; observables and g5a not entered | certificate (terminal) | certificate passed |
| `capture` (d) | the commit (PP:3546–3549) | all ten completed | g5a (terminal) | all passed |
| `proof` (a) | `begin_prepared_product` (PP:3468–3469) | through native; proof_start **failed** | proof_start | none |
| `proof` (b) | `project` (PP:3472–3473) | through proof_start; projection **failed** | projection | none |
| `proof` (c) | `certify_final` fails: certificate checked failed (PP:3495). If the verdict copy succeeds and covers every row (PP:3501–3502), observables and g5a are entered and checked (PP:3503–3508). Returns at PP:3515. | through aliases; certificate **failed**; observables and g5a either both not entered, or both entered and completed/failed by their checks | certificate | certificate failed; observables and g5a not entered, or checked |
| `values` | `complete_maxima` (PP:3480–3481) | through maxima; values **failed** | values | — |
| `abandoned` (a) | `prepared_maxima` (PP:3475–3476) | through projection; maxima **failed** | maxima | — |
| `abandoned` (b) | `prepared_alias` (PP:3483–3486) | through values; aliases **failed** | aliases | — |
| `abandoned` (c) | `bind_rows_view` (PP:3489–3491), before the certificate is entered (PP:3492) | through aliases; nothing failed | aliases (terminal) | none entered |
| `numeric` | full-case check after both checks (PP:3538–3543) | all ten completed | g5a (terminal) | certificate, observables and g5a all passed |
| `observable` | PP:3528–3543: the observables check failed | through certificate; observables **failed**; g5a entered and completed or failed | observables | certificate passed; observables failed; g5a passed or failed |
| `g5a` | PP:3528–3543: observables passed, G5a failed | through observables; g5a **failed** | g5a | certificate and observables passed; g5a failed |

**Run presence** (D4d) is checked separately:
- `native` needs a nonselected Run;
- `capture` (a) has no Run;
- every other kind except `preparation` has a selected Run.

The `capture`/Run pairing follows S06 §1.

## 2. Python D37 (class 3, G5 PRODUCT_ATTEMPT)

`_g5_typed` now requires the attempt's ten-stage record to be one of the records the table lists for its error kind (`ERROR_STAGE_RECORDS`, the table above as code). This enforces both directions at once:
- the kind is one the first failed or terminal stage can produce;
- every stage the kind presupposes is recorded as entered, completed or passed.

**Checks:**
- Stage ⇔ check consistency is unchanged (class 2, `_g5_stages`).
- The check-wrapper rule (P9) is unchanged.
- No 07e entry changes its outcome under D37. All 263 mutations and 22 must-pass entries were rechecked.

**Reader-local test:** `test_error_kind_agrees_with_stage_record_d37` validates every error kind against every native record, accepting it if listed and rejecting it otherwise. It also pins the D35 facts directly.

**Mutants** (`mutants_07f.txt`): dropping D37, or widening the `g5a`, `numeric` or `proof` records, is killed every time.

## 3. Snapshot 07f (the delta from 07e)

Five new mutations, each on F′ (`two_case_facade_after_certificate_synthetic`) and each expecting G5 PRODUCT_ATTEMPT_MISMATCH. The 07e Python reader accepted all five.

| Entry | Source |
|---|---|
| `g5a_error_with_g5a_not_entered` | RV79 X1; RV78 **Y1** (identical edits) |
| `observable_error_with_observables_not_entered` | RV79 X1; RV78 **Y2** (identical edits) |
| `numeric_error_with_checks_not_entered` | RV79 X1; RV78 **Y4** (identical edits) |
| `proof_error_with_certificate_passed` | RV79 X1 |
| `values_error_with_values_completed` | RV79 X1 |

- **Y1, Y2 and Y4:** their edits are byte-identical to three of RV79's five X1 probes, so each is pinned once rather than duplicated (RV78-N3, pin overlap).
- **Y6** (numeric, all ten stages completed, all checks passed) passes in Python under D37. RV78 recorded it passing in Rust and TypeScript. It was not added as an entry.
- **Everything else is byte-identical** (asserted).

## Results

| File | Before (07e) | Now |
|---|---|---|
| P/core/analysis_runs/retained_precision.py | 3333142b4c… | d77008e24fe1775c9def1e6b838fb535fb3155f1df3f427a227d3d990433228e |
| P/tests/test_retained_precision_contract.py | e8f22d8bc7… | 0485b86198e6e78dc1686ea157e93a8b284c0b649c964ec1a540a6d042aa653b |
| P/fixtures/results/retained_precision_cases.json | bbca15d940… | 2cae6d68231f945e21f883e4b7d4dd7ed0c53e533b7ca7740c2cf402c50fdabe |

These are unchanged: the schema (`07951edacf`) and the schema test.

- **Full suite:** `pytest -q -rA tests/test_retained_precision_contract.py tests/test_retained_precision_schema.py` gave **371 passed, 0 failed** (up from 365) (`pytest_07f.log`).
- **Every entry, installed 07f** (`PYTHON_OUTCOMES_07F.json`): 15 of 15 bases with expected classifications; 268 of 268 mutations at their expected first failure; 22 of 22 must-pass entries.
- **Staged, then installed:** 07f was staged and validated in memory first, and the installed corpus is byte-identical to the stage.

**Counts:** 15 cases, 268 mutations, 22 must-pass entries.

**For I63 and I64:** SHARED_SNAPSHOT_07F.json is here, and READER's corpus hash is `2cae6d6823`.
