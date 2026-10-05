# I62 return: checkpoint C2-1 (snapshot 06a and the Python checklist)

**Basis:** the ruling "Reader audit plan approved: one 42-item G5 checklist and snapshot 06 in two grants", including its G7, row-index and synthetic-trigger rulings.

**Run window:** 2026-10-03T21:11:24Z to the SHARED_SNAPSHOT_06A freeze (21:25Z), about 14 minutes of the 3-hour box. The memory guard (PID 5387) was running.

**Limits held:** no Git writes, no Cargo, no native job.

## Changed READER files

| File | Before (05c / C2-0) | Now |
|---|---|---|
| P/fixtures/results/retained_precision_cases.json | 85bff98ea7… | 58562c88dc492b7e71f663332208045484b04a0cdc125574837c5d08a62ddfe8 (3743331 B) |
| P/core/analysis_runs/retained_precision.py | 3b12ca7511… | 5bb6357a3633d2693313ffc80396e4e4015a830800ba2320bc5456b8b28e1d14 (92576 B) |
| P/tests/test_retained_precision_contract.py | df61ed5de4… | 514745e24569ee28585bc0047370c900786d4bd79cacc3a1546cf3071fb7f9d3 (19034 B) |

- **05c is preserved byte for byte:** all 9 cases, 121 mutations and 16 must-pass entries (asserted). The schema, table, yaml and definition are unchanged.
- **No existing expected outcome moved.** The C2-0 reader differs from 06a only on 15 of the new mutations; it already agreed with the other 15.
- **Final counts:** 12 cases, 151 mutations, 16 must-pass entries.

## Snapshot 06a (SHARED_SNAPSHOT_06A.json)

The snapshot JSON lists every new mutation with its checklist ID, expected first failure, Python's raising line and the previous reader's result.

**New synthetic bases:**
- **`candidate_failure_skip_synthetic`:** the p128 shared build fails with Pivot (non-budget, cached); a fresh p256 candidate is accepted and p512 verifies.
- **`verification_failure_skip_synthetic`:** the p256 verification build fails with Condition; the ladder skips two slots to a fresh p512, accepted, with p1024 verifying and floor Φ.
- **`two_case_shared_failed_cache_synthetic`:** two selected cases with identical inputs. Case 1 reuses case 0's cached failed s128 under the same build id. Identical inputs give identical outcomes, so the receipt is consistent.

**New mutations (30), each matching in Python at the intended check:**

| Checklist area | Mutations | Expected first failure |
|---|---|---|
| N3 | 2 | G5 ATTEMPT |
| N4 | 2 | G5 ATTEMPT |
| C1/C3 | 3 | ATTEMPT or WORK |
| N14 | 1 | G5 ATTEMPT |
| C5 | 1 | G5 ATTEMPT |
| O2/O3/O4 | 3 | G5 ATTEMPT |
| P11 | 1 | G5 PRODUCT_ATTEMPT |
| P5 conversions | 5 | four G5 PRODUCT_ATTEMPT; one G2 ENCODING |
| G7 enclosure (bare code) | 1 | G7 `SOURCE_PREVIEW_PHYSICS_EVIDENCE_INVALID` |
| Row index | 4 | three G3 COVERAGE; the Ready subset G5 PRODUCT_ATTEMPT |
| G8 maps | 2 | G8 PREPARATION |
| G8 sourceless prefix | 1 | G8 PREPARATION |
| P6 | 1 | G5 PRODUCT_ATTEMPT |
| P9 | 1 | G5 PRODUCT_ATTEMPT |
| P2 | 2 | G5 PRODUCT_ATTEMPT |

## Tests

Final run `python_schema_G6`: **196 passed, 0 failed.** It ran the brief's command with both BIN variables and a 1,200 s wall, and its input hashes match the final files.

The count is 05c's 164, plus 30 corpus mutations, plus 2 Python-only reader-logic tests:
- the schedule terminal branches (N5, N6, N8 including a positive Ceiling replay, N10);
- the source_decline relation (O5).

## Within-G5 order in Python (C3:304)

1. The native schedule and origin checks. These include the N replay, the C1 cache rules, the budget and idle-run rules (N10/N17) and the C5 groups.
2. Ordinary evidence and source_decline (O2–O5).
3. C3 association and stage legality (P1–P3, P6, P10, P11).
4. Typed check/result pairing (P9).
5. The C3 WORK equations, now deferred until after every attempt's association.

## Authoritative checklist status (Python, 06a). I63 and I64 audit against this table.

