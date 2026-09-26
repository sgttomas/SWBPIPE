# R5-4 — realized curved bends and the D-5 formation check

D1 (TASK), 2026-09-26. This note answers ROOT's R5-4 request (`T3/ROOT_RULINGS_V1.md`, BACKCHECK_R5 item 6, and ROOT's crossing message on R5-4 and process). It is delivered ahead of DESIGN revision 5a, which adopts it in §4.3.1.

- **Basis.** Product source at the T3 branch head `9e4d52c3b`. The product tree equals the merged tree `303609725`, and `P/core/solver/**` is unchanged since `c61a540ea`.
- **Scope of work.** Standard-library Python only, `nice 19`. Nothing was built, no product code ran, and no Git write was made.

## 1. Answer in brief

1. **Realizing a curved bend is opt-in, and the desktop cannot do it.**
   - A bend is solved as a DEC-070 curved macro-element only when the model document carries `mechanics_interface.solver_consumption = "curved_bend_macro_element"` on the bend component. The default is `mechanics_geometry_only`: a straight chord.
   - The desktop's component creation writes geometry only, with no `mechanics_interface` (`apps/desktop/src/features/component-creation/componentIntent.ts:318-327`). The structured operations have no field path for `mechanics_interface` (`core/model_operations/operation_applier/src/lib.rs`, bend canonicalization at `:2580-2586`). The desktop only displays the mode (`apps/desktop/src/features/model-workspace/modelView.ts:154`).
   - So a realized curved bend reaches a solve only through an imported or hand-authored model document.
   - **No committed model realizes one.** The value appears only in two product tests (`preview_physics_runtime.rs` `arc_model`, `:834-918`), the schema, a desktop slot test and the non-solve `results/invented/result_export_v0_2.json`.
   - None of D1's earlier invented realistic models realizes one either: M1–M11 are straight-member models.
2. **Loss of Current under revision 5's D5C-2, which demotes every case with a curved contribution.**
   - I added six invented realistic models with realized long-radius elbows, E1–E6. All 12 of their case-modes publish Passed, and **D5C-2 as written demotes 12 of 12**: every Passed model with a realized elbow loses Current until W1c.
   - Under the design below, **0 of 12** demote. The estimate is at most 0.0019 of the criterion on them.
3. **A sound alternative exists within T3, and it is stronger than a bound.** EF re-forms the curved element in `Wide<2>` from its binary64 inputs, as it already does for frames, as an **objective** element: the equilibrium transfer is built from the **actual chord** x_j − x_i. The curved contribution then enters the exact residual like any frame, so the check has the same exactness as for frames.
   - It needs one addition to K3a: a `Wide<2>` arctangent for the included angle, a short series. Sine and cosine come from the radial vectors with square roots only.
   - Its cost per curved element is O(6³) `Wide` operations, and it needs no extra solve.
   - In the emulation it catches every curved Passed-band breach (4 of 4), with EF equal to the actual error to three digits. A check that reuses the product's binary64 curved matrix misses 2 of the 4, so re-formation is necessary.
4. **T4's null-space confirmation is not a prerequisite.**
   - The check's intended element is objective by construction. It does not rely on the product's curved formulation having an exact rigid-body null space.
   - The emulation shows the product's formulation does **not** have one exactly on binary64 inputs. The check measures that defect and demotes where it threatens 1e-9.
   - That is a finding for T4 and W1c, stated in §5.
5. **User-stiffness elements (expansion joints)** are re-formed the same way, without the arctangent. The ordinary route already refuses a joint with lateral stiffness (`JOINT_ELEMENT_EQUILIBRIUM_UNQUALIFIED`, `preview_physics.rs:120-147`). With lateral stiffness zero, the element's relative axial, torsional and angular springs are objective.
   - **So D5C-2's demotion no longer applies to any contribution the ordinary route solves today.** It stays as the fail-closed default for any future family the check cannot re-form.

So ROOT's owner options (a), (b) and (c) are not needed. For the record, if ROOT prefers the demotion anyway, the counts are:
- under (a), 100 % of Passed models with a realized elbow are demoted, but only among hand-authored or imported documents;
- 0 committed models are affected;
- the curved-arc product tests change quality.

