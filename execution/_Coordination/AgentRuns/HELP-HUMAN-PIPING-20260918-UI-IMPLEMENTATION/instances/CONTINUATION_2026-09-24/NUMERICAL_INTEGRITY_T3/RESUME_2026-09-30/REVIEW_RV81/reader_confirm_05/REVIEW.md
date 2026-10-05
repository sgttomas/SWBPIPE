# RV81 confirmation round 05: TypeScript D34 and snapshot 07e (final scoped check)

RV81 is a TASK (Type 2) reviewer. ROOT (HELP_HUMAN, Agent 0) dispatched it on the final scoped grant before reader acceptance (workflow §3), and ROOT is the return path. RV81 did not write any of the changes and did not delegate.

All evidence comes from RV81's own probes, mutants and oracle-free comparisons, rerun on the new head and on a pre-D34 copy of the reader. The authors' tests serve only as the kill criterion for the mutants.

**Candidate:** READER `63355a91d24dce246f2b85ee4601825824fd1dac`, reviewed from a `git archive` in `WT/rv81/`. NUM is at `5c5d2a15c4`.

**Files under review:**

| File | sha256 |
|---|---|
| `P/apps/desktop/src/features/results/retainedPrecision.ts` | `eb8b9d5aa04eb1984e60315959b4d64c474e4c255908f0e163c3de447e01abee` |
| the test file | `a8c03ee1014f…` |
| corpus (snapshot 07e) | `bbca15d940…` |
| schema (unchanged) | `07951edacf…` |

Snapshot 07e has 15 cases, 263 mutations and 22 must-pass entries.

**Diff reviewed:** `abcb16fd27..63355a91d2`. The reader changed by +11/−3 lines and the test file by +79/−4.

**Basis read:**
- ROOT_RULINGS_V1.md, from "RV79 confirmation 04" to the end;
- I64's RETURN_07E.

**Host steps, disclosed:**
- `WT/rv81/P/node_modules` is linked to READER's target.
- READER's prebuilt `public/wasm-engine` and `public/self-weight-engine` were copied into the copy, not built.

**Window:** 2026-10-04 02:20Z to about 02:33Z, inside the 30-minute box. The memory guard (PID 5387) was running throughout.

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

**Runs:** baseline and final, vitest 428/428 and tsc exit 0.

## 1. D34: `negativeZeroFree` rejects −0 anywhere in the receipt at G2

- **Placement:**
  - The walk is defined at `:160–165`.
  - `integrity` calls it at `:222`, after every G1 hash check and before the schema `encoding` walk. It therefore reports G2.
  - It covers the whole receipt (`source.retained_precision`) through every object and array.
  - The transport entry reaches it too, because it shares `integrity`.
- **Sweep (probe A):** RV81 wrote every numeric 0 in all 15 base receipts as −0, one position at a time and without rehashing; canonical JSON writes 0, so the hashes still match. All **9,114 positions** give G2 ENCODING.
  - The same sweep on a pre-D34 copy also gives G2 at all 9,114. Every counter and index position was already rejected by `uint` and the encoding check, so the sweep alone does not prove the new walk is needed.
- **Where D34 changed the outcome (probes B1 and B2):** both positions are integer fields the schema writes as enum or const members, and no base carries either. RV81 built each one, rehashed it, and then wrote −0:

| Field | Before D34 | Now |
|---|---|---|
| `G5aError.quantity_kind` (enum [0, 1]), in a g5a error on the facade base's unavailable attempt | G1 | **G2** |
| `source_decline.constructor_counts.directional_springs` (const 0), on P′ case 1 | G1 | **G2** |

- **Transport (probe A2):** a −0 `source_ref` gives G2 through `validateRetainedPrecisionTransport`, and the unedited control passes.

## 2. The G1 change admits nothing new at G1, and every rejection still holds

- **The change itself** (`shape`, `:133–136`): before the const and enum comparison, only a value for which `Object.is(v, -0)` holds is mapped to 0.
  - The schema has 27 numeric const or enum members, and no object or array const.
  - No `oneOf` mixes a numeric const with another numeric branch, so the mapping cannot create a second match.
  - Only the two zero-admitting positions above can change outcome.
- **Before and after**, from probes B1–B4 run on both readers:

| Position | Value | Before D34 | Now |
|---|---|---|---|
| `quantity_kind` | 0, 1 | past G2 (G5) | past G2 (G5) |
| | **−0** | G1 | **G2** |
| | 2, −1, 0.5, 1e-310, `true`, `"0"`, `null` | G1 | G1 |
| `directional_springs` | 0 | pass | pass |
| | **−0** | G1 | **G2** |
| | 1, 1e-310, 0.5, `true`, `"0"`, `null` | G1 | G1 |
| record `precision` (enum, no 0) | −0, 0, 129, `true` | G1 | G1 |
| | 128.0 | pass | pass |
| `receipt_version` | −0, 1e-310 | G0 | G0 |
| | 1.0 | pass | pass |
| `work.case_limit` | −0 | G0 | G0 |
| | 2e10 | pass | pass |

  The only changed rows are the two −0 rows, and each moved from G1 to G2.

