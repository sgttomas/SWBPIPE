# RV21: independent review of slice V-K

- **Reviewer:** RV21 (Type 2 TASK, independent reviewer). I did not write V-K.
- **PR:** [#1057](https://github.com/sgttomas/chirality/pull/1057), `codex/piping-vk-20260929`.
- **Head reviewed:** `3fd1baff33` (`3fd1baff335e7f62a78095499c83488c6ad387d8`). Base main `0f5d8c7b4` (K4 and KF1 merged); ROOT's merge of that main into the branch is `485320e95`.
- **Date:** 2026-09-29.
- **Verdict: PASS.** No BLOCKING finding. 2 SHOULD-FIX and 5 NOTEs.

**Paths.** `P/` = `projects/chirality-piping/`; `T3/` = the NUMERICAL_INTEGRITY_T3 folder; `FK` = `P/core/solver/frame_kernel`; `K4R` = `FK/src/structural/retained/`; `VR` = `P/validation/benchmarks/numerical_robustness/`; `R1` = `T3/REFERENCES/`. Code lines are at the head. My records are in `REVIEW/_run_records/vk_review/` (`RR/` below), with placeholders `<wt>` and `<scratch>`.

**In short.**
- **The harness cannot pass a wrong answer (priority 1).** I found no path by which a wrong kernel value on a covered row is counted as anything but a failure.
  - The exact engine is correct by reading. It agrees with my own Python-`Fraction` oracle on 225,405 fresh vectors: the predicate, the magnitude form, the floor comparison and the range test, with references from 10^-3000 to 10^310, subnormal observations and both sides of every boundary. There are 0 disagreements.
  - **My probe fed the harness 129,968 wrong answers through its own path** (published rows → `lane::observe` → `compare::judge`), on every one of the 25,321 covered rows of the 191 selected CI cases. Each value was moved by 3t either way, its sign flipped, the other end's bending substituted, the row deleted or overflowed, or set one ulp past the exact boundary. **Every one failed.** On 16,778 rows the last value inside the exact boundary passes and the next ulp fails.
  - Whole-case probes also fail as they must: a stable case joining the expected-unresolved list, THIN leaving it, a published mechanism, a mechanism marked stable, a wrong K4SRC, a wrong G, a reference moved by 3e-9, a lowered class scale, and a dropped not-covered entry.
  - **Two harness paths that R1's CI data never reaches are untested** (RV21-2). The code is right on both, but a regression would pass. My mutants RV21-H4 (an overflowed row observed as 0) and RV21-H6 (a sub-range reference passing as absolute-range whatever the observation) each survive VR's whole suite.
- **The adapter is faithful to R1 (priority 2).** My own adapter, written from R1's README and using my own integer rounding, reproduces every committed model field for field.
  - I checked it against `references.py --model` run as a subprocess on 191 cases, 2 of them RF-LARGE, and against `model_json(full=True)` in process on the other 22 RF-LARGE cases. On both sampled cases the in-process output equals the subprocess output.
  - The 12 large models' sha256 equal `large_models.sha256`.
  - All 27,752 rows, 715 controls and every scale equal `references.json` string for string. RF-CANCEL uses the recommended (binding) column.
  - `gen_vk_cases.py --check` regenerates all 16 files byte for byte, in 282 s.
- **The floor check and the lists re-derive exactly (priority 3).** My independent derivation, from `references.json` and not from `floor_kinds.json` or the case files, gives RF-WEAK 46, RF-CANCEL 3 and RF-SKEW 2. That set equals `not_covered.json` and every per-case list, on all 206 cases up to 1,000 members.
  - The expected-unresolved list is narrow: exactly THIN-A and THIN-B, pinned by id and by the family counts, and their reason is pinned through the committed records.
  - The three input-derived exemptions are pinned by name, and they require both `InputDerived` and a restrained DOF.
- **The seeded faults and FK (priority 4).**
  - Against main plus A0's patch, V-K's FK change is **insertions only**: 186 lines in 11 files, plus `seeded.rs`. Every inserted code line sits inside a statement or item gated by `#[cfg(any(test, feature = "mutation-controls"))]`, so the code is unchanged in effect with the feature off.
  - No site is in `factor.rs`'s five protected functions, in `bound.rs`'s shift loop, or on KF1's tracker lines.
  - I re-ran 7 of V-K's faults. Each is killed by exactly the tests the committed matrix lists. An unknown id panics.
  - **Two faults of my own** are also killed: the j-end axial force published 1 + 2^-27 high (6 lanes), and small displacements zeroed (5 lanes).
  - FK's full suite passes 402 of 402 with the variable unset, on a clean archive.
  - **The feature guard has one gap** (RV21-1): a manifest that enables VR's `seeded-faults` would transitively enable FK's `mutation-controls`, and the guard would not flag it.
- **The export (A0) is visibility only (priority 5).**
  - The patch-id equals `bb89e4f8f`'s.
  - All 260 removed lines are 240 `pub(crate)` and 20 dead-code markers, and every added line is their `pub` counterpart or the 29-line facade.
  - Nothing takes a matrix, factor, closure or label.
  - No product crate names `retained_api`.
- **B's runner exception is narrow and tested (priority 6).** Its 8 tests pass. B's 18 records are consistent with RETURN §14, and `summary.txt` regenerates byte for byte.
- **CI and the records (priority 7).**
  - CI discovers VR as its 40th manifest. VR's step took 46.6 s in the dispatch's numerical job (20 min 6 s, green on this head). Locally VR's 44 tests pass in 38.6 s.
  - All five `SHA256SUMS` files verify (400, 15, 12, 2 and 1 entries), each set-equal to its folder's files (in `cases/`, apart from the generator itself), and no machine path appears in the PR's 484 files.

## Findings

| ID | Class | Site | Evidence | Fix |
|---|---|---|---|---|
| RV21-1 | SHOULD-FIX | `VR/tests/feature_guard.rs:47` (`names_it`) and `:52-58` | **The guard misses the transitive way to enable the feature.**<br>For any manifest other than VR's, `names_it` looks only for the literal `mutation-controls`. Two lines would slip through: a product manifest's `piping_numerical_robustness = { path = "…", features = ["seeded-faults"] }`, or a feature `x = ["piping_numerical_robustness/seeded-faults"]`. Either enables FK's `mutation-controls` through VR's `seeded-faults = ["open_pipe_stress_frame_kernel/mutation-controls"]`, and `no_manifest_enables_the_mutation_controls_feature` stays green.<br>My self-test, appended to the file, fails on the head (`RR/guard/`).<br>**Nothing enables it today.** No manifest outside VR names `numerical_robustness` or `seeded-faults`, and CI passes no features. But this is the guard Scope 9 and §4.10 require ("no product manifest enables the feature"), and a dependency on VR is the likeliest way it would be enabled by accident, for example from V-P's product-side work. | Treat `seeded-faults` like `mutation-controls` in every manifest except as the key of VR's own `[features]` table: `let names_it = line.contains(FEATURE) \|\| line.contains("seeded-faults");`. Add the two lines above to `the_guard_flags_what_it_must`.<br>**Tested** (`RR/guard/`): with the fix, the self-test and the guard's four tests pass on the head. |
| RV21-2 | SHOULD-FIX | `VR/src/compare.rs:65` (the absolute-range verdict) and `VR/src/lane.rs:85` (`Overflow`); `VR/tests/engine.rs:38-65`; plan `PLAN_CHECKPOINT0.md:117` and `:154` | **Two failure paths are right in the code but pinned by no test, so a regression in either would pass wrong answers.**<br>Plan §6.4 promised that `engine.rs` checks, on R1's five sub-range rows, that "a normal value above 1e-9·scale fails; an `Overflow` fails". The delivered test only checks that ±0 passes and is counted apart, and nothing reaches `value_of`'s `Overflow` arm.<br>R1's CI rows reach neither path. The five sub-range rows belong to RF-LARGE-CONT-n10000, which runs only as an example (and gave 4 absolute-range passes at B), and no CI row overflows.<br>My mutants **RV21-H4** (`Overflow` observed as +0) and **RV21-H6** (`Some(_) if outside_binary64(exp) => PassAbsoluteRange`: any observation of a sub-range row passes) **each survive VR's whole suite**, 44 of 44 (`RR/mutants/`). Under RV21-H6, V3's absolute-range rows would pass whatever W1 published.<br>This is the same class of gap as C's VK-H6, H9 and H11, which ROOT closed with constructed tests. | Add the two constructed tests I drafted (`RR/scripts/rv21_proposed_tests.rs.txt`):<br>– `a_wrong_value_on_a_sub_range_row_fails`: ±2t and 1.0 fail on each of R1's five rows;<br>– `an_overflowed_published_row_fails`: an `Overflow` row, either sign, through `lane::observe`, is `Unavailable` and fails.<br>**Tested** (`RR/probes/proposed_tests_fixcheck.txt`): both pass on the head, and each kills exactly its own mutant (H4 and H6). |
| RV21-N1 | NOTE | `T3/IMPLEMENTATION/VK/RETURN.md:11`, `:32-39`, `:70`, `:492-494`; `CHANGE_RECORD.md:57` | **Parts of RETURN and CHANGE_RECORD describe an earlier head.**<br>– RETURN §1 says "The checked head is `485320e95` … The runner and README edits of §12.4 are not yet committed", and its commit list ends at `485320e95`. The later commits (`f94342a3d`, `64470c6ba`, `a4b8c1957`, `f5379a5d4`) appear only in CHANGE_RECORD.<br>– RETURN §2 gives "73 files and 41,086 lines". That is C's count, and only CHANGE_RECORD labels it so. The head's product-tree change is 83 files, +42,867 and −260 against main.<br>– CHANGE_RECORD "What changes" lists 8 test files and "43 tests in all", and RETURN §15 lists the suite as it was at C. The head has 9 test files and 44 tests (`tests/scale.rs`, added at B). RETURN §14 says "44 of 44", and my run passes 44. | Refresh RETURN §1's head and commit list, and label §2 as at C or give the head's count. List `scale` (1) among the tests, and state 44 at the head. |
| RV21-N2 | NOTE | `VR/cases/gen_vk_cases.py:420`; plan `PLAN_CHECKPOINT0.md:64` | **The plan's subprocess `--model` check was not implemented.** Plan §3 said the generator would also run `python3 -B references.py --model <id>` as a subprocess, on three cases (one of 10,000 members), and assert byte equality with the in-process JSON. The generator only calls `model_json(defn, full=True)` in process. That this is what `--model` prints rests on `references.py:2984-2994`, which I confirmed.<br>**I closed it:** 191 subprocess runs of `--model` equal the committed models under my adapter, and on two RF-LARGE cases the in-process JSON equals the subprocess output (`RR/probes/adapter_check.log`). | Record the deviation in RETURN, or add the three subprocess checks to `--check`. |
| RV21-N3 | NOTE | `VR/tests/files.rs:27-41`; `:194-209` | **CI's pin on the case files is self-referential.** The test checks the files against `cases/SHA256SUMS`, which sits beside them, so regenerating both together passes CI. Faithfulness to R1 rests on `--check`, which CI cannot run because R1 lies outside its checkout. Such a change would still show in review. ROOT had K6b's test assert `r1_large.txt`'s sha256 for this reason.<br>Also, `not_covered.json` is checked for inclusion in the case lists and for per-family counts, but not for set equality or duplicates. My re-derivation confirms the sets are equal. | Optional: assert `SHA256SUMS`'s own sha256 as a constant in the test, and assert set equality between `not_covered.json` and the per-case lists. |
| RV21-N4 | NOTE | `VR/runner/vk_scale_runner.py:218-232` | **The KF3 exception is keyed on size and outcome text, not on where the stop occurred.** It checks 10,000 members, `Unresolved ExactSumSpan`, no published row, and a single "not selected" failure, as ROOT ruled. It does not check that the stop is in the 256 verification's shared build. All five B cases stop there (`_run_records/b/runs/`, and KF3's diagnosis), so the exception covered exactly the finding. It is to be retired after KF3. | Optional, if it is kept after KF3: also require attempt 2 to be the verification, `Failed(Stop(Span))`, with no `uc` stage work. |
| RV21-N5 | NOTE | `VR/src/lane.rs:279-300` | **A below-floor structural-zero row would be counted twice in the bookkeeping.** `run_parts` pushes a row's key to `not_covered` whenever it is not covered, even when `judge` has returned `StructuralZero` (tallied as a structural zero). So the list and the tally could disagree for such a row. R1 has none: 51 listed and 51 tallied. It has no effect on any verdict. | Optional: skip the not-covered bookkeeping for `Target::StructuralZero`, or pin the list length equal to `tally.not_covered`. |

