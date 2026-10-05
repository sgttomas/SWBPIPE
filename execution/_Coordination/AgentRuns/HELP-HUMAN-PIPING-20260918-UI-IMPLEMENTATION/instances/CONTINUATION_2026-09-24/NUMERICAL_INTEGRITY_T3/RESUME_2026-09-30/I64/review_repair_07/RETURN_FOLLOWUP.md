# I64 return: review repair 07, phase-1 follow-up (checkpoint A rulings)

I64 is a TASK (Type 2) under ROOT (HELP_HUMAN, Agent 0). This follow-up grant was sent by ROOT's mid-run message in the same harness-native subagent session. I64 had no descendants.

- **Run:** 2026-10-03T23:38:20Z to the final checks at 23:40:18Z; this file was written about 23:43Z. Inside the 30-minute box.
- **Host:** the M5 host. The memory guard (PID 5387) was running.
- **Limits held:** no Git writes or index operations; no install, build, Cargo, solver or native job.
- **Other authors:** the shared files and I62's and I63's files were not touched. I63's Rust files showed as modified in READER.
- **Basis:** the ruling "Checkpoint A: rulings on the native facts for snapshot 07" (NUM ff570d05d8). ROOT_RULINGS_V1.md was read at sha256 `0e7a2a0c930629f1a87ed2e8449b4807525d616844770785b5c4cec4f601962d`.

## Changed files (READER at e09b86958f, inside the fence)

| File | Before (phase 1, e09b86958f) | After |
|---|---|---|
| P/apps/desktop/src/features/results/retainedPrecision.ts | 240ccf4532104f559f51029d08332b0f827d9ad4f1713a44334fce1f1cb02ade | d5854a9a6c1e7279ddcddbf034d7bd0f1e2235800f6ace97ba933559ffadb104 (107748 B) |
| P/apps/desktop/src/features/results/retainedPrecision.test.ts | 03aa2d347d7dfd8f976242f59f1bca015f283cb1e99ef007234f7f671a813fdd | 7e64148c15ed4ef80dfe3ea12e5c600f23a82bcd9a9789120c5461fd2c0cce20 (65169 B) |

The shared files are unchanged: corpus 06d `d02701ed6a…`, schema, definition, table and inherited table.

## Changes

1. **D1 correction.** The `old.length >= 1` requirement on unsourced complete old coverage is removed. G3 still requires the list to match every CaseSource's member count (one model). Without a CaseSource, G8 compares the list with the invocation, as before.
   - Test: "D1 (checkpoint A): …". An empty list still fails G3 here, but only through the count, because P′ carries a one-member CaseSource.
   - No corpus base lacks a CaseSource, so the no-inventory branch is not exercised. ROOT's ruling records the known limit there.
2. **D6a.** The list-level "names the case" check is dropped. Untyped `diagnostic_refs` must be unique and resolve (G5 ATTEMPT, class 2). Typed references are unchanged: listed, resolving, naming the case, and non-null where required.
   - Test: "D6a (checkpoint A): …":
     - a listed model-level diagnostic with null `affected_refs` passes;
     - a dangling reference fails;
     - a duplicate reference fails;
     - the same model-level diagnostic used as the typed `d5_diagnostic_ref` fails.
3. **D6b.** Kept unchanged: a selected case's quality must be `sensitive`, `unresolved` or `failed`. Test "D6b (checkpoint A): …": those three pass; `checks_passed` and `not_assessed` fail with G5 ATTEMPT.
4. **Native-class crash fallback.**
   - The "WORK if a WORK defect was recorded" heuristic is removed.
   - Every reference the native class follows is now resolved explicitly, and a dangling one fails with the code of the check that follows it.
   - The catch-all in `nativeRuns` is only a fail-closed fallback, with the class code G5 ATTEMPT_MISMATCH.
   - Test "native class: each dangling reference reports the code of the check that follows it":
     - a dangling `shared_build_ref` gives WORK;
     - a dangling `candidate_record` gives ATTEMPT;
     - a dangling `origin.group` gives ATTEMPT.

**References in the native class (`nativeClass` and `nativeSchedule`), each resolved explicitly:**

