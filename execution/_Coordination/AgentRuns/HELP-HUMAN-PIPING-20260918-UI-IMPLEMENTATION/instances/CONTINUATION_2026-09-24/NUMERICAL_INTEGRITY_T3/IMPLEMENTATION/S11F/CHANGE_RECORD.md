# S11-F: exact load sums on the product side (change record / PR body draft)

**Branch.** `codex/piping-s11f-20260927`, cut at `origin/main` `3488a236a` (S11-K merged, PR973). The manager merges `origin/main` forward before the PR; the RETURN addendum records the merged head and the fixture identity re-check.

**Basis.**
- S11-F of the selected S11 containment: `T3/DESIGN_NUMERICS/S11_CONTAINMENT.md` revision 5a.2 (`e6507587…`), §4, §4.2–4.5, §5, §6, §8.2, §8.3 and §9.
- `T3/ROOT_SELECTION_DESIGNS.md` and `T3/ROOT_SELECTION_S11.md`.
- Brief: `T3/TASK_BRIEFS/I4_S11F_IMPLEMENTATION.md` with addendum 1 (boundary decisions) and addendum 2 (F12 and the formation list, `db665f2cb`).

**Approvals and rulings cited.** All are in `T3/ROOT_RULINGS_V1.md`:
- ROOT's conditional pre-approval of the S11-F regeneration (`1397e8c43`). The manager checked the pre-regeneration report against it before regeneration.
- ROOT's F12 ruling, option (c) (`db665f2cb`).
- The amendment on formation-class rows (`42b300344`).

**Author.** Type 2 TASK I4 (Claude). This is not owner review. It needs:
- independent review of the full diff (RV3);
- hosted CI;
- a clean DEC-025 sweep.

## What changes

**One exact load ledger per case, built at every product producer (S11 §4.2).** `product_physics` pushes each contribution into a `LoadLedger` as its own term. The producers are:
- nodal loads;
- uniform element loads;
- pressure thrust (straight, curved and expansion-joint);
- thermal equivalent loads (straight and curved, the curved one as the exact product K_rc·fl(ε·chord));
- constant-effort supports;
- the 0.4.0 eigen terms.

The case force is `finish(n)` of that ledger. It is correctly rounded once per degree of freedom, never a binary64 fold.

**The typed seams are live.**
- `solve_load_case` builds the force only from the ledger. `global_load_vector` has no product caller left; its doc note now says it is not for solve input.
- The linear path uses S11-K's typed seams: `reduce_assembled_system*` and `StructuralSystem::assembled`.
- `solve_preview_reduced_system` calls `AssemblyEvidence::solve_assembled(…, &AssembledForce, …)`. The function's shape is unchanged, for K-D5's one-line re-application.
- The nonlinear loop takes the typed force through `solve_active_set_frame_with_mode_and_springs_assembled`, which checks the force bit for bit.
- **Kernel sites.** KS1 (`exact_scaled_rhs` / `prepare_bound`), KS2 (`reduced_right_hand_side`) and KS3 (`evaluate_original_residual_bound`) now receive the ledger's individual terms, not a pre-folded force.

**Exact recovery sums on the facade (S11 §4.4).** Each published sum of three or more terms is now one exact sum rounded once:
- E5 `exact_straight_end_forces`;
- E7, the span intensity in `exact_straight_summary_extrema`;
- E8/E9 `recover_curved_bend_local_forces`, with exact products;
- E10, where `curved_bend_uniform_intensities_by_pipe` keeps one intensity per load instead of a summed intensity;
- E11 `arc_section_resultant_terms`, the section resultant as the exact sum of each load's and each thrust's contribution;
- E12 `restrained_reactions`, as K·u − f with f the exact net;
- E15 `pressure_for_pipe`;
- E16, the expansion-joint aggregate.

The removed binary64 helpers are `corrected_local_forces_for_axial_effects`, `curved_bend_uniform_intensity_by_pipe` and `pressure_thrust_for_pipe`.

