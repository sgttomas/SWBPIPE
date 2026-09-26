# VERIFY_R5 — V1 targeted verification of D1 5a.1 and D2 5b.1

**Reviewer:** V1 (Type 2 TASK, independent design reviewer), for the T3 WORKING_ITEMS manager.
**Brief:** `T3/TASK_BRIEFS/V1_TARGETED_VERIFICATION.md` (commit `43409a1e8`), under ROOT's process ruling (`ROOT_RULINGS_V1.md`, BACKCHECK_R5): verify only the listed fixes and R5-4's four conditions. This is not a full review round, and settled design is not reopened.
**Date:** 2026-09-26.

**Basis (final, as the manager stated it):**

| Design | Revision | Commit | File (sha256 prefix) |
|---|---|---|---|
| D1 | 5a.1 | `8dfd72921` | `DESIGN_NUMERICS/DESIGN.md` `13c1a7d5`; `S11_CONTAINMENT.md` `82c1b072`, `D5_TRIGGER.md` `f6e24a69`, `R5_4_CURVED.md` `4843c4e9` (unchanged since `16bcbf369`) |
| D2 | 5b.1 | `ae2333a80` | `DESIGN_STANDING/DESIGN.md` `71bb7278` |

The worktree later moved to `f5a315280` (RF-ELOAD and selection-package commits). `git diff 8dfd72921 f5a315280` touches no file under `DESIGN_NUMERICS/`, `DESIGN_STANDING/` or `GATE/`, and both design hashes above are unchanged at that head.

The line numbers below (`D1:n`, `D2:n`) refer to those two files at the basis commits.

## Verdict

**PARTIAL (not VERIFIED).**
- All four R5-4 conditions hold, and I reproduced them with my own independent re-formation.
- Every listed fix is verified except one: D1's mirror of ROOT's nonlinear-support rule.
  - D1's operative rule is correct (D1:30, D1:558, D1:587, D1:655).
  - Two stale sentences still contradict it: D1:166 and D1:720.
  - D1:720 is in §4.6, a normative boundary section. It says "At the facade, D1 refuses every model with a nonlinear support record", which is exactly what ROOT ruled out ("must never refuse the solve").

The fix is a two-sentence text alignment, with no design change:
- **D1:166.** Change "Every excluded family is refused per case…" to exclude nonlinear and contact supports, which are not selected (§4.3).
- **D1:720.** Change it to "At the facade, a case with a nonlinear support is not selected (§4.3); its ordinary result and standing are unchanged."

After that edit, the verdict becomes **VERIFIED**. A diff of those two lines is enough to confirm it; no further review round is needed.

## Items

