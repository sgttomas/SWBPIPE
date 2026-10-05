# I62 return: checkpoint C2-3 (WorkAccounting rejection, invocation edits, snapshot 06c)

**Basis:** ROOT's C2-3 grant (2026-10-03T21:56:01Z): reject WorkAccounting terminals at G5 ATTEMPT_MISMATCH; add invocation edits to the shared format; align Python to both.

**Run window:** 2026-10-03T21:56:01Z to the 22:10:37Z freeze of SHARED_SNAPSHOT_06C, about 15 minutes of the 90-minute box. The memory guard (PID 5387) was running.

**Limits held:** no Git writes, no Cargo, no native job. Git reads only, with GIT_OPTIONAL_LOCKS=0.

**Native citations** are FK/adaptive.rs and PP/lib.rs (product_physics) at CODE/NUM 652ad0cc1f.

## Changed READER files

| File | Before (06b) | Now |
|---|---|---|
| P/fixtures/results/retained_precision_cases.json | e7983fc641… | d4235f59b6c9c1fdc6e346ff085f9c119d5da011f34a68751b8dde186d3338d0 (4648711 B) |
| P/core/analysis_runs/retained_precision.py | ddf85962c3… | 3200f560201bff2c3aed7fb5730dd8925610abd3e0df15efd0b22cebd163242e (97035 B) |
| P/tests/test_retained_precision_contract.py | 1c14810343… | 354821ddf449b4bdd0304fd16bf5d78e596d71e96358204a30f09fa977a82de3 (16977 B) |

The schema, results yaml, definition table and semantic fixture are unchanged.

**Final counts:** 15 cases, 173 mutations (10 new, 1 renamed), 23 must-pass entries.

**Python tests:** `c2_3_corpus_06c`, **221 passed, 0 failed.** The brief's command ran with both BIN variables and a 1,200 s wall, and the input hashes match the final files. Accounting from 06b's 215: four Python-only invocation tests were removed (moved to the corpus), giving 211, and the ten new mutations bring it to 221.

**All entries (Python, `reasons_06c_current.json`):**
- all 15 bases pass with their expected classifications;
- 173 of 173 mutations fail at their expected first failure;
- 23 of 23 must-pass entries pass.

## 1. WorkAccounting rejection: gate and code confirmed

**Rule:** a `kernel_terminal` of unresolved/`work_accounting` anywhere in a receipt fails **G5 `RETAINED_PRECISION_ATTEMPT_MISMATCH`**. This covers idle runs, ends after an escalating stop, ends at or past the last slot, and terminal-stop translations. The check sits in the native schedule/terminal class (P1). It therefore comes before C3 association (P2) and the work equations (P4).

**Why G5 ATTEMPT:**
- **No emitted receipt can carry it.**
  - C1 WIRE_CONTRACT.md:66 says a checked inconsistency or a max counter "prevents selected successor emission".
  - C1:68 says an unencodable or saturation-ambiguous run abandons successor finalization.
  - Natively, a WorkAccounting terminal exists only when a work status is not exact:
    - an attempt's status, checked before escalation (adaptive.rs:4545–4546);
    - an attempt's status in `finish_terminal` (adaptive.rs:4777–4781);
    - the meter's status at invocation entry (adaptive.rs:4996–4997).
- **The C1 G5 row assigns the code.** C1:148 puts the actual native schedule and selected terminal at G5, with `ATTEMPT_MISMATCH` or `WORK_MISMATCH`. Here the defect is that the terminal lies outside the emitted domain, not that a work equation fails, so the code is ATTEMPT.
- **C2 has no such reason.** The reachable Reason map in C2 CONTRACT_DELTA.md:17–40 has no unresolved/`work_accounting`.
- **C3's G5 row confirms the order and the code.** C3_DELTA.md:304 runs the existing native schedule and origin checks first, and says "Native p/P/stop stay native". So C3's PRODUCT_ATTEMPT code does not apply.
- **Not G1.** The closed schema still admits `{space, tag, fault}`, and C3:261–263 imports WorkAccounting{fault}. That schema tension remains ROOT's record and is not changed here. `prior` stays off the wire (G1, `work_accounting_prior_not_on_wire`).

