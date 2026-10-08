# RV113 (RV-R), round 1: independent review of B1's SR-TS slice (the TypeScript reader)

TASK (Type 2), RV113, an independent reviewer dispatched by ROOT (HELP_HUMAN, Agent 0). ROOT is the return path; I made no delegation. I wrote none of the change. 2026-10-07 UTC.

**Basis.** The coordinator's message (review SR-TS as SR-RS, adapted to TS; weigh RR "I92's SR-TS verified; …" ruling 2), my brief `R/BRIEFS/RV113_RVR_ROUND1.md` (`a753ea5b…cf9cd4`) and its host rules as restated in RR "RV113 passes SR-RS with S-1; …".
- I read I92's brief, `R/BRIEFS/B1_SR_TS.md` (`9698cd33…edbfe`), and the specification as for SR-RS: PLAN_v2 §2.4, and DESIGN_v2 §2, §3.2 and §3.3.
- I also read C2 (`R/I32/f2a_wire_c2/CONTRACT_DELTA.md`) §2 for the cause branches, and I83's RETURN for the Node and wasm set-up.
- I read I92's RETURN (`R/I92/b1_sr_ts_01/RETURN.md`, `cb50e471…0040`, verified) only after writing my own TS D38 audit (`evidence/d38/`).

**Placeholders:** WT, NUM, P, T, R and RR as in the dispatch, plus:
- DT = `P/apps/desktop/src`; TS = `DT/features/results/retainedPrecision.ts`; TT = its test file;
- RS = `P/core/reporting/result_export/src/retained_precision.rs`; PY = `P/core/analysis_runs/retained_precision.py`;
- NMS = the shared `node_modules` that `WT/t6-outputs/P/node_modules` links to.

**The candidate.** `codex/piping-t3-b1-t-20261007` at `7e47e51b5d935fda7a8289d14b21f8979e4f4876`: four commits over I1 `262bd687f0`, 2 files, +262/−9.
- **What I reviewed:** my own `git archive` copies of I1, of the head, and of the head for mutants (`WT/rv113/ts-{i1,head,mut}`, P without `execution/`). `diff -rq` between the I1 and head copies lists exactly TS and TT.
- **File hashes, I1 → head:** TS `7f9b47a9…` → `695f95d5…`; TT `9d6071c3…` → `4756deac…` (as I92 states).
- **The set-up:** in each copy, NMS was linked at `P/node_modules` and the eight wasm assets copied, as I83 did. The assets' sha256 values are I71's eight (`evidence/host/wasm_assets.sha256`).
- **RS for comparison:** an archive of SR-RS's reviewed head `cc81e78801`.
- **Corpus:** 07m (`c21112fd…6807`).

## Verdict: PASS

**Counts: 0 BLOCKING, 1 SHOULD-FIX, 2 NOTE.**

TS implements DESIGN §2 and §3.2–§3.3 as RS does:
- **The census.** My census over 07m finds **0 changes** in 339 entries and three verdicts each, and 0 misses against the corpus's TypeScript expectations. R5 does not fire.
- **Agreement with RS.** On my 103 probes, which carry the 82 I built for SR-RS, **TS at the head and RS at `cc81e78801` agree on every B1 shape**: (4b), m1–m8, (4a), G8 P1–P4 per case, the `not_required` rule including RV113 S-1's own-attempt shape, the N-2 shapes, the disclosed limit, and RV108 N4. They differ only on C2's cause branches and on the two header classes that TS's doc declares.
- **D38.** I92's 24-item D38 list is complete.
- **Mutants.** Of my 26 mutants, 22 die at the candidate's assertions; the 4 survivors are equivalent.
- **Suites.** vitest goes from 3,620 to 3,626 with +6 added tests and nothing else changed; `tsc` is clean on both sides.

The SHOULD-FIX is the cross-reader question of RR ruling 2. It is not a TS defect; it is a gap in RS and PY, which I recommend closing by alignment.

## Findings

