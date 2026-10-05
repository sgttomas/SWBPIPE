# I63 return: Rust reader on snapshot 07d

I63 is a TASK (Type 2) on ROOT's 07d standing repair round. Scope: D31, D32 and D33, a reader-local test for D21's last-slot case (RV80-N2, M39), then adopt 07d. The basis is T3/ROOT_RULINGS_V1.md, from "T1 and T2: I61's analysis and the rulings" through "Round 03 closed; the 07d repair round" (NUM `f4d5cbbe49`). I63 had no descendants.

- **Run:** first tool call about 2026-10-04T01:29Z; done about 01:45Z, inside the 90-minute box. The memory guard (PID 5387) was running.
- **Limits held:** no Git writes or index operations; no install, new tooling or native job. One Cargo job at a time under the 1,200 s wall.
- **Toolchain:** the default, with no `DEVELOPER_DIR`.
- **Basis:** READER was at `a894d9d0ba` when I started. ROOT committed I62's 07d as `2af4a5dc50` during the round, touching only the Python reader, its tests and the corpus. NUM is now at `aca9ad785c`. Its ruling additions (T1 confirmed, 07d verified) say the readers need no change for T1.
- **Paths** use the brief's placeholders.
- **Status:** 07d passes in full. **Not accepted; eligibility still held** (`IMPLEMENTATION_COMPLETE = false`).

## Inputs verified

- **I62's SHA256SUMS** verify, all 18 lines, including SHARED_SNAPSHOT_07D.json (`a4fe71870c`).
- **READER's corpus** hashes to `12da125d9d`, the 07d hash. The schema (`07951edacf`) and the other shared files are unchanged.
- **Counts:** 15 cases, 259 mutations and 21 must-pass entries.
  - 5 mutations are new, at 254..258: `model_schema_version_0_4_0_rejected`, `forged_source_identity_float_source_ref`, `verification_estimate_names_translation_row`, and the two D19 Ready negatives.
  - 2 must-pass entries are new: `integral_float_integers_and_references` and `model_schema_version_0_1_0_accepted`.
  - One mutation changed: `unavailable_attempt_under_source_error_cause` (RV78-N2), with its expectation unchanged.

## Changed files (READER, inside the fence)

Paths are under P/core/reporting/result_export.

| File | Before (`a894d9d0ba`) | After |
|---|---|---|
| src/retained_precision.rs | abacc74664d6… | bb713adcebc6c14db1f4c531e54b3da2e7027210b9cf501408c613b0a957ba7d (173927 B) |
| tests/retained_precision_contract.rs | c984f49a54b6… | 37cf97b4ab5a8924a0f1bba6d35b8efd43fd78c7fa79135990e3791b35b336d8 |
| src/lib.rs | 375b073135… | unchanged |

## Decisions

| Item | Reader change | Reader-local test |
|---|---|---|
| **D31** | G8 admits model `schema_version` 0.1.0, 0.2.0 or 0.3.0. 0.4.0 stays excluded, and every other G8 check is unchanged. | `d31_model_schema_versions_at_g8`: 0.1.0, 0.2.0 and 0.3.0 pass; 0.4.0 and 0.0.9 give G8 INVOCATION_MISMATCH. These are invocation edits, with the digest rebound. |
| **D32, at G0** | `receipt_version` and the 20B and 60B limits are now tested by `uint()`, a value test (finite, integral, ≤ 2^53−1, not −0). This replaces the `is_u64`/`as_u64` checks at the old RS:478 and 492. | `d32_integers_by_value`: `receipt_version: 1.0` passes, and so do the float-written limits 20000000000.0 and 60000000000.0. These still give G0: `1.5`, `-0.0`, `2.0`, `20000000000.5` and `60000000001.0`. |
| **D32, after G2** | New function `integral_receipt`, in the same style as Python's `_normalize_integrals`. Once G2 has passed, every receipt number is a U or I32. Each integral-valued float under `retained_precision`, within ±(2^53−1) and not −0, is then rewritten once as the integer it denotes. Applied in `validate` and in `validate_transport_metadata`; it copies the statement only when the receipt contains a float. | **Float references resolve and pass:** `cases[0].source_ref: 0.0`, `product_attempt_ref` and `preparation.attempt_ref` as `0.0`, and `quality_binding.index: 0.0`. Every integer in every one of the 15 bases written as a float also validates. **Still G2:** `source_ref` as `-0.0` or `0.5`. |
| **D32, forged identity** | No change: G1 already resolves `source_ref` by `uint()`. | `d32_forged_source_identity_under_float_ref`: with `source_ref` written as `0` or `0.0`, the control passes and a forged identity gives G1 RECEIPT_MISMATCH. |
| **D32, harness** | The new `index()` helper indexes by integral value, used by `rehash` and by `edit` paths. `rehash` skips a case whose source cannot be addressed, as G1 does (RV79-N-e). | Covered by the shared entries and the tests above. |
| **D33** | In the D28 block, a `verification_estimate` reason must also name a Force or Moment row (G5 ATTEMPT). `charge` is not restricted. | `d33_verification_estimate_names_force_or_moment`, on p512_ladder: the node-1 UX translation row and the RX rotation row give G5 ATTEMPT. A Force row (end_action UX), a Moment row (RX), and `charge` on the translation row all pass. |
| **RV80-N2 / M39** | No change. | `d21_last_slot_verification_shared_build` (detail below). |

