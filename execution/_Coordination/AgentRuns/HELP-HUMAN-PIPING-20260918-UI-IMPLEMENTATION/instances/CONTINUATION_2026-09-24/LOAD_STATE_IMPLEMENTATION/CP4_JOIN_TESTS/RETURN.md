# CP4_JOIN_TESTS — return (CP3 review N-1)

**Result: COMPLETE.** The module has 8 maintained tests. Every targeted mutant (K5, K7, K8, K9, K10) is killed by at least one of them, and K1–K4 and K6 remain killed. No production code, existing test, tolerance, fixture or reference was changed.

## Identity and scope

- **Role.** TASK (Type 2), dispatched by ROOT at the request of the T1 WORKING_ITEMS manager (load/reference states). I did not delegate, and I made no Git writes.
- **Checkout.** `codex/piping-load-states-20260925` at `c0ef4a8e0` (this includes `a819a2b1f` and the SF-1 commit `6235f6b43`).
- **Paths.** Paths are relative to WORKING_ROOT = `projects/chirality-piping/`. `LSI` is the LOAD_STATE_IMPLEMENTATION instance folder.
- **Files written**, all inside the write boundary:
  - `core/product_physics/src/source_receipt/load_state_join_tests.rs`, 1017 lines, sha256 `a934c640c7405ccbc7a51e7367f6cde2f5e311f04c5fa0b50fdfcf8d25768a59`;
  - `LSI/CP4_JOIN_TESTS/` (this file and `_run_records/`).
- **Inputs.** All inputs are invented. The tests use the maintained `eigen_motion` and `mixed` witnesses plus invented edits, and each edit carries provenance `invented_load_state_join_closure_test_not_library_data`. No library value, code rule or default is used. The witness constants are asserted against the fixture before use.

## Tests and what each pins

| Test | Replaces | Pins |
|---|---|---|
| `a_selected_case_whose_member_pair_differs_from_the_base_qualifies_with_its_own_pair` | P1 | An exact-point pair (E 150 GPa, ν 0.3; base 200 GPa / 0.25) gives a qualified `load-reference-source-1` receipt in both modes. The published `pipe_materials` E/ν/G, the record's `selected_E_pa`/`selected_nu` and ε* are the selected pair's values. The axial rows match the closed form N = E·A·(δ/L − ε*) at 1e-9 relative: end i = −N; end j and the three stations = N; midspan σ = N/A. Tip UX and UY follow the prescribed motions. |
| `two_selected_cases_with_different_pairs_and_motions_each_bind_their_own_state` | P2 | Two cases with different pairs, strains and root motions. Both are selected, and each record carries its own E and ε*. Per-case closed forms at 1e-9: tip UY is 1.2 mm and 2.2 mm (g + θ·L), tip UX = δ, and midspan N uses that case's own E and ε*. |
| `a_zero_valued_pressure_region_is_not_joined` | P4 | A 0 Pa region in both modes publishes `load-reference-1` with no receipt. The record is `not_joined`, the NOT_JOINED diagnostic ends in `retained_source_attempt=unavailable`, and there is exactly one `SOURCE_BLOCK_RECOVERY_UNAVAILABLE`. The rows are bit-identical to the uncaptured ordinary route. Also kills K6. |
| `post_capture_perturbations_accepted_live_are_refused_by_captured_replay` | P5 | Five perturbations after capture: strain split with the same total, resolver evidence only, an explicit zero motion on root UX, an effective primitive magnitude, and ν only. For each, the **live closure must accept** (asserted, not skipped as in the probe). Captured replay must then refuse with `captured source replay`, and the genuine selection must fail current binding. Kills K5 and K10, and helps kill K7. |
| `a_resolved_motion_without_exactly_one_rigid_owner_is_refused_at_source_closure` | motion ownership (K7) | A resolved motion inserted on a free DOF (tip UY), on the spring-only DOF (root RX), as an explicit zero on a free DOF, or outside the DOF range. The test asserts that the actual partition never carries it, and that `source_recovery::solve` refuses at stage `source closure` with `SourceMismatch("resolved support motion lacks one rigid boundary owner")`. |
| `a_primitive_load_colliding_with_the_reserved_eigen_source_id_is_refused_at_closure` | P6 (K8) | A declared 1e-6 N primitive named `load_state_eigenstrain:6:member`.<br>• **In module:** `solve` refuses at stage `source closure` (helper stage `SourceClosure`) with `SourceMismatch("eigen source identity collides with a primitive load")`.<br>• **Public route, both modes:** ordinary `load-reference-1` and `unavailable`, with rows bit-identical to the ordinary route. The producer's attempt diagnostic names that stage and refusal. |
| `the_mixed_joined_envelope_binds_its_ordinary_pressure_case_record` | P7 (K9) | **Public route:** the `mixed` witness in both modes gives a joined, qualified envelope. Case 0 is `selected`. The pressure case is `not_joined`, with its ordinary method, `checks_passed` and `retained_source_attempt=not_required_ordinary_checks_passed`.<br>**In module:** the test re-runs the route's per-case `solve_load_case` finalization (`route_cases`). It checks that this reconstruction is the product's: `finalize_composite` reproduces the **published receipt byte-for-byte**, and `validate_publication` is `Ok`. It then changes a published record between the product and receipt finalization: four pointers on the ordinary case's record and one on the selected case's. `validate_publication` refuses each with `case load/reference-state record changed`, and complete `finalize_composite` refuses the changed ordinary record. |
| `joined_rows_agree_with_the_ordinary_route_and_closed_forms` | P11 | **Joined rows against the uncaptured ordinary route**, in both modes, at the reviewer's tolerance: 1e-6 relative with a floor of 1e-9 of the unit's largest magnitude, excluding torsion rows. Every joined row must exist in the ordinary route, and more than 70 rows are compared. The four variants are `eigen_motion`, the exact-point pair, factor 2.5 with an excluded 7 kN source, and a parallel spring on the moving rigid root UY.<br>**Closed forms at 1e-9:**<br>• factored: root RX = 2.5·T/k; tip UZ = 0 (|UZ| ≤ 1e-12 mm); the torque contribution has factor 2.5 and applied magnitude 2.5·T; `excluded:force` is absent from the contributions and present in `excluded_sources`;<br>• parallel spring: spring Fy = −k·g = −1 N, anchor Fy = +1 N, root UY = 1 mm. |

