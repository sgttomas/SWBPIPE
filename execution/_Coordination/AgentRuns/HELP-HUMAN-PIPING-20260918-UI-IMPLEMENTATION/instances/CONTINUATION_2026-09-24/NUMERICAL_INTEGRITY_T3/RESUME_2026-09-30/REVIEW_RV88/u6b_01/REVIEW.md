# RV88 review of U6b (`c89a7a986c`): the Python carriers, with D-U6-9

RV88, the standing independent U6 reviewer, is a TASK (Type 2) under ROOT (HELP_HUMAN, Agent 0), working to ROOT's U6b message, `BRIEFS/U6_FANOUT_COMMON.md` and PLAN §6 U6b and §1b. ROOT is the return path, and RV88 did not delegate.

RV88 did not write this code and did not use I66's tests as oracles. Every control below is RV88's own script or probe. I66's tests and mutants were rerun only as items to report.

## Verdict: PASS, with 0 BLOCKING, 1 SHOULD-FIX and 3 NOTE findings

- **Existing behaviour is unchanged.**
  - RV88's own sweep through the Python carriers covered 69 existing-identity envelopes, including legacy 0.1.0, with 6 injected forms each. It is identical to base apart from the intended guards.
  - The 24-file sweep, plus the retained schema and carrier tests: no test changes its outcome; 21 new tests pass; the remaining 19 failures are the same archive-lane handoff artefacts at base and candidate.
- **Dispatch, standing, binding and AnalysisRun behave as ruled.** RV88's 497 checks pass, in both modes:
  - standing comes from the receipt only (the seam ignores `numerical_quality` and respects case order);
  - binding equals the receipt's own `absolute_verified` list, row for row;
  - AnalysisRun refuses 6 receipt mutations and a base record carrying a receipt or a null member.
- **D-U6-9 and F-U6b-3 hold:** the 0.1.0 wrapper refuses a receipt, or a null member, and the v0.2 builder refuses one.
- **The 14 parity cases:** RV88's own Python mapping equals the shared expectations. Those are the values Rust U6a asserts and that RV88's TS mapping reproduced in U6d.
- **The pin patch keeps exact (set) equality.**
- **S-1:** a legacy 0.1.0 source with a W1 token on a row is **accepted** by Python's dispatch, its v0.2 AnalysisRun builder and the 0.1.0 wrapper. **Rust U6a and TS U6d refuse it.** That is an undeclared cross-language difference of the kind D2 §4.7 forbids. It is not a reliance path (legacy is never Current).
- **Mutants:** I66's suite kills all 15 sampled (every 4th of its 60), none by an error; RV88's checks also kill 10 of them. Of RV88's own 6, RV88's checks kill all 6 and I66's suite kills 4; **Q02 (the token guard reads only the first row) and Q06 (requested refs order-insensitive) survive I66's suite** (N-3).

## Basis and host

- **The copies:** `c89a7a986c` (candidate) and `cb03315779` (base) were copied by `git archive`, excluding the records tree. They went to `WT/rv88/b_cand` and `WT/rv88/c_cand`, with a mutant copy in `WT/rv88/b_mut`.
- **The candidate's files match I66's record.** The 3 changed product and test files equal I66's `changed_files_sha256.txt`; the pin-test patch is the 4th file.
- **Python:** `REPO_ROOT/projects/chirality-piping/.venv` with `PYTHONDONTWRITEBYTECODE=1`, and the checked-JSON and units CLIs built `--release` from the U6c archive (core crates unchanged). The memory guard ran throughout.
- **When:** 2026-10-04, about 11:07Z to 11:45Z.
- **Not run:** no native, solver-at-scale or DEC-025 job, and no install.

## 1. Existing behaviour

**RV88's carrier sweep** (`rv88_py_carrier_sweep.py`, both lanes, base APIs only; `rv88_py_sweep_compare.py` → `pysweep_summary.txt`).
- **What it walks:** every JSON file under P (excluding `execution`), recursing to depth 9 into objects with a producer id, plus legacy 0.1.0 raw envelopes.
- **What it records per envelope:**
  - raw and transport `_source_contract`;
  - `numerical_use_standing` with the quality refs, with none, and with the sibling invocation;
  - `standing_reason` and freshness;
  - every row's `rule_binding_refusal`;
  - `build_analysis_run`, then `validate_analysis_run_v0_3`;
  - `build_analysis_run_v0_2`;
  - the 0.1.0 wrapper.
- **What it injects into each existing envelope:** a receipt, a null member, the token on the first row and on the last row, another method string, and the R-2 notice.

