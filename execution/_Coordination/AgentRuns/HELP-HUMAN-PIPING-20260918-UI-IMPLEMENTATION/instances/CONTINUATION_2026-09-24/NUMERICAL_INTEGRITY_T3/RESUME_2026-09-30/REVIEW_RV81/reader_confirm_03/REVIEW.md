# RV81 confirmation round 03: the TypeScript reader on snapshot 07c

RV81 resumes as a TASK (Type 2) reviewer, on ROOT's scoped round-03 grant under workflow §3. ROOT is the return path. RV81 did not write any of the repairs and did not delegate. The evidence comes from RV81's own probes, mutants and oracle, rerun on the new head. The authors' tests serve only as the kill criterion for the mutants.

- **Candidate:** READER `a894d9d0bacc98deb0adcab215d1bc9a91ea4373`, reviewed from a `git archive` of that commit in `WT/rv81/`. NUM is at `19065f4828`.
- **Files under review:**
  - `P/apps/desktop/src/features/results/retainedPrecision.ts` `9a8e6d4aaf0bcaab9fdd088e18039e5d7f9d672718ef7d9e1dec728f59e96000`;
  - its test file `96268c8c81022b4059fc42ec27affc8694b59c9c5cfca05e33b6c9b5277a1948`.
  
  Both equal I64's RETURN_07C. The corpus is 07c `d33667719e…`: 15 cases, 254 mutations, 19 must-pass entries.
- **Diff reviewed:** `b36739112a..a894d9d0ba` for both TypeScript files. The reader changed by +30/−8 lines and the test file by +92/−2.
- **Basis read:** ROOT_RULINGS_V1.md, from "Confirmation findings: disposition D19–D26 and the repair round" through "All readers on 07c"; I64's RETURN_07B and RETURN_07C.
- **Disclosed host steps:** `WT/rv81/P/node_modules` was linked to READER's target. READER's prebuilt `public/wasm-engine` and `public/self-weight-engine` were copied into the archive, not built; their hashes are unchanged.
- **Window:** 2026-10-04 01:10Z to about 01:20Z, inside the 60-minute box. The memory guard (PID 5387) was running throughout.
- **Limits held:** no Git writes or index operations; no install, build, Cargo, Python, native, solver or DEC-025 job; nothing written in READER.
- **Out of scope:** T1 and T2 from I61's experiment.

## Verdict: PASS

| Severity | Count |
|---|---|
| BLOCKING | 0 |
| SHOULD-FIX | 0 |
| NOTE | 3 |

- **Every confirm_02 finding is fixed or recorded as ROOT disposed it.**
- **No check was removed or weakened.** D20's move of a check and D27's change of input are both decision-directed, and neither weakens anything.
- **The new `@internal` `productAttempts` hook gives no path to eligibility.**
- **Baseline and final runs:** vitest 409/409 and tsc exit 0.
- **Oracle:** the arithmetic oracle (12,502 numeric vectors and 12,960 feasibility cases) found 0 mismatches.

## Disposition of the confirm_02 findings

| Finding | Disposition | RV81 evidence on `a894d9d0ba` |
|---|---|---|
| S1: a dangling `source_ref` failed at G3 | **Fixed (D22)** | Probe P17 (source_ref = 9) on two bases gives G5 PRODUCT_ATTEMPT; it gave G3 before. `sourced` (`:239`) resolves the reference, and both G3 source comparisons run only when it resolves (`:240`, `:251`). The mutant R18 (fail at G3 on a dangling reference) is killed by 2 tests. |
| N1: M02, the v-build half of D5b | **Fixed (D21, widened)** | `solveFailure` (`:293`) needs all three: no v-build, `verification_lme` 0, and a null summary. It is used at both sites (`:317`, `:354`). **Probe P18** (v-build reference only, `verification_lme` 0) gives G5 ATTEMPT. With only the v-build indicator removed (M02), the same probe gives G5 WORK, so the indicator decides the outcome. M02, R16 and R17 (one indicator dropped each) are killed. |
| N2: the kernel-scope test did not isolate its rule | **Fixed** | The test now edits the `nonbudget_failure` build and asserts it exists. R11 (kernel scope removed), which survived before, is now killed by 1 test. |
| N3: survivors M11, R03, R06 | Unchanged; the same reasons hold | See the mutant section. |
| N4: no-invocation acceptance of wrong member properties | **Recorded as a known limit by ROOT** | Probe P13 is unchanged: G8 PREPARATION with the invocation, `needs_recompute` without. This matches the recorded limit. |
| N5: no weakened check | Still holds (see the diff review) | — |

## Diff review: `b36739112a..a894d9d0ba`