P3, P9 and P12 are already pinned in `load_state_fallback_tests.rs` (SF-1) and are not duplicated here. P8 and P10 were budget characterizations with no assertion to carry, so they are not ported.

## Checks

Commands: `cargo +1.97.1 … --locked --offline -j 2` in `core/product_physics`, with a dedicated target directory. The logs are in `_run_records/`.

| Check | Result | Log |
|---|---|---|
| `cargo test --lib source_receipt::load_state_join_tests` | 8 passed, 0 failed. Every test loops over both modes, except the two in-module closure tests (mode-independent) and P5, which uses the dense capture as the implementer's controls do. | `join_module_tests.log` |
| `cargo test` (whole crate) | **415 passed, 1 ignored**, 0 failed. That is the 407 baseline plus these 8; lib is 324 (316 + 8), and the other binaries are unchanged. | `product_physics_all_tests.log` |
| `rustfmt +stable --edition 2021 --check src/source_receipt/load_state_join_tests.rs` | clean (rustfmt 1.8.0-stable) | `rustfmt_check.log` |
| Mutation runs K1–K12 | table below | `cp4_join_mutations.{py,log}` |
| `git status --short` | Only my test file and `LSI/CP4_JOIN_TESTS/` are mine. It also lists the concurrent TASK's result_export and analysis_runs edits, `CP4_READERS/`, and manager records created during this run (`CHECKPOINT_4.md`, `CP4_WIRE_ADDENDUM.md`, `T1_PLAN.md`). I touched none of them. | — |

## Mutation runs

**Method.**
- The scratch copy was `git archive HEAD` (`c0ef4a8e0`) of `core/`, `fixtures/` and `validation/benchmarks/numerical_integrity/`, plus this module.
- It was built from `git archive` rather than copied from the working tree, so that the concurrent TASK's uncommitted reader edits could not enter it.
- **Prehash:** all 55 files under `core/product_physics` in the scratch copy have the same sha256 as the candidate working tree (`scratch_prehash.txt`).
- **Anchors:** `cp4_join_mutations.py` parses the reviewer's mutant table from `REVIEW_CHECKPOINT_3/_run_records/review3_mutations.py`, so the anchors are verbatim rather than retyped. It applies each mutant once and restores and sha256-verifies after each one.
- **Suites** (no mutant caused a compile error):
  - "prior" is the reviewer's committed-suite set (lib, `load_reference_state_runtime`, the extension and `physics_source_runtime`), skipping this module. It shows what was maintained before CP4.
  - "join" is this module alone.

