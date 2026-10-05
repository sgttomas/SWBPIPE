# I66 U6c, addendum 01: the pin patch applied, and the worktree sweep

This follows RR "U6c returned with a stop…". U6c's fence was extended to exactly the three pin-test files, for the proposed 78-line patch alone.

**Applied.** The patch is in `_run_records/applied_pin_tests.diff`. Its hunks are identical to `proposed_pin_tests.diff`; only the file-path headers differ. It removes no assertion. Each pinned list or count gains the successor (D2 §4.9.6), and T1's "last entry" pins become "last two entries", `[T1's entry, the successor's]`.

| File | sha256 |
|---|---|
| P/tests/test_load_reference_schema.py | `2b28d663…` |
| P/tests/test_load_reference_source_schema.py | `40c9a242…` |
| P/tests/test_source_block_schema_contract.py | `5ee63f64…` |

Before the edit, all three files were byte-identical to `844448112f`.

**The sweep, in the worktree.** I ran the 24 files of `sweep_test_files.txt`, plus `tests/test_retained_precision_schema.py`, in `WT/f2a-carriers` itself, so the execution-record fixtures were present. The setup:
- the same Python lane as before (REPO_ROOT's `.venv`, the I52 CLIs, `PYTHONDONTWRITEBYTECODE=1`);
- the memory guard running.

**The result: 1,785 passed, 30 skipped and 0 failed** (`worktree_suites_with_pins.summary`, per-test list in `worktree_suites_with_pins.outcomes`). The count is the earlier 1,745, plus the 37 retained-schema tests, plus the 3 pin tests that now pass.

**The worktree diff is now 7 files:** the 3 schemas, `tests/test_retained_precision_schema.py` and these 3 pin-test files. Their hashes are in `changed_files_sha256.txt` and `pin_files_sha256.txt`. I made no Git writes.
