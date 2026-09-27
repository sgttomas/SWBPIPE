# T3 V1: final narrow re-pass R5 (DESIGN r5, S11 r5, D5_TRIGGER r2, D2 r5a)

Type 2 TASK V1, 2026-09-26. This is the last design check before the selection package, as the T3 manager requested.

**Scope.** Read-only. Standard-library Python, single-threaded, `nice 19`. No cargo, no Git write.

**Inputs at `8bf23f794`.** Each sha256 was verified, and the files are unchanged at the branch head:
- `DESIGN_NUMERICS/DESIGN.md` r5 (`42bba414…`);
- `S11_CONTAINMENT.md` r5 (`776a6309…`);
- `D5_TRIGGER.md` r2 (`da8ce574…`);
- `recal_d5.py` and `b_proof.py`, with their records;
- `DESIGN_STANDING/DESIGN.md` r5a (`04a466b1…`);
- `ROOT_RULINGS_V1.md` through `3acd84048`.

**Citations.**
- Product lines are cited at the merged tree `303609725`, as the manager asked. `FK`, `SA` and `P/core/solver/**` are identical to `c61a540ea`.
- Line prefixes: `D1:n` is DESIGN r5, `D2:n` is DESIGN_STANDING r5a, `S11:n` is S11_CONTAINMENT r5.

## 1. Verdict: **FINDINGS** (nothing blocking). ROOT's D5C-1 acceptance condition is met.

**The fixed EF has no misses** (§2). I ran r5's rule, 2·|w_i| > 1e-9·max(|q_i|, S*_kind), with the exact intended-system residual, on three sets:
- my 230 absorbed-spring Passed breaches: 0 missed;
- my 105 solve-error Passed breaches: 0 missed;
- P1's 122 case in both modes, under all ten y references, including P1's (1,0,0) at 2.413 dense and 1.214 sparse: 0 missed.

On the breaches, the lowest EF/actual ratio is 0.99999993. D1's recalibration reruns byte-identically.

**Five SHOULD-FIX items remain:**
- **R5-1.** The proof-carrying B claims zero reactions at restrained DOFs of loaded members. That is unsound. A one-rule fix changes no committed count.
- **R5-2.** Which support families are "rigid" is not pinned for `input_derived_dofs` or for B. D1's scripts treat every non-`spring` family as rigid, including spring hangers and constant-effort supports.
- **R5-3.** S11 r5's caller list still misses two callers of KS-changed kernel functions. This matters now, because I1 is implementing S11-K.
- **R5-4.** D5C-2 demotes every Passed invocation that contains a realized DEC-070 curved bend. That is a field loss of Current for a user-selectable feature, and §8.1 does not state it.
- **R5-5.** Gate condition 3 counts C-bindable rows as restored. But sign and equality checks on unproven zero rows become indeterminate where exact-block decides them today, and the row count does not show that.

## 2. D5C-1: the fixed EF (ROOT's acceptance condition), and D1's recalibration

**The design.** EF is defined in D1:562-575:
- ρ = f − K_int·u, one `ExactAccumulator` sum per free row;
- K_int is the `Wide<2>` re-formation from primitives, local coefficients included, plus springs, the prescribed coupling and the ledger or folded f;
- w = K̃⁻¹ρ with the mode's factor;
- the rule is 2·|w_i| > 1e-9·max(|q_i|, S*_kind), plus a zero-scale clause.

Neither the published residual nor element-level ΔK is used. This is exactly the D5C-1 fix.

**The check (probe E, `probe_r5_ef`).** My `probe_d5_check` estimator `EF_exact` is this definition. Its K_int is exact, not 128-bit; the difference is 2^-128·cond, which is negligible. I regenerated the D5_CHECK sweeps and applied r5's rule, with main's outcome Passed in each mode.

| Set | Passed breaches (actual > 1) | Missed by r5's rule | Min EF/actual | Firings with actual ≤ 1 | Of those, actual ≤ 0.5 |
|---|---|---|---|---|---|
| Absorbed soft spring (D5_CHECK probe C grid) | **230** | **0** | 0.99999993 | 30 | 0 |
| Solve error only (probe D grid) | **105** | **0** | 0.99999994 | 41 | 0 |
| 122, ten y references × 2 modes | 18 | **0** | 1.0000002 | 2 (y_ref (2,−2,1) and (−2,−1,2) dense, 0.726) | 0 |

Every firing below the criterion lies between 0.5 and 1, which is the intended conservatism of factor 2. **P1's measured variant** fires in both modes: EF 2.413 dense and 1.214 sparse, equal to actual.

