# I66 return: the N-9 scope sentence (records only)

I66 is a TASK (Type 2) under ROOT. This round follows ROOT's message "one small records change before U6 merges" and RR "RV92 on the post-U6f round: one transport residue, declared by a scope sentence". I66 did not delegate.

**Verdict: done.** The case file's `scope` gains the N-9 sentence, and the three consumers assert it, one line each. Every run passes, and no other outcome changed.

## Basis and fence

- **Worktree:** `WT/f2a-carriers` on `b10ee5cf08`, uncommitted. No Git writes; Git reads used `GIT_OPTIONAL_LOCKS=0`.
- **When:** 2026-10-04, about 14:45Z to 15:00Z. The memory guard (PID 5387) was running.
- **The fence held: 4 files, one line each.** The TS line is the ruled fence extension.

| File | sha256 | Change |
|---|---|---|
| P/fixtures/results/retained_precision_carrier_cases.json | `bbc05bd254964739fbf3c21a4c7a97cfda00deb72982a8508f1070197821f12f` | `scope` gains two sentences. Nothing else changes; the file still round-trips at indent 2. |
| P/core/reporting/result_export/tests/retained_precision_carriers.rs | `3655d7db9448fd7b110408b0f17c6dc98cafa9111c4c30e23dce784409a87750` | The scope assertion now also requires "parity there compares only accept against refuse". |
| P/tests/test_retained_precision_carriers.py | `21d7042b7e1d6f3017a4a8f56dcae5bce07e7c6214b74099766167560a71b024` | The same. |
| P/apps/desktop/src/features/results/retainedPrecisionIntegration.test.tsx | `b02d71285a8eafb7d3f3e5ede27e300cba2ef3fc2f2c311fee174097fbb13307` | The G7 regex now also requires the new phrase after it. |

## The added text

The text below is appended to `scope`, after the N-4 sentence. It is worded against RV92 u6f_02 N-9 (a) and (b):

> Rust's header dispatch checks only that a preview contract_evidence is an object, so TS's transport route, like Python's transport check, refuses evidence content that Rust's transport accepts (for example a successor's contract_evidence with a key added and its hashes made consistent, which TS refuses at the reader's G7). A transport refused by a header check before the reader runs carries each language's own code (TS SOURCE_NUMERICAL_CONTRACT_UNSUPPORTED; Rust the reader's G0 code or its base header code; Python F-U6b-2's code), so parity there compares only accept against refuse (ROOT_RULINGS_V1 "RV92 on the post-U6f round: one transport residue, declared by a scope sentence"; RV92 u6f_02 N-9 (a), (b)).

## Runs (`_run_records/suite_totals.txt`)

- **Desktop Vitest, the three files that read the case file:**
  - `retainedPrecisionIntegration.test.tsx`;
  - `retainedPrecisionOutputRefusal.test.tsx`;
  - `retainedPrecisionAnalysisRun.test.ts`.

  Base `b10ee5cf08` and candidate both give **224 passed of 224, with the same test names and 0 changed outcomes** (`vitest_compare.txt`, `vitest_outcomes.tsv`).
  - Node v24.18.0 and Vitest 4.1.10, through the worktree's linked `node_modules`.
  - The base lane is a `git archive` of `b10ee5cf08`, with `node_modules` linked to `REPO_ROOT/P/node_modules`. The prebuilt WASM assets and their glue were copied from the worktree's `apps/desktop/public`, not built. No install.
- **Python `test_retained_precision_carriers.py` and `test_retained_precision_schema.py`:** 78 passed. Per-test outcomes are identical to the post-U6f run.
- **result_export, every target:** 168 passed. Outcomes are identical to the post-U6f run.
- **Not run:** no npm install, build, native, solver or DEC-025 job.

## Records

`_run_records/` holds:
- the runners;
- the Vitest outcomes and comparison;
- the Python and result_export outcomes;
- the candidate diff against `b10ee5cf08` and the changed-file hashes.

All use placeholder paths. SHA256SUMS covers this folder.
