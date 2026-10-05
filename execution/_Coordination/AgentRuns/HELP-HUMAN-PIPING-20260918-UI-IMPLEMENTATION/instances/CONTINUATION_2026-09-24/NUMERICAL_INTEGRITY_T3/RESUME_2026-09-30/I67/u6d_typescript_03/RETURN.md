# I67 return: the U6d follow-up on the merged carriers head (round 03)

I67 is a TASK (Type 2) under ROOT, working to ROOT's follow-up message after the U6d merge (`52052ece61`) and I66's format-v2 repair (`da274dd961`). The rulings are RR "RV88 on U6a, U6c, U6b (and U6d)…" (the shared declared differences and U6a S-2) and "RV91 on U6d…". I did not delegate.

**Verdict: all four items are done, and every control passes.**
- **Suite:** Vitest gives 3,466/3,466, and `tsc` is clean.
- **The 7 expected failures now pass,** and every other base outcome is unchanged.
- **Parity:** the 20 shared cases and the 4 declared differences agree with Rust and Python.
- **Mutants:** 118 of 118 are killed, all by assertion.
- **No change is needed to the case file,** which I did not touch.

## Basis, host and fence

- **Worktree:** `WT/f2a-carriers`, at `52052ece61` on `codex/piping-f2a-carriers-20261004`. The work is uncommitted, and I made no Git writes; Git reads used `GIT_OPTIONAL_LOCKS=0`.
- **When:** 2026-10-04, about 12:33Z to 12:51Z. The memory guard (PID 5387) ran throughout.
- **Runtime:** I used I66's existing setup (`_run_records/runtime.txt`). The `P/node_modules` symlink still shows as `??`, so do not stage it. The WASM assets are copied (same hashes), not built. No install, Cargo, native or DEC-025 job was run.
- **Python check:** a read-only pytest of Python's shared-file consumers on the same tree, using the existing venv and the I52 CLIs, with no bytecode and no cache.
- **Lanes:**
  - `base3` is a `git archive` of `52052ece61`;
  - `mut3` is the candidate (identical to the worktree: `lane_mut3_vs_worktree.txt` is empty).
- **The fence:** 3 U6d files, 1 product and 2 tests. The case file is unchanged (`case_file_sha256_unchanged.txt`).

| File (TS = P/apps/desktop/src) | sha256 | Change |
|---|---|---|
| TS/features/results/knownSemanticLimitations.ts | `b0af6b5b…` | S-2: the SI table now equals Rust's `si_unit` |
| TS/features/results/retainedPrecisionIntegration.test.tsx | `3437bceb…` | Format v2, the 20 cases, and the declared differences |
| TS/services/retainedPrecisionAnalysisRun.test.ts | `7752a0b2…` | SF-1 tests over the 0.1.0 and 0.2.0 shapes, with a token on a later row |

## The items

### 1. Case format v2

- **The format pin:** `I66-U6-CARRIER-CASES-v2`, 20 cases, and exactly the four fixtures.
- **A new loader, `sharedFixture`:**
  - it checks each fixture's sha256;
  - a `milestone` file gives {source, invocation};
  - a `raw` file is the source itself, with no invocation.
- **Cases and entries are applied by one shared harness (`applyShared`):**
  - edits are applied literally;
  - a case with an invocation is a capture of it through mocked IPC; a null invocation is a delivery without a capture;
  - the standing model carries the requested load cases.
- **All 20 cases pass, the 6 new ones included.**
  - The 6 new ones are `legacy_0_1` and `preview_physics_1`, each with `token_last_row_only`, `receipt_member_empty` and `receipt_member_null`.
  - Each is `unsupported`, with dispatch `RETAINED_PRECISION_DOWNGRADE_FORBIDDEN`. That comes from TS's existing guard (any member, `{}` and null included, or any token row), so no product change was needed.
- **The raw fixtures' guard is reached only through the edit,** as Rust asserts. Unedited, they read `legacy` and `preview_physics`, with no downgrade, and `needs_recompute`.
- **A coverage check:** every case's fixture is in the file.
- **Nothing is weakened:**
  - the exact pins are now exact for v2 (20 cases, four fixtures);
  - the parity assertions are unchanged: standing, dispatch, and never eligible.

### 2. F1 and F2 now come from `declared_differences`, and TS's expectations are confirmed

The TS-local F1 pin is replaced by a consumer of the shared section, mirroring Rust's `u6_declared_differences_rust`.

**The checks on the section itself:**
- the ids are exactly the four ruled entries;
- each has a non-empty ruling;
- `expected` has exactly the keys rust, python and typescript.

**Each entry, on each of its fixtures, against `expected.typescript`:**

