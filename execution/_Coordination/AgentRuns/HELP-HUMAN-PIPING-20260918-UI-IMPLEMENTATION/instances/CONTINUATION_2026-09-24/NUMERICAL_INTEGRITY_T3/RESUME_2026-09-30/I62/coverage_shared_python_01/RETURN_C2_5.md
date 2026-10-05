# I62 return: checkpoint C2-5 (snapshot 06d: allocator-refusal triggers, R1–R3)

**Basis:** ROOT's ruling, "Accounting triggers: only allocator refusals are emittable; F and P rebased as snapshot 06d", and ACCOUNTING_CAUSES.md (d562af562d, accepted).

**Run window:** about 2026-10-03T22:27Z to the 22:35:06Z freeze of SHARED_SNAPSHOT_06D, well inside the 2-hour box. The memory guard (PID 5387) was running.

**Limits held:** no Git writes, no Cargo, no native job.

**Native citations** are at CODE/NUM 652ad0cc1f:
- PP = P/core/product_physics/src/retained_product.rs;
- FC = FK/product_certificate/final_case.rs.

## Changed READER files

| File | Before (06c, `c765e4f5b3`) | Now |
|---|---|---|
| P/fixtures/results/retained_precision_cases.json | d4235f59b6… | d02701ed6afb82fdd8d925900119185412c3169276ed281423dff7dd726afdf4 (4633346 B) |
| P/core/analysis_runs/retained_precision.py | 3200f56020… | 55736ea65aee641fb22288d869632c5ea30f8c016ce56307a974ea598826192f (99000 B) |

Unchanged:
- the test files (`test_retained_precision_contract.py` stays at 354821ddf4);
- the schema, results yaml, definition table and semantic fixture.

**Final counts:** 15 cases (2 rebased), 178 mutations (5 new), 18 must-pass entries.
- Must-pass: 7 replaced in place, 1 of them renamed; 4 retired and 1 deferred.

**Python tests:** `c2_5_corpus_06d`, **221 passed, 0 failed**. That is 221 − 5 removed must-pass entries + 5 new mutations. It ran with both BIN variables and a 1,200 s wall, and the input hashes match the final files.

**All entries (Python):**
- all 15 bases pass with their expected classifications;
- 178 of 178 mutations fail at their expected first failure;
- 18 of 18 must-pass entries pass.

Every entry has a single expectation for all readers. The only exception is `g7_maximum_off_enclosure`, whose `expected_by_reader` is unchanged.

## 1. F and P rebased onto allocator refusals

**F′ (`two_case_facade_after_certificate_synthetic`):**
- Case 1 is now `capture{storage{"adapter vector"}}`, with `adapter.fault` null.
- Native path: the certificate passes, then `prepared_verdict_copy` (PP:3522–3525 → 3362–3363) calls `AdapterWork::reserve`. Its `try_reserve_exact` refusal gives `CaptureError::Storage("adapter vector")` (PP:2932–2941). The Observables stage (PP:3529) is never entered.
- **The stage shape is unchanged.**
  - ACCOUNTING_CAUSES wrongly placed F's old trigger at the commit (PP:3548).
  - It was actually the per-verdict MapWrite in the same verdict copy (PP:3364), which is what the base's qualification says.
  - So F′ refuses one step earlier in the same call.

**P′ (`two_case_preparation_failure_synthetic`):**
- Case 1 is now preparation `capture{storage{"prepared vector"}}`, with section null and `adapter.fault` null.
- Native path: after the prelude completes old coverage (PP:3165), the first `prepared_reserve` (PP:3168) is refused at its `try_reserve_exact` (PP:3083–3086). This happens before any helper.
- The F1 row is unchanged: "prelude passed; reserve fails before first helper".

**Each base was rehashed in full, and its qualification text updated.**

**All 33 retained F/P-derived entries were rechecked.** That includes the 17 inherited mutations, the 7 replaced must-pass entries, and `prefix_unattached_old_operand_attested`.
- Every expected outcome is unchanged.
- For every carried-over entry, the first failure is raised at the same reader function and source line as in 06c (asserted).
- No expectation moved, because the stage shapes are unchanged.

