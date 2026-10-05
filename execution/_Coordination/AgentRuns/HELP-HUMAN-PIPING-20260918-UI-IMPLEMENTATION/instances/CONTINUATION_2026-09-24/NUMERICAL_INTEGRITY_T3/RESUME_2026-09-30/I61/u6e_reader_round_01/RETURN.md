# I61 RETURN: U6e, the reader round (F5, RV79-N1, RV80-N2; snapshot 07g)

**Status: complete, with no stop.** All three items are uncommitted in WT/f2a-readers-round, on top of U6a's `844448112f`.
- **All three readers pass snapshot 07g in full.** Results agree on every entry.
- **The two real milestone receipts still pass,** with classes 25/69/3/1 and 25/69/3/2. Eligibility is off and standing is `needs_recompute`.
- **No 07f entry changes its outcome.** The only intended changes, each listed below, are:
  - the six new F5 mutations, which 07f's readers admit and F5 refuses;
  - the corpus's bases, repaired to A2's list (under F5 an unrepaired 07f base is refused).
- **Mutants:** 22 of 22 killed, none by a compile error. This includes RV79's M35 and M37, the two that survived 07f.
- **The completeness flags are untouched** (false until U7), and RV78-N1 is not touched.
- **U4 G5 has not returned during this run,** so there is no boundary state to report.

**Run facts.**
- **Role and basis:** TASK Type 2 under ROOT, with no descendants. The grant is ROOT's message, `BRIEFS/U6_FANOUT_COMMON.md` and `BRIEFS/I61_U6E_READER_ROUND.md` (NUM `4e4b7eb5ad`), and PLAN §6 U6e (D-U6-7).
- **Time:** 2026-10-04, 08:34Z to about 09:25Z.
- **Host:** the memory guard (PID 5387) was running throughout. Cargo ran `--locked --offline` with `CARGO_BUILD_JOBS=4` and `RUST_TEST_THREADS=2`, one job at a time.
- **Git:** no Git writes. Reads used `GIT_OPTIONAL_LOCKS=0`, and `git archive HEAD` made the scratch base and lanes.
- **The non-Rust tooling, all existing:**
  - **Python:** pytest. The checked-JSON and units authorities are I52's prebuilt binaries in WT/targets/i52-readers, as in experiment 03 and U5.
  - **TypeScript:** vitest and tsc in a scratch lane. P's `node_modules` is linked from the main checkout and the prebuilt `public/` is copied. Nothing was built or installed.
- **Other agents' files:** none touched.
- **Writes:**
  - the seven fenced files in WT/f2a-readers-round;
  - WT/scratch/i61_u6e/ (`old` base archive, `stage`, `lane`, `lane_base`, `mut`, `mutts`);
  - WT/targets/i61-u6e/;
  - this folder.

## 1. F5: the readers enforce A2's exact ordinary list (D-U6-7)