**Python alignment** (`_g5_schedule`, `_g5_native`):
- A new top-level check rejects any WorkAccounting terminal.
- **Idle runs:** an idle run with group null is accepted only as Budget(invocation) with `invocation_before` ≥ Li. An idle run in a ready group must be refused `ledger_unavailable`. Both rules are kept.
- **N5:** a terminal stop must end on its exact `terminal()` translation (adaptive.rs:4349–4378). The WorkAccounting alternative is removed.
- **N9:** an escalating last stop with slots left always fails. No emitted terminal exists for it.
- **N8:** a run that leaves the loop past the last slot must end on the Ceiling.
- **Exhaustion rule** in `_g5_native`: Budget(invocation) only.
- **N17:** unchanged. Budget(case) requires case_charge > Lc. Budget(invocation) requires case_charge ≤ Lc and after > Li (adaptive.rs:276–286).
- **Unit test:** the N10 test now asserts that an idle WorkAccounting run is rejected.

## 2. Invocation edits in the shared format

A mutation or must-pass entry may carry an optional `invocation_edits` field. An absent field or `[]` means none; every 06b entry has none. It uses the same grammar as `edits`, `{path, op: set|remove, value?}`, applied in order, with paths rooted at the base case's `invocation`.

**Readers apply an entry in five steps:**
1. Copy the base case's `source` and apply `edits` to the copy.
2. Copy the base case's `invocation` and apply `invocation_edits` to that copy.
3. If `invocation_edits` is non-empty, set `source.retained_precision.body.invocation.value` = H(`source_blocks_invocation_v1`, edited invocation).
   - H(domain, payload) = `canonical_sha256_checked_v1({domain, payload})`. This is the digest G8 compares.
   - The value overrides anything written by `edits`.
4. Rehash per `rehash`, unchanged and after step 3. For `all`, recompute in order:
   1. the prepared-attempt preparation sha256;
   2. `source_identity_sha256` of selected cases;
   3. `publication_sha256`;
   4. `receipt_sha256`.
5. Validate the edited source against the edited invocation.

The Python reference is in the test harness, in `apply_entry`, `apply_mutation` and `_apply_edits`. The full text is in SHARED_SNAPSHOT_06C.json under `format_change`.

**These Python-only tests moved to the corpus, and their tests and helpers were removed:**
- `rebind_invocation`;
- the failing half of the mm-normalization test (the passing half is base M);
- `interpolated_control` (it is base I);
- the strict-bracket test at 299, 300, 310 and 311 K;
- the missing-alpha and duplicate-temperature refusals.

## Snapshot 06c (SHARED_SNAPSHOT_06C.json)

**Replaced 06b entry (item 1). Only its id changes; its edits and expectation are byte-identical (asserted):**
- `escalating_end_requires_work_accounting` becomes **`ceiling_before_last_slot`**.
- **Why:** the old id stated the superseded C2-2 rationale. Under this ruling no terminal is emitted there, and the entry still pins that the Ceiling is unavailable before the last slot.

**Everything else is byte-identical (asserted):** all 15 cases, all 23 must-pass entries, and the other 162 mutations.

**New mutations (10):**

| Id | Base | Expected | Python raised at | C2-2 reader |
|---|---|---|---|---|
| `work_accounting_after_escalating_stop` | K | G5 ATTEMPT | `_g5_schedule` WorkAccounting check | PRODUCT_ATTEMPT (accepted the terminal) |
| `idle_work_accounting_run` | F | G5 ATTEMPT | same | PRODUCT_ATTEMPT (accepted the terminal) |
| `work_accounting_at_last_slot` | L | G5 ATTEMPT | same | ATTEMPT (Ceiling-only rule) |
| `mm_unnormalized_coordinate` | M | G8 PREPARATION | `_g8` node map | same |
| `interpolation_missing_alpha` | I | G8 PREPARATION | `selected_material` | same |
| `interpolation_duplicate_temperature` | I | G8 PREPARATION | `selected_material` | same |
| `interpolation_target_below_range` / `_at_lower_point` / `_at_upper_point` / `_above_range` | I | G8 PREPARATION | `selected_material` strict bracket | same |

