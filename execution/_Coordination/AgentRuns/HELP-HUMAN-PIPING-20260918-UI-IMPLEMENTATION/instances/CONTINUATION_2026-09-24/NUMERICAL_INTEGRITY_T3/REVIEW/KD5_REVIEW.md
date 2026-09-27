# RV5: independent complete-diff review of slice K-D5

**Verdict: **NOT PASS**, for head `2409de83ec327d7ec4c67149d60ea3a641d28f89`. There is 1 BLOCKING finding, 3 SHOULD-FIX findings and 7 NOTEs. The blocking finding: M31b is observable at the criterion, so ROOT's equivalence (`c2042fd9c`) does not hold, and M31b and M31b0 must be killed by required tests before merge (RV5-B1). The check itself is correct: it uses the actual chord. The combined-tree gate passes (888 runs, 0 trusted breaches, and 122 is the only change against main). A delta check of I3's repair commit follows, and the final verdict will name that head.**

**Reviewer.** RV5 is a Type 2 TASK. The brief is `T3/TASK_BRIEFS/RV5_KD5_REVIEW.md` (`512abb123`) with addendum 1 (`c8cd6d1c2`, ROOT's M31b counterexample definition), read with `_COMMON.md` and the manager's spawn updates. Those updates are: check 4 against main's empty lists; the combined tree after the S11-G merge; the M31b attack is required; T9, the comment-only commit and the re-kills; and the performance NOTE.
- **Independence.** I did not design D-5, R5-4 or K3a, check their designs, or implement K3a or K-D5. This is not owner review.
- **Changes.** I fixed nothing, and I made no Git writes. The trees are `git archive` extractions under `<scratch>`.

## 0. Revisions reviewed

| Item | Value |
|---|---|
| Candidate | branch `codex/piping-kd5-20260926`, head `2409de83ec327d7ec4c67149d60ea3a641d28f89` (pushed; `git fetch` confirmed it) |
| Base (merge base with `origin/main`) | `b24b3d5360a4809d7c584c1780a39955fa810dcd`, which contains S11-K, K3a, S11-F and S11-G. `origin/main` has since moved to `9963645a7`, with records only |
| Chain | `17f3d6e05` (the K-D5 implementation), then `b6156d49d`, `8fd409e78` and `3befacff4` (merges of main), then `a89fde17b` (the addendum-4 pass), then `fbc9661a4` (the S11-G merge), then `2409de83e` (a comment-only commit) |
| Diff | `git diff b24b3d536...2409de83e`: 140 files, +31782 −19. Of these, 13 are product or test code, and the rest are `IMPLEMENTATION/KD5/**` records |
| Design basis | `DESIGN_NUMERICS/DESIGN.md` rev 5a.2 (`fb62ef4a…`, verified); `R5_4_CURVED.md` (`2c9fae78…`, verified) |
| Rulings | `ROOT_RULINGS_V1.md` on main `9963645a7`: D-5, D5C-1..5, R5-4, the S11-G rulings (DS-1), "K-D5 mutation M31b" (`c2042fd9c`), and "K-D5: the combined-tree gate after S11-G" |

## 1. Findings

| ID | Severity | Site (at `2409de83e`) | Evidence | Resolution |
|---|---|---|---|---|
| **RV5-B1** | **BLOCKING** | The M31b equivalence (ROOT `c2042fd9c`), as recorded at `IMPLEMENTATION/KD5/CHANGE_RECORD.md:117-120` and `RETURN.md:181-193`, `:408`. The test that was meant to carry it is `core/solver/nonlinear_integration/src/structural_adapter/kd5_tests.rs:419-445` (`kd5_curved_intended_element_uses_the_actual_chord`) | **M31b is observable at the criterion, under both (a) and (b) of addendum 1.** Confirmed in Rust (§3.3).<br>**The mechanism.** The chord error δc = d·cos(φ/2) (d = \|r_j\| − \|r_i\|) moves the free end of the bend by about θ_i × δc to first order, on the stiff translation rows. That is about (d/R)·cot(φ/2)/2 relative to S\*_tr. Only the soft-mode moment is second order.<br>**Why I3 missed it.** I3 compared only the maximum-row trigger (1.681 against 1.685). On `CSKEW_30_RADIUS_MISMATCH` the chord alone puts 0.433 of the criterion on row uz at node 1 (DOF 8). The test's own comment (`kd5_tests.rs:423`, "adds about 0.4 of the criterion") says so. That row is not the maximum row, so the test cannot tell the two variants apart.<br>**Counterexamples** (admissible: product geometry check passed, radius mismatch ≤ 1e-9; well conditioned, rcond ≥ 6e-5), in Rust, both modes:<br>- CANT60_PLANAR: published Passed, actual error 1.102 of the criterion. The correct check demotes (trigger 2.204, row DOF 8); **M31b does not demote (trigger none)**. This is (a) and (b).<br>- CANT30_SKEW: actual error 1.901, trigger 3.801 against none.<br>- CANT10_SKEW: actual error 6.484, trigger 12.97 against none.<br>- At product level, X 5e6 m / φ 5° (PP's own centre, no designed mismatch): the candidate publishes SENSITIVE and the M31b build publishes CHECKS_PASSED, on both entries in both modes, with actual error 1.126. Also X 5e6 m / φ 2° (actual error 3.12) and X 7.3e6 m / φ 10° (2.28).<br>**M31b0** is killed by the same test | 1. Add required tests that kill M31b and M31b0: the 60° planar and 30° skew models at adapter level, both modes; and the product-level X 5e6, Y 3.5e6, φ 5° case (must demote) with the X 5e5 control (must not demote). My `rv5_test.rs.txt` and `rv5_models.rs.txt` (exact u_int) reproduce them.<br>2. Record that the equivalence is withdrawn. Correct the "second order … far below the criterion" reasoning in CHANGE_RECORD and RETURN §7 / A4-4.<br>3. No change to the check itself: it already uses the actual chord (§4.2) |
| **RV5-S1** | SHOULD-FIX | `_run_records/callers.txt` and `RETURN.md:146-156` (§5) | These are the phase-1 enumeration at `a2e804a75`. They name the removed `solve_with_formation_check` and use phase-1 line numbers. `RETURN.md:17` flags §§1–11 as phase-1, but the candidate has **no** refreshed caller list. My lexer scan of the candidate (§5) confirms the substance | Regenerate `callers.txt` on the final head |
| **RV5-S2** | SHOULD-FIX | The nonlinear pins at `s11k_tests.rs:894-1013` (`FORMATION_ENTRY_POINTS`, the lexed source pin and the behavioural loop pin) | **Evasion E4 escapes every NI test and the nonlinear benchmark.** It adds a sibling module `unit_bridge.rs`, a copy of `solve_linearized_system_evidence` whose solve is an SA helper wrapping `solve_assembled_with_formation_check`. lib.rs uses it only for the derived-friction unit-force solves (`lib.rs:1409`).<br>- lib.rs then names no formation entry point.<br>- The SA scan does not flag a call to `.solve_assembled_with_formation_check(` from another SA function, because `EXACT_ENTRY_POINTS` holds `.solve_assembled(`, which does not match.<br>- The behavioural pin exercises only the first iteration of a never-closing gap.<br>Result: 84/84 lib tests and 4/4 doctests pass; `validation/benchmarks/nonlinear` 18/18 behavioural pass (its 19th test fails only because my partial copy lacks `validation/hand_calcs`). E1, E2 and E3 are caught behaviourally (§7) | Either extend the source scan to every non-test file under `nonlinear_integration/src`, and add `.solve_assembled_with_formation_check(` to the SA scan outside its defining function; or add a behavioural pin with a derived-friction candidate. Mutation (32) then covers the unit-force solves |
| **RV5-S3** | SHOULD-FIX | `IMPLEMENTATION/KD5/CHANGE_RECORD.md:5-12`, `RETURN.md` | The committed records stop at the pre-S11-G tree. CHANGE_RECORD says "The merge of it into this branch, and the combined-tree checks, come next". The candidate's combined-tree evidence (suites on `fbc9661a4`, T9 against `b24b3d536`, both gate parts on `2409de83e`, the attribution against main) exists only in I3's uncommitted scratch addendum, which still carries `COMBINED_STATUS`/`PART2_BLOCK` placeholders | Commit the combined-tree RETURN addendum and records, with the results of part 2 (§6) |
| RV5-N1 | NOTE | I3's mutation runner, `_run_records/addendum4/mutations/run_mutants.sh.txt` | The copy step (`tar -xf`) preserves mtimes. M32a and M32b patch only NI, so cargo reused the previous run's frame_kernel artifact, which carried M31b0's patch. `M32a.log`/`M32b.log` show only `Compiling …nonlinear_integration`. The kills themselves stand: my clean re-run (fresh mtimes, §3.4) kills both with the same tests | Use fresh mtimes (`tar --touch`) or a clean target per mutant in the repair's re-run |
| RV5-N2 | NOTE | `core/product_physics/tests/formation_check_runtime.rs:173` | The receipt loop over `source_block_recovery.body.cases` is vacuous for a nonlinear invocation, which has no receipt (`.as_array().into_iter().flatten()` iterates nothing). The receipt byte identity required by addendum 2 rests on T9 (the committed nonlinear request `preview_physics_invented_model.json`, 112/112) and on SA `kd5_not_selected_invocation_runs_the_unchanged_solve_assembled` (`==`) | Optional: assert the field's actual shape |
| RV5-N3 | NOTE | `formation_check.rs:413` (`body_scales`) | S\*(kind) uses free DOFs only. Design §4.1.6.1 item 4 uses the body's published rows, which include prescribed motions. On 0.4.0 prescribed motion this gives a smaller scale, so it is conservative (more demotion) | Record the reading |
| RV5-N4 | NOTE | Product finding for T4/W1c | With exact inputs, PP's binary64 centre at coordinates of 2e6 m and above makes the product's non-objective curved element publish errors above the criterion (1.13–3.12 at 5e6 m, 1.0–2.28 at 7.3e6 m). The correct check demotes these (loss of Current). At 1e6 m and below the error is ≤ 0.36 (§3.2). ROOT has narrowed the T4 finding to ≳ 2e6 m | For T4/W1c (build H from the actual chord) |
| RV5-N5 | NOTE | Near-boundary (addendum 1) | Largest tuned-flip shift found: sparse k_X = 45 on the CSKEW30 geometry, trigger 1.1615 correct against 1.1590 M31b. At k_X = 1000 the chord-only row reads 0.433 in both variants, with no demotion | Record only |
| RV5-N6 | NOTE | Record hygiene | Four empty committed logs (`_run_records/mutations/nohup*.log`). `git diff --check` flags trailing whitespace and blank lines at EOF only in hash-bound raw logs (`MUTANTS.txt`, suite logs). The code files are clean | Optional |
| RV5-N7 | NOTE | RV5's own method (disclosed) | My first emulation lifted binary64 inputs with `Decimal(repr(v))`, which is not exact. I3 caught it. Every row was re-derived with exact inputs: small-coordinate rows are unchanged to 4 digits, and large-coordinate rows changed (the old run is kept as superseded; §3.2) | — |

