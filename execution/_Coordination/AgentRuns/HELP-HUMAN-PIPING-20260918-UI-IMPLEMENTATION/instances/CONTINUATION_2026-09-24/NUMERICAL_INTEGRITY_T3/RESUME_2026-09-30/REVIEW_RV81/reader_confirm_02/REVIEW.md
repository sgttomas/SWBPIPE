# RV81 confirmation review: TypeScript reader repairs on snapshot 07a

RV81 resumes as a TASK (Type 2) reviewer under `BRIEFS/RV78_RV81_CONFIRMATION.md`. ROOT is the return path. RV81 did not write any of the repairs and did not delegate. All evidence below comes from RV81's own probes, mutants and oracle, rerun on the new head. The authors' tests were used only as the kill criterion for the mutants.

- **Candidate:** READER `b36739112a48da40cd22a7a3bdf7691bf2ad4404`, reviewed from a `git archive` of that commit in `WT/rv81/`. NUM is at `c57496275b`.
- **Files under review:**
  - `P/apps/desktop/src/features/results/retainedPrecision.ts` `b92f29250880393cda55a1bc49de81abcded7da825906ff6716543c06da5ef0e`;
  - its test file `4a883f07714fe761358afdaeaff682ba998f036dc3e93c9affe507f082aa2751`.
  
  Both equal I64's RETURN_D18. The corpus is 07a `a6fa398731…` (15 cases, 236 mutations, 19 must-pass entries); the schema is `07951edacf…`.
- **Diff reviewed:** `6b607fd01f..b36739112a` for both TypeScript files: reader +139/−36 lines, test +251/−7 lines. It spans phase 1, the follow-up, phase 2 and D18.
- **Disclosed host steps:** `WT/rv81/P/node_modules` was linked to READER's `node_modules` target. READER's prebuilt `public/wasm-engine` and `public/self-weight-engine` were copied into the archive, not built; their hashes are unchanged from `reader_review_01`. Both steps are as D15 allows.
- **Window:** 2026-10-04 00:26Z to about 00:40Z, inside the 90-minute box. The memory guard (PID 5387) was running throughout.
- **Limits held:** no Git writes or index operations; no install, build, Cargo, Python, native, solver or DEC-025 job; nothing written in READER.

Line numbers are those of the new `retainedPrecision.ts` unless prefixed. PY means `P/core/analysis_runs/retained_precision.py` at the same head; it was read only, not executed.

## Verdict: PASS

| Severity | Count |
|---|---|
| BLOCKING | 0 |
| SHOULD-FIX | 1 |
| NOTE | 5 |

Every original finding is fixed, or superseded by a decision that the reader implements as written. **D18 is implemented correctly, and removing the G5b member-property duplicate is right.** The one new SHOULD-FIX is a first-gate difference on a single defect that no shared entry exercises: a dangling `product_attempts[i].source_ref`. Under ROOT's rule it is repaired before acceptance, but it cannot make a receipt eligible.

**Baseline on the new head:** vitest 379/379 and tsc exit 0, before and after the review work. The arithmetic oracle (the same 12,502 numeric vectors and 12,960 feasibility cases as `reader_review_01`) found 0 mismatches.

## Original findings and their disposition