**The M39 test.** It uses a Ceiling run built from p512_ladder:
- a p128 candidate, rejected;
- a reused p256 candidate whose escalating v512 verification solve fails (stop: condition), so `c` reaches 3;
- the Ceiling terminal;
- consistent work, charges (10 and 27), `cache_after`, builds 0–3, and the call and body meters.

**What the test asserts:**
- **The control** clears all of G5 class 1, with neither ATTEMPT nor WORK. It fails later only because no native-faithful Ceiling case exists; that base is deferred.
- **The schedule replay alone** (`reader_logic::schedule`) accepts the run with the shared build present.
- **`verification_shared_build_ref: 2` on the failed v512 record** gives G5 ATTEMPT. No fresh attempt follows, so only the D21 record check can catch it.

**Mutant checks.** Each rule change was reverted alone in a temporary copy, then the file was restored to the same bytes. Each reversion failed its own test, and only that test:

| Reverted | Failing test |
|---|---|
| D31 | d31 |
| D32 at G0, the `receipt_version` check | d32, `receipt_version 1.0` |
| D32 at G0, the limits | d32, float limits |
| D32 normalization | d32 and the forged-identity test |
| D33 | d33 |
| M39 | the last-slot test |

**The 07c reader on the 07d entries** (`before_reader_on_07d.txt`):
- `verification_estimate_names_translation_row` was admitted;
- `integral_float_integers_and_references` failed at G0;
- `model_schema_version_0_1_0_accepted` failed at G8;
- the other four new entries already matched.

**The integer-site audit:**
- The receipt and the invocation now have no integer check based on the host's number type.
- The only `as_u64` calls left read the `minItems`, `maxItems` and `minLength` keywords of the bundled schema, not statement data.
- G8 reads invocation numbers through `bits(unit_value(..))`, which compares by value.
- The schema `const`/`enum` check (`equal`) already compares by value.
- Hashes are unaffected: canonical JSON renders `17.0` and `17` identically (canonical_json `ecma_number_to_string`).

## Commands and results

The command is the standing one: from READER, `CARGO_BUILD_JOBS=4 RUST_TEST_THREADS=2 CARGO_NET_OFFLINE=true CARGO_TARGET_DIR=WT/targets/i63-reader/result_export perl -e 'alarm shift; exec @ARGV' 1200 cargo test --locked --offline --manifest-path RE/Cargo.toml --test retained_precision_contract -- --test-threads=2`. It ran on the default toolchain.

| Run | State | Result |
|---|---|---|
| new tests only (`d31 d32 d33 d21_last`) | rule changes, on the corpus as committed in `2af4a5dc50` | 5 passed |
| six mutant runs | one rule reverted at a time | each failed its own test, as listed above |
| **full suite (final)** | final bytes; counts 259/21; slice 254..259 added | **53 passed, 0 failed** |
| outcome capture, `--nocapture snapshot_0 shared_must_pass` | final bytes | 12 passed; 259 outcome lines and 21 must-pass lines |
| the 07c reader on the 07d entries | before-bytes, restored afterwards | evidence only (above) |

**Test changes for adopting 07d:**
- the slice helper asserts 259 mutations;
- a new slice, `snapshot_07d_mutation_outcomes` (254..259, tag `I63_OUTCOME_07D`), tallies G1 RECEIPT 1, G5 ATTEMPT 1, G5 PRODUCT_ATTEMPT 2 and G8 INVOCATION 1;
- must-pass asserts 21 entries.

