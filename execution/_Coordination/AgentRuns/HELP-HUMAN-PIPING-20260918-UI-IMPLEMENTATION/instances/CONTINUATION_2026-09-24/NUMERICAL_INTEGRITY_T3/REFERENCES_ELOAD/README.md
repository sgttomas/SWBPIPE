# T3 R1 addendum RF-ELOAD: independent references for element, eigen, support-effort, prescribed and generated loads

This is the return of a Type 2 TASK for tranche T3 (numerical integrity), dated 2026-09-26. The author is a fresh, product-code-blind reference author. The brief is `T3/TASK_BRIEFS/R1_ADDENDUM_ELOAD.md` at `3bf587cf4`, read with `_COMMON.md` and `R1_REFERENCES.md`.

**Status: candidate.** The package is frozen only when ROOT selects it after a separate V2-style refutation. It proposes no tolerance. Every comparison uses the unchanged form

```
|observed - expected| <= 1e-9 * max(|expected|, scale)
```

Here `scale` is the class scale of the value (§4). The exception is the cancellation cases (RF-ELOAD-CANCEL and RF-ELOAD-CE-CANCEL): there the per-value recommended scale, which the net load governs, is binding, as ROOT ruled for RF-CANCEL.

The package sits beside the frozen `T3/REFERENCES/**` (R1). That package was read for conventions only and was not touched.

## 1. Summary

| | |
|---|---|
| Cases | 48, in 8 families. Item 2 (generated self-weight) was dropped by ruling; see §2 |
| Published expected values | 2111, at 40 significant digits. 1275 of them are exact zeros, each with its derived zero scale |
| Represented-input values | 382: every generated-load case, plus the 2 cases on the represented basis |
| Negative controls | 213. 190 discriminate under the criterion; the other 23 are reported as non-discriminating (§7) |
| Arithmetic | Exact rationals. π → PI_Q, the rational of a 190-digit Machin value, with \|PI_Q − π\| = 5.5e-191, checked against Gauss–Legendre at 230 digits |
| Independent routes | Route A (tree integration plus force method) produces the values. Route B (direct stiffness with consistent loads) must agree exactly, and it does for every intended, represented, net-state and gross-state model |
| Represented basis | `RF-ELOAD-CANCEL-SEIS-G1e7` and `RF-ELOAD-CANCEL-SEIS-G1e8` (§7, finding 1) |
| Python | 3.11.15 (CPython), standard library only, about 3 s |

Files. The hashes are in `_run_records/SHA256SUMS`. Its paths are relative to the `REFERENCES_ELOAD/` root, so verify it from there: `cd REFERENCES_ELOAD && sha256sum -c _run_records/SHA256SUMS`.

- `references_eload.py`: the derivation, standard library only and deterministic.
  - `python3 references_eload.py` regenerates `references_eload.json` next to itself and prints the run summary.
  - `--case ID ...` runs the named cases only, and writes no JSON.
- `references_eload.json`: the candidate frozen values. For every case it gives:
  - the inputs, exactly as authored strings;
  - the model;
  - the method and the closed-form checks;
  - the expected values;
  - the class scales with their derivations, and the list of zero-valued keys with their scales;
  - `finite_input`;
  - the negative controls with their results;
  - where relevant, `expected_represented`, the per-quantity represented-versus-intended table, `generated_intensities` and `cancellation`.
- `_run_records/references_eload.stdout.txt`, `PYTHON_VERSION.txt` and `SHA256SUMS`.

## 2. Theory and adopted definitions

The package covers the implemented theory, which is a small-displacement, linear-elastic Euler–Bernoulli space frame:

- circular annular sections, with Iy = Iz = I and J = 2I;
- rigid or prescribed restraints on global DOFs;
- grounded linear springs on global DOFs.

These are references for that theory. They are not physical validation. Curved bends are excluded (W1c, with T4). The manager answered my definition questions from the accepted records and the source. The answers are recorded in `T3/MANAGER_NOTES/RF_ELOAD_DEFINITIONS.md` (at `28d96084f`, sha256 `1f236a2d…16eb`), and ROOT accepted them (`ROOT_RULINGS_V1.md`, at `edea3364d`).

