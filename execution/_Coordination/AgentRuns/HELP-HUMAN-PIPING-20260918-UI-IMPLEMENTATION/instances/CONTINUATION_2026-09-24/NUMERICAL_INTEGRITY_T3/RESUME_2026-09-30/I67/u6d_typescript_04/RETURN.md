# I67 return: the post-U6f round (04), TypeScript part

I67 is a TASK (Type 2) under ROOT, working to ROOT's message after RV92. The ruling is RR "RV92 (U6f) on the whole of U6: PASS; the post-U6f repair round; U7 preconditions". The findings come from `R/REVIEW_RV92/u6f_01/REVIEW.md`, and I66's account is `R/I66/u6_postf_01/RETURN.md`. I did not delegate.

**Verdict: all three items are done, and every control passes.**
- **Suite:** Vitest gives 3,494/3,494, and `tsc` is clean.
- **Transport:** RV92's 10 transport probes are now refused with Rust's codes, and an untampered transport still passes.
- **Parity:** the 20 shared cases and all 5 declared entries (20 forms) pass, and every `typescript` expectation I66 wrote is confirmed.
- **Existing outcomes:** none changed.
- **Mutants:** 123 of 123 are killed, all by assertion.
- **The case file:** untouched, and it needs no change.

## Basis, host and fence

- **Worktree:** `WT/f2a-carriers`, at `6383e8e70e`, which is I66's round as you committed it. The work is uncommitted, and I made no Git writes; Git reads used `GIT_OPTIONAL_LOCKS=0`.
- **When:** 2026-10-04, about 14:11Z to 14:32Z. The memory guard (PID 5387) ran throughout.
- **Runtime:** unchanged (`_run_records/runtime.txt`).
  - The `P/node_modules` symlink still shows as `??`, so do not stage it.
  - The WASM assets are copied, not built.
  - No install, Cargo, native or DEC-025 job was run.
- **Python check:** a read-only pytest of Python's shared-file consumers on this tree, with no bytecode and no cache.
- **Lanes:**
  - `base4` is a `git archive` of `6383e8e70e`;
  - `mut4` is the candidate, identical to the worktree (`lane_mut4_vs_worktree.txt` is empty).
- **The fence:** 2 U6d files.
  - The new name `sourceContractTransport` is absent at NUM `4e4ca3e44e`, `origin/main`, the facade and the carriers branch.
  - The case file is unchanged (`case_file_sha256_unchanged.txt`).

| File (TS = P/apps/desktop/src) | sha256 | Change |
|---|---|---|
| TS/features/results/numericalResultQuality.ts | `da694f26…` | N-1: the carrier transport route |
| TS/features/results/retainedPrecisionIntegration.test.tsx | `6d22894a…` | Case format v3; the 5 entries; the transport tests |

## The items

### 1. N-1: the carrier transport route runs the reader's transport checks

- **The change:** `sourceContractTransport(source)` is the TS twin of Rust's `semantic_contract::for_source_metadata`.
  - It runs TS's header dispatch, `sourceContract`.
  - An unsupported header is refused with its standing code: `RETAINED_PRECISION_DOWNGRADE_FORBIDDEN` for the guard, `SOURCE_NUMERICAL_CONTRACT_UNSUPPORTED` otherwise.
  - **For the successor, it then awaits the reader's `validateRetainedPrecisionTransport`:** G0–G2, and the base transport metadata on the reader's projection. This is what Rust does.
  - It resolves the route, or rejects with the first code: the reader's own `RetainedPrecisionError` (message = code; its `gate` is kept).
  - Other identities' routes are unchanged.
- **RV92's 10 probes** (both modes; the no-invocation twin is the same bytes for transport) are now refused with Rust's codes:

| Probe | The header route alone | Transport route, TS (= Rust) |
|---|---|---|
| `receipt_sha_zero` (and its no-invocation twin) | retained | `RETAINED_PRECISION_RECEIPT_MISMATCH` |
| `receipt_body_edit`, unsealed | retained | `RETAINED_PRECISION_RECEIPT_MISMATCH` |
| `receipt_empty` (`{}`) | retained | `SOURCE_PRODUCER_CONTRACT_UNSUPPORTED` |
| `transport_no_results_receipt_zero` | retained | `RETAINED_PRECISION_RECEIPT_MISMATCH` |

- **An untampered transport still passes**, both the full statement and the header-only form with no `results`. It is never eligible.
- **A receipt-consistent but base-inconsistent transport** (`contract_evidence.combination_gates` broken) is refused at the reader's G7, compared by gate as N-4 rules.
- **Other identities:** legacy, preview-physics-1 and source-blocks resolve their routes, and a downgraded or `carrier_evidence` header rejects with its code.
- **Product callers: none today.** Every TS consumer of a transported successor is a T6 surface, and each refuses the successor first: either through the shared output refusal, or because the stress-neutral packet header cannot carry the receipt (round 01, F4). The route is the carrier function those consumers, and RV92's harness, compare. When T6 admits successor outputs, its transport points should call it. That joins the T6 notice.

### 2. Case format v3