| Change | Decision | Assessment |
|---|---|---|
| `sourced` resolution; the G3 source-dependent checks are skipped on a dangling reference (`:239–251`) | D22 | Right. A dangling reference falls through to `at(b.sources, a.source_ref)` in the attempt loop, which gives PRODUCT_ATTEMPT. The `else if` for unsourced complete coverage is still reached only when `source_ref` is null. |
| A non-empty body inventory in the G3 sources loop (`:255`) | D29 | Right. Probe P21 gives G3. R19 is killed. |
| `solveFailure` with three indicators (`:293`) | D5b, D21 and its third indicator | Right, and stricter only. |
| The idle-run rule reads the recorded `invocation_before` (`:408`) | D27 | Right. The condition remains, and the WORK chain check above it is unchanged. R20 (the derived value restored) is killed. |
| `productAttempts` exported `@internal` (`:622–623`) | D30 hook, D14 | It reads the body and row map, throws or returns void, and sets no state. Only the two validate entries build a `RetainedPrecisionValidation`, and the hold is still a non-exported `const false` (M01 is killed by 23 tests). Acceptable as a test hook. |
| A new class-2 case pass after D4c (`:631–641`), covering D20 (selected ⇒ an own attempt), D19 unavailable (⇒ `prepared_product_failure` naming it) and D19 Ready (⇒ selected, or a `receipt_failure` cause) | D19, D20 | Right, at G5 PRODUCT_ATTEMPT, after the ordinary pass as D17 requires. **Probe P19**: an unavailable attempt under a `receipt_failure` cause, a C2 branch 07c does not pin, gives G5 PRODUCT_ATTEMPT; with the D19 unavailable half removed (R22), it passes. **Probe P20**: a selected case with no attempt gives G5 PRODUCT_ATTEMPT. R21, R22, R23 and R24 (D20 put back in the ordinary pass as ATTEMPT) are all killed. |
| `c.product_attempt_ref !== null` removed from the ordinary selected check (`:1238`) | D20 | This moves the check, not weakens it. The relation is enforced at `:633` with the decided code, and probe P20 confirms it. |
| Test file | — | Two lines were removed: the old import line (now extended) and the old kernel-scope build edit (now corrected). Everything else is added; nothing is skipped. |

D28 needed no reader change. The locator applies to every record reason that carries a `quantity`. **Probe P22**, with RV81's own variants that are not corpus edits, gives G5 ATTEMPT for both: a `verification_estimate` reason naming a layout quantity on another body, and a `charge` reason with the wrong kind.

## Mutants: 40 run, 36 killed, 4 survived

The run was the confirm_02 set, with M02 and M12 re-anchored to the new code, plus R16–R24 for this round's changes. Each mutant was applied to a clean copy and run with `npm test -- src/features/results/retainedPrecision.test.ts --maxWorkers=2`, then restored and checked by sha256. Evidence: `mutant_results.json` and `mutants3.py`.

| Survivor | Reason |
|---|---|
| M11 (the G8 member-sequence check weakened) | Equivalent. D1 at G3 already requires `0..len−1`, and the weakened G8 check still compares lengths. |
| M12 (`cov.length >= 1` removed at G3) | **Now equivalent, under D29.** An empty roster can match only an empty body inventory, and D29 then fails the same gate with the same code (`:255`). This mutant was killed in confirm_02, before D29 existed. |
| R03 (the `captured_prefix` association line removed) | Redundant: other association checks catch it, as confirm_02 showed by probe. |
| R06 (the bundled table-byte hashes removed) | Cannot be reached from a receipt (accepted). |

## NOTEs

1. **M12 is now an equivalent mutant.** The `cov.length >= 1` term at `:251` is redundant under D29. It could be kept as defence in depth or removed; either way the outcome is the same.
2. **Probes P18 and P19 decide their own outcomes.** With only the rule under test removed, each changes outcome (`probe_results_under_M02_R22.json`). So the D21 v-build indicator and the D19 unavailable half are each pinned by an RV81 vector independent of the corpus.
3. **The remaining survivors** (M11, R03, R06) keep their confirm_02 reasons, and the known limit N4 is unchanged.

## Runs and evidence

| Run | Command (cwd `WT/rv81/P/apps/desktop`) | Result |
|---|---|---|
| vitest_01 / tsc_02 | `npm test -- src/features/results/retainedPrecision.test.ts --maxWorkers=2`; `../../node_modules/.bin/tsc --noEmit --pretty false -p tsconfig.json` | 409 passed; exit 0 |
| mutants | `WT=WT python3 mutants3.py` | 40 run, 36 killed, copy restored |
| oracle and probes (03) | `RV81_SCRATCH=WT/scratch/rv81_confirm03 npx vitest run src/features/results/rv81_oracle_only.test.ts src/features/results/rv81_confirm_probes.test.ts --maxWorkers=2`. These are review-only files, removed afterwards. | 0 oracle mismatches; 39 probe outcomes |
| probes under M02 and R22 (04) | the same probe file, with only those two edits applied; the file was restored from `git show a894d9d0ba` with a sha256 check | P18 gives G5 WORK; P19 passes |
| vitest_05 / tsc_06 (final, restored copy) | same as 01/02 | 409 passed; exit 0 |

**Environment:** Node v24.18.0, npm 11.16.0, vitest 4.1.10.

**In this folder:**
- `REVIEW.md`;
- `rv81_confirm03_probes.test.ts.txt`;
- `probe_results.json`;
- `probe_results_under_M02_R22.json`;
- `oracle_results.json`;
- `mutants3.py`;
- `mutant_results.json`;
- `SHA256SUMS`.

**Bulk, in `WT/scratch/rv81_confirm03/`:**
- `reader.diff` `fdd5add604…` and `test.diff` `07546b76ae…`;
- the run and mutant logs;
- `oracle_vectors.json` `7bd94f31de…`.

**Not done:** Python and Rust were not executed, as this round's scope is TypeScript only.