| Topic | Definition adopted | Basis |
|---|---|---|
| Uniform and partial-span element loads | A uniform intensity q per unit **member** length over the fractions [a, b], in global axes or local axes. It enters as consistent Euler–Bernoulli (Hermite) equivalent nodal loads with fixed-end moments. Member actions are recovered with the fixed-end correction. For a uniform load this is the exact Euler–Bernoulli solution at the nodes and at every station | Brief ("theory, stated by the manager"); tp_phys_004/006/007; manager Q4, Q5 |
| Local frame | Every member has an explicit y_reference. x = (x_j − x_i)/L, y = normalize(y_ref − (y_ref·x)x), z = x × y. Local-load cases state the y_reference and publish the equivalent global intensity. Every member in the JSON carries a y_reference; the results do not depend on it unless a local load is present | tp_phys_005; manager Q3 |
| 50/50 lumping and lever rule | Negative controls only | Brief; manager Q5 |
| Thermal eigen axial load | P = EA·ε*, applied as the pair ∓P·e at i and j, with N = EA(ext/L − ε*). **Legacy:** ε* = α·ΔT. **0.4.0 resolved:** ε* = λ_fit·λ_thermal − 1, with engineering secant λ(T) = 1 + α_sec(T)(T − T_datum), λ_thermal = λ(T)/λ(T_install) and λ_fit = 1 + ΔL_fit/L. Each member has its own E per state | tp_phys_008, fixed_fixed_thermal_axial; T1 `generate_references.py` (thermal_datum_ratio, signed_fit_states); manager Q7 |
| Pressure thrust (straight members) | F_p = p·π/4·ID², with ID = OD − 2·t_eff. The cap pair {−F_p·e at i, +F_p·e at j} enters as nodal loads, and recovery subtracts the same pair. So the published N is the **effective** force EA·ext/L − F_p: 0 at a free end and −F_p when both ends are restrained. Reactions follow R = K·d − f, so the anchor of a free-ended member carries nothing | tp_phys_008; curved_bend_pressure_thrust_arc §7 (the straight limit); manager Q1 |
| Section, effective wall | A single effective wall t_eff = t_nom − m (m is the absolute mill tolerance, zero if none) for A, I, J, the thrust area and the mass | Manager Q6 |
| Constant effort | A positive nodal force along the + axis of the declared translational DOF, in every solved case, with no stiffness and no restraint row | constant_effort_support_applied_load; manager Q10 |
| Generated seismic (D-14) | For each global axis, w = g_factor · g · m′. Mass per length m′ = ρ_m·π/4(OD² − ID²) + ρ_c·π/4·ID² + ρ_ins·π/4((OD + 2t_ins)² − OD²). It is computed **exactly** from the inputs | tp_pmm_p3_occloadgen; ROOT D-14; manager Q6 |
| Generated wind (D-14) | w = p · Cs · (OD + 2 t_ins) along a global axis, per unit member length with no projection, on marked spans or sub-span extents only. It is computed exactly | tp_pmm_p3_occloadgen, tp_pmm_p3_subspan_wind_exposure; manager Q4 |
| Generated self-weight (item 2) | **Dropped as a solve-time family.** Self-weight is generated at authoring time: a model operation writes a distributed_force intensity into the document, and the solve reads it as an ordinary input. Item 1's user-given intensity covers it (`RF-ELOAD-UDL-WEIGHT-SKEW`). D-14's exact-from-inputs rule applies only to the solve-time seismic and wind generators | Manager Q2; ROOT D-14 scope ruling (`edea3364d`) |
| Prescribed motion (0.4.0) | Prescribed values on restrained DOFs, rotations in radians, in the same state as the loads. Reactions are signed, R = K·d − f, and act on the structure | T1 `generate_references.py` (prescribed_translation, prescribed_rotation); manager Q8 |
| Item 8a, "opposing" | Opposing **fixed-end moments** at the shared node, produced by loads in the same global direction (−Y) on the two adjacent spans, as in R1's CANCEL item 4 | Manager Q9 |
| Combination | Plain algebraic A + B and A − B of the component solutions. Mb is the magnitude of the combined moment vector | Manager Q11 |

**Inputs.** All inputs are invented and user-entered: E = 200 GPa and G = 80 GPa (M2 of the resolved L-frame uses 180 and 72 GPa, and the serial second member uses OD 0.25 m with wall 0.012 m). The N section is OD 0.2 m with wall 0.01 m, so ID = 0.18 m, EA = 380000000π N, EI = 1719500π N·m² and GJ = 1375600π N·m².

Generated-load inputs:

- section G: OD 0.2 m, wall 0.01 m, mill tolerance 0.00125 m, so t_eff = 0.00875 m and ID = 0.1825 m;
- densities: metal 7000, contents 900 and insulation 150 kg/m³;
- insulation thickness 0.03 m;
- g = 9.80665 m/s², user-entered;
- g-factors 0.3 (X), −0.15 (Y) and −0.2 (Z);
- wind pressure 520 Pa and shape factor 0.65.

Thermal inputs:

- legacy: α = 1.2e-5 1/K with ΔT = 75 or 150 K;
- resolved: datum 20 °C, installation 30 °C, operating 230 or 180 °C, with α_sec 11.5e-6 at 30 °C, 14e-6 at 230 °C and 13.4e-6 at 180 °C (only differences of temperature enter);
- fit: ΔL_fit = −0.002 m.

The thrust pressure is 2.5e6 Pa. None of these values is material, component, catalogue or code data. Every printed input parses back to exactly the rational used.

## 3. Methods and published quantities

**Published quantities** (convention-free):