## 1. Can the harness pass a wrong answer? (priority 1)

### 1.1 The exact engine (`VR/src/exact.rs`, `compare.rs`), by reading

- **`Nat`** (`exact.rs:16-231`) is little-endian with u32 limbs, and every constructor and operation trims. Four points needed checking:
  - `mul` cannot overflow u64: acc < 2^32, and (2^32 − 1)² + 2·(2^32 − 1) = 2^64 − 1;
  - `sub` asserts self ≥ o, and trimming guarantees o has no more limbs;
  - `cmp_nat` compares length, then limbs from the most significant;
  - `from_decimal`'s 9-digit chunks keep 10^9 < 2^32.
- **`Exact`** represents ±m·2^p2·10^p10. The sign is normalized away at zero in `new` (`:252-260`), which every constructor uses.
  - `align` (`:329-355`) lifts both operands to the common minimum exponents, which is exact.
  - `from_f64` decodes subnormals as (fraction, −1074) and normals as (fraction | 2^52, biased − 1075).
  - `parse_decimal` accepts only sign, digits, point and exponent. It rejects "", "e5", "1..0", "1e", "--1" and "0x10".
- **The predicate** (`:466-473`) is |obs − exp| ≤ 10^-9·max(|exp|, scale), exact, and it never uses |obs| (VK-H6's test).
  - A negative scale string cannot widen the tolerance below |exp|. I probed this.
- **The magnitude form** (`:479-492`) is equivalent to |√(y² + z²) − exp| ≤ t for any sign of exp:
  - upper < 0 fails;
  - otherwise s ≤ upper² bounds it above;
  - lower ≤ 0 has no lower bound;
  - otherwise s ≥ lower².
- **`outside_binary64`** (`:496-504`) checks |exp| ≤ 2^-1075 (the tie rounds to even, 0) or |exp| ≥ 2^1024 − 2^970 (the tie rounds to ∞). Both boundaries are right.
- **`covered`** (`:508-512`) checks max(|exp|, scale) ≥ S\*·2^-34 exactly. S\* comes from the reference values only, so no observation can change coverage.
- **`judge`** (`compare.rs:47-69`), in order:
  - a structural zero requires exp = 0, else it fails;
  - a not-covered row is `NotCovered` and never a pass;
  - a missing, overflowed or unavailable row fails when covered;
  - an absolute-range pass requires the predicate to hold.
  - `Tally::accounted` and the pins cover every verdict.

### 1.2 The engine against my own oracle

`RR/scripts/rv21_engine_vectors.py.txt` is seeded and uses the standard library only. It writes 225,405 vectors, each decided in `Fraction`s. `RR/scripts/rv21_engine.rs.txt` checks VR's engine against them.

| Kind | Vectors | Holds / fails | Disagreements |
|---|---:|---|---:|
| predicate | 159,732 | 46,989 / 112,743 | 0 |
| magnitude (exact square bracketing, independent sign flips) | 43,749 | 15,074 / 28,675 | 0 |
| covered (S\* within 2 ulps of 2^34·max(\|exp\|, scale)) | 16,000 | 8,793 / 7,207 | 0 |
| outside binary64 (both boundaries ±, exact decimals of 2^-1075 and 2^1024 − 2^970) | 5,924 | 1,842 / 4,082 | 0 |

The references are random 1- to 40-digit decimals at exponents from −3000 to +310, in three spellings. The observations include fl(exp ± t), their ulp neighbours, ±0, 5e-324 and random doubles. The vectors' sha256 is `2c0db58e…` (26 MB, not committed; the generator is).

### 1.3 Wrong answers through the harness's own path (`RR/scripts/rv21_probes.rs.txt`)

For each of the 191 selected CI cases, I took the kernel's actual publication (`run_case(..).published`). For each covered row I then built mutated publications and judged the row through `lane::observe` and `compare::judge`, with the source rows mapped by my own restatement of plan §4.2 (so a mapping error in the harness would show as a miss).

| Mutation | Applies to | Result |
|---|---|---|
| the source value set so the observation is exp ± 3t (via k_t or k_a for twist and extension; (0.6w, 0.8w) for a magnitude) | every covered row | fails |
| the source value's sign flipped (where \|exp\| > 2t) | values, twist, extension | fails |
| the other end's (My, Mz) substituted (where R1's two ends differ by > 4t) | `Mb.i` and `Mb.j` | fails |
| one ulp beyond the exact boundary (the boundary value itself passes) | 16,778 direct values | fails |
| the source row removed; the source row `Overflow` | every covered row | fails |