| Result | Count |
|---|---|
| Existing-identity envelopes, identical to base on every recorded outcome | **69/69**: load-reference-1 ×6, load-reference-source-1 ×10, physics-1 ×6, physics-source-1 ×14, precision-1 ×4, preview-physics-1 ×7, source-blocks-1 ×15, legacy ×7 |
| `!receipt` and `!null`: raw refusal `RETAINED_PRECISION_DOWNGRADE_FORBIDDEN`, standing `unsupported`, AnalysisRun refused, 0.1.0 wrapper refused | 69/69 each. At base, 30 were admitted by raw dispatch. |
| `!token0` and `!tokenlast`: raw refusal `RETAINED_PRECISION_DOWNGRADE_FORBIDDEN`, standing `unsupported`, AnalysisRun refused | 60/69 each. Of the other 9: **7 are the legacy 0.1.0 envelopes, which are admitted (S-1)**, and 2 preview-physics-1 envelopes have no rows, so no token was injected. |
| `!othertoken` and `!r2notice`: identical to base | 69/69 each |
| The 17 successor envelopes | base: `SOURCE_PRODUCER_CONTRACT_UNSUPPORTED`. Candidate: the successor tuple, `needs_recompute`, and an AnalysisRun that builds and validates. |

**The suite sweep** (24 files, plus `test_retained_precision_schema.py` and `test_retained_precision_carriers.py`, in the archive lanes):

| Lane | Passed | Failed | Skipped |
|---|---|---|---|
| Base `cb03315779` (24 files plus the retained schema test) | 1,766 | 19 | 30 |
| Candidate `c89a7a986c` (the same, plus the carrier tests) | 1,787 | 19 | 30 |

- **Changed outcomes:** none. That includes the patched pin test, which passes in both lanes.
- **New:** 21, all passing, in `test_retained_precision_carriers.py`.
- **The 19 failures** are the same handoff tests in both lanes. They read execution-record fixtures that the archive excludes. ROOT's in-worktree run (1,806 passed, 0 failed) equals 1,787 + 19.

## 2. Dispatch, standing, binding, summary and AnalysisRun (`rv88_u6b_checks.py`: 497/497 pass; facts in `u6b_facts.tsv`)

All checks run in both modes.
- **Dispatch:** the tuple is `(successor id, c74742ce…)`.
- **Standing:**
  - `needs_recompute` with no invocation, with the invocation, and with the invocation but empty refs;
  - an edited row gives `unsupported`, with and without the invocation;
  - a foreign mode gives `unsupported`.
- **The seam** (`_retained_standing_from`, eligibility forced on a real validation):
  - it gives `numerically_eligible` with `numerical_quality` replaced by `{}`, by a `checks_passed` claim, or by `failed`; quality is never read for a selected case;
  - a two-case receipt is eligible with its refs in order, and `needs_recompute` with them reversed.
- **Binding, against the receipt's own `absolute_verified` list (69 per mode):**
  - every listed row gets `RULE_QUANTITY_BELOW_VERIFIED_FLOOR`, and every other row `None`;
  - a refused statement gets `RULE_QUANTITY_NOT_COVERED` on every row (as Rust F5);
  - the projection gets `None` on every row;
  - a relabelled successor gets `None` on every row, as in Rust (dispatch and standing refuse it).
- **Summary:** 25/69/0/0/3/1 (dense …/2), with 97 withheld. That equals Rust U6a, and the Python reader's own class counts.
- **AnalysisRun:**
  - the copy equals the source receipt, and the record validates; `verify_analysis_run_record` gives `match`;
  - a dropped, null, `receipt_sha256`-edited, body-edited, extra-key or other-mode copy is refused `ANALYSIS_RETAINED_PRECISION_RECEIPT_MISMATCH`. The other-mode case is the one the schemas cannot catch (U6c N-2).
  - A preview-physics-1 record carrying a receipt, or a null member, is refused `ANALYSIS_RETAINED_PRECISION_DOWNGRADE_FORBIDDEN`.
  - Python's `!=` treats `1 == 1.0`, so the representation exposure in U6a's N-5 does not arise here.
- **D-U6-9:**
  - the wrapper refuses the successor;
  - it builds the plain legacy fixture (`invented_mechanics_result.json`);
  - it refuses that fixture with a receipt or a null member (`ANALYSIS_RETAINED_PRECISION_DOWNGRADE_FORBIDDEN`).