- `u.<node>.U*` and `th.<node>.R*`: global nodal translations and rotations.
- `R.<node>.<DOF>`: the action of a rigid or prescribed restraint on the structure, R = K·d − f.
- `S.<node>.F*|M*`: the action of a spring on the structure, −k·u.
- Per member, at authored end i, the quarter points and end j:
  - `N.<m>.i|mid|j`: axial force, tension positive and fixed-end corrected.
  - `T.<m>.i|mid|j`: torque, with the sign of (θj − θi)·e.
  - `Mb.<m>.i|q1|mid|q3|j`: bending magnitude hypot(My, Mz) at the fractions 0, 1/4, 1/2, 3/4 and 1.
  - `tw.<m>` = (θj − θi)·e and `ext.<m>` = (uj − ui)·e.
- N, T, Mb, tw and ext do not change when i and j are swapped.

**Route A (the derivation).** An exact Euler–Bernoulli tree integration from a fully restrained root, which generalizes R1 M1 and M2.

- For member p→c (unit vector e, length L) take the action of the outer part on the inner part at station t:
  - F(t) = F_c + Σ q·|[max(t, A), B]|;
  - M(t) = M_c + (L − t) e×F_c + e × Σ q·((B − t)² − (max(t, A) − t)²)/2.

  F_c and M_c are the resultant of the subtree at c, including member loads, nodal loads, efforts, thrust caps and redundant actions.
- Then Δθ = C∫M dt, and Δu = θ_p×(L e) + (C∫(L − t)M dt)×e + (∫N dt/EA + ε*L) e, with C = e eᵀ/GJ + (I − e eᵀ)/EI.
- The integrands are piecewise polynomials of degree at most 3. Simpson's rule is exact for them on each piece between load breakpoints.
- Prescribed values at the root enter as rigid-body motion. Every other restraint and spring is a redundant, with compatibility u_k + X_k/k_k = δ_k, solved exactly.
- Member actions are read directly from F(t) and M(t), minus F_p for the effective thrust convention.

**Route B (cross-check, separate code).** Exact direct stiffness in global vector form. Its 12×12 member stiffness is built from P_a = e eᵀ, P_t = I − P_a and S = [e]×, with no local frame, since the section is circular. It uses:

- consistent Hermite loads with exact integrals over [a, b];
- the thermal pair and the cap pair;
- springs on the diagonal and prescribed DOFs partitioned out;
- recovery K_e u_e − f_eq, followed by statics from end i.

**Checks inside the script.**

- Route A equals route B exactly for every value of every intended, represented, net-state and gross-state model, and the global force and moment equilibrium residual is exactly zero.
- 26 closed-form checks are asserted, for example:
  - propped cantilever R = −3q_yL/8, M = −q_yL²/8 and θ_B = −q_yL³/(48EI);
  - fixed-fixed thermal N = −EAαΔT;
  - soft-spring thermal u = EAε/(EA/L + k) and N = −k·u;
  - serial N = −ε*L₁/(L₁/EA₁ + L₂/EA₂);
  - free thrust u = F_p L/EA;
  - settlement R = 12EIδ/L³;
  - FEM cancellation θ_S1 = net/(4EI/L_A + 4EI/L_B).
- The Q3- and Q9-rotated cases equal the rotated axis-aligned solutions.
- The combinations equal A ± B component by component.
- Before this package was written, the engine also reproduced the hand-calc values of tp_phys_006 (u_y = −7/500, θ_z = −13/3000, V_i = 4, M_i = 8) and tp_phys_009 (u_y = −0.070875, θ_z = −0.014625, M_i = 18, M(3) = 2.25, N = −12).

## 4. Scales (zero scales included)

Each value belongs to a class: translation, rotation, force, moment, twist or extension. This follows R1 §4.

- **Non-empty class.** The scale is the largest magnitude in the class. Nodal translations, rotations and support actions are measured as vector norms; member values as absolute values or bending magnitudes.
- **All-zero class.** L_c is the longest member. F_ref is the largest of: a nodal force norm (including efforts and caps), a member-load resultant |q|(b − a)L, and EA|ε*|. M_ref is the largest nodal moment norm.
  - Force: moment scale/L_c, or, when moments are also all zero, max(F_ref, M_ref/L_c).
  - Moment: force scale·L_c, or, when forces are also all zero, max(M_ref, F_ref·L_c).
  - Translation: rotation scale·L_c. Rotation: translation scale/L_c.
  - **When translations and rotations are both all zero** (for example the fixed-fixed thermal, thrust and prescribed-free cases), the translation scale is the largest of force scale·max(L/EA), the largest prescribed translation, the largest prescribed rotation·L_c and max|ε*|L. When moments are nonzero, moment scale·max(L²/(2EI)) is included too. The rotation scale is then translation/L_c. For example, TH-FF-LEG-AX has translation scale 5.4e-3 m (εL) and rotation scale 9e-4 rad.
  - Twist: moment scale·max(L/GJ). Extension: force scale·max(L/EA).
- Each case lists every zero-valued key with its scale (`zero_valued`), each class's derivation (`classes`) and `nonzero_below_class_scale`.
- **Cancellation cases.** Each row is `[key, expected, class, recommended_scale, gross_scale, governed_by]`.
  - `recommended_scale` is binding. It is the magnitude of the value's response to the net contribution alone, or the class scale where the net does not affect the value or the value is zero. It never exceeds the class scale (asserted).
  - The gross scale is the response to the gross contribution alone. It is shown for review only.

