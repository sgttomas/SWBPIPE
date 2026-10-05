# I63 return: Rust reader on snapshot 07e

I63 is a TASK (Type 2) on ROOT's 07e round, the last repair round before reader acceptance. Scope: D34, RV78-N1's rehash indexing rule, then adopt 07e. The basis is T3/ROOT_RULINGS_V1.md, from "RV79 confirmation 04" through "Round 04 closed; 07e is the last repair round before acceptance" (NUM `94fe144302`). I63 had no descendants.

- **Run:** first tool call 2026-10-04T02:09Z; done about 02:18Z, inside the 45-minute box. The memory guard (PID 5387) was running.
- **Limits held:** no Git writes or index operations; no install, new tooling or native job. One Cargo job at a time under the 1,200 s wall.
- **Toolchain:** the default, with no `DEVELOPER_DIR`.
- **Basis:** READER was at `abcb16fd27` when I started, and my 07d bytes were committed there. During the round, ROOT committed I62's 07e as `b890ce6c30`, touching only the Python reader, its tests and the corpus. NUM is now at `1df380753b`. Its two later ruling notes concern Python and do not change this scope.
- **Paths** use the brief's placeholders.
- **Status:** 07e passes in full. **Not accepted; eligibility still held** (`IMPLEMENTATION_COMPLETE = false`).

## Inputs verified

- **I62's SHA256SUMS** verify, all 21 lines, including SHARED_SNAPSHOT_07E.json (`d8d0355be0`).
- **READER's corpus** hashes to `bbca15d940`, the 07e hash. The schema (`07951edacf`) and the other shared files are unchanged.
- **Counts:** 15 cases, 263 mutations and 22 must-pass entries.
  - 4 mutations are new, at 259..262: three non-integral G0 values (`receipt_version: 1.5`, and the two limits written with `.5`), and `verification_estimate_names_rotation_row`.
  - 1 must-pass entry is new: `verification_estimate_names_moment_row`.
  - Nothing from 07d changed.

## Changed files (READER, inside the fence)

Paths are under P/core/reporting/result_export.

| File | Before (`abcb16fd27`) | After |
|---|---|---|
| src/retained_precision.rs | bb713adcebc6… | 06204326ac4658f84bbb198a70d632951491134c10aa363aeddf0789d038760c |
| tests/retained_precision_contract.rs | 37cf97b4ab5a… | e895305d11e78182fa77d264d98044a491127625b143457f75d23339b929a069 |
| src/lib.rs | 375b073135… | unchanged |

## Decisions

### D34: applied, not just confirmed

**What the 07d reader did.** It already rejected −0 at G2 in every field encoded as U or I32. But it **did not** reject −0 in the two integer fields the schema writes as enum or const values. The value-based shape check admits −0 there because it equals 0, and the field has no wire encoding. The reverted-rule run shows the result: −0 reached G5 PRODUCT_ATTEMPT, exactly like the +0 control.

**The change.** G2 is now one function, `g2()`, used by both `validate` and `validate_transport_metadata`. It runs:
1. the existing encoding walk;
2. then a scan over the whole receipt. Any JSON number equal to −0 gives G2 ENCODING_MISMATCH.

This runs before the D32 normalization, as Python's does. G0 is unchanged: `receipt_version: -0` still fails G0, because it is not 1.

**Test:** `d34_negative_zero_anywhere_in_receipt_fails_g2`. No base carries the two fields, so the test supplies them:
- **`G5aError.quantity_kind`,** in the `sanity` and `lower` variants, set as F_BASE's unavailable attempt error `{kind: g5a, cause}`:
  - −0 gives G2;
  - +0 (the control) passes G1 and G2 and fails later, at G5 PRODUCT;
  - 1 gives the same result as the control;
  - the JSON text `"quantity_kind":-0`, parsed from text, also gives G2.
- **`source_decline.constructor_counts.directional_springs`,** on the shared `unavailable_attempt_under_source_error_cause` entry:
  - +0 keeps the entry's expected G5 PRODUCT_ATTEMPT;
  - −0 gives G2;
  - 1 gives G1, the const.
- **A U field** (`case_charge: -0.0`) still gives G2.

**Mutant check.** With the −0 scan removed, the test fails on all five −0 cases, each giving G5 PRODUCT_ATTEMPT. With the scan restored, it passes.

### RV78-N1: harness rehash aligned to the 07e `format_rule`

- **`index()`** is a strict integral value: a JSON number, never a boolean, finite, integral, ≥ 0 and not −0. This was already the case after 07d. It applies to rehash references and to edit-path indices.
- **Source identities.** The rehash now recomputes `source_identity_sha256` for each **selected** case whose `source_ref` resolves, as the format rule says. Before, it recomputed it for each case that carried the field. All 263 mutations matched under both rules (`shared_rehashed_first_failure_mutations`, in the in-progress run and in the final run). The 22 must-pass entries were checked under the new rule only.
- **Preparation hashes** are recomputed only when the attempt resolves and every one of its members is prepared.
- **Order:** preparations, then source identities, then the publication hash, then the receipt hash, then `after_rehash` edits applied literally. This was unchanged.
- **Test:** `rehash_index_rule_07e`.
  - These are indexes: 0, 1, 1.0, 0.0 and 7.
  - These are not: true, false, 0.5, −0.0, −1, −1.0, null, "0", [0] and {}.
  - On the ORD base, an unresolved or invalid `source_ref` or `attempt_ref` (true, 0.5, −0.0 or 9) is skipped, and the stated digest is left for the reader to report.