| Finding | Disposition | Decision | RV81 evidence on the new head |
|---|---|---|---|
| B1: member coverage was not checked at G3 for unsourced attempts | **Fixed** | D1 | Probe P1 (member 5 on P′ attempt 1) gives G3 COVERAGE with or without the invocation (was G8, or a pass). Probe P16 (an empty unsourced complete list beside a one-member CaseSource) gives G3. `:233`, `:241`. Mutant R01 (the reviewed code restored) is killed by 3 tests. |
| B2: an unbound `prepared_product_failure` cause was accepted | **Fixed** | D4c | Probe P12, with both the dangling and the foreign-Ready variants, gives G5 PRODUCT_ATTEMPT (was a pass). `:620`. R02 is killed. |
| S1: the `captured_prefix` association was checked at G3 | **Fixed** | D1 | Probe P11 (run_ref set) gives G5 PRODUCT_ATTEMPT (was G3). G3 keeps only the member part (`:236`); the association moved to `:631`. R03 survives as a redundant check; see N3. |
| S2: G0 scope | **Fixed** | D2, settled reading 3 | Probes:<br>• P8 (component_version) and P9 (schema_version) give G0 (were G7);<br>• P4 (missing `work`) gives G0;<br>• P3 (non-array `product_attempts`) gives G1 (was G0).<br>Table and inherited-table bytes are hashed (`:181`), and an absent body fails G0 (`:185`). R04 and R13 are killed; R06 survives, unreachable (N3). |
| N1/1: escalating verification failure, solve or pass | Fixed | D5b | `:310`, `:347`. RV78's probe T1 is pinned. **M02 survives**: the v-build half of the discriminator is not pinned (N1 below). |
| N1/2: stop-rule reason locator | Fixed (pinned) | D5d | M03 is killed. |
| N1/3: candidate record shape | Fixed (pinned) | D5a | Probe P5 gives G5 ATTEMPT. M04 is killed. |
| N1/4a: every listed diagnostic must name its case | **Superseded** | D6a | Dropped at `:1202`, which now requires only uniqueness and resolution. Probes: P2 (a ref naming the other case) and P2b (a model-level ref) pass; P2c (dangling) gives G5 ATTEMPT. M05, re-anchored to the resolve rule, is killed. Typed references stay strict. |
| N1/4b: W2 trigger and nonzero exponent | Fixed (pinned) | D6c | R15 is killed. |
| N1/4c: `solve_quality` of a selected case | Fixed (pinned) | D6b | Probe P7 (`checks_passed`) gives G5 ATTEMPT. M06 is killed. |
| N1/4e: order of ordinary and product checks | Fixed (pinned) | D3, D17 | M14 is killed. |
| N2: legacy `work_ref` | **Fixed** | D6d | Probe P15 (dangling `work_ref`) gives G5 ATTEMPT (was WORK). `:1214`. R08 is killed. |
| N3: checklist N11 | Fixed in the checklist; TypeScript was already right | D12 | — |
| N4: unpinned rules | **Fixed** | D13 | M07, M08, M13 and M16 are now killed. M12 is killed too. |
| N5: test-only exports | Fixed | D14 | Each export is marked `@internal`. `nativeRuns` is newly exported, also `@internal`; it throws or returns void and touches no state. Probe P6: `accountingRules` returns four booleans, does not mutate its input, and returns false for R3 on a prototype key (`constructor`). M01 (the eligibility flag) is killed by 23 tests. |
| N6: WASM assets | Not a defect | D15 | The asset hashes are unchanged. |
| N7: ATTEMPT/WORK order inside the native class | Fixed | D3 | Native WORK is deferred to the end of class 1 (`:366–372`). R07 (WORK raised immediately) is killed by 2 tests. |
| N8: arithmetic | Still sound | — | 0 mismatches. |

## New findings

