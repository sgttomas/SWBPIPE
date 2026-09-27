# RV3: independent complete-diff review of slice S11-F

**Verdict: PASS.** There are no BLOCKING findings. There is 1 SHOULD-FIX and 9 NOTEs.

**Reviewer.** Type 2 TASK RV3 (Claude), role TASK. The brief is `T3/TASK_BRIEFS/RV3_S11F_REVIEW.md` with Addendum 1, read with `_COMMON.md`. I am independent: I did not design S11, implement S11-K or S11-F, review S11-K, or advise on any of them. This is not owner review. I fixed nothing, and I made no Git write except adding and then removing my own detached scratch worktree.

## 0. Revisions reviewed

| Item | Value |
|---|---|
| Candidate | PR #1000, branch `codex/piping-s11f-20260927`, head `11ae0667f1f1ef96e7516af0258cefa9d38f8db2` |
| Chain | `9398142e4` (I4's S11-F) → `e46a62f79` (merge of main) → `8cbd4785d` (records addendum) → `11ae0667f` (merge of origin/main `f0a6159c9`, no change under `projects/chirality-piping`) |
| Merge base with origin/main | `f0a6159c9440557d18a728416166cc1e3e0c862d` |
| Diff reviewed | `git diff f0a6159c9..11ae0667f`: 157 files, +38267 −361 |
| Basis records | `<wt>/numerics` at `16517523b` |
| Design basis | `DESIGN_NUMERICS/S11_CONTAINMENT.md` sha256 `e6507587f3e97f6f28ecea213d5a21636235c66822c69e967c283248a3272ef4` (revision 5a.2, verified) |

The brief names a head in Addendum 1. The manager's spawn message supersedes it with `11ae0667f`, and that is the head reviewed here.

## 1. Findings

| ID | Severity | Site | Evidence | Resolution |
|---|---|---|---|---|
| RV3-S1 | **SHOULD-FIX** | The S11 §6 / §8.2 Sensitive mapping: `PP` `append_load_contribution_absorbed` (`lib.rs:1075`) and its call site (`lib.rs:2796-2798`) | Mutant EV5 deletes the call. **It survives every product_physics test:** 348 lib tests plus the 9 site tests pass (`mutations/results.json`, `EV5-no-sensitive-warning`). No test in the repository asserts `LOAD_CONTRIBUTION_ABSORBED`: the two F2 references only assert its absence. The unaudited-row and `audit_error` branches of the message (RV1-N5 at facade level) are also never executed by a test. The kernel side (Sensitive quality, the unaudited row, the audit error) is tested in `frame_kernel::structural::s11f_tests`. The facade diagnostic is not. Through the typed seam the audit cannot flag (an `AssembledForce` always holds the correctly rounded net of its own terms), so this defence-in-depth path is unreachable without a test hook. That is why it needs a direct test. | Add a PP unit test that calls `append_load_contribution_absorbed` with synthetic `LoadFidelityReport`s: one audited flagged row, one `unaudited: Some(..)` row, and `rows: []` with `audit_error: Some(..)`. Assert: code `LOAD_CONTRIBUTION_ABSORBED`; severity `warning`; id `diagnostic:load-fidelity:<case>`; `affected_refs` = [case, sorted and deduplicated sources]; the message names each row and the unaudited reason or audit error; no blocking diagnostic. Preferably also add one end-to-end test through `solve_load_case`, using a test-only injected report: the case is published `NUMERICAL_INTEGRITY_SENSITIVE` with the warning, and is never refused. The test must first show that EV5 fails it. |
| RV3-N1 | NOTE | The line-2709 explanation (`CHANGE_RECORD.md`, `PRE_REGENERATION_REPORT.md` §3.2) | **The conclusion is verified.** The base producer (main `f0a6159c9`) reproduces the base-committed dense thermal file byte for byte. It reproduces the sparse file except line 2709, where it already emits `3741.657386773944`. S11-F does not touch that row (`producers/producer_comparison.txt`). **The stated mechanism does not reproduce.** At 22452ecd1, the chain `hypot(hypot(fx, fy), fz)` over the published components (−1000, −2000.0000000000018, 3000.000000000002) gives `3741.657386773944` with glibc 2.39 libm, not the committed `…9435`. 22452ecd1's `scaled_norm` gives the same value (`line2709/norm_check.out.txt`). So the committed value came from different operands, or from another platform's libm, not from that chain over these components. | Correct the record's wording to: "fixture stale since 22452ecd1; the code changed at 1792774a2; the base producer emits the correctly rounded value; the exact 22452ecd1 mechanism is not reproduced on this host". No product change. The separate disclosure required by ROOT's condition B is in place. |
| RV3-N2 | NOTE | `IMPLEMENTATION/S11F/_run_records/callers/callers.json` and RETURN §3 | I4's caller list was not re-run on the final tree. Its `prepare_sources` test lines (1614…) predate the T1-test edit; at `9398142e4` they are 1617…. It classes `membrane_publication_range.rs:187` as non-test, but that module is `#[cfg(test)]` (`lib.rs:94-95`). It omits `load_state_eigen_loads`, `build_thermal_element_loads` (both changed to fill the new `ThermalElementLoad.source`) and `source_recovery::Sources::system`. My independent list (§3) finds no non-test caller that I4 missed, and nothing of consequence. | Record correction only. |
| RV3-N3 | NOTE | Text-pin limits (site test rules 1, 5 and 8; the header already states the limits) | Four evasions pass the whole site test: EV1 (a function-pointer alias of `global_load_vector` inside the listed producer `push_nodal_loads`), EV2 (E7 folded with `Iterator::reduce`), EV4 (E16 folded by a helper in the unscanned `pressure_sum.rs`) and EV6 (E12 likewise). **All four are killed behaviourally** (§7). The behavioural tests carry the guarantee, as the header says. | Optional hardening: have the scans also flag `.reduce(`, and `global_load_vector` not followed by `(`. No action is required. |
| RV3-N4 | NOTE | The changed T1 test `source_plan_preserves_absorbed_loads_and_distinct_colocated_springs` | The precondition folds the literal `[1e16, 1.0, -1e16]`, not the fixture's own fold. It is still meaningful: those are exactly the authored loads in authored order, and on the old path `force[9]` was `global_load_vector`'s 0.0, so the new `== 1.0` fails there. Nothing is weakened (§8). | Optional: also assert `fixture.loads.global_load_vector(n)[9] == 0.0`. |
| RV3-N5 | NOTE | Preconditions of F3 (0.4.0), F5 and F9 | These compute the fold from test-side closed forms (`hot_eigen_axial`, `6EI/L²·g`), not from the product's own ledger terms or kernel rows. They remain meaningful, because the matching mutants are killed (M9 by F5 in my run; M12 by F9 in I4's). | Optional: take the operands from `case_ledger` or the reduced row. |
| RV3-N6 | NOTE | `nonlinear_integration::s11k_tests::option_c_structural_adapter_legacy_variants_reach_only_binary64_entry_points` | The scan replaces `.solve(` with `self.solve(`. A `.solve(` on another binding inside the adapter (for example `let e = self; e.solve(..)`) would pass the text scan. The doc comment already says the behavioural pins are authoritative. | None required. |
| RV3-N7 | NOTE | `PP::finish_case_ledger` | Every `LedgerError` (`EmptySource`, `DofOutOfRange`, `Sum`) is mapped to `NonFiniteInput { name: "computed mechanics", value: inf }`. The first two are unreachable: empty ids are blocked by `validate_ids`/`detect_empty_ids`, and every producer DOF is in range by construction. For a non-finite term, only the reported value changes (inf where the old fold gave the actual value). No committed fixture exercises this path. | None required. |
| RV3-N8 | NOTE | Scope of the Sensitive mapping | The mapping applies only on the linear path (`built.nonlinear_supports.is_empty()`). The nonlinear loop has no load audit. This is consistent with S11 §10 item 1 (T5) and with option (c). | Record only. |
| RV3-N9 | NOTE | Hygiene | `git diff --check` reports "new blank line at EOF" in 3 committed run-record logs (`fixture_diff/logs/cand_consumers_rust.log`, `cand_headless_lane.log`, `cand_result_export_writers.log`). RETURN §8 says the check is clean; that holds for the product files only. | Trim the logs or record the exception. |

