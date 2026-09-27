# S11-G implementation: RETURN (I5)

I5 (Type 2 TASK, Claude), 2026-09-27. Brief: `T3/TASK_BRIEFS/I5_S11G_IMPLEMENTATION.md` (`14dcf9103`) with Addendum 1 (`c0d090172`). Basis: `S11G_GUARD.md` revision 2.2 (selected; D22-1 binding), its erratum §0.2 (`6e5e7f03b`), revision 2.1's remaining text, and ROOT's rulings, as listed in `CHANGE_RECORD.md`.

**Summary.** The guard is implemented and every test passes.
- **Gate.** All 14 formation rows are published non-Passed on both entries and both modes.
- **Fixtures.** The committed-fixture diff is zero: 218 of 218 runs are byte-identical.
- **Mutations.** Every mutant is killed on the final tree (§4), including the rev 2.2 and E-1 mutants M19–M23.
- **Git.** None: no commit, index, stash or checkout operation. The manager commits.
- **Paths.** Records use placeholders only: `<wt>/s11g`, `<s11g-target>`, `<s11g-mut-target>`, `<scratch>`, `<venv>`.

## 1. Files and lines

The diff is against `3d844fea4`, from `git diff --numstat` plus untracked files.

| File | Change |
|---|---|
| `core/solver/frame_kernel/src/load_ledger.rs` | +707 −2. `Formation` (l.84); `push_formed` (l.152); `push_formed_product` (l.168); `formation_rows` / `has_formation_records` (l.297/307); `gamma`, directed-rounding helpers (l.330ff); `FormationRow` (l.412); `add_defect`, `add_twelve_value`, `formation_row` (l.470–575); 5 unit tests `s11g_*` (l.719–935) |
| `core/solver/straight_pipe/src/lib.rs` | +588. `equivalent_global_nodal_loads_with_spans_formed` (l.672); `bending_formation_bound` (l.701); exact-expansion helpers (l.1689ff); `mod s11g_tests` with T7, T14, T17 (l.1963ff) |
| `core/product_physics/src/formation_guard.rs` | new, 502 lines: the decision, S\*, R-b′, `demote` / `amend_integrity_report` |
| `core/product_physics/src/lib.rs` | +288 −31: formed pushes at the classified sites; `append_integrity_report(…, formation)`; the routing predicates `source_eligible`, `decline_for_formation`, `needs_source_recovery` (l.1095–1125); the guard call in `solve_load_case` (l.2621); routing (l.2713–2790); R-b′ records and the amendment (l.3317, 3784–3800) |
| `core/product_physics/src/source_recovery.rs` | +34: `FORMATION_GUARD_DECLINE`, `formation_decline_without_attempt` (l.70–85), `decline_formation` (l.258) |
| `core/product_physics/src/source_receipt.rs` | +8 −2: `OrdinaryAttempt::passed(…, formation_sensitive)` (l.454–470) |
| `core/product_physics/src/s11g_tests.rs` | new, 2339 lines: 27 product and unit tests (§3) |
| `core/product_physics/tests/s11f_site_test.rs` | +238 −2: `formation_guard.rs` in PRODUCT, formed-push counting, T8, T10b |
| `core/product_physics/tests/fixtures/s11g/rb_controls.json` | new test input (sha256 `c12b3cb9…`), from `generators/gen_rb_controls.py` |
| `core/runner/headless/tests/s11g_zero_work_receipt.rs` | new (110 lines), T23. The manager extended the write set for it |

**Untouched:**
- SA, FK `structural.rs`, `solve_preview_reduced_system` and `finish_checked_factor` (the K-D5 boundary);
- CB and `pressure_runtime`;
- every receipt, reader and schema;
- all library and code-rule data.

## 2. Caller list (lexer scan)

`_run_records/callers/caller_scan.py.txt` scans every `.rs` file under `core/`, `apps/` and `validation/`. It strips comments, string and char literals first. The full list is in `_run_records/callers/callers.txt`.
- **Changed signatures.** Each has only in-crate callers:
  - `OrdinaryAttempt::passed` has one call (PP l.2714);
  - `append_integrity_report` has two (PP l.2944 linear, l.2985 nonlinear).