- **F-U6b-3:** `build_analysis_run_v0_2` refuses the legacy fixture with a receipt or null (`ANALYSIS_LEGACY_SOURCE_DOWNGRADE_FORBIDDEN`).
- **Relabel forms (80 per mode: 8 identities × 2 profiles × 5 forms):**
  - every one is refused at dispatch, and its standing (with the invocation) is `unsupported`;
  - every form carrying a member gets `RETAINED_PRECISION_DOWNGRADE_FORBIDDEN`;
  - the rest are refused earlier by the identity's own checks (`SOURCE_FORMULATION_BASIS_UNSUPPORTED` 17, `SOURCE_PHYSICS_CONTRACT_MISMATCH` 8, `SOURCE_PRODUCER_CONTRACT_UNSUPPORTED` 4).
- **The R-2 noticed envelope:** dispatch and standing are identical to the plain projection.
- **A legacy 0.1.0 source with the W1 token on a row** (S-1), recorded:
  - raw dispatch is **accepted** as `openpipestress_result_semantics_v0_2`;
  - standing is `needs_recompute`;
  - `build_analysis_run` builds a 0.2.0 record;
  - the 0.1.0 wrapper builds.

## 3. The 14 parity cases and F1 across the three languages

**RV88's own Python mapping** (`case/*` in `u6b_facts.tsv`) gives exactly the shared file's `[expected_standing, expected_dispatch]` for all 14. The mapping is:
- the edits are applied literally;
- the invocation is passed or omitted;
- the requested refs come from the invocation model or the explicit list;
- dispatch is `_source_contract`.