## 2. Check 1: §8.2 write set and behaviour

- **The ledger at every §4.2 producer.** Nodal loads (`push_nodal_loads`); straight and curved uniform equivalents, one term per (load, DOF); straight thrust fl(P·x_a); curved caps fl(P·t_a) and one term per wall slot; straight thermal and T1 eigen pairs; curved thermal as `push_product(K_rc, fl(ε·chord_c))` per nonzero column; exact-pressure group operands (`pressure_runtime::assembled_operands`, one per group, never `assembled_loads`); constant effort, one term per application. Every one pushes into one `LoadLedger` in `case_force_ledger`. Rule 5's producer list matches exactly.
- **The force is built only from the ledger.** `solve_load_case` builds `force = finish_case_ledger(...)`. `global_load_vector` has no product caller (§3). The observation lanes read `values()`, are allow-listed, and never reach a solve.
- **E-sites.** Each was read against S11 §4.4:
  - E5 (`exact_straight_end_forces`): local_i − each per-load E1 term ± each thermal and thrust axial_load on the UX rows, rounded once.
  - E7: per-axis exact intensity.
  - E8/E9 (`recover_curved_bend_local_forces`): the exact products K·d − K·u_free(ε_l) per thermal load, − each per-load uniform equivalent, − each thrust's radial equivalent.
  - E10: removed. One intensity per load, the same intensity vector the force side uses.
  - E11: `arc_section_resultant_terms`.
  - E12 (`restrained_reactions`): K·u − each ledger term, via `accumulate_dof(.., negate = true)`.
  - E15 and E16: exact sums.
  - `pressure_thrust_active` now uses the exact net's nonzeroness.
  - The force side and the recovery side see the same represented terms.