## 5. Reference accuracy

- Every value is an exact rational, with π → PI_Q.
- Each nonzero value is rounded once to 40 significant digits.
- Bending magnitudes are square roots of exact rationals, evaluated at 80 digits.
- Zeros are exact.
- All Decimal work runs at 60 digits or more, never at Python's default 28 digits.
- Represented-input solutions decode every authored input with `Decimal.from_float(float(x))` and then use exact arithmetic.
  - π is not an input and stays PI_Q.
  - Generated intensities in the represented variant are exact products of the decoded inputs.

`finite_input` = max |rep − int| / max(|int|, scale). Above 1e-9 the represented solution becomes the basis, by R1's RF-FINITE rule.

## 6. Families and cases

| Case (prefix `RF-ELOAD-`) | Item | Values | Zeros | NC discriminating | finite_input | Basis |
|---|---|---|---|---|---|---|
| `UDL-CANT-FULL-GLOB-AX` | 1 | 31 | 14 | 2/2 | 1.65176E-16 | intended |
| `UDL-CANT-PART-GLOB-AX` | 1 | 31 | 15 | 5/5 | 3.35685E-16 | intended |
| `UDL-CANT-FULL-LOC-Q3` | 1 | 31 | 12 | 4/4 | 1.76188E-16 | intended |
| `UDL-CANT-PART-LOC-122` | 1 | 31 | 12 | 6/6 | 1.64323E-16 | intended |
| `UDL-CANT-PART-GLOB-345` | 1 | 31 | 14 | 5/5 | 2.94495E-16 | intended |
| `UDL-PROP-FULL-AX` | 1 | 34 | 19 | 2/2 | 1.82741E-16 | intended |
| `UDL-PROP-PART-AX` | 1 | 34 | 22 | 4/4 | 1.82604E-16 | intended |
| `UDL-CONT2-AX` | 1 | 56 | 35 | 4/4 | 1.69333E-16 | intended |
| `UDL-CONT2-Q9` | 1 | 56 | 29 | 4/4 | 1.56878E-16 | intended |
| `UDL-LFRAME-3D` | 1 | 50 | 16 | 4/4 | 1.92626E-16 | intended |
| `UDL-WEIGHT-SKEW` | 1 | 53 | 21 | 2/2 | 1.92626E-16 | intended |
| `TH-FF-LEG-AX` | 3 | 37 | 32 | 3/3 | 1.03488E-16 | intended |
| `TH-FF-RES-122` | 3 | 37 | 28 | 5/5 | 6.25106E-17 | intended |
| `TH-SPRING-LEG-r1e-06` | 3 | 37 | 30 | 3/4 | 2.53343E-17 | intended |
| `TH-SPRING-LEG-r1e-12` | 3 | 37 | 30 | 4/4 | 7.32559E-17 | intended |
| `TH-SPRING-RES-r1e-08` | 3 | 37 | 30 | 6/6 | 1.56433E-17 | intended |
| `TH-SERIAL-RES-FIT` | 3 | 56 | 45 | 7/7 | 2.54885E-17 | intended |
| `TH-LFRAME-LEG` | 3 | 56 | 29 | 3/3 | 2.17626E-16 | intended |
| `TH-LFRAME-RES` | 3 | 56 | 29 | 5/5 | 1.81089E-16 | intended |
| `PT-FREE-AX` | 4 | 31 | 29 | 5/5 | 4.05783E-17 | intended |
| `PT-FF-AX` | 4 | 37 | 32 | 5/5 | 1.18732E-16 | intended |
| `PT-FREE-122-MILL` | 4 | 31 | 27 | 6/6 | 3.97609E-17 | intended |
| `PT-LFRAME-FF` | 4 | 56 | 29 | 5/5 | 2.32830E-16 | intended |
| `PT-TH-SPRING` | 4 | 37 | 30 | 6/6 | 2.62648E-17 | intended |
| `CE-ALONE` | 5 | 31 | 23 | 3/3 | 1.92626E-16 | intended |
| `CE-NODAL` | 5 | 50 | 18 | 3/3 | 1.91139E-16 | intended |
| `CE-CANCEL-G1e5` | 5 | 50 | 35 | 4/7 | 2.29634E-16 | intended |
| `CE-CANCEL-G1e8` | 5 | 50 | 35 | 6/7 | 2.29634E-16 | intended |
| `GEN-SEIS-CANT-AX` | 6 | 31 | 14 | 5/6 | 1.19293E-16 | intended |
| `GEN-SEIS-CONT2-Q9` | 6 | 56 | 25 | 5/6 | 1.44312E-16 | intended |
| `GEN-SEIS-WIND-LFRAME-FF` | 6 | 56 | 12 | 7/8 | 1.06303E-16 | intended |
| `GEN-WIND-MARKED-CONT2` | 6 | 56 | 41 | 4/5 | 1.23639E-16 | intended |
| `GEN-WIND-SUBSPAN` | 6 | 56 | 41 | 6/7 | 2.88410E-16 | intended |
| `GEN-WIND-SKEW-PROP` | 6 | 34 | 23 | 4/5 | 1.23639E-16 | intended |
| `PM-SINGLE-FF` | 7 | 37 | 28 | 2/2 | 2.13443E-16 | intended |
| `PM-ROT-PROP` | 7 | 34 | 20 | 2/2 | 2.13443E-16 | intended |
| `PM-DIFF-CONT2` | 7 | 56 | 32 | 2/2 | 2.13443E-16 | intended |
| `PM-ELOAD-CONT2` | 7 | 56 | 32 | 6/6 | 1.62140E-16 | intended |
| `PM-SKEW-122-FF` | 7 | 37 | 8 | 2/2 | 2.36789E-16 | intended |
| `CANCEL-FEM-G1e5` | 8 | 59 | 43 | 3/6 | 1.63976E-16 | intended |
| `CANCEL-FEM-G1e7` | 8 | 59 | 43 | 3/6 | 1.63976E-16 | intended |
| `CANCEL-FEM-G1e8` | 8 | 59 | 43 | 5/6 | 1.63976E-16 | intended |
| `CANCEL-SEIS-G1e5` | 8 | 31 | 23 | 1/4 | 1.42347E-11 | intended |
| `CANCEL-SEIS-G1e7` | 8 | 31 | 23 | 3/4 | 1.12034E-9 | represented |
| `CANCEL-SEIS-G1e8` | 8 | 31 | 23 | 3/4 | 1.27288E-8 | represented |
| `COMB-B-NODAL` | 9 | 56 | 27 | 0/0 | 1.92626E-16 | intended |
| `COMB-SUM` | 9 | 56 | 27 | 3/3 | 1.92626E-16 | intended |
| `COMB-DIFF` | 9 | 56 | 27 | 3/3 | 1.92626E-16 | intended |