**129,968 mutations of 25,321 covered rows: 0 missed.** The unmutated rows all pass, so the probe reads the same rows. Not covered (51) and structural-zero (282) rows are skipped, by design.

**Whole-case probes, through `run_case`:**

| Probe | Expected failure | Result |
|---|---|---|
| O1: RF-CHAIN-T-n03-r1e-04 renamed RF-RANGE-THIN-A (listed, but selected) | "on the expected-unresolved list, but Selected at 128" | fails |
| O2: THIN-A renamed THIN-Z (unresolved, not listed) | "not selected: Unresolved Ceiling"; all 25 rows fail, 0 expected-unresolved | fails |
| O2b: THIN-A itself | 25 expected-unresolved, 0 passes, no failure | as required |
| O3: a stable case marked `refuse` | "a mechanism was published (88 rows)" | fails |
| O4: RF-MECH-LINE122-TORQUE marked stable | "not selected: Refused MechanismWitnessed" | fails |
| O5: K4SRC digest altered | the sha line | fails |
| O6: member 2's G × (1 + 10^-6), K4SRC kept | the sha line and `tw.M2` | fails |
| O7: RF-SKEW-T-CANT-AX-122-r1e-12's largest rotation reference × (1 + 3·10^-9) | `th.N1.RY` predicate | fails |
| O8: RF-ZERO-SYM's translation class scale set to 1e-300 | 9 rows leave coverage; the not-covered set leaves the committed list (the lane test's FLOOR) | fails |
| O9: a committed not-covered entry dropped | set ≠ list | fails |