- **The Sensitive mapping.** It is present and correct in code: `LOAD_CONTRIBUTION_ABSORBED`, a warning, refs = case plus sources, and an unaudited row or audit error named in the message only. The Sensitive quality comes from FK. **It is untested at the facade** (RV3-S1).
- **T1's three sites.** `source_recovery::prepare_sources` and `close_load_state` push into a ledger, whose `finish()` bits are compared with `input.force.values()`. Both receipt replays use `nodal_and_eigen_case_force`. `pressure_runtime` exposes `assembled_operands`.
- **0.4.0 prescribed motion.** `reduce_assembled_system_with_prescribed_displacements`, then `solve_assembled` with `StructuralSystem::assembled`, giving KS1–KS3 exact. F9 and N6 exercise it.
- **Removal of the `&[f64]` product entry points.** SA `solve` is `pub(crate)`, and its only callers are s11k tests. The FK `&[f64]` reductions remain for non-product callers (`linear_supports`, `performance_harness`, the mechanics benchmark; §3). The product is barred from them by `FORBIDDEN_PRODUCT_CALLS`.
- **Option (c).** The loop still reaches only `reduce_system_with_prescribed_displacements_binary64` and `solve_binary64` (`nonlinear_integration/src/lib.rs:1977`, `:1990`). I1's pins are unedited, and all 5 option (c) tests pass.
- **Caller list.** Re-derived independently (§3), and compared with I4's (RV3-N2).

## 3. Independent caller list (lexer scan)

`_run_records/s11f_review/callers/`. `rv3_callers.py.txt` was written from scratch; it does not reuse I4's `scan.py`. It maps every changed line of every changed `.rs` file to its enclosing `fn` on both sides, then scans every `.rs` file in the head tree (outside `execution/`). It lexes comments and literals out, and classes a call as test when it sits in `#[cfg(test)]`, `#[test]`, `tests/` or `*_tests.rs`. `rv3_callers_seams.json` covers the kernel seams.

**Non-test callers of the changed non-test functions** (lines at `11ae0667f`):

| Function | Non-test callers |
|---|---|
| `push_nodal_loads` | 2: `case_force_ledger`, `nodal_and_eigen_case_force` |
| `case_force_ledger`, `finish_case_ledger`, `restrained_reactions`, `exact_straight_end_forces` | 1 each: `solve_load_case` |
| `curved_bend_uniform_intensities_by_pipe`, `pressure_thrusts_for_pipe`, `recover_curved_bend_local_forces`, `curved_bend_station_resultants`, `pressure_for_pipe`, `append_expansion_joint_pressure_thrust_results`, `append_load_contribution_absorbed`, `legacy_observation_force`, `legacy_dense_observation`, `append_nonlinear_support_loop_results`, `solve_preview_reduced_system` | 1 each: `solve_load_case` |
| `nodal_and_eigen_case_force` | 2: `source_receipt.rs` (0.4.0 and pre-0.4 replays) |
| `add_uniform_element_loads`, `add_pressure_thrust_loads`, `add_constant_effort_support_loads`, `push_exact_pressure_operands` | 1 each: `case_force_ledger` |
| `add_thermal_equivalent_loads` | 2: `case_force_ledger`, `nodal_and_eigen_case_force` |
| `add_curved_bend_pressure_thrust_load`, `add_curved_bend_thermal_equivalent_load` | 1 each: their straight dispatchers |
| `curved_bend_section_resultants` | 2 |
| `exact_straight_summary_extrema` | 2 (`lib.rs:3318`, `:3496`) plus 1 test (`membrane_publication_range.rs:187`, test-only module) |
| `solve_load_case` | 3 |
| `prepare_sources` | 3 |
| `close_load_state`, `check_load_state_input`, `finish_source_groups` | 1 each |
| `check_input_with_physical` | 2 |
| `load_state_eigen_loads` | 2 |
| `build_thermal_element_loads` | 1 |
| `audit_load_fidelity`, `audit_load_row`, `unaudited_row` | 1 each (FK) |
| `row_sources` | 2 (FK) |
| `finish_checked_factor` | 1 (FK) |
| Removed `corrected_local_forces_for_axial_effects`, `curved_bend_uniform_intensity_by_pipe`, `pressure_thrust_for_pipe` | 0 |

**Seams:**

| Seam | Non-test callers |
|---|---|
| `AssemblyEvidence::solve` (method or path, AssemblyEvidence receivers) | **0 in any crate** (5 test calls) |
| `solve_assembled` | 1: `PP:4177` `solve_preview_reduced_system` |
| `solve_binary64` | 1: `nonlinear_integration/src/lib.rs:1990` |
| `global_load_vector` | 5, all in `validation/benchmarks/mechanics` |
| `evaluate_original_residual` | 1: `product_equilibrium.rs:56` (option (c)) |
| `solve_active_set_frame_with_mode_and_springs_assembled` | 1: `PP:3846` |
| `reduce_assembled_system*` | 1 each: `PP:2543`, `:2550` |
| `reduce_system` | 13 (performance_harness, mechanics benchmark) |
| `reduce_system_with_prescribed_displacements` | 2 (`linear_supports:467`, `:487`) |
| `arc_section_resultant_terms` | 1: `PP:9226` |
| `equivalent_nodal_load_terms_with_spans` | `PP:3094` plus SP internals |