## 2. Check 1: complete-diff review

I read every changed product and test line at `2409de83e`: FK `structural.rs`, `formation_check.rs`, `formation_check_tests.rs`, `retained/mod.rs` and `wide.rs`; SA `structural_adapter.rs`, `kd5_tests.rs` and `kd5_models.rs`; PP `lib.rs` (one hunk); `tests/formation_check_runtime.rs` and `tests/s11f_site_test.rs`; `s11k_tests.rs`; `s11g_tests.rs` (comment-only). I also read CHANGE_RECORD, RETURN and the records.
- **Wiring.** `finish_checked_factor` (`structural.rs:1465-1491`) computes `ordinary_sensitive = rcond < √eps || load_fidelity.is_some()` exactly as main. It runs the check only when `prepared.formation` is `Some` and the case is not already Sensitive, and ORs the demotion into `quality`. `StructuralReport` is unchanged. `formation_check` is a separate `Option` on `StructuralSolution` and is dropped at PP (`lib.rs:4429` copies only `checked.report` and `load_fidelity`).
- **The check.** It is `formation_check.rs:155-339`: ρ from ledger terms or folded force (`:210-226`); the re-formed element contributions through exact splits (`:228-270`); the truncation allowance, widened away from zero (`:271-281`); a scaled solve with the attempt's factor (`:283-302`); the rule (`:304-337`).
  - Frames (`:580-635`) re-form L, the six coefficients and TᵀKT at p, sharing nothing with binary64.
  - Joints (`:640-661`) use the chord frame.
  - Curved elements (`:665-762`) follow R5-4 §2 steps 1–6. H is built from `axes·(x_j − x_i)`, **the actual chord**, at `:733-749`.