## 3. RV81 round 04 NOTE 1: the D33 kind set is now pinned

- **Shared entries in 07e:**
  - `verification_estimate_names_rotation_row` expects G5 ATTEMPT;
  - the must-pass `verification_estimate_names_moment_row`.
- **Mutants:** R26 (rotation admitted) and R27 (moment refused), which survived in round 04, are now each killed by 1 test.
- **Probe D:** RV81's own estimate reason, naming a real body-0 layout row of each kind, gives:

| Row kind | Result |
|---|---|
| translation | G5 ATTEMPT |
| rotation | G5 ATTEMPT |
| force | pass |
| moment | pass |

## 4. The diff and the harness's 07e rehash rule

**Reader:** the only removed lines are the two const/enum comparisons and the `encoding` call. Each is replaced:
- the comparisons by the same comparison on the −0-mapped value;
- the `encoding` call by `negativeZeroFree` followed by `encoding`.

Both pre-existing −0 checks are kept: in `uint` (`:128`) and in the `uint`/`i32` encoding. Nothing is weakened.

**Harness (test file):**
- **`rehashRef`:** it resolves a reference only when the reference is a strict integral value: a number, never a boolean, finite, integral, ≥ 0, not −0, and in range. Otherwise it skips the reference, so the reader reports the defect.
  - Before, `true` or `0.5` caused a `TypeError` inside the harness.
- **Preparation hash:** recomputed only when every member is prepared. The reader checks the hash only in that state, so this changes bytes, not outcomes.
- **Edit paths:** an array index in an edit path must be a strict index.

These are harness-only changes that make it stricter. The harness change only adds and replaces lines, nothing is skipped, and the reader's outcomes are unaffected.

## Mutants: 51 run, 43 killed, 8 survived

The run used round 04's set plus R32–R35 for D34. Each mutant was applied to a clean copy, run with `npm test -- src/features/results/retainedPrecision.test.ts --maxWorkers=2`, then restored and checked by sha256.

| Mutant | Result and reason |
|---|---|
| R32 (`negativeZeroFree` call removed) | killed (2) |
| R33 (G1 −0 mapping removed) | killed (2) |
| R26, R27 (D33 kind set) | **killed** (1 each); they survived in round 04 |
| R34 (walk moved after `encoding`) | survived: equivalent, since both are G2 with the same code |
| R35 (mapping broadened to any \|v\| < 1e-300) | survived: **a test gap**, see NOTE 1 |
| R30, R31 (−0 dropped from `uint`, or from the encoding check) | survived: now triply redundant with the D34 walk |
| M11, M12, R03, R06 | survived for the reasons recorded before: equivalent, equivalent under D29, redundant, unreachable |

## NOTEs

1. **The rule that "only −0 is mapped" has no test.**
   - Mutant R35 maps any |v| < 1e-300 to 0 at G1, and no test catches it. Under R35, `directional_springs: 1e-310` would pass both G1 and G2 (it is a const with no encoding tag).
   - The reader itself is correct: probe B2 gives G1 for 1e-310.
   - Optionally, add a reader-local test that a tiny non-zero value at a const-0 or enum position still fails G1.
2. **−0 at a position where 0 is not a valid value fails G1, not G2.**
   - Example: record `precision`, enum 128/256/512.
   - Under value semantics (D25, D32), −0 equals 0, and 0 is not a member there. G1 precedes G2, so G1 is the right first failure.
   - D34's G2 code applies where 0 is a valid value. This matches the gate order. RV78's cross-reader probes can confirm that the readers agree.
3. **Unchanged from round 04:**
   - the survivors M11, M12, R03 and R06;
   - the N4 known limit.

## Runs and evidence

| Run | Command (cwd `WT/rv81/P/apps/desktop`) | Result |
|---|---|---|
| vitest_01 | `npm test -- src/features/results/retainedPrecision.test.ts --maxWorkers=2` | 428 passed |
| mutants | `WT=WT python3 mutants5.py` | 51 run, 43 killed, copy restored |
| probes (03) | `RV81_SCRATCH=WT/scratch/rv81_confirm05 npx vitest run src/features/results/rv81_confirm05_probes.test.ts --maxWorkers=2` | `probe_results.json` |
| probes on the pre-D34 reader (04) | the same, with the two D34 edits reverted in the copy; restored from `git show 63355a91d2` and checked by sha256 | `probe_results_pre_d34.json` |
| vitest_05 / tsc_06 (final, restored copy) | the vitest command above; `../../node_modules/.bin/tsc --noEmit --pretty false -p tsconfig.json` | 428 passed; exit 0 |

**Environment:** Node v24.18.0, npm 11.16.0, vitest 4.1.10.

**In this folder:**
- `REVIEW.md`;
- `rv81_confirm05_probes.test.ts.txt`;
- `probe_results.json`;
- `probe_results_pre_d34.json`;
- `mutants5.py`;
- `mutant_results.json`;
- `SHA256SUMS`.

**Bulk:** in `WT/scratch/rv81_confirm05/`, with `reader.diff` `9002e32a22…`, `test.diff` `7c9a15b1a7…`, and the logs.

**Not done:** Python and Rust were not executed; the scope is TypeScript only.