## 2. The design: an objective re-formation of the curved element in EF

The product's element (`P/core/solver/curved_bend/src/lib.rs`) is formed like this:
- The geometry comes from the node coordinates and the centre. `PP` computes the centre in binary64 from the chord, the pipe y reference and the user radius (`PP:5451` `build_curved_bend_macro_elements`, the sagitta and centre at `:5659-5667`).
- R is the mean of the two radial lengths, and the included angle φ = atan2(|r_i × r_j|, r_i·r_j).
- The 6×6 tip flexibility F comes from closed-form unit-load integrals in sin φ, cos φ and φ (`end_flexibility`, `trig_gram`, `unit_load_actions`).
- K_t = F⁻¹, computed through FK `solve_dense` one column at a time and then symmetrized.
- The element is K = [[H K_t Hᵀ, −H K_t], [−K_t Hᵀ, K_t]], with H = [[I, 0], [skew(c), I]] and **c = (R(cos φ − 1), R sin φ, 0)** (`equilibrium_transfer`, `assemble_macro_stiffness`).
- It is then transformed to global coordinates with local axes from r_i and r_i × r_j.

**Where the rigid-body null space comes from.** A rigid rotation ω gives node displacements (ω × x_i, ω) and (ω × x_j, ω). Then Hᵀu_i = (ω × (x_i + c), ω), which equals u_j exactly **iff c = x_j − x_i** in the local frame.
- The product's c is built from R and trigonometric values, not from the coordinates. In exact arithmetic on binary64 inputs, the two radial lengths differ by a rounding, so the product's element carries an O(u) rigid-mode stiffness even before any rounding of its own.
- The binary64 trigonometric values and products add more of the same. This is the 122 mechanism (a rigid mode that only a soft spring holds), located in the curved element.

**The check's intended element (K-D5, `FK/structural/formation_check.rs`).** From the same binary64 inputs (coordinates, the product's centre, E, G, A, I, J and the two factors), all in `Wide<2>`:
1. r_i = x_i − c and r_j = x_j − c exactly. Then |r_i|, |r_j| and R = (|r_i| + |r_j|)/2.
2. cos φ = r_i·r_j / (|r_i||r_j|) and sin φ = |r_i × r_j| / (|r_i||r_j|), with square roots only. sin 2φ = 2 sin φ cos φ and cos 2φ = cos²φ − sin²φ.
3. φ = 2·atan(sin φ / (1 + cos φ)), valid because the included angle is below π. Reduce by half-angle steps t ← t/(1 + √(1 + t²)) until t < 0.05, then sum the arctangent series and scale by 2^k. This is the one addition to K3a.
4. F from the product's closed forms, evaluated at p. Then K_t = F⁻¹ by Gauss–Jordan at p. F is 6×6 and well conditioned: cond 3.5e2 for the realistic 90° elbows, and 1.7e4 at 1e-3 rad.
5. Local axes x = r_i/|r_i|, z = (r_i × r_j)/|r_i × r_j| and y = z × x.
6. **H from the actual chord**, c_act = axes · (x_j − x_i). Then K_e = Tᵀ [[H K_t Hᵀ, −H K_t], [−K_t Hᵀ, K_t]] T at p.
7. K_e enters ρ = f − K_int·u exactly as a re-formed frame does (DESIGN §4.3.1 step 2). The rule, factor and scale are unchanged.

**Why this is the right intended system.**
- For arc-consistent geometry (equal radii and exact trigonometry), step 6 equals the product's formulation.
- On binary64 inputs it is the unique objective element with the product's flexibility. It is a symmetric positive semidefinite matrix whose null space is exactly the six rigid modes of the actual nodes, for any K_t.
- So every rigid-mode defect of the product's binary64 element shows up in ρ, and nothing in the reference hides it. That covers the chord and trigonometry mismatch, the rounding of H·K_t·Hᵀ, and the transform.
- The tip flexibility is re-formed too (steps 3–4). Its own formation error, including cancellation in φ − sin φ-type terms at small angles, is therefore in ρ as well. Nothing depends on the accuracy of the platform's `sin`, `cos` or `atan2`, which the product does not specify.

