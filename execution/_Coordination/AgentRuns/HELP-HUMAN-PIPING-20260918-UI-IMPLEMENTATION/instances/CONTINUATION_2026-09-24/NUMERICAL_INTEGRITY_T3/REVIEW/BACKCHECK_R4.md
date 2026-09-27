# T3 V1: final narrow backcheck R4 (DESIGN r4, D5_TRIGGER update, S11 errata, D2 r4, D-15)

Type 2 TASK V1, 2026-09-26. The T3 manager requested this pass as the last design check before the selection package. It includes ROOT's D-15 scope additions.

- **Scope.** Read-only. Standard-library Python, single-threaded, at `nice 19`. No cargo, no build, no Git write. T1 was read only through `git show f3270ea79`.
- **Inputs at `e94af71f2`.** Each sha256 was verified, and each file is unchanged at the branch head `b6fe1eb75`:
  - `T3/DESIGN_NUMERICS/DESIGN.md` revision 4 (`7ca6fb9e…`);
  - `D5_TRIGGER.md` (`0d14db3b…`);
  - `S11_CONTAINMENT.md` (`8f5d5df8…`);
  - `T3/DESIGN_STANDING/DESIGN.md` revision 4 (`e12bd015…`).
- **Rulings.** The end of `T3/ROOT_RULINGS_V1.md`, through `b6fe1eb75`: BACKCHECK_R3, D-5, D-15, D-15 option C, T1 merged, and S-H/S11-F ordering.
- **Line numbers.**
  - `D1:n` is DESIGN revision 4; `D2:n` is DESIGN_STANDING revision 4.
  - Product code is at main `c61a540ea`, as the manager asked (T1 is now on main at `5aa4285c2`). `PP` = `P/core/product_physics/src/lib.rs`.

## 1. Verdict: **BLOCKING**, only through D5C-1 carried by reference; the items new to this pass are SHOULD-FIX

**The blocking item.** Revision 4 adopts `D5_TRIGGER.md` (a2) by reference (D1:497, K-D5 row D1:840). D5C-1 (BLOCKING in `REVIEW/D5_CHECK.md`) therefore still applies to revision 4's text: the EF is built from element-level ΔK plus the binary64 published residual. ROOT has adopted the exact-residual fix for revision 5. I re-check it there against the 230 absorbed-spring cases, the 105 solve-error cases and P1's 122 case, as ROOT's O1 condition requires.

**Everything else in this pass is SHOULD-FIX or NOTE:**
- **R3B-1's proof holds for rows formed from one end's actions.** It misses the span-statics rows, the circular maximum and the open-formula summary, whose far-end moment is rebuilt as M + V·L, which doubles the moment error (R4-1).
- **D-15.** The counts are confirmed exactly. Some withheld rows are non-zero values a user can rely on (§6). Revision 4's retirement gate and F2 slice conflict with ROOT's D-15 ordering (R4-2).
- **D2's DD-13 text does not meet ROOT's conservative-binding constraint for C** (R4-3).
- **The D1 table and D2's §4.9.10 differ in eleven places**, one of which gives a bit mismatch on the committed i = 1.15 fixture (R4-4).

## 2. R3B-1: propagation factors and the proof

