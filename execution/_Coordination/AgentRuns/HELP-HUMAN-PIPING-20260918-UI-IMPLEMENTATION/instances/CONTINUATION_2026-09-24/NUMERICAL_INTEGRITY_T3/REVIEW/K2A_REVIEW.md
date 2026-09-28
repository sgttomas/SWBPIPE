# RV7: independent complete-diff review of slice K2a

**Verdict at head `79c0d320b`: NOT PASS.** There is 1 BLOCKING finding (records only), 3 SHOULD-FIX findings and 5 NOTEs. The code is correct. The final verdict on the closing head is in §8.

**Reviewer.** RV7 is a Type 2 TASK, briefed by the T3 manager to review PR #1032.
- **Independence.** I did not design, check or implement K2a, K-D5 or M03. This is not owner review.
- **What I changed.** I made no Git writes and no edits to the K2a worktree.
- **Builds.** I built only from `git archive` extractions (80290ce98, and 5ae22926e for one main probe) in `<scratch>`, in the cargo slot the manager granted after ROOT's DEC-025 sweep had exited. The target `<wt>/rv7-target` was deleted afterwards.
- **Run records:** `_run_records/k2a/` (README.txt lists every run, including the three superseded ones).

## 0. Revisions reviewed

| Item | Value |
|---|---|
| Candidate | PR #1032, head `79c0d320b`; parents `80290ce98` (I6's K2a, on `5ae22926e`) and main `649162522` |
| Diff | `5ae22926e..80290ce98`: 166 files. Code and tests: FK `lib.rs` +119 −13, diagnostics `lib.rs` +49, and three new test files (678, 222 and 503 lines) |
| Merge check | The K2a file set and the merge's file set do not overlap. `git diff 649162522 79c0d320b` is exactly K2a's file set, and every K2a blob is unchanged by the merge. F1a touches PP only in the integrity evidence line and the ground-DOF message, and K2a's tests don't depend on either |
| Test-file pins (80290ce98) | FK `tests/k2a_checked_formation.rs` 54383afe…; FK `tests/k2a/rf_range_models.rs` f4a6f6aa…; PP `tests/k2a_formation_range_runtime.rs` 18ec1d5e… |
| Basis | DESIGN.md 5a.2 (§4.7 step 1, the §6 K2a row, §7.3 mutation 18); ROOT_RULINGS_V1 (the S11 decisions, "K2a product reach" and corrections 1–3); I6 brief with addenda 1–3 |

## 1. `local_stiffness`: correct
- **Operation order.** Compared line by line with main (FK :711-726 at 5ae22926e), every coefficient uses the same binary64 operations in the same order: L², L³ = L²·L, (E·A)/L, (G·J)/L, ((k·E)·I)/Lⁿ.
  - k·E is formed once and shared by the y and z coefficients.
  - There are 26 checks.
  - Every accepted coefficient has exactly main's bits. The bit-identity tests show this, and mutants R6 and R7 below (reorderings) are killed.
- **Zero operands.** `checked_formation_value` exempts zero and non-finite operands. Because `validate_positive_finite` runs first, no such operand reaches a check, and today's handling is unchanged.
- **Name.** `NumericalRange { name }` is as the design names it.
- **Subnormal input.** A subnormal input is refused only through a non-normal intermediate. This matches D1 §4.7 step 1, which checks intermediates of nonzero finite operands, and ROOT's ruling 3, which routes the missing lower bound on inputs out of T3.
  - For E, the k·E checks leave only E in [2^-1023, 2^-1022) passing, which is benign.
  - For derived section values, see N1.

## 2. Diagnostics: correct
- **Mapping.** `NumericalRange` maps to `InvalidNumericInput`, `Blocking`, `ModelValidation`, identical to `NonFiniteInput`. No new code is added.
- **My lexer scan** (`rv7_match_scan`, over core, validation and apps at 79c0d320b) finds exactly two exhaustive matches: FK Display and `diagnostic_from_frame_error`. Both gained the arm. It also finds FK :2035, which has a catch-all. This agrees with `callers.txt` §B.

