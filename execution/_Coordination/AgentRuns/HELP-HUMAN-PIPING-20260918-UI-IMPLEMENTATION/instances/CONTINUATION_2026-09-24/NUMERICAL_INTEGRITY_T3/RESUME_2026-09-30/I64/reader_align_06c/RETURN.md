# I64 return: TypeScript reader aligned to snapshot 06c

I64 is a TASK (Type 2) under ROOT (HELP_HUMAN, Agent 0). This was its fifth grant, sent by ROOT's mid-run message in the same harness-native subagent session. It had no descendants.

- **Run:** 2026-10-03T22:14:16Z to the final checks at 22:16:48Z; return written about 22:20Z. Well inside the 60-minute box.
- **Host:** the M5 host. The memory guard (PID 5387) was running.
- **Limits held:** no Git writes or index operations, and Git reads used `GIT_OPTIONAL_LOCKS=0`. No install, build, Cargo, solver, native, UI or DEC-025 job.
- **Other authors:** the shared files and I62's and I63's files were not touched. I63's Rust test file showed as modified in READER.
- **Paths** use the brief's placeholders.

## Changed files (READER at c765e4f5b3, inside the fence)

| File | Before (= 90d61a05fc) | After |
|---|---|---|
| P/apps/desktop/src/features/results/retainedPrecision.ts | 66fae3b25a5dcafb9b5e69243998565b7de676fdf6efad590873f71e1055fb8c (100659 B) | 2d0c710073b815aee77e5a7ae70b66d5a358c6ade1ff1442e707057049c5e5a6 (100714 B) |
| P/apps/desktop/src/features/results/retainedPrecision.test.ts | fd6c1c4f8bffa25c011185e71866e37265021c16efb3556a51ad614c1b60a0c7 (26947 B) | 2a00c815c4092cc9fb65a2eab4a72b2d3880a134b591b8cbce9e29c349414f64 (27967 B) |

**Shared files, verified at the start and unchanged at the end:**
- corpus `d4235f59b6c9c1fdc6e346ff085f9c119d5da011f34a68751b8dde186d3338d0`;
- schema `f943ebd351…`;
- definition `3e0779a45a…`;
- table `c74742ce6a…`.

The Python reference is `3200f56020…`.

## Changes

Eligibility stays held (`SUMMARY_COVERAGE_COMPLETE = false`).

1. **WorkAccounting rejection** (C1:66–68, C1:148). `nativeSchedule` now rejects any unresolved/`work_accounting` kernel terminal at G5 ATTEMPT_MISMATCH, idle or not, before C3 association. This removes the 06a acceptance.
   - **N5:** the exact `terminal(stop)` translation stays.
   - **N9:** an escalating last stop with slots left fails.
   - **N8:** a run past the last slot must end on the Ceiling.
   - **N17:** unchanged.
2. **N10 idle rules.**
   - An idle group-null run must be Budget(invocation), with `invocation_before` ≥ Li. The meter-fault alternative is removed.
   - An exhausted meter still forces group null.
   - A run with no attempts in a ready group must be refused `ledger_unavailable` (adaptive.rs:5055–5075).
   - A refused group keeps its own reason (N11).
