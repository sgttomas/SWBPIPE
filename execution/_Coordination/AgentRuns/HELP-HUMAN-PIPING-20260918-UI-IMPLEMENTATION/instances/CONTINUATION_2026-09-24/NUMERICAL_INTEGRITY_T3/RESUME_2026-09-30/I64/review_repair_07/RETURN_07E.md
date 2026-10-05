# I64 return: D34, RV78-N1, and snapshot 07e

I64 is a TASK (Type 2) under ROOT (HELP_HUMAN, Agent 0), working on its standing ownership of the TypeScript reader. ROOT sent the 07e round as a mid-run message in the same harness-native subagent session. I64 had no descendants.

## Run conditions

- **Timing:** the grant was read at 2026-10-04T02:09:25Z and the final checks finished at 02:16:57Z. This file was written at about 02:20Z, inside the 45-minute box.
- **Host:** the M5 host. The memory guard (PID 5387) was running throughout.
- **Limits held:**
  - no Git writes or index operations;
  - no install, build, Cargo, solver, native, UI or DEC-025 job;
  - the existing node_modules link and WASM assets were used as found.
- **Other authors' files:** no shared file, and no file of I62's or I63's, was touched.

## Basis

| Item | Record |
|---|---|
| Rulings read | NUM 94fe144302, ROOT_RULINGS_V1.md, from "RV79 confirmation 04" through "Round 04 closed; 07e is the last repair round before acceptance" (sha256 `3e5bb96fa730aa1c515ebd20365accd21feabc55b25d8e9779ac2031290c1c21`) |
| Later NUM head | NUM moved to 1df380753b. Its two commits add only the 07e commit record and a correction about the Python reader's G2 call. Neither changes this scope. |
| Snapshot | I62's SHARED_SNAPSHOT_07E.json, sha256 `d8d0355be04ecc0fecb3fe8b038ce53a753ee104428dee96667ef50694002bf7` |

**No stop condition arose.** One change moves a rejection between gates, so it is set out here for review rather than left to the diff:

- **Before:** in the prior reader, −0 in the two enum/const fields already failed, but at **G1**. G1's `shape` compared const and enum members with `same()`, which uses `Object.is`, so −0 did not match the member 0.
- **After:** D34 rules that rejection to **G2 ENCODING**. G1 now compares those members as JSON values (−0 matches 0), and the new G2 walk `negativeZeroFree` rejects the −0.
- **Why it is not a weakening:** the walk runs in the same `integrity` call, straight after the G1 hashes and before any later gate, so a receipt can no longer pass G1 and then go unchecked.
  - Every receipt the prior reader rejected is still rejected. The set of accepted receipts did not grow.
  - The rejection now also covers positions the schema walk never reaches.
- **Scope check:** the schema's only numeric enum and const members sit in the receipt. The publication rows (`RawRow`) have none, so the G1 comparison change affects nothing outside the receipt.

## Changed files (inside the fence)

READER was at abcb16fd27 when the round began. It is now at b890ce6c30, I62's 07e commit, which touches only the corpus and Python files. HEAD's copies of both fenced files equal the before copies below.

| File | Before (HEAD) | After |
|---|---|---|
| P/apps/desktop/src/features/results/retainedPrecision.ts | 136f39108d30760695e3c632d699462dac66650ae5d364b162931482ad60c8dc | eb8b9d5aa04eb1984e60315959b4d64c474e4c255908f0e163c3de447e01abee (114945 B) |
| P/apps/desktop/src/features/results/retainedPrecision.test.ts | ca980d70672e87c229e5b30d6b88283ed63ec735a51eca7fa94da2199eedf87e | a8c03ee1014feb1eac144ee65799cb028506f335ae780393a2603f7044f5f199 (86020 B) |

**Shared files:**
- The snapshot 07e corpus is `bbca15d94055227da364ce4b8a7223d1350ac5b85b1219c94b1405b43c56340a`. It is READER's working-tree and HEAD corpus, and equals the hash in SHARED_SNAPSHOT_07E.json. 07e was adopted on that match.
- These are unchanged: the schema `07951edacf`, the definition `3e0779a45a`, the table `c74742ce6a` and the inherited table `ae55503d44`.

## Changes

### D34 in the reader

**The new G2 walk (lines 160–165, called at line 222).** `negativeZeroFree` is a total walk of the receipt (`source.retained_precision`). It fails G2 ENCODING on any number for which `Object.is(x, -0)` holds; `x === 0` is also true for −0, so equality alone cannot detect it.
- It runs at the end of `integrity`, before the schema-driven `encoding` walk.
- It reaches every position: counters, indexes, enum and const members, and items the schema leaves untyped.
- The existing −0 checks are kept. These are `uint` at line 128 and the `uint`/`i32` encoding check at line 151, the pair RV81-N2 pinned. −0 is now caught in three places.