**The WorkAccounting mutations in native terms:**
- **After an escalating stop:** the K base truncated after its p128 pivot stop, with slots left. These are the same edits as `ceiling_before_last_slot`, except for the terminal.
- **Idle:** F case 1 as an invocation-entry meter-fault return. It has no attempts, zero charges, group null and empty caches, and its source is removed from group 0, because an entry return precedes group lookup (C2:143). Without that last edit, the C2-2 reader failed it at the C5 partition (also ATTEMPT), so it would not have discriminated.
- **At the last slot:** the L ladder's p512 candidate is rejected by its stop rule, and its p1024 verification is only solved. The only emitted terminal there is the Ceiling (adaptive.rs:4762–4766).

**Precedence note:** each of the three keeps a downstream C3 association defect.
- After an escalating stop and at the last slot, case 0 stays selected with a Ready product attempt.
- In the idle run, case 1 keeps its covered proof.

G5 P1 precedence (C3:304; READER_AUDIT_PLAN Part 1) makes ATTEMPT the first failure, as for the four 06b truncation mutations.

**The G8 refusals in native terms** (PP/lib.rs):
- duplicate temperatures: 9219–9230;
- a strict adjacent bracket, blocking at points and beyond the range: 9237–9250;
- E, G and alpha required on both bracket points, with no fallback to the base: 9258–9285.

For the four bracket mutations, the receipt's selector and `target_kelvin` are rebound to the new temperature, so the bracket is the only difference.

**C2-2 reader on 06c (`reasons_06c_c22.json`):**
- 171 of 173 mutations at their expected failure, and 23 of 23 must-pass entries pass.
- Two mutations moved, as intended: `work_accounting_after_escalating_stop` and `idle_work_accounting_run`.

## Not built

- **A WorkAccounting mutation whose only defect is the terminal.** It would need an unavailable case with a consistent unresolved-kernel C3 envelope. No such base exists, and building one is a separate base.
- **A shared exhausted-invocation idle base.** Still deferred: it needs at least 60B of work against the schema-constant limits.

## Observations for ROOT

- **`idle_budget_below_invocation_limit` (06b, unchanged) does not discriminate.** It keeps case 1's source in group 0 with group null. A reader without the exhaustion rule still fails it at the C5 partition, with the same code. A sharpened sibling could be added later.
- **Product-level `work_accounting` causes** in product-attempt proof/values errors (for example, must-pass `cert_failed_before_summary`) are not kernel terminals and are unchanged. Whether C1:66–68 also reaches them is outside this grant.

## Authoritative checklist status (Python, 06c)

