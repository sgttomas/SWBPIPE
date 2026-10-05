# I63 return: Rust reader, confirmation repair round (D19–D30) and snapshot 07b

I63 is a TASK (Type 2) holding ROOT's standing assignment for the Rust reader in the confirmation repair round. The basis is the rulings "Confirmation findings: disposition D19–D26 and the repair round" (NUM `886c1a865c`) and "RV80 confirmation: disposition D27–D30" (NUM `b76f82289f`). I63 had no descendants. This is the round's single return.

- **Run:** first tool call 2026-10-04T00:45:27Z; done about 01:01Z, inside the 2-hour box. The memory guard (PID 5387) was running.
- **Limits held:** no Git writes or index operations; no install, new tooling or native job.
- **Toolchain:** the default, with no `DEVELOPER_DIR`.
- **Basis files:** READER at `b36739112a` for the rule changes; 07b adopted once I62's SHARED_SNAPSHOT_07B.json (`d5fb202233`) was present and READER's corpus matched it (`729c12574a`). NUM was at `c2934b733d` when hashed.
- **Paths** use the brief's placeholders.
- **No stop condition was reached.**
- **Status:** the 07b bar is met. **Not accepted; eligibility still held** (`IMPLEMENTATION_COMPLETE = false`).

## Changed files (READER, inside the fence)

Paths are under P/core/reporting/result_export.

| File | Before (`b36739112a`) | After |
|---|---|---|
| src/retained_precision.rs | a0973ae5455c… | abacc74664d67ce34887cf86481f3ff12090bab2b40689e20e297a9f9fc2c80f (171474 B) |
| tests/retained_precision_contract.rs | e1ef9da64055… | d800f975dde26bb5d35708181d640a6fbcdcf4716baaa3d9f6b65bfe9111d050 (103163 B) |
| src/lib.rs | 375b073135… | unchanged |

## Decisions: status and pinning test

"Probe" means the test was also run against the pre-round source (`a0973ae5`, plus the test hooks; the new `reason_table` hook as a stub) and failed there as expected. The source was then hash-restored.