**D1's recalibration (`recal_d5.py`).**
- Rerun with my probe as its import (sha256 `d13cf7c8…`), the output is **byte-identical** to D1's `recal_d5.stdout.json` and `recal_d5.json`.
- Confirmed with folded f and factor 2 on the coupled S*: 28 Passed breaches; misses only in the 26 RF-CANCEL case-modes (S11's named class); **0 false positives in the Passed band**. Factor 8 gives 3 false positives, and the uncoupled S gives 6, so keeping S* is right.
- **N-3.** With ledger terms, factor 2 gives 2 firings at actual 0.80 (RF-CANCEL-F-G1e80-GnG-INPLANE). That is conservative and fine, but "0 false positives" holds for folded f only.
- **N-4.** EF is blind by design to input-representation error: RF-FINITE-TENTHS has actual 0.058 against EF 2.6e-4. Such cases are compared on the represented basis where R1 marks them. State it beside N-1 and N-2.

**The zero-scale clause (D1:575)** is a sound safeguard. **N-2:** it cannot fire in the product.
- Before S11-F the kernel solves with the folded f, so u = 0 implies ρ = f_folded = 0.
- After S11-F the product solves with the exact force, so u ≠ 0.
- It is reachable only when ledger terms differ from the force the solve used. K-D5's listed test of it (D1:973) must therefore be a kernel-level test that supplies ledger terms.

## 3. Order and D5C-2 to D5C-5

| Item | Revision 5 | Check |
|---|---|---|
| Order | S11-K → K3a → K-D5 → K2a → K1 → K2b → K5, with K3a = `Wide<2>` only and the `mod retained;` declaration now merging after S11-K and before K1 (D1:968, D1:986-989) | **Consistent.** The write sets serialize correctly: `SA` and `FK/structural.rs` for S11-K, K-D5, K1, K2b and K5; `FK/lib.rs` for S11-K, K2a and K2b. The `Wide` need is justified, because EF divides and takes square roots far below 2^-53 |
| K-D5 callers | `SA:280-282` supplies the formation source; `nonlinear_integration/src/lib.rs:1933-1936` and `product_equilibrium.rs:122-153` are pinned unchanged | **Verified complete** for `solve_structural_dense`/`_sparse`: these are the only non-test callers. `product_equilibrium.rs:122-153` sits in its test module |
| D5C-2 | Any curved or user contribution is demoted with `formation_check_unavailable`. No committed solving fixture is affected; the b1/b2 curved-arc tests are declared (D1:593-600) | **Committed claim confirmed.** No committed request realizes `curved_bend_macro_element`; it appears only in semantic tables and the non-solve `result_export_v0_2.json`. See **R5-4** for the field impact, and N-5: "possibly also the `mechanics` and `nonlinear` benchmark crates" (D1:600) must be enumerated before K-D5 merges, per ROOT's lesson |
| D5C-3 | EF lives in a `FormationCheck` record on `StructuralSolution`, never on `StructuralReport` (D1:602-606) | **Verified.** `PP:1057` and the nonlinear diagnostics at `PP:2605-2615` render `StructuralReport` and equilibrium values only, never a `StructuralSolution` |
| D5C-5 | (entry, case, quantity) triples: 106 in 13 cases captured, 168 in 22 typed, including UDL-W1e8; any triple outside the list fails the gate; emptied after S11-F; both entries (D1:847-854) | **Verified** in `s11_exceptions.stdout.json`: 274 distinct triples (106 captured, 168 typed); captured ⊂ typed; the extra 62 are exactly the nine G = 1e80 cases. The seeded non-S11 breach inside a named case is the right negative control |
| Mutations 25–28 | D1:1070-1073 | Correct. 26 and 27 cite my sweep counts for the design as it was before the fix |

## 4. R4-1: span statics

D1:395 and D1:397-406 give k = 2√2 for the circular maximum and k = 4 for the open-formula summary. The proof now covers M(x) = M_i − V_i·x as one exact sum rounded once (E4/E6 through S11-K), with |δM| ≤ ε·mo + L·ε·fo ≤ 2ε·mo, since L·fo ≤ L_b·fo = mo.

**Correct.**
- Probe J from R4 gives 9.31e-10 at the threshold.
- k_{2√2} = `0x4006A09E667F3BCD` is an exact doubling.
- The pressure carve-outs now cover the rebuilt membrane (`PP:8183-8189`), and both tables show `pipe_axial_membrane_stress_v2` and the circular maximum as `not_covered` on a pressure member.

## 5. R4-2 and D-15: condition 3, F2a and F2b, coexistence, B and S-J

| Item | Revision 5 | Check |
|---|---|---|
| Condition 3 at row level | Successor withheld ≤ retiring withheld per case, per language, side by side, as a pass condition (D1:650, D1:656) | **Applied.** See R5-5 on check-level indeterminacy |
| F2a and F2b; C first | F2a wires W1 and retires nothing, atomic with S-G1. S-I1 can land any time; S-I2 lands with or after F2a. **Both come before F2b and before F3's retirement step** (D1:977-980, D1:993; D2:802-806) | **Applied, consistent in D1 and D2** |
| Coexistence | No W1 attempt in any invocation where exact-block selects a case; such invocations publish as today, bytes included; never both selected diagnostics (D1:632) | **Applied.** A D-5-routed case in such an invocation stays demoted, never Passed (D1:977), so this is fail-safe |
| B coverage | `b_proof.py` rerun on the merged tree reproduces `b_proof_main.json` exactly: N05/N06, mixed `case`, fields, multicase `case` and rejected all proven; signed-companion 46 of 57; eigen_motion 32 of 62; ordinary-pressure 8 of 68 | **Coverage confirmed.** Soundness: see R5-1 and R5-2 |
| B proof steps 1–4 | Exact rational frame zero pattern; four families; seeds; BFS reach | **Sound for the intended system.** Normalization preserves zeros. Tᵀ K T couples only within family sets. The unreached block of a positive definite K_ff with zero right-hand side and no coupling is exactly zero. The binary64 frame can leak tiny entries where the rational pattern is zero, but the exemption also requires a published ±0.0, so B binds 0 only where the intended value is exactly 0 |
| S-J reader contract (D2 §4.12) | Re-derivation with set equality both ways, a pattern digest, the ±0.0 requirement, three-language exact arithmetic, and mutants per seed rule | **Sound as a contract**, but it mirrors step 5's reaction rule, so R5-1 and R5-2 must be fixed on both sides |

**R5-1 (SHOULD-FIX): B's reaction rule claims zeros it has not proven.**
- **The rule.** Step 5 (D1:1166; D2:843) proves a restrained DOF's reaction zero "from the restrained DOF's element couplings when no load acts at that DOF". `b_proof.reaction_zero` implements "no load" as "no nodal load". It ignores the end forces that a member load, pressure or eigen strain on an incident member puts at that DOF.
- **Probe B (`probe_r5_b`) calls D1's `analyse` unchanged** on two product-shaped models in which one member runs between two anchors:
  - with a uniform load −1000 N/m on it, the A-end reactions Fz (intended +1000 N) and My are **claimed proven zero**;
  - with a uniform axial eigen strain instead, both anchors' Fx (intended ∓E·A·ε) are **claimed proven zero**.
- **Exposure.** It is limited, because a row is exempt only if it is also published as ±0.0. But the step claims an exact zero for a nonzero intended value, which contradicts D1:1177 ("never claims a zero it has not proven").
- **Fix** (`b_proof_reaction_fix.diff.txt`). A restrained DOF lying in any family set of a member-loaded, pressure or state-loaded member, or in the axial family of an eigen-strained member, is not provable. The corrected script gives **identical proven counts on all 48 committed case-modes** and rejects both synthetic claims. D2 §4.12.2 step 5 and S-J's negative controls must mirror it; add "a loaded fixed-fixed member's reaction listed".

**R5-2 (SHOULD-FIX): "rigidly restrained" is not pinned.**
- **The product's rule.** Rigid restraints are only the supports that `PP` maps through `rigid_linear_support_from_preview`. It excludes:
  - `spring` and spring hangers, selected by hanger type (`is_variable_spring_hanger`, `PP:10239`);
  - constant-effort supports (`PP:5178`);
  - nonlinear supports without restraints (`PP:5173-5177`).
- **D1's scripts differ.** `withheld_rows.py` and `b_proof.py` treat every family except the literal `"spring"` as rigid.
- **D2 is ambiguous.** Rule 2a derives the list from "the invocation's support restraint sets" (D2:552), which does not say which families count.
- **Consequences.** A spring-hanger or constant-effort DOF could be listed in `input_derived_dofs`, which makes a solved displacement bindable with no class. In B, it would be seeded as a restrained zero.
  - If producer and reader disagree, the result is `RETAINED_PRECISION_INPUT_DOF_MISMATCH` (fail-closed).
  - If both follow the loose reading, it is a silent mislabel.
- No committed selected case has these families, so no count changes.
- **Fix:** define `input_derived_dofs` as exactly the DOFs the kernel's `StructuralSystem` holds as restrained or prescribed (not free). Have D2 derive them with the product's family rule, stated explicitly. Correct both scripts.

**R5-5 (SHOULD-FIX): gate semantics for indeterminate outcomes.**
- **Why the count is not enough.** After S-I, condition 3 counts a C-bindable `absolute_verified` row as not withheld (D1:650, D1:1189; D2:867). But a zero row that B cannot prove binds as [−b, b], so `x ≥ 0`, `x = 0` and `x ≠ 0` read `RULE_RESULT_INDETERMINATE`, where exact-block, which publishes +0 as a point, decides them today.
- **Affected rows.** On the committed families: the 60 unproven zeros of ordinary-pressure and the 7 cancellation zeros of eigen_motion. More would be affected if B is not built, since it is optional.
- **The effect.** The row count passes while a check moves from decided to indeterminate. That is fail-safe, never a false pass, but it is a regression the gate does not show.
- **Fix:** add a check-level comparison to condition 3. Run the committed rule packs and run fixtures that bind successor rows against both identities, and report every check that moves from checked or failed to indeterminate. Alternatively, count a zero-straddling C row as withheld wherever a rule uses it in a sign or equality comparison.

## 6. R4-3: D2's C design (§4.11)

**Checked:** scope; the bound; outward endpoints; the operator table (negate, abs, add and subtract, multiply, divide, compare including equality, Kleene not/and/or, select, min/max, interpolate, lookup, and non-finite → U); the exhaustive match with no default arm; the containment lemma; three-valued outcomes mapped to existing statuses with no schema change; the honest basis statement; parity cases and mutants.

**Constructing a straddling pass.** I tried to build a straddling input box that reads T, and could not:
- **Monotone pieces.** The corner rule for multiply and divide (with 0 excluded), the abs hull, add and subtract, and linear interpolation with breakpoints all enclose the exact result and the point path's binary64 result, because fl is monotone. The outward ulp step then covers the endpoints' own rounding.
- **Comparisons.** Compare reads T only if a.hi ≤ b.lo. Equality is T only for equal points. Select with an undetermined condition takes the hull. Dependency, as in x·x, only widens.
- **Rules as they exist.** The rule formula language (`rule_pack_document/src/lib.rs:240-296`) has no operator outside the table.

**N-1: the b = 0 case.**
- D2 binds an `absolute_verified` row with b = 0 as the exact point q (D2:28, D2:706), reasoning that b = 0 means S* = 0.
- But b = fl(2^-64·S*) underflows to 0 for 0 < S* < 2^-1011. Then q is bound as exact while the stop-rule bound is nonzero, and a rule such as `x ≥ 0` at q = +0 could pass on a value the bound does not pin.
- The magnitudes are below 1e-304, so it is harmless, but formally it breaks ROOT's constraint.
- **Fix:** skip widening only when S* = 0 exactly; otherwise widen by at least one subnormal step.

**Gate semantics:** see R5-5. **Verdict on C: meets ROOT's constraint**, subject to N-1.

## 7. R4-4: D1's table and D2's §4.9.10, row by row

**Identical in every row:**
- the classes, kinds and admitted units;
- k = 1, k_i, 2√2 and 4;
- the pressure conditions: membrane and maximum `not_covered` on a pressure member, and the summary only with zero pressure;
- the `input_derived` list;
- `non_quantity`: exactly four kinds, with any future kind `not_covered`;
- the `not_covered` list;
- entity rules 2a and 2b;
- k_i rounded upward by "nearest, then next-up if below the exact product", decided exactly in each language (Rust `mul_add` sign, Python `Fraction`, TS BigInt);
- the receipt field set A, Z, L, k_a and k_t, with twist and extension harness-only (`fl(mo/k_t)`, `fl(fo/k_a)`);
- N-2.

**Differences:**
- the rigid-family derivation behind 2a (R5-2);
- **N-6:** D1:446 says "units as committed envelopes publish them" inside a closed table. Drop the phrase; the explicit parentheticals are the list.

## 8. S-H/S11-F ordering, both entries, and I1's KS correction

- **S-H/S11-F ordering and both entries: applied.** D1:975, D1:992; S11 §8.2, §9 items 11–13; D2:41.
  - S-H never lands before S11-F; if separate, its PR re-runs the G = 1e80 RF-CANCEL cases through the captured route.
  - S11-F's tests run through both entries, including 1e80 F, M, ORTHO, INPLANE and UDL-W1e80.
  - The citations are correct: headless `run_preview_in_memory_mode` (`P/core/runner/headless/src/lib.rs:804`) calls `run_linear_static_preview_with_mode` (`PP:1397`), and both entries share `solve_load_case` (`PP:2137`).
- **KS correction: applied as ROOT ruled.** S11 §2.4, §4.6 (the named legacy variant, S11:350), §8.1 and §8.3 now say that KS1–KS3 are live in the nonlinear loop, keep that loop on a pinned binary64 legacy variant, leave DEC-046 untouched, and add B1 and B2 to the expected diffs.

**R5-3 (SHOULD-FIX): the caller list at S11:447-450 is still incomplete.** ROOT's recorded lesson was to find every caller, and the list says "a caller not listed is a stop". Two callers are missing:
1. **`evaluate_original_residual`, KS3, is called directly by `product_equilibrium::evaluate`** (`nonlinear_integration/src/product_equilibrium.rs:56`), not only through `solve_structural_*`.
   - Production calls: the nonlinear loop's final equilibrium (`nonlinear_integration/src/lib.rs:1957`, over the loop's `original` system with its nonzero closed-gap prescribed values) and the exact-gap projection gate (`SA:1030`).
   - The legacy variant must cover this path too, or the loop's equilibrium records change under KS3.