Each case's `purpose` in the JSON gives its full definition. In brief:

- **RF-ELOAD-UDL (item 1, 11 cases).**
  - Cantilevers along X, full span and on [0.2, 0.7], each with the global load (300, −2000, 1200) N/m.
  - A local load on Q3·e_x, with y_reference (2,1,2), asserted equal to the rotated X-axis case.
  - A local load on (1,2,2)/3 over [0.25, 1], with y_reference (2,1,−2).
  - A global load on (3,4,0)/5 over [0.1, 0.6].
  - A propped cantilever, full and partial span, indeterminate in both planes and axially.
  - A two-span continuous beam with 9 m and 18 m spans, and the same beam rotated by Q9 (asserted equal to the rotated base).
  - A 3-D L-frame in which the loads on M2 twist M1.
  - Weight entered as a user intensity of −512.5 N/m on a skew propped member with an overhang.
- **RF-ELOAD-TH (item 3, 8 cases).**
  - Fixed-fixed members: legacy on X, and resolved on (1,2,2)/3, where ε* = 1.00294/1.000115 − 1 = 565/200023.
  - A soft axial spring at k/(EA/L) = 1e-6 and 1e-12 (legacy) and 1e-8 (resolved). Here N = −k·u, which is proportional to the soft spring.
  - A serial pair in which the restraint comes from the second member, with resolved thermal composed with a fit (ε* = 232353/100011500).
  - The fixed-fixed L-frame, legacy with one leg heated, and resolved with both legs heated and different E.
- **RF-ELOAD-PT (item 4, 5 cases).**
  - Free end: effective N = 0, u = F_p L/EA and zero reactions.
  - Both ends restrained: N = −F_p and R = ±F_p.
  - A skew free end on the effective (mill) wall.
  - The fixed-fixed L-frame, where the caps at the corner leave F_p(1,−1,0) unbalanced.
  - Thrust plus thermal on a soft spring.
- **RF-ELOAD-CE (item 5, 4 cases).**
  - The effort alone.
  - Efforts with nodal loads.
  - The effort nearly cancelling a nodal load at the same node: (−G, 0.3, +G) with G = 1e5 and 1e8, a gross/net ratio of G/0.3.
- **RF-ELOAD-GEN (item 6, 6 cases).**
  - Seismic on a cantilever along X, and on both spans of the Q9-rotated beam (global-axis intensities on skew members).
  - Seismic plus marked wind on the fixed-fixed L-frame.
  - Wind on the marked span of a two-span beam.
  - Sub-span wind on the disjoint extents [0.2, 0.7] and [0.8, 1.0], by consistent partial loads.
  - Wind on a skew propped member at cosine 3/5 to the wind, with no projection.

  Each case publishes `generated_intensities`: the intended exact value (with m′ and the seismic w as rational multiples of π), the represented exact product, and the binary64 product, each with its relative difference. For example, m′ = 20.24296875π kg/m, and w_X = 59.55471284765625π N/m, with a binary64-product relative error of 9.6e-16. Each case also publishes the represented-versus-intended difference for every quantity.