`rv21_judge_constructed` covers the verdicts that a kernel observation cannot reach:
- a structural zero with a nonzero reference fails;
- `Unavailable` is never a pass;
- a not-covered wrong value is `NotCovered`, not a pass;
- a sub-range reference passes absolutely at 0 and fails at 1e-300 over a 1e-295 scale;
- a negative scale string does not widen the tolerance;
- a magnitude 2t off fails, and a sign swap of both components passes.

### 1.4 Harness mutants of my own (`RR/mutants/`, one clean edit each, VR's committed tests only)

| Id | Mutant | Verdict | Killed by |
|---|---|---|---|
| NONE | – | 44 of 44 pass | – |
| RV21-H1 | floor comparison strict (>) | killed | `exact::tests::the_floor_is_r_times_s_star_exactly` |
| RV21-H2 | tolerance 1e-8 | killed | 10 tests (engine units and vectors, the controls, five lanes) |
| RV21-H3 | N and ext read from the i end | killed | 6 lanes |
| RV21-H4 | `Overflow` observed as +0 | **survived** | – (RV21-2) |
| RV21-H5 | a magnitude judged on \|My\| alone | killed | 9 lanes |
| RV21-H6 | a sub-range reference passes as absolute-range whatever the observation | **survived** | – (RV21-2) |