| Reference | Resolution | Code (the check that follows) |
|---|---|---|
| `call.run_refs[pos]` → Run | `at(runs, ri)` | ATTEMPT (Run origin) |
| `call.owner_refs[pos].index` → case | `at(b.cases, oi.index)` | ATTEMPT |
| `call.source_refs[pos]` → CaseSource | `at(b.sources, si)` | ATTEMPT |
| `run.origin.group` → group | `at(b.groups, group)` | ATTEMPT (group call and stiffness) |
| `record.shared_build_ref` and `record.verification_shared_build_ref` → build | `buildOf(bi)`, which is null when it dangles: `work(false)`, and stage sums treat it as absent (**made explicit in this follow-up** at the stage-sum site, and shared with the slot check) | WORK (slot, group, work and stages) |
| `attempt.candidate_record` and `attempt.verification.record` → record | `at(records, …)` (also inside `nativeSchedule`) | ATTEMPT |
| `attempt.origin.attempt` → prior attempt | `at(attempts.slice(0, ai), …)` | ATTEMPT |
| `charge.record` → record (then `amounts[record]`) | `at(records, f.record)` | ATTEMPT (fragments) |
| The selected terminal's candidate and verification records | `at(records, …)` (**made explicit in this follow-up**; earlier `at` calls already guaranteed it) | ATTEMPT |
| `group.call` → call | `at(b.calls, g.call)` | ATTEMPT (D5e) |
| `group.source_refs[i]` → CaseSource | optional lookup inside the membership predicate | ATTEMPT |
| The C5 partition's `run_refs[pos]` and `source_refs[pos]` | `at(runs, ri)` and `at(b.sources, si)` (**made explicit in this follow-up**; reachable only after the earlier call-loop lookups) | ATTEMPT (C5) |
| A record reason's `quantity` → layout row | `source.layout.some(…)` | ATTEMPT (D5d) |
| A build's cache entries | the cache map built from validated builds only | WORK |

## D3 confirmation

I62's checkpoint A matched this reader's phase-1 work on D3: no single-defect code differs between the readers. Under the class-1 convention, all 178 06d mutations and 18 must-pass entries keep their outcomes in TypeScript; phase 1's probe_05 and this follow-up's vitest_02 both show this.

## Commands (from READER/P/apps/desktop)

| Run | Command | Result |
|---|---|---|
| vitest_01 | `npm test -- src/features/results/retainedPrecision.test.ts --maxWorkers=2` | 318 passed |
| **vitest_02 (final)** | same | **318 passed, 0 failed**: all 06d entries (178 mutations, 18 must-pass, 15 cases), the earlier tests, and the phase-1 and follow-up relation tests |
| **tsc_03 (final)** | `READER/P/node_modules/.bin/tsc --noEmit --pretty false -p tsconfig.json` | **exit 0** |

All shared hashes were identical before and after (final_hashes.txt). No vitest or tsc process remained afterwards.

## Remaining known differences from the other readers

- **The fail-closed fallback code.** A non-gate exception inside the native class reports G5 ATTEMPT_MISMATCH. With every reference explicit, I know of no input that reaches it. The other readers' catch-alls map to their G5 default.
- **Bundled-file G0 codes** (unchanged from phase 1, and unreachable from a receipt).
- **Until Python and Rust land their repairs,** RV78's probes still separate them from TypeScript, as listed in RETURN.md.

There are no other known differences.

## Bulk (WT/scratch/i64_review_repair_07_followup/)

| sha256 | Bytes | File |
|---|---|---|
| 240ccf4532104f559f51029d08332b0f827d9ad4f1713a44334fce1f1cb02ade | 107200 | before/retainedPrecision.ts |
| 03aa2d347d7dfd8f976242f59f1bca015f283cb1e99ef007234f7f671a813fdd | 62142 | before/retainedPrecision.test.ts |
| d5854a9a6c1e7279ddcddbf034d7bd0f1e2235800f6ace97ba933559ffadb104 | 107748 | retainedPrecision.after.ts |
| 7e64148c15ed4ef80dfe3ea12e5c600f23a82bcd9a9789120c5461fd2c0cce20 | 65169 | retainedPrecision.test.after.ts |
| 4b3805a9de91f384230f16435762f080bf3c01afeb77267b6d4ad8dc04e92307 | 7785 | followup_reader.diff |
| 1ccaa49e7e7737d3e5779028e6815b35714fda97ef78797504db5a65762dbc6f | 4898 | followup_test.diff |
| f00f98e32cf3406fd52139e76b98248b15d45f49802cbbf8686cf03ae97e1b9d | 440 | vitest_01.log |
| 62dcbc4e25ae71e837935529069a87811419c6d58320889e6f1c9c76349dd2e7 | 560 | vitest_02_final.log |
| d0e2f316a5acd1331099b5adbd816149d71bbf4a38db56b7d9c788dc5a0a5365 | 112 | tsc_03_final.log |
| 038b80e98e078abb50ef163f2656d60298e096e071f0daf09c9beec6956ec80b | 259 | final_hashes.txt |