- **RF-ELOAD-PM (item 7, 5 cases).**
  - A single settlement of a fixed-fixed member.
  - Prescribed RZ and RY at the fixed end of a propped cantilever.
  - Differential settlement of a two-span beam.
  - The same settlement combined with element loads.
  - A skew fixed-fixed member with prescribed axial, transverse and torsional motion.
- **RF-ELOAD-CANCEL (item 8, 6 cases).**
  - **FEM.** S0 and S2 are fixed and S1 is translation-pinned. The spans are 2 m and 3 m, with w_A = 9s and w_B = 4s + 0.25 N/m, both along −Y, and M_z = −0.2 N·m at S1. The net at S1 is −0.3875 N·m. The gross/net ratios are 9.68e4, 9.68e6 and 9.68e7, for s = 12500, 1.25e6 and 1.25e7; w_A and w_B are exact in binary64.
  - **Generated seismic cancelled.** The seismic intensity w_s = −0.2·g·m′ (about −124.73 N/m) is cancelled by an authored (0, 0, D) N/m. D is −w_s(1 − 1/r), printed to 20 significant digits, for r = 1e5, 1e7 and 1e8. Every value is proportional to the net intensity.
- **RF-ELOAD-COMB (item 9, 3 cases).** The nodal case B on the continuous-beam structure, and the combinations A + B and A − B, with A = UDL-CONT2-AX.

## 7. Negative controls and findings

The controls are listed with each case. Each reports `discriminates`, the number of failing values, and the worst key with its ratio to the criterion (above 1 fails). The cancellation cases also report the result under the class scale and under the gross scale.

The defects modelled:

- lumped 50/50 loads;
- the lever rule;
- the fixed-end correction omitted in recovery;
- a partial load integrated over the full span;
- local applied as global, global applied as local, and a transposed frame;
- the thermal eigen term with its sign reversed, omitted, or omitted from recovery only;
- α·(T − T_install) and λ(T) − λ(T_install) in place of the datum ratio;
- the fit treated as additive, or omitted;
- the spring lost in a binary64 addition;
- the thrust recovered as wall force without correction, with its sign reversed, omitted, or computed on the steel area, the OD area or the nominal wall;
- the effort with its sign reversed, omitted, or modelled as a restraint;
- the prescribed motion omitted, or with its sign reversed;
- the binary64 left-to-right product (the D-14 hazard);
- insulation omitted from the mass or from the diameter;
- the nominal wall used for the mass, or everywhere;
- wind applied to unmarked spans;
- the projected wind form;
- float left-to-right sums of the cancelling contributions, in every order;
- the small contribution dropped;
- summed magnitudes in a combination, the wrong difference, and case B dropped.

Findings the designers should see:

1. **D-14 under cancellation: input rounding alone moves the basis.**
   - In `CANCEL-SEIS`, decoding the binary64 inputs (g = 9.80665, the densities, the section, D) moves the net response by 1.4e-11 of its scale at r = 1e5, by 1.12e-9 at r = 1e7 and by 1.27e-8 at r = 1e8.
   - By R1's rule, the r = 1e7 and 1e8 cases therefore cannot discriminate at 1e-9 on intended inputs, and their represented expectation (the exact product of the decoded inputs) is the frozen basis. An intended-input comparison would need the product to read the authored decimals exactly.
   - The D-14 hazard (NC-BIN64-PRODUCT) still fails against that basis, by 9.1× at 1e7 and 90.9× at 1e8. At r = 1e5 it stays inside the criterion (0.099).
2. **Without cancellation, the D-14 hazard cannot be detected at 1e-9.**
   - In all six non-cancelling generated cases, the binary64 product differs from the exact intensity by about 1e-15 relative (1e-7 to 1e-6 of the criterion).
   - D-14's exact-from-inputs rule is observable only where the generated load nearly cancels another contribution.
3. **Summing two nearly cancelling binary64 values is exact (Sterbenz).**
   - NC-FLOAT-SUM in CANCEL-SEIS, which uses the correctly rounded generated intensity plus the binary64 authored load, never discriminates: the maximum is 0.24 of the criterion at 1e8. The loss comes from the product and from input rounding, not from the final sum.
   - Three-contribution orders do lose the net. In CE-CANCEL, (−G, n, +G) and (n, −G, +G) fail by 9.9× at G = 1e8, and (−G, +G, n) is exact. In CANCEL-FEM, orders (A, n, B) and (n, A, B) fail by 7.7× at 9.7e7. They pass at 9.7e6 (0.48) and at 9.7e4, and (A, B, n) is exact.
4. **Only the shared-node rotation is net-governed in CANCEL-FEM.** Its 15 reactions and bending values are `mixed`.
   - Every cancellation control that discriminates does so under the recommended scale and also under the class scale here: the only nonzero rotation is the net-governed one.
   - Under the gross scale, none of the float-sum or product controls discriminate. This is the RF-CANCEL scale point again.
5. **Soft-spring thermal.**
   - N = −k·u ≈ −kε*L is proportional to the soft spring.
   - The lost-spring control fails by 537× at k/(EA/L) = 1e-8 and by 1.7e4× at 1e-12. It does not fail at 1e-6, where the binary64 addition happens to be exact.
   - Omitting the eigen term from recovery fails by 1e15 to 1e21×.