| Decision | Status | Pinned by |
|---|---|---|
| D19 converse of D4c, both halves (G5 PRODUCT_ATTEMPT, class 2). I62's native check of the Ready half passed, so it is not held | **new**: an unavailable attempt needs its case unavailable with `prepared_product_failure` naming it; a Ready attempt's case is selected, or unavailable with `receipt_failure`. D4d then applies by the attempt's own error | `d19_converse_cause_binding`: F′ under `receipt_failure` and `unavailable_precondition`; RV79-B1c (a preparation error with a selected Run) under `receipt_failure`; P′ under `unavailable_precondition`; the Ready direction on `two_case_synthetic` (C2 cause → PRODUCT_ATTEMPT; `receipt_failure` passes G5). Probe: admitted before. Shared: `unavailable_attempt_under_facade_failure_cause`, `…_under_source_error_cause`, `preparation_error_selected_run_under_receipt_cause` |
| D20 selected case with no C3 attempt → PRODUCT_ATTEMPT | **new**: split out of the ordinary check (which keeps the rcond label at ATTEMPT) into the class-2 case pass | `d20_selected_without_c3_attempt` (probe: ATTEMPT before); shared `selected_case_without_c3_attempt` |
| D21 a verification shared build is evidence that the pass ran | **confirmed and generalized**. Rust's replay already rejected it when a fresh attempt follows; D5b now requires a null `verification_shared_build_ref` on every escalating failed verification record, alongside no report and `verification_lme` 0 | `d21_verification_shared_build_is_pass_evidence`; shared `vbuild_on_escalating_failed_verification` |
| D22 a dangling attempt `source_ref` → G5 PRODUCT_ATTEMPT, with the source-dependent G3 checks skipped | **new**: Rust gave G3 COVERAGE before | `d22_dangling_attempt_source_ref` (probe: G3 before); shared `dangling_attempt_source_ref` |
| D23 sourced complete old ids = the source map's `kernel_member` sequence (G3) | already compliant | shared `source_member_map_kernel_id_noncanonical` |
| D24 `after_rehash` | **new in the harness**: applied literally after `rehash:"all"`, with no further rehash, per the 07b `format_change` | `d24_after_rehash_edits` (forged receipt and publication hashes give G1; an empty list changes nothing); shared `forged_receipt_hash`, `forged_publication_hash`, `forged_preparation_hash`, `source_identity_stale_receipt_rehashed` |
| D25 numbers are values | confirmed: `uint` reads the parsed value, so 17.0 and 17 are one number; 17.5 fails G2 | `d25_integral_float_is_the_same_number` |
| D26 the M21 vector | no reader change | shared `g5a_sanity_margin_between_2m40_and_2m39` |
| D27 the idle (group-null) Run rule reads the recorded `invocation_before`, not the running meter | **new**: the value is replaced and the condition kept. No other class-1 ATTEMPT check reads a WORK-derived value (audited: the only one used `current`) | `d27_idle_rule_reads_recorded_invocation_before` (RV80 PR14 shape: G5 WORK; probe: ATTEMPT before); shared `idle_run_exhausted_meter_chain_broken` |
| D28 every quantity-bearing reason (`stop_rule`, `verification_estimate`, `charge`, `publication_enclosure`) resolves to a layout row with the same body and kind (ATTEMPT) | **new**: widened from `stop_rule` | `d28_quantity_reasons_resolve_to_layout` (probe: all three admitted before); shared `verification_estimate_…`, `charge_…`, `publication_enclosure_quantity_not_in_layout` |
| D29 an empty body inventory fails G3 | confirmed under a coverage roster, and **generalized** to every CaseSource as Python does it (PY:1590) | `d29_empty_body_inventory` (RV80 PR11); `d29_empty_body_inventory_without_roster` (probe with the general check disabled: G8); shared `empty_body_inventory` |
| D30 the native `run_ref` on a nonselected Run | **new test**: the S06 reason table is factored as `reason_table`, with an internal hook | `d30_native_run_ref_on_nonselected_run`; the M13 mutant (conjunct dropped) fails it |

**Test hooks (D14):** `#[doc(hidden)] reader_logic` gains `reason_table`. It is documented as internal, with no path to eligibility.

## Commands and results

Every run used the 06d command and environment variables, with the default toolchain and no `DEVELOPER_DIR`.

| Run | State | Result |
|---|---|---|
| run1 | rule changes, on 07a | 45 passed |
| run2 (probe) | the new tests against the pre-round source | D19, D20, D22, D27, D28 and D30 failed; D21, D24, D25 and D29 passed (they confirm existing behaviour) |
| run3 (probe) | M13 mutant | `d30` failed (the mutant is killed) |
| run4 | 07b adopted (counts 253 and 19; slice 236..253) | 46 passed |
| run5 | plus the general D29 check | 47 passed |
| run6 (probe) | general D29 check disabled | `d29_empty_body_inventory_without_roster` failed (G8) |
| run7 (**final, full command**) | final bytes | **47 passed, 0 failed** |
| run8 | `--nocapture snapshot_0 shared_must_pass` | outcomes captured |

**Against the bar (07b):**
- **Mutations:** all 253 match their expected first gate and code (G7 per reader).
- **must_pass:** all 19 pass with the base case's classifications.
- **Cases:** all 15 validate with their expected classifications.
- **Every other test passes.**
- **Tables:** `OUTCOMES_07B.json`, all 253 mutations in corpus order and all 19 must-pass entries.

## Remaining known differences

**From Python `f6ec97fb09`,** by comparing the D19–D30 code paths:
1. **D5b, a verification report.** Rust also treats a verification report (`record.verification` non-null) on an escalating failed verification as evidence that the pass ran. That was phase-1 D5b ("no report"). Python's evidence is `verification_lme` or a verification shared build. So an escalating failed verification that carries a report, with `verification_lme` 0 and no build, is G5 ATTEMPT in Rust and admitted in Python. No entry exercises it, and natively a solve failure returns before the pass produces a report (adaptive.rs:4556–4590).
2. **Otherwise none known.** This comparison is not exhaustive. G7 per-language codes and the fallback codes are by design.