## 2. Must-pass replacements (same position; one rename)

| Entry | Was | Now | Native allocator refusal |
|---|---|---|---|
| `cert_failed_before_summary` | proof `work_accounting{overflow}` | proof `{kind:"storage"}` | `certify_final` (FC:1772) → `check_intervals` (FC:1788), while forming coverage (FC:1195): `summary_coverage_data`'s reserve (FC:1374 → FC:443–451 `Cause::Storage`); coverage null |
| `cert_failed_after_summary_storage` (was `…_accounting`) | proof `work_accounting{overflow}` | proof `{kind:"storage"}` | Coverage formed at FC:1195; then `row_scales` (called at FC:1197) is refused at its reserve (FC:1019); coverage retained. **Kept under the ruling's condition.** Renamed because the old id named the accounting trigger. |
| `values_failed_separate_completion` | values `work_accounting{overflow}`, proof "PP abandoned completed values" | values `{kind:"storage"}`, proof **"PP abandoned prepared draft"** | `complete_maxima` `reserve::<bool>` (FC:1643). A values failure returns `projected.abandon()`, whose detail is "PP abandoned prepared draft" (FC:1765). The old detail came from `abandon_values` (FC:1766–1770), so it was not native for this failure. Completion stays `separate_failure`. |
| `maxima_abandoned` | abandoned `accounting{map_write}` | abandoned `storage{"adapter vector"}` | `prepared_maxima` `adapter.reserve` (PP:3394 → 2932–2941) |
| `aliases_abandoned` | same | abandoned `storage{"identity copy"}` | `prepared_alias` `adapter.copy` (PP:3452 → 2953–2962) |
| `bind_rows_abandoned` | same | abandoned `storage{"adapter vector"}` | `bind_rows_view` `adapter.reserve` (PP:1603 → 2932–2941) |
| `prefix_captured` | preparation `accounting{allocation_request}` with fault | preparation `storage{"observation text"}`; the fault edit is removed | An earlier observation refusal: `solver_observations` → `observation_copy` (PP:670, 503–522), kept at PP:693, returned by the prelude (PP:3145–3148) before old coverage completes (PP:3165). This is the `captured_prefix` row. |

**`prefix_unattached_old_operand_attested` is byte-identical.** Rebased on P′, it still passes: there is no PreparedMember, so the edited old operand stays an unattached attestation (F1:101–106).

## 3. Retired or deferred

| Entry | Action | Reason |
|---|---|---|
| `prefix_unequal_helper_new` | retired (**a correction**) | No allocator refusal exists between a helper's return and its new evaluator. PP:3215–3242 only enters adapter accounting and pushes into vectors reserved before the loop (PP:3168–3181). My earlier proposal (`prepared_string`, PP:3096) was wrong: `prepared_string` runs only for loads, before the member loop (PP:3195), and would give the P′ row instead. |
| `prefix_after_new_evaluator` | retired | After the new evaluator, only MapWrite accounting can fail before `preparations.push` (PP:3242–3245; reserved at PP:3175). |
| `observables_failed_after_certificate` | retired | `observables_view` (PP:1822–1900) allocates nothing. Its other causes depend on the data. |
| `g5a_failed_after_certificate` | retired | `g5a` (PP:2522–2640) allocates nothing, and G5aError has no storage variant (C3:276). |
| `prefix_helper_refused` | deferred | SectionError has no storage variant (C3:268). A faithful refusal needs a producer-derived member input. |

**Seven mutations keep accounting context and are byte-identical:**
- `cert_failed_before_summary_g5a_passed`
- `certificate_check_wrong_wrapper`
- `maxima_abandoned_with_coverage`
- `maxima_abandoned_separate_failure`
- `prefix_captured_with_members`
- `refused_member_conversion_kind_bits`
- `prefix_attached_old_input_unbound`