3. **P7 narrowed.** The 06a binding of every retained old tuple is removed. Old inputs are bound only where a PreparedMember exists, for every attempt (the existing `j < members` loop). Other old entries are attestations.
4. **G8 interpolated material** (lib.rs:9258–9333, Python's rule). Both bracket points must carry elastic modulus, shear modulus and thermal expansion coefficient before interpolation. This is needed for `interpolation_missing_alpha`.
5. **Harness** (test file only).
   - `applyEntry` implements SHARED_SNAPSHOT_06C `format_change` exactly:
     1. apply the source edits;
     2. apply `invocation_edits` to a copy of the base invocation;
     3. if any were applied, set `body.invocation.value` = H(`source_blocks_invocation_v1`, edited invocation), overriding the edits;
     4. rehash per `rehash`;
     5. validate against the edited invocation.
   - Mutations and must-pass entries both use it.
   - The expected outcome is `expected_by_reader.typescript` when present, otherwise `expected`. TypeScript's G7 base code is `SOURCE_PREVIEW_PHYSICS_EVIDENCE_INVALID`, with detail carried separately, unchanged from 06a.
6. **Reader-logic tests updated.**
   - The N5 test now rejects WorkAccounting after an escalating pass stop, and after a structure stop.
   - The N10 test rejects an idle WorkAccounting run.
7. **The renamed id** `ceiling_before_last_slot` comes from the corpus, and no TypeScript code names it.
8. **P5 and O2:** no change. TypeScript already applies P5 to every conversion, including refused preparation members, and keeps its stricter O2 reference rule (diagnostic references must resolve and name the case).

## Commands (from READER/P/apps/desktop)

| Run | Command | Result |
|---|---|---|
| vitest_00 (baseline: the 06a reader on 06c) | `npm test -- src/features/results/retainedPrecision.test.ts --maxWorkers=2` | 270 tests: 264 passed, 6 failed. The failures were `idle_ready_group_not_ledger_refusal`, `idle_work_accounting_run`, `mm_unnormalized_coordinate`, `interpolation_missing_alpha`, `interpolation_duplicate_temperature`, and the must-pass `prefix_unattached_old_operand_attested`. |
| vitest_01 | same | 269/270 (`interpolation_missing_alpha`) |
| vitest_02 | same | 270/270 |
| tsc_03 | `READER/P/node_modules/.bin/tsc --noEmit --pretty false -p tsconfig.json` | exit 0 |
| probe_04 | a temporary probe block (using `applyEntry`) recording the raising line for every mutation; removed, with the test file restored byte-identically | 173 of 173 |
| **vitest_05 (final)** | same as vitest_00 | **270 passed, 0 failed** |
| **tsc_06 (final)** | same as tsc_03 | **exit 0** |

All four shared hashes were identical before and after (final_hashes.txt). No vitest or tsc process remained afterwards.

## Against the bar (06c)

- **All 173 mutations** produce their expected first gate and code. `g7_maximum_off_enclosure` is checked against its TypeScript per-reader expectation. `MUTATION_OUTCOMES.json` has the raising line for each.
  - `work_accounting_after_escalating_stop` and `work_accounting_at_last_slot` are raised by the new WorkAccounting line in `nativeSchedule`.
  - `idle_work_accounting_run` is raised by the N10 group-null idle line in `nativeRuns`. Python raises it at its WorkAccounting line. Both are G5 ATTEMPT, and the N10 line itself admits only Budget(invocation).
  - The seven invocation-edit mutations are raised at G8: the node coordinate binding, and in `selectedMaterial` the three-quantity rule, unique temperatures and the strict bracket.
- **All 23 must-pass entries pass,** including `prefix_unattached_old_operand_attested`.
- **All 15 cases validate.**
- **All earlier tests pass.**
- **tsc** exits 0.

## Checklist status (TypeScript, 06c). Changes since 06a are in bold.

| ID | TypeScript status | Evidence |
|---|---|---|
| N1 | checked | record and attempt counts; pre-schedule shape; N10/N1 reader-logic test |
| N2 | checked | `schedule_fresh_first_p256` |
| N3 | checked | `skip_after_failed_candidate_*` |
| N4 | checked | `failed_verification_*` |
| N5 | checked (exact `terminal()` translation; **no WorkAccounting alternative**) | `terminal_stop_wrong_translation`, `terminal_refusal_wrong_kind`; reader-logic test N5 |
| N6 | checked | p512 ladder; reader-logic test N6 |
| N7 | checked | every selected base; the selected-if-and-only-if-accepted check |
| N8 | checked | `work_accounting_at_last_slot`; reader-logic test N8. The shared Ceiling base is deferred |
| N9 | **checked: no emitted terminal for an escalating end with slots left; WorkAccounting rejected** | `ceiling_before_last_slot`, `escalating_end_not_a_terminal_translation`, `work_accounting_after_escalating_stop` |
| N10 | **checked: group-null idle runs are only Budget(invocation) with before ≥ Li; an idle run in a ready group is refused `ledger_unavailable`** | `idle_budget_below_invocation_limit`, `idle_ready_group_not_ledger_refusal`, `idle_work_accounting_run`; reader-logic test N10/N1. The exhausted shared base is deferred (≥60B) |
| N11 | implemented: a refused group means no attempts and a refused terminal with the group's reason | no native-faithful base |
| N12 | checked by the G1 schema (`work_accounting_prior_not_on_wire`) | — |
| N13 | checked | bases K and V |
| N14 | checked | `corrections_above_three` |
| N15 | checked | `certified_bound_unbound_drop_existing_g5` |
| N16 | checked | the `physical_*` mutations |
| N17 | partly: fragments and sums, chaining, final guard, Budget scope (case wins) | no shared base (deferred, ≥20B/60B) |
| C1 | checked | `failed_build_reason_mismatch`, `cached_failed_slot_rebuilt` |
| C2 | checked | all bases |
| C3 | checked | `failed_slot_not_cached` |
| C4 | checked | `native_work`; call chaining |
| C5 | checked | `group_sources_out_of_order`, `distinct_stiffness_merged_group`; base `two_case_two_groups_synthetic` |
| C6 | checked | all bases |
| O1 | checked (G3) | — |
| O2 | checked (stricter reference rule, kept) | `formation_d5_dangling_diagnostic`, `report_reference_unrelated_diagnostic` |
| O3 | checked | `w2_published_without_initial_failure` |
| O4 | checked | `legacy_source_dangling_diagnostic` |
| O5 | checked (logic) | reader-logic test O5. The shared base is deferred |
| P1 | checked | `rebind_source_run_only` |
| P2 | checked | `stage_entered_after_failure`, `certificate_stage_check_disagree` |
| P3 | checked | `lane_k_failed_with_coverage` |
| P4 | checked (G3 / G5 ruling) | `row_index_*` |
| P5 | checked at G5 for every conversion | `conversion_*`, `refused_member_conversion_kind_bits` |
| P6 | checked | `maxima_abandoned_separate_failure` |
| P7 | **checked: old inputs bound only where a PreparedMember exists; other old entries attested** | `prefix_attached_old_input_unbound`; must-pass `prefix_unattached_old_operand_attested` |
| P8 | checked | the reason/phase table |
| P9 | checked (separate typed pass) | `certificate_check_wrong_wrapper` |
| P10 | checked | the 05c coverage set |
| P11 | checked | `native_stage_disagrees_with_run` |
| W1 | checked | `product_work_only` |
| W2 | checked | lane and conversion counts |
| W3 | checked | — |
| W4 | not publicly checkable (attested) | — |
| G7 | per-reader code (`SOURCE_PREVIEW_PHYSICS_EVIDENCE_INVALID`), detail separate | `g7_maximum_off_enclosure` (`expected_by_reader.typescript`) |
| G8 invocation | **checked: mm normalization, strict bracket, unique temperatures, all three bracket quantities** | the seven invocation-edit mutations; bases I and M |

## Notes for ROOT

- **No expected outcome looks wrong,** and I found no defect in 06c.
- **I62's open product-level question.** Product-level `work_accounting` causes, such as `cert_failed_before_summary`, are untouched in TypeScript. TypeScript accepts them wherever the corpus does.

## Files read in this grant (sha256)

| sha256 | File |
|---|---|
| e78c519779085714a589a2855d3bc999ff90b31a3e0b608bd950854b7d4fad4c | T3/ROOT_RULINGS_V1.md at 133d547b8b (the "Snapshot 06b verified…" and "Snapshot 06c verified…" sections) |
| d7cc260cff250c648c2e53437a63ce160b2519b4979d43b1bfb63ca14667dd39 | R/I62/coverage_shared_python_01/RETURN_C2_2.md (N10 and TypeScript lines) |
| 0d78d88559dfb8086a8e185b6e5102c109ed6f6889e2d190f25a140387c48026 | R/I62/coverage_shared_python_01/RETURN_C2_3.md (sections 1–2 and the 06c summary) |
| 2e82e61861b983ee32a2abf20c12543a792fd2522629cc49ad753d682eab6d52 | R/I62/coverage_shared_python_01/SHARED_SNAPSHOT_06C.json |
| 1ddd8146ed3724bb9cb94403b52955a3a7d8b9dfd884156371fd300e9b3c1932 | R/I62/coverage_shared_python_01/SHARED_SNAPSHOT_06B.json (via the 06c summary and corpus entries only; not opened in full) |
| 3200f560201bff2c3aed7fb5730dd8925610abd3e0df15efd0b22cebd163242e | READER/P/core/analysis_runs/retained_precision.py (`_terminal_of`, `_g5_schedule` head, `selected_material`) |

I also used the corpus at d4235f59b6 and the fenced files at their 90d61a05fc bytes.

## Bulk (WT/scratch/i64_reader_align_06c/)

| sha256 | Bytes | File |
|---|---|---|
| 66fae3b25a5dcafb9b5e69243998565b7de676fdf6efad590873f71e1055fb8c | 100659 | before/retainedPrecision.ts |
| fd6c1c4f8bffa25c011185e71866e37265021c16efb3556a51ad614c1b60a0c7 | 26947 | before/retainedPrecision.test.ts |
| 2d0c710073b815aee77e5a7ae70b66d5a358c6ade1ff1442e707057049c5e5a6 | 100714 | retainedPrecision.after.ts |
| 2a00c815c4092cc9fb65a2eab4a72b2d3880a134b591b8cbce9e29c349414f64 | 27967 | retainedPrecision.test.after.ts |
| a18892343967dab10688b0f06148e224013a73c29981a279577ef2ffbbac0bad | 7168 | i64_06c_reader.diff |
| fc620c1be0d05f2b737e1cce8c7aa14addb8cfb10f1046662906d9e40fb9f5fe | 6507 | i64_06c_test.diff |
| 28d34dab4799b6c20e6ebff02bce44326f7071d16da2114650a208ce4341d0d6 | 24932 | probe_04.txt |
| 84c2d293e9e13b604f46ee571aa88757ad9133211ee65f57d762e6cdbf40b9af | 338 | probe_04.log |
| 05277ebd5b87ee8b4887e666825feddb5911b2e384f4a2a1d0b662cbed4dc582 | 4660 | vitest_00_baseline.log |
| 8890ced36cd07b565c9bd6645f7131d051e3245a9ccdc742ae2e49ac6bfecaa3 | 1973 | vitest_01.log |
| 455cd5dd02c8ffd5f566c62e36361a773427b4fa5f19a33a13307289296f0486 | 440 | vitest_02.log |
| e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855 | 0 | tsc_03.log (exit 0, no output) |
| 9397f24ec82590c24ffcb024a6d10d43efbd6f2917b9cb0bdd2acd45d27f9e37 | 560 | vitest_05_final.log |
| bf99ebeba96a234061547b517e5a2ddeab9361b42cacf2d5bbc17af6bbaad89b | 112 | tsc_06_final.log |
| fd9b75e9c38cc855d0eb350d3597cc981e040ba84def61adf6a0d2c8a886c121 | 237 | final_hashes.txt |