| # | Item | Result | Evidence | Site |
|---|---|---|---|---|
| 1 | **R5-1**: B's reaction rule (a restrained DOF in any family set of a member-, pressure- or state-loaded member, or in the axial family of an eigen-strained member, is not provable) | **VERIFIED** | Both designs carry my fix verbatim in substance. D2 adds the negative control (a loaded fixed-fixed member's reaction listed; the uniform-axial-eigenstrain variant as well). I reran my `r5_backcheck/probe_r5_b.py.txt` against `b_proof.py` at `8dfd72921`. Every one of the four earlier false claims now reads `false`: M-UDL anchor Fz and My, and M-EIG anchor A and B Fx (`verify_r5/probe_r5_b_on_5a1.stdout.json`). I reran `b_proof.py` from `P/fixtures/product_preview`. The output is byte-identical to the committed `b_proof_main.json` (`4a4a8603…`), so the proven counts across the 48 case-modes are unchanged against my R5 baseline | D1:33, D1:1216, D1:1229; D2:875, D2:914; `DESIGN_NUMERICS/_run_records/b_proof.py` (`cc3b95c2`) |
| 2 | **R5-2**: `input_derived_dofs` is exactly the kernel's non-free DOFs, under PP's family rule | **VERIFIED** | I checked the rule against the merged-tree source. Each clause matches `PP`: <br>• the filter at `PP:5168-5230`: nonlinear skip `:5173-5177`, constant effort `:5178`, spring or variable hanger `:5209`; <br>• `rigid_linear_support_from_preview` `:5279`; <br>• `support_hanger_type` `:10229-10237`, `is_variable_spring_hanger` `:10239`, `is_constant_effort_support` `:10246`; <br>• `boundary_motion` on already-rigid DOFs, `case_state/resolve.rs:1025-1045`. <br>Two details match too: the hanger type is trimmed, with the family as fallback, while the family comparison with `"spring"` is untrimmed. `withheld_rows.py` (`1428ae57`) reruns byte-identical to `withheld_rows_merged.json` (`f8e22724…`) | D1:34, D1:476 (rule 2a); D2:574-582 |
| 3 | **R5-3**: S11 caller list gains `product_equilibrium::evaluate` (KS3) and `linear_supports::apply_linear_supports` (KS2) | **VERIFIED** (NOTEs 1, 2) | Both callers are listed and classified. I compared the list with I1's complete lexer list (`IMPLEMENTATION/S11K/_run_records/fixture_diff/ks_callers.txt` and `PRE_REGENERATION_REPORT.md` §2 at `14354efdb`). Every M03 route is covered; D1's bullets omit only the zero-prescribed `reduce_system` callers in the benchmark and harness code, and I1's list is the authoritative enumeration | D1:35; `S11_CONTAINMENT.md` §8.1 |
| 4 | **R5-5**: gate condition 3 gains the check-level comparison | **VERIFIED** (NOTE 3) | D1 states the rule. D2's form is stricter ("undecided" includes withheld-row refusals, not only `RULE_RESULT_INDETERMINATE`), and ROOT accepted it (D2 5b choices, item 1) | D1:36, D1:694; D2:390-393 |
| 5 | **N-1**: b = fl↑(2^-64·S\*); b = 0 only when S\* = 0 | **VERIFIED** | Both designs state it. I probed D2's decision algorithm (r = fl(S\*·2^-64); b = r if r·2^64 ≥ S\*, else next_up(r)) over 25,008 samples: the boundaries 0, 2^-1074, 2^-1022, 2^-958±, the maximum finite value, 20,000 random magnitudes across the whole exponent range, and the first 4,999 subnormal multiples. Checked exactly against `Fraction`, b is always the least binary64 ≥ 2^-64·S\*, and b = 0 only when S\* = 0: **0 mismatches** (`verify_r5/probe_n1_flup.*`) | D1:37, D1:443, D1:448; D2:28, D2:592, D2:735-736 |
| 6 | **N-2**: zero-scale clause unreachable in the product; the K-D5 test is kernel-level | **VERIFIED** | Correct reasoning: before S11-F, u = 0 means the folded f = 0, so ρ = 0; after S11-F, the solve uses the exact force. The K-D5 row carries the kernel-level test | D1:38, D1:616, D1:1019 |
| 7 | **N-3**: "0 false positives" qualified to the folded force | **VERIFIED** | The ledger-terms qualification (factor 2 fires twice at actual 0.80, conservative) is stated in both places | D1:39, D1:626 |
| 8 | **N-4**: EF blind to input-representation error | **VERIFIED** | Stated with the RF-FINITE-TENTHS figures and the represented-basis route | D1:40, D1:656 |
| 9 | **N-5**: benchmarks enumerated | **VERIFIED** (NOTE 2) | Mechanics solves with `reduce_system`/`solve_dense`, never the M03 structural solve. The nonlinear benchmark goes through the loop, which K-D5 does not reach | D1:41, D1:641 |
| 10 | **N-6**: stray phrase removed | **VERIFIED** | "units as committed envelopes publish them" now appears only in the change-row description, not in §4.1.6.1 item 2 | D1:42, D1:460-472 |
| 11 | **[align D1-B]** | **VERIFIED** (NOTE 4) | No `[align D1-B]` marks remain in D2. The B pins (BFS order, digest domain, read-set table) are stated identically in D1's change row and D2's S-J | D1:43; D2:29, D2:885-886, D2:949 |
| 12 | **D1 mirror of ROOT's nonlinear-support rule** ("not selected, never refused") | **PARTIAL** | The operative rule is correct in the 5a.1 change row, the §4.2 table row, §4.3's "Never selected" and §4.3.1's nonlinear note. Two stale sentences contradict it: D1:166 ("Every excluded family is refused per case with a named reason") and D1:720 (§4.6: "At the facade, D1 refuses every model with a nonlinear support record"). The fix is under **Verdict** | D1:30, D1:558, D1:587, D1:655 (correct); D1:166, D1:720 (stale) |
| 13 | **D2 nonlinear-support rule as narrowed by ROOT** | **VERIFIED** | Never selected, no proof or successor identity, ordinary result and standing kept, never withheld or refused. `INPUT_DOF_MISMATCH` only where a receipt falsely claims selection, as a reader-integrity refusal of the receipt. S-G adds a positive control for a nonlinear case | D2:32, D2:580, D2:990 |
| 14 | **D2 "undecided" wording (R5-5)** | **VERIFIED** | "Undecided" = `RULE_INPUTS_INCOMPLETE`, from `RULE_RESULT_INDETERMINATE` or a withheld-row refusal. Any check decided under the retiring identity and undecided under the successor stops retirement. This matches ROOT's acceptance | D2:390-393 |
| 15 | **D2 R5-2 family detail** | **VERIFIED** | Clause-by-clause equal to `PP` (item 2), including trimmed hanger type versus untrimmed `spring`, and the constant-effort and spring-hanger exclusions. Duplicate pairs collapse | D2:574-582 |
| 16 | **K-D5 tests, mutations 31 and 32, `solve_with_formation_check`**, against option (c) in S11-K | **VERIFIED** (NOTE 2) | `solve_with_formation_check` is called only from `PP:3965`. The loop's calls (`nonlinear_integration/src/lib.rs:1915` `assembly.solve`, `:1933-1936` direct) and `product_equilibrium` are pinned. Mutation 32 fails that pin. This agrees with I1's option-(c) implementation: the loop's four call targets move to the `_binary64` variants, pinned by `option_c_nonlinear_loop_is_pinned_to_the_binary64_kernel_path`. Mutation 31 (shared binary64 curved K, or the product chord) reproduces as a miss at k_X = 8.5 dense (my \|w\|/criterion 0.254; D1's "0.25") | D1:643, D1:1019, D1:1122-1123 |
| 17 | **K3a arctangent spec** (R5-4) | **VERIFIED** (NOTE 5) | I emulated the specified algorithm with every operation correctly rounded to p = 128: t = s/(1+c), half-angle steps t ← t/(1+√(1+t²)) while t ≥ 0.05, then the series and ×2^(k+1). Against a 120-digit reference over 13 angles from 1e-300 to π − 1e-7, the largest error is **2.69 ulp** of p, with at most 5 reductions (matching the note) and at most 15 series terms. That is ~1e-38 relative, far inside the check's first-order margin | D1:1014 (K3a row); `R5_4_CURVED.md` §2 step 3, l.68 |
| 18 | **D-14** (`GeneratedUniform`; equivalent-static in F3; self-weight an ordinary input) | **VERIFIED** (NOTE 6) | `GeneratedUniform { member, direction, source, inputs }` with Seismic/MassPerLength and Wind/ExposedDiameter forms at p, one effective wall, and one p-bit ledger term. Placed in **F3** in the slice table, atomic with RF-ELOAD. Refused as `equivalent-static unsupported` until then. Self-weight is an authored input (ROOT, after `28d96084f`). D2 needs no text (D2:31). This matches `SELECTION_PACKAGE.md` §5's D-14 row | D1:556, D1:561-570, D1:1026, D1:1156, D1:1273 |
| 19 | **Exception pin**: `GATE/S11_EXCEPTIONS.json` (sha256 `8f3687f4…`) | **VERIFIED** | `s11_exceptions.py REFERENCES/references.json GATE/S11_EXCEPTIONS.json` reconciles **228/228** (`equal: true`, no difference either way), and its stdout is byte-identical to the committed `s11_exceptions.stdout.json`. Counting the pin directly: 228 distinct triples; captured 88 in 13 cases, typed 140 in 22 cases | D1:45, D1:893-898 |
| 20 | **`SELECTION_PACKAGE.md` §5 rulings as reflected in D2 §9** | **VERIFIED** | DD-1/DD-2 (F1; R-1a and R-1b open until W1), DD-3 TS-a, DD-4 H-a, DD-5 tighten, DD-6 shared gate and D-15, DD-8 as proposed, DD-9 as proposed, DD-10 keep eligibility only if S-E2 is built, DD-12 H-1, and DD-15 build after S-I. All match. DD-7, DD-11, DD-13 and DD-14 carry their earlier rulings | D2:1064-1084 |