Each pins a P2/P3 association, a G3 coverage or a G8 binding defect that fires before the new P4 rules. Their first failures and raise sites are unchanged. Their unmutated context is not emittable; moving it onto storage is optional.

## 4. Python R1–R3 (G5 WORK_MISMATCH, P4; `_accounting_rules`, every product attempt)

- **R1:** fails if `adapter.fault` is not null, or if any CaptureError/G5aError `{kind:"accounting", event}` cause appears.
  - The overflowing counter's retained prefix is at least 2^62 (PP:2896–2907) and must be emitted (C3:233–236).
  - PP builds each such cause only from the sticky fault (PP:2910–2912, 617, 2527, 2586).
- **R2:** fails if any ScalarTrace has `lost` true (PP:2310–2349).
- **R3:** each `{kind:"work_accounting", fault}` fault (where "both" means overflow plus inconsistent) must be contained in the join of the attempt's emitted unavailable-Count faults and `sticky_status` values (FC:358–379, 1580; FK product_certificate.rs:186–206).

These checks run in the deferred work list, so P2/P3 failures still come first.

## 5. New mutations (5, all on F′)

| Id | Expected | Python | C2-3 reader on 06d |
|---|---|---|---|
| `adapter_fault_present` | G5 WORK | WORK (R1) | pass |
| `accounting_cause_without_fault` | G5 WORK | WORK (R1) | pass |
| `scalar_trace_lost_unavailable` | G5 WORK | WORK (R2) | pass |
| `work_accounting_cause_exact_status` (06c `cert_failed_before_summary` edits) | G5 WORK | WORK (R3) | pass |
| `idle_budget_not_exhausted_no_group` | G5 ATTEMPT | ATTEMPT (exhaustion rule) | ATTEMPT |

- For each R mutation, only the intended rule is false (checked through `_accounting_rules`).
- **The idle sibling discriminates** (in-memory probe on 06d, `idle_sibling_probe_06d.txt`):
  - With the exhaustion clause removed, the reader gives PRODUCT_ATTEMPT for the sibling.
  - The 06b entry still gives ATTEMPT, at the C5 partition.

## Authoritative checklist status (Python, 06d)

These rows change from 06c; all others are as in RETURN_C2_3.

| ID | Python status | Evidence |
|---|---|---|
| N10 | checked; plus a discriminating shared control | adds `idle_budget_not_exhausted_no_group` |
| P6 | checked | `maxima_abandoned_separate_failure`; must-pass `values_failed_separate_completion`, `maxima_abandoned`, `aliases_abandoned`, `bind_rows_abandoned` (now allocator refusals) |
| P7 | checked (attached entries, every attempt) | `prefix_attached_old_input_unbound`; must-pass `prefix_unattached_old_operand_attested` (on P′) |
| P8 | checked | the reason/phase table on F′/P′; must-pass `prefix_captured` (captured_prefix, allocator refusal) |
| P9 | checked; the post-certificate observables/G5a must-pass rows are retired (no allocator trigger) | `certificate_check_wrong_wrapper`; F′ itself (certificate passed, then refused) |
| W1–W3 | checked (existing) | unchanged |
| **R1** | **checked (new)** | `adapter_fault_present`, `accounting_cause_without_fault` |
| **R2** | **checked (new)** | `scalar_trace_lost_unavailable`; plus `product_work_only` (Ready) |
| **R3** | **checked (new)** | `work_accounting_cause_exact_status` |
| Certificate failure prefix | checked | must-pass `cert_failed_before_summary` (coverage null) and `cert_failed_after_summary_storage` (coverage retained) |

## Open for ROOT

- **Rust and TypeScript:**
  - implement R1–R3 at G5 WORK (P4);
  - adopt the two rebased bases, the 7 replaced or renamed must-pass entries, the 5 removals and the 5 new mutations.
- **Faithful witnesses deferred:**
  - a SectionError refusal for `prefix_helper_refused`;
  - optionally, storage contexts for the seven pin mutations that keep accounting context.