**Comparison with I4.** The sets are identical, except:
- the FK lines are +1, from the merge's `mod retained;`;
- the stale test lines and the membrane classification (RV3-N2);
- I4's four extra-name entries (`solve`, `solve_assembled`, `global_load_vector`, `evaluate_original_residual`), which agree with the seam scan above.

## 4. Check 2: F12 and the gate re-run

- **The generator.** I re-ran `IMPLEMENTATION/S11F/generators/gen_rf_cancel_cases.py` from the candidate, on the numerics worktree's inputs:
  - `REFERENCES/references.json` sha256 `7b176dbb…` (equal to `c0f14201c`'s blob);
  - `GATE/S11_EXCEPTIONS.json` `1d8979f6…`;
  - `GATE/FORMATION_EXCEPTIONS.json` `454bbc24…`.
  - **The output is byte-identical to the committed `rf_cancel_cases.json`** (sha256 `b1cc9266…`, `generator/reproduction.txt`).
  - The generator uses the column-3 "recommended_scale (net-governed)", which is the binding column. Its intervals are derived exactly from `Fraction` and rounded inward; Mb uses the squared interval.
  - F1 asserts all three input hashes.
- **The gate harness.** `f1_f11_f12_rf_cancel_cases_meet_the_binding_predicate_on_both_entries` reads both pinned files by hash. I re-ran it: **pass** (product_physics full suite, `suites/product_physics_full.summary.txt`).
  - It asserts that every one of the 221 triples (87 captured, 134 typed) is published and checked on its entry in both modes, and is not a breach. **The S11 list is empty.**
  - Any breach outside the 14 formation rows fails the test.
  - The 14 rows equal `FORMATION_PINS` bit for bit.
  - The G = 1e80 cases run typed, and the captured entry is asserted to refuse them.

## 5. Check 3: formation rows

- **UDL-W1e8, recomputed in `Fraction`.** Scratch probe `formation/probe_udl_w1e8.txt`, dumped from the product's own `case_ledger`:
  - The represented terms at S1.RZ are 0.2 (`3fc999999999999a`), −33333333.333333325 (`c17fca0555555553`) and 33333333.625000015 (`417fca055a000004`).
  - The correctly rounded net is **0.4916666902601719**, which is exactly the case force (`3fdf777790cccccd`).
  - The binary64 fold in push order is 0.49166668951511383.
  - All four rows (captured and typed, dense and sparse) publish `2.2754051790793294e-08`, the `FORMATION_PINS` value.
  - The exact ratio is 47.9868×. Base's fold/k is 2.2754051756312433e-08, at 46.4714×.
  - This confirms "4 rows 3% worse, the published value is the correctly rounded net of the represented terms".
- **The other 10 rows.** They are exactly pinned by F1, and I4's table shows them bit-identical (UDL-W1e80) or better (INPLANE). I did not rebuild base for the RF-CANCEL INPLANE rows (§13).
- The disclosure is in CHANGE_RECORD, as the amendment `42b300344` requires.

## 6. Check 4: regeneration, on the committed bytes

- **The 12 committed files equal `fixture_diff/measurement_sha256.txt`** (`sha256sum -c`: all OK).
- **The hash pins.** The old values `89bbc3f6…` and `187a6d8d…` are the sha256 of the base raws. The new values `75f8bf1b…` and `8413d25c…` are the sha256 of the committed raws. Both pinning tests are green: `frozen_inputs_table_and_schema_are_pinned` ok; `test_load_reference_readers.py` 239 passed, 1 skipped.
- **Producers re-run by RV3** (release, 1.97.1; `producers/`):
  - Candidate `physics_source_connected` and `exact_pressure_connected`, both modes: **all 4 outputs byte-identical to the committed files.**
  - Base (main `f0a6159c9`): the raws and the dense thermal file are byte-identical to base-committed. The sparse thermal file differs only at line 2709 (RV3-N1).
- **A.** Base against candidate, the raw JSON leaf diff has exactly one leaf per mode: `diagnostics[5].message`, the NUMERICAL_INTEGRITY_CHECKS_PASSED Debug text. The `ResidualRow` global_dof 6 residual and normalized residual change, and so does the audit ratio text. The code lists are equal. No value, status or code moves. The derived carriers are checked by `result_export` (91 passed), the readers pytest, and T1's `cp3 --check` and joined `--check` (exit 0).
- **B.** Base against candidate producer: exactly the two rows per mode.
  - The rows are `support:fixture-root` Fx and its force magnitude, in case:closed-pressure.
  - Dense: 5.82e-11 → 6.64e-11 N. Sparse: 0.0 → 8.19e-12 N.
  - That is at most 4.2e-15 of the 15708 N scale, with no status change.
  - The consumers are green: `result_export` `physics_contract` and `physics_source_contract` (inside the 91 passed); the physics consumer, physics source, stress_neutral physics source and source readers pytest, 367 passed, 4 skipped; the desktop parity test inside the full vitest (2822/2822, ×2).

## 7. Check 7: every pin behavioural, and the evasion attempts

**The pins and their behavioural backing:**
- Rules 1, 2, 4 and 5 (ledger and typed seams): F1, F2, F3, F11, F12 and F14, each with an in-test fold-differs precondition from the product's own `case_ledger` terms.
- Rule 3 and the rule 8 E-sites: F2 (E5, E7, E12; bit-equal to the net-load model), F8 (E8–E11) and F10 (E15, E16).
- T1's sites: F4, F5 and F6.
- KS1: F9. N6: `n6_product_residual_rows_use_the_exact_numerator`, which first asserts that the exact and binary64 numerators differ.
- The sparse_direct text pin: the sparse-mode F-tests.
- The Sensitive mapping has no behavioural test (RV3-S1).

**Evasion and mutation attempts** (scratch worktree, one group at a time, restored by rewriting the saved bytes; `mutations/`). Every entry also counts as a test run.

| ID | Attempt | Site test | Killed behaviourally by |
|---|---|---|---|
| EV1 | `global_load_vector` through a function-pointer alias inside `push_nodal_loads`, pushing the folded vector | **passes** (rules 1 and 5 do not see it) | F1, F4, F6 |
| EV2 | E7 folded with `Iterator::reduce` | **passes** (rule 8 does not count `.reduce`) | F2 g1e8, F2 g1e80 (the simply supported variant) |
| EV3 | E15 restored as `s = s + v` | rule 8 (RV1-N1 self-assignment) | F10 g1e8, F10 g1e80 |
| EV4 | E16 folded by a helper in the unscanned sibling `pressure_sum.rs` | **passes** | F10 g1e8, F10 g1e80 |
| EV6 | E12 folded through that sibling helper (UFCS-style call) | **passes** | F2 ×2, F8 uniform ×2, F10 ×2, F14 |
| EV5 | The Sensitive mapping call deleted | passes | **none: survives** (RV3-S1) |
| — | Comment or string text satisfying a source pin | n/a | Not viable: the scanners lex comments out and blank literals. The `solve_preview_reduced_system` and `solve_load_case` presence pins need real code (lexer checked, `scanner_self_checks`). |

## 8. Check 8: mutations (§9 sample, G = 1e8 and 1e80 as separate tests)

These mutants are my own patches, written independently of I4's (`mutations/rv3_mutants.py.txt`).

| Mutant | Killed by (behavioural) | Also |
|---|---|---|
| M1f (E5 fold) | F2 g1e8, F2 g1e80, F14, F10 g1e8, F10 g1e80 | rule 8 |
| M1l (E12 on the folded terms) | F2 ×2, F14, F8 uniform ×2, F8 thermal, F10 ×2 | rule 8 |
| M1n (E15 fold) | F10 g1e8, F10 g1e80 | rule 8 |
| M2 (E5 as two roundings) | F10 g1e8, F10 g1e80 (the only axial-effect straight case) | rule 8 |
| M3 (the ledger folded) | F1, F2 ×2, F3 ×2, F4, F5, F6, F8 ×3, F10 ×2, F14 | rules 5 and 8 |
| M8 (pre-0.4 replay folded) | F4, F6 | rule 8 (the mutant's own `+=`) |
| M9 (0.4.0 replay folded) | F5 | rule 8 |

- Every sampled mutant is killed by the named §9 test, at both G values where the test exists at both.
- None is killed only by a source-text pin.
- M2 is killed only by F10, which carries the axial-effect loads. §9 names F2 for M2, but probe A has no axial term; I4's table shows the same.

**The changed T1 unit test (Addendum 1, check 2).**
- The new assertion `force[9] == 1.0` is the exact value: the correctly rounded net of 1e16 + 1 − 1e16 is 1.
- The precondition's fold of the authored loads in authored order is 0.0 (1e16 + 1 ties to even at 1e16), so the old `global_load_vector` path gave 0.0 at DOF 9 and fails the new assertion.
- The source-plan assertions are unchanged: the three terms in order, the two co-located springs, and `descriptors.len() == 42+12+2+18`. Nothing is weakened (see RV3-N4 for an optional strengthening).

## 9. Check 5 (line 2709) and Check 6 (carried items)

- **Line 2709.** It is disclosed separately from S11-F's effects in CHANGE_RECORD. Provenance: 22452ecd1 was the last regeneration, before 1792774a2 in ancestry, and the base producer emits the corrected value. The formula explanation does not reproduce (RV3-N1).
- **The sparse_direct sibling.** There is none. `sparse_direct_factor_inherits_the_prepared_ledger_binding` passes; `sparse_direct/structural.rs` reads no force; SA `solve_assembled` goes through `prepare_assembled_structural` and then `factor_structural_ldlt(&prepared)`. Behavioural coverage comes from the sparse-mode F-tests.
- **The `global_load_vector` doc note.** Present, with "no product caller remains", which is confirmed (§3).
- **RV1-N1.** Both scanners count self-assignment folds (`scanner_counts_self_assignment_folds`, `scanner_self_checks`). The re-baselined counts pass, with no unlisted hit.
- **RV1-N5.**
  - Per-row `Range` becomes an unaudited row with ratio ∞. An audit input error becomes `LoadFidelityReport { rows: [], audit_error }`.
  - `finish_checked_factor` never propagates an audit error.
  - The (1e80, −1e80, 1e-300) test passes.
  - The audited rows equal the base signature bit for bit.
  - There is no new envelope field: `LoadFidelityReport` reaches the facade only as diagnostic text.
  - The facade message is untested (RV3-S1).
- **RV1-N6.** No rename. `evaluate_original_residual(` is in `FORBIDDEN_PRODUCT_CALLS`, and the N6 behavioural test has its paths-differ precondition.
- **RV1-N8.** Positive doctest twins. Nonlinear doctests: 2 compile_fail, 2 compile, all ok.
- **RV1-B-N1.** `::solve(` and `::solve_assembled(` are in `EXACT_ENTRY_POINTS`; the new adapter scan passes; the doc comment says the behavioural pins are authoritative.
- **SA `solve` pub(crate).** CHANGE_RECORD and the s11k doc comment say, correctly, that this closes EV4 only for callers outside `nonlinear_integration`, and that inside the crate the behavioural pins remain the guarantee.

## 10. Check 9: hard constraints

- **S-H is not in the slice.** The captured entry still refuses |x| ≥ 2^53, which F1 asserts.
- **F11's 1e80 cases run typed.** This covers F1, F2, F8 and F10 at G = 1e80.
- **No in-band marker.** No envelope or serialized struct gains a field: only the private `PreviewLinearSolve` and `ThermalElementLoad` change.
- **DEC-046 and `validation/benchmarks/nonlinear` are unchanged.** No file under them changed, and `multisupport_acceptance_inventory_uses_narrow_dec_046_policy` is ok.

## 11. Check 10: the K-D5 boundary

**Boundary kept:**
- `solve_preview_reduced_system` changes only as follows: `global_force: &[f64]` becomes `&AssembledForce`, the one call becomes `assembly.solve_assembled(original_stiffness, global_force, &free, prescribed, mode)?` (`PP:4176-4177`), and the result carries `load_fidelity: checked.load_fidelity`. Its shape is unchanged.
- `finish_checked_factor` changes only in the N5 audit block.
- SA's public surface: `solve` becomes `pub(crate)`; `solve_assembled` and `solve_binary64` stay public.

**What K-D5's forward merge must adapt to:**
1. Its `solve_with_formation_check` must take `&AssembledForce`, and it replaces `assembly.solve_assembled(` at `PP:4177`.
2. Its result must still populate `PreviewLinearSolve.load_fidelity`.
3. It must merge its formation check around the N5 block in `finish_checked_factor`.
4. The new `option_c_structural_adapter_legacy_variants_reach_only_binary64_entry_points` blanks only `solve`, `solve_assembled` and `with_force_terms`. An SA function calling `prepare_assembled_structural(` or `prepare_structural(` must be added to that defining list, or that test fails.
5. The product site test's `rule_1…` asserts `body("solve_preview_reduced_system").contains("assembly.solve_assembled(")`. That pin must be updated for the switch.
6. Rule 6 (`FORCE_FUNCTIONS`) must list any new SA or FK function that touches the force.
7. I1's `EXACT_ENTRY_POINTS` now contains `::solve(` and `::solve_assembled(`. These must be kept when the pins are integrated.

## 12. Check 11: disclosure and hygiene

- **CHANGE_RECORD** covers:
  - the §8.3 items (the quantities that move, the one-rounding bound, no status change unless absorbing, no in-band marker);
  - the formation rows, including the 3% worsening;
  - the byte sizes of the regenerated files;
  - line 2709, separately;
  - the pre-approval citation `1397e8c43`;
  - the carried items.
- **No machine paths.** A grep of all added lines for home-directory, temp, root and tool-install path prefixes (and macOS user paths) finds none.
- **Formatting.** With `rustfmt` 1.8.0-stable, the new files are clean. In `lib.rs` the unformatted hunks drop from 110 (base) to 107 (head); `pressure_runtime.rs` stays at 5 and `nonlinear_integration/src/lib.rs` at 24. No other changed file has a hunk.
- **`git diff --check`:** 3 blank-line-at-EOF records (RV3-N9).
- **The records' `SHA256SUMS`** verify: 126 entries, all OK, covering every file under `_run_records/` plus the 4 record files outside it.
- **No `node_modules`** or `target` path is committed. The authority targets are untouched.

## 13. Desktop vitest timeout (Addendum 1, check 1)

**Result: the timeout did not reproduce, on origin/main or on the candidate, at load averages up to 5.6.** `desktop/desktop_runs.txt` has the details.

**Setup.**
- `node_modules` was linked from `<wt>/engine`, whose package-lock is identical for main and the candidate (`0dd1616e…`). The links were removed afterwards.
- The candidate wasm came from `<wt>/s11f` (built by ROOT's sweep at `11ae0667f`).
- Main's wasm was built by me with `npm run build:wasm` in a `git archive` export of `f0a6159c9`.
- No timeout was raised, and nothing was skipped or isolated.

**Runs:**

| Run | Load (start → end) | Result |
|---|---|---|
| Main, full vitest | 3.37 → 3.99 | 134/134 files, 2822/2822 tests |
| Main, `App.test.tsx` ×2, concurrently with a full candidate vitest | peak 4.72 | 225/225, twice |
| Candidate, full vitest, concurrently with two release cargo builds | 1.89 → 5.61 | 2822/2822 |
| Candidate, full vitest, second run | 2.85 → 5.43 | 2822/2822 |

- ROOT's DEC-025 sweep on `11ae0667f` also passed desktop_vitest (`SWEEP_20260927T071900Z_11ae0667f1f1.json`: overall pass).
- The failing test (`qualifies only a matching captured native unit replay…`) replays `physics_connected_ui_*`, `precision_connected_ui_*` and `preview_physics_connected_*` through `src/test/nativeMechanicsReplay.ts`. None of those fixtures changed in S11-F.

**Conclusion.** There is no evidence that S11-F introduced the failure. It reproduced on neither side, so it is at most a rare, load-sensitive flake. This observation cannot prove the flake is pre-existing on main.

## 14. What I ran

The toolchain was Rust 1.97.1 (`RUSTUP_AUTO_INSTALL=0`, `CARGO_INCREMENTAL=0`, `--offline --locked`), with target `<scratch>/rv3-target`, in a detached scratch worktree at `11ae0667f`. Python was 3.11.15 (DEC-025 venv) and Node was 24. I ran one cargo job at a time, after ROOT's DEC-025 sweep on this head had finished.

- **Authority targets.** Built in scratch.
- **Full suites** (`--no-fail-fast`, all pass):

| Crate | Passed | Failed | Ignored |
|---|---|---|---|
| product_physics | 474 | 0 | 1 (pre-existing) |
| frame_kernel | 139 | 0 | 0 |
| nonlinear_integration | 74 | 0 | 0 |
| primitive_loads | 49 | 0 | 0 |
| sparse_direct | 25 | 0 | 0 |
| curved_bend | 25 | 0 | 0 |
| straight_pipe | 39 | 0 | 0 |
| result_export | 91 | 0 | 0 |
| runner/headless | 83 | 0 | 0 |
| benchmarks/nonlinear | 19 | 0 | 0 |
| benchmarks/mechanics | 41 | 0 | 0 |

- **Python.** The load_reference readers (239 passed, 1 skipped); the physics and stress-neutral consumers plus the source readers (367 passed, 4 skipped); T1's cp3 and joined `--check` (both exit 0).
- **Producers.** Candidate and base, both kinds, both modes.
- **The generator re-run.**
- **Mutations and evasions.** 7 §9 mutants and 6 evasion attempts.
- **One scratch probe.** UDL-W1e8's ledger terms. The probe was added to the scratch copy only, then removed.
- **The caller scans.**
- **The line-2709 recomputation.**
- **Desktop.** 5 vitest runs and a candidate `npm run build`, which was ok.

Run records: `REVIEW/_run_records/s11f_review/`, with its own `SHA256SUMS`.

## 15. What I did not check

- The full I4 mutation set. I did not run M1g–M1k, M1o, M4, M5 or M12 myself; I accept I4's records for those.
- A base build of the RF-CANCEL formation rows other than UDL-W1e8. For the INPLANE and UDL-W1e80 base values I rely on I4's table and on F1's exact pins.
- A full DEC-025 sweep (ROOT ran one on this head: pass).
- Hosted CI.
- The full Python suite list, beyond the consumer files above (ROOT's sweep ran the python_pytest surface: pass).
- operation_applier, `apps/desktop/src-tauri`, and the stress and physics_audit_regression benchmarks under cargo (covered by ROOT's cargo sweep: pass).
- The exact 22452ecd1 mechanism of the line-2709 value (RV3-N1).
- A line-by-line review of the 24130-line generated fixture. It is verified by byte-identical regeneration instead.

## Addendum 1 (2026-09-27): delta check of the RV3-S1 follow-up (PR #1002)

**Verdict: PASS.** RV3-S1 is resolved, and so are N1, N2 and N9. The follow-up adds no new finding. No product code or fixture changes, so no DEC-025 sweep is needed.

**What I reviewed.**
- ROOT's request: PR #1002, branch `codex/piping-s11f-s1-20260927`, head `79654820159167f9c88e5450ef1b6ab62e18b2cb`.
- The range is `43b8f83aa..796548201`, one commit on the merge of PR #1000.
- The PR #1000 merge `43b8f83aa` has parents `b0a9a52b6` (main) and `11ae0667f` (the head reviewed above). Its `projects/chirality-piping` tree is identical to `11ae0667f`'s.
- I built in a fresh detached scratch worktree at `796548201`, with its own target. The records tree was removed from the scratch copy to save disk. The authority targets were built in the copy.
- Cargo was product_physics only, one job at a time, in the slot the T3 manager granted.
- `CARGO_PROFILE_DEV_DEBUG=0` and `CARGO_PROFILE_TEST_DEBUG=0` were set to save disk, with the manager's agreement. They do not change what any test asserts.
- Records: `_run_records/s11f_review/delta_pr1002/`. The `SHA256SUMS` of `s11f_review/` has been refreshed.

**1. The new tests catch the fault, and pass on the repair.**
- On `796548201`, the full product_physics crate passes: 476 passed, 0 failed, 1 ignored (the existing `composite_fields_work_measurement`). That is lib 350 (348 plus the 2 S1 tests), the site test 9, and the other integration tests unchanged.
- Four mutants, applied one at a time to my copy and restored afterwards, are all killed (`delta_pr1002/results.json`):

| Mutant | Change | Killed by |
|---|---|---|
| EV5 | the `append_load_contribution_absorbed` call in `solve_load_case` deleted | `s1_unauditable_load_row_is_published_sensitive_with_the_warning` |
| EV5c | the unaudited-row message wording altered | both S1 tests |
| EV5d | sources dropped from `affected_refs` | both S1 tests |
| EV5e | the audit error not named in the message | `s1_sensitive_mapping_names_every_flagged_row_and_refuses_nothing` |

- This matches I4's recorded EV5 and EV5b runs.

**2. Coverage of the three branches.**
- **Audited row:** the unit test checks the full line, with dof, both bit patterns, the ratio and the sources. That branch cannot be reached through the typed seam, which is why it is driven with a synthetic report.
- **Unaudited row:** the unit test checks it, and so does the end-to-end test. The end-to-end test uses an authored (G, −G, 1e-300) tip load, with no test hook. It runs on both entries at G = 4e15, on the typed entry at G = 1e80, and in both modes.
- **audit_error:** the unit test checks `Flagged rows: []` and the exact suffix.
- Both tests also assert:
  - code `LOAD_CONTRIBUTION_ABSORBED`, severity `warning`, and the stable id;
  - refs = the case followed by the sorted, deduplicated sources;
  - exactly one such diagnostic;
  - no error or blocking diagnostic.
- The end-to-end test also asserts `MECHANICS_SOLVED`, `NUMERICAL_INTEGRITY_SENSITIVE` (the kernel's verdict, as the precondition) and that the rows are published.
- In my review I said the path was unreachable without a hook. That is corrected: only the audited branch is unreachable, and the unaudited branch is reachable from an authored model.

**3. The record changes.**
- **N1.** The CHANGE_RECORD provenance now states only what is established: stale since `22452ecd1`, the code changed at `1792774a2`, and the base producer emits the correctly rounded value. It says the `22452ecd1` mechanism is not reproduced. PRE_REGENERATION_REPORT §3.2 and §5 carry a marked correction and are otherwise kept as measured. The disclosure and the conclusion are unchanged. **Resolved.**
- **N2.** `callers.json` was re-run on the final tree. It adds `load_state_eigen_loads`, `build_thermal_element_loads` and `Sources::system`, and it now classes files of `#[cfg(test)] mod x;` modules as test (`membrane_publication_range.rs:187` is test). The `prepare_sources` test lines are current (1617…). It agrees with my independent list. **Resolved.**
- **N9.** RETURN §8 now records the three blank-line-at-EOF run-record logs as a stated exception: they are raw tool output, hash-bound by `SHA256SUMS`. That is acceptable under the brief's "or the exceptions recorded". **Resolved.**
- The `IMPLEMENTATION/S11F/_run_records/SHA256SUMS` at `796548201` verifies: 131 entries, all OK.
- RETURN §11 correctly records N3–N8 as not taken on this branch.

**4. Nothing else changed.**
- `git diff --name-only 43b8f83aa..796548201` lists exactly 12 paths: `core/product_physics/src/s11f_tests.rs` (+209, tests only) and 11 files under `IMPLEMENTATION/S11F/` (`delta_pr1002/range_files.txt`).
- There is no product source, fixture, schema or other test file, so no DEC-025 sweep is needed.
- `git diff --check` is clean. The added lines hold no machine paths, and rustfmt 1.8.0 finds `s11f_tests.rs` clean.

**Not checked in this delta:** hosted CI for PR #1002. I also ran no other crate, because the change is confined to one product_physics test module.
