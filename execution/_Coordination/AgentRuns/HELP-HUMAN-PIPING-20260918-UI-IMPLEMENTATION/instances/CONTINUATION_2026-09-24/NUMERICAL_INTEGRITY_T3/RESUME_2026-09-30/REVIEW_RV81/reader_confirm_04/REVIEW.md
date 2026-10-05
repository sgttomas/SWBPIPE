# RV81 confirmation round 04: the TypeScript reader on snapshot 07d

RV81 is a TASK (Type 2) reviewer working on ROOT's scoped round-04 grant (workflow §3), and ROOT is the return path. RV81 did not write any of the changes and did not delegate the review. Every result below comes from RV81's own probes, mutants and oracle, rerun on the new head. The authors' tests appear only as the kill criterion for the mutants.

**Candidate:** READER `abcb16fd27d7c3ccd019f2533eb261d4c564fdc7`, reviewed from a `git archive` of that commit in `WT/rv81/`. NUM is at `83732c5677`.

**Files under review** (both hashes equal I64's RETURN_07D):

| File | sha256 |
|---|---|
| `P/apps/desktop/src/features/results/retainedPrecision.ts` | `136f39108d30760695e3c632d699462dac66650ae5d364b162931482ad60c8dc` |
| `retainedPrecision.test.ts` | `ca980d70672e87c229e5b30d6b88283ed63ec735a51eca7fa94da2199eedf87e` |

The corpus is snapshot 07d `12da125d9d…`, with 15 cases, 259 mutations and 21 must-pass entries.

**Basis:**
- ROOT_RULINGS_V1.md, from "T1 and T2: I61's analysis and the rulings" to the end;
- I64's RETURN_07D.

**The diff under review:** `a894d9d0ba..abcb16fd27`. The reader gains 5 lines and loses 1; the test file gains 31 lines.

**Host steps, disclosed:**
- `WT/rv81/P/node_modules` is a link to READER's own `node_modules` target.
- READER's prebuilt `public/wasm-engine` and `public/self-weight-engine` were copied into the archive, not built. Their hashes are unchanged.

**Window:** 2026-10-04 01:46Z to about 01:58Z, inside the 45-minute box. The memory guard (PID 5387) was running throughout.

**Limits held:**
- no Git writes or index operations;
- no install, build, Cargo, Python, native, solver or DEC-025 job;
- nothing written in READER.

## Verdict: PASS

| Severity | Count |
|---|---|
| BLOCKING | 0 |
| SHOULD-FIX | 0 |
| NOTE | 3 |

**D31, D32 and D33 are implemented as ruled, and no check is weakened.** In the reader diff, the only removed line is the old version rule that D31 replaces. The test diff only adds lines.

**Runs:**
- The baseline and the final run both give vitest 419/419 and tsc exit 0.
- The arithmetic oracle found 0 mismatches over 12,502 numeric vectors and 12,960 feasibility cases.

## The diff: `a894d9d0ba..abcb16fd27`

| Change | Decision | Assessment |
|---|---|---|
| `:1079–1081`: G8 admits model `schema_version` 0.1.0, 0.2.0 or 0.3.0. The old line admitted only 0.2.0 and 0.3.0. | D31 | Implemented as ruled; every other G8 check is unchanged. |
| `:306–307`: a record reason in the attempt space tagged `verification_estimate` must have kind `force` or `moment`. Otherwise it fails G5 ATTEMPT. The check runs after the D28 layout-row check, inside the same record loop. | D33 | Correct. Verification reasons and logical outcomes copy record outcomes, so the record loop reaches every place the reason can appear. `charge` is left unrestricted, as ruled. |
| Test block "07d round (D31–D33)" | — | Adds lines only: nothing removed, nothing skipped. |

**D32 needed no code change in TypeScript**, and RV81 confirms that by probe below. RV81 also checked the integer sites I64 audited:
- `at`, `uint` and the schema `uint`/`i32` encoding;
- `sourced`, `corrections`, `buildOf` and `data_blocks`;
- the K4 `u32` words.

Each is a value test (`Number.isSafeInteger`, where an unsigned value also excludes −0), not a type test. The `typeof === 'number'` checks that remain are G1 shape tests only.

## D32, checked with RV81's own probes

RV81 serialized the inputs with its own JSON writer, applied its own edits and rehash, and read the results directly. The file is `rv81_confirm04_probes.test.ts.txt`; the results are in `probe_results.json`.

**1. Integral floats are accepted wherever an integer is required.**
- RV81 rewrote every integer in each of the 15 cases' complete envelope and invocation, not only the receipt. That is 15,468 integer tokens per spelling, in two spellings: `7.0` and `7e0`.
- For every case and both spellings:
  - JSON.parse returns values equal to the integer-written input;
  - the reader returns `pass`;
  - its classifications equal the case's expected classifications (deep, key-order independent) and the integer-written run's output.
- This covers every integer and reference in all 15 bases: run, record and build references; counters and limits; coverage bodies; row indices; the K4 integer fields; and `receipt_version`.

**2. −0 is still rejected.** Each field below was written as `-0.0` and the receipt rehashed:

| Field | Result |
|---|---|
| case `source_ref` | G2 ENCODING |
| `product_attempt_ref` | G2 ENCODING |
| `run.case_charge` | G2 ENCODING |
| coverage `body` | G2 ENCODING |
| projection `row_index` | G2 ENCODING |

The un-rehashed case `source_ref` and `product_attempt_ref` probes also give G2. That is consistent with JCS hashing −0 as 0, so their receipt hash still matches. The un-rehashed `case_charge` and `row_index` probes give G1 only because they change a non-zero value to zero.

**3. Booleans are still rejected.**

| Field | Result |
|---|---|
| `source_ref`, `case_charge`, a `run_refs` entry | G1 RECEIPT (shape) |
| `receipt_version`, `work.case_limit` | G0, as D2 requires for a mistyped G0 field |

**4. A float-written reference resolves.**
- On a receipt written entirely with integral floats, a forged preparation hash behind `attempt_ref: 1.0` gives G1. So does a forged source identity behind `source_ref: 0e0`. Both references therefore resolve to their targets.
- A dangling `9.0` attempt `source_ref` gives G5 PRODUCT_ATTEMPT, as D22 requires.
- A non-integral `source_ref: 0.5`, after a receipt-only rehash, gives G2 ENCODING.

## D31 and D33, checked with RV81's own probes

**D31.** The invocation was edited, its digest rebound into the receipt, and the receipt rehashed:

| Model `schema_version` | Result |
|---|---|
| 0.1.0, 0.2.0, 0.3.0 | pass |
| 0.4.0, 0.0.1, 0.1 | G8 INVOCATION |

**D33.** On `p512_ladder_synthetic`, the reason of record 0 and attempt 0 was set to name a real layout row of each kind on body 0:

| Reason tag | translation | rotation | force | moment |
|---|---|---|---|---|
| `verification_estimate` | G5 ATTEMPT | G5 ATTEMPT | pass | pass |
| `charge` (control) | pass | pass | pass | pass |

## Mutants: 47 run, 39 killed, 8 survived

The set is round 03's, re-anchored to the new code, plus R25–R31 for D31, D32 and D33. Each mutant was applied to a clean copy, run with `npm test -- src/features/results/retainedPrecision.test.ts --maxWorkers=2`, then restored and checked by sha256. The evidence is in `mutant_results.json` and `mutants4.py`.

| Mutant | Target | Result and reason |
|---|---|---|
| R25 | D33 check removed | killed (2) |
| R26 | D33 admits `rotation` | **survived: a genuine test gap** (NOTE 1) |
| R27 | D33 admits only `force` | **survived: a genuine test gap** (NOTE 1) |
| R28 | D31 reverted | killed (2) |
| R29 | D31 admits 0.4.0 | killed (2) |
| R30 | `uint` admits −0 | survived: redundant, since the G2 encoding check rejects −0 first (NOTE 2) |
| R31 | G2 encoding admits −0 | survived: redundant, since `uint` still rejects −0 later, at G2 (NOTE 2) |
| M11, M12, R03, R06 | — | survived, for the reasons recorded in round 03: equivalent, equivalent under D29, redundant, and unreachable |
| all others | — | killed |

## NOTEs

1. **The D33 kind set is pinned only at translation.**
   - The reader-local test and the shared pin both use a translation row, so mutants R26 (rotation admitted) and R27 (moment refused) survive.
   - The implementation is correct: RV81's probe shows rotation rejected, and force and moment admitted.
   - Recommend a reader-local pin with a rotation estimate expected to fail and a moment estimate expected to pass. A shared pin would also hold the other readers to the full kind set.
2. **−0 rejection is redundant by design.**
   - Removing it from `uint` alone (R30), or from the G2 encoding alone (R31), is not observable.
   - Removing both kills 3 shared entries: `negative_zero_counter`, `coverage_body_negative_zero` and `coverage_encoding_then_duplicate`.
   - Under that double mutant, a rehashed `source_ref: -0` passes. So the rule is real, and it is pinned as a pair.
3. **Unchanged:**
   - the survivors M11, M12, R03 and R06;
   - the N4 known limit.

## Runs and evidence

| Run | Command (cwd `WT/rv81/P/apps/desktop`) | Result |
|---|---|---|
| vitest_01 / tsc_02 | `npm test -- src/features/results/retainedPrecision.test.ts --maxWorkers=2`; `../../node_modules/.bin/tsc --noEmit --pretty false -p tsconfig.json` | 419 passed; exit 0 |
| mutants | `WT=WT python3 mutants4.py` | 47 run, 39 killed; copy restored |
| oracle and probes (03, 04) | `RV81_SCRATCH=WT/scratch/rv81_confirm04 npx vitest run src/features/results/rv81_oracle_only.test.ts src/features/results/rv81_confirm04_probes.test.ts --maxWorkers=2` (review-only files, removed afterwards) | 0 oracle mismatches; probe outcomes as above |
| the R30 and R31 double mutant, with the suite and the probes (05) | both edits applied, then restored from `git show abcb16fd27` and checked by sha256 | suite: 3 failed; the −0 probes pass or move to G3/G5 |
| vitest_06 / tsc_07 (final, on the restored copy) | same as 01/02 | 419 passed; exit 0 |

**Environment:** Node v24.18.0, npm 11.16.0, vitest 4.1.10.

**In this folder:**
- `REVIEW.md`;
- `rv81_confirm04_probes.test.ts.txt`;
- `probe_results.json`;
- `probe_results_under_R30_R31.json`;
- `oracle_results.json`;
- `mutants4.py`;
- `mutant_results.json`;
- `SHA256SUMS`.

**Bulk, in `WT/scratch/rv81_confirm04/`:**
- `reader.diff` `2a984bd974…` and `test.diff` `e843e19a55…`;
- the run and mutant logs;
- `oracle_vectors.json` `7bd94f31de…`.

**Not done:** Python and Rust were not executed. This round's scope is TypeScript only.