The proposed tests of RV21-2 kill H4 and H6 exactly (`RR/probes/proposed_tests_fixcheck.txt`).

## 2. The adapter (priority 2)

- **My adapter** (`RR/scripts/rv21_adapter_check.py.txt`) is written from R1 README §2, with my own integer round-to-nearest-even. That rounding is self-tested against CPython's on 20,000 random rationals and at the subnormal and tie boundaries.
  - Sections: A = PI_Q·(OD² − ID²)/4, Iy = Iz = PI_Q·(OD⁴ − ID⁴)/64 and J = 2I, each rounded once. G comes from `G`, or from E/(2(1 + ν)) exactly for THIN. On the represented basis, inputs are rounded first.
  - Springs: global-axis where exactly one direction component is nonzero, and directional otherwise. k = 0 is omitted.
  - Rigid DOFs; nonzero load components in R1's order, then the contributions; one station per member at 0.5.
  - y_reference: I check validity (a unit axis not parallel to the chord) and V-K's stated rule.
- **Result, 213 cases:**
  - 0 problems in nodes, members (ends and six section values, bit for bit), springs, constraints, loads, stations and omitted springs;
  - every `references.json` model equals `--model`'s where unabbreviated;
  - the 12 large models' sha256 equal `large_models.sha256` and each line's `model_sha256`;
  - 27,752 rows, 715 controls, the scales, `basis`, `refuse` and `needs_directional_spring` equal R1's;
  - RF-CANCEL's column 3 is the "recommended_scale (net-governed)" column.
- **Plan §4.4's differences from K4's adapter** are the only ones by construction: y_reference, the six mixed-node spring representations and K0's omitted spring. My adapter reproduces V-K's committed models exactly, and V-K's `compare_k4.json` reports 0 unexpected differences over 140 cases. I did not re-run the one-off K4 comparison.
- **`gen_vk_cases.py --check`:** all 16 files OK, 282 s, 0.93 GB peak RSS (`RR/suites/gen_check.log`).
- **In CI:** `every_ci_models_canonical_bytes_equal_the_generators_from_references_py_model` (201 cases) and C's VK-H10a–d show that path B does not share path A's code.

## 3. The floor check, the lists and the exemptions (priority 3)

- **My derivation** (`RR/scripts/rv21_floor.py.txt`) starts from `references.json`.
  - S(kind) is the largest |fl(exp)| of each of the four kinds.
  - L_b and the coupling follow §4.1.6.1 items 5 and 6, in binary64.
  - Variant F: k_t and k_a are fl(fl(a·b)/L). I compute them with the product rounded to 53 bits without range limits, which equals C1's pre-scaled form and the design's where that is defined.
  - Covered iff max(|exp|, scale) ≥ S\*/2^34 in `Fraction`s, with R1's class scales (the recommended column for RF-CANCEL).