## R5-4: the four conditions, each with my reproduction

**How I reproduced it.**
- **(i) D1's probe.** I reran `curved_ef.py`. Its stdout and JSON are byte-identical to the committed `curved_ef.stdout.txt` and `curved_ef.json` (`3de8f2fe…`).
- **(ii) My own probe.** `verify_r5/probe_r5_4_indep.py.txt` re-forms the curved element a different way:
  - the arc flexibility by composite Simpson quadrature (N = 4000) of the complementary energy, written directly in global coordinates from the section resultants m(ψ) = M + (x_tip − x(ψ)) × P, resolved on (tangent, radial, binormal). There is no trig Gram, no unit-action table and no local frame;
  - X = F⁻¹ in 60-digit Decimal;
  - K_e = AᵀXA, where A is the rigid-transport map of the **actual** nodes;
  - the intended solution by my own Decimal elimination;
  - for the cantilevers, a **statics reference**. The root rotation about the sprung axis is exactly M/k for any equilibrium-consistent element, because the structure is statically determinate under a pure end moment.
- **What (ii) reuses.** Only D1's model definitions, and the product-path system from D1's binary64 port of the curved crate. I checked that port line by line against `P/core/solver/curved_bend/src/lib.rs`: geometry `:149-178`, `end_flexibility` `:205-235`, `local_stiffness` chord `:238-248`, `unit_load_actions` `:670-741`, `trig_gram` `:901-918`, `invert_symmetric6` via FK `solve_dense` with the first-strictly-larger pivot `:932-955`, `equilibrium_transfer` `:957`, `assemble_macro_stiffness` `:972`. I also checked the centre against `PP:5610-5667`. The product solve uses my own `probe_d5_check.py`.