**T1's three sites (S11 §4.5).**
- The `source_recovery` fold check is now a ledger compared bit for bit with the solved force.
- `close_load_state` pushes the eigen terms.
- Both `source_receipt` replays (0.4.0 and pre-0.4) rebuild the force through `nodal_and_eigen_case_force`.
- `pressure_runtime` exposes the group operands (`assembled_operands`) so that the pressure producer pushes each operand as a term.
- **One existing T1 unit test changes its expectation, because it asserted the defect.**
  - The test is `source_recovery::tests::source_plan_preserves_absorbed_loads_and_distinct_colocated_springs`.
  - Its fixture force is now built by `nodal_and_eigen_case_force` instead of `global_load_vector`.
  - It asserted that the case force at DOF 9, for loads (1e16, 1, −1e16), was the absorbed 0.0.
  - It now asserts, as a precondition, that the binary64 fold of those loads is 0.0, and then that the case force is the exact net 1.0.
  - Its source-plan assertions (the three terms kept in order, the two co-located springs, the descriptor count) are unchanged.

**The Sensitive mapping (§6, D-S11-4).** When S11-K's load-fidelity audit flags a row, or cannot audit it, the case gets a `LOAD_CONTRIBUTION_ABSORBED` warning. The warning names the case, the rows and their sources, and the case's quality is Sensitive. Nothing is refused. There is no new envelope field and no in-band marker: an unaudited row is named only in the diagnostic's message.

**Carried items (resolved as ROOT accepted in addendum 1):**
- **RV1-N5.** A per-row audit range error becomes an unaudited row: guarded ratio ∞, and `unaudited: Some(reason)` on `LoadFidelityRow`. An audit input error becomes `LoadFidelityReport { rows: [], audit_error }`. Either way the solve is Sensitive and never an `Err`. Audited rows are bit-identical to S11-K's audit, checked against a base signature.
- **K-D5 boundary.** `solve_preview_reduced_system` makes the one typed call. `structural_adapter::solve` (`&[f64]`) is now `pub(crate)`. A lexer scan finds no non-test caller of `AssemblyEvidence::solve` in any crate. The narrowing closes the UFCS route for callers outside `nonlinear_integration`. Inside that crate a `pub(crate)` item is still reachable, so there the behavioural option (c) pins remain the guarantee.
- **RV1-N6.** No rename. `evaluate_original_residual` keeps its name. A new product site-test rule forbids a product call to it, and a behavioural test first shows that the binary64 and exact residual numerators differ.
- **The sparse_direct sibling.** No typed sibling. The typed path goes through `prepare_assembled_structural`, then `factor_structural_ldlt(&PreparedSystem)`, which carries the ledger binding. The site test checks this.
- **RV1-N1.** Both site scans detect self-assignment folds (`x = x ± …`, `x = checked_value(x ± …)`). The header states the scans' limits, and the counts are re-baselined. There are no new unlisted hits.
- **RV1-N8.** Positive twins were added to the two `compile_fail` doctests: `solve_assembled` and `solve_active_set_frame_assembled`.
- **RV1-B-N1.** `::solve(` and `::solve_assembled(` are added to the option (c) pin's exact entry points. A new test scans `structural_adapter.rs` outside the defining variants, and the doc comment states that the behavioural pins are authoritative.
- **Option (c) is unchanged.** The nonlinear loop's closed-gap prescribed solves still reach only the named `_binary64` variants. I1's pins are unedited and green, and DEC-046 is unchanged.

## Which quantities change, and by how much

- **Where bits can move.** The only candidates are sums that the change makes exact:
  - a force DOF with three or more contributions, or a non-trivially ordered pair;
  - straight end forces and span intensities with element loads;
  - curved-bend recovery, section resultants and thermal loads;
  - support reactions (K·u − f);
  - pressure and expansion-joint aggregates;
  - kernel residual rows on prescribed-coupled rows.
- **The bound in the non-cancelling case.** A published value moves by at most the binary64 fold's own rounding error: at most about one rounding of the gross sum. In the committed fixtures, no published result value moves at all, apart from reaction residues of an analytically zero reaction, at ≤ 4.2e-15 of the case's load scale.
- **Where a load was being absorbed** (a gross-to-net ratio of about 1e7 or more in an unfavourable order), the exact net is now published.
  - R1's RF-CANCEL cases meet the binding criterion on both entries and in both modes, except the 14 formation rows.
  - P1's probe A publishes the exact net answer.