- **The pins:**
  - the format pin is `I66-U6-CARRIER-CASES-v3`;
  - there are 20 cases (unchanged) and exactly the five fixtures, now including `source_blocks_n05_sparse`, whose raw form reads `source_blocks` unedited.
- **The harness, `applyShared`, now also takes a literal invocation object** (v3, for example `{}`). No IPC capture can carry such an object, so it is offered to the product registration (`registerRetainedPrecision`) after a capture-less delivery. A `"fixture"` invocation, edited as the form says, is still captured through mocked IPC.
- **The consumer reads entries and forms** and checks their structure:
  - the 5 ids;
  - a ruling on each;
  - each form's `expected` has exactly rust, python and typescript;
  - all four subjects are exercised.
- **It implements every v3 value,** not only TS's:
  - `standing`;
  - `transport`, through the new transport route;
  - `binding`: `by_validated_class`, `every_row:<code>`, `every_row:none` and `source_blocks_summary:<code>`, plus the precheck notice when one is given;
  - `summary`: `empty` or `by_validated_class`.
- **The N-4 `scope` sentence** is asserted to be present: inherited differences are not U6's, and G7 parity compares (gate, code).

### 3. The `typescript` expectations I66 wrote: all confirmed, none wrong

| Entry / form | TS expectation | Result |
|---|---|---|
| I67-F1 / edited_row:no_invocation (×2) | standing `needs_recompute`, `RETAINED_PRECISION_VALIDATION_REQUIRED` | passes |
| I67-F2 / none:binding (×2) | `every_row:RULE_QUANTITY_NOT_COVERED`, notice `N_RP_UNVALIDATED` | passes |
| I67-F2 / none:summary (×2) | `empty` | passes |
| I67-F2 / refused_registration: foreign_mode (×1 each mode), edited_model (×2), empty_invocation (×2) | `every_row:RULE_QUANTITY_NOT_COVERED`, `N_RP_UNVALIDATED` | passes (each registration is refused, so no class is registered) |
| F-U6b-2 / unedited (×2) | transport `ok` | passes, now through `validateRetainedPrecisionTransport`, as the entry's corrected text says |
| F5 / edited_row:no_invocation (×2) | `every_row:RULE_QUANTITY_NOT_COVERED`, `N_RP_UNVALIDATED` | passes |
| RV92-N2-N5 / header:token_row on legacy 0.1.0 and on preview-physics-1 | transport `RETAINED_PRECISION_DOWNGRADE_FORBIDDEN` | passes |
| RV92-N2-N5 / source_blocks_summary_row: receipt_member and token_row | `every_row:none` | passes (TS's source-blocks branch is keyed on the dispatched route, which is unsupported) |

The kept extra TS input under I67-F1 (`numerical_quality` rewritten to `checks_passed`) now reads its expectation from that entry's first form.

## Controls

- **Suites:**
  - **Base `6383e8e70e`:** 3,366 tests pass, and the integration file fails to collect. Its v2 consumer cannot read v3; that is the expected break.
  - **Candidate:** **3,494/3,494**, `tsc` 0.
- **Per test** (`compare_outcomes.txt`):
  - **(a) Every other file:** all base tests are identical (3,365 distinct titles; one viewport title occurs twice at base). None changed, and none is new.
  - **(b) The integration file,** against its last full run (round 03, whose file equals `76477534f6`'s, `3437bceb…`): all 100 keep their outcome after the declared renames. Those renames are format v2 → v3, and each v2 entry → its v3 form. There are 28 new tests, all passing:
    - 14 transport tests, plus 1 for other identities;
    - 12 new declared forms, plus the scope test.
  - So no existing outcome changed. The transport refusals N-1 intends are new tests on a new function; no existing test exercised that route.
- **Parity:**
  - TS passes all 20 cases and all 20 declared forms, against its own expectations.
  - Python's `test_shared_carrier_cases_python` and `test_declared_differences_python` pass on this tree (`python_shared_consumers.log`).
  - Rust asserts the same file (I66's round; I ran no Cargo).
- **Mutants** (`mutants_r4.py`, `.json`, `.log`; the mutant lane against the 3 U6d test files and 8 related ones):
  - **123 of 123 killed, all by assertion.** The control passes 361/361.
  - The set is round 03's 118 plus 5 new ones on the transport route:
    - **X01:** the reader's checks not run;
    - **X02:** the checks applied to every identity;
    - **X03:** the downgrade code lost;
    - **X04:** an unsupported header admitted;
    - **X05:** a row-less transport skipped.

## For ROOT

- **Nothing to rule.** The `typescript` expectations are correct, and the case file needs no change.
- **For the T6 notice:** the transport route has no product caller until T6 admits successor outputs.
- **Commit:** stage the 2 files only.

## Records

`_run_records/` holds:
- basis and runtime;
- the diff and the changed-file hashes;
- the case-file hash, unchanged;
- the run scripts;
- base and candidate outcomes, exit codes and `tsc`;
- round 03's integration outcomes;
- the outcome comparison and its script;
- the Python log;
- the lane note;
- the mutant programme, results and log.

All paths in the records are placeholders. SHA256SUMS covers this folder.