**The G1 comparison (lines 133–136).** `shape` compares const and enum members after mapping −0 to 0, as set out above.

**Reader-local tests**, in the block "07e round (D34): -0 anywhere in the receipt fails G2 ENCODING". No corpus base carries the two named fields, so these tests construct them.

| Test | What it does | Result |
|---|---|---|
| `G5aError.quantity_kind` | Starts from the must-pass `cert_failed_after_summary_storage` and sets attempt 1's error to `{kind: g5a, cause: {kind: sanity or lower, ..., quantity_kind}}`. | −0 (asserted to reach the reader as −0) gives G2 ENCODING. 0 gets past G2. |
| `constructor_counts.directional_springs` | Starts from the 07d mutation `unavailable_attempt_under_source_error_cause` and sets the field. | −0 gives G2 ENCODING. 0 keeps the entry's G5 PRODUCT_ATTEMPT. |
| Every numeric 0 | Rewrites each numeric 0 in the `ordinary_prepared_synthetic` receipt, one at a time, as −0. The hashes are unchanged, because canonical JSON writes 0. | Every rewrite gives G2 ENCODING. |

On the prior reader, the first two tests fail with G1 RECEIPT and the sweep passes (vitest_02_new_on_before). So the tests are decisive exactly where the reader changed, and the sweep confirms that the counter and index positions were already at G2.

### RV78-N1 in the test harness

The harness now follows the 07e `format_rule`, matching Python's `_rehash_ref`.

- **The index rule.** `rehashRef` resolves `sources[*].preparation.attempt_ref` and, for each selected case, `cases[*].source_ref` only when the reference is a strict integral value: a JSON number, never a boolean, that is finite, integral, ≥ 0 and not −0.
  - A reference that is not an index, or does not resolve, is skipped and nothing is rehashed for it. The reader then reports the defect.
  - Before this change the harness indexed directly. For example, `true` or `0.5` gave `undefined`, and a selected case's rehash then threw a `TypeError` while destructuring it.
- **The preparation hash** is recomputed only when the attempt resolves and every member is prepared (`format_rule.preparation_hash`). Before, it was recomputed whenever the attempt existed.
  - This changes bytes only, not outcomes. The reader checks that hash only when every member is prepared.
- **Edit paths** (`format_rule.edit_paths`): an array index in an edit path must be a strict index, or the entry is refused. Every corpus path already uses integers.
- **The harness test** "RV78-N1: the harness rehash indexes only strict integral values and skips everything else" covers:
  - the index table (`0, 1, 1.0, 0.0` resolve; `true, false, 0.5, -0, -1, 2, NaN, Infinity, null, undefined, '0'` do not);
  - a selected `source_ref` of `true`, `0.5` or `'0'`: the recorded identity hash is kept, and the reader reports G1, G2 and G1 respectively;
  - edit-path indices `true, 0.5, -0, '0'`, which are refused, and `0.0`, which is accepted.

## Runs (from READER/P/apps/desktop)

| Run | Corpus | Result |
|---|---|---|
| vitest_01_new (07e block only) | 07e working tree `bbca15d940`, before freeze | 3/3 |
| vitest_02_new_on_before (prior reader) | the same | the two field tests fail (G1); the sweep passes |
| tsc_04, vitest_05_wip_corpus, probe_wip | the same | exit 0; 428/428; 263/263 and 22/22 |
| vitest_06_07e, tsc_07_07e | 07e `bbca15d940`, frozen | 428/428; exit 0 |
| probe_07e (temporary probe) | 07e | **263/263 mutations** at the expected first gate and code (expected_by_reader.typescript where present); **22/22 must-pass** |
| probe_07e_on_before (temporary probe, prior reader swapped in) | 07e | 263/263; 22/22. No shared 07e entry depends on D34, and the prior reader already met S1 and the D33 kind set. |
| **vitest_08_final** | 07e | **428 passed, 0 failed**; all 15 cases validate |
| **tsc_09_final** | — | **exit 0**, no output |

- **Probe hygiene:** each temporary probe was removed afterwards. The test file and the reader were restored byte-identically.
- **Commands:**
  - `npm test -- src/features/results/retainedPrecision.test.ts --maxWorkers=2`
  - `READER/P/node_modules/.bin/tsc --noEmit --pretty false -p tsconfig.json`
- **Hash bracketing:** the final run's hashes were identical before and after (final_hashes.txt). It covers the reader, the test, the schema, the corpus, the definition and both tables.

