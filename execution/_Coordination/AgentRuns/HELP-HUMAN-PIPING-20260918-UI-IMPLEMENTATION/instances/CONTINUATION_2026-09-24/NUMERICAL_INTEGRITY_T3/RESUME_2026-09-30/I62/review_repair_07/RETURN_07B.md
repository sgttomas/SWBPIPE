# I62 review repair 07b: rulings D19–D30 (corpus and Python)

**Basis:**
- "Confirmation findings: disposition D19–D26 and the repair round" (NUM `886c1a865c`);
- "RV80 confirmation: disposition D27–D30" (NUM `b76f82289f`).

**Starting state:** READER at `b36739112a`.

**Run window:** 2026-10-04T00:45:26Z (first tool call) to the 00:57:26Z freeze of SHARED_SNAPSHOT_07B, about 12 minutes of the 2-hour box. The memory guard (PID 5387) was running.

**Limits held:** no Git writes, no Cargo, no installs. Writes are within the five-file fence. **No stop condition was reached.**

## D19 native check (done first): passed, no stop

No native path emits an unavailable-case cause other than `receipt_failure` beside a Ready C3 attempt:
- **One Ready exit:** PP/retained_product.rs:3456–3571 (`project_candidate`) produces a Ready attempt only at its single `Ok(PrivatePreparedCandidate)` exit (3555–3567). Every other exit is a `PreparedCandidateRefusal`, which becomes an unavailable attempt.
- **No serializer:** none exists that could attach another cause (PP/retained_receipt.rs:1).
- **Contract:** S06 §1 says "If an encoder/hash/publication fails after a Ready private product, use the existing C2 receipt_failure … Do not mutate Ready into an invented C3 unavailable attempt."
- **Later errors:** a later publication error keeps the legacy terminal, so no successor is emitted (I30 ROUTING §3).

The second half of D19 was therefore built.

## Changed READER files

| File | Before (07a) | Now |
|---|---|---|
| P/core/analysis_runs/retained_precision.py | 9669b88359… | f6ec97fb0938c066febb2c31ee2e4e7e509e60d766fa6b495c09965a28597f4f |
| P/tests/test_retained_precision_contract.py | 2d2ca008c9… | 57257094daaec6531f4d768a00c946deabe180e9876794ea664678a1f24518fd |
| P/fixtures/results/retained_precision_cases.json | a6fa398731… | 729c12574afe5f46f7e37ec3805cf5b50d684100a1ba46465501939df7cb9ce4 |

These are unchanged: the schema (`07951edacf`), the schema test, the definition, the semantic fixture and the results yaml.

## Python repairs

| Decision | Change | Pinned by |
|---|---|---|
| D19 | Per attempt, class 2, G5 PRODUCT_ATTEMPT. An unavailable attempt needs its case unavailable with `prepared_product_failure` naming it. A Ready attempt needs its case selected, or unavailable with `receipt_failure`. D4d then applies by the attempt's error. | Shared: `unavailable_attempt_under_facade_failure_cause`, `…_under_source_error_cause` and `preparation_error_selected_run_under_receipt_cause` (all passed in the 07a reader). Reader-local test for both directions, including the Ready one. |
| D20 | The selected-case attempt reference moved out of the ordinary check into the class-2 case pass (PRODUCT_ATTEMPT). The ordinary pass keeps the D6b quality rule (ATTEMPT). | `selected_case_without_c3_attempt` (07a reader: ATTEMPT) |
| D21 | D5b evidence is `verification_lme` > 0 **or** a verification shared build. | `vbuild_on_escalating_failed_verification` (07a reader: pass) |
| D22 | Already compliant. G3's source-dependent checks are guarded, and the dangling reference reports PRODUCT_ATTEMPT. | `dangling_attempt_source_ref` |
| D23 | Sourced complete old ids must equal the source map's `kernel_member` sequence (G3). | `source_member_map_kernel_id_noncanonical` (07a reader: G5a) |
| D24 | The harness applies `after_rehash` literally after `rehash:"all"`. | Four G1 pins, each reaching its own check: `forged_receipt_hash`, `forged_publication_hash`, `forged_preparation_hash` (the source identity is re-literalized so the preparation check is reached) and `source_identity_stale_receipt_rehashed` (= T4c) |
| D25 | The integral-float rule is dropped. Integrality and range stay. | `test_numbers_are_values_d25` (17.0 is accepted with identical classifications; 17.5 fails G2) |
| D26 | No reader change. | `g5a_sanity_margin_between_2m40_and_2m39` (RV79's vector) kills M21 |
| D27 | Confirmed compliant. Python's idle rule reads the recorded `invocation_before`, and the broken chain is deferred native WORK. | `idle_run_exhausted_meter_chain_broken` → G5 WORK |
| D28 | D5d now covers every attempt reason carrying a quantity (`stop_rule`, `verification_estimate`, `charge`, `publication_enclosure`). | `verification_estimate_…`, `charge_…`, `publication_enclosure_quantity_not_in_layout` (07a reader: pass) |
| D29 | Every CaseSource's body inventory is non-empty (G3), and the coverage-roster check requires a non-empty inventory. | `empty_body_inventory` (07a reader: G5a) |
| D30 | No reader change. | Reader-local `test_native_run_ref_on_nonselected_run_d30` kills the run_ref mutant |

**Targeted mutants** (`mutants_07b.txt`):
- **Killed:** M21, D30, D21, both D19 directions, D28 and D20.
- **The D29 single-site mutant survives**, because the second, coverage-roster guard still catches the pin. The relation is pinned by both sites together.

## Results

- **Full suite:** `pytest -q -rA tests/test_retained_precision_contract.py tests/test_retained_precision_schema.py` gave **347 passed, 0 failed** (`pytest_07b.log`).
- **Every entry, installed 07b** (`PYTHON_OUTCOMES_07B.json`): 15 of 15 bases with expected classifications; 253 of 253 mutations at their expected first failure; 19 of 19 must-pass entries.
- **Staged, then installed:** 07b was staged at WT/scratch/i62_review_repair_07/stage_07b/ and validated in memory (253/253, 19/19) before install. The installed corpus is byte-identical to the stage (`729c12574a`).

**Counts:** 15 cases, 253 mutations (17 new), 19 must-pass entries.

## Deferred-list corrections

- **T4c** is now pinned through `after_rehash` as `source_identity_stale_receipt_rehashed`.
- **N13** isolation needs the Ceiling base (RV78-N2).
- **The Ready direction of D19 and the D30 nonselected native Run** are reader-local only: no faithful shared base.
- Everything else is as in SHARED_SNAPSHOT_07.

**For I63 and I64:**
- SHARED_SNAPSHOT_07B.json is in this folder, and READER's corpus hash is `729c12574a`.
- The new format field is `after_rehash`.
- Expected codes per entry are in the snapshot's `delta`.