| Item | Revision 4 | Check |
|---|---|---|
| k values | 1 for component and membrane stress; √2 for the circular maximum; 2 for the open-formula summary; √2·i for intensified rows (the row's own i); 1 for magnitudes formed at p (D1:362, D1:369, D1:408-411) | Correct for rows whose moment comes from **one end's published actions**: the component stresses and the intensified measure, which is built from the end rows (`PP:15344-15363`). **Not correct for span-statics rows** (R4-1) |
| k constant `0x3FF6A09E667F3BCD` | Nearest to √2, and ≥ √2 | Verified (probe K: it equals `math.sqrt(2)`, its square is ≥ 2, and the next lower double's square is < 2) |
| Proof (D1:364-371) | 2^-30(1 + 4u) + 6u ≈ 9.3132e-10 < 1e-9, margin 6.9e-11 | **Arithmetic verified** (probe P: 9.313232e-10, margin 6.868e-11). The roundings are conservative: even with 1-ulp `hypot` and the MPa conversions of the intensified path, c stays in single digits, far inside the margin |
| Carve-outs (D1:373) | The summary with a pressure-longitudinal term; pressure members whose wall force is not a published, stop-rule-checked row | Correct: both are signed sums whose operands can exceed q. **Clarify** that `exact_straight_summary_extrema`'s pressure membrane (`recover_wall_effective_membrane(r[0], …)`, `PP:7571-7577`) takes the axial force rebuilt by span statics, not the published wall-force row, so it falls under the carve-out |
| Magnitudes | Formed at p, k = 1 | Consistent with §4.1.5 |

**R4-1 (SHOULD-FIX): span-statics rows need twice the moment factor.**
- `exact_straight_summary_extrema` (`PP:7528-7596`) and the open-formula summary's `straight_summary_extrema` (`PP:2684-2697`) evaluate section resultants from **one end's** published actions: `straight_section_resultants` (`PP:7490-7518`) rebuilds M(x) = M_i − V_i·x (+ load terms).
- The stop rule bounds |δM_i| ≤ ε·mo and |δV_i| ≤ ε·fo.
- The coupling of D1:422 gives fo·L ≤ mo, because a member's length is at most L_b. So the rebuilt far-end moment carries up to 2ε·mo per component, not ε·mo.
- At the threshold (probe J), the worst relative error is **1.86e-9 for the circular maximum with k = √2**, and **1.86e-9 for the summary with k = 2**. Both exceed 1e-9.
- **Required change:**
  - k = 2√2 for `pipe_elastic_normal_stress_maximum_v2` and k = 4 for `open_formula_stress_summary`, which gives 9.31e-10 at the threshold. The alternative is to form those maxima from p-precision station actions.
  - Correct the proof's premise "for a row that is a published action" to cover rebuilt resultants.
  - Mirror the change in D2's list.
  - Extend mutation 25 with a span-statics row.
- The recount with these factors (probe W) changes **no** withheld count on the committed selected cases.

## 3. R3B-2: D1's closed table against D2's §4.9.10 (DD-14)

D1:400-416 is the design basis, and D2 marked §4.9.10 **[align D1-r4]**. D2 must mirror every row below.

| # | Item | D1 revision 4 | D2 revision 4 | Mismatch |
|---|---|---|---|---|
| 1 | Units | The key is (kind, unit), with units per kind (m, mm; rad; N, kN; N·m, kN·m; MPa, Pa as listed) | Keyed by (kind, unit) in text, but its table lists no units | Add the units |
| 2 | Input-derived restrained DOFs | Displacement and rotation rows at a **rigidly restrained or prescribed DOF** are `input_derived` | Absent | Add. This is not a (kind, unit) rule: readers need the invocation's restraints, so the rule and its invocation dependence must be stated |
| 3 | Echoed-input review kinds | `input_derived`: `component_user_stress_multiplier_review`, `component_user_stiffness_macro_element_review`, `constant_effort_user_input_review`, `spring_hanger_user_input_review`, `expansion_joint_pressure_thrust_load_review` | D2 classes rows by table `category`, so "review evidence" is non-quantity | Align the class: D1 makes them bindable `input_derived`, D2 makes them classless |
| 4 | `constant_effort_support_applied_load` (N) | `input_derived` | Absent | Add |
| 5 | `non_quantity` | An explicit closed list of 4 kinds | Defined by `category != physical_quantity` | Align on one mechanism. A category-defined class is not closed against a future physical-looking kind with another category. Prefer D1's explicit list, checked against the categories |
| 6 | `curved_bend_macro_element_review`, curved arc and station rows | `not_covered` | Review category, so non-quantity; curved stations not listed | Align. Curved station rows can share straight-row kinds, so a (kind, unit) key cannot separate them. They need an entity rule (they cannot occur in a selected case today) |
| 7 | `open_formula_stress_summary` condition | k = 2 **only without a pressure-longitudinal term**, otherwise `not_covered` | Unconditional k = 2 | Add the condition, and say how readers detect the term. Take k = 4 per R4-1 |
| 8 | Intensified k_i | `fl(k√2·i)`, nearest (D1:423) | `fl↑(fl↑(√2)·i)`, upward (D2:502, D2:622) | **Bit mismatch on 455 of 901 i values in 1.00–10.00, including i = 1.15, the committed fixture's factor (`PP:15363`)** (probe K). The result would be a false `RETAINED_PRECISION_SCALE_MISMATCH`. Pick one; nearest is simpler in TS |
| 9 | Magnitude factor | 1 (formed at p) | "1 if formed at p, otherwise √3" | Drop the √3 alternative |
| 10 | Receipt section terms | A, Z, L, E·A/L, G·J/L (D1:423, D1:810); twist and extension scales harness-only, not in the receipt | A, Z, and L/(GJ), L/(EA) "if kept"; G5b forms tw and ext "if present" (D2:498-500) | Mirror D1's field set and drop tw and ext from G5b. **D1 is itself inconsistent:** §5 item 1 still lists "per member, twist, extension and stress" S\* in the receipt (D1:799), and item 7's harness formula `fl(mo·fl(L/fl(G·J)))` cannot be formed from the receipt's G·J/L without another rounding. Fix both (harness: `fl(mo/k_t)` with the receipt's k_t = G·J/L, matching the derived twist T/k_t) |
| 11 | Small body scales (N-2) | S\* < 2^-988 means every row of that body and kind is `absolute_verified` (D1:384) | Not mirrored | Add to G5c |

Also consistent: `reaction_resultant` → force; the not_covered default; the two refusal codes; `input_derived` bindable and absent from both receipt lists; set equality in both directions (D2:516). D1's stale sentence at D1:800 ("Every other published quantity … is `relative_verified`") should point to the class table.

## 4. R3B-3, R3B-4, R3B-5 and N-1 to N-5

| Item | Status |
|---|---|
| R3B-3 (section terms as bit strings, cross-checked; twist and extension harness-only) | **Applied in D1** (D1:423, D1:810, `RETAINED_PRECISION_SECTION_MISMATCH`). Mirror and inconsistency as in §3 row 10. Published section evidence exists only on the exact route (`PP:7521-7525`, A_s and Z). Elsewhere L, E·A/L and G·J/L rest on the source identity digest, which D1:423 states |
| R3B-4 (D2: stress kind, input-derived class, set equality) | **Applied in D2** (D2:485-516, D2:570) |
| R3B-5 (site list keyed by function plus match count) | **Applied** (`S11_CONTAINMENT.md` §4.3 rule 8 erratum, with the count taken at the slice's base) |
| N-1 (293 and 303) | Applied (D1:28, D1:48, D1:731) |
| N-2 (S\* < 2^-988) | Applied in D1 (D1:384); not yet mirrored in D2 (§3 row 11) |
| N-3 (compare classes) | Applied (D1:726) |
| N-4 (diff sizes in the PR record) | Applied (S11 §8.3 erratum: bytes changed and the largest relative change per kind) |
| N-5 (two codes) | Applied (D1:390) |

## 5. D-5 as folded into revision 4

| Item | Revision 4 | Check |
|---|---|---|
| §4.3 trigger | (a2) by reference, 8·EF > 1e-9·max(\|q\|, scale) (D1:497) | **D5C-1 open** (EF definition). Fixed in revision 5 by ROOT's adoption; to be re-checked there |
| K-D5 row | After K3; the 122 case a required true positive; 345, the RF-CHAIN r1e-04 controls and M11 must not demote; byte-identical suites; mutation 23 (D1:840) | Consistent with ROOT's conditions. Still needed: D5C-2 (non-frame contributions; ROOT's new rule that such a case is demoted), D5C-3 (Debug byte-identity), and D5C-1's controls and mutations. The ordering is now "ahead of K2a" (ROOT, relayed), for the revision-5 re-pass |
| "No Passed breach" gate | The 12 RF-CANCEL breaches named as S11 exceptions until S11-F (D1:725) | The names are not enumerated (D5C-5: list them as (case, quantity) pairs). **ROOT's later ruling (`b6fe1eb75`)** requires the gate to run through **both** entries (captured and historical typed), with the exceptions removed only when both are clean. Revision 4 predates this and must add it |
| Mutations 23–25 | D1:929-931 | 23 and 24 as stated. 25 needs R4-1's span-statics row. D5C-1's three mutations are to be added |

## 6. §8.1: the combined withholding figure and D-15

**The counts are confirmed exactly.**
- D1's `withheld_rows.py`, rerun on main's fixtures, reproduces `withheld_rows_main.json` once the root-prefix difference in the paths is allowed for.
- My independent classifier (probe W) gives the same per-case counts on main and on T1's `eigen_motion` fixtures (read with `git show f3270ea79`):
  - N05 and N06: 61–62 of 79–82;
  - mixed: 62 of 81–82, and 68 of 110–111, plus 25 input_derived;
  - fields: 8 of 81–82;
  - multicase: 61 and 57 of 79–80;
  - rejected_stress_range: 59 of 79–80;
  - eigen_motion: 62 of 89–90.
- **No `not_covered` row** appears in any committed selected case.
- Every selected row carries a `basis_ref`, so the per-case attribution is complete.
- Applying R4-1's k values changes no count.

**Withheld rows that are not zero** (probe W lists each one):
- **multicase `case:signed-companion`, 11 of its 57 withheld rows:**
  - the soft support's reaction moment, 2.0e-8 N·m;
  - five member torsional moments of ±2.0e-8 N·m;
  - five torsional shear stresses of −3.70e-11 MPa.
  - Each is 1e-11 of the body scale, and 0.08–0.17 of its threshold.
- **T1 eigen_motion `case:join`, 23 of its 62 withheld rows:**
  - the soft spring's reaction moment of −1.0e-8 N·m, with its magnitude row;
  - member torsional moments of ±1.0e-8 N·m;
  - torsional shear stresses of 1.85e-11 MPa;
  - anchor, bending and bending-stress rows at 1.86e-13 N·m and 6.9e-16 MPa. These last are 1.8e-8 of their threshold, about 18 times the absolute bound b, and are indistinguishable from noise.

**Would a user rely on them?** Yes, for the first group. They are the soft-path responses that these N05-class fixtures exist to test: the torque carried by the soft restraint, and the member torsion and shear it implies. Exact-block publishes them as Current today.
- Engineering stress limits would never be governed by 1e-11 MPa.
- But a support-load rule bound to `support_reaction_component_v2` at the soft restraint, or a sign or direction check on it, relies on those values.
- Under C, b is about 2^-64·S\* ≈ 1e-16 N·m against 2e-8 N·m, a relative ±5e-9, so C restores them. Under A they are lost.

D1's reading that the withheld rows are "almost all exact structural zeros" (D1:1001) holds for the N05, N06 and fields families. It is overstated for signed-companion (46 of 57) and eigen_motion (39 of 62).

**R4-2 (SHOULD-FIX): revision 4's retirement gate and F2 slice conflict with ROOT's D-15 ordering.**
- §4.4.1 condition 3 still compares envelope standing, and treats the withheld counts as information only (D1:534). §8.1 leaves open "whether retirement waits for C" (D1:1014). F2 runs "the retirement gate for source-blocks-1 and physics-source-1" (D1:844).
- ROOT ruled otherwise:
  - condition 3 compares row-level withheld counts for both identities;
  - exact-block selection is not retired where the successor would withhold rows that are Current today, until C or a proof-carrying B restores no-worse standing.
- On the committed cases every family in §8.1 fails that condition until C lands, so F2 as written would retire against the ruling.
- **Required change.**
  - Make "successor withheld ≤ retiring withheld, per case, in all three languages" a pass condition of §4.4.1.
  - Order F2's retirement after D2's C slice, or split F2 into W1 wiring and a later retirement.
  - **Define coexistence while exact-block stays selected for its domain**, which D1 does not specify:
    - which method runs when both could;
    - how an invocation with one exact-block-selected case and one W1-eligible case is published, given option A's "a fresh invocation never mixes two selected methods" (D1:514) and D2's G4 ("No `SOURCE_BLOCK_RECOVERY_SELECTED` anywhere");
    - what `numerical_quality` and the diagnostics say.
  - One simple rule: while exact-block selection is retained, W1 is not attempted in an invocation in which exact-block selects a case, and those invocations publish exactly as today.
- **Proof-carrying B.** D1's rigid or prescribed-DOF `input_derived` rule is already a mechanical exact-zero proof from restraints, consistent with ROOT's B. Any wider B needs the same mechanical proof from topology and restraints.

## 7. D-15 option C: does D2's DD-13 text meet ROOT's conservative-binding constraint?

**R4-3 (SHOULD-FIX): no.**
- D2 §4.9.9 says a rule "passes only if it passes at both ends" (D2:585). ROOT requires a rule to be satisfied only if it holds for **every** value in [q − b, q + b], and otherwise to fail or read indeterminate.
- Rule formulas in `rule_check_runner` are general expressions: `add`, `subtract`, `multiply`, `divide`, `abs`, `negate`, `not`, and `compare` including `equal` and `not_equal` (`P/core/rules/rule_pack_document/src/lib.rs:240-296`).
- Testing the endpoints is not sound for non-monotone expressions:
  - `abs(x) ≥ c`, or `x·x ≤ c`, with x straddling 0;
  - `divide` by an interval containing 0;
  - `not(...)`;
  - several interval inputs, where a corner test misses interior extrema;
  - `equal` or `not_equal` on non-degenerate intervals.

**D2's alignment revision must add:**
1. **Evaluation.** Sound, outward-rounded interval evaluation of the whole rule expression, with three-valued results:
   - **pass** only if the enclosure proves every value passes;
   - **fail** if every value fails;
   - **indeterminate** otherwise.
   - Division by an interval containing 0 is indeterminate. `equal` or `not_equal` on non-degenerate intervals is indeterminate unless the intervals are disjoint. `not` uses three-valued (Kleene) logic.
2. **The indeterminate outcome.** A named result (for example `RULE_RESULT_INDETERMINATE`) that is never counted as a pass, is treated like `RULE_INPUTS_INCOMPLETE` in summaries and headlines, and is mirrored in `compatibility.py`, the TS mirror and the desktop runner, with shared parity cases.
3. **Scope.**
   - Only `absolute_verified` rows are bound as intervals.
   - `relative_verified` and `input_derived` rows bind exactly as today, as points.
   - `not_covered` rows stay withheld, because they have no bound.
   - A test shows that the covered-row outcomes are unchanged.
4. **The bound.** b is taken per row from the validated receipt, as the listed `fl(2^-64·S*)` with the row's k-inclusive S\*. The endpoints are formed outward (q − b rounded down, q + b rounded up), and unit conversion of intervals is outward too. TS needs an explicit widening step.
5. **Basis stated.** b rests on the stop rule, which is operational convergence evidence, not a forward-error enclosure (D1:378). So C is conservative to the same standard as the relative class's 1e-9 claim, no stronger. ROOT's "verified bound" should be read that way, or b doubled for margin.
6. **Gate counts.** State whether an interval-bindable row counts as withheld in `classification_summary` and in gate condition 3. It must not count as withheld once C lands, since C restores it.

## 8. Other silent-wrong routes

I found none beyond R4-1, which is a mislabel inside a narrow band, not a route to Passed, and D5C-1, which is already known. Two things were checked:
- The prescribed-DOF `input_derived` rule is exact by construction: `finish_checked_factor` sets u_i = v on prescribed DOFs.
- The echoed-input kinds are sums of inputs (E16 is exact after S11).

## 9. Run records (`T3/REVIEW/_run_records/r4_backcheck/`)

- `probe_r4.py.txt` → `probe_r4.stdout.json`: checks P, K and J.
- `probe_r4_withheld.py.txt` → `probe_r4_withheld.stdout.json`: check W. It was run on `P/fixtures/product_preview` and on T1's `load_reference_source/eigen_motion*` extracted with `git show f3270ea79` into a scratch folder.
- D1's `withheld_rows.py` was rerun on main's fixtures. Its output equals `withheld_rows_main.json` once the path prefix is normalized, so it is not copied here.
- Python 3.11.15, `nice 19`, `PYTHONDONTWRITEBYTECODE=1`. Hashes are in `T3/REVIEW/_run_records/SHA256SUMS`.