- **Typed PP entry (addendum 4).** `lib.rs:4390-4407` calls `assembly.solve_assembled_with_formation_check(original_stiffness, global_force: &AssembledForce, &free, prescribed, mode, &curved_sources, built.nonlinear_supports.is_empty())`. There is no other PP change and no `&[f64]` entry.
- **SA.** `solve_assembled_with_formation_check` (`structural_adapter.rs:375-409`) returns `solve_assembled` when `selected` is false. Otherwise it is `solve_assembled`'s body with `with_formation_source` and `prepare_formation_checked_structural`. Curved matching (`:411-460`) goes by node indices plus bitwise `global_stiffness()`.

## 3. Check 2: the M31b attack

### 3.1 Method

- **Exact-arithmetic emulation.** Standard-library Python, Decimal at 80 digits, with exact binary64 inputs (`m31b/exact_inputs/`).
  - K_act is the check's intended system (H from the actual chord).
  - K_frm is identical except that H comes from the product's binary64 chord R(cos φ − 1), R sin φ, 0. That is M31b's intended element and, up to its own rounding, the product's element.
  - EF_correct = K_act⁻¹(f − K_act·u) and EF_M31b = K_act⁻¹(f − K_frm·u), per free nodal row, over 1e-9·max(\|q\|, S\*) as in `formation_check.rs`.
  - The port reproduces I3's exact u_int of `CSKEW_8_5` and `CSKEW_30_RADIUS_MISMATCH` digit for digit (`validate_cskew.stdout.txt`).