**From TypeScript:** not compared at its head; I64 reports its own 07b outcome.

## Files read (sha256)

| sha256 | File |
|---|---|
| 79044a9a9bb0cbf78c2422f4f7ec35aa05123fca37a88b49de577266de1340b0 | T3/ROOT_RULINGS_V1.md at NUM `c2934b733d` (D19–D26, D27–D30) |
| d5fb2022338d5bfb8542765ce8e8e797563bd0bf140ff553f0fcfca5305b6488 | R/I62/review_repair_07/SHARED_SNAPSHOT_07B.json |
| ddf4e17b58a79dab3ff4c157e92a0126c9cc3c9ee708aee337d2b1b272e518d9 | R/I62/review_repair_07/RETURN_07B.md |
| be090c5d640de60577b8c207eb501a7295bd055cbb51cc9234431c6151608cbd | R/REVIEW_RV79/reader_confirm_02/REVIEW.md (C1, C2) |
| 28f2449eadc6c35931283d7b128e86b9400faab1de0712b152dec9f45a27c80d | R/REVIEW_RV80/reader_confirm_02/REVIEW.md (C1–C4; PROBES.json PR11, PR12, PR14) |
| f6ec97fb0938c066… | READER/P/core/analysis_runs/retained_precision.py (D29 placement; read only) |

## Bulk (WT/scratch/i63_review_repair_07b/)

| sha256 | bytes | file |
|---|---|---|
| 0a1199082d90bce568b6d5af69080efde6dc9bb3a6aa213bb82b20bf93976c6b | 12386 | I63_07B_DELTA_src.diff |
| 048ce1460efdaab826bbb57188c92f73add35dd9885e8d567c64fe5723da32cc | 12698 | I63_07B_DELTA_test.diff |
| 50d94380cea41744696a650ea5d043836f4169fa9b757f1a72a37703884a7ee2 | 3116 | run1.log |
| 0673ceca7c9e3355f35aab69f1c50f06b05aa7107f221d27453c4bf8d436f7d7 | 3862 | run2_probe_pre_round.log |
| 61e6792c698e263d453829d131708603e98c312fd84348ae4b77e16d718e936f | 1449 | run3_probe_m13.log |
| 2250ba852936fa574a1713ccdb2fad6eaed2242d405f27253a22cd14e53af091 | 3159 | run4.log |
| b53b11e960f24bb4537cef505e34812693a5ad4baba966245fd28d6d39f1dcda | 3211 | run5.log |
| f00cd20d8a0fe77280d455be8297b33b9894dbfa5343ce5c298d8f2a3a541780 | 1667 | run6_probe_d29.log |
| 8f3c934d2e7f86efbcdb261c8d3dd5d9348d129edabeab13ead82019cfb52f08 | 3211 | run7_final.log (final) |
| ddf3a3e9067070525a5c0078a938b9845f814680d988eeaeba274a35d1cf8bcc | 65516 | run8_outcomes.log |
| a0973ae5455ce15a12d4717b49d4af1d3796a24e3023c7d88c0788d02ecf99de | 169071 | before/retained_precision.rs |
| e1ef9da64055dc088b0fbc2d0d21c4346d42152c0153f0c0def0b9def90a7d77 | 92404 | before/retained_precision_contract.rs |
| 375b07313518a0cdd09e317b83b9ca7536acef5c80fd4cd0a94920fd8bf2486d | 66051 | before/lib.rs |
| abacc74664d67ce34887cf86481f3ff12090bab2b40689e20e297a9f9fc2c80f | 171474 | final/retained_precision.rs |
| d800f975dde26bb5d35708181d640a6fbcdcf4716baaa3d9f6b65bfe9111d050 | 103163 | final/retained_precision_contract.rs |

The run*.exit and run*_start.txt files hold the exit codes and UTC bounds.