**1. The re-formation is sound and rotation-consistent by construction: HOLDS.**
- **Rigid modes.** The largest value of |K_e·r| / (max|K_e|·max|r|) over the six rigid modes of the actual nodes, across all 26 curved elements in the 22 models:
  - D1's re-formed Decimal element: **1.7e-59**;
  - mine: 4.4e-60 (60-digit round-off; zero by construction);
  - the product's binary64 matrix: **2.4e-16** (smallest 3.7e-17). This confirms the recorded finding that the product element is not rotation-consistent on binary64 inputs.
- **Agreement.** D1's closed-form element and my quadrature element agree to 2.7e-14 relative, which is the accuracy of my binary64 quadrature.
- **Statics.** On all 15 cantilevers, my intended solution's root rotation equals M/k to 8.9e-49 relative.
- **No libm dependence.** Nothing depends on the platform's sin, cos or atan2. The re-formation uses square roots and the K3a arctangent, whose specified algorithm I measured at ≤ 2.69 ulp of p = 128 (item 17).

**2. It misses none of the 4 curved Passed breaches on main: HOLDS.** My independent reference finds the same four Passed breaches, all at the root row N0.RX. There the exact value is M/k by statics, so the breach is real against physics and not only against D1's choice of intended element. My EF fires on all four:

| Case (C-skew, root spring k_X) | Mode | Actual ratio (vs statics) | EF trigger 2\|w\|/criterion (mine / D1) | Shared binary64 K (D1) |
|---|---|---|---|---|
| 10 | dense | 1.076 | 2.153 / 2.153 | 0.798 (**missed**) |
| 9 | sparse | 1.035 | 2.070 / 2.070 | 1.209 |
| 8.5 | dense | 1.481 | 2.963 / 2.963 | 0.509 (**missed**) |
| 8.5 | sparse | 1.096 | 2.191 / 2.191 | 1.280 |

**3. It demotes none of the 12 realistic elbow case-modes (E1–E6): HOLDS.**
- All 12 are Passed, and none fires under my re-formation.
- The largest trigger value 2|w|/criterion is **0.0038** (E6 sparse), where |w|/criterion = 0.0019, D1's "largest reading".
- The largest actual ratio is 0.0019. Mine and D1's agree to 3–4 digits in every case-mode except E4, where both are near 1e-5 of the criterion and agree within 1%.
- Across all 34 Passed case-modes, my EF fires on exactly the same 10 as D1's. Six of them are Passed non-breaches between 0.5 and 1 of the criterion: the factor-2 margin working as designed.

**4. Reusing the product's binary64 curved matrix misses 2 of the 4: HOLDS.**
- D1's EF_shared (the product's binary64 curved K_int) misses C-skew k_X = 10 dense (0.798) and k_X = 8.5 dense (0.509). So re-formation is required.
- The misses come from non-objectivity. The binary64 matrix's rigid-mode residual (2.4e-16 relative) enters ρ as a spurious term, of the same order as the solve error in the soft rotational mode.

## NOTEs (non-blocking)