## 3. RETURN §5 (product reach): every step checked

### Steps that follow
- **The lifts (§5.2):** L > 2^-39.863; zero bounds 2^-1035.14, 2^-995.27 and 2^-955.41; subnormal bounds 2^-982.14, 2^-942.27 and 2^-902.41.
- **The coefficient lifts** against the code: EA, GJ, 4EI and 2EI over L; 6EI over L²; 12EI over L³.
- **The M03 floor:** g ≈ 2^-48.415, bound ≈ 2g|c|, floor 2^-974.585.
- **Springs:** the floor does not bound springs.
- **The 2/L relation:** fl(12E) = 2·fl(6E).
- **reach_zero:** (12E)·I = 0.90·2^-1075 → 0; intended value 3.698e-289; u = 250 mm against 125.031 mm, 99.95 %.
- **reach_lef:** 1.35·2^-1075 → 2^-1074, siblings 0; 8.209e-289 against 5.547e-289; 249.72 mm against 369.55 mm, 32.4 %.
- **"Not established"** is stated for the pre-K-D5 standing.
- **K-D5** is stated as an estimate-based trigger.

### B1 (BLOCKING, records only): M03's element-entry floor is an axis-aligned argument
- **Sites:**
  - RETURN §5.3 :155 ("for a rotated element … of the same order");
  - §5.4 :164-166, the right-hand column of the 1/L, 6EI/L² and 12EI/L³ rows;
  - §5.7 :192, as far as it restates them;
  - CHANGE_RECORD, "Partial underflow with a nonzero subnormal-derived sibling: main's M03 already refuses these".
- **Why it fails.** `transform_roundoff` bounds each global entry through T^T|K|T. On a skew member:
  - the rotational-block bounds mix GJ/L with 4EI/L and 2EI/L;
  - the coupling block contains only 6EI/L² terms.
- **Confirmed with FK's real `transform_roundoff`** (`fk_base3.log`). Setup:
  - main's pre-K2a local matrix, formed operation for operation, and FK's own orientation;
  - pipe OD 1e-11 m, wall 1e-12 m, G = 1e-100 Pa, L = 2^-39 m;
  - relative errors against E scaled by 2^600.

| (12E)·I | E (Pa) | 6EI/L² | 4EI/L | 2EI/L (rel err) | axis x / yref +y | (1,1,1) / yref +z | (1,2,2) / yref +x |
|---|---|---|---|---|---|---|---|
| 2^-1030 | 2.4992e-266 | 2^-953.00 | 2^-992.58 | 2^-993.58 (1.1e-13) | refused | **accepted** | **accepted** |
| 2^-1040 | 2.4407e-269 | 2^-963.00 | 2^-1002.58 | 2^-1003.58 (1.16e-10) | refused | **accepted** | **accepted** |
| 2^-1045 | 7.6271e-271 | 2^-968.00 | 2^-1007.58 | 2^-1008.58 (3.73e-9) | refused | **accepted** | **accepted** |
| 2^-1050 | 2.3835e-272 | 2^-973.00 | 2^-1012.58 | 2^-1013.58 (1.19e-7) | refused | **accepted** | **accepted** |
| 2^-1055 | 7.4483e-274 | 2^-978.00 | 2^-1017.58 | 2^-1018.58 (3.8e-6) | refused | refused | refused |
| S6a: 2.5·2^-1075 exact | 1.7758e-279 | 2^-996.00 (rel 0.60) | 0 | 0 | refused | refused | refused |