| ID | Severity | Where | Finding | Remedy |
|---|---|---|---|---|
| S-1 | SHOULD-FIX | RS `g5_ordinary`; PY `_g5_ordinary` (no C2 branch rules). TS `ordinaryAttempts` has them | **C2's code/phase/cause compatibility is enforced in TS only, so RS and PY admit receipts that TS refuses.** C2 §2 (CONTRACT_DELTA:72–74) says each unavailable cause has its own phase and codes, "one-to-one by the table above; a source error cannot claim a facade numeric predicate". TS checks this for every unavailable case whose cause is not `prepared_product_failure`, at G5 `ATTEMPT_MISMATCH`, before the product class; RS and PY do not. My probes (two-case base, case 1 unavailable with its Ready attempt and selected Run, D19's `receipt_failure` form) show more than a first-failure difference:<br>• the branch satisfied (`receipt_encoding`/`receipt`): both admit;<br>• `receipt_failure` with phase `kernel` and code `kernel_unresolved`, with code `facade_certificate`, or with phase `preparation`: **RS admits; TS refuses (G5 `ATTEMPT`)**;<br>• `facade_failure` with phase `kernel`: RS G5 `PRODUCT_ATTEMPT` (D19); TS G5 `ATTEMPT`;<br>• I92's (4b) variant (`receipt_failure` cause, phase `preparation`): RS `PRODUCT_ATTEMPT`, TS `ATTEMPT`.<br>PY has no such branch either (it names none of `receipt_encoding`, `publication_hash_range` or `caller_not_qualified`; its only cause test is D19's), so by reading it admits as RS does; SR-PY's review checks this by run. The producer emits none of these causes in B1 (T-11 leaves `receipt_failure` reader-legal but unemitted), so aligning changes no producer receipt. | **Align, do not declare.** A declared difference here would mean the readers disagree on what is reader-legal, not only on order. Steps:<br>• ROOT rules the exact table (TS's present one, or the tighter one-to-one form of N-2);<br>• RS and PY add it at G5 `ATTEMPT_MISMATCH` in the ordinary class, where TS has it;<br>• 07n pins each branch, satisfied and broken.<br>RS's addition rides in B1's one RS re-qualification (SQ's G5) if it lands before I5; otherwise it waits for the next RS change. Until then, SC builds any 07n entry of this form in its branch-satisfying shape (ruling 2). |
| N-1 | NOTE | TS `validateRetainedPrecisionTransport`; RS `validate_transport_metadata` | **The two transport readings differ on header defects. This is pre-existing and not B1's.** RS's transport reads the base header (`semantic_contract::for_source_metadata`) and refuses at G2; TS's transport reads only `formulation_basis` and `contract_evidence`.<br>• On 07m, six G7 mutations' sources (invalid `numerical_quality` enums, case members or status) are refused on transport by RS (G2 `SOURCE_NUMERICAL_*`) and **admitted** by TS. Three more are refused by both, with different gate and code (RS G2 base code; TS G7 `SOURCE_PREVIEW_PHYSICS_EVIDENCE_INVALID`).<br>• My eight N6 enum probes show the same split.<br>No corpus entry pins transport verdicts. TS at I1 behaves the same. | Fold into the alignment set (SR-PY's review adds PY's transport): ROOT rules what the transport reading checks and its gate, then pins it. |
| N-2 | NOTE | TS `ordinaryAttempts`, the C2 branches | **TS's branch table is coarser than C2's "one-to-one".** A `receipt_failure` code must be one of three, but is not keyed to the cause's `check`. An `unavailable_precondition` code must be one of four, but is not keyed to its `precondition`. The kernel branch compares the whole reason with the Run's terminal. | Settle the exact table once, under S-1, and apply it in all three readers. |

## 1. The census over 07m (R5)

**My harness** (`evidence/harness/rv113Census.test.ts`) is copied into my archives only. It is written from the snapshot format rules (SHARED_SNAPSHOT_06C, 07E), not from TT's helpers:
- it uses the strict index rule, the invocation digest rebind, the 07E rehash order, then `after_rehash`;
- hashes come from the TS hash service, `canonicalSha256HexCheckedV1`;
- for each entry it records the input's sha256 and three verdicts: bound, unbound and transport. Each is the gate and code, or the eligibility with the standing, the publication hash and a digest of the classifications.

**Runs:** inside the whole vitest suite at I1 and at the head, each under the lock (`vitest_i1`, `vitest_head`).

**Result: 0 changes in 339 entries** (17 bases, 294 mutations, 28 must-pass):
- the inputs are byte-identical;
- the bound, unbound and transport verdicts are identical entry by entry;
- both sides match the corpus's TypeScript expectations: all 294 first failures (`expected_by_reader.typescript`, else `expected`), all 28 must-pass eligibilities and standings, and all 17 bases.

**TS against RS on the same 07m entries** (my SR-RS census): one bound difference, `g7_maximum_off_enclosure`, which is the corpus's declared per-reader G7 code (`expected_by_reader`). For transport, see N-1.

Evidence: `evidence/census/TS_CENSUS_07M.json`, `census_ts_{i1,head}.jsonl`. **R5 does not fire.**

## 2. R-D38 (4b) in `productAttempts`, and the D38 audit

**The change.** The I1 check `(native == not_entered) == (run_ref == null)` becomes: not entered ⇒ no Run; otherwise a Run, **or** native `failed` and `d38CaptureBeforeRun(a, c, s)`. Rule 1 (`run_ref` null ⇔ the case's Run null) is unchanged. The predicate carries RS's conjuncts plus "a source that binds this attempt".

**My probes in TS** (`evidence/probes/TS_PROBE_TABLE.json`, with RS's verdicts beside them):
- my two-group (4b) derivation and the F_BASE one are admitted, not eligible, standing `needs_recompute`;
- m1–m8 (with the five m7 forms) and my 24 other shapes give exactly RS's first failures. That includes m4 at G3, m7 at G5 `ATTEMPT`, the leftover Builds and the stale meter at G5 WORK, and `recovery_method` kept at G6;
- my (4a) witnesses (an idle `ledger_unavailable` Run, native- or capture-failed) are admitted, as in RS;
- the only (4b)-family difference is I92's own `receipt_failure` variant (S-1).

**Which conjuncts are load-bearing.** The branch's `native === 'failed'` guard is load-bearing in TS, unlike RS: no other TS check refuses a completed native stage with no Run whose capture came before `proof_start`. My T01 (guard dropped) is killed by m2. The predicate itself is wholly redundant in TS: my T04 (predicate → `true`) survives every test and all 103 probes, and so do T05–T07. ROOT's ruling 3 keeps it for the three-reader shape; I agree.

**D38's audit.** My list (written first) and I92's 24 agree, item for item:

| Mine | I92 # |
|---|---|
| T1 the relaxed check | 1 |
| T2 rule 1 | 2 |
| T3 | 3 |
| T4 the source association (already binds the case's source, so TS needs no extension) | 4 |
| T5–T7 the proof, coverage and Ready rules | 6–8 |
| T8 the reason table | 9 |
| D37/P9 | 10 |
| T9 `nativeClass` | 11 |
| T10 G3's Runs | 12 |
| T11 `unselectedCoverage` | 16 |
| T12 the selected-only checks | 17 |
| T13 C2's branches | 19 |
| the no-Run items | 5, 13–15, 18, 20–24 |

**I92's list is complete.** My one addition is an observation, not an omission: T4 makes (4b)'s equality redundant in TS, which is why TS needed no "extended" item where RS had one.

## 3. G8 per case

In `invocationBinding`'s loop, every case runs in request order and is checked in this order:
1. the requested mode;
2. the material basis, before P1, as DESIGN orders it;
3. P1 (exactly one mode row, valued 1 in sparse and 2 in dense; **mode code 3 dropped**);
4. P2, P3 and P4, each its own `fail`.

All are G8 `PREPARATION_MISMATCH`; the requested mode and P1 were `INVOCATION_MISMATCH` at I1. My probes, I1 → head:
- **The codes.** The mode-row and requested-mode defects on unavailable and selected cases move from `INVOCATION_MISMATCH` to `PREPARATION_MISMATCH`.
- **Mode code 3** on an unavailable case: admitted at I1, now refused.
- **P2–P4 per case.**
  - Two parity rows: from admitted to refused.
  - A parity row beside a published W2 (on case 1, on case 0, on an unavailable case, on a `not_required` case): from admitted to refused.
  - A sparse parity row on an unavailable or (4b) case: from admitted to refused.
- **Dense b = 0 without a parity row** was already admitted at I1 (TS had no parity rule) and still is. One parity row on a dense b = 0 unavailable case is admitted.
- **P4 is exactly `published`.** A parity row beside a failed W2 is admitted. TT pins this, I92's commit `7e47e51b5d`.
- **The disclosed limit.** The deletion resealed with its row indices is admitted. P5 is not implemented.
- **Agreement with RS.** Every G8 probe gives RS's verdict at the head.

## 4. G5's `not_required` rule

TS's rule (`product_attempt_ref` null, `initial != not_attempted`, verdict `checks_passed`) is unchanged and is DESIGN's.
- **RS's conjuncts never existed in TS.** RS's three dropped conjuncts, added to TS as mutants (T33–T35), are each killed by TT.
- **RV113 S-1's shape is pinned in TS.** A `not_required` case naming its own attempt is refused at G5 `ATTEMPT` (my probe, and TT's added row). T30 (the conjunct dropped) is killed.
- **N-2's two non-emittable shapes** are admitted in TS as in RS, so parity holds as ruled.
- **Every `not_required` probe** gives RS's verdict at the head.

## 5. RV108 N4 and N6

**N4.** Only object rows lose `recovery_method` in `projection`. My five forms on `ordinary_prepared_synthetic` were `results[0]` = null, a number, a string or an array, and a null appended:
- the full reader refuses each at G1 `RECEIPT_MISMATCH` (as RS does);
- transport admits each (as RS's transport does);
- at I1, TS's transport refused the two null forms at G7 `SOURCE_PREVIEW_PHYSICS_EVIDENCE_INVALID` (RV108's finding);
- T40 (the guard removed) is killed by TT's N4 test.

**N6.** I checked `baseHeaderCode`'s doc claims about Rust by running RS on the same inputs:
- list- and dict-valued `structural_status`, `model_matrix_fidelity` and `accuracy_evidence` give G7 `SOURCE_NUMERICAL_CASE_INVALID` in both readers; `numerical_quality.status` gives `SOURCE_NUMERICAL_QUALITY_INVALID` in both;
- `carrier_evidence` with a case defect gives RS `SOURCE_NUMERICAL_CASE_INVALID` and TS `SOURCE_PRODUCER_CONTRACT_UNSUPPORTED`, the declared inherited class;
- a null `contract_evidence` with `source_block_recovery` gives RS `SOURCE_BLOCKS_LEGACY_DOWNGRADE_FORBIDDEN` (the latter defect, as the doc says) and TS `SOURCE_PREVIEW_PHYSICS_EVIDENCE_REQUIRED`.

The doc is accurate for Rust. Its Python clause, that the codes match after SR-PY's guard, I check in SR-PY's review.

## 6. vitest and tsc, test by test

Each ran under the lock in my copies (`evidence/suites/`).

| Suite | I1 | Head | Test by test |
|---|---|---|---|
| vitest, the whole desktop suite | 142 files, 3,622 passed | 142 files, 3,628 passed | **+6 added (passed):** TT's six B1 SR-TS tests. 0 removed, 0 changed. (Both counts include my two harness tests; without them, 3,620 → 3,626, as I92 reports) |
| `tsc --noEmit -p tsconfig.json` | rc 0, no output | rc 0, no output | — (run on the pristine copies, before my harness was added) |

## 7. Mutants

**The method.** I used a mutant schema in my third copy (`evidence/mutants/mutant_schema_TS.diff`). Each edit is guarded by `mx('<id>')`, which reads `RV113_MUT`. Each mutant was one vitest job under the lock, over the seven test files that import TS plus my probe harness.
- The control (`RV113_MUT` unset) passes all 806 tests and gives the head's probe verdicts exactly.
- Every mutant collected all 806 tests; none failed to load.

| ID | Mutant | TT and the other six files | My probes changed | Disposition |
|---|---|---|---|---|
| T01 | (4b) branch without `native === 'failed'` | killed: the (4b) test (m2) | m2 | killed |
| T02 | every failed native stage must be (4b) ((4a) refused) | killed: D30 (`productAttempts` on a native error with its own Run) | both `a4_*` | killed |
| T03 | the relaxed check restored | killed: the (4b) test | 7 | killed |
| T04 | (4b)'s predicate → `true` | survives | none | **equivalent** (the predicate is wholly redundant in TS) |
| T05 | the predicate → the source equality alone | survives | none | equivalent |
| T06 | the predicate's source equality dropped | survives | none | equivalent (T4, the source association) |
| T07 | the predicate's "source binds the attempt" dropped | survives | none | equivalent (T4) |
| T10 | P4 reads case 0's ordinary attempt | killed: the P2–P4 test | 3 | killed |
| T11 | P4 stricter (parity only beside W2 `not_triggered`) | killed: the P2–P4 test (W2 failed) | 1 | killed |
| T12 | P4 dropped | killed | 4 | killed |
| T13 | P2 widened to two | killed | 2 | killed |
| T14 | P3 dropped | killed | 2 | killed |
| T15 | **mode code 3 restored** | killed: the P1 test | 2 | killed |
| T16 | the requested mode's code back to `INVOCATION_MISMATCH` | killed | 1 | killed |
| T17 | P1's code back to `INVOCATION_MISMATCH` | killed | 7 | killed |
| T18 | P1–P4 for case 0 only (**the per-case loop narrowed**) | killed: P1 and P2–P4 tests | 12 | killed |
| T19 | P1–P4 for selected cases only | killed | 10 | killed |
| T21 | P1–P4 read case 0's rows | killed | 11 | killed |
| T30 | `not_required`: `product_attempt_ref` null dropped | killed: the own-attempt row | 1 | killed |
| T31 | `not_required`: verdict `checks_passed` dropped | killed | 1 | killed |
| T32 | `not_required`: `initial != not_attempted` dropped | killed | 1 | killed |
| T33–T35 | RS's dropped conjuncts added (`report`; outcome `checks_passed`; W2 `not_triggered`) | killed: the `not_required` and P2–P4 tests | 5–6 each | killed |
| T36 | the report-outcome equality dropped | killed | 1 | killed |
| T40 | **N4's rule removed** | killed: the N4 test | 2 | killed |

**Totals: 26 mutants.** 22 are killed by the candidate's assertions, and 4 are equivalent ((4b) predicate forms). Every mutant that I92's brief names is killed: the per-case loop, P2–P4, mode code 3 restored, the relaxed check restored, RS's conjuncts, and N4. This agrees with I92's run 2 (21 killed and 12 equivalent, on single-conjunct forms).

## 8. RR ruling 2: align, or declare?

**Recommendation: align (S-1).**
- **It is an admission difference, not only an order difference.** My `c2_receipt_*` probes are admitted by RS and refused by TS. Three readers that disagree on what is reader-legal break C1 §6's parity aim. Declaring the difference would freeze that.
- **The rule is the contract's own.** C2:72–74 states the code/phase/cause compatibility. RS and PY already apply its `prepared_product_failure` row (D4d's reason table), so the gap is the other rows.
- **Aligning is cheap and safe for the producer.** It is a short table in the ordinary class, and the B1 producer emits none of these causes. In RS it fits inside B1's single re-qualification if it lands before I5.
- **The table needs one ruling first** (N-2): TS's set-membership form, or C2's one-to-one form keyed to `check` and `precondition`.

I fold this, N-1 (transport) and the SR-PY disagreements into one alignment set in my SR-PY review, so that ROOT can rule them together.

## Host and limits

- **Under the lock** (`/usr/bin/lockf -k WT/guard/cargo_job.lock`), each its own hold:
  - `tsc` ×2 and vitest ×2 (whole suite, with the census and probes);
  - the probe-only vitest runs ×2;
  - 27 mutant vitest runs (the control and 26 mutants), one hold each;
  - two RS probe runs, as cargo jobs through `WT/tools/t3_cargo.sh` (`--locked --offline`) in a fresh archive of `cc81e78801` with target `WT/targets/rv113-rs`.

  The holds are listed in `evidence/host/`. I ran no test binary or vitest outside the lock.
- **Waits.** One wait per job (its background task), each ending when the job's process ended. None of my waits remain. I killed no job.
- **Node and the wasm assets.** The links and the copied assets lived only in my copies. I removed the links and deleted the copies (`WT/rv113/ts-{i1,head,mut}`), the RS copy and its target. NMS's `.vite-temp/` is empty; only its mtime moved. Scratch (`WT/scratch/rv113_rvr_01/ts/` and `tools/`) is kept, and `TMPDIR` was in scratch.
- **Not done, and not allowed:** no DEC-025, installs or Git writes (`git archive` and reads only, with `GIT_OPTIONAL_LOCKS=0`). No record folder is named `build`. Every write used an absolute path, and this folder carries placeholder paths only.
- **Not run:** PY (SR-PY's review), and the full 40-manifest and src-tauri suites (ROOT's).

## For ROOT

1. **R5 does not fire;** SR-TS passes.
2. **S-1, ruling 2: align C2's cause branches in RS and PY** after ruling the exact table (N-2), and pin each branch in 07n. Until then, 07n uses the branch-satisfying form. This is folded into my SR-PY review's alignment set.
3. **N-1:** rule the transport reading's header scope (RS reads it at G2; TS does not), together with SR-PY's transport, in the same set.
4. **The (4b) predicate stays in TS** (ruling 3). It is wholly redundant there, and only the `native === 'failed'` guard is load-bearing; m2 pins it.