| ID | Python status | Evidence |
|---|---|---|
| N1 | checked | record/attempt counts; `_g5_schedule` record contiguity; unit test (idle run) |
| N2 | checked | `schedule_fresh_first_p256` (05a) |
| N3 | checked | `skip_after_failed_candidate_reused`, `_wrong_slot`; base K |
| N4 | checked | `failed_verification_reused_as_candidate`, `_one_slot`; base V |
| N5 | checked | unit test (non-escalating verification-pass failure is terminal) |
| N6 | checked | p512 ladder (05b); unit test (a rejected candidate must hand on its verification) |
| N7 | checked | every selected base |
| N8 | checked (logic) | unit test (positive Ceiling replay and wrong terminal). Shared Ceiling base **deferred** (needs a producer-solved witness) |
| N9 | partly: non-selected terminal kind and non-null reason | The WorkAccounting fault/prior is not publicly derivable (attested). Shared row is plan item 16 (C2-2) |
| N10 | checked (logic): idle shape; runs after charged ≥ Li are idle Budget(invocation) | unit test. Shared base **deferred** (≥60B of work) |
| N11 | implemented: a refused group means no attempts and a refused terminal with that reason | no control; no native-faithful base identified yet |
| N12 | checked by G1 schema; the replay reads the Reason structure | — |
| N13 | checked (replay role/outcome rules) | bases K and V; unit tests |
| N14 | checked | `corrections_above_three` |
| N15 | checked (existing) | `certified_bound_unbound_drop_existing_g5` (05a) |
| N16 | checked (existing) | `physical_*` mutations |
| N17 | partly: selected final guard (existing); Budget(case/invocation) overshoot and idle-after-exhaustion implemented | no shared base (**deferred**, needs ≥20B/60B) |
| C1 | checked | `failed_build_reason_mismatch`, `cached_failed_slot_rebuilt`; base S |
| C2 | checked (existing) | all bases |
| C3 | checked (cache_after equals the derived cache) | `failed_slot_not_cached` |
| C4 | checked (existing) | `native_work`, calls chaining |
| C5 | checked (first-seen stiffness partition, run group) | `group_sources_out_of_order` |
| C6 | checked (existing) | all bases |
| O1 | checked (existing) | — |
| O2 | checked: diagnostic refs resolve | `formation_d5_dangling_diagnostic`. Structural-failure ref implemented, no base |
| O3 | checked | `w2_published_without_initial_failure` |
| O4 | checked | `legacy_source_dangling_diagnostic` |
| O5 | checked (logic) | unit test. Shared base **deferred** (no natural trigger) |
| P1 | checked (existing) | `rebind_source_run_only` (05b) |
| P2 | checked | `stage_entered_after_failure`, `certificate_stage_check_disagree` |
| P3 | checked (existing) | `lane_k_failed_with_coverage` (05a) |
| P4 | checked per ruling | `row_index_*` (4) |
| P5 | checked | `conversion_*` (5). Positive subnormal/underflow rows **deferred** |
| P6 | checked | `maxima_abandoned_separate_failure` |
| P7 | checked, including G8 for sourceless attempts | 05a prefix bases; `prefix_old_inputs_unbound` |
| P8 | checked (existing) | the reason/phase table on bases F/P |
| P9 | checked | `certificate_check_wrong_wrapper` |
| P10 | checked | 05c coverage set |
| P11 | checked | `native_stage_disagrees_with_run` |
| W1 | checked (existing) | `product_work_only` (05a) |
| W2 | checked (existing) | lane and conversion counts |
| W3 | checked (existing) | — |
| W4 | not publicly checkable (attested cumulative adapter prefix) | — |
| G7 ruling | applied (bare code, detail separate) | `g7_maximum_off_enclosure` |

## Deferred (unchanged)

Each of these needs a producer-solved witness or ≥20B/60B of real work:
- the Ceiling;
- L = 0;
- source-construction failure;
- old-Err/new-Ready;
- positive subnormal/underflow rows;
- budget overshoot and exhaustion;
- ordinary W2 and initial failures;
- combinations.

## Open

**Readers (I63/I64)** need these Python changes:
- the N3–N5, N8–N11, N13 replay;
- C1/C3 cache equality;
- C5;
- O2–O5;
- P2, P6, P9, P11;
- the G3 hull-row rule;
- G8 binding for sourceless prefix attempts;
- the G7 bare code;
- the deferred product WORK order.

**C2-2** remains: plan items 4, 5, 11, 14, 15 and 16.