| ID | Severity | Where | Evidence | Remedy |
|---|---|---|---|---|
| S1 | SHOULD-FIX | `:237` and `:248` (sourced D1 comparison and G3 roster check) | **A dangling `product_attempts[i].source_ref` reports G3 COVERAGE in TypeScript.** Probe P17 (source_ref = 9, rehashed) on `ordinary_prepared_synthetic` and on `two_case_facade_after_certificate_synthetic` attempt 1 both give G3 COVERAGE. `:237` fails because `b.sources[9]` is undefined; `:248` would throw a TypeError, which the outer catch maps to the G3 default.<br>Python skips both G3 comparisons when the reference does not resolve (PY:1592, PY:1603, commented "A null/invalid source reference is left to the G5 association pass"). It then fails at G5 PRODUCT_ATTEMPT (PY:913 `_at`, or the run check before it). This is from reading the source; Python was not run.<br>No shared entry covers it. | **Reading:** the reference is association, since C3:146–148 ("attempt.source_ref = that CaseSource.index") and C3:165 cover it. I57 §4's G3 row compares only "the source inventory already associated". D16 gives a dangling C3 reference the code of its association check. So G5 PRODUCT_ATTEMPT is right.<br>**TypeScript** should skip `:237` and `:248` when `source_ref` does not resolve, as Python does.<br>**Shared pin:** add one shared mutation (probe P17's edit), and have RV78 check Rust. |
| N1 | NOTE | `:310` (and `:347`) | **Mutant M02 survives.** Dropping `verification_shared_build_ref === null` from the D5b solve-failure test is not caught. RV78's T1 sets `verification_lme = 1` with the reference null, so the v-build half is never exercised alone. The rule itself follows native code (adaptive.rs:4282–4291 always records the v-build request once the pass is entered). | Add a reader-local pin: T1 with a resolved v-build reference and `verification_lme = 0`. |
| N2 | NOTE | test file, the "D8 kernel scope" test | **The kernel-scope test does not isolate its rule, and R11 survives.** The test edits `findIndex(x => x.state === 'failure')`, a state name that does not exist, so it always edits build 0, a success build. The global state/reason ATTEMPT check then fires as well. The group variant is caught by the refused-group run checks: the base run still has records and attempts. I found no input where the kernel-scope check (`:380`) alone decides the outcome: a kernel `work_accounting` also fails the terminal check or an exact-translation check, with the same ATTEMPT code. | Fix the test's state name (`nonbudget_failure`), or record the rule as defence in depth. No reader change is needed. |
| N3 | NOTE | — | **The other survivors, with their reasons:**<br>• **M11** (the G8 member-sequence check weakened): equivalent. D1 at G3 already requires `0..len−1`, and the weakened G8 check still compares lengths.<br>• **R03** (the `captured_prefix` association line at `:631` removed): redundant. Probe P11 still gives G5 PRODUCT_ATTEMPT under R03 (`probe_results_under_R03.json`), through D4e `(run_ref === null) === !c.run`. A non-null source is caught by the sourced-attempt checks, and a Ready result by the Ready checks.<br>• **R06** (bundled table-byte hashes removed): unreachable from a receipt, as ROOT accepted. | — |
| N4 | NOTE | `:953` (G5b), with TS `:1128` (G8) | **The G5b member-property duplicate's removal is right on the contract.**<br>• C3:155–158 lists "new native A/Iy/Iz/J=[A,I,I,J]" among the old/new operand equalities, and C3:308 assigns those to G8.<br>• All three readers now check it only at G8 (TS `:1128`, PY:1422).<br>• Probe P13 (`A_K` or `Iz_K` changed, rehashed) gives G8 PREPARATION with the invocation.<br>• **Without the invocation, the receipt passes as `needs_recompute`.** Before phase 2, TypeScript rejected it at G5b. This is the same class as D1's accepted no-invocation limit: the receipt can never become eligible, because eligibility needs the invocation.<br>D18 itself is right: probe P14 gives G5b SECTION for a zero in each of the five echoed terms and for a positive mismatch, and R05 is killed. | ROOT records the no-invocation limit for the section/member identity alongside D1's. Or, if C1:160's "malformed evidence yields unsupported" should hold without an invocation, all three readers move the receipt-internal half of the identity to an earlier gate together. Recommend the first, unless ROOT wants the second. |
| N5 | NOTE | — | **No weakened check outside D18's ruled removal, and no decision is implemented differently from its text.** I read the reader diff in full. In the test diff, the only removed test is the old R1–R3 isolation test, replaced by the R1′–R4 version, and nothing is skipped. D16's explicit references are present (`at` lookups and `buildOf`). The catch-all codes only fail closed. | — |

## The repair diff, item by item

**G0 (D2):**
- `header` (`:174–190`) checks:
  - the producer identity, component and schema version;
  - the table fields (FORMATION_MISMATCH);
  - the definition hash;
  - the raw-byte hashes of the table and the inherited table;
  - that the body is an object (settled reading 3);
  - the six fixed values;
  - the thresholds on an object `work`;
  - `definition_id`, only on object attempts inside an array.
- It never walks a malformed body; it reads only the fixed fields above. This matches D2 and its corrections: `receipt_version` is exactly 1, and an absent or non-object field fails G0.

**G3 (D1):**
- member ids are exactly `0..len−1` (`:233`);
- `captured_prefix` checks only the member part (`:236`);
- a sourced attempt's ids are compared with the map (`:237`);
- an unsourced complete list is compared with each CaseSource's count (`:241`), with no non-empty clause, per the checkpoint-A correction;
- `a.ordinary_attempt_ref === i` moved to G5, where it is resolved explicitly (D16; the `dangling_ordinary_attempt_ref` pin).
- The dangling `source_ref` is the S1 above.