- **Result** (`RR/probes/floor_rederive.log`): RF-WEAK 46, RF-CANCEL 3 and RF-SKEW 2 over 206 cases. That equals `not_covered.json` (51 distinct entries) and every committed per-case list, with 0 differences.
  - The lists are:
    - RF-WEAK-W-AX-rho1e-12: 36;
    - RF-WEAK-W-3D-rho1e-12: 9;
    - RF-WEAK-W-3D-rho1e-08: `tw.C`;
    - RF-SKEW-T-CANT-AX-122 and -345-r1e-12: `tw.M1`;
    - RF-CANCEL: F- and M-G1e80-GnG-ORTHO `u.N1.UY`, F-G1e80-GnG-INPLANE `Mb.M2.mid`.
  - The six 1,000-member cases are empty. The 10,000-member cases rest on B's records (C9: both sets empty on all 12 large models).
- **The expected-unresolved list** (`VR/cases/expected_unresolved.json`; `cases.rs:311-324`; `lane.rs:250-257`, `:310-313`) is keyed by case id. It is narrow:
  - `the_expected_unresolved_list_is_thin_a_and_thin_b` pins it to exactly those two;
  - every family's lane pins `expected_unresolved` and the outcome triple (RF-RANGE 50 and (30, 0, 2); 0 elsewhere);
  - a listed case that is selected or refused fails (O1);
  - a non-listed unresolved case fails (O2);
  - THIN's reason (`Unresolved Ceiling [Restrained]`) is pinned by the committed per-case record, which the lane compares.
  - No RF-LARGE case is listed, so no scale run can hide behind it.
- **The three input-derived exemptions** (`lane.rs:282-298`, pinned at `tests/lane.rs:156-160`):
  - exempt only when the published class is `InputDerived` *and* the DOF is in the model's constraints;
  - in RF-WEAK-W-AX-rho1e-12's committed model, N5 (the far fixed end) is rigid in all six DOFs, so rule 2a applies to `u.N5.UX/UY/UZ`;
  - any other case's list is pinned empty.

## 4. The seeded faults and the FK feature (priority 4)

- **FK's diff.** I built main `0f5d8c7b4` plus A0's patch (applied with `patch -p1`), then compared it with the head's FK.
  - V-K's own change is 197 `+` lines with 0 deletions: 11 file headers and 186 inserted lines, plus the new 126-line `seeded.rs` (`RR/vk_own_fk.diff.txt`).
  - I read each hunk. Every inserted code line lies in one statement or item under `#[cfg(any(test, feature = "mutation-controls"))]`: an `if` with an early return, a shadowing `let`, or the `mod seeded;` line. The rest are comments, and the feature table in `Cargo.toml` is inert unless enabled.
  - F06's hunk reuses the original block's `{` in the diff's alignment. Removing the gated `let` restores the original text.
  - **So with the feature off and outside `cfg(test)`, FK is its base code.** Panic messages' line numbers shift, and no output depends on them: FK has no `line!()`, `file!()` or `Location::caller`.
  - In `cfg(test)` the sites are present and inactive unless `FK_SEEDED_FAULT` names a fault. F10's site clones six lists whether or not it is active, which costs FK's test build time only.
- **Placement:**
  - `geometry_first` (`factor.rs:130`) lies outside `pivot_passes` (`:444`), `negative_pair` (`:462`), `factor` (`:505`), `solve_scaled` (`:630`) and `solve` (`:662`);
  - R28 is at the entry of `shift_needed` (`bound.rs:655`), not in `shift_schedule`'s loop (`:687-760`);
  - the `adaptive.rs` sites (`:403`, `:3050`, `:3100`) are outside KF1's tracker code (`:540-2117`).
- **The selector** (`seeded.rs:75-92`):
  - it reads `FK_SEEDED_FAULT` once through a `OnceLock`;
  - unset, empty or `NONE` means no fault;
  - an unknown id panics inside `get_or_init`, so it panics at every site reached and never runs as NONE.
- **Re-run** (`RR/faults/`): the feature on, one build, NONE first. Each fault's failing tests equal the committed `kill_matrix.jsonl` row.