**Against the bar (07d):**
- **Mutations:** all 259 match their expected first gate and code (G7 per reader).
- **must_pass:** all 21 pass with the base case's classifications.
- **Cases:** all 15 validate with their expected classifications.
- **Tables:** `OUTCOMES_07D.json`, with all 259 mutations in corpus order and all 21 must-pass entries.

## Remaining known differences

**None known.** Rust and Python both normalize the receipt only, after G2. Rust checks the value is integral and in range before it converts. The base reader (G7) sees the projected, normalized receipt. G7 per-language codes are by design.

This rests on the shared corpus and targeted comparisons, not an exhaustive comparison. I have not compared TypeScript at its head.

## Files read (sha256)

| sha256 | File |
|---|---|
| 65b21261e9311c01db736da30c1b21a87c1f29366cff03d03044a31654b3df87 | T3/ROOT_RULINGS_V1.md (as at NUM `aca9ad785c`; the grant's sections were read at `f4d5cbbe49`) |
| f9cc84d0bffcc8a59ec03c0577c5b4e74600d2423e46e4766b21d4eb988d735d | R/REVIEW_RV80/reader_confirm_03/REVIEW.md |
| a4fe71870cb466180d03b4d8d820b5ece6835f9c10bdd55db6130eb00e99ad8d | R/I62/review_repair_07/SHARED_SNAPSHOT_07D.json |
| a728bce495a739eb96c38f9b93f3eba5f8b7c466a919144d9708ebfe73f5ad52 | R/I62/review_repair_07/SHA256SUMS |
| b1f4281fd8025c8744b78d9b2812c0f3d157fe84771f433d1225a736a5479c02 | R/I63/review_repair_07/OUTCOMES_07C.json (format) |
| 12da125d9dcfcb55debe2fc1ab1eadcdaf1b35fd188206b251d48f563309e184 | READER/P/fixtures/results/retained_precision_cases.json |
| 07951edacfedd410c153929ee75bb5bada15dbd222369ec63240c678b233b61c | READER/P/schemas/retained_precision_mp_v2.schema.json |
| 96e6ca5562849b9136bc75b665d10b421e75a9b9805303c40b68982101889336 | READER/P/core/serialization/canonical_json/src/lib.rs |
| 8e86e1514a986b376f4a95dac6d4dcb19ec7bd55675812e800798e284a3438d9 | READER/P/core/reporting/result_export/src/source_blocks.rs (`domain_hash`) |
| 031334e29fb150e3bdb63f4b88816679cf83d4243f91db05a94ee9a8a6afa8f3 | READER/P/core/analysis_runs/retained_precision.py (D32 alignment only) |
| 375b07313518a0cdd09e317b83b9ca7536acef5c80fd4cd0a94920fd8bf2486d | READER/P/core/reporting/result_export/src/lib.rs |
| abacc74664d67ce34887cf86481f3ff12090bab2b40689e20e297a9f9fc2c80f / bb713adcebc6c14db1f4c531e54b3da2e7027210b9cf501408c613b0a957ba7d | src/retained_precision.rs, before and after |
| c984f49a54b69d6f08fc86ee0147ecfc8ad733ef9dc366db154ad7d23da38ce4 / 37cf97b4ab5a8924a0f1bba6d35b8efd43fd78c7fa79135990e3791b35b336d8 | tests/retained_precision_contract.rs, before and after |

## Bulk (WT/scratch/i63_review_repair_07d/)

| sha256 | file |
|---|---|
| 9dbfb6a8d536f24ec7c161ea5e0341b0c7422dbcf397fbe4d153d3c53ee7cdc4 | full_test_07d.txt (final full run) |
| 5a06301aec860e181293bf88a8fe6e7310791b8ba786227dea35cca4a082d5ce | outcomes_07d.txt (outcome capture) |
| 83888436fd588fa7c16440700693134fba8fcd9ca3eafcd009881297695029bb | before_reader_on_07d.txt |
| baf8028d6b61c87a0991041f5bed949ddfbcd2a5383cf93b584c94432017c062 | mutant_kills_07d.txt |
| 18df8f5ebe2f220144326cebd22d7ad0d941f3e154aaca61e47446db0bc2677d | mutants.py |
| b7eba1e7486248e57dd6cb7019a20245c0efc91ef33c9d9bfe4ed7b41b743e34 | reader_07d.diff |
| bb713adcebc6c14db1f4c531e54b3da2e7027210b9cf501408c613b0a957ba7d | after_rs.rs |
| abacc74664d6… / c984f49a54b6… | before/ (both files) |