| Mutant | Prior suites | This module | Killed by (this module) |
|---|---|---|---|
| K1 replay builds with base materials | killed (7) | killed (4) | P1, P2, P5, P11 tests |
| K2 replay prescribed zero | killed (5) | killed (4) | P1, P2, P5, P11 |
| K3 replay omits eigen loads | killed (5) | killed (4) | P1, P2, P5, P11 |
| K4 live eigen fold sign flipped | killed (9) | killed (5) | P1, P2, P5, P11, motion-ownership control |
| **K5** identity omits resolver evidence | survives | **killed** | `post_capture_perturbations_…` (evidence_only) |
| K6 pressure check removed | killed (1) | killed (1) | `a_zero_valued_pressure_region_is_not_joined` |
| **K7** motion-ownership loop removed | survives | **killed** (2) | `a_resolved_motion_without_exactly_one_rigid_owner_…` and `post_capture_perturbations_…` (explicit_zero_motion) |
| **K8** collision check removed | survives | **killed** | `a_primitive_load_colliding_…` |
| **K9** ordinary-case record binding removed | survives | **killed** | `the_mixed_joined_envelope_binds_…` |
| **K10** identity omits strain split | survives | **killed** | `post_capture_perturbations_…` (strain_split_same_total) |
| K11 identity omits eigen local axis | survives | survives | not targeted; see below |
| K12 frame-area check removed | survives | survives | not targeted; see below |

**Why K8 needed an in-module observable.** Under K8, source closure *accepts* the colliding input: a separate scratch run shows the test failing at `source closure accepted the input`. The public route still publishes `unavailable`, but only because of a later refusal, which is why P6's outcome alone could not kill K8. The maintained test therefore asserts on:
- the closure's own refusal (`RecoveryFailure.stage`, `helper_stage` and the `SourceMismatch` variant);
- the producer's attempt diagnostic naming that stage.

It does not rely on any reader-side message.

**Why K9 needed an in-module tamper.** In a joined envelope the ordinary case's published record is compared only by that one line; `physical_source` uses the proof's copy, not the envelope's. The test proves its reconstruction first: it reproduces the published receipt exactly. So the refusals it then observes are the product's own.

**K11 and K12** survive both suites, as they did at CP3. The reviewer classed both as equivalent in practice, because the local axis and the areas are bound elsewhere: node coordinates, stiffness aggregate and section geometry. They were outside this brief's targets. I made no production change for any mutant.

## Design questions and notes for the manager

1. **Reconstruction in the K9 test.** `route_cases` mirrors the 0.4.0 branch of `run_linear_static_preview_captured_once`: validate, normalize, resolve every case, build each case's pair model, `assemble_case_stiffness` and `solve_load_case`. The byte-equal receipt assertion guards it against drift: if the route changes, that control fails loudly rather than letting the tamper checks pass vacuously. A small `#[cfg(test)]` hook exposing the route's finalized cases would remove the duplication, but that is a production-code change and I did not make it.
2. **K12 (optional).** An in-module test could kill K12: perturb `built.frame_elements[0].section.area`, reassemble K, and expect `resolved member pair/section did not reach formation`. I did not add it because K12 was not a target and the reviewer judged it equivalent. The same holds for K11. This is ROOT's call if the table should show them killed.
3. **Observed, not asserted.** The ordinary route publishes one row more than the joined route for these witnesses (for example 90 against 89 for `eigen_motion`). The comparison therefore requires every joined row to have an ordinary counterpart, not equal counts.
4. **Exact equality of ε\*.** The producer's ε\* is the correctly rounded λ_fit·λ_thermal − 1 (`1.0000240000000001e-4`), while the naive binary64 product in the test gives `…39999996552e-4`. The tests compare it at 1e-9 relative, the same criterion as the existing closed-form tests, and do not require bitwise equality.

## Cleanup

The scratch copy and both cargo targets (`ls-join-scratch`, `ls-join-target`) were deleted after the runs. The logs in `_run_records/` carry no absolute machine paths; placeholders `<scratch>`, `<checkout>` and `<target>` stand in for them.