- **Notes on the table:**
  - Every refusal is `Range("arithmetic outside normal range")`.
  - On the accepted skew members, the least rotational-block bound is 2^-492.47 on (1,1,1) and 2^-494.05 on (1,2,2), and the least bound overall is in the coupling block. On (1,1,1) that is 2^-1001.21 … 2^-1021.21 down the rows; on (1,2,2), 2^-1002.00 … 2^-1022.00.
  - In S6a, (12E)·I = (6E)·I = 2^-1074, and 12EI/L³ is 2^-957.00, 20 % wrong.
- **What it shows:**
  - On skew members M03 accepts subnormal-derived 4EI/L and 2EI/L below the floor, with errors up to 1.2e-7 here.
  - 6EI/L² is the limiting coefficient: once it falls below the floor, skew members are refused too.
  - A subnormal-derived 12EI/L³ dominated by EA/L in the translational block was not probed.
- **Main's product route** (archive of 5ae22926e; `pp_main_skew3.log`):
  - Model: one member, N0 anchored, N1 free, moment RZ = 0.1·EI/L, cases 2^-1045 and 2^-1050, axis and both skew members, both modes, captured and typed entries.
  - All 24 runs are `NUMERICAL_INTEGRITY_UNRESOLVED` and publish no result.
  - The axis member is refused by M03 `Range`.
  - The skew members pass M03 and are then refused by "nonpositive or cancellation-unresolved structural pivot", at global DOF 11 (sparse) or 10 (dense). GJ/L exceeds bending by about 10^170 here.
  - That main refuses every skew case downstream is **not established**.
  - On the candidate, all 24 runs are refused by name at `12EIy/L^3: (12*E)*Iy` (`pp3.log`).
- **The same assumption in ROOT_RULINGS_V1 (d7525ac60):**
  - correction 2 :801, :806, :823 (a probe statement, axis-aligned), ruling 3 :833, and :834 (true, but not exclusive on skew members);
  - correction 3 :843 ("element entries only" holds per entry only for axis-aligned members);
  - correction 1 :765 and :767 are probe-specific and correct as scoped.
- **Closure:**
  - A records addendum scopes §5.3, §5.4 and the CHANGE_RECORD bullet to axis-aligned members.
  - It states the skew figures above and main's product outcome for this probe.
  - It records that K2a refuses all of these at formation.
  - ROOT_RULINGS_V1 gets in-place pointers at the sites listed.

### S1 (SHOULD-FIX): RETURN §5.4 :166
"12EI/L³ ≤ 2^-954.4 is normal, above the floor" holds only for L ≤ about 2^-33.14 m. For L in (2^-33.14, 2^-17.33) m it is normal but below the floor, and M03 refuses it on axis-aligned members.

### S2 (SHOULD-FIX): the entries claimed for main's gap-route standing
- PP test :421-423 and RR correction 2 claim main's standing on "both entries".
- `main_zero_probe` and `main_lef_probe` are captured-entry only (NOTES.txt :1-3; RETURN §5 :105).
- The addendum must say so.

### S3 (SHOULD-FIX): RETURN §3 :66 cites a missing record
- It cites `_run_records/phase1/pp_k2a_4.log`, which does not exist and is not in SHA256SUMS.
- The final PP test file (18ec1d5e…) passes 3/3 in RV7's run on the 80290ce98 archive (`logs/pp2.log`) and in CI on 79c0d320b.

## 4. Tests
- **§5.8** maps the four cases correctly. Each asserts, on both entries and in both modes, no results plus one blocking `SOLVER_SYSTEM_BLOCKED` with the named reason.
- **The gap route's UNRESOLVED standing** is cited, not pinned. That is acceptable under addendum 2 and disclosed in RETURN §3 and §5.8; the entry wording is S2.
- **Kernel tests:** the paths-differ preconditions hold. The test copy of main's layout matches main. `k2a_models.py` regenerates `rf_range_models.rs` byte-identically, and its JSON matches.
- **RV7's mutants** (`rv7_mutants.py.txt`; FK k2a kernel tests; the NONE control passes 10/10):