| ID | Python status | Evidence |
|---|---|---|
| N1 | checked | record/attempt counts; record contiguity; unit test (idle run) |
| N2 | checked | `schedule_fresh_first_p256` (05a) |
| N3 | checked | `skip_after_failed_candidate_reused`, `_wrong_slot`; base K |
| N4 | checked | `failed_verification_reused_as_candidate`, `_one_slot`; base V |
| N5 | **checked: exact `terminal()` translation only (C2-3)** | `terminal_stop_wrong_translation`, `terminal_refusal_wrong_kind`, `escalating_end_not_a_terminal_translation`; unit test |
| N6 | checked | p512 ladder (05b); unit test |
| N7 | checked | every selected base |
| N8 | **checked (logic): past the last slot, only the Ceiling (C2-3)** | unit test (Ceiling replay); `work_accounting_at_last_slot`. Shared positive Ceiling base deferred (needs a producer-solved witness) |
| N9 | **checked (C2-3): no WorkAccounting terminal anywhere; an escalating end with slots left has no emitted terminal** | `ceiling_before_last_slot`, `escalating_end_not_a_terminal_translation`, `work_accounting_after_escalating_stop`, `work_accounting_at_last_slot` |
| N10 | **checked (C2-3): idle group-null runs are Budget(invocation) with `invocation_before` ≥ Li only; idle WorkAccounting rejected; idle in a ready group is refused `ledger_unavailable`** | `idle_budget_below_invocation_limit`, `idle_ready_group_not_ledger_refusal`, `idle_work_accounting_run`; unit test. Exhausted shared base deferred (≥60B) |
| N11 | implemented; no native-faithful base | — |
| N12 | checked by G1 schema; the replay reads the Reason structure; `prior` off the wire | `work_accounting_prior_not_on_wire` |
| N13 | checked | bases K and V; unit tests |
| N14 | checked | `corrections_above_three` |
| N15 | checked (existing) | `certified_bound_unbound_drop_existing_g5` (05a) |
| N16 | checked (existing) | `physical_*` mutations |
| N17 | checked in code (case-first budget rule); unchanged | no shared base (≥20B/60B) |
| C1 | checked | `failed_build_reason_mismatch`, `cached_failed_slot_rebuilt`; base S |
| C2 | checked (existing) | all bases |
| C3 | checked | `failed_slot_not_cached` |
| C4 | checked (existing) | `native_work`; calls chaining |
| C5 | checked | `group_sources_out_of_order`, `distinct_stiffness_merged_group`; base `two_case_two_groups_synthetic` |
| C6 | checked (existing) | all bases |
| O1 | checked (existing) | — |
| O2 | checked (strict) | `formation_d5_dangling_diagnostic`, `report_reference_unrelated_diagnostic` |
| O3 | checked | `w2_published_without_initial_failure` |
| O4 | checked | `legacy_source_dangling_diagnostic` |
| O5 | checked (logic) | unit test; shared base deferred |
| P1 | checked (existing) | `rebind_source_run_only` (05b) |
| P2 | checked | `stage_entered_after_failure`, `certificate_stage_check_disagree` |
| P3 | checked (existing) | `lane_k_failed_with_coverage` (05a) |
| P4 | checked per ruling | `row_index_*` (4) |
| P5 | checked (every Conversion) | `conversion_*` (06a), `refused_member_conversion_kind_bits`. Positive subnormal/underflow rows deferred |
| P6 | checked | `maxima_abandoned_separate_failure`; must-pass `values_failed_separate_completion`, `aliases_abandoned`, `bind_rows_abandoned` |
| P7 | checked (attached entries, every attempt) | `prefix_attached_old_input_unbound`; must-pass `prefix_unattached_old_operand_attested` |
| P8 | checked (existing) | the reason/phase table on bases F and P |
| P9 | checked | `certificate_check_wrong_wrapper`; must-pass `observables_failed_after_certificate`, `g5a_failed_after_certificate` |
| P10 | checked | 05c coverage set |
| P11 | checked | `native_stage_disagrees_with_run` |
| W1 | checked (existing) | `product_work_only` (05a) |
| W2 | checked (existing) | lane and conversion counts |
| W3 | checked (existing) | — |
| W4 | not publicly checkable (attested cumulative adapter prefix) | — |
| G7 | per-language base codes | `g7_maximum_off_enclosure` `expected_by_reader` |
| G8 material/units | **checked; the invocation-level refusals are now shared (C2-3)** | `named_point_value_mismatch`, `interpolation_target_mismatch`, `mm_unnormalized_coordinate`, `interpolation_missing_alpha`, `interpolation_duplicate_temperature`, `interpolation_target_{below_range,at_lower_point,at_upper_point,above_range}`; bases I and M |

## Open for ROOT

- **Rust and TypeScript:** implement `invocation_edits` per the five steps above; reject WorkAccounting at G5 ATTEMPT; drop the N9/N10 WorkAccounting acceptance; adopt the renamed id.
- **The schema/contract tension** over `work_accounting` in the closed schema remains.
- **Two optional follow-ups:**
  - a sharpened `idle_budget_below_invocation_limit` sibling;
  - a ruling on product-level `work_accounting` causes.