| Fault | Failing tests (mine) | Matches the committed row |
|---|---|---|
| NONE | none; 44 pass | yes (43 then; `tests/scale.rs` added at B) |
| VK-F13 | `rf_cancel` | yes |
| VK-S2 | 9 lanes (all but `rf_chain`) | yes |
| VK-F17 | all 10 lanes | yes |
| VK-F06 | `rf_range` | yes |
| VK-F04 | 10 lanes, the invariance map and the RF-INVARIANCE permutation | yes |
| VK-R02 | 8 lanes | yes |
| VK-F99 (unknown) | 17 tests; the panic "unknown fault id" | as required |
| **RV21-A** (mine): the j-end axial force × (1 + 2^-27) | `rf_chain`, `rf_skew`, `rf_weak`, `rf_zero`, `rf_finite`, `rf_large` | killed |
| **RV21-B** (mine): displacements below 1e-6 of their body's largest published as 0 | `rf_cancel`, `rf_range`, `rf_skew`, `rf_weak`, `rf_large` | killed |

- **FK's full suite** on a clean archive without the `execution/` tree (as CI checks out), with the variable unset: **402 passed, 0 failed**, with no warnings. The lib has 336 tests in 765 s, including K4's tests and its source scan. The seven integration binaries and the doc tests have 66, including the S11 site table (3 of 3). This equals V-K's post-KF1 count (`RR/suites/fk_suite.log`).
- **The feature guard:** RV21-1. `ci_passes_no_cargo_features` holds, and no workflow in `.github/` passes `--features`.

## 5. The export (A0) (priority 5)