### The 07e delta on this reader

OUTCOMES_07E.json has every entry with its raising line and source decision.

| Entry | Expected | Final reader | Prior reader |
|---|---|---|---|
| g0_receipt_version_non_integral (RV79-S1) | G0 | G0 (header:195) | same |
| g0_case_limit_non_integral (RV79-S1) | G0 | G0 (header:196) | same |
| g0_invocation_limit_non_integral (RV79-S1) | G0 | G0 (header:196) | same |
| verification_estimate_names_rotation_row (RV81-N1) | G5 ATTEMPT | G5 ATTEMPT (nativeSchedule:315, D33) | same |
| MP verification_estimate_names_moment_row (RV81-N1) | pass | pass | pass |

The S1 pins hold in TypeScript because G0 compares `receipt_version`, `case_limit` and `invocation_limit` with `===`. TypeScript has no step that truncates non-integral values (M29 is Python's `_integral`).

## Remaining known differences

These are unchanged from RETURN_07B.md:
- the fail-closed fallback code (accepted, unreachable);
- the bundled-file G0 codes (unreachable from a receipt);
- the explicit G1/G2 guards;
- the `@internal` test hooks.

`SUMMARY_COVERAGE_COMPLETE` stays false.

## Bulk (WT/scratch/i64_review_repair_07e/)

| sha256 | Bytes | File |
|---|---|---|
| 136f39108d30760695e3c632d699462dac66650ae5d364b162931482ad60c8dc | 114288 | before/retainedPrecision.ts |
| ca980d70672e87c229e5b30d6b88283ed63ec735a51eca7fa94da2199eedf87e | 79948 | before/retainedPrecision.test.ts |
| eb8b9d5aa04eb1984e60315959b4d64c474e4c255908f0e163c3de447e01abee | 114945 | retainedPrecision.after.ts |
| a8c03ee1014feb1eac144ee65799cb028506f335ae780393a2603f7044f5f199 | 86020 | retainedPrecision.test.after.ts |
| 7f3d75a98224febf4e5fbc94081bb381c256e9b67153a0f09e701154a36332fe | 2779 | r07e_reader.diff |
| 11b5cb1a3cc43c0bd6bafd80914214cc6220ca0cceda8b517320aed0216e1f53 | 9333 | r07e_test.diff |
| ec4f6ebe6a6e495da906b7dcce97bcce4dfa559d1c1785b7eb8134f1625ea372 | 338 | vitest_01_new.log |
| 96905fe7fab308ced8a4b56257eba6d7896fc5f7ad453661141f8890fc101d42 | 2673 | vitest_02_new_on_before.log |
| e3be99767213eccdb1680d391e60e5bf04b674d9c89da750924d14548e72b205 | 338 | vitest_03_new.log |
| e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855 | 0 | tsc_04.log |
| 69ac00106ac709cedea89f3687d6d5ba866d3229a97f489d33caaf9967ff5f64 | 440 | vitest_05_wip_corpus.log |
| e6446d8ea204dfd1aece1041ee401015fcc01e35898412486f2d348a83798c1f | 37260 | probe_wip.txt |
| 36f42228f19c5cf25b3898e021e9b9f71e287432ac4a435d8ee1f6f1d47274ea | 338 | probe_wip.log |
| 5901b39809377e3e10fb04818ad9e0463cc6df31481cbebbf384ba238d42327d | 440 | vitest_06_07e.log |
| e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855 | 0 | tsc_07_07e.log |
| 4e352ebbec3e0e87fc77e388616208c2ee7694658b93ecc940720e3b58bd97f3 | 37260 | probe_07e.txt |
| 0356557d44a1d12fbad3f91ba1e136e66d3bbb7d5d4deac11b1e0c8f74500ef7 | 338 | probe_07e.log |
| 64af669afffdb075a3f9abb738c8a01f634c09c3e8afeced3915e01f4a108bd7 | 37211 | probe_07e_on_before.txt |
| 3fa250dd0bb8a68f41c0405c310bd85fe87b3f2c51eff6e714656489dc2e0272 | 338 | probe_07e_on_before.log |
| d3b0004b1a669b7bd5365a7edf102ed3255bbc5c92e8ed33e002ceb6959ef43b | 440 | vitest_08_final.log |
| e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855 | 0 | tsc_09_final.log |
| c931f0b3e43a815d543fc0a5c4f8f82d8249b836770c1709d8efca8e67abff7b | 259 | final_hashes.txt |
| d3d8f4e059e1086c3bdc455b546fb76b72d57da00d94997c0d10c2d20a5e6129 | 4735 | make_outcomes.py |