| Input | Rust U6a | Python U6b | TS U6d |
|---|---|---|---|
| The 14 shared cases | as expected (U6a `u6a_shared_carrier_cases_rust`, rerun by RV88) | as expected (RV88) | as expected (RV88's own TS mapping, U6d review) |
| An edited row (an invalid statement) with **no** invocation | `unsupported` (U6a) | `unsupported` (RV88, both modes) | `needs_recompute`, `RETAINED_PRECISION_VALIDATION_REQUIRED` (no registration; I67 F1) |
| A valid statement with no invocation: binding | class-based | class-based | every row refused (`N_RP_UNVALIDATED`; I67 F2) |
| A legacy 0.1.0 source with a W1 token row | refused, `RETAINED_PRECISION_DOWNGRADE_FORBIDDEN` (U6a) | **accepted** (S-1) | refused, route `unsupported` (U6d) |
| A transported successor (no rows) | transport checks only, never eligible | refused, `SOURCE_PRODUCER_CONTRACT_UNSUPPORTED` (F-U6b-2) | transport checks only |

**RV88's judgement on I67's F1, across all three: acceptable as a declared difference, and it should be pinned.**
- **Nothing is ever accepted.** No language makes the input eligible or binds it, so "zero cases accepted by one language and refused by another" (D2 §4.7) holds.
- **TS's `needs_recompute` is the only honest synchronous answer.** TS standing is registration-based by design (C1:162; PLAN §3 rule 1), and no reader has run on the bytes.
- **Reopen does run the reader.** It adds the reader's code as a finding (U6d review §3).
- **What is missing is pinning.** The difference is declared only in I67's RETURN, and no shared case expresses it.

**F-U6b-2** (transport refused in Python) is the opposite direction: Python fails closed where Rust and TS run transport checks. It is declared and tracked to wider F2a (RR "U6b verified"). **S-1 is different from both:** it is an undeclared case accepted by Python and refused by Rust and TS.

## 4. The pin patch

`test_preview_physics_consumer_contract.py:161–163` asserts `FRESH_CONTRACT_IDS == {…}` on the whole set. The set gains only `PREVIEW_PHYSICS_RETAINED_CONTRACT_ID`, imported by name, with a comment. **It keeps exact equality.**

## 5. Mutants (`rv88_py_mutants.py` → `mutants.json`)

Each mutant edits one snippet in `WT/rv88/b_mut`, then runs I66's `TESTS` (the carriers and `test_analysis_run_compatibility.py`) and RV88's checks. RV88's checks kill when a check fails, or when a recorded fact differs from the candidate's; the control is 497/497 with identical facts.

| Mutant | I66's suite | RV88's checks |
|---|---|---|
| D01_no_successor_dispatch | killed | killed |
| D05_transport_admitted_checked | killed | survived |
| D09_no_member_guard | killed | killed |
| D13_is_retained_profile | killed | killed |
| F04_router_old_set | killed | killed |
| S03_reader_error_needs_recompute | killed | killed |
| S06_no_eligibility | killed | killed |
| S10_no_not_required | killed | survived |
| N03_no_basis_check | killed | survived |
| N07_accuracy_unresolved | killed | survived |
| B01_codes_swapped | killed | killed |
| B05_no_binding_branch | killed | killed |
| C04_not_per_case | killed | survived |
| A02_receipt_body_only | killed | killed |
| A06_v02_admits_receipt | killed | killed |
| Q01_member_guard_null_slips | killed | killed |
| Q02_token_guard_first_row_only | survived | killed |
| Q03_ar_equality_by_sha_only | killed | killed |
| Q04_refused_statement_binds | killed | killed |
| Q05_wrapper_null_slips | killed | killed |
| Q06_refs_order_insensitive | survived | killed |


The Q mutants:
- **Q01:** a null member slips past the member guard.
- **Q02:** the token guard reads only the first row.
- **Q03:** the AnalysisRun equality checks `receipt_sha256` only. Under Q03 a body-edited copy is still refused, by the record's own checksum (`ANALYSIS_RECORD_CHECKSUM_MISMATCH`), so the receipt is protected twice.
- **Q04:** a refused statement's rows bind.
- **Q05:** the 0.1.0 wrapper lets a null member through.
- **Q06:** requested refs are order-insensitive.

## Findings

| ID | Severity | Where | Evidence | Remedy |
|---|---|---|---|---|
| **S-1** | SHOULD-FIX | P/core/analysis_runs/compatibility.py:371–374 (the 0.1.0 branch returns) and :404–406 (the token guard); records.py:90–93 | `_source_contract`'s 0.1.0 branch returns before the W1 row-token guard. So a legacy 0.1.0 source carrying `recovery_method: contribution_preserving_multiprecision_v1` on a row is admitted (`openpipestress_result_semantics_v0_2`), stands `needs_recompute`, builds a 0.2.0 AnalysisRun, and is wrapped by the 0.1.0 wrapper. Rust U6a refuses the same input at raw dispatch, standing (`unsupported`) and derive (RV88's U6a `refusals.tsv`, `legacy/token`). TS U6d's guard runs before the version switch, so the route is `unsupported`. This is an undeclared difference, accepted by one language and refused by two (D2 §4.7). No reliance is affected, since legacy is never Current, and the receipt-member guard does cover legacy. | Run the token guard before the 0.1.0 return (raw path only, as now), so a legacy token row gets `RETAINED_PRECISION_DOWNGRADE_FORBIDDEN`, as in Rust and TS. Add a shared carrier case, a legacy source with a token row, with the same expected refusal in all three languages, and a Python test. Optionally the 0.1.0 wrapper refuses token rows too, though D-U6-9 is about the receipt. |
| **N-1** | NOTE (I67 F1, all three languages) | P/fixtures/results/retained_precision_carrier_cases.json | See §3. Acceptable, but declared only in I67's RETURN. | At U6f: add per-language expectations for (a) an invalid statement with no invocation and (b) binding on a valid statement with no invocation. Record both, with F-U6b-2, as declared differences in the parity summary. |
| **N-2** | NOTE (F-U6b-2) | compatibility.py `_retained_contract` | Python has no transport validator, so a transported successor is refused where Rust and TS run G0–G2. This fails closed, is declared and is tracked to wider F2a. | As ruled (wider F2a, before T6 admits successor packages). |
| **N-3** | NOTE (test gap) | P/tests/test_retained_precision_carriers.py | Two of RV88's mutants survive I66's suite, and RV88's checks kill both: **Q02**, the row-token guard reads only the first row (I66's token-only relabel keeps every row's token, so the first row alone triggers it); and **Q06**, requested refs compared as a set (no two-case statement). This is the same pattern as U6a's S-1 (R01, R05). Conversely, RV88's checks miss 5 of I66's sampled mutants that I66's tests kill (the not-required conjunct and predicate fields, transport and per-case summary), so the coverage is complementary. | Add a relabel with a token on a non-first row only, and a two-case seam test with the refs reversed, to `test_retained_precision_carriers.py`. That can go in the S-1 repair. |

## For ROOT to rule

1. **S-1:** a small U6b repair, a few lines plus one shared case, before U6f. Or rule it a declared difference. RV88 recommends the repair, because it is the only parity break found that is accepted by one language and refused by the others.
2. **N-1:** have the shared case file carry the declared differences (I67 F1 and F2, F-U6b-2) with per-language expectations.

## Records

Everything is in `_run_records/`, with placeholder paths only. **The scripts:**
- `rv88_py_carrier_sweep.py` and `rv88_py_sweep_compare.py`;
- `rv88_u6b_checks.py` and `rv88_py_mutants.py`;
- `run_suite.sh`.

**The outputs:**
- `pysweep_{base,cand}.tsv.gz` and `pysweep_summary.txt`;
- `u6b_facts.tsv`;
- `suite_compare.txt` and `suite_*.outcomes`;
- `mutants.json`.

SHA256SUMS covers this folder.