- **Rust.** A copy of `git archive 2409de83e` (tree `c954590e…`). NI `cargo test rv5_` was run on the unmutated tree and with I3's M31b and M31b0 patches (`mutate.py.txt`, unchanged), using my appended `rv5_*` tests and models with exact u_int.
- **Product.** Two probes built from P1's probe source: the candidate, and the candidate with the M31b patch.

### 3.2 Models (EF = |w|/criterion; the trigger is 2·EF). Full inputs and rows are in `m31b/exact_inputs/m31b_emulation_results.json`

| Model (one realized bend, node 0 translations rigid, rotational springs, tip moment (1,1,1) unless stated) | Admissible | EF_correct (max row) | EF_M31b | \|ΔEF\|/crit | Actual | Class |
|---|---|---|---|---|---|---|
| CSKEW_30_RADIUS_MISMATCH (I3's), chord-only part, row DOF 8 | yes | 0.433 (that row) | ~0 | 0.433 | 0.433 (that row) | shows the mechanism |
| φ 90°, planar, max mismatch, springs 1e6 | yes | 0.640 | 8e-8 | 0.640 | 0.640 | **(b)** |
| φ 90°, skew (normal ∝ (1,2,2)) | yes | 0.480 | 7e-8 | 0.480 | 0.480 | — |
| **φ 60°, planar** | yes | **1.102** | 1e-7 | **1.102** | **1.102** | **(a), (b)** |
| φ 60°, skew | yes | 0.750 | 5e-8 | 0.750 | 0.750 | (b) |
| **φ 30°, skew** | yes | **1.901** | 5e-8 | **1.901** | **1.901** | **(a), (b)** |
| φ 10°, planar / skew | yes | 6.12 / 6.48 | ~1e-7 | 6.12 / 6.48 | same | (a), (b) |
| φ 3° / 1° (planar, skew) | yes | 19.5–68.4 | ~1e-7 | same | same | (a), (b) |
| Near π: π − 1e-3, π − 1e-6, π − 1e-8 (max and zero mismatch) | yes | ≤ 2e-4 | ~1e-7 | ≤ 2e-4 | ≤ 2e-4 | none |
| Extreme R/L: R 30 m φ 1e-3; R 30 m φ 0.05; R 0.01 m φ 30°; R 3 m φ 10° | yes | 993.6; 15.2; 2.28; 5.51 | ~1e-7 | same | same | (a), (b) |
| Stiff-X / soft-Y combinations (φ 60° skew; k = 1e8/1e2/1e6 and permutations; k_X 30) | yes | 0.43–0.63 | ~1e-7 | 0.43–0.63 | same | (b) in 3 of 4 |
| Four-bend chain, 60°: spiral / zigzag | yes | 0.627 / 0.245 | ~1e-7 | same | same | (b) / — |
| PP route, PP's own centre, R 0.3 m: X0 1e3 / 1e5 / 5e5 (φ 2°) | yes | 0.0003 / 0.019 / 0.043 | ~1e-7 | same | same | none |
| PP route: X0 5e6 m, φ 5° / φ 1° / R 1.5 m φ 10° | yes | 1.125 / 0.977 / 0.219 | ~1e-7 | same | same | (a), (b) / (b) / — |
| Coordinate scan (72 geometries, `coord_scan.json`) | yes | ≤ 0.36 for X0 ≤ 1e6 m; up to 1.00 at 2e6 m; 1.13–3.12 at 5e6 m; 1.0–2.28 at 7.3e6 m | | | | |

- **The first emulation used `Decimal(repr(v))`** (RV5-N7). Its results are kept in `m31b/superseded_repr_inputs/`, and `repr_vs_exact.txt` compares every row. Only the large-coordinate PP rows changed; for example 5e5 m φ 2° went from 2.50 to 0.043, which confirms I3's objection.
- **Fail-closed coverage (addendum 1).** No admissible curved geometry reaches a fail-closed path.
  - `AngleDomain` needs 1 + cos φ to round to 0 at p = 128, i.e. φ within about 1e-19 of π. The product refuses φ > π − 1e-9 (`curved_bend/src/lib.rs:28`, `:166-169`).
  - The other Wide operations in `curved_matrix` cannot fail for finite admissible inputs.
  - M31b changes only the chord, after every fallible operation that the correct path also performs. So it cannot turn a fail-closed case into a completed check.
  - My near-π Rust models (π − 1e-6 and π − 2e-9, with and without the maximum mismatch) complete in both variants and do not demote (actual ≤ 1.4e-5).

### 3.3 Rust confirmation (`rust/ni_rv5_{base,M31b,M31b0}.log`)

| Model | Mode | Plain | rcond | Actual | Correct check | M31b | M31b0 |
|---|---|---|---|---|---|---|---|
| RV5_CANT60_PLANAR | dense | Passed | 3.9e-4 | 1.1019 | **demoted, 2.2037 @ DOF 8** | not demoted | not demoted |
| | sparse | Passed | 3.9e-4 | 1.1019 | demoted, 2.2037 | not demoted | not demoted |
| RV5_CANT30_SKEW | dense / sparse | Passed | 2.7e-4 | 1.9005 | demoted, 3.8010 @ DOF 6 | not demoted | not demoted |
| RV5_CANT10_SKEW | dense / sparse | Passed | 8.8e-5 | 6.4842 | demoted, 12.968 | not demoted | not demoted |
| RV5_PP_UTM (X 5e6, φ 5°) | dense / sparse | Passed | 6.4e-5 | 1.1261 / 1.1260 | demoted, 2.2523 / 2.2520 | not demoted | not demoted |
| RV5_CANT90_PLANAR | dense / sparse | Passed | 4.3e-4 | 0.6399 | demoted, 1.2798 | not demoted | not demoted |
| RV5_CSKEW30 k_X 1000 / 100 | both | Passed | | 0.433 | not demoted | not demoted | |
| RV5_CSKEW30 k_X 45 | sparse | Passed | | 0.579 | demoted, 1.1615 | demoted, 1.1590 | |
| CSKEW_30_RADIUS_MISMATCH (I3) | dense / sparse | Passed | | 0.840 / 0.869 | 1.6847 / 1.7422 | 1.6809 / 1.7385 | |
| Near-π, 3 models | both | Passed | 1.7e-3 | ≤ 1.4e-5 | not demoted | not demoted | |

Demoted solutions pass `assert_demoted_only_in_quality`: values bit-identical, only the quality differs. With M31b or M31b0, `rv5_m31b_counterexample_actual_error_above_criterion_is_demoted` fails, so each is **killed**. Every other kd5 test passes under both mutants (`rust/M31b.kd5.log`, `M31b0.kd5.log`).

**Product level** (`rust/product/SUMMARY.txt`). Probes: candidate sha256 `771900fe…`, M31b `01f07f70…`.

| Request (R 0.3, OD 0.2, wall 0.01, E 2e11, G 8e10, rigid N0 translations, rotational springs 1e6, moments (1,1,1)) | Candidate (4 runs) | M31b build (4 runs) | Actual (dense / sparse) |
|---|---|---|---|
| X 5e6, Y 3.5e6, φ 5° | SENSITIVE / needs_recompute | CHECKS_PASSED / numerically_eligible | 1.1257 / 1.1258 |
| X 5e6, φ 2° | SENSITIVE | CHECKS_PASSED | 3.1227 / 3.1212 |
| X 7.3e6, φ 10° | SENSITIVE | CHECKS_PASSED | 2.2755 / 2.2754 |
| X 5e5, φ 2° (I3's PP_UTM_2; control) | CHECKS_PASSED | CHECKS_PASSED | 0.0441 / 0.0431 |

The `results` arrays are byte-identical between the two builds for every request, mode and entry.

**Conclusion.** The equivalence does not hold. M31b and M31b0 must be killed by required tests before merge (ROOT's condition 2). The implementation's actual chord is correct and required, and `kd5_curved_intended_element_uses_the_actual_chord` kills M31a (§3.4).

### 3.4 Check 5: mutation re-kills (clean copies with fresh mtimes; `cargo test kd5` in NI; `rust/*.kd5.log`)

| Mutant | Result | Killed by |
|---|---|---|
| (23) trigger disabled | killed | the 122 true positive, D5C-1 controls, the 8.5 elbow, the actual chord, the joint, the one-ulp slots, the not-selected precondition, the nonlinear-loop precondition, RV5's counterexample |
| (27) element ΔK plus binary64 residual | killed | the 122 true positive, D5C-1, E1/E6, the 8.5 elbow, the actual chord, matching order, the joint, the one-ulp slots, RV5 |
| (31a) curved K_int is the product's matrix | killed | **`kd5_curved_intended_element_uses_the_actual_chord`**, the 8.5 elbow, RV5 |
| (31b) / (31b0) | **killed only by RV5's new test** | `rv5_m31b_counterexample_actual_error_above_criterion_is_demoted` |
| (32a) `solve_binary64` through the check | killed | `kd5_nonlinear_loop_reaches_no_formation_check` (behavioural) |
| (32b) the loop calls the typed entry | killed | the behavioural pin and the lexed source pin |

No mutant is killed by a source pin alone. I3's recorded kill sets agree, apart from the stale build (RV5-N1).

## 4. Check 3: fail-closed, no value change, never selected

- **Every path that cannot re-form demotes, and none returns `Err`.** Enumerated from source:
  - `check` (`formation_check.rs:166-179`): any `unavailable` entry; a joint with lateral stiffness ≠ 0; any `evaluate` error.
  - `evaluate` errors:
    - `Shape`: DOF map (`:195-200`), force-term DOF (`:213`), element node (`:231`), spring DOF (`:264`), residual exponent (`:355`), correction output (`:300`);
    - `Sum`: every accumulator operation;
    - `Wide`: `WideArith::new`, every re-formation (AngleDomain and ArctangentLimit included), `add_product_to`;
    - `Structural`: the correction solve.
  - SA adds `curved_bend_explicit_matrix` and `curved_bend_source_unmatched`. The one-ulp mismatch is unmatched by bitwise comparison.
  - `ρ = 0` returns `None` (w = 0, correctly Passed). An overflowing w gives ∞, which demotes.
  - Tests cover the seeded family, the DOF map, both AngleDomain cases, unmatched, explicit and one-ulp slots, and the lateral joint.
- **No value change.** Only `quality` changes. Every demoting adapter test asserts bit-identical displacements and a report equal apart from quality. In the gate the 122 `results` are identical to main, and in the product flips above the results are identical.
- **No in-band marker.** `FormationCheck` is never rendered (`lib.rs:4429`), and the runtime test asserts that no "FormationCheck" appears in the message.
- **Nonlinear supports never selected.** `selected = built.nonlinear_supports.is_empty()` makes SA return `solve_assembled` unchanged. The loop reaches only `_binary64` targets (§5). Option B's matching runs only on the selected path.

## 5. Independent caller list (lexer scan, `callers/rv5_callers.py.txt` and `callers_rv5.txt`)

Comments and literals are blanked, and test code is marked. Non-test sites only:
- `FormationCheckedSystem` is built only by `StructuralSystem::with_formation_source` and `AssembledStructuralSystem::with_formation_source` (FK `structural.rs:102-121`). The only non-test caller is SA `solve_assembled_with_formation_check` (`structural_adapter.rs:403-405`). FK `solve_formation_checked_structural_dense` (`:1672-1678`) has test callers only.
- `formation_check::check` is called only from `finish_checked_factor` (`:1470`). `finish_checked_factor` is called only from `finish_structural` (`:1652`), which is the completion of every backend (FK dense `:1700`, SA `solve_prepared` `:566`, sparse_direct `structural.rs:26`).
- `solve_assembled_with_formation_check` has exactly one non-test caller: PP `solve_preview_reduced_system` (`lib.rs:4399`). That in turn is called from `solve_load_case` (`lib.rs:2708`).
- The nonlinear loop: `solve_linearized_system_evidence` calls `solve_binary64` (`lib.rs:1990`) and, in the test-only `assembly: None` branch, `solve_structural_dense_binary64` / `solve_structural_sparse_binary64` (`:2008`, `:2013`). It also calls `product_equilibrium::evaluate` (`:2034`, also `scrutinize_gaps` SA `:1311`), which reaches `evaluate_original_residual` (`product_equilibrium.rs:56`). None names a formation entry point.
- `AssemblyEvidence::new` (non-test): PP `:4376`, `source_recovery.rs:879` (no solve), and NI `lib.rs:600`. It now also clones the primitives.

This agrees in substance with I3's `callers.txt`, which is stale (RV5-S1).

## 6. Check 4: the gate (ROOT's combined-tree ruling)

- **The runs file.** I3's `<scratch>/kd5-i3/gate3/runs.jsonl` (sha256 `e365541b…`, HEAD `2409de83e`, probe `39791d93…`). The lists equal main's empty `GATE/S11_EXCEPTIONS.json` (`138515b3…`) and `GATE/FORMATION_EXCEPTIONS.json` (`0e110b4b…`), with references `7b176dbb…`.
- **My recount** (`gate/rv5_gate_recount.txt`, `cmp_runs.py.txt`):
  - 888 runs = 222 cases × 2 modes × 2 entries, none missing.
  - Timeouts: 600 s for 836 runs and 1800 s for 52.
  - Part 1: 884 runs. `part1_result.json` has 764 frozen-reference runs, 328 trusted, and **0 trusted breach triples**.
  - Part 2: the 4 dense cases RF-LARGE-CHAIN-n01000-ROT and RF-LARGE-TREE-n01000-AX, both entries. Each **timed out at 1800 s (not raised)** on a quiet host (load 0.46–1.27), and nothing was published. These match P1 on main.
  - **The union verdict is PASS.**
- **Attribution against main** (I3's probe built on `b24b3d536`, 836 small runs). Every envelope is byte-identical (sorted-key JSON) except 122 in all 4 runs. There only `diagnostics`, `numerical_quality` and `standing` differ; `results` and `source_block_recovery` are identical. On the captured entry 122 gains `SOURCE_BLOCK_RECOVERY_UNAVAILABLE`, which is main's real recovery attempt.
- **122.** The trigger is 4.8267 dense and 2.4279 sparse, and the actual error is 2.4134 and 1.2139 (`rust/base.kd5.log`), matching CHANGE_RECORD.
- **My own re-run** (`gate/sample.stdout.txt`). My probe build (`771900fe…`) and my independently regenerated requests (byte-identical to I3's `gen/out`). 34 cases × 2 modes × 2 entries: 122, 345, the six RF-CHAIN r1e-04 controls, S11-G's demoted INPLANE/UDL/LFRAME/WEAK cases, and 22 seeded-random cases. **136 of 136 envelopes are byte-identical** to I3's part-1 records.
- **Provenance.** Part 1 and part 2 ran from one runs file and one binary on the candidate head. My timed runs (§9) did not overlap part 2.

## 7. Evasions against the folded pins (check 6; full NI suite, `rust/E*.all_ni.log`)

| Evasion | Construction | Caught by |
|---|---|---|
| E1, alias or helper | The loop calls a neutral-named SA helper (`solve_binary64_audited`) that wraps the checked entry. The legacy call text is kept in an unreachable branch | **behavioural**: `kd5_nonlinear_loop_reaches_no_formation_check`, the two option-(c) loop pins, and two gap-inspection tests |
| E2, UFCS | `AssemblyEvidence::solve_assembled_with_formation_check(assembly, …)` in the loop | behavioural and source (`kd5_nonlinear_sources_name_…`, `option_c_nonlinear_loop_is_pinned…`) |
| E3, comment or string text | The helper call, with the legacy text only in a comment and a string literal | behavioural and source (the lexer strips both) |
| E4, sibling module on an unexercised path | See RV5-S2 | **not caught** (84/84 lib, 4/4 doctests, 18/18 nonlinear benchmark behavioural) |

## 8. Other standing checks

- **Composition with S11-G** (`fbc9661a4`):
  - `append_integrity_report` (`lib.rs:1068-1095`) takes its code from `report.quality`, which carries K-D5's kernel demotion. `formation_guard::demote` (`formation_guard.rs:481-489`) is a no-op unless the code is CHECKS_PASSED, so a K-D5-Sensitive case gets no guard sentence. R-b′'s amendment uses the same function.
  - Routing: `report_sensitive` reads `checked.report` (`lib.rs:2741`). D22-1's zero-work decline runs only when `needs_source_recovery(report_sensitive, attempt_err, None)` is false (`:2760`), so a K-D5-Sensitive case keeps main's real attempt, as confirmed in the gate.
  - `OrdinaryAttempt::passed` records `sensitive` from the report (`source_receipt.rs:459-482`).
  - Site tests: K-D5's rule-1 edit, T8 and T10b are intact (`s11f_site_test.rs` differs from base only in K-D5's three hunks). KERNEL gains `formation_check.rs` at the end, with indices unchanged. FORCE_FUNCTIONS gains `check` and `evaluate`.
  - FK, SA and `formation_check.rs` are byte-identical between `a89fde17b` and `2409de83e`.
- **The comment-only commit `2409de83e`.** It touches one file (`s11g_tests.rs`, T20's doc comment), and every changed line is `///`. The wording matches ROOT's corrected C1 text.
- **T9.** I3's hash lists (base `b24b3d536` against the candidate) are identical, 112/112 (`<scratch>/kd5-i3/a5/`). I did not rebuild the harness.
- **dead_code.** No module-wide or `cfg_attr` allowance remains. The FK non-test build is warning-free with the 13 per-item allowances. With the 13 removed, rustc reports exactly those 13 items (`rust/dc_without_allows.log`). Each carries a one-line reason.
- **Unchanged surfaces.** DEC-046, `benchmarks/nonlinear` and PP's other code are unchanged in the diff. Option (c) is intact (§5).
- **Records.** `IMPLEMENTATION/KD5/SHA256SUMS` verifies 126/126 with full coverage. No machine paths appear anywhere in the diff.
- **Formatting.** stable rustfmt 1.8.0 is clean on every changed `.rs` file. PP `lib.rs` is not clean at base either, and `kd5_models.rs` is generated with `rustfmt::skip`. `git diff --check` is clean on code.
- **Portability.** GEN-8: `pytest tools/practitioner_harness/test_live_baseline.py -k gen8` passes on `<wt>/numerics` with my untracked records present. It needs git history, so it cannot run on an archive; for the candidate I grepped every changed file for machine paths instead (none found).
- **Disclosure.** CHANGE_RECORD states the changing classes, no value change, 112/112, the cost per curved element (2,499 Wide operations), no in-band marker and the design note on M31a/M31b. Its M31b reasoning is refuted (RV5-B1), and it stops before S11-G (RV5-S3). RETURN §7 and §8 match the phase-1 records, and A4-2..A4-5 match the addendum-4 records.

## 9. Performance NOTE (the dense 1000-member slowdown)

**The question.** On the combined tree, the dense 1000-member runs took about 15 % longer than on the pre-S11-G tree (for example 490 s against 413 s). Does K-D5's check contribute?

- **Code reading.** Per case, the check costs:
  - one `Wide<2>` formation per element (a few hundred operations per frame; 2,499 per curved element);
  - one exact sum per free row over the element contributions (O(nnz) exact products);
  - one solve pair with the existing factor.

  That is well under a second, against a dense O(n³) factorization of n ≈ 6000 at 200–490 s. It runs only on a case that would publish Passed. K-D5's code (FK, SA and `formation_check.rs`) is byte-identical between the pre-S11-G tree (`a89fde17b`) and the candidate. The large cases' qualities did not change between those trees either, so K-D5 does identical work in both, and it cannot explain a difference between them.
- **Timed comparison** (`gate/timed.jsonl`, `timed.py.txt`). RF-LARGE-CONT-n01000-AX, dense, captured. The case is Passed, so the check runs. Alternating runs on a quiet host (load about 1): main `b24b3d536` (S11-G, no K-D5; I3's probe `12811c32…`) took 219.6 s and 221.9 s; the candidate (my probe `771900fe…`) took 204.8 s and 206.5 s. The `results` are byte-identical.

**Conclusion.** K-D5's cost is below the build and run-to-run noise. The slowdown is not K-D5's; it is S11-G's guard cost or host variation, which is not separated here.

## 10. What I ran

- Standard-library Python: the emulation, the regeneration of P1's requests, the gate recount and attribution, the caller scan, and the product-output checks.
- Cargo, one job at a time in my slot, with RUSTUP_TOOLCHAIN=1.97.1, CARGO_INCREMENTAL=0, `--offline --locked` and my own target under `<scratch>`:
  - NI `cargo test`, 14 variants: base, M23, M27, M31a, M31b, M31b0, M32a, M32b, E1–E4, with the `rv5_`, `kd5` or no filter;
  - `validation/benchmarks/nonlinear` under E4;
  - the FK build, with and without the per-item allowances;
  - two release probe builds.
- Probe runs: 136 gate-sample runs, 32 product runs from my builds (plus 12 through I3's candidate probe before my slot, as a binary with no cargo), and 4 timed runs.
- GEN-8 pytest on `<wt>/numerics`.

## 11. What I did not check

- I did not rebuild the T9 harness; I checked I3's hash lists instead.
- I did not re-run the full 888-run gate or the other suites; I recounted I3's records and re-ran a 34-case sample.
- I did not run the C2 scripts; K-D5 changes none of their inputs.
- I did not re-kill M26 or M28 (the brief asks for one of (26)–(28); I did M27).
- I did not build the authority targets: nothing I ran needed them.
- I did not run GEN-8 on an archive of the candidate (it needs git); I grepped instead.