### Adopting 07e

- The slice helper asserts 263 mutations, and the must-pass test asserts 22 entries.
- A new slice, `snapshot_07e_mutation_outcomes` (259..263, tag `I63_OUTCOME_07E`), tallies G0 UNSUPPORTED 3 and G5 ATTEMPT 1.
- **Rust already met the new pins:** the 07d G0 uses `uint()`, and D33 already restricts the estimate to Force and Moment rows.

## Commands and results

The command is the standing one: from READER, `CARGO_BUILD_JOBS=4 RUST_TEST_THREADS=2 CARGO_NET_OFFLINE=true CARGO_TARGET_DIR=WT/targets/i63-reader/result_export perl -e 'alarm shift; exec @ARGV' 1200 cargo test --locked --offline --manifest-path RE/Cargo.toml --test retained_precision_contract -- --test-threads=2`. It ran on the default toolchain.

| Run | State | Result |
|---|---|---|
| D34 and index-rule tests | the new rules | 2 passed |
| D34 reverted | the −0 scan removed | the d34 test failed on all five −0 cases |
| full suite, on I62's in-progress corpus (`bbca15d940`) | before the counts were updated | 43 passed and 12 failed, every failure on a count assertion (263/22 against 259/21). Every mutation already matched in `shared_rehashed_first_failure_mutations`. |
| **full suite (final)** | final bytes on the published 07e | **56 passed, 0 failed** |
| outcome capture, `--nocapture snapshot_0 shared_must_pass` | final bytes | 13 passed; 263 outcome lines and 22 must-pass lines |

**Against the bar (07e):**
- **Mutations:** all 263 match their expected first gate and code (G7 per reader).
- **must_pass:** all 22 pass with the base case's classifications.
- **Cases:** all 15 validate with their expected classifications.
- **Tables:** `OUTCOMES_07E.json`, with all 263 mutations in corpus order and all 22 must-pass entries.

## Remaining known differences

**None known.** D34 now matches Python: −0 anywhere in the receipt fails at G2, before normalization. The harness rehash follows the stated format rule, as Python's `_rehash_ref` does. The only exception is that Rust also skips a source or attempt that resolves to a non-object, where Python would raise; no corpus entry reaches it.

This rests on the shared corpus and targeted comparisons, not an exhaustive comparison. I have not compared TypeScript at its head.

## Files read (sha256)

| sha256 | File |
|---|---|
| 86589d2cd3511bb4ce71da8195128c526e1d1e72de130536ba62d0583b3dc031 | T3/ROOT_RULINGS_V1.md (as at NUM `1df380753b`; the grant's sections were read at `94fe144302`) |
| d8d0355be04ecc0fecb3fe8b038ce53a753ee104428dee96667ef50694002bf7 | R/I62/review_repair_07/SHARED_SNAPSHOT_07E.json |
| f9981ec23298e39337a2c2395a269ce4d9d260d7a1d97eb94df34d4b50968ee4 | R/I62/review_repair_07/SHA256SUMS |
| e8f22d8bc7e0551cd722b928b13a8c76ee5339035752fd153079c1096fff180c | READER/P/tests/test_retained_precision_contract.py (`_rehash_ref`, for the index rule only) |
| bbca15d94055227da364ce4b8a7223d1350ac5b85b1219c94b1405b43c56340a | READER/P/fixtures/results/retained_precision_cases.json |
| 07951edacfedd410c153929ee75bb5bada15dbd222369ec63240c678b233b61c | READER/P/schemas/retained_precision_mp_v2.schema.json (`G5aError`, `PublicError`, `constructor_counts`) |
| 375b07313518a0cdd09e317b83b9ca7536acef5c80fd4cd0a94920fd8bf2486d | READER/P/core/reporting/result_export/src/lib.rs |
| bb713adcebc6… / 06204326ac4658f84bbb198a70d632951491134c10aa363aeddf0789d038760c | src/retained_precision.rs, before and after |
| 37cf97b4ab5a… / e895305d11e78182fa77d264d98044a491127625b143457f75d23339b929a069 | tests/retained_precision_contract.rs, before and after |

## Bulk (WT/scratch/i63_review_repair_07e/)

| sha256 | file |
|---|---|
| 2de8c1ed6d4557348b13bb343e859a62176efc0ed1515bb97b0a9aa88662f2eb | full_test_07e.txt (final full run) |
| 098a810f5deac0eeaa63f64017697b69ff34ce339f0c5297e08e80e383c93bd6 | outcomes_07e.txt (outcome capture) |
| 8d13b60693c54ddc9bc7fcc5c58e3fdd775053b6b00cf8a1a027c167a9099317 | full_on_07d.txt (run on the in-progress corpus; count failures only) |
| 36097094a7a4e44b05aa0d71969979deabc13086eab9d9609c184ac4a5238baa | mutant_d34.txt |
| 218c98423e52cf945caf11215b03635930761341a03ee710d431df4c28bb809e | mutant_kills_07e.txt |
| da6a951ee4fe2b8bfbda06e0b7d812bfeb7a9816bf1ee5dfd2afe9167e1b97c6 | reader_07e.diff |
| 06204326ac4658f84bbb198a70d632951491134c10aa363aeddf0789d038760c | after_rs.rs |
| bb713adcebc6… / 37cf97b4ab5a… | before/ (both files) |