6. **Non-discriminating controls (23), reported rather than hidden:**
   - NC-LOST-SOFT at r1e-06;
   - NC-BIN64-PRODUCT in the six non-cancelling generated cases and in CANCEL-SEIS-G1e5;
   - float sums at the lower ratios and in the exact orders;
   - NC-FLOAT-SUM in all three CANCEL-SEIS cases;
   - NC-FLOAT-SUM-BIN64 at G1e5.

   A harness should use the discriminating controls of a case as its mutation tests.

## 8. Limits

- Curved bends are not covered (W1c). Also out of scope: shear deformation, releases, offsets, expansion-joint effective areas, nonlinear supports, dynamics and any code rule.
- Self-weight generation is not a solve-time family (§2). Stored intensities are ordinary inputs.
- Constant efforts are kept out of the combination, because they apply in every solved case.
- The defect models are models. For example, the lost-spring control uses a single rounded addition in the frame of the spring, and the binary64 product follows the formula order of the records (areas with `math.pi`, then m′·g_factor·g, and p·Cs·(OD + 2t_ins)). The product's own rounding differs in detail, not in order of magnitude.
- The resolved thermal cases take α_sec at table points exactly, with no interpolation. Temperatures enter only as differences.
- The zero scales of fully restrained axial cases are derived from the axial compliance and εL (§4). They are a choice about the comparison, not a tolerance.

## 9. Independence, what was read and what was run

**Independence.**

- No product source (`P/core/**`, `P/apps/**`), product test, product fixture or benchmark source was read, imported or called. That includes `core/product_physics/tests/fixtures/load_reference_states/`.
- The manager's definition note cites source line numbers. I used it only as the theory statement and did not open those lines.
- Every value comes from the stated theory, by route A, and is checked against route B. Both routes are my own code, written from the theory.
- The frozen `T3/REFERENCES/**` package was read for conventions only and was not modified.

**Read** (C/ = `P/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/`, H/ = `P/validation/hand_calcs/mechanics/`, VM/ = `P/docs/validation_manual/cases/mechanics/`, SHA-256 of the bytes read):