**Load formation** for curved elements (consistent uniform and radial-pressure equivalents) stays outside EF, as for frames (DESIGN §4.3.1, N-2). It is S11's.

**Cost.**
- Per curved element: about 400 `Wide<2>` operations for the flexibility, 6³ for the inverse, and 2·12³ for H and the transform.
- The arctangent takes about 30 series terms after at most five reductions.
- No extra solve.
- Implementation: `atan` joins K3a, with test vectors and the seeded `Fraction` differential. `formation_check.rs` gains the curved and user families. `SA` passes each element's primitives from `AssemblyEvidence::new` (`SA:25-143`); SA has them from `CurvedBendStiffnessElement` and `UserStiffnessElement`.

**Why not a bound (V1's alternative).**
- SA's curved symmetry trace (`curved_formation`, `SA:288-310`) bounds only the symmetrization stages: H·K, (H·K)·Hᵀ and TᵀKT. It says so ("does not bound inverse accuracy or qualify the curved physical nullspace"). It covers neither the trigonometric chord, the flexibility nor the inverse, and the chord mismatch is the dominant defect.
- A rigorous bound on the flexibility would need a specification of `sin`, `cos` and `atan2` accuracy, and the product has none.
- Pushing an absolute entrywise bound through K⁻¹ componentwise needs |K⁻¹| on the element's DOFs: six extra solves per curved node.
- Re-formation is exact, needs no libm assumption, and costs no solve.

## 3. Evidence (`_run_records/curved_ef.py` → `curved_ef.json`, `curved_ef.stdout.txt`)

**What the probe does.**
- It ports the product's binary64 curved element and `PP`'s centre computation, in source order as read, and imports V1's product-faithful frame, assembly and solve path (`probe_d5_check.py`, cited).
- It forms the intended system in Decimal at 60 digits: frames exactly, and the curved element by §2's objective re-formation with a Decimal arctangent.
- It solves the intended system exactly, and runs the product's dense and sparse paths.
- It reports the actual error on free nodal rows, EF with the curved element re-formed, and "EF_shared", where the curved part of K_int is the product's own binary64 matrix. EF_shared is what a check that cannot re-form curved elements would see.
- Section: V1's default 0.2/0.18 m steel pipe. Elbows are long-radius, R = 0.3 m, 90°, with their centres on the inside of each turn.

**Realistic models with realized elbows.** The table gives ratios to the 1e-9 criterion, dense / sparse.

| Model | cond (product estimator) | Actual | EF (rule's 2·EF shown as EF) | Fires | D5C-2 as in r5 |
|---|---|---|---|---|---|
| E1: L in plan, anchors at both ends, one elbow, hangers | 3.4e4 | 9.1e-4 / 1.9e-4 | 9.1e-4 / 1.9e-4 | no | demoted |
| E2: E1 with anchor, guide and free end | 3.7e5 | 1.5e-3 / 4.8e-4 | same | no | demoted |
| E3: plan elbow plus riser elbow (3D), anchors | 4.7e4 | 1.6e-4 / 6.8e-5 | same | no | demoted |
| E4: skew line (2,1,0) with a riser elbow | 3.5e4 | 6.6e-6 / 7.2e-6 | same | no | demoted |
| E5: elbow with flexibility factor 5, free end on a soft rotational stabiliser | 3.6e5 | 6.7e-5 / 1.4e-4 | same | no | demoted |
| E6: expansion U-loop, four elbows, anchors, hangers | 8.1e5 | 5.7e-4 / 1.9e-3 | same | no | demoted |

**All 12 case-modes are Passed; D5C-2 as written demotes all 12, and the re-formed check demotes none.**

**Curved formation breaches (the 122 mechanism in an elbow).**
- The model is one realized elbow as a cantilever: root translations rigid, stiff root rotational springs, one soft root spring, and a tip moment. Its only soft mode is the rigid rotation of the elbow about the root.

| Case | Soft spring | cond | Outcome | Actual (d / s) | EF (d / s) | EF_shared (d / s) |
|---|---|---|---|---|---|---|
| In-plane elbow, soft k_Z | 100 | 2.2e7 | Passed | 0.085 / 0.037 | 0.085 / 0.037 | 0.033 / 0.155 |
| | 34 | 6.5e7 | Passed | 0.567 / 0.110 | 0.567 / 0.110 | 0.220 / 0.456 |
| Skew-plane elbow (normal ∝ (1,2,2)), soft k_X | 30 | 1.8e7 | Passed | 0.626 / 0.310 | same | 0.134 / 0.181 |
| | 15 | 3.7e7 | Passed | 0.932 / 0.621 | same | 0.051 / 0.363 |
| | **10** | 5.5e7 | Passed | **1.076** / 0.931 | 1.076 / 0.931 | **0.399** / 0.544 |
| | **9** | 6.1e7 | Passed | 0.940 / **1.035** | same | 0.699 / 0.604 |
| | **8.5** | 6.4e7 | Passed | **1.481 / 1.096** | same | **0.254** / 0.640 |
| Either plane, k ≤ 1 | | ≥ 5.5e8 | Sensitive on main | up to 3.6e5 | equal | — |

**What this shows:**
- **Passed-band breaches with a curved element exist on main:** 4 case-modes, up to 1.48.
- **The re-formed check catches all 4.** EF equals the actual error to three digits in every row of both tables.
- **The shared-matrix check misses 2 of 4** (the bold EF_shared values below 0.5), and under-reads by up to 18 times (k_X = 15, dense). So re-forming the curved element is necessary, not optional.
- **False positives in the Passed band:** 6 case-modes fire with actual between 0.567 and 0.94. This is factor 2's intended conservatism (all ≥ 0.5), the same as for frames.
- **Small angle.** A 1e-3 rad bend (R = 30 m) between anchors has cond 4.0e6 and actual 0.060 / 0.015. EF is equal and does not fire. The flexibility's cancellation is inside the re-formation.

**Run.** Python 3.11.15 (recorded in `curved_ef.json`), `nice 19`, `PYTHONDONTWRITEBYTECODE=1`, about 3 minutes. V1's `probe_d5_check.py.txt` (sha256 `d13cf7c8…`) is copied to a scratch folder as `probe_d5_check.py`, and that folder is passed as the first argument. Hashes are in `_run_records/SHA256SUMS`.

**Limits.**
- This is an emulation, not a product run. The ported element follows the source as read, and the frame and solve path is V1's.
- The Decimal intended system has about 1e-60 relative error, negligible against 2^-53·cond.
- The realistic models are invented. Their magnitudes are typical, but they are not a survey of field models.

## 4. What changes in DESIGN (adopted in revision 5a, §4.3.1)

- **D5C-2 becomes:** a case with a contribution EF cannot re-form is demoted. Straight frames, springs, realized curved bends (objective re-formation) and user-stiffness elements (lateral zero) are all re-formed, so no contribution the ordinary route solves today is demoted for coverage.
- The curved-arc product tests (`b1_…`, `b2_…`) no longer change quality for coverage. They change only if EF fires on them, which the fixture diff reports.
- **K3a** gains `atan` for 0 < φ < π, with test vectors and the differential.
- **K-D5 tests** gain:
  - E1 and E6-class models, which must not demote;
  - the skew-elbow cantilever at k_X = 8.5, which must demote in both modes;
  - an EF_shared mutation (reuse the product's curved matrix), which must miss that case;
  - an expansion-joint (lateral zero) model, which must not demote.

## 5. For T4 and W1c

- The product's curved element is not objective on binary64 inputs, because its chord comes from R and trigonometric values rather than from the nodes. W1c should build H from the actual chord, as §2 step 6 does, or W1 at any precision would converge to a slightly non-objective element.
- The same applies to T4's null-space confirmation: the null space holds exactly only for arc-consistent geometry, which binary64 centres do not give.
- None of this blocks K-D5, which measures the defect.
