# I63 return: Rust reader on snapshot 07f

I63 is a TASK (Type 2). This return covers ROOT's 07f round and its continuation grant. **Scope:**
- implement D37 in Rust;
- RV80-N1, the D34 test on the transport path (M56);
- adopt 07f.

**Basis:** T3/ROOT_RULINGS_V1.md, from "RV78 final check (confirm 05)" to the end (NUM `c8e97c6df1`), and the continuation grant (NUM `8f90ec3a45`). I63 had no descendants.

**Runs:**
- **The first run** began about 2026-10-04T02:35Z. It handed off at about 02:41Z, before 07f was published, and that hand-off recorded the remaining steps.
- **The continuation** ran from about 02:45Z to 02:48Z, inside its 45-minute box.

**Environment:**
- **Memory guard:** PID 5387 was running.
- **Limits held:** no Git writes or index operations; no install, new tooling or native job; one Cargo job at a time under the 1,200 s wall. I did not touch I64's TypeScript files.
- **Toolchain:** the default, with no `DEVELOPER_DIR`.
- **Heads:** READER was at `63355a91d2` when I started and at `fd76145542` (07f, I62's Python) for the final runs. NUM is at `8f90ec3a45`.
- **Paths** use the brief's placeholders.

**Status:** 07f passes in full. **Not accepted; eligibility still held** (`IMPLEMENTATION_COMPLETE = false`).

## Inputs verified

- **I62's SHA256SUMS:** all 24 lines verify, including SHARED_SNAPSHOT_07F.json (`c26a419f93`) and RETURN_07F.md (`5ef4b15c11`).
- **Corpus:** READER's corpus hashes to `2cae6d6823`, 07f's. The schema (`07951edacf`) and the other shared files are unchanged.
- **Counts:** 15 cases, 268 mutations and 22 must-pass entries.
  - **New:** five mutations at 263..267, all on F′ (`two_case_facade_after_certificate_synthetic`) and all expecting G5 PRODUCT_ATTEMPT:
    - `g5a_error_with_g5a_not_entered` (Y1);
    - `observable_error_with_observables_not_entered` (Y2);
    - `numeric_error_with_checks_not_entered` (Y4);
    - `proof_error_with_certificate_passed`;
    - `values_error_with_values_completed`.

    Y1, Y2 and Y4 are pinned once each.
  - **Unchanged:** everything else from 07e.
- **The native source:** I read F2A/P/core/product_physics/src/retained_product.rs at sha256 `d07383fc02`, the same bytes as I62's cited PP.

## Changed files (READER, inside the fence)

Paths are under P/core/reporting/result_export.

| File | Before (`63355a91d2`) | After |
|---|---|---|
| src/retained_precision.rs | 06204326ac46… | **bf222f79073b7b661563c39c2fa0d23b98b413b83bbd295b13c191046b079961** |
| tests/retained_precision_contract.rs | e895305d11e7… | **f7c648643dd8e29320602d6b22995018f621c49250166c9170b08be3b6f9b4ba** |
| src/lib.rs | 375b073135… | unchanged |

## D37: `error_stages`, reconciled row by row with I62's table

**The change:** `error_stages(a)` replaces the old one-direction P9 check. It is in the same class-3 position, after every attempt's association checks, and gives G5 PRODUCT_ATTEMPT.

**What it relies on:** these existing checks fix the rest of the record:
- **in `g5_stages`:** the not-entered tail after the first stage that did not complete, and observables and G5a entered together;
- **in `g5_products`:** stage ⇔ check, with stages limited to not entered, completed or failed, and a null proof meaning nothing from proof_start on was entered.

**Rows** ("through X" means every stage up to and including X completed):

| I62 row | I62 stage record | `error_stages` | Agrees |
|---|---|---|---|
| preparation | preparation failed, all else not entered | first failed = preparation | yes |
| native | through preparation; native failed | first failed = native | yes |
| capture (a) | through preparation; native failed | first failed = native | yes |
| capture (b) | through native; nothing failed | nothing failed, native completed, proof_start not entered | yes |
| capture (c) | through certificate; observables and g5a not entered | certificate completed, nothing failed; observables and g5a not entered | yes |
| capture (d) | all ten completed | all ten completed | yes |
| proof (a), (b) | proof_start failed, or projection failed | first failed ∈ {proof_start, projection} | yes |
| **proof (c)** | through aliases; certificate failed; observables and g5a both not entered, or both entered and each completed or failed | first failed = certificate. Observables and g5a are left to `g5_stages` (both or neither entered) and the stage ⇔ check rule (each completed or failed). Admitted pairs: (N,N), (C,C), (C,F), (F,C), (F,F) | yes |
| values | through maxima; values failed | first failed = values | yes |
| abandoned (a), (b) | maxima failed, or aliases failed | first failed ∈ {maxima, aliases} | yes |
| **abandoned (c)** | through aliases; nothing failed; certificate not entered | nothing failed, aliases completed, certificate not entered. The not-entered tail and "observables entered ⇒ certificate entered" keep observables and g5a not entered | yes |
| numeric | all ten completed | all ten completed | yes |
| **observable** | through certificate; observables failed; g5a completed or failed | certificate completed, nothing failed in the first eight, observables failed, g5a entered (so completed or failed) | yes |
| g5a | through observables; g5a failed | certificate completed, observables completed, g5a failed | yes |

**`capture` after a failed preparation.** In my first draft, with only the native code and I62's in-progress Python to go on, `capture` was also admitted after a failed preparation. That was carried over from the old P9. I removed it before 07f was published, to match I62's table: a failed preparation is always the `preparation` kind, which carries its capture cause. It is pinned in the D37 test.

**No stop condition.** I62's table reads the native code as I do, for all 16 rows.

**A note outside D37, not acted on.** I62's §1 says, under "Run presence", that `capture` (a) has no Run. But both Rust and Python require a Run whenever the native stage was entered:
- Rust: `g5_products`, "native != not_entered ⇒ run_ref non-null";
- Python: PY:833–836.

So both readers would reject a native-failed `capture` record that has no Run. This is a parity point, not a D37 row, and it affects unavailable paths only. Under D36 I am reporting it rather than changing it. It needs a ruling on what a Run means when the private prepared solve fails before `solve_cases` returns (PP:3279–3285).

## Tests

**`d37_error_kind_agrees_with_stage_record`**, run on F_BASE attempt 1 and P_BASE.

These give G5 PRODUCT:
- `g5a`, `observable` or `numeric` with the checks not entered;
- `proof` with the certificate passed;
- `values` with values completed;
- `abandoned` with the certificate completed;
- `g5a` or `observable` with the checks passed;
- `numeric` or `capture` with G5a failed;
- `g5a` or `numeric` with observables failed;
- `proof`, `numeric`, `g5a` or `capture` after a failed preparation.

These stay clear of PRODUCT_ATTEMPT:
- `capture` after a certificate (the base);
- `numeric` with both checks passed (Y6's shape);
- `capture` at the commit;
- `g5a` with observables passed and G5a failed;
- `observable` with observables failed;
- P_BASE's `preparation`.

**RV80-N1 (M56).** `d34_negative_zero_anywhere_in_receipt_fails_g2` now also checks:
- `validate_transport_metadata` on every −0 case: both enum/const fields, the parsed `-0` text, `directional_springs` and `case_charge`;
- `validate` with no invocation.

A +0 control on the transport path stays clear of G0, G1 and G2 ENCODING.

**Adopting 07f:**
- The slice helper asserts 268 mutations, and the must-pass test asserts 22 entries.
- A new slice, `snapshot_07f_mutation_outcomes` (263..268, tag `I63_OUTCOME_07F`), expects G5 PRODUCT_ATTEMPT 5.

**Mutant checks.** Each mutant ran on a temporary copy of the final bytes, which were then restored (`bf222f7907`).

| Mutant | Result |
|---|---|
| D37 reverted to the old P9 | The D37 test fails: eight inconsistent shapes are admitted. The 07f slice also fails. |
| M56 (transport uses `encoding`, not `g2`) | The D34 test fails. |
| The 07e reader (before-bytes) on the 07f slice | It admits 3 of the 5 pins (`numeric`, `proof`, `values`). It already rejected the `g5a` and `observable` pins, through the check-error relation in the reason table. |

## Commands and results

**The command** is the standing one, run from READER on the default toolchain:

`CARGO_BUILD_JOBS=4 RUST_TEST_THREADS=2 CARGO_NET_OFFLINE=true CARGO_TARGET_DIR=WT/targets/i63-reader/result_export perl -e 'alarm shift; exec @ARGV' 1200 cargo test --locked --offline --manifest-path RE/Cargo.toml --test retained_precision_contract -- --test-threads=2`

| Run | State | Result |
|---|---|---|
| full suite on the 07e corpus (first run) | the first D37 draft | 56 passed, 0 failed |
| D37 test after the `capture` alignment | — | passed |
| **full suite (final)** | final bytes, 07f corpus `2cae6d6823` | **58 passed, 0 failed** |
| outcome capture, `--nocapture snapshot_0 shared_must_pass` | final bytes | 14 passed; 268 outcome lines and 22 must-pass lines |
| mutants and the before-reader | see above | evidence only |

**Against the bar (07f):**
- **Mutations:** all 268 give their expected first gate and code (G7 per reader).
- **must_pass:** all 22 pass with the base case's classifications.
- **Cases:** all 15 validate with their expected classifications.
- **Tables:** `OUTCOMES_07F.json` holds all 268 mutations in corpus order and all 22 must-pass entries.

## Remaining known differences

**None in D37.** It is row-for-row identical to I62's table and Python's `ERROR_STAGE_RECORDS`.

The Run-presence note above is shared by Rust and Python, so it is not a cross-reader difference. I have not compared TypeScript at its head.

## Files read (sha256)

| sha256 | File |
|---|---|
| ce190fb66ff1c72b32abe846b6ec5524bfb3cf2ffa47ac876ee603309432e819 | T3/ROOT_RULINGS_V1.md (as at NUM `8f90ec3a45`; the grant's sections were read at `c8e97c6df1`) |
| 5ef4b15c114e7227a702758fa4d4c40c808489cddab7b064b2c6b1ebf138b8ca | R/I62/review_repair_07/RETURN_07F.md |
| c26a419f93c34d0d5be66567393a3769845406facfeb9820ea20d22729bffbf7 | R/I62/review_repair_07/SHARED_SNAPSHOT_07F.json |
| a97c81996d3c0be41e2901eaf2221b8bca55c130f48cfc88b2067798e5da1c61 | R/I62/review_repair_07/SHA256SUMS |
| d07383fc026e61e494a2b0a307271eaf2eb0329c333da95533f4b39c5d52af1a | F2A/P/core/product_physics/src/retained_product.rs (3130–3300, 3440–3600) |
| 84810876aaae59e168165227bccf7f8ecaed233ac8be3981d4663d9fc9b8eace | F2A/P/core/product_physics/src/retained_receipt.rs (20–70) |
| d77008e24fe1775c9def1e6b838fb535fb3155f1df3f427a227d3d990433228e | READER/P/core/analysis_runs/retained_precision.py (`ERROR_STAGE_RECORDS`, PY:828–840; read in progress, hashed as committed) |
| 2cae6d68231f945e21f883e4b7d4dd7ed0c53e533b7ca7740c2cf402c50fdabe | READER/P/fixtures/results/retained_precision_cases.json |
| 07951edacfedd410c153929ee75bb5bada15dbd222369ec63240c678b233b61c | READER/P/schemas/retained_precision_mp_v2.schema.json (`ProductError`, `Check`, `CaptureError`) |
| 375b07313518a0cdd09e317b83b9ca7536acef5c80fd4cd0a94920fd8bf2486d | READER/P/core/reporting/result_export/src/lib.rs |
| 06204326ac46… / bf222f79073b7b661563c39c2fa0d23b98b413b83bbd295b13c191046b079961 | src/retained_precision.rs, before and after |
| e895305d11e7… / f7c648643dd8e29320602d6b22995018f621c49250166c9170b08be3b6f9b4ba | tests/retained_precision_contract.rs, before and after |

## Bulk (WT/scratch/i63_review_repair_07f/)

| sha256 | file |
|---|---|
| b9d5abdd71c5a8d903cdfad7345f1961c67fe515f4d89724e4fea419605b820d | full_test_07f.txt (final full run) |
| b1361b8fca3e74e55eba1e3d1785d62fd6fe2ca766131eab78bab6f2e4f85843 | outcomes_07f.txt (outcome capture) |
| 412b51c2d6d9cbdc05846226f2a0f6f3112c096c461885bc70ba7227cd9f719f | full_on_07e_d37.txt (first D37 draft on 07e) |
| 31dc944e0af3e062d0db2ac4ad7e6c3af83f75fd1a54b429474054f813686d39 | mutants_07f.txt (first run) |
| 9ac70f6ab7a59953239ef3d255ed152653f03b283d43aeb08c83ac7962ad0b5e | mutants_final_07f.txt (final bytes; the before-reader on the 07f slice) |
| e5971c77065de2f059e14abc9ca776cf8b6a480b65feaf21f64b661f6624f178 | mutants.py |
| 7388af1fdadc39d2704b82d6fc1fa632d0c4743df22f67eeee86bb5aa06987c4 | reader_07f.diff |
| bf222f79073b7b661563c39c2fa0d23b98b413b83bbd295b13c191046b079961 | after_rs.rs |
| 06204326ac46… / e895305d11e7… | before/ (both files) |