| Entry | Subject | TS expectation, as I66 wrote it | Confirmed by |
|---|---|---|---|
| I67-F1:unregistered_invalid_statement | standing | `needs_recompute`, finding `RETAINED_PRECISION_VALIDATION_REQUIRED` | `retainedPrecisionStanding` and `numericalResultStanding` (contract `retained_preview_physics`, not eligible), both modes |
| I67-F2:display_only_binding_precheck | binding | `every_row:RULE_QUANTITY_NOT_COVERED`, notice `N_RP_UNVALIDATED` | `ruleBindingRefusal` on all 98/99 rows; the precheck shows `N_RP_UNVALIDATED` on every row |
| F-U6b-2:python_refuses_transport | transport | `ok` | the header route, then the reader's `validateRetainedPrecisionTransport` |
| F5:refused_statement_binding | binding | `every_row:RULE_QUANTITY_NOT_COVERED`, notice `N_RP_UNVALIDATED` | as for F2, on the edited statement |

- **All four TS expectations are correct as written.**
- **The harness is generic:** it also implements `by_validated_class` over the reader's classes, although no TS expectation uses it.
- **One extra TS input** is kept under the I67-F1 entry and reads its expectation: `numerical_quality` rewritten to `checks_passed`.
- **Rust and Python** were checked on this same tree. Python's `test_shared_carrier_cases_python` and `test_declared_differences_python` pass (`python_shared_consumers.log`: 2 passed). Rust asserts the same file in `u6a_shared_carrier_cases_rust` and `u6_declared_differences_rust`; I ran no Cargo, so that rests on I66's and your runs of `da274dd961` and `52052ece61`.

### 3. S-2: SI unit naming

**Which applies: TS emits only the display label `N_RP_ABSOLUTE`** (D2 §4.9.9's text), never the derivative's disclosure message. TS refuses every successor output, including derivation, so Rust's byte-for-byte disclosure format has no TS emitter.

**The label's unit naming now matches Rust's `si_unit`:**
- `RETAINED_SI_UNIT` is the same nine-entry table: m and mm to m; rad; N and kN to N; N\*m and kN\*m to N\*m; Pa and MPa to Pa. It is read with `Object.hasOwn`, so inherited members such as `toString` are not units.
- As in Rust, a unit outside the table names no bound: the row is labelled `N_RP_NOT_COVERED`. Before, it printed b in the published unit. That case is unreachable from the reader, which classes a row `absolute_verified` only in a normalized unit.

**What still differs from Rust's message, as before, by design (D2 §4.9.9 versus D-U6-2):** the label prints b rounded upward to 3 significant digits in `e+N` form (`upwardBoundText`, never below b; RV91 checked 25,017 values). Rust prints the exact b with `{:e}` digits and the binary64 bits. The unit names are identical.

**Tests:** the table is checked entry by entry, and `degC`, `mode_code`, `unitless`, `toString` and `""` each give `N_RP_NOT_COVERED`. All 69 + 69 milestone labels still name `m`, `Pa`, `N`, `N*m` or `rad`.

### 4. RV91 N-1: SF-1 with the 0.2.0 shape and a later-row token

The SF-1 tests now run over both legacy shapes the builder accepts, 0.1.0 and 0.2.0.
- **The control:** builds a v0.2 record without the member.
- **Refused, each with `ANALYSIS_LEGACY_SOURCE_DOWNGRADE_FORBIDDEN`:**
  - a receipt object;
  - an empty `{}` object;
  - a null member;
  - a W1 token on the first row;
  - a W1 token on the **last row only**.

This is 12 tests, replacing round 02's four 0.1.0 tests under new titles (`compare_outcomes_r3.py` maps them).

## Controls

| Run | Result |
|---|---|
| Base `52052ece61` (lane) | 3,442 passed, 7 failed (the expected 7); `tsc` 0 |
| Candidate (worktree) | **3,466/3,466**; `tsc` 0 |

**Per test** (`compare_base_vs_candidate.txt`, with the declared title mapping):
- 3,437 base tests keep their outcome;
- **exactly the 7 change, from failed to passed.** They are the format pin and the 6 new cases, under the parity block's corrected title ("20 … Rust U6a and Python U6b");
- the 4 TS-local F1 pin tests are replaced by the shared consumer, as directed;
- 21 tests are new, all passing.

**Sweep** (base `52052ece61` against the candidate; `sweep_compare.txt`): all 80 envelopes are identical, the 63 existing-identity ones and the 17 successors.

**Mutants** (`mutants_r3.py`, `.json`, `.log`; the mutant lane against the 3 U6d test files and 8 related ones):
- **118 of 118 killed, all by assertion.** The control passes 333/333.
- The set is round 02's 113, with K17–K20 rewritten for the new table, plus 5 new:
  - **U01:** m missing from the table;
  - **U02:** rad missing;
  - **U03:** the SI units themselves missing;
  - **U04:** inherited members read as units;
  - **U05:** an unnormalized unit labelled unvalidated.

## For ROOT

- **The case file:** no change needed. I66's TS expectations for all four entries are confirmed.
- **Commit:** stage the 3 files only.
- **RV91 confirms.**

## Records

`_run_records/` holds:
- basis and runtime;
- the diff and the changed-file hashes;
- the case-file hash, unchanged;
- the run scripts;
- base and candidate outcomes, exit codes and `tsc`;
- the outcome comparison and its script;
- the Python log;
- the sweep and its comparison;
- the lane note;
- the mutant programme, results and log.

All paths in the records are placeholders. SHA256SUMS covers this folder.