1. **R5-3 caller detail.** D1's §8.1 bullets are a summary. Point to I1's `ks_callers.txt` / `PRE_REGENERATION_REPORT.md` §2 as the authoritative enumeration: it also lists the zero-prescribed `reduce_system` callers in `performance_harness` and the mechanics benchmark. Record I1's classification of `scrutinize_gaps` (V1's "SA:1030", now `SA:1146` on I1's branch) as nonlinear/legacy.
2. **Naming after S11-K.** D1 says the nonlinear loop stays on "the unchanged `solve`" (D1:41, D1:1019) and cites `lib.rs:1915` and `:1933-1936`. With option (c), the loop calls `assembly.solve_binary64` and the other `_binary64` targets (I1: `lib.rs:1968/1981/1999/2004/2025` on the branch). The K-D5 pin test and mutation 32 should name the `_binary64` targets and I1's pin test. The behaviour is the same; only the names and line numbers differ.
3. **R5-5 wording.** D1:694 should adopt D2's "undecided" (which includes withheld-row refusals), as ROOT accepted. "In all three languages" should read as D2 states it: Rust computes, Python recomputes, TS gives the same binding decision only.
4. **Stale marks in D2.** `[align D1-r4]` marks remain at D2:653, D2:933 and D2:1135. They are cosmetic.
5. **K3a arctangent wording.**
   - The K3a row says "faithful to within one ulp of p". The specified naive algorithm measures up to 2.69 ulp, so the test vectors' tolerance should be stated as a few ulp (or the arithmetic given a guard precision). The margin is immaterial.
   - `R5_4_CURVED.md` l.68's "about 30 series terms" is conservative; I measured at most 15.
   - The domain is 0 < φ < π open. At φ = π, 1 + cos φ = 0, which the note already excludes.
6. **D-14 text.** D1:44 (change row) still says "W1b extension in F2b … joins W1b as an F2b extension". D1:556 still carries the "slice flag for ROOT", which ROOT has since answered (F3; `ROOT_RULINGS_V1.md`, "D-14 slice correction"). The operative placement (§6 F3 row, D1:1026) is correct. Align D1:44 and resolve the flag at the next edit.
7. **"Reading" convention.** D1 and `SELECTION_PACKAGE.md` §6 quote EF as |w|/criterion: "largest reading 0.0019", "EF equals the actual error to three digits", "EF_shared 0.25". The firing test is 2|w|/criterion > 1. Both are correct, but the margin to firing on E1–E6 is 0.0038 of the threshold, not 0.0019. Saying which quantity is quoted would avoid confusion.
8. **`support_stiffness_input`** (`PP:10253`) also reads `hanger.stiffness`, whereas `withheld_rows.py` and `b_proof.py` check only the top-level `stiffness`. This does not change the restrained set, because the rule decides on family and hanger type.

## What I ran

All runs used standard-library Python 3, single-threaded, `nice 19`, `PYTHONDONTWRITEBYTECODE=1`, from `T3/`. I made no Git writes and used no cargo. I read no product code beyond the cited sources, and ran no product code.

| Run | Result |
|---|---|
| `DESIGN_NUMERICS/_run_records/curved_ef.py <v1_dir> <out>` | stdout and JSON byte-identical to the committed records |
| `verify_r5/probe_r5_4_indep.py.txt <v1_dir> <out>` (~5 s) | conditions 1–4 above; output `probe_r5_4_indep.json` |
| `verify_r5/probe_atan_p128.py.txt` | largest error 2.69 ulp of p = 128; ≤ 5 reductions; ≤ 15 terms |
| `verify_r5/probe_n1_flup.py.txt` | 25,008 samples, 0 mismatches |
| `DESIGN_NUMERICS/_run_records/s11_exceptions.py REFERENCES/references.json GATE/S11_EXCEPTIONS.json` | 228/228 equal; stdout byte-identical to the committed file |
| `b_proof.py` and `withheld_rows.py` from `P/fixtures/product_preview` | byte-identical to `b_proof_main.json` and `withheld_rows_merged.json` |
| `r5_backcheck/probe_r5_b.py.txt` against `b_proof.py` at `8dfd72921` | all four earlier false claims now `false` |

- `<v1_dir>` holds my `REVIEW/_run_records/d5_check/probe_d5_check.py.txt`, copied to `probe_d5_check.py`.
- The sha256 of each rerun output (scratch files, not committed) is in `verify_r5/rerun_hashes.txt`.
- New records are under `REVIEW/_run_records/verify_r5/` and listed in `REVIEW/_run_records/SHA256SUMS`.