- **`equivalent_global_nodal_loads_with_spans`** is unchanged, and its callers (SP, `user_loads`) are unchanged. The formed variant is called once in the product (PP l.8836) and otherwise only from SP tests.
- **New functions** are called only from PP `solve_load_case` and the formation-site producers:
  - `push_formed` at 10 sites and `push_formed_product` at 1;
  - `load_row_finding`, `moment_scales`, `recovery_finding`, `amend_integrity_report` once each.
- **T8** (`t8_every_case_force_producer_is_classified`) pins the producer classification. **T10b** pins the routing site.

## 3. Tests

**All pass.** The full crate suites are in §8. The final rerun after formatting and the added tests is in §8a.

| Test | Where | What it pins | Result |
|---|---|---|---|
| T1 | PP | UDL-W1e8 demoted, captured and typed, both modes (load-row) | pass |
| T2 | PP | UDL-W1e80 demoted, typed | pass |
| T3 | PP | UDL-W1e5 stays CHECKS_PASSED | pass |
| T4 | PP | nodal cancellation never demoted by the load-row guard | pass |
| **T4b** (new) | PP | exact formed terms cancelled by inputs stay silent (M3's kill) | pass |
| T5 | PP | probe A: same-expression defects cancel | pass |
| T6, T6a, T6b | PP | collinear skew pair; collinear runs silent with the floor (thrust run in the historical scope, §9); the floor never hides a net defect | pass |
| T7, T14, T17 | SP | formed variant bit-identical, with the exact oracle; range fallbacks; B on a skew member | pass |
| T8, T10b | site test | producer classification; the routing site calls the tested predicates (G-1, G-2, G-3, E-1) | pass |
| T9 | run evidence | committed-fixture diff: zero (§6) | zero diff |
| T10 | PP | the routing predicates (G-1, G-2) | pass |
| T11, T12 | PP | INPLANE demoted by R-b′ alone; accurate small-moment rows below the floor stay Passed | pass |
| T13, T13b | PP | `load_reference_fallback_uz` byte-identical; an already-Sensitive case is untouched | pass |
| T15 | PP | curved uniform load: CannotBound | pass |
| T16 | PP | the gate: 14 rows non-Passed, the S11 list empty, no breach outside the lists | pass |
| T18 | PP | path 2 is not refused; case B keeps main's attempt (byte-identical to base, §5a) | pass |
| T19 | PP | path 1, load-row variant, is not refused; per-case bases select case A (settles N3) | pass |
| T20 | PP | characterization of the R-b′ residual construction (§6a) | pass |
| T21 | PP | G-3: a guard-fired selection is declined, with its work charged | pass |
| T22 | PP | E-1 / D22-1: the invocation ledger equals the unguarded one on a Passed, guard-fired case | pass |
| T23 | headless | the Rust reader accepts the zero-work decline entry, with and without the invocation | pass |
| Ruling and unit tests | PP, FK | ruling 1 (both guards), ruling 2 (boundary), no-op rule, M10, M18, **`rb_prime_clauses_at_their_boundaries`** (new; X4 and X7), 5 ledger unit tests | pass |

Every verdict pin asserts its paths-differ precondition inside the test.

## 4. Mutations

**The driver.** `_run_records/mutations/s11g_mutants.py` applies exact textual patches, where each anchor must be unique. It runs the killing command, then restores the bytes and verifies the sha256. It uses no Git.
- Each run used its own target (`<s11g-mut-target>`). Nothing else ran cargo during it.
- `BASELINE` (unmutated) runs every killing command and passes, so each kill belongs to its mutant.

| Mutant | Patch | Failing tests (killing) | Verdict |
|---|---|---|---|
| BASELINE | no mutation | (none; every killing command passes) | SURVIVED |
| M1 | drop a formed term defect (Exact defect = 0) | t1_udl_w1e8_is_demoted_on_both_entries_and_modes, t2_udl_w1e80_is_demoted_on_the_typed_entry | KILLED |
| M2 | tag the straight uniform formed term as input (plain push) | t1_udl_w1e8_is_demoted_on_both_entries_and_modes, t2_udl_w1e80_is_demoted_on_the_typed_entry, t8_every_case_force_producer_is_classified | KILLED |
| M3 | give input terms a bound of u\|t\| (T4 alone: equivalent, no formed row) | t4b_exact_formed_terms_cancelled_by_inputs_stay_silent | KILLED |
| M4 | replace SP's exact defect with the a-priori gamma_16 sum\|monomials\| bound | t3_udl_w1e5_stays_checks_passed, t5_probe_a_same_expression_defects_cancel | KILLED |
| M5 | take the free-row S* over all rows | t1_udl_w1e8_is_demoted_on_both_entries_and_modes, t2_udl_w1e80_is_demoted_on_the_typed_entry | KILLED |
| M6 | sum \|eps\| instead of the signed sum | t5_probe_a_same_expression_defects_cancel | KILLED |
| M8 | demote through report.quality before routing | t1_udl_w1e8_is_demoted_on_both_entries_and_modes | KILLED |
| M9 | map a fired case to a blocking diagnostic | t1_udl_w1e8_is_demoted_on_both_entries_and_modes | KILLED |
| M10 | combine the Exact and RoundedProduct families in binary64 (revision 1) | m10_families_are_combined_exactly | KILLED |
| M11 | drop the floor | t6a_collinear_runs_are_silent_with_the_floor | KILLED |
| M12 | take the floor from all formed terms | t1_udl_w1e8_is_demoted_on_both_entries_and_modes | KILLED |
| M13 | drop R-b''s floor clause (R-b ships) | t12_accurate_small_moment_rows_below_the_floor_stay_passed | KILLED |
| M15 | demote an already-Sensitive case again and append text | no_op_rule_leaves_a_non_passed_record_untouched, t13b_already_sensitive_case_is_left_untouched | KILLED |
| M16 | treat CannotBound as a zero defect | t15_curved_uniform_load_is_cannot_bound | KILLED |
| M17 | apply the floor to the whole row (revision 2's rule) | t6b_floor_never_hides_a_net_defect | KILLED |
| M18 | the binary64 literal 1e-9 | m18_threshold_constant_is_rounded_down | KILLED |
| M19 | reintroduce revision 2.1's load-row routing gate | t18_path2_invocation_is_not_refused, t19_path1_load_row_variant_is_not_refused | KILLED |
| M20 | the routing predicate ignores the load-row finding (G-2) | t10_routing_predicates, t19_path1_load_row_variant_is_not_refused | KILLED |
| M21 | the ordinary attempt ignores the finding: passed(..., false) (G-2) | t19_path1_load_row_variant_is_not_refused, t10b_routing_site_calls_the_tested_predicates | KILLED |
| M22 | a guard-fired selection is not declined (G-3) | t21_selection_is_declined_for_a_formation_finding | KILLED |
| M23 | always run the charged attempt on a guard-fired case (E-1 / D22-1) | t22_invocation_budget_equals_main_on_a_passed_guard_fired_case | KILLED |
| M14a | signed sums in B | t11_inplane_rows_are_demoted_by_rb_prime_alone | KILLED |
| M14b | \|T u\| in B | t17_bending_formation_bound_on_a_skew_member | KILLED |
| MR2 | ruling 2: restore revision 2.1's first clause B >= T0 alone | ruling2_boundary_of_the_first_clause, t6_collinear_skew_pair_cancels_signed_defects | KILLED |
| X1 | drop B's gamma_16 | t11_inplane_rows_are_demoted_by_rb_prime_alone, t17_bending_formation_bound_on_a_skew_member | KILLED |
| X2 | gamma_1 instead of gamma_16 in B | t17_bending_formation_bound_on_a_skew_member | KILLED |
| X3 | drop the RZ row from B | t11_inplane_rows_are_demoted_by_rb_prime_alone | KILLED |
| X4 | drop R-b's resolution clause q > 2^10 B | rb_prime_clauses_at_their_boundaries | KILLED |
| X5 | S*_moment without the L_b S(force) coupling | t12_accurate_small_moment_rows_below_the_floor_stay_passed | KILLED |
| X6 | drop the R-b' amendment | t11_inplane_rows_are_demoted_by_rb_prime_alone | KILLED |
| X7 | R-b's first clause at 1e-6 instead of 1e-9 | rb_prime_clauses_at_their_boundaries | KILLED |
| M7 | drop revision 2.1's load-row gate | withdrawn: revision 2.2 G-1 removed the gate (M19 reintroduces it) | n/a |

The full commands and exit codes are in `_run_records/mutations/results.json`, and the failing tests per mutant in `failing_tests_per_mutant.txt`. **Every mutant is killed by a failing test; none by a compile error.** (BASELINE is recorded as SURVIVED, which is expected: it is the unmutated tree.)

**First pass (before the final tests).** The results are in `_run_records/mutations/first_pass_results.json`.
- **M3 survived.** It is equivalent on T4, whose rows have no formed term. T4b was added.
- **X4 and X7 (my extras) survived.** No product row separates them: forecast rows are either resolved far above noise with B/q > 1e-6, or sit below the floor. The unit test `rb_prime_clauses_at_their_boundaries` was added.
- M7 (drop 2.1's gate) is **withdrawn**, since revision 2.2 G-1 removed the gate. M19 reintroduces it and is killed by T18 and T19.

**A slip, disclosed.** I stopped the first rerun of the final tree after its BASELINE failed: T4b's first draft used a transverse load whose SP rotation coefficient is not exact. The driver was killed while it was running M1, so M1's patch (two anchors in FK `load_ledger.rs`) stayed applied.
- **The restore.** I reverted it by exact reverse replacement. I then verified that `load_ledger.rs` equals the rustfmt-formatted pre-run copy byte for byte, and that no other mutant's text is present in any file (anchor check: all unique, none applied).
- **The rerun.** The final table above comes from a fresh run on the restored tree.

## 5. Gate table (the 14 formation rows)

Probes use the fixture-diff harness's `probe` mode on base and candidate binaries (`_run_records/gate/gate_table.py.txt`; table `_run_records/gate/gate_table.md`). Base publishes CHECKS_PASSED on every row and the candidate publishes SENSITIVE:

| Entry, case | Rows × modes | Guard | Leaves that differ, base against candidate |
|---|---|---|---|
| captured, UDL-W1e8 | th.S1.RZ × 2 | load-row | the case's integrity code, severity and message; one added info diagnostic (`SOURCE_BLOCK_RECOVERY_UNAVAILABLE`, E-1's zero-work decline); `numerical_quality` case and status |
| typed, UDL-W1e8 | th.S1.RZ × 2 | load-row | integrity code, severity, message; `numerical_quality` |
| typed, UDL-W1e80 | th.S1.RZ × 2 | load-row | same |
| typed, F-G1e80-GnG-INPLANE | Mb.M1.j, Mb.M2.i × 2 | R-b′ | same |
| typed, M-G1e80-GnG-INPLANE | Mb.M2.i, Mb.M2.j × 2 | R-b′ | same |

- **No values change.** No `/results` leaf differs on any row. S11-F's code-level `FORMATION_PINS` stay bit-identical (T16 and S11-F's `f1_f11_f12` pass).
- **Next step (the manager's).** Run `GATE/pin_s11_exceptions.py` to empty `FORMATION_EXCEPTIONS.json`, and commit the GATE change.

### 5a. Run evidence for T18, T19 and T23 (captured entry, both modes)

- **T18.** The whole envelope is byte-identical between base and candidate (sha256: dense `832319d7…`, sparse `5cf0c071…`). Case B is `unsupported`, with main's real attempt (`charged` 36480).
- **T23 / T19-shape request** (`_run_records/reader/t23.request.json`).
  - **Candidate:** case A qualified; case B `unsupported` / `source_validation` / `unsupported_family`, ordinary `sensitive`, work {0, 0, 0}, integrity code `SENSITIVE`. No blocking diagnostic.
  - **Base:** case B is qualified ordinary, `checks_passed`.
- All the hashes are in `_run_records/reader/envelopes.sha256`.

## 6. Fixture diff (T9)

**The harness.** `_run_records/fixture_diff/fixdiff_s11g.rs.txt` is S11-F's `fixdiff` harness, plus a typed-entry pass. It ran as an example of a scratch copy of each tree:
- base = `git archive` of `3d844fea4`;
- candidate = a copy of the worktree.

It covered every committed request or model under `fixtures/`, `core/` and `validation/`, in both modes, through the captured entry (112 outputs) and the typed entry (106 outputs). The runner is `run_t9.sh.txt`, and `status.txt` records the runs.

**The result.** All 218 base outputs equal the candidate outputs, and `base_outputs.sha256` equals `candidate_outputs.sha256`.
- 6 runs are ERR on both trees, identically: requests that are not preview requests.
- No output contains "S11-G".
- 88 outputs carry `NUMERICAL_INTEGRITY_SENSITIVE`, identically on both trees.

### 6a. The R-b′ residual: constructions tried (ruling 3; T20)

None selected case A beside an R-b′-firing Passed case B:
1. V1's N1 single-basis construction: case A is refused by retained recovery (exact radix range).
2. INPLANE loads plus N05's torsion spring in one body: `UnsupportedBlock{order: 3}`.
3. Two disjoint bodies (N05's cantilever plus the plain INPLANE body), with case B on an invented soft basis: case A is refused with `UnsupportedBlock{order: 4}`.
   - T20 pins what this construction actually does: no selection, no receipt, and no refusal. The invocation publishes, with case B demoted by R-b′.

The residual stays "not demonstrated reachable" and fail-closed if reachable. RV4 tries independently. The owner is the `SOURCE_BLOCKS_FINALIZATION_FAILED` item.

## 7. Reader window (ruling 1)

**The window** runs from the append (PP l.2944 / l.2985) to the amendment (PP l.3796). It contains only:
- the nonlinear-branch pushes of `NUMERICAL_INTEGRITY_RECOVERY_BASIS_UNQUALIFIED`, which are never CHECKS_PASSED, so the amendment is a no-op there;
- `require_finite_mechanics`;
- the element and support recovery calls, which push result rows and diagnostics.

**No reader in the window.** A scan of l.2944–3796 finds no reader of `solve_quality`, `numerical_quality`, a diagnostic's `code`, or standing. `assessed_numerical_quality` and receipt finalization run after `solve_load_case` returns.

## 8. Suites

Crate suites ran sequentially with `cargo test --offline --locked --no-fail-fast`. The runner is `_run_records/suites/run_suites.sh.txt`, and the summary is `summary.txt`. Every crate exits 0 with 0 failures:

| Crate | Passed |
|---|---|
| FK | 144 |
| SP | 42 |
| CB | 25 |
| sparse_direct | 25 |
| linear_supports | 15 |
| nonlinear_integration | 74 |
| nonlinear_supports | 22 |
| performance_harness | 25 |
| primitive_loads | 49 |
| **product_physics** | **503** (1 ignored, pre-existing) |
| headless | 84 |
| result_export | 91 |
| operation_applier | 194 |
| mechanics benchmarks | 41 |
| nonlinear benchmarks | 19 |
| numerical_integrity observer | 0 (no tests) |
| src-tauri | 114 |

- **Python consumers** (`<venv>`): the load_reference (headless artifacts, readers, schema, source readers, source schema), physics consumer and source contracts, preview_physics consumer, stress_neutral (export package, physics source, precision) and `validation/benchmarks/numerical_integrity/test_reference.py`.
  - Result: **1570 passed, 20 skipped.** The skips are env-gated and pre-existing (`python_consumers.txt`).
- **Python qualification group:** gate, load_reference, physics, physics integration and physics structure: **128 passed, 83 subtests passed** (`python_qualification.txt`), run on the final tree. The two Python groups together cover the suites the manager listed.
- **Desktop reader** (scratch only, stock `sourceBlockRecovery.ts`, sha256 `9c53c8cd…`; the wasm engine is the engine worktree's build of identical `operation_applier`/`units` sources, sha256 `cb40d116…`).
  - The T23 envelopes validate without a throw, with the same standing as main's own `unsupported` shape (the T18 envelope), with or without `callerModel`. The same request on base is `{eligible: true}`.
  - Records: `_run_records/reader/` (the test, the vitest output, and the hashes).

### 8a. Final rerun

I reran the touched crates after rustfmt, T4b, the R-b′ clause unit test and the module doc update. The tree was the final one.
- **The runs:** `run_final.sh.txt`, `final_rerun_summary.txt`.
- **Results:**
  - FK 144 passed;
  - SP 42;
  - product_physics 505 (1 ignored, pre-existing), which includes T4b and `rb_prime_clauses_at_their_boundaries`;
  - headless 84, which includes T23.
- **Failures:** 0.
- **Warnings:** the remaining product_physics warnings (`not_attempted`, `derived`, `expected_primary`, `is_exact_pressure_result_kind`, `mode`, `spring_actions`) all exist on base. The one S11-G warning, an unused test helper `with_second_case`, was removed.
- **The mutation run** (§4) used the same tree, except that the s11g_tests.rs module doc comment was updated after it.
- **Rest of the tree.** The full-suite counts in §8 predate these changes. They cover every other crate, and those crates depend on the touched crates only through APIs that did not change after that run.

## 9. Deviations, slips and disclosures

- **Rulings 1 and 2** are implemented as ruled; see CHANGE_RECORD, deviations 1–2.
- **T6a.** The straight-thrust family is unreachable at a nonzero value on a fresh solve (`pressure_runtime.rs:207-224`; exact profile l.237–240). T6a's thrust run uses the `#[cfg(test)]` historical scope, following S11-F's F10 precedent. The unit test is `s11g_rounded_product_defect_is_exact`.
- **The desktop coverage gap.** No committed desktop fixture carries a non-qualified receipt entry. It is disclosed in CHANGE_RECORD, and no fixture was added.
- **Disk.** Free disk touched **4.9 GB** as src-tauri finished, below the 5 GB stop point the manager set mid-run. That job had already completed, so it was not stopped. I then pruned my own `<s11g-target>/debug` (7.0 GB), and free disk returned to 12 GB. The authority targets were not touched.
- **rustfmt.** The 1.97.1 toolchain has no rustfmt component, and nothing was installed. The stable toolchain's rustfmt 1.8.0 formatted the touched files whose base was clean. PP `lib.rs` was left unformatted, because base has 147 rustfmt diffs across that crate.
- **The mutation-run slip** (§4) is disclosed and repaired.
- **The earlier path-2 test.** The rev 2.1 characterization test (`ruling3_characterization_…`, PATH2_CONSTRUCTION.md) was replaced by T18 when rev 2.2 removed the gate. PATH2_CONSTRUCTION.md is kept as the historical record for D1.

## 10. Toolchain and environment

- `RUSTUP_TOOLCHAIN=1.97.1`, `RUSTUP_AUTO_INSTALL=0`, `CARGO_INCREMENTAL=0`, `--offline --locked`.
- Build targets: `<s11g-target>` for the suites and T9, pruned between them; `<s11g-mut-target>` for the mutations.
- The authority binaries came from the prerequisite targets `core/serialization/canonical_json/target` and `core/units/target` (built once, never deleted).
- Python: `<venv>` (dec025-venv). Node 22 with vitest 4.1.10 for the scratch reader run.

## 11. Not done

- The GATE generator run and its commit (the manager's).
- A committed desktop-reader test of the zero-work shape (ruled out; coverage gap disclosed).
- A construction selecting case A beside an R-b′-firing case (the residual, §6a).
- Formatting PP `lib.rs` (base not rustfmt-clean).