**The rule** (decision 2 and A2, RR:8821, amending checkpoint A's D6a, RR:8117): each case's `ordinary_attempts[].diagnostic_refs` must equal exactly:
- the envelope's diagnostics whose `affected_refs` name the case;
- once each, in envelope order;
- excluding the `RETAINED_PRECISION_*` codes.

A T1 (a)-omitted disclosure is absent from the published envelope, so the comparison runs over the published diagnostics, as PP's serializer builds the list.

D6a's unique-and-resolve check stays. Typed references are unchanged and still strict. The failure is G5 `ATTEMPT_MISMATCH` (class 2), at the same point in all three readers.

| Reader | File | Change |
|---|---|---|
| Python | `retained_precision.py`, `_g5_ordinary` | `fail(refs == [...])`, after D6a's check |
| Rust | `retained_precision.rs`, `g5_ordinary` | `exact`, then `fail(list == exact)` |
| TypeScript | `retainedPrecision.ts`, `ordinaryAttempts` | `exact`, then a length and element-wise comparison |

**The corpus (snapshot 07g).**
- **Base repairs.** Every one of the 15 bases listed only the integrity diagnostic, which is 07f's relaxed form. Their lists are now A2's exact list, and their receipt hashes are recomputed. Nothing else changes: the publication hash, classifications and every other byte are asserted unchanged. The before and after lists are in `_run_records/base_repairs_07g.json`.
- **Six new mutations,** each G5 `ATTEMPT_MISMATCH`. 07f's readers admit all six:
  - `f5_ordinary_refs_omit_naming_diagnostic`;
  - `f5_ordinary_refs_out_of_envelope_order`;
  - `f5_ordinary_refs_list_retained_precision_m09` (U1 M09);
  - `f5_ordinary_refs_list_invocation_level_m10` (U1 M10);
  - `f5_ordinary_refs_list_other_case`;
  - `f5_ordinary_refs_relaxed_d6a_form`.

**U1's M09, M10 and M20 are killed at the readers.** Each language has a test on both real milestone receipts that applies the mutant's output, reseals it like a corpus entry, and checks the result:
- M09 (`RETAINED_PRECISION_*` listed) and M10 (diagnostics not naming the case listed) are refused at G5 ATTEMPT;
- M20 (row token "other") is refused at G6 ROW_METHOD_MISMATCH;
- the unedited receipt reseals to itself.

**On M20:** the readers already pinned the row token (07f's `method_wrong` and `method_empty`, G6). The new tests confirm it on the real receipt.

**Tests updated for the amended rule** (a ruled change, not a weakening):
- **Rust** `d6_d7_ordinary_and_diagnostic_relations` and **TypeScript**'s D6a unit test both asserted 07f's relaxed form ("a listed diagnostic of another scope is admitted"). They now assert refusal under F5, and that the repaired base passes.
- **Python** had no such pin.

## 2. RV79-N1: an independent D37 expected table, pinned in the corpus

**The table** (`D37_TABLE.md`) comes from the native sequence alone: the transitions in `PP/retained_receipt.rs` and the stage sequence in `PP/retained_product.rs`, `prepare_owned_case`, `solve_native` and `freeze_candidate`, with line citations. The kinds are C3's. It does not come from any reader.
- **Universe:** 25 well-formed stage records.
- **Kinds:** 9, plus the unknown kind `storage`.
- **Comparison:** it matches I62's RETURN_07F table and all three readers on every record.

**It is pinned as the corpus's new top-level `d37`,** with `stage_order`, `marks`, `records`, `kinds`, `unknown_kinds`, a `records_rule` and a `basis`.

**Every reader's D37 test now takes its expected outcome from the corpus:** 10 kinds × 25 records, accept if and only if the record is listed.

| Reader | Test | What it calls |
|---|---|---|
| Python | `test_error_kind_agrees_with_stage_record_d37` | `_g5_typed` |
| Rust | in-crate `u6e_reader_round_tests::d37_error_stages_matches_the_corpus_table_rv79_n1` | the private `error_stages` |
| TypeScript | new `@internal` `errorStageRecordAgrees` | the predicate `productAttempts` now calls; behaviour identical |

Python's 07f pins on its own table are kept as `…_d37_reader_table_pins`.

**Result:** RV79's M35 (abandoned after a certificate) and M37 (values with values completed) survived 07f. Both are now killed by the corpus-table test, as are all of M33 to M39. Equivalent widenings in Rust (RS03, RS04) and TypeScript (TS04, TS05) are also killed.

## 3. RV80-N2: `integral_receipt`'s receipt-only scope, pinned

**Rust**, in-crate `rv80_n2_integral_receipt_touches_only_the_receipt`:
- a statement whose receipt carries an integral float (`work.charged` as a float), beside a row value `17.0` and a diagnostic number `3.0`;
- the receipt's float becomes the integer;
- every non-receipt member stays byte-identical, and the row's `17.0` stays a float.

This kills M50, normalizing the whole statement (RS05).

**Python**, `test_rv80_n2_integral_normalization_touches_only_the_receipt`: the outermost `_normalize_integrals` call receives the receipt and never the statement.
- Python's M50 analogue is also killed by the shared corpus itself (197 tests), because normalizing a Python statement's rows changes product-attempt checks.

**TypeScript** has no normalization (JSON numbers are already integral by value), so there is nothing to pin.

**No new shared entry.** No shared observable distinguishes the scope in Rust or TypeScript; RV80 found the same. The existing D32 must-pass `integral_float_integers_and_references` stays the shared entry.

## Controls

| Control | Result |
|---|---|
| All three readers pass 07g in full | **Python** 384 passed (base 374, +10). **Rust** `result_export` 163 passed (base 159, +4: 07g slice, F5 milestone, D37 table, RV80-N2). **TypeScript** vitest 444 passed (base 436, +8: six mutations and two tests), tsc 0 errors (base 0) |
| The milestone receipts pass, 25/69/3/1 and 25/69/3/2 | Python `MILESTONE_PINS`; Rust carriers (13) and the F5 milestone test; TypeScript F5 milestone test (counts asserted). All `needs_recompute`, not eligible |
| Parity on every entry | Each reader asserts the corpus's expected first failure on all 274 mutations and passes all 22 must-pass entries and 15 bases; the per-reader G7 entry keeps its `expected_by_reader`. Python's per-entry outcomes are in `PYTHON_OUTCOMES_07G.json`; Rust's 07g slice outcomes in `rust_outcomes_07g.txt`; TypeScript asserts each entry as a named test |
| No 07f entry's outcome changes, except as ruled | `OUTCOME_DELTA_07F_07G.json`: the 07f reader on 07f against the F5 reader on 07g shows **0** changed outcomes over 15 cases, 268 mutations and 22 must-pass entries. The **6** new mutations are admitted by the 07f reader and refused by the F5 reader. Without the base repair, F5 refuses all 15 unrepaired 07f bases, which is why they are repaired |
| Mutants killed | **22/22**, none by a compile error (`mutants_py.json`, `mutants_rs.json`, `mutants_ts.json`). Python 12 (F5 ×4, RV79's M33–M39 verbatim, M50 analogue); Rust 5; TypeScript 5 |
| Nothing weakened; flags untouched | No check removed or narrowed. The two tests that pinned 07f's relaxed D6a form now assert the stricter ruled rule. `_IMPLEMENTATION_COMPLETE`, `IMPLEMENTATION_COMPLETE` and `SUMMARY_COVERAGE_COMPLETE` are unchanged. The fence is respected (7 files) |

## Findings

- **F-1, the base repair.** All 15 synthetic bases followed checkpoint A's relaxed D6a, not A2. Enforcing F5 therefore needed the corpus repair, as well as the new mutations. The repair changes only `diagnostic_refs` and the receipt hash. It is listed per base in `base_repairs_07g.json`, and the outcome of every 07f entry is unchanged.
- **F-2, a new top-level corpus key.** The corpus gains `d37`, with the 07g format note in `SHARED_SNAPSHOT_07G.json`. Python's format test now pins the top-level key set. Rust and TypeScript ignore unknown keys.
- **F-3, a TypeScript internal export.** `errorStageRecordAgrees` is exported `@internal` for the test, following the file's existing pattern. `productAttempts` calls it, so behaviour is identical.

## Records

All paths are placeholders, with no machine paths.

**Top level**
- `SHARED_SNAPSHOT_07G.json`: snapshot metadata, delta and counts. Corpus sha256 `aa8e930e…`.
- `D37_TABLE.md`: the independent derivation.

**In `_run_records/`**
- Code and hashes: `changed_files_sha256.txt` (against `844448112f`), `candidate_code.diff` (code only; the corpus delta is in the snapshot record), `candidate_status.txt`.
- Corpus and outcomes: `base_repairs_07g.json`, `PYTHON_OUTCOMES_07G.json`, `OUTCOME_DELTA_07F_07G.json`, `rust_outcomes_07g.txt`, `test_runs.txt`.
- Scripts: `build_07g.py`, `outcomes.py`, `mutants_py.py`, `mutants_rs.py`, `mutants_ts.py`.
- Mutant results: `mutants_*.json`.

**`SHA256SUMS`** covers the folder.