- **Status.** No case changes status unless it was absorbing a load. No committed fixture changes status or diagnostic code.
- **No in-band marker** (D-S11-4).

## The exception lists

- **The S11 list is empty.** Every one of the 221 pinned triples in `GATE/S11_EXCEPTIONS.json` (87 captured, 134 typed) is published, checked and inside its interval, on its entry, in both modes (F12).
  - The expected values come from `REFERENCES/references.json` (`7b176dbb…`) through the committed generator `IMPLEMENTATION/S11F/generators/gen_rf_cancel_cases.py`.
  - The G = 1e80 cases run on the typed entry. The captured entry still refuses them at capture, because S-H is not in this slice.
- **The formation list stays exactly pinned** (`GATE/FORMATION_EXCEPTIONS.json`, 7 triples, 14 rows with mode, owned by S11-G and then F2/F3). Measured base against candidate (`_run_records/formation_rows/`):
  - **10 rows are bit-identical or better.** UDL-W1e80 is bit-identical. The four INPLANE triples go from about 1e9× to 628–5767× the criterion.
  - **The 4 UDL-W1e8 th.S1.RZ rows (captured and typed, dense and sparse) are 3% worse:** 46.47× → 47.99× the criterion.
    - Cause: the fixed-end terms carry formation error, and S11-F publishes their exact, correctly rounded net (0.4916666902601719). Base's binary64 fold happened to land closer to the intended 0.49166666666666667.
    - ROOT accepted this under the amended formation-class condition (`42b300344`): the published value is the correctly rounded net of the represented terms, and the row stays exactly pinned.
    - S11-G's load-row guard is required to catch all 4 rows.
  - F1 pins all 14 rows' published bits, so any change to them fails.
- A breach outside both lists fails F1.

## Committed fixture changes (measured, then regenerated by the actual producers)

ROOT's conditional pre-approval covers these (`T3/ROOT_RULINGS_V1.md`, `1397e8c43`). The manager checked `PRE_REGENERATION_REPORT.md` against its conditions before regeneration. Every other committed request is byte-identical in both modes: 108 of the 112 base-against-candidate runs.

| File | Bytes before → after | Changed leaves | What moves |
|---|---|---|---|
| A `fixtures/product_preview/load_reference/connected-dense_scrutiny.raw.json` | 246182 → 246181 | 1 | Debug `StructuralReport` text in NUMERICAL_INTEGRITY_CHECKS_PASSED: case:hot `ResidualRow` global_dof 6, residual −1.42e-10 → −2.00e-10 (normalized −1.36e-16 → −1.91e-16), audit guarded_ratio 9.41e-16 → 9.63e-16 |
| A `…/connected-sparse_interactive.raw.json` | 243663 → 243666 | 1 | as dense: residual 5.19e-11 → −6.31e-12, guarded_ratio 9.07e-16 → 8.91e-16 |
| derived `fixtures/results/load_reference_connected_dense.document.json` | 1452871 → 1452871 | 4 | derivative hash, raw source hash, the two carrier checksums |
| derived `…/load_reference_connected_sparse.document.json` | 1445192 → 1445192 | 4 | as dense |
| derived `…/load_reference_connected_dense.analysis_run.json` | 419728 → 419727 | 3 | the copied diagnostic text, two hashes |
| derived `…/load_reference_connected_sparse.analysis_run.json` | 415863 → 415866 | 3 | as dense |
| derived `…/load_reference_connected_dense.stress_neutral.json` | 883920 → 883920 | 6 | member, package and source checksums |
| derived `…/load_reference_connected_sparse.stress_neutral.json` | 876244 → 876244 | 6 | as dense |
| B `fixtures/results/physics_thermal_ui_mechanics_dense.json` | 169867 → 169867 | 2 | case:closed-pressure, `support:fixture-root` Fx and its force magnitude, an analytically zero reaction: 5.82e-11 → 6.64e-11 N |
| B `…/physics_thermal_ui_mechanics_sparse.json` | 167374 → 167409 | 3 | the same two rows: 0.0 → 8.19e-12 N; plus the pre-existing drift below |