| Record | SHA-256 |
|---|---|
| `AGENTS.md` (root) | `c8ce87ef342902cb081bc659b26fc9a4edda1b6dba513814e5cb1e14e0b1dffd` |
| `agents/AGENT_TASK.md` | `1a13a5b00b3ce01ff8519efe6b46bcbe0cd6a5b7985e24282fa7efa2c57c8fb7` |
| `C/NUMERICAL_INTEGRITY_T3/TASK_BRIEFS/_COMMON.md` | `892a2e4e6f8b2e68967bd4c2a92996a141e93bf456bff12690248af91eac8fb3` |
| `C/NUMERICAL_INTEGRITY_T3/TASK_BRIEFS/R1_ADDENDUM_ELOAD.md` (at `3bf587cf4`) | `12524b3a365e27ee1d3a2ba7e8526c07cab6b160a05350898624556c06d09c28` |
| `C/NUMERICAL_INTEGRITY_T3/TASK_BRIEFS/R1_REFERENCES.md` | `0031453f75f30f342588783351c4db993ff34cebcd85115bf5ad47af55ddf99c` |
| `C/NUMERICAL_INTEGRITY_T3/REFERENCES/README.md` (frozen R1, conventions) | `5a89bad917d36aa71e3abb7f7ac237f7ba111af33878aac7fa1568a6017cc2a9` |
| `C/NUMERICAL_INTEGRITY_T3/STAGE0_MAP.md`, `STAGE1_PLAN.md` (keyword grep only) | `398868102890b0ed3bc6b6ab67badacc0114db6107e6f809a00f66773f975829`, `f5c75dcf5bb1bb4a093696edc5f21ad13150fe832ac4d6a3a1f11ea118c476bb` |
| `C/NUMERICAL_INTEGRITY_T3/MANAGER_NOTES/RF_ELOAD_DEFINITIONS.md` (at `28d96084f`) | `1f236a2d20fec1f83e53ba8191d5ae19b9fb4d46c393a5db75817c0ec2e416eb` |
| `C/NUMERICAL_INTEGRITY_T3/ROOT_RULINGS_V1.md` (only the lines added by `edea3364d`) | — |
| `H/README.md` | `68437a1356df8417cef726a1894b3535ba1e921390febbc6b8b3b4e6ba70d279` |
| `H/straight_pipe_weight_recovery.md` | `47d3d713d86f0c761fb83fd7a87d4083e669df5846251f2b72a66c79265cce9a` |
| `H/primitive_load_preparation.md` | `27516cf26d79e7241b943c9689e3b8fe2623d1def9dd8b9ac20484bd405620ac` |
| `H/tp_phys_004_load_to_resultant.md` | `0a668d9e8c9899ea0d1bea607edc83ba090f03ee67d9afca06a6f34f56a04dd5` |
| `H/tp_phys_005_oriented_load_to_resultant.md` | `2ce5852f038e63b55681fe7770e726619b6023987da6714d317d3782bc491d93` |
| `H/tp_phys_006_partial_span_load_to_resultant.md` | `fe7b35886eb40d3af3237cce40d7c1cb056772d513dae3a1a015b3ac4fd47eb0` |
| `H/tp_phys_007_station_sweep_resultants.md` | `a1238ea6e94e61e616e8d943577d5f69ac45b93f8ffab0b02ce59ab433d04132` |
| `H/tp_phys_008_thermal_pressure_axial_effects.md` | `7e262f37a6fc7987696771b9984dbbd2f578775f0d37f4dfa60a6ea2d1a2241b` |
| `H/tp_phys_009_combined_load_axial_effects.md` | `d803269c614eb40f62532b266c80129b8a1493403211957331ef1a4f9084caf8` |
| `H/fixed_fixed_thermal_axial.md` | `631512d5952514a34d809cb308a33c8c784eeea4f6292311f7d8e1932b1a87df` |
| `H/constant_effort_support_applied_load.md` | `6d8193316564b0e6cb8bffa630b6bd4008a3509db3a7361413b0a1b9ea4934cb` |
| `H/imposed_displacement_spring.md` | `7007b544fa1bd2a520a0953ad176a7e770f6b7659dafd03ba25ef25e375b9e9e` |
| `H/tp_pmm_p3_occloadgen_equivalent_static.md` | `4b602c33e0bb59e3d0a6c55a0ebad477c3df0b181ab2e3982ec68c7573ba5709` |
| `H/tp_pmm_p3_subspan_wind_exposure.md` | `065a8b1570999c612fa5c8b9a0b06ec8b6bdf80aebfcafc1385ac1a24a6b23ec` |
| `H/curved_bend_pressure_thrust_arc.md` (§1–§8, for the straight-limit definition) | `69bd7291abe3a1d00379523e1fa946a5e3706b57e7749cb52540a1c058bdf00f` |
| `H/curved_bend_distributed_load_fixed_end.md` (keyword grep only) | `9e9f6f893aa8916cf3a96209c7baf047a15875f07259c0dcc12f389634162e11` |
| `VM/mech-fixed-fixed-thermal-axial.md` | `02c3ac9f2a80b2147b67d4ac5fb04b161a38cd6bc6c292f7e2a990a31e8a677d` |
| `VM/mech-tp-pmm-p3-occloadgen-equivalent-static.md` | `55ee541455d3373f60a9fe411f4d15ecf85ccec2ada7054d3c69dc6475d0b5fe` |
| `VM/mech-tp-phys-008-thermal-pressure-axial-effects.md` | `2b4e867a68c17a75512791bd2ecada47e62a73277e709ac1f769a3287d84e0a0` |
| `C/LOAD_STATE_IMPLEMENTATION/ANALYTICAL_REFERENCE/BRIEF.md` (at `f3270ea79`) | `8cd830c8f325d0430a46cd1cb204775ebd5b8f63e8c62b628db7e604d396ddb5` |
| `…/ANALYTICAL_REFERENCE/CP2_EXTENSION/BRIEF.md` (at `f3270ea79`) | `6a92fdc99539377313ea0dd510de06a79da66e4c38ad4e26c3c572411f21fb2c` |
| `…/ANALYTICAL_REFERENCE/CP2_EXTENSION/RETURN.md` (at `f3270ea79`) | `42676c024fae64ca28225c89f11f41ef62b4d6e90b9de92e36fcc2bcc7834158` |
| `…/ANALYTICAL_REFERENCE/_run_records/generate_references.py` (at `f3270ea79`) | `429930281dfa3112f993e4536ce4dc4e7f26939890c6f153653275de9c690dcf` |

The other validation-manual mechanics pages were listed but not read. They are generated boilerplate that points to the hand calcs. The T1 files were read with `git show f3270ea79:<path>`; T1's worktree and branch were not touched.

**Definition questions asked** (batch 1, 11 questions, all answered in `RF_ELOAD_DEFINITIONS.md`):

- Q1: thrust N convention and area;
- Q2: generated self-weight;
- Q3: local frame;
- Q4: skew intensity and wind projection;
- Q5: the route by which generated loads enter the solve;
- Q6: the section for generated cases;
- Q7: legacy and resolved thermal;
- Q8: prescribed motion and rotations;
- Q9: "opposing" in item 8a;
- Q10: constant-effort sign and scope;
- Q11: combination semantics.

**Ran.** Everything ran single-threaded under `nice`, with standard-library Python 3.11.15, and with prototypes in `<scratch>`:

- `python3 -B references_eload.py` in this directory: about 3 s, exit 0. The stdout is in `_run_records/references_eload.stdout.txt`.
- The same run repeated in `<scratch>`: `references_eload.json` was byte-identical, which confirms determinism.
- `sha256sum`.

**Not done.**

- No cargo, npm, product build or test was run.
- No Git write was made.
- No file outside `T3/REFERENCES_ELOAD/**` was written.
- No frozen reference, fixture, hash or protected criterion was edited.
- No tolerance was proposed.