- `git patch-id --stable` gives `b43efeb4…` for `3018343c2`, `bb89e4f8f` and the extracted FK patch alike.
- Over 11 files, the 260 removed lines are 240 `pub(crate)` and 20 `#[allow(dead_code)]` markers, each alone on its line. The 269 added lines are the 240 `pub` counterparts and the 29-line facade (`RR/a0_check.txt`).
- **The public functions take plain data only:** coordinates, scalars, `SourceParts`, `&PrimitiveSource`, `&[Vec<usize>]` (RCM's adjacency), layouts and `Binary64Outcome` values, or `&RetainedSolve` operands of a combination.
  - There is no generic, closure, matrix, factor or label parameter.
  - `PrimitiveSource`, `RetainedSolve`, `CaseLimit` and `InvocationMeter` keep private fields. `PrecisionState` stays crate-private.
  - The method, policy and rcond labels are exported only as constants.
- **The product scan:** outside `execution/`, `retained_api` is named only by `FK/src/structural.rs` and VR's files.

## 6. B's runner exception and records (priority 6)

- `availability_exception` (`vk_scale_runner.py:218-232`) applies only when all of these hold:
  - 10,000 members;
  - the outcome starts with `Unresolved ExactSumSpan `;
  - pass, absolute-range, not-covered and structural-zero counts are all 0, and fail = rows;
  - exactly one failure line, the "not selected" one.
- Every other check still applies under it (`:245-268`), including RCM, counts, the not-covered set, C9, class mismatches and controls.
- Its 8 tests pass (`RR/suites/runner_tests.log`). The edges are covered: size, reason, extra failure, published row, other checks, process failures, and a selected case. The locus is NOTE RV21-N4.
- **B's 18 records** (`_run_records/b/runs/`) are consistent with RETURN §14.1:
  - the 12 cases at 100 and 1,000 members are selected at 128, pass every row, and have empty C9 sets at 1,000;
  - CONT-n10000-AX: 211 passes and 4 absolute-range passes;
  - the five others: `Unresolved ExactSumSpan [Restrained]`, one failure line each, rows equal to `unresolved_availability`;
  - RCM is equal and the counts match storage on all 18.
  - `summary.py` regenerates `summary.txt` byte for byte.

## 7. CI and the records (priority 7)

- **Discovery:** `check_release_readiness.discover_cargo_manifests()` on the archive gives 40 manifests, VR's among them.
  - The dispatch run `36640222412`, job "Numerical cargo suite", is green on `3fd1baff3` and took 20 min 6 s against the 45-minute budget. VR's step ran from 22:52:24.8 to 22:53:11.4Z: 46.6 s, build included (`RR/suites/ci_numerical_vr_excerpt.txt`).
- **Local** (Mac, debug, `-j 4`, `RUST_TEST_THREADS=2`): the fresh build takes 4.1 s. The 44 tests pass in 38.6 s wall; the binaries sum to about 36 s (lane 18.1, parity 10.9, adapter 3.8).
- **The `SHA256SUMS` files:**
  - `T3/IMPLEMENTATION/VK/`: 400 of 400, set-equal to the folder's files;
  - `VR/cases/`: 15 of 15, set-equal apart from `gen_vk_cases.py` itself;
  - `observations/kernel_lane`: 12 of 12;
  - `observations/seeded`: 2 of 2;
  - `observations/harness`: 1 of 1.
- **Machine paths:** none in any added line of the PR's 484 files, for home, temp, `/var/folders`, `/opt/homebrew`, `.rustup` or `.cargo/registry` paths, or the user name.

## 8. What I read, ran and did not do

- **Read.** sha256 prefixes where they matter; T3 records at the numerics head, the code at the V-K head.
  - Root `AGENTS.md` (`c8ce87ef`), `agents/AGENT_TASK.md` (`1a13a5b0`), `_COMMON.md` (`6bb845bf`) and `I8R_K1_RESUME.md:24-50`.
  - `I17_VK_IMPLEMENTATION.md` (`06b16b82`).
  - `DESIGN.md` (`fb62ef4a`): §4.10, §4.1.6 and §4.1.6.1 in full, and §7.3.
  - `ROOT_RULINGS_V1.md`, every V-K and A0 section named in my brief, "V-K: rulings on I17's checkpoint-0 plan" through "K6b: slot K6B-S4 (b3) accepted", read at numerics `382ab0943`. ROOT committed two sections during the review (`b60a2022a`, sha256 `00fdad11`): KF3's plan and A2, and K6b's D. I read both. Neither changes a V-K ruling. KF3's decision 5 may reuse V-K's `seeded` module.
  - On the V-K head: `PLAN_CHECKPOINT0.md` (`adaf697a`), `RETURN.md` (`2412f404`), `CHANGE_RECORD.md` (`a131e68e`), `_run_records/` (B's runs, summary and setup; the a2 and kf1_merge kill matrices) and `SHA256SUMS` (`7fe93418`).
  - R1: `README.md` (`5a89bad9`), `references.json` (`7b176dbb`) and `references.py` (`80d473a7`), read-only (`main`, `model_json`, PI_Q).
  - VR's whole source: `src/` (all 12), `tests/` (all 9), `examples/vk_scale.rs`, `cases/gen_vk_cases.py`, and `runner/vk_scale_runner.py` with its tests.
  - V-K's FK diff in full and A0's patch in full; `seeded.rs`; and the surrounding code at each site.
  - `check_release_readiness.py` (discovery).
- **Ran.** Each from clean `git archive` copies of `3fd1baff3` under `<wt>/rv21/`, with targets `<wt>/rv21-target`, `<wt>/rv21-mut-target` and `<wt>/rv21-fk-target`. The host rules applied: one cargo job at a time at `-j 4`, `RUST_TEST_THREADS=2`, `RUSTUP_TOOLCHAIN=1.97.1`, `--offline --locked`, and the memory guard running (no KILLED line). Python ran as `python3 -B`, standard library only.
  - VR's suite: 44 of 44.
  - My probes: `rv21_probes` (3 tests), `rv21_engine` (225,405 vectors) and the proposed tests.
  - The seeded-fault subset and my two faults, in the first archive. I added my own two sites to that archive's FK copy only, after VR's suite had run there. They are cfg-gated like V-K's, so they are absent from the no-feature builds used later by my probes and mutants.
  - FK's suite ran on the second, clean archive.
  - Six harness mutants of my own, each edit reverted and checked against the head.
  - FK's full suite.
  - The guard probe and its fix.
  - `gen_vk_cases.py --check`; my adapter check (191 subprocess `--model` runs, 162 s); my floor derivation; the runner's tests; `summary.py`.
  - `gh pr view`, `gh pr checks` and `gh api …/jobs/…/logs`, all reads.
- **Git.** Reads only: `rev-parse`, `log`, `diff`, `show`, `archive`, `patch-id` and `grep`.
  - One `git apply` (working tree only, no `--index`) ran in a scratch folder under `<wt>/rv21/`. Git resolved it to the enclosing repository and, as it does for paths outside the current directory, applied nothing. That repository's status was clean afterwards, and I used `patch -p1` instead.
  - No commit, index, stash, checkout, merge, fetch or push, and no GitHub write.
  - One slip: a file list was written briefly to the system temp directory instead of the scratchpad, and deleted at once.
- **Not done.**
  - No scale run, and nothing above 100 members apart from the generator's in-memory builds and the adapter check's model adaptation.
  - No re-run of V-K's other 8 seeded faults or its 18 harness mutants. Their committed results stand, and my subset matches.
  - No one-off K4 adapter comparison, no DEC-025, T9 or GEN-8.
  - No timing claim: the times above are observations.
- **Cleanup.** My copies (`<wt>/rv21/`) and targets are deleted after the records are written. The 26 MB vector file is not kept; its generator and sha256 are.