| Mutant | Result | Killed by |
|---|---|---|
| R1: accept a subnormal only at the 12EI/L³ quotients | **survived** | – |
| R2: only at E·A, (E·A)/L, G·J, (G·J)/L | **survived** | – |
| R3: only at L³ | **survived** | – |
| R4: only at (12E)·Iz | **survived** | – |
| R5: only at 12·E | **survived** | – |
| R9: accept a subnormal at every product | killed | per-site table; both controls |
| R5b: check numerator·(1/L), return the quotient | **survived** (boundary only) | – |
| R6: reorder 6EIz as 6·(E·Iz) | killed | RF-RANGE byte identity; 1,683-case bit identity |
| R7: 12EIz as (12E)·(Iz/L³) | killed | per-site table; both bit-identity tests |
| R10: also refuse exactly 2^-1022 | **survived** (availability only) | – |

- **Why R1–R5 survive (N2):** the per-site rows for L³, E·A, (E·A)/L, G·J, (G·J)/L, 12·E, (12E)·Iy, the 12EIy/L³ quotient, (12E)·Iz and the 12EIz/L³ quotient produce zero or infinity, not a subnormal (`rv7_site_kinds`).
- **ROOT's ruling:** close them in this PR with per-site subnormal rows, plus boundary rows for R5b and R10. RV7 re-runs the survivors on the new tree (§8).

## 5. Evidence
- **SHA256SUMS** verifies: 160 entries, covering exactly the K2A tree.
- **Provenance:** hashes equal FK `lib.rs` 7622e7cc and diagnostics `lib.rs` a3feef13 at 80290ce98.
- **Gate:**
  - 768 runs in `final_result`, 328 trusted, 0 breaches;
  - 884 runs in part 1 and 4 in part 2;
  - RV7's rerun of `vs_main.py` against KD5's combined result shows 0 changes.
- **T9:** base and candidate are identical (112/112), and base equals KD5's combined candidate (112/112).
- **Mutation logs:** 31 in total. NONE passes 10/10; the other 30 each fail at a behavioural assertion.
- **Hygiene:** no machine paths or model identifiers. The only hit is a "model_id" test string in a suite log.

## 6. NOTEs
- **N1: subnormal derived sections.** A subnormal derived section value passes K2a whenever every intermediate is normal. For example, OD 5e-81 m and wall 5e-82 m give I = 1.98e-323 with 9.1 % error from OD⁴ − ID⁴ in `derive_pipe_section`, and E = 9e24 Pa keeps every intermediate normal. CHANGE_RECORD Limits' "the user's exact value" is wrong for derived A, I and J. Add this to ROOT's routed input-validation finding.
- **N2: per-site subnormal branch** not pinned (§4); closed by ROOT's ruling in this PR.
- **N3: typed LEF-large message.** RETURN §5.1 :122 and §9 :307 cite gate runs for it, but the committed gate records carry no messages.
- **N4: wording.**
  - RETURN §5.2 :147: at |p| = 2^-1074, 2^-1075/|p| is 50 %; the 100 % is relative to the exact value.
  - §5.2 :148: for L ≥ 2 m, a zeroed 6EI/L² is below 2^-1077, not 2^-1078.
  - The cost sentences keep correction 1's superseded "where main's value was accurate".
- **N5: incomplete "killed by" lists.** RETURN §6 :233 and :236 omit one killing test each, against MUTANTS.txt.

## 7. What I did not do
- Hosted CI and the DEC-025 sweep are not mine; the manager reports both green on 79c0d320b.
- I ran no full suites.
- I did not probe a better-conditioned skew model on main.

## 8. Final verdict
Pending. This section records the delta check (`git diff --stat 79c0d320b <new head>`, expected to touch only FK `tests/k2a_checked_formation.rs` and K2A record paths), the re-run of R1–R5, R5b and R10 on the new tree, and the verdict on that head.
