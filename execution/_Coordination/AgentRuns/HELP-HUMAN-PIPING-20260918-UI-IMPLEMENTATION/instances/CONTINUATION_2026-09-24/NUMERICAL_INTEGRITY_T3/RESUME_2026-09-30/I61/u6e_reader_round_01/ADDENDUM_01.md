# I61 U6e, addendum 01: one count corrected

RETURN.md stays as written. This addendum corrects one number in it:
- no other result, verdict or file changes;
- no rerun is needed.

The basis is ROOT's grant of the 07h repair round (NUM `a45a3201f8`), item 6, after RV90's review (`R/REVIEW_RV90/u6e_reader_round_01/REVIEW.md`, §7, whose re-run of I61's PY_M50 also failed 130 tests).

## §3, Python's M50 analogue: 130 tests, not 197

**Replaces** in RETURN §3 ("RV80-N2"), the Python item: "Python's M50 analogue is also killed by the shared corpus itself (197 tests)".

**The corrected statement:** Python's M50 analogue (`PY_M50_statement_normalized`) is also killed by the shared corpus itself, with **130** failing tests.

**The evidence is already in this folder:**
- `_run_records/mutants_py.json` records `PY_M50_statement_normalized` with `killing_test_count` 130;
- 197 is the count for `PY02_f5_retained_included` and `PY04_f5_case_filter_removed`, in the same file. The RETURN copied the wrong row.

The conclusion is unchanged: normalizing a Python statement's rows changes product-attempt checks, so the shared corpus kills the analogue as well as the local scope test.
