# RV81: independent review of the TypeScript retained-precision reader

RV81 is a TASK (Type 2) reviewer dispatched by ROOT (HELP_HUMAN, Agent 0) under `BRIEFS/RV78_RV81_READER_REVIEW.md`. ROOT is the return path. RV81 did not write any of this code or corpus, did not delegate, and did not use the authors' tests as oracles.

- **Candidate:** READER `codex/piping-f2a-readers-20261003` at `6b607fd01f9819a3b6526dd9fde02cd3bc4db586`, reviewed from a `git archive` of that commit in `WT/rv81/`.
- **Files under review** (hashes equal I64's RETURN):
  - `P/apps/desktop/src/features/results/retainedPrecision.ts` `7c802881a280385926930a90e624fb60a65069100268bdffcd3a77eb84d18845`;
  - `P/apps/desktop/src/features/results/retainedPrecision.test.ts` `654caced2ead914297b0d26c72ab1a6735924396ca7f17ed18897c2fca408a1f`.
- **Shared inputs:** corpus `d02701ed6a…` (06d), schema `f943ebd351…`, definition `3e0779a45a…`, table `c74742ce6a…`. The Python reader (`55736ea65a…`) was read for comparison only. Native code was read at NUM `6a5b131b98`.
- **Window:** 2026-10-03 22:47Z to about 23:12Z, inside the two-hour box. The memory guard (PID 5387) was running throughout.
- **Limits held:** no Git writes or index operations (Git reads used `GIT_OPTIONAL_LOCKS=0`); no install, build, Cargo, Python, native, solver or DEC-025 job; nothing written in READER. Each mutant was applied to the copy in `WT/rv81/` and restored, with its sha256 checked after every mutant.

Line references are to `retainedPrecision.ts` unless prefixed. Shorthand: `PY` = `P/core/analysis_runs/retained_precision.py`, FK = `P/core/solver/frame_kernel/src/structural/retained`, PP = `P/core/product_physics/src`. C1, C2, C3, F1, S06, I57 and the checklist are the brief's basis documents.

## Verdict: FAIL

| Severity | Count |
|---|---|
| BLOCKING | 2 |
| SHOULD-FIX | 2 |
| NOTE | 8 |

Arithmetic, the coverage rules, fail-closed eligibility and the native schedule logic are sound. On 25,462 independent oracle vectors there were 0 mismatches. The eligibility hold is pinned: the flag mutant is killed by 21 tests.

The verdict is FAIL because rehashed single-defect probes show two malformed-receipt classes that the reader returns as a validation. Under C1:160 these should be refused: "malformed evidence yields unsupported". The reader instead returns standing `needs_recompute`. Neither can make a receipt eligible while the hold stays false. Both are small to repair, and each needs a shared mutation. Two further single-defect cases report the wrong gate.

## Findings

| ID | Sev | Where | Evidence | Remedy |
|---|---|---|---|---|
| B1 | BLOCKING | `:213` (G3 checks only uniqueness and prefix overlap); `:1064–1065` (member sequence checked only at G8) | F1:130: "G3 checks the declared complete/prefix inventory". C3:302: G3 owns member-prefix coverage. **Probe P1:** on base `two_case_preparation_failure_synthetic`, the source-less attempt 1 gets `old[0].member = 5`, then a rehash. With the invocation, TS fails at G8 PREPARATION_MISMATCH. **Without it, TS returns a validation (needs_recompute).** Python fails at G3 COVERAGE_MISMATCH (`PY:1424`). Mutant M11 (the G8 sequence check weakened) survives, so no test pins the rule. | At G3, require the old, preparation and new member ids to be `0..len−1` (as `PY:1424`); keep the source-map equality at `:215`. Add a shared mutation expecting G3 COVERAGE_MISMATCH. |
| B2 | BLOCKING | `:600–601` (the cause is checked only inside the product-attempt loop); `:1114` (the ordinary pass skips this branch) | S06 §1: `prepared_product_failure` "is permitted only when the containing case's product_attempt_ref is the same U … cannot reference a Ready attempt, another owner, an absent attempt". **Probe P12:** remove case 1's own attempt and set `product_attempt_ref = null`, rehashed. A cause naming the removed attempt (dangling), or the other case's Ready attempt 0, is **accepted** (needs_recompute). By source reading, Python has the same gap (`PY:835` sits inside the attempt loop). | For every unavailable case with this cause, require a non-null `c.product_attempt_ref === cause.product_attempt_ref` at G5 PRODUCT_ATTEMPT_MISMATCH. Add a shared mutation. Python, and likely Rust, need the same fix. |
| S1 | SHOULD-FIX | `:214` | The `captured_prefix` check requires `source_ref`, `run_ref` null and an unavailable result **at G3**. F1:97 and 130–131 and checklist P7 put that association at G5; G3 owns only the inventory and overlap. **Probe P11:** the must-pass `prefix_captured` shape with `run_ref = 0`, rehashed, fails TS at G3 COVERAGE_MISMATCH; Python fails at G5 PRODUCT_ATTEMPT_MISMATCH (`PY:771/779`; its G3 at `PY:1425` checks only empty members). | Keep `!pm.length && !fresh.length` at G3 and move the rest into `productAttempts`. Add a shared mutation. |
| S2 | SHOULD-FIX | `:163–173` (`header`); `:1156` | **The readers disagree on what G0 covers, and only one shared mutation (`v1_relabel`) pins G0.** (a) TS checks the 20B/60B thresholds, `receipt_version` and `canonicalization` at G0 (`:169–170`), which C1:143 supports ("all registered policy ids/thresholds"). Python leaves them to the G1 schema constants. (b) Python checks `producer.component_name/version` and `schema_version` at G0 (`PY:1376`); TS reaches them only at G7 through `sourceContract` (probes P8/P9: G7 SOURCE_PRODUCER_CONTRACT_UNSUPPORTED). (c) Python hashes the table bytes and the inherited base table at G0 (`PY:1379–1382`); TS compares fields with a constant `BASE_HASH` (`:166`). (d) TS reports G1 shape defects in the body at G0: a non-array `product_attempts` or a missing `work` (probes P3/P4). | ROOT reads C1:143 for G0 ("known identity/profile/table/inherited hash … thresholds"). The readers align; add one shared mutation per identity element. TS should verify the bundled tables by hash, and should not walk a malformed body at G0. |
| N1 | NOTE | — | I64's five known-difference groups. TypeScript is right on groups 1, 2, 3, 4b, 4c (checks_passed), 4d and 4e. Group 4a needs a ruling. Group 5 has no observable effect. Details are in the "Named question 1" section below. | Rule on 4a and on whether `not_assessed` is excluded. Add a shared mutation for each settled reading (mutants M02–M06 and M14 survive today). |
| N2 | NOTE | `:1111` | **An unlisted TypeScript-only rule:** `legacy_source.work_ref` must resolve into `legacy_source_work` with the same `case_index`. It raises WORK_MISMATCH inside the ordinary pass, before the C3 association class (C3:304). Python has no such check. C2:160–162 defines the array but not the reader rule or its code. | Rule on the rule, its code (ATTEMPT or WORK) and its place; then add a shared pin. |
| N3 | NOTE | `:359` | Checklist N11 says a group-refused run has "group null". Native code gives it `Some(index)` (FK `adaptive.rs:5040–5049`, `RunPhase::GroupPreparation`). TypeScript follows the native code. | Correct the checklist text; check Python and Rust against the native code. |
| N4 | NOTE | — | **Implemented rules no test pins** (mutants that survive): M13, the theta=+0 rule for no-data bodies (I57 §4 item 4); M16, the Ceiling threshold after a p128 verification-solve failure (`:320`); M08, R3 containment for `both`; M07, N17 precedence; M12 is near-equivalent. R3 is checked against the attempt-level join, not the owning trace that the 06d ruling text names. Python's rule is identical. | Add shared mutations for M13, M16 and M08. Confirm the attempt-level R3 reading. |
| N5 | NOTE | `:247, :494, :1088` | **The test-only exports** `nativeSchedule`, `accountingRules` and `ordinaryAttempts` are pure: they throw, or return void or booleans, and set no state. No non-test module imports the reader (a grep of `src/` found none). Probe P6: `accountingRules` does not mutate its input and rejects a prototype key. | Keep them. Optionally mark them `@internal`, or group them as a single test namespace (see "Named question 2"). |
| N6 | NOTE | `loadWasmEngine`, `hashService` | **The source-bound WASM assets match what the reader claims.** The four asset hashes equal I60's MANIFEST runtime entries. The operation_applier dependency closure (`product_physics`, `serialization/canonical_json`, `units`, `model_operations`) has no READER commit after the assets' mtime (2026-10-03T16:34Z), and canonical_json and units last changed on 2026-09-25. The assets were not rebuilt here. A missing or broken engine surfaces as a G0 or G8 gate code: fail-closed, but misattributed. | None for review. Optionally, map infrastructure failures to a distinct non-gate error. |
| N7 | NOTE | `:341, :346, :365, :452–473` | The native pass interleaves ATTEMPT and WORK checks per call and run: for example a call's `invocation_before` (WORK) comes before its owner order (ATTEMPT), and the final WORK totals come before the group ATTEMPT checks. Only dual defects are affected. This is the same open question as Rust's "order of the native checks within G5". | One ruling for all three readers. |
| N8 | NOTE | — | The arithmetic oracle found 0 mismatches. The extent, coupling, e_hat, phi_512 and stress-scale operation orders match the native code. See "Arithmetic" below. | — |

## Review item 1: gate order and first-failure codes

- **The passes run in order** at `:1141–1162`: G0, G1, G2, G3, G4, G5, G5a, G5b, G5c, G6, G7, G8. Every pass loops over all cases before the next pass, so the order is gate-major across cases.
  - G5a covers every selected case (`numericSummaries`), then every unavailable attempt that keeps coverage (`unselectedCoverage`).
  - All G5a failures share SCALE_MISMATCH, so the order inside G5a is not observable.
- **G1 runs before G2:**
  - the schema-shape walk (`:128–142`) mirrors Python's vocabulary: patterns, minimum and maximum are left to G2;
  - the independent `coverageShape` guard (`:175–180`) and the hash checks come next;
  - `encoding()` runs last inside `integrity`, with an explicit G2 code.
- **G5 follows the C3:304 classes:**
  1. native schedule, origin and work (`nativeRuns`);
  2. ordinary references, then C3 association, stages and lanes (`ordinaryAttempts`, then the `productAttempts` loop);
  3. typed check/error pairing for every attempt (`:625–633`);
  4. the deferred C3 WORK list (`:635`), which includes R1–R3.

  Classes 2 and 3 share PRODUCT_ATTEMPT_MISMATCH, so their interleaving is not observable. The exceptions are N2 and N7.
- **G7 (`:1156–1157`)** reports the bare base code, with the detail kept separately (`baseError`, `:21–25`). This matches the G7 ruling and the per-reader corpus expectation.
- **Single-defect gate disagreements found:** B1 (G3 against G8, or none), S1 (G3 against G5), and S2 (G0, G1 or G7).

## Review item 2: the G5 checklist (42 IDs)

**I confirm I64's 06d status table, with these exceptions:**
- **P7** is "checked", but at the wrong gate for the `captured_prefix` association (S1). Member-inventory coverage is enforced only at G8 for source-less attempts (B1).
- **N11** is implemented according to the native code, which contradicts the checklist's wording (N3).
- **N8:** the threshold boundary is untested (M16).
- **R3:** the containment for `both` is untested (M08).

**Spot verification against the code:**
- **N1–N10** at `:252–329` and `:351–361`. N4 and N5 use the record discriminator discussed under group 1 below.
- **N13–N16** at `:254, :296–304, :370–380, :404, :438–440`.
- **N17** at `:444–447`.
- **C1–C6** at `:336–341, :365, :381–400, :450–473`, and G3 `:228`.
- **O1–O5** at G3 `:205/:209`, then `:1095–1123`.
- **P1–P6** and **P8–P11** at `:532–633`.
- **W1–W3** at `:551, :557, :566, :597`. W4 is attested only.

## Review item 3: arithmetic, with an independent oracle

- **The oracle** (`oracle_gen.py`) is Python `Fraction` code. It computes RU(x) by constructing the exponent and significand directly, with no binary search and no code shared with the reader or its tests.
- **What was compared**, against the reader's exports:
  - `upwardProduct`: 3,000 vectors, 298 of them overflow refusals;
  - `upwardSmallSum`: 2,000;
  - `absoluteBound`: 3,000, a third of them forced into the S<2^-988 branch, following C1:158;
  - `phi512`: 3,000, RU(2^-438·ê);
  - `eHat`: 1,502;
  - `stopFeasible`: all 12,960 rows of an independently written I57 §2 generator, with D=false, presence implied by non-input presence, L=0 or not, and every floor;
  - decode/encode round trips, plus refusals of non-finite or malformed bits.
- **Result:** 0 mismatches (`oracle_results.json`).
- **Checked by reading the native code:**
  - body extent `((d0²+d1²)+d2²).sqrt()`, with max−min per axis (FK `adaptive.rs:321–336`), against `:657`;
  - `coupled_scales` (`:341–352` against `:658`);
  - `e_hat` and `phi_512` (FK `verify.rs:323–334, :367–376` against `:660–669`);
  - the stress scale (`adaptive.rs:354–355` against `:866`);
  - the zero test `n.to_bits() != 0` (PP `retained_product.rs:2600–2601` against `:818`).
- **Counters** are safe integers converted to BigInt, with checked sums of at most 2^53−1 (`:125–127`), as C1:62 requires.

## Review item 4: coverage (I57)

- **Feasibility** (`:718–729`) equals the I57 §2 generator exactly (the oracle above).
- **The estimate and charge rederivation** (`:759–762`) is I57 §2 verbatim, including p512 charge = stop[force/moment].
- **Exact rosters** at `:767–769`.
- **Resolution, theta and certified_bound** coverage and positivity at `:770–771`.
- **Record relations** (`:734–743`): one bound entry per body, in order, non-null iff `has_data`; theta=+0 without data; `data_blocks`.
- **Direct data facts** (`:745–748`).
- **The canonical layout** is rebuilt from the source maps, with +0 prescriptions (`:698–715`).
- **G3 roster cardinality and order** (`:221–222`), and the G5 custody/stage table (`:584–590`).
- **Unavailable attempts with a complete roster** (`:779–797`) apply only native-run facts: feasibility, with the p512 floor sign taken from ê of the run's verification record; record relations; and data facts. They apply no Selection rosters, as I57 §4's last paragraph requires.
- **Gap:** the theta=+0 rule has no shared pin (M13).

## Review item 5: fail-closed behaviour and public surface

- **Eligibility** is `SUMMARY_COVERAGE_COMPLETE && invocation && MECHANICS_SOLVED && every case selected or not_required` (`:1159`). The flag is a non-exported `const false` with no setter.
  - Mutant M01 (flag set true) is killed by 21 tests.
  - Every exception becomes a gate error (`:1161`).
- **Every return path runs all gates.** Without an invocation, G8 is skipped and the validation is unbound, as designed. Transport runs only G0–G2 and the base metadata, with no classes (`:1164–1170`, C1:162).
- **Inputs are snapshotted synchronously** before the first await (`:1145`), and results are frozen recursively.
- **No production importer exists.**
- **Two malformed classes still return a validation** instead of an error: B1 (without an invocation) and B2.

## Review item 6: mutation testing

16 single-edit mutants were run. Each was applied to a clean copy, run with the suite (`npm test -- src/features/results/retainedPrecision.test.ts --maxWorkers=2`), and restored, with the restore verified by sha256. Evidence: `mutant_results.json` and `mutants.py`.

| Mutant | Target | Result |
|---|---|---|
| M01 | eligibility flag set true | **killed** (21 failed) |
| M02 | N4 continuation without the v-build discriminator | survived (known difference 1) |
| M03 | stop-rule reason locator removed | survived (known difference 2) |
| M04 | candidate record `verification_lme` clause | survived (known difference 3) |
| M05 | listed diagnostic refs unchecked | survived (known difference 4a) |
| M06 | selected quality admits `checks_passed` | survived (known difference 4c) |
| M07 | N17 budget scope precedence | survived (no shared base) |
| M08 | R3 `every` → `some` | survived |
| M09 | upward rounding increment removed | **killed** (5) |
| M10 | phi_512 `<` → `<=` | **killed** (8) |
| M11 | G8 member-sequence check weakened | survived (B1) |
| M12 | `cov.length >= 1` removed at G3 | survived (near-equivalent: length equality still holds) |
| M13 | no-data theta=+0 removed | survived |
| M14 | ordinary and product G5 passes swapped | survived (dual defect only) |
| M15 | positive force floor no longer forces coverage | **killed** (3) |
| M16 | N8 threshold `>` → `>=` | survived |

4 of the 16 mutants were killed. The survivors map to the untested rules listed under N1, N4 and B1.

## Named question 1: I64's known differences (readings for TypeScript)

1. **Failed verification: solve or pass (N4/N5). TypeScript is right.**
   - **Native behaviour:**
     - A verification **solve** failure with an escalating stop advances two slots (FK `adaptive.rs:4576–4590`, `c += 2` at `:4586`).
     - A **pass** failure always goes to `finish_terminal` (`:4593–4611`).
     - `terminal()` is unreachable for escalating stops (`:4377`).
     - `verify_precision` always records the verification-build request through `trace.requested` before anything can fail (`:4282–4291`).
     - `verification_work` is written only inside the pass (`:4327`). Records start at zero (`:4076–4079`).
   - So "v-build ref null and `verification_lme` 0" characterizes a solve failure exactly.
   - Both readers accept every emittable receipt. TypeScript also rejects records that show the pass was entered with an escalating stop, which are not emittable.
   - Python's rule is permissive, not wrong on emitted receipts. A shared mutation should pin it.
2. **The stop-rule reason locator. TypeScript is right.**
   - Native reasons that carry a quantity take `quantity`, `body` and `kind` from the same `prep.layout[index]`: `rejection_reason` (`adaptive.rs:4169–4199`) and `PublicationEnclosure` (`:4716`).
   - C1:114 requires the exact payload. A mismatched locator is not emittable.
3. **The candidate record shape. TypeScript is right.**
   - `verify_precision` runs only on the `v_index` record (`:4568, :4593`). Candidate records keep zero verification work, a null verification summary, and no verification-build request (C2:139: null iff not requested).
4. **Ordinary-pass extras.**
   - **(a) Every `diagnostic_refs` entry resolves, names the case, and is unique.** Uniqueness and resolution follow from C2:166 ("exact diagnostic refs") and C1:148 ("ordinary refs resolve"). "Names the case" for untyped listed refs has no clause, and the producer seam (C2:168) is not built. Base envelopes carry model-level diagnostics with null `affected_refs` (for example `diagnostic:physics:rule-inputs-missing`). A producer that cites one on a case's ordinary attempt would be rejected by TypeScript alone. **ROOT should rule.**
   - **(b) W2 trigger equals the initial failure; a published W2 has `force_scale_exponent ≠ 0`.** C2:158 ("records … nonzero b"; preserves the initial trigger). TypeScript is right.
   - **(c) A selected case needs `solve_quality` in {sensitive, unresolved, failed}.** Excluding `checks_passed` follows C1:101 (`not_required` means the ordinary solve passed), so TypeScript is right there. Probe P7 confirms the rejection. Excluding `not_assessed` is plausible, because selection needs an attempted initial, but it is unproven; ROOT should confirm.
   - **(d) Python's G5 re-check of the source identity.** TypeScript checks the same hash at G1 (`:189–191`), which is the correct gate (C3:300). Python's G5 copy is redundant.
   - **(e) Order.** C3:304 places ordinary references in class 2, before typed checks and work. Running them before the product WORK list, as TypeScript does, is right; Python's late report/quality/not_required block can report WORK first for a dual defect. Ordinary before product within class 2 is not specified; one ruling is needed.
5. **Structural-only differences.** I agree: the G1/G2 guards, the unreachable −0 check, and the raise locus have no observable effect.

## Named question 2: the new export

`accountingRules` (`:494–505`) is a pure predicate over one attempt object. It returns three booleans and mutates nothing (probe P6). It reads only constant tables and gives no path to a validation result, a standing value or the hold. `nativeSchedule` and `ordinaryAttempts` likewise throw or return void.

Only `validateRetainedPrecision` and `validateRetainedPrecisionTransport` build a `RetainedPrecisionValidation`, and both run their full gate sequences. **No export bypasses a gate or reaches eligibility.**

The cost is surface area: a future app module could mistake these helpers for APIs. **Keep the export** for the reader-logic tests, but mark these exports internal, or group them into one test-only namespace. Add a check that no non-test module imports them.

## What ROOT must rule on

1. The repairs for B1 and B2, and their shared mutations. B2 likely applies to Python and Rust too.
2. The G0 scope across readers (S2): thresholds, producer component/version and schema_version, and table and inherited-table hashing.
3. Known difference 4a (whether every listed diagnostic must name the case), and whether `not_assessed` is excluded for selected cases (4c).
4. The legacy `work_ref` rule (N2): whether it applies, its code, and its place in the order.
5. The checklist N11 wording (N3).
6. The R3 attempt-level reading (N4), and the order of ATTEMPT and WORK checks within the native class (N7).

## Runs and evidence

| Run | Command (cwd `WT/rv81/P/apps/desktop`) | Result |
|---|---|---|
| vitest_01 | `npm test -- src/features/results/retainedPrecision.test.ts --maxWorkers=2` | 271 passed |
| tsc_02 | `../../node_modules/.bin/tsc --noEmit --pretty false -p tsconfig.json` | exit 0 |
| oracle and probes (03–06) | `RV81_SCRATCH=WT/scratch/rv81_reader_review npx vitest run src/features/results/rv81_oracle.test.ts --maxWorkers=2` (review-only file, now removed) | 2/2. 0 oracle mismatches; probe outcomes in `probe_results.json` |
| mutants | `WT=WT python3 mutants.py` | 16 run, 4 killed, copy restored |
| vitest_07 / tsc_08 (final, on the restored copy) | same as 01/02 | 271 passed; exit 0 |

**Environment:**
- Node v24.18.0, npm 11.16.0, vitest 4.1.10.
- `node_modules` is linked to READER's target.
- The WASM assets were copied verbatim from READER's `public/` into the copy, with no build:
  - `open_pipe_stress_operation_applier.js` `5682432840…`;
  - `_bg.wasm` `7843297271…`;
  - the self-weight `.js` `ea2b3fd611…` and `_bg.wasm` `3bc83f88bf…`.
- The first run, before the copy, failed with WASM-ENGINE-ASSET-ABSENT, as expected for an archive.

**The evidence files in this folder** are listed with hashes in SHA256SUMS: `oracle_gen.py`, `oracle_results.json`, `probe_results.json`, `rv81_oracle.test.ts.txt`, `mutants.py` and `mutant_results.json`.

**Bulk files** stay in `WT/scratch/rv81_reader_review/`:
- `oracle_vectors.json`, sha256 `7bd94f31dea4d1b2f14f3ef9670c2a352a3877db5c4245357eec845573e7d3e5`. It is deterministic: seed `0x5281`, regenerated with an identical hash.
- the per-mutant and run logs.

**Not done:**
- Python was not executed: its probe outcomes are cited from its source.
- The 42 checklist IDs were confirmed by reading the code, not one shared entry per ID.
- The WASM assets were not rebuilt.