2. **`reduce_system_with_prescribed_displacements`, KS2, is called by `linear_supports::apply_linear_supports`** (`linear_supports/src/lib.rs:467`, `:487`).
   - `validation/benchmarks/mechanics/src/lib.rs` uses that function with `LinearSupport::imposed_displacement` (6 sites, for example `:908`, `:4333`, `:4406`), so KS2 is live in that benchmark crate on main.

**I1 is implementing S11-K now,** so these should go to I1 before its next fixture and benchmark run.

## 9. New silent-wrong routes

These are all SHOULD-FIX or NOTE, and none gives a false Passed:
- **R5-1:** a false B zero, only if the value is also published as ±0.0.
- **R5-2:** a mislabelled spring-hanger DOF, only if producer and reader agree on the loose reading.
- **N-1:** a b = 0 point below 1e-304.

**R5-4 (SHOULD-FIX): the field impact of D5C-2.**
- Realized DEC-070 curved bends (`solver_consumption = curved_bend_macro_element`) are a user-selectable product feature: the desktop shows the mode (`apps/desktop/src/features/model-workspace/modelView.ts:154`) and `PP` assembles them (`PP` DEC-070 constants near :125).
- Under D5C-2, **every** invocation containing one that would publish Passed is demoted to Sensitive, whatever its conditioning. Its envelope then loses Current until W1c. Curved cases are outside exact-block's scope too, so nothing recovers them.
- D1 §8.1 still reads "for ordinary and realistic models the combined loss is none … D5C-2's demotion touches no committed solving fixture" (D1:1200), which is true of fixtures only. D1's own §8 says real models "almost always contain … elbows" (D1:1115).
- ROOT's O1 condition 2 asks whether the combined withholding shows broad loss on realistic models. **State this cost in §8.1**: every Passed model with a realized curved bend. Then either take it to the owner, as ROOT's condition requires, or offer the bound-based alternative for curved contributions: add the element's `curved_formation` roundoff bound to ρ as an absolute term (`SA`'s curved symmetry trace already forms one), which is conservative and not silent.

## 10. Not checked

- No build or product run. EF, B and C are checked by emulation and by reading.
- P1's final `results.json` was not available, so the exception triples are D1's prediction, which is to be re-pinned (ROOT).

## 11. Run records (`T3/REVIEW/_run_records/r5_backcheck/`)

- `probe_r5_ef.py.txt` → `probe_r5_ef.stdout.json` (check E). Run it in a folder that also holds `probe_d5_check.py`, copied from `../d5_check/probe_d5_check.py.txt`.
- `probe_r5_b.py.txt` → `probe_r5_b.stdout.json` (check B). Its argument is D1's `DESIGN_NUMERICS/_run_records`, and it imports `b_proof` unchanged.
- `b_proof_reaction_fix.diff.txt` and `b_fixed_vs_original.json`: R5-1's fix, and proof that the 48 committed proven counts are unchanged.
- D1's `recal_d5.py` rerun is byte-identical to its committed outputs, and `b_proof.py` rerun equals `b_proof_main.json`, so neither is copied here.
- Python 3.11.15, `nice 19`, `PYTHONDONTWRITEBYTECODE=1`. Hashes are in `T3/REVIEW/_run_records/SHA256SUMS`.