**What the A changes are.**
- KS1 and KS3 now take the ledger's individual terms, so the reported residual on the prescribed-coupled row is the exact numerator of the represented terms minus K·u.
- No published result value, status, diagnostic code or work unit moves.
- The byte changes are only the lengths of three decimal tokens.

**Hash pins, old → new:**
- connected-sparse `89bbc3f6…` → `75f8bf1b6fc53f7c13148bce54099ae9cdd3a2f4c9ae014f48be02e562a864b4`;
- connected-dense `187a6d8d…` → `8413d25c71a62d740614e98e3e86160e99dc76e35f5fdddcb936df80a0f6c0b0`.

They are pinned in `core/reporting/result_export/tests/load_reference_contract.rs` and `tests/test_load_reference_readers.py`.

**What the B change is.** The residue is recovery roundoff of K·u − f, with f the exact net (E12). The largest force in the case is 15708 N, so the residue is at most 4.2e-15 of it. There is no status change.

**Pre-existing drift corrected by regeneration (not an S11-F effect).**
- Where: `physics_thermal_ui_mechanics_sparse.json` line 2709, `results[122]`, the fixture-root force magnitude in case:six-component-load.
- Values: committed 3741.6573867739435 (`0x1.d3b5094ffcdd2p+11`); regenerated 3741.657386773944 (`0x1.d3b5094ffcdd3p+11`), the correctly rounded norm of the exact 3741.6573867739438… The committed value is one ulp off.
- Provenance: the file was last regenerated at `22452ecd1` (2026-09-25), when this norm was the binary64 chain `hypot(hypot(fx, fy), fz)`. Then `1792774a2` switched to `source_receipt::composite_support_norms` without regenerating it.
- Base `3488a236a`'s own producer already emits the corrected value, and S11-F does not change it.

**Producers.**
- `product_physics` examples: `physics_source_connected` (the raws) and `exact_pressure_connected` (the thermal files);
- the `result_export` writer test `load_reference_contract` (documents);
- the Python reader writers (analysis runs);
- T1's `cp3_stress_neutral_outputs.py` (stress-neutral).

**Checks.**
- All 12 regenerated files (the 10 above plus the 2 pin files) are byte-identical to the pre-regeneration measurement (`_run_records/fixture_diff/measurement_sha256.txt`).
- Pinning and consumer tests are green after regeneration; counts are in RETURN.md.
- **Post-restart (2026-09-27).** The container restarted after regeneration, during the Python suite run.
  - The 12 files were re-verified against the measurement and still match, so no producer was re-run.
  - Re-run in full: the result_export pinning and consumer tests, the Python suites, the headless artifact lane, and the full desktop vitest and build. All are green.
  - The first full desktop vitest, before the restart, had one load-sensitive failure: an `App.test.tsx` `findByTestId` wait, at load average 4.6. That test reads none of the regenerated fixtures. It did not recur after the restart (2822/2822). RETURN §7 and §9 give the details.

## Limits and follow-ups

- **S-H is not in this slice.** The captured entry still refuses |x| ≥ 2^53 at capture.
- **The 14 formation rows** belong to S11-G, then F2/F3 (W1a/W1b).
- **E15/E16 on fresh solves.** Fresh solves refuse legacy nonzero pressure (PRESSURE_MODEL_REAUTHOR_REQUIRED), so E15 and E16 are exercised only under `historical_pressure_reference::with_scope` (F10). The manager and ROOT accepted this.
- **N05.** A transverse tip force on T1's N05 exceeds the pre-0.4 retained-replay budget. This is pre-existing on base, in both modes, and routed to T3's N05 item. F4/F6 use the torque DOF.
- **F9's limit.** The 0.4.0 support-motion test checks the displacement answer. A moment check at the prescribed station is not achievable in the displacement representation (W1).
- **Downstream notice.** K-D5 (I3) re-applies its one call onto the typed `solve_assembled` call. No instruction or contract mirror changes.