**G5:**
- **Class 1** (`nativeRuns` and `nativeClass`):
  - the D8 kernel scope;
  - explicit `at` and `buildOf` resolution;
  - deferred WORK, including `checked`, which keeps going without trusting dependent values (settled reading 4);
  - D5c (`:331–332`).
- **Class 2:**
  - the ordinary pass (D6a, D6b, D6c, D6d);
  - the D4c case pass (`:620`);
  - then per-attempt association: an explicit ordinary reference, and the `captured_prefix` association.
- **Class 4:** R1′–R4 (`accountingRules`), with D8's owner scopes for R3′ in `faultOwner`.

**G5b (D18):**
- Each of the five echoed terms must equal the source's term and be positive, else SECTION (`:953`).
- This runs after the body-scale check and before any stress-scale division. So a zero area or modulus reaches SECTION rather than an arithmetic fault, and P14 confirms this for all five terms.
- Only the member-property duplicate was removed (N4).

## Mutants: 31 run, 26 killed, 5 survived

Each mutant was applied to a clean copy and run with `npm test -- src/features/results/retainedPrecision.test.ts --maxWorkers=2`. Each was then restored and checked by sha256. Evidence: `mutant_results.json` and `mutants2.py`.

| Group | Killed | Survived (reason) |
|---|---|---|
| The 16 review mutants. M05 was re-anchored to D6a's resolve rule; M08 to R3′. | 14 | **M02** (gap: N1); **M11** (equivalent: N3) |
| 15 repair mutants, R01–R15, one per repaired rule (D1, D2, D3, D4c, D5c, D6c, D6d, D8 R2′/R4/kernel scope, D18, settled reading 3) | 12 | **R03** (redundant: N3); **R06** (unreachable: N3); **R11** (redundant, with a non-isolating test: N2) |

## Challenges to decisions

None, beyond these two:
- **S1:** a reading for ROOT to confirm, which affects all readers' first gate on a dangling attempt source reference.
- **N4:** a known limit to record.

## Runs and evidence

| Run | Command (cwd `WT/rv81/P/apps/desktop`) | Result |
|---|---|---|
| vitest_01 | `npm test -- src/features/results/retainedPrecision.test.ts --maxWorkers=2` | 379 passed |
| tsc_02 | `../../node_modules/.bin/tsc --noEmit --pretty false -p tsconfig.json` | exit 0 |
| oracle and probes (03, 04) | `RV81_SCRATCH=WT/scratch/rv81_confirm npx vitest run src/features/results/rv81_oracle_only.test.ts src/features/results/rv81_confirm_probes.test.ts --maxWorkers=2`. These are review-only files, removed afterwards. The oracle file is the first `describe` of `reader_review_01`'s `rv81_oracle.test.ts.txt`. | 0 oracle mismatches; 32 probe outcomes |
| probes under R03 (05) | the same probe file, with only R03 applied; the file was restored from `git show b36739112a` with a sha256 check | P11 still gives G5 PRODUCT_ATTEMPT |
| mutants | `WT=WT python3 mutants2.py` | 31 run, 26 killed, copy restored |
| vitest_06 / tsc_07 (final, restored copy) | same as 01/02 | 379 passed; exit 0 |

**Environment:** Node v24.18.0, npm 11.16.0, vitest 4.1.10.

**In this folder:**
- `REVIEW.md`;
- `rv81_confirm_probes.test.ts.txt`, the probe source;
- `probe_results.json`;
- `probe_results_under_R03.json`;
- `oracle_results.json`;
- `mutants2.py`;
- `mutant_results.json`;
- `SHA256SUMS`.

**Bulk, in `WT/scratch/rv81_confirm/`:**
- `reader.diff` `9ee428c94c…` and `test.diff` `b46cb9ffc2…`;
- the run and mutant logs;
- `oracle_vectors.json` `7bd94f31de…`, the same deterministic vectors as `reader_review_01`.

**Not done:**
- Python and Rust were not executed. The Python outcome in S1 is from reading its source, and RV78's parity run can confirm it with probe P17's edit.
