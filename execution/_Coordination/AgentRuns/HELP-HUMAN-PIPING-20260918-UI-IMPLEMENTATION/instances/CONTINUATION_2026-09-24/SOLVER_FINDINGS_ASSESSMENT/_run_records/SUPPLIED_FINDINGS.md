# SWBPIPE solver engine: expert review findings

**For:** the agent responsible for the SWBPIPE code (`projects/chirality-piping`).
**Reviewed revision:** `sgttomas/chirality` `main` at `bc6d3459b` (2026-09-24).
**Method:** six independent reviewers, each working as a senior pipe-stress engineer
and solver developer, audited one area of the engine from the code itself:
elements and numerics, loads, supports and nonlinear behaviour, load cases and
stress, verification, and app, data and export. Each finding was checked in the
code with file and line references. Most were reproduced by running small models
through the live solve entry point the desktop app uses
(`run_linear_static_preview`) at real pipe sizes. Where several reviewers found
the same problem independently, the master list says so. The headline Critical
items were also re-checked by hand against the source. Probe programs are under
`scratchpad/review/` in the reviewing session; the numbers quoted come from those
runs.

Project governance and policy documents were deliberately ignored. The review
judges the engineering.

## Verdict

The solver core has sound foundations. The curved-bend element, the coordinate
transformation, the straight element, exact rigid restraints, the contact logic
of the nonlinear supports, and the RCM plus skyline factorization were all
checked and found correct. The expansion-loop hand calculation reproduces exactly.

The engine that the app actually drives is not yet fit for real piping models.
Five problems on their own make results wrong or unobtainable on ordinary systems:

1. The pressure model.
2. Bends solved as straight chords.
3. Unstable models reported as solved.
4. Reactions without components or moments.
5. A text gate that blocks any model built from real data.

Several capabilities every hot system needs are also missing, or present but
not connected: imposed nozzle and anchor movements, spring-hanger preload,
code stress categories, rigid elements and component weights. The verification
suite passes, but mostly because it tests component crates with invented tiny
values on axis-aligned geometry, which hides exactly these defects.

## How to use this document

Work the master list in priority order. Each master item (M-nn) lists the
source finding IDs; the full evidence, file:line references, probe numbers and
recommended fixes for each source ID are in the appendices, grouped by area.
Recommended fixes use standard engineering practice. Where a fix changes an
existing "hand calculation" that encodes the wrong model (notably pressure),
the reference must be corrected too.

## Master list

### P0: wrong answers, or real models that cannot be solved (fix first)

| ID | Problem | Source findings | Consequence on a real line |
|---|---|---|---|
| M-01 | **Pressure model.** A closed-end thrust pair on every pressurised element acts as an always-on axial self-strain without the Poisson term, and the longitudinal pressure stress is suppressed whenever that thrust is active (`include_pressure_longitudinal = !pressure_thrust_active`). | ELEM-3, LOAD-01, LOAD-10, STR-01, STR-04, VER-02, LOAD-09 | Free-to-grow pipe reports 0 MPa longitudinal pressure stress (PD/4t ≈ 30 MPa at 5 MPa, 8"). An anchored run reports compression with spurious anchor loads (93–161 kN). Straight pipe and bend disagree at every tangent point. The reference hand calculation (tp_phys_008) encodes the same wrong model. |
| M-02 | **Bends from the app are straight chords.** Bends default to `mechanics_geometry_only`; the app never sets the curved element, so *k*, SIF and arc weight are lost. The curved element also needs the chord to match R and θ to 1e-6 rad, and there is no bend-at-corner input. | ELEM-1, APP-2, APP-3, ELEM-9 | L-bend probe: anchor force 58% high, anchor moment 31% high, elbow stress about 23% low, bend weight 10% low. Results are identical to having no bend. |
| M-03 | **Mechanisms are not detected.** The pivot guard is an absolute 1e-12, negative pivots are accepted, and the "six ground DOFs" pre-check is too weak. Unstable models publish `MECHANICS_SOLVED`. | ELEM-4, VER-01, SUP-17 | A skewed run held only in translation "solves" with rotations of 1e9–1e12 rad. Sparse and dense give opposite signs, and no diagnostic is raised. |
| M-04 | **Real data cannot be solved.** Every record's provenance text must contain "invented" or "cleared" (`validation.rs:1320`). | APP-1 | A model with a real P&ID reference returns no results. The app's own default provenance fails the gate. |
| M-05 | **Reactions are only a translational force magnitude.** Components, signs and all moments are discarded (`slot < 3`). Two supports on one DOF each report the full reaction. | ELEM-8, STR-08, SUP-08, APP-4, VER-05, STR-15 | Anchor and nozzle loads cannot be reported, checked or handed off. An anchor carrying 3000 N·m of torque reports 0 N. |
| M-06 | **Nonlinear iteration cap of 4** (one pass only confirms convergence). Non-convergence blocks the whole analysis. | SUP-01, SUP-16, SUP-18 | A rack line with 15–60 resting supports needs 6–14 iterations; 5 friction shoes already fail. With the cap at 100, every probe converged correctly. |
| M-07 | **Expansion joints do nothing.** The joint stiffness sits in parallel with a full-stiffness pipe on the same span. Joints created in the app are ignored. The element is not invariant under rigid rotation, and there is no tied option. | ELEM-2, ELEM-7 | Thermal anchor load 1.86 MN against about 1.8 kN expected. Pressure thrust 1.9 kN against 40 kN (P × effective area). |
| M-08 | **SIF use is wrong.** The review row is stress × SIF × *k* in the default mode. There is one SIF per bend (no in-plane/out-of-plane split), torsion is ignored, the factor is applied at one node only, and SIF rows are then combined as if linear. | ELEM-5, STR-02, STR-03 | Probe: 699 MPa against 47 MPa correct. A down − up range gives 0 MPa where the true range is 86 MPa. |
| M-09 | **Supports with a nonlinear block silently lose their linear restraints.** | SUP-02 | A guided rest modelled as one support moves 30.7 mm laterally under a 1 kN side load. |
| M-10 | **No imposed displacements.** Nozzle and anchor thermal movements and settlement are unavailable (the kernel supports prescribed DOFs, but the live path never uses them). The UI's `imposed_displacement` option makes the whole model unsolvable. | LOAD-03, SUP-10 | Every hot system with equipment connections needs these. |
| M-11 | **Constant-effort force is applied in every load case.** It doubles in combinations, and its reaction is reported as 0 N. | LOAD-02, SUP-04 | A thermal-only case lifts the pipe 12–21 mm. W+T carries twice the hanger force. |
| M-12 | **Spring hangers carry no preload.** Installed, cold and hot loads are only echoed back, and there is no hanger design. | LOAD-05, SUP-03, SUP-07 | Probe: hanger carries 409 N against a 675 N design load, and the pipe sags 8 mm where it should sag 0. |

### P1: capability any real system needs

| ID | Problem | Source findings |
|---|---|---|
| M-13 | **No code stress framework.** There are no load-case types (SUS, OPE, EXP, OCC, HYD), no sustained stress, no expansion stress range using the resultant of SIF-multiplied moments plus torsion, no occasional stress, no allowables and no stress ratios. `recover_stress_range` exists but the live path never calls it. The combination algebra is too thin. | STR-06, LOAD-14, APP-9, STR-09 |
| M-14 | **The summary stress is wrong.** It sums \|σby\| + \|σbz\| (up to √2 high against the resultant moment) and ignores torsion. | STR-05 |
| M-15 | **Nonlinear superposition, silently.** Linear combinations of cases with nonlinear supports are added with no warning (45.4 mm against 16.6 mm true). There is no sustained analysis in the operating support state. | STR-07, SUP-11, SUP-12 |
| M-16 | **Thermal and material basis.** α and E come from one basis per case, so expansion stress uses the hot modulus. There are no per-line operating conditions (T1..Tn, P1..Pn); ΔT and pressure are typed pipe by pipe. | LOAD-07, LOAD-08, APP-8 |
| M-17 | **No rigid element and no component weights.** Valve and flange weight, centre of gravity and rigidity are ignored, and there is no reducer element. | ELEM-6, LOAD-04, ELEM-14 |
| M-18 | **Friction.** It is per global DOF, not vector Coulomb (the probe shows the wrong sliding direction), and friction cannot sit on a lift-off rest. | SUP-05, SUP-06 |
| M-19 | **Restraint types.** Global axes only: no skewed or pipe-local restraints and no connected nodes. `vertical_support` is locked to UZ. There are no double-acting gaps or limit stops, and stiffness entered on a rigid support is ignored. | SUP-09, SUP-13, SUP-14, SUP-15 |
| M-20 | **Nozzles.** No nozzle flexibility, no loads in nozzle-local axes, and tees have no branch Ze and no local flexibility. | SUP-10, ELEM-12 |
| M-21 | **Hydrotest.** No working path; hydrotest pressure is silently ignored, and there is no hanger locking. | LOAD-06, SUP-12 |
| M-22 | **The CAEPIPE hand-off is not real.** The .mbf uses invented record types with no sizes, materials, fittings, temperatures or loads; partial restraints become GUIDE; imperial coordinates are mislabelled mm. There is no real results import or comparison, and the "external run" panel fabricates rows against the user's model. | APP-5, APP-6, VER-06 |
| M-23 | **Libraries and tables.** Libraries cannot populate the model, and there are no pipe schedule, fitting or material-versus-temperature tables. | APP-7 |
| M-24 | **Units.** A blank project is SI metres only, and the default thermal unit `C` is rejected by the solver. Common units are missing (bar, kN, N/mm, 1/°F). | APP-10 |
| M-25 | **Model building is too slow for real work.** Routing is by absolute coordinates only, with no bulk edit, table paste or line numbers. | APP-11 |
| M-26 | **The report and results are not issuable.** There is no input echo, stress-versus-allowable table, restraint-load table or hanger table; results are a flat row list. | APP-12, STR-12 |
| M-27 | **The verification does not cover the live path.** Benchmarks exercise component crates, and the only live-path suite is one 2 m cantilever. There is nothing for tees, code stresses, imposed movements, real-geometry friction, hangers or temperature-dependent 3D systems. Several cases are tautological, and CI does not run the Rust tests. | VER-03, VER-04, VER-08, VER-09, VER-10, VER-11 |

### P2: accuracy and usability in common cases

| ID | Problem | Source findings |
|---|---|---|
| M-28 | Wind is not projected onto the pipe and has no height profile. Seismic acts in one sense per case with all axes together, and there are no concentrated masses. | LOAD-11, LOAD-12 |
| M-29 | No cold spring or cut-short. | LOAD-13 |
| M-30 | **Section properties.** The mill-tolerance wall is used for stiffness and weight (flexibility analysis uses nominal). There is no corrosion allowance, one Z serves sustained and expansion stress, and there is no branch Ze. Pressure stresses use the mean radius. | ELEM-11, LOAD-09, STR-11 |
| M-31 | No shear deformation, so short, stubby legs are 1.8–8 times too stiff laterally. | ELEM-10 |
| M-32 | Global stiffness is assembled dense even on the "sparse" path, so memory grows as O(n²) (2.8 GB at 2000 nodes). | ELEM-13 |
| M-33 | Summary maxima come only from the first load case, and nothing warns when combinations mix cases solved at different moduli. | STR-10, STR-09 |
| M-34 | Absolute tolerances and 6-decimal publication rounding are scale-dependent (a 1.28e-6 rad rotation publishes as 1e-6). | VER-07, STR-13 |
| M-35 | Concentrated moments created in the UI are rejected by the solver, and the self-weight generator is a stale snapshot. | LOAD-15, LOAD-16 |
| M-36 | PCF export writes straight pipe only, with a non-numeric bore. The native JSON export is a manifest, not the model. | APP-13, APP-14 |

### P3: polish

| ID | Problem | Source findings |
|---|---|---|
| M-37 | End and station rows use opposite sign conventions, bend end rows are in the chord frame, the bend maximum is sampled at 5 stations only, and `y_reference` is mandatory for circular sections. | ELEM-15, STR-14, ELEM-16 |
| M-38 | Minor app and authoring inconsistencies. | APP-15, SUP-19 |

## Suggested order of work

1. **Stop wrong answers.**
   - M-03: relative pivot tolerance (for example |dᵢ| < 1e-10 · max Kᵢᵢ), reject non-positive pivots, name the free node and DOF, and add an equilibrium residual check on every solve.
   - M-01: sustained longitudinal pressure stress always from pressure. Thrust only as the unbalanced resultant at ends and changes of direction. Optional Bourdon and pressure elongation with the Poisson term, ε = pD(1−2ν)/(4tE). Correct tp_phys_008.
   - M-08: remove *k* from stress; use in-plane and out-of-plane SIFs on their own moments, plus torsion.
   - M-05: full signed reaction components and moments.
   - M-04: remove the provenance gate.
2. **Make the app build what the solver can solve.**
   - M-02: create bends as curved elements from a corner node with auto tangent points.
   - M-09: keep a support's linear restraints alongside its nonlinear behaviour.
   - M-07: the joint replaces the pipe over its length, with a tied option.
   - M-06: raise the iteration cap to 50–100, with a real convergence criterion.
3. **Add what hot systems need:**
   - M-10: imposed displacements.
   - M-11 and M-12: hanger preload and constant-effort force applied only in the cases where they belong.
   - M-17: rigid elements and weights.
   - M-16: operating conditions per line, with the cold modulus for the expansion range.
4. **The code stress framework** (M-13, M-14, M-15), with user-entered allowables per category.
5. **Verification at real scale through the live path** (M-27), including CAEPIPE side-by-side models, alongside items 1–4, not after them.
6. **Hand-off, libraries, units, model-building speed and the report** (M-22 to M-26).

## Done well (keep)

- **The curved-bend macro-element formulation.** With *k* = 1 it matches a 64-segment straight model to five significant figures, and it gives correct live L-bend thermal results against independent calculation (8–10 significant figures).
- **The frame kernel and straight element.** The coordinate transformation is exact on skewed members for any `y_reference`.
- **Exact rigid restraints.** Restrained DOFs are eliminated, not modelled as stiff springs.
- **The sparse RCM plus skyline factorization pipeline.**
- **Nonlinear support contact logic.** It is correct, and non-convergence fails loudly rather than silently. With a sane iteration cap every probe converged to the physically correct state.
- **Consistent (Hermite) distributed loading, arc-length bend loads, and exact thermal restraint** (−480 MPa matches EαΔT).
- **No silent defaults.** Missing inputs block the solve instead of being filled in.
- **Undoable, audited model edits,** and a working path from the hanger library to the model.
- **The expansion-loop hand calculation** is correct and reproducible.

---

# Appendices: full findings by area


---

## Appendix — Part A: Element formulation, model topology and solver numerics (prefix ELEM-)

Reviewer scope: straight pipe element, curved bend, tees and branches, rigid components, reducers, expansion joints, coordinate transformation, assembly and solve, and recovery of end and station forces.

Code reviewed: `core/product_physics/src/lib.rs` (live path), `core/product_physics/src/validation.rs`, `core/solver/frame_kernel`, `core/solver/curved_bend`, `core/solver/sparse_direct`, `core/solver/straight_pipe`, and `apps/desktop/src/features/component-creation/componentIntent.ts`. All paths below are relative to `projects/chirality-piping/`.

Probes: each probe ran real models through the public `run_linear_static_preview_with_mode` entry point. The probes are in `scratchpad/review/elem_probe/` (Rust driver `src/main.rs`, direct-kernel probes `src/bin/ejrigid.rs` and `src/bin/singular.rs`, Python model generators under `py/`). The full output is in `scratchpad/review/elem_probe/probe_output.txt`.

Common probe data: 8 in Sch 40 (OD 219.1 mm, t 8.18 mm), E = 200 GPa, G = 77 GPa, α = 12e-6 /°C. For a long-radius elbow (R = 304.8 mm), h = tR/r² = 0.224, k = 1.65/h = 7.36 and i = 0.9/h^(2/3) = 2.44.

---

## Summary table

| ID | Title | Severity |
|---|---|---|
| ELEM-1 | Bends created in the desktop app are solved as a straight chord, with no flexibility factor and no SIF | Critical |
| ELEM-2 | Expansion joint stiffness sits in parallel with a full-stiffness pipe element, so the joint does nothing | Critical |
| ELEM-3 | Pressure is applied as an axial self-strain on every element; the longitudinal pressure stress is lost or has the wrong sign | Critical |
| ELEM-4 | Mechanisms are not detected: the pivot guard is an absolute 1e-12, negative pivots are accepted, and the six-DOF check is weak | Critical |
| ELEM-5 | In the default bend and tee mode the flexibility factor k multiplies the stress (i·k) | High |
| ELEM-6 | No rigid element: valve and flange weight, centre of gravity and rigidity are ignored | High |
| ELEM-7 | The expansion joint element is not invariant under rigid-body rotation, and there is no tied-joint treatment | High |
| ELEM-8 | Support reactions are reported only as a translational force magnitude, with no components and no moments | High |
| ELEM-9 | Curved-bend geometry: y_reference sets the bow side, tangency is not checked, the angle is optional, and there is no corner-node input | Medium |
| ELEM-10 | Straight element is Euler-Bernoulli with no shear deformation; short stubby legs are 1.8 to 8 times too stiff laterally | Medium |
| ELEM-11 | Section properties: mill tolerance reduces stiffness and weight, there is no corrosion allowance, and one Z serves both sustained and expansion | Medium |
| ELEM-12 | Tees are rigid node junctions: no branch Ze, no local nozzle or branch flexibility | Medium |
| ELEM-13 | The global stiffness is assembled dense even on the "sparse" path: memory grows as O(n²) | Medium |
| ELEM-14 | No reducer element; the end sizes are ignored | Low |
| ELEM-15 | Recovery conventions: end rows and station rows use opposite signs, bend end rows are in the chord frame, and bending stresses are summed arithmetically | Low |
| ELEM-16 | y_reference is mandatory on every pipe, although it has no effect on a circular section | Low |

---

## Findings

### ELEM-1: Bends created in the desktop app are solved as a straight chord, with no flexibility factor and no SIF

**Severity:** Critical

**Evidence**

- The desktop bend payload contains only geometry: `bend_pipe_ref`, `bend_radius`, `bend_angle`, `bend_plane_orientation` and the source (`apps/desktop/src/features/component-creation/componentIntent.ts:320-327`). The payload built at `:263-270` has no `mechanics_interface`. `componentModifiers` returns `null` for every kind except `expansion_joint` (`:371-372`). So a desktop bend has no `flexibility_factor_user_value`, no `sif_user_value` and no `solver_consumption`. No editor in `apps/desktop/src` writes `mechanics_interface.solver_consumption`; it is only read for display (a grep finds only `modelView.ts`, `ModelTree.tsx`, `PropertyInspector.tsx` and `ReportPanel.tsx`).
- `is_curved_bend_macro_component` defaults the mode to `"mechanics_geometry_only"` (`core/product_physics/src/lib.rs:3765-3769`, via `component_solver_consumption` at `:3756`). Only `curved_bend_macro_element` removes the chord element (`:3491`, `if !curved_bend_pipe_ids.contains(...)`). Every other bend span is assembled as an ordinary `StraightPipeElement` between its two nodes.
- In that mode `bend_stress_modifier` returns `None` when the modifiers are absent (`let modifiers = component.modifiers.as_ref()?;` at `:7745`), so a desktop bend gets no SIF on stress either.
- Distributed weight on a chord span is integrated over the chord length, not the arc length (`add_uniform_element_loads`, `:6643-6646`, uses the straight element).

**Probe** (L-bend in plan, legs 6 m to the corner, both ends anchored, ΔT = 150 °C on every span, 8 in Sch 40, LR elbow):

| Model | Anchor force \|F_A\| | Anchor moment \|M_A\| | Max stress (no SIF) |
|---|---:|---:|---:|
| Sharp corner (typical user model) | 5113 N | 10 847 N·m | 40.0 MPa |
| Desktop bend (chord, `mechanics_geometry_only`) | **5389 N** | **11 246 N·m** | 41.5 MPa |
| Curved macro, k = 1 | 5299 N | 11 113 N·m | 41.0 MPa |
| Curved macro, k = 7.36 (correct) | **3417 N** | **8 553 N·m** | 31.5 MPa (i·M/Z at bend: 47–50 MPa) |

- Compared with the correct elbow model, the desktop chord bend overstates the anchor force by 58% and the anchor moment by 31%.
- It also *understates* the elbow stress. It reports 38.9 MPa on the bend span with no SIF, while the correct i·M/Z is about 50 MPa at the arc crown (20.6 MPa base × 2.44). That is 23% non-conservative at the fitting that governs.
- Weight on the chord span is 0.431 × w instead of 0.479 × w, so about 10% of the elbow's own weight is lost.
- As a cross-check, the macro element with k = 1 matches a 64-segment polygon of straight elements to five significant figures (5299.1 N against 5299.1 N; 11 113.0 N·m against 11 113.1 N·m).

**Why it matters to a stress engineer.** Elbow flexibility is the main reason piping layouts absorb thermal growth. With k ignored, expansion reactions on anchors and nozzles come out tens of percent high, which leads to loops and supports that are not needed. Because the SIF is also missing, the elbow, which is usually the location that governs the displacement stress range, is under-predicted. Every bend a user draws in the product behaves this way. The one correct element exists but cannot be reached from the UI.

**Recommended fix**

- Make the curved-bend macro element the only realization of a bend, and delete `mechanics_geometry_only` for bends.
- Generate the bend from a corner (tangent-intersection) node with radius and angle, the standard CAESAR II, CAEPIPE and AutoPIPE input, creating the near, mid and far points automatically.
- Default k and i from the selected code's flexibility and SIF table (B31.3 Appendix D, B31.1 Table D-1, or B31J), with a user override.
- Integrate weight over the arc length.

---

### ELEM-2: Expansion joint stiffness sits in parallel with a full-stiffness pipe element, so the joint does nothing

**Severity:** Critical

**Evidence**

- `build_model` pushes a straight frame element for every pipe except curved-bend spans (`lib.rs:3491-3497`). The expansion joint `UserStiffnessElement` is then built on the *same* two nodes (`build_expansion_joint_user_stiffness_elements`, `:3502-3503`, `:3643-3737`), and both are assembled (`assemble_global_stiffness_with_user_elements`, `:939-941`; sparse path `:2639-2659`). The span on which the joint sits is never excluded.
- Every missing input silently `continue`s: no mechanics interface (`:3666`), no geometry, no modifiers, no pipe ref, or any of the four stiffnesses missing (`:3671-3719`). In each case the only output is a validation *warning* (`validation.rs:1028-1044` and the stiffness-missing warnings), and the joint span stays a rigid pipe.
- The desktop joint form writes the four stiffnesses but no `mechanics_interface` (`componentIntent.ts:263-270`). A joint created in the app is therefore never consumed, and its pressure thrust is dropped as well (`expansion_joint_pressure_thrust_inputs_by_pipe` needs the same mode, `lib.rs:6779`).

**Probe:** a straight 10.5 m line with anchors at both ends and a 0.5 m joint span in the middle; user k_axial = 1e5 N/m; ΔT = 150 °C on the pipes. An untied joint should give an anchor load of about k_ax × δ = 1e5 × 0.018 m ≈ 1.8 kN.

| Case | Anchor axial load |
|---|---:|
| No joint | 1 858 378 N |
| Joint from the desktop form (no interface) | 1 858 378 N (identical; warning only) |
| Joint with `mechanics_geometry_and_user_flexibility` | 1 858 382 N |
| Same, k_ax = 1e3 N/m | 1 858 378 N |
| Pressure 1 MPa, A_eff = 0.04 m² (expected untied thrust 40 kN) | **1 905 N** |

**Why it matters to a stress engineer.** A bellows is installed precisely to decouple the anchors. Here the anchors see the fully restrained thermal load, about 1000 times too high, and almost none of the pressure thrust, about 20 times too low. The pressure-thrust error is the dangerous one: main anchors on bellows systems are sized for pA_eff, and this program reports a small fraction of it.

**Recommended fix**

- Replace the pipe span with the joint element; never assemble them in parallel.
- Model the bellows as a zero-length or short spring element with lumped bellows weight at its end nodes.
- Make every missing joint input a blocking error.
- Emit `mechanics_interface` from the desktop form, or remove the mode switch altogether.

---

### ELEM-3: Pressure is applied as an axial self-strain on every element; the longitudinal pressure stress is lost or has the wrong sign

**Severity:** Critical. This overlaps with the loads and stress area; it is reported here because it is a recovery defect.

**Evidence**

- Every element-targeted `pressure` load creates a thrust pair −pA_i at node i and +pA_i at node j (`build_pressure_thrust_loads`, `lib.rs:6715-6752`; applied at `:6826-6866`).
- In recovery the pair is added back to the axial force like a thermal strain (`corrected_local_forces_for_axial_effects`, `:6959-6991`, `corrected[UX] += axial_load` at `:6974`).
- The pD/4t term is then suppressed whenever thrust is active (`include_pressure_longitudinal = !pressure_thrust_active`, `:1756`). It is always active for a pressurised pipe.
- The project's own hand calculation, `validation/hand_calcs/mechanics/tp_phys_008_thermal_pressure_axial_effects.md`, codifies the approach: fixed-fixed anchors carry +pA like thermal restraint.

**Probe:** 8 in pipe, 5 m, P = 1 MPa. Here pA_i = 32.28 kN, pA_i/A_m = 5.96 MPa and pD/4t = 6.70 MPa.

| Case | Reported axial stress | Anchor load | Correct (B31 convention) |
|---|---:|---:|---|
| Cantilever with a closed free end | **0.0 MPa** | 0 | +5.96 to 6.70 MPa tension |
| Anchored at both ends | **−5.96 MPa** (compression) | **32.3 kN** | +5.96 MPa; anchor load 0, or 12.9 kN with the Poisson (Bourdon) effect |
| Free-end elongation | 0.149 mm | – | 0 (Bourdon off) or 0.060 mm (σ_L(1−2ν)L/E) |

The curved-bend path reports a wall tension of +pA (per the comment at `:6856-6867`), so straight and curved spans disagree on the same physics.

**Why it matters to a stress engineer.** The sustained longitudinal stress S_L must include pD/4t. On a free-draining line it is lost entirely, which is non-conservative. Anchors on a closed straight line are loaded with a full pA_i that does not exist, and pressure elongation is overstated 2.5 times because the Poisson term is missing.

**Recommended fix**

- Treat closed-end pressure as a stress term, pD/4t or pA_i/A_m, in S_L, and do not apply it as a load.
- Apply unbalanced thrust pA_eff only at expansion joints (untied), at open ends and at equipment where the user asks for it.
- Offer the Bourdon option as a pressure-elongation strain σ_L(1−2ν)/E on straight pipe, plus the bend Bourdon rotation.
- Fix the benchmark.

---

### ELEM-4: Mechanisms are not detected: the pivot guard is an absolute 1e-12, negative pivots are accepted, and the six-DOF check is weak

**Severity:** Critical

**Evidence**

- Dense solve: the guard is `DENSE_SOLVE_ZERO_PIVOT_GUARD = 1.0e-12`, absolute (`frame_kernel/src/lib.rs:21, 935, 965`).
- Sparse LDLᵀ: `SPARSE_SOLVE_ZERO_PIVOT_GUARD = 1.0e-12`, absolute and with no pivoting (`sparse_direct/src/lib.rs:21, 418`). Negative pivots are only *counted* (`:424-431`).
- `product_physics` copies `nonpositive_pivot_count` and `pivot_condition_ratio_estimate` into a metadata string and never raises a diagnostic (`lib.rs:2339-2342`, `:2382-2426`).
- The pre-check is only "fewer than six distinct ground DOFs" (`lib.rs:902-924`). Two nodes restrained in UX, UY and UZ pass it, although rotation about the line through them is free.
- In SI units, K_ii for pipe is around 1e5 to 1e9. A roundoff pivot of about ε·K_ii ≈ 1e-11 to 1e-7 always clears 1e-12.

**Probe**

- Through the product path, an inclined 6 m pipe along (1,2,3), guided (UX, UY, UZ) at both ends, with weight and a 10 N·m RX moment:
  - Status `MECHANICS_SOLVED` with no diagnostic.
  - The midpoint rotations are about 1.8e9 to 5.4e9 rad, with opposite sign in the sparse and dense modes.
  - Reported reaction magnitudes sum to 5989 N against 6000 N of applied weight, so equilibrium is violated.
  - Evidence string: `nonpositive_pivots=1; pivot_condition_ratio_proxy=9.8e15`.
- On the frame kernel directly, the same system in SI is accepted by both solvers (max |x| ≈ 1e9). With stiffness scaled by 1e-3, the dense solver rejects it but the sparse one still accepts it. The result depends on unit scale and on the solver chosen.

**Why it matters to a stress engineer.** An unstable model, the classic case being a line on guides with no axial stop or no torsional restraint, must be refused. This program prints plausible stresses and displacements, because the maximum-displacement summary looks only at translations (`:1129`), and hides the instability.

**Recommended fix**

- Use a relative pivot test: |d_i| ≤ 1e-10 × max|K_ii| of the original row (or the Nastran MAXRATIO equivalent) is singular.
- Any d_i < 0 is blocking, because an elastic K is SPD.
- Check the residual and energy after the solve.
- On failure, report the unrestrained DOF and its node.
- Replace the "six DOFs" heuristic with a proper rigid-body-mode test. For example, project the six rigid-body modes onto the restrained set per connected component and check the rank.

---

### ELEM-5: In the default bend and tee mode the flexibility factor k multiplies the stress (i·k)

**Severity:** High

**Evidence**

- `let multiplier = if modifier.flexibility_in_assembled_stiffness { modifier.sif } else { modifier.sif * modifier.flexibility }` (`lib.rs:7842-7846`). This is used for bends in `mechanics_geometry_only` and for *all* branches (`branch_stress_modifier_for_pipe`, `:7767-7822`, with the flexibility multiplier at `:7804`).
- The multiplier is applied to the whole open-formula summary, including axial and pressure terms (`:7679`), and only at pipe ends that touch the component node (`:7673-7700`). The other end of the bend and the arc interior get no SIF.

**Probe:** the same L-bend with k = 7.36 and i = 2.44 entered in `mechanics_geometry_only` mode gives a bend "user-multiplier" row of **699 MPa**. The correct i·M/Z from the curved element is 47.2 MPa at the tangent. The maximum over the arc, 50.3 MPa, is not reported, because only the end at the component node gets a row.

**Why it matters to a stress engineer.** k is a flexibility (stiffness-reduction) factor and has no place in a stress. Multiplying by it overstates elbow and tee stresses up to about 10 to 20 times, which makes the review rows useless. The SIF must also cover the whole fitting, and it must not multiply the pressure or axial terms that B31.3 treats separately. Axial i_a appears only in B31.3 2016 and later.

**Recommended fix**

- Apply k only in stiffness: the bend element, and branch or nozzle flexibility.
- Apply i_i and i_o to the in-plane and out-of-plane moments at every station of the fitting, and both ends, per B31.3 319.4.4 and Appendix D, or B31J.

---

### ELEM-6: No rigid element: valve and flange weight, centre of gravity and rigidity are ignored

**Severity:** High

**Evidence**

- `rigid_body_length`, `weight`, `center_of_gravity`, `end_a_size`, `end_b_size` and `stiffness_scaling_user_value` are only validated and unit-normalized (`validation.rs:413-480, 568-580`; `lib.rs:4708-4770, 4886-4896`).
- Nothing in `build_model` (`lib.rs:3384-3587`) or the load build uses them.
- Validation even emits `RIGID_COMPONENT_STIFFNESS_SCALING_REVIEWED` (info), which says the component "carries … weight, center of gravity … stiffness scaling" (`validation.rs:1185-1199`). That message implies the inputs are used, and they are not.

**Probe:** a 6.6 m line with a 0.6 m valve span, an 8 kN valve weight, CoG offset 0.4 m, stiffness scale 1000, pipe weight 500 N/m. Anchor reactions are 1650 N and 1650 N with or without the valve, and the valve-node displacement is identical to the last digit.

**Why it matters to a stress engineer.** Valves and flanged pairs are the heaviest concentrated loads on most lines, often with eccentric actuators, and they are also locally rigid. Leaving out their weight makes sustained stresses and hanger loads non-conservative. Leaving out their rigidity misplaces the flexibility.

**Recommended fix**

- Add a rigid element: stiffness about 10³ × the connected pipe's stiffness (or a user scaling), and weight lumped at the CoG through a rigid offset. Alternatively, split the weight to the end nodes with the statically equivalent force and moment.
- Add a flange-pair or valve generator from a user or catalogue weight and length.

---

### ELEM-7: The expansion joint element is not invariant under rigid-body rotation, and there is no tied-joint treatment

**Severity:** High

**Evidence**

- `user_stiffness_local_matrix` places independent relative springs on UX, UY, UZ, RX, RY and RZ between two nodes a finite distance apart (`frame_kernel/src/lib.rs:1018-1040`). There is no coupling term between lateral offset and the chord (no L·k_lat terms).
- A rigid rotation θ of the whole element therefore produces a lateral relative displacement θL, and with it forces.
- There is no input for tie rods, hinge or gimbal types, or tied versus untied behaviour (a grep for `tie_rod`, `untied` or `gimbal` finds nothing). Thrust is always applied as untied (`lib.rs:6730-6740`).

**Probe:** 0.5 m joint element, k_lat = 1e5 N/m, rigid rotation 0.01 rad about z. K·u = (Fy_i, Fy_j) = (−500, +500) N, which leaves an unbalanced moment of 250 N·m about node i.

**Why it matters to a stress engineer.** The element generates forces from pure rigid motion, so any joint on a rotating branch or riser picks up spurious lateral loads. A tied or hinged joint, the common case, cannot be modelled: tie rods carry the thrust and make the joint axially rigid.

**Recommended fix**

- Formulate the joint as a zero-length element at a single location, or as a short beam-like element whose lateral stiffness is transformed with the rigid offset. The standard form is K = Tᵀ k T, with T the rigid-link transformation from the end nodes to the element's centre.
- Add a tied option (axial rigid, thrust not transmitted to anchors) and hinge or gimbal options (angular only, per plane).
- Include the bellows weight.

---

### ELEM-8: Support reactions are reported only as a translational force magnitude, with no components and no moments

**Severity:** High

**Evidence**

- The reaction row is `kind: "reaction_resultant"` with value `vector[0].hypot(vector[1]).hypot(vector[2])` (`lib.rs:1489-1504`).
- Only slots less than 3 are collected (`:1467`, `:1487`). There is no per-DOF reaction row and no moment row anywhere in the output (a grep for `result:reaction` finds only this one).

**Why it matters to a stress engineer.** Anchor and equipment-nozzle qualification (API 610, API 617, NEMA SM23, vendor allowables) needs Fx, Fy, Fz, Mx, My and Mz in a stated axis system. Restraint design needs the signed load in each restrained direction, for example up-lift at a vertical support. This output gives neither. Anchor moments can only be reconstructed by hand from adjacent element end rows, and those are in element-local axes.

**Recommended fix**

- Report the signed global Fx, Fy, Fz, Mx, My and Mz for every support per load case and combination, the CAESAR II restraint summary layout.
- Report per-restrained-DOF values for guides and stops.

---

### ELEM-9: Curved-bend geometry: y_reference sets the bow side, tangency is not checked, the angle is optional, and there is no corner-node input

**Severity:** Medium

**Evidence**

- The arc centre is placed on the −y_reference side of the chord midpoint (`lib.rs:3999-4007`), so the sign of the pipe's y_reference decides which way the arc bows.
- Nothing checks that the arc end tangents match the adjacent pipes (a grep for `tangen` finds no tangency validation).
- `bend_angle` is checked only if it is present (`:3957`).
- The single user k is passed for both in-plane and out-of-plane (`:4009-4022`). k less than 1 is accepted (`positive_finite`, `:3890`).
- There is no correction of k for pressure (B31.3 Appendix D Note 5, or B31J), because k is purely user-entered.

**Probe:** with the bend y_reference flipped, so the arc bows away from the corner and both tangents are kinked, the model still returns `MECHANICS_SOLVED` with no diagnostic and a different anchor load (3586 N against 3417 N). Omitting `bend_angle` also solves silently.

**Why it matters to a stress engineer.** Tangent points have to be computed by hand, which is error-prone. A sign slip produces a physically impossible kinked elbow without any warning.

**Recommended fix**

- Input the bend at the tangent-intersection node with radius and angle, and generate the tangent points.
- Otherwise, block when the arc end tangents differ from the adjacent element axes by more than a small tolerance.
- Require k ≥ 1 or warn below it. Offer separate k_i and k_o, which the crate already supports. Offer the code k with the pressure correction.

---

### ELEM-10: Straight element is Euler-Bernoulli with no shear deformation; short stubby legs are 1.8 to 8 times too stiff laterally

**Severity:** Medium

**Evidence**

- `local_stiffness` uses 12EI/L³, 6EI/L², 4EI/L and 2EI/L (`frame_kernel/src/lib.rs:714-723`), with no Φ term.
- The curved-bend crate also excludes shear (`curved_bend/README.md:40`).

**Calculation:** Φ = 12EI/(G·A_s·L²) with A_s = A/2 for a tube:

| Size | L/D = 1 | L/D = 2 | L/D = 3 | L/D = 5 | L/D = 10 |
|---|---:|---:|---:|---:|---:|
| 8 in Sch 40, lateral stiffness × | 8.2 | 2.8 | 1.8 | 1.3 | 1.07 |
| 24 in Std, lateral stiffness × | 8.6 | 2.9 | 1.8 | 1.3 | 1.08 |

Refining the mesh does not help, because each Euler-Bernoulli element is exact for Euler-Bernoulli theory.

**Why it matters to a stress engineer.** Short legs between elbows, short risers into nozzles, and dummy legs are common and often govern. They come out too stiff, which overstates their expansion loads. The effect is largest on large-bore, compact layouts.

**Recommended fix**

- Use Timoshenko beam terms with Φ_y and Φ_z (shear area A/2 for a thin tube, or the Cowper expression), as CAESAR II and AutoPIPE do.
- Add the shear term to the curved-bend flexibility integral for consistency.

---

### ELEM-11: Section properties: mill tolerance reduces stiffness and weight, there is no corrosion allowance, and one Z serves both sustained and expansion

**Severity:** Medium

**Evidence**

- `derive_pipe_section` computes A, I, J = 2I and Z = I/(D_o/2) from t − mill_tolerance (`lib.rs:6340-6369`). That one section feeds stiffness (`:3455-3463`) and stress (`:7602-7630`).
- The mass calculation also uses the reduced wall (`:5350`).
- `PipeSectionInput` has no corrosion or erosion allowance field (`:180-202`).

**Why it matters to a stress engineer.**

- B31.3 319.3.5 requires *nominal* dimensions in flexibility calculations. Reducing the wall by 12.5% lowers I by about 12%, and thermal reactions by about the same. That is non-conservative for anchors and nozzles.
- Weight is also low.
- B31.3 320 needs Z on the corroded or eroded wall for sustained stress, while 319.4.4 uses nominal Z for the displacement stress range. One Z cannot satisfy both.

**Recommended fix**

- Always build stiffness and weight on nominal dimensions.
- Add a corrosion and erosion allowance, and compute Z_sustained from (t_n − c − mill tolerance where the code requires it) and Z_expansion from t_n, each code-selectable.
- Accept mill tolerance as a percentage as well as a length.

---

### ELEM-12: Tees are rigid node junctions: no branch Ze, no local nozzle or branch flexibility

**Severity:** Medium

**Evidence**

- `build_model` never reads branch geometry (`lib.rs:3384-3587`). A tee is simply three elements sharing a node.
- Stress uses each element's own section (`let section = built.sections.get(&pipe.element_id)` at `:1749`). There is no Ze = πr₂²T_s for the branch (B31.3 319.4.4(c)).
- The branch "flexibility" is used only as a stress multiplier (ELEM-5). There is no WRC 297 or B31J local flexibility for nozzles or branches, and no rigid-tee option.

**Why it matters to a stress engineer.** A rigid junction is the conservative default for loads, and it matches CAESAR II's default. However, branch stresses at unreinforced fabricated tees and stub-ins need Ze. Nozzle loads at vessels are commonly overstated without WRC 297 or B31J nozzle flexibility.

**Recommended fix**

- Use Ze for the branch leg and header Z for the run legs.
- Add optional B31J branch and nozzle flexibility as a local spring element at the junction, and a WRC 297 vessel-nozzle stiffness.

---

### ELEM-13: The global stiffness is assembled dense even on the "sparse" path: memory grows as O(n²)

**Severity:** Medium

**Evidence**

- `assemble_global_stiffness_with_user_elements` allocates `vec![vec![0.0; total_dofs]; total_dofs]` (`frame_kernel/src/lib.rs:765`). It is always called (`lib.rs:938-945`).
- The dense `reduce_system` copy is always built (`:1310`).
- Reactions use a dense K·d (`multiply_matrix_vector(stiffness, &displacements)` at `:1449`).
- A further dense K is built per modulus-basis load case (`:1009-1024`).
- The sparse solve re-assembles from entries (`:2290-2298`), but the dense copies are still held.

**Probe:** a straight-chain model, sparse mode. Peak RSS was 102 MB at 250 nodes, 815 MB at 1000 nodes and **2.8 GB at 2000 nodes**. By extrapolation, about 11 GB at 4000 nodes, and multiplied again for each temperature basis. The dense fallback is also O(n³).

**Why it matters to a stress engineer.** Real plant models run to 2000 to 10 000 nodes. At those sizes the desktop app will run out of memory, even though the sparse solver itself handles them easily.

**Recommended fix**

- Assemble straight into the sparse profile (the `from_entries` path already exists).
- Compute reactions from element contributions at the restrained DOFs.
- Drop the dense matrices except in explicit scrutiny mode.

---

### ELEM-14: No reducer element; the end sizes are ignored

**Severity:** Low

**Evidence.** `end_a_size` and `end_b_size` are only validated and normalized (`validation.rs`; `lib.rs:4708+`). There is no tapered or stepped-section element.

**Why it matters to a stress engineer.** A size change is possible only as a sharp step at a node. Concentric reducers have a small effect on flexibility, but they need their own SIF (B31.3 Appendix D) and weight.

**Recommended fix.** Add a reducer element: two or more segments with interpolated D and t (the CAESAR II approach), with its SIF and weight.

---

### ELEM-15: Recovery conventions: end rows and station rows use opposite signs, bend end rows are in the chord frame, and bending stresses are summed arithmetically

**Severity:** Low

**Evidence**

- Endpoint rows are node-on-element actions, while station rows are j-side section cuts (comment at `lib.rs:1608-1609`; constant at `:7069`).
- In the cantilever probe, the same continuous moment reads −6000 N·m at end i and +4500 N·m at the quarter point.
- Curved-bend end rows are rotated into the *chord* frame (`:7054-7066`), so the "axial" row at a 90° bend end mixes axial force and shear at 45°. The bend station rows are in the arc-tangent frame (`:7070`).
- `open_formula_summary_mpa` adds |σ_by| + |σ_bz| (`:7646-7649`). The resultant for a round pipe is √(My² + Mz²)/Z, so this overstates by up to √2.

**Why it matters to a stress engineer.** The same moment appears with different signs in the same table, and "axial" at a bend end is not the pipe axial force. Both invite misreading. Summing the bending stresses arithmetically is conservative but departs from the B31 resultant moment.

**Recommended fix**

- Publish element forces in one convention (local, at each node, force on the element), plus the global end forces.
- Report bend ends in the tangent frame.
- Use the resultant moment for bending stress.

---

### ELEM-16: y_reference is mandatory on every pipe, although it has no effect on a circular section

**Severity:** Low

**Evidence**

- A missing y_reference is blocking (`lib.rs:3445-3453`).
- The desktop drafts default it to blank (`apps/desktop/src/features/viewport/PipeViewport.tsx:3109-3111`).
- A y_reference parallel to the pipe (for example (0,0,1) on a riser) is blocked by an absolute 1e-12 norm test (`frame_kernel/src/lib.rs:1143-1148`).
- **Probe:** a skewed cantilever gives identical displacements, matching the analytical result to seven digits, for three different y_reference vectors. The orientation matters only for naming local axes, and for the bend bow side (ELEM-9).

**Why it matters to a stress engineer.** It is pure input burden, and a common cause of blocked vertical runs.

**Recommended fix.** Default the local y from a global convention: global vertical projected normal to the element, falling back to global X for vertical elements, as in CAESAR II. Keep the override for non-circular sections only.

---

## Suspected, not verified

- The sparse singular-pivot failure falls back to the dense LU with *partial pivoting* (`lib.rs:2300-2316` and `:2347-2362`), with only a warning. Because the two paths eliminate in different orders with the same absolute guard, a system the sparse solver correctly rejects may be accepted by the dense fallback. The reverse was observed in the frame-kernel probe; a sparse-rejects, dense-accepts case was not constructed.
- Out-of-plane loading of the curved bend was checked only indirectly, through the crate's own tests and the in-plane polygon comparison. An out-of-plane polygon comparison was not run.

## Done well

- **Curved-bend macro element.** Exact closed-form Castigliano flexibility over the arc, with correct in-plane and out-of-plane moment and torsion decomposition. The 12×12 is built by equilibrium transfer. It converges to a 64-segment polygon to about 1e-5 with k = 1, both for thermal load and for arc-length weight. Thermal load uses the exact free-expansion identity, and weight uses arc-consistent fixed-end integration. This is a sound element; it only needs to be made the default (ELEM-1).
- **Coordinate transformation.** The transform is correct and invariant to y_reference: the skewed-cantilever probe matched the analytical result exactly.
- **Straight element.** Standard, correct Euler-Bernoulli 12×12. The cantilever tip deflection matched PL³/3EI.
- **Section formulas.** A, I, J = 2I, Z = I/(D_o/2) and the hoop mean-radius formula are correct for the wall thickness they are given.
- **Boundary conditions.** Rigid restraints are applied by exact DOF elimination (no penalty), which is good for conditioning.
- **Solver pipeline.** The RCM plus skyline LDLᵀ is clean and deterministic, and it records the profile, bandwidth and a pivot-ratio proxy. The instrumentation is there; it only needs to be turned into checks (ELEM-4).
- **Input policy.** Most missing inputs block with explicit diagnostics instead of being silently defaulted. The exception is the expansion joint (ELEM-2).


---

## Appendix — B. Loads and load generation (prefix LOAD-)

Scope: the live solve path in `core/product_physics/src/lib.rs` (abbreviated `pp`), `core/product_physics/src/self_weight.rs`, `core/loads/primitive_loads/src/lib.rs` (`prim`), and the related pieces of `core/loads/stress_recovery`, `core/solver/linear_supports`, the desktop load-case UI and the model-operation applier.

All paths are relative to `/Users/ryan/dev/chirality-review/projects/chirality-piping/`.

## Probe

I wrote a probe program that builds JSON models and runs them through `run_linear_static_preview`, the same entry point the desktop's Tauri solve command uses. It is at `/private/tmp/claude-501/-Users-ryan/08937a6e-e544-4c91-b97d-34e629979c7f/scratchpad/review/probe_loads/`.

Test pipe: 6 in. Sch 40 (OD 168.3 mm, t 7.11 mm), E = 200 GPa, α = 1.2e-5 /°C. Hand values for P = 1 MPa:

| Quantity | Value |
|---|---|
| Longitudinal stress, PD/4t | 5.918 MPa |
| Longitudinal stress, P·d²/(D²−d²) | 5.179 MPa |
| Longitudinal stress, P(D−t)/4t | 5.668 MPa |
| Hoop stress, PD/2t | 11.835 MPa |
| Hoop stress, P(D−t)/2t | 11.335 MPa |

Probe results are quoted in the findings below.

---

## Findings

### LOAD-01: Longitudinal pressure stress is dropped whenever a pressure load is present; free-to-grow lines report about zero sustained longitudinal stress

**Severity:** Critical

**Evidence**
- `pp:1754-1756`: when any genuine pressure load exists on a pipe, the longitudinal pressure term is excluded from every stress row and from the summary:
  ```
  let pressure_thrust_active = pressure_thrust_for_pipe(...) != 0.0;
  let include_pressure_longitudinal = !pressure_thrust_active;
  ```
  A genuine pressure load always produces a non-zero thrust (`pp:6744-6749`), so the term is effectively never included.
- `pp:6826-6853`: pressure enters the frame as a self-equilibrated pair, −pA_i at node i and +pA_i at node j, along the chord. This is equivalent to an initial strain pA_i/(EA).
- `pp:6959-6978` (`corrected_local_forces_for_axial_effects`): the recovered axial force then has pA_i subtracted, which gives the "effective" force N − pA_i.
- Net effect: reported σ_L = (N_FE − pA_i)/A, with no PD/4t term.
- The project's own fixture asserts this as intended behaviour: `pp` test `endpoint_section_cut_fixed_and_free_pressure_thermal_match_uniform_stations` says "fixed pressure state is compressive". So does `validation/hand_calcs/mechanics/tp_phys_008_thermal_pressure_axial_effects.md`, which treats pressure thrust as analogous to thermal restraint.

**Probe results (P = 1 MPa)**
- Cantilever, 10 m, anchored at one end: `axial-normal` = 0.000 MPa. No `pressure-longitudinal` row is emitted. `open_formula_stress_summary` = 0.000 MPa. Code value is 5.2 to 5.9 MPa.
- Same pipe anchored at both ends: `axial-normal` = −5.179 MPa and summary 5.179 MPa. The magnitude only resembles the code value by accident, and the sign is compressive.
- L-bend, 6 m + 4 m, anchored at both ends: P1 summary 0.578 MPa and P2 summary 0.778 MPa. These are bending from pressure elongation only; the ≈5.9 MPa pressure term is missing entirely.

**Why it matters to a stress engineer**
- The sustained longitudinal stress (B31.3 para. 320 / B31.1 para. 104.8) always includes the longitudinal pressure stress, whatever the restraint.
- In a normal flexible system, where most of the pipe is free to grow axially, the program reports sustained stress with the pressure term missing. For high-pressure or thin-wall lines this is usually the dominant part of S_L.
- The result is unconservative and gives no warning.

**Recommended fix**
- Always add the code longitudinal pressure stress to S_L, with the formula selectable by the rule pack: PD_o/4t_n, or P·d²/(D²−d²), using the code's wall basis.
- Separately, report the mechanical axial force F_ax excluding the closed-end thrust. The existing effective-force subtraction is correct for this purpose.
- Make pressure elongation an option (see LOAD-10). With it off, the thrust pair is not applied at all, as in CAESAR II / AutoPIPE defaults.
- Replace the "fixed pressure state is compressive" assertion with a benchmark in which both the free and the anchored pipe give S_L = PD/4t.

---

### LOAD-02: Constant-effort support force is added to every load case, so it multiplies under linear combination and acts alone in thermal-only and occasional-only cases

**Severity:** Critical

**Evidence**
- `pp:1301` calls `add_constant_effort_support_loads(&mut force, model)` inside `solve_load_case`, for every case, with no condition on the case's content.
- `pp:8453-8460` adds `hanger.constant_load` to the force vector.
- `pp:8306` documents the behaviour: "applied ... in every solved load case".

**Probe results** (anchor, 10 m pipe to a node with a 1000 N constant-effort support, 5 m leg to a second anchor; W = 200 N/m; T = +100 °C)

| Case | Constant-effort applied load | uy at the constant-effort node |
|---|---|---|
| W alone | 1000 N | +2.15 mm |
| T alone | 1000 N | **+11.91 mm** (constant-effort force with no weight to balance) |
| W+T solved directly | 1000 N | +2.15 mm |
| Mechanics combination W(1.0) + T(1.0) | **2000 N** | **+14.06 mm** |

The combination is wrong by a factor of 6.5 in displacement at the hanger.

**Why it matters to a stress engineer**
- Engineers routinely build operating cases by superposing separately solved W, P, T and occasional cases, and run pure-thermal or pure-wind cases for ranges and occasional stresses.
- Every such case carries a phantom upward force at each constant hanger, and a linear combination counts it once per term. The results are:
  - wrong displacements, reactions and stresses near every constant-effort support;
  - EXP ranges polluted if the range is built from a T-only case;
  - occasional cases (wind, seismic) that include a gravity-type preload they should not have.

**Recommended fix**
- Treat the constant-effort load as a weight-type load, not a support property:
  - apply it only in cases flagged as containing weight (sustained, operating, hydrotest);
  - or, equivalently, as a load with a "gravity" category that the user adds to W;
  - and exclude it from thermal, wind and seismic primitives.
- In mechanics combinations, never re-add support preloads per term.
- The standard approach in commercial tools is to treat the hanger load as part of the weight load set, so that W+T superposition stays exact.

---

### LOAD-03: Imposed displacements (nozzle thermal movements, settlement, anchor movement) are unsupported, and the UI record makes the whole model fail to parse

**Severity:** Critical

**Evidence**
- The UI offers it: `apps/desktop/src/features/load-cases/LoadCaseManagerPanel.tsx:561` has `<option value="imposed_displacement">`.
  - The payload target is `{ type: "support", support, dof }` (`:1611-1616`, `:2517-2522`).
  - The model-operation applier accepts it and writes it into the model (`core/model_operations/operation_applier/src/lib.rs:5063-5066`, `:5143`).
- The solver's model type cannot deserialize it. `pp:549-554` has `enum LoadTargetInput { Node, Element }`, a tagged enum with no `support` variant.
  - Probe: `MODEL DESERIALIZE ERROR: unknown variant 'support', expected 'node' or 'element'`.
  - The Tauri command (`apps/desktop/src-tauri/src/lib.rs:1528-1530`) returns this as an error string. **One imposed-displacement record makes the entire model unsolvable.**
- Even with a node target, `parse_category` has no `imposed_displacement` arm (`pp:9599-9611`). Probe with a node target gives the blocking diagnostic `LOAD_INPUT_INVALID`, confirming the reported `parse_category` rejection.
- The plumbing exists but is not wired:
  - `prim` has `PrimitiveLoad::imposed_displacement` and `prepare_support_load` (`prim:948-962`, `:2494-2535`), and `LoadApplication.imposed_displacements` is produced but never read by `pp`.
  - `core/solver/linear_supports` has `SupportFamily::ImposedDisplacement` (`:24`, `:207`) and a prescribed-displacement reduction.
  - `core/solver/frame_kernel` has `reduce_system_with_prescribed_displacements` (`:814`).
  - The live path always builds supports with `imposed_displacement: None` (`pp:3639`) and calls plain `reduce_system` (`pp:1310`).
- The support family whitelist (`pp:3594-3604`) does not include an imposed-displacement family.

**Why it matters to a stress engineer**
- Almost every hot system connects to equipment whose nozzles move thermally: pumps, vessels, exchangers, turbine casings.
- Without imposed displacements the program cannot:
  - check nozzle loads with correct equipment growth;
  - model tank settlement or building and platform movement;
  - model anchor movement from other lines.
- The workarounds (fake thermal legs, or a rigid stub with ΔT) are error-prone and not auditable.

**Recommended fix**
- Add `support` as a `LoadTargetInput` variant, or better, make the displacement a per-load-case attribute of the restraint: D1/D2/D3 per restrained DOF, as CAESAR II and AutoPIPE do.
- Map the displacement to prescribed DOFs through the existing `reduce_system_with_prescribed_displacements`, and recover reactions as K·u − F at those DOFs.
- Displacements must belong to a load case, so they appear in OPE and cancel correctly in OPE − SUS, and they must combine with thermal cases.
- A displacement on an unrestrained DOF should be applied through a stiff spring (about 10^n × local pipe stiffness) or rejected with a clear message.
- Until this is implemented, remove the UI option. It currently destroys the model's solvability.

---

### LOAD-04: Component weights (valves, flanges, rigid elements) are required inputs but are never applied, and weight cannot be entered as a nodal load

**Severity:** High

**Evidence**
- Rigid component geometry requires `weight` and `center_of_gravity` (`pp/validation.rs:1582-1583`), and they are unit-normalized (`pp:4747-4770`).
- No code in the solve path reads `geometry.weight`. A search of `lib.rs` finds only validation and normalization, and there is no load generation from components.
- `prim:2537-2544`: `category_allows_node_target` allows only Wind, Seismic and Occasional. A nodal `weight` load is rejected.
  - Probe: blocking `"Weight is not a nodal primitive load target"`.
- The only way to apply a valve weight is a `concentrated_force`. It is re-labelled "occasional" with a warning on every solve (`pp:9609`, `:9614-9619`).
- Seismic mass (`pp:5341-5474`, `compute_pipe_mass_per_length`) covers only pipe metal, contents and insulation. Component masses and user concentrated weights receive no seismic inertia.
- The self-weight generator (`self_weight.rs:354-355`) emits only per-pipe distributed loads.

**Why it matters to a stress engineer**
- Valves and flanged fittings are the largest concentrated masses in most systems. A 6 in. class 600 gate valve is roughly 3 to 5 kN.
- Omitting them underestimates:
  - sustained moments at adjacent supports and nozzles;
  - spring hanger loads;
  - seismic inertia, where the valve's eccentric CG drives torsion.
- The user is asked to enter weight and CG, which suggests they are used.

**Recommended fix**
- Model rigid components as rigid elements with a stiffness of about 10^n × the adjacent pipe stiffness, with the entered weight lumped at the entered CG. The CG offset produces the correct moment through a rigid link.
- Add the lumped weight to the weight case and its mass to seismic generation. Include contents and insulation over the rigid length, as the standard tools do.
- Allow `weight` (and `hydrotest`) as nodal categories, so concentrated weights are sustained loads and not labelled "occasional".

---

### LOAD-05: Spring hangers carry no installed or cold preload, there is no hanger design, and hangers cannot be locked for hydrotest

**Severity:** High

**Evidence**
- A variable spring is a plain linear spring. `pp:3550-3567` gives `LinearSupport::spring(...)` with stiffness only.
- `installed_load`, `cold_load` and `hot_load` are only echoed as review rows (`pp:8119-8130`), labelled `load_side_review_reference`. They never enter the force vector.
- No hanger-sizing code exists. A search for hanger design, sizing and load variation returns nothing.
- Support definitions are global to the model, with no per-load-case support state, so hangers cannot be pinned or locked for a hydrotest case.

**Probe result**
- Cantilever, 10 m, 400 N/m, with a 50 kN/m spring at the tip entered with installed load 2000 N.
- Weight case: tip sag −26.3 mm. The spring carries 1315 N purely from k·δ, and the anchor carries 2685 N.
- With the 2000 N preload applied as an upward force at the hanger, the sag would be close to zero and the anchor load about 2000 N.

**Why it matters to a stress engineer**
- Without the preload, every weight and operating case at a spring gives the wrong displacement and reaction, and the sustained stress is distributed wrongly.
- The standard hanger-design process cannot be carried out:
  1. Solve weight with the hanger as a rigid +Y restraint to get the hot load.
  2. Solve operating with the hanger removed to get the travel.
  3. Select the spring rate from the catalogue so that the load variation |k·Δ|/H stays within the chosen limit (commonly 25 %).
  4. Set the cold load to H + k·Δ.
  5. Re-run with the spring and its cold load.
- Hydrotest normally requires hangers to be locked.

**Recommended fix**
- Apply the user cold (installed) load as a constant upward force in weight-bearing cases, not in pure thermal or occasional cases (see LOAD-02). Keep the spring stiffness in all cases, so that thermal movement changes the load by k·Δ. This is the standard linear hanger model.
- Add a hanger design mode running the rigid-weight, free-operating and sizing steps above. Take spring rates from user-supplied (not bundled) catalogues, and apply the load-variation limit.
- Add per-load-case support status (active, locked, removed) for hydrotest and hanger design.

---

### LOAD-06: Hydrotest has no working path; "hydrotest" pressure is silently ignored

**Severity:** High

**Evidence**
- `parse_category` accepts `hydrotest` (`pp:9605`).
- The pressure paths only accept `category == "pressure"`, both for thrust and for stress (`pp:6754-6762`, `genuine_pressure_element_target`, used by `pp:6724` and `pp:9429`).
- A hydrotest load with the pressure dimension is also skipped by `add_uniform_element_loads` (`pp:6556-6561`), so it does nothing. No diagnostic is emitted.
- The project test `endpoint_section_cut_hydrotest_pressure_dimension_has_no_phantom_pressure_effect` (`pp:15826`) asserts this.
- Probe: a hydrotest load of 1.5 MPa solves with `MECHANICS_SOLVED`, displacement 0, stress 0, and no warning.
- A hydrotest load with `force_per_length` behaves exactly like weight. Probe: tip uy = −266.7 mm for 500 N/m.
- The UI does not offer `hydrotest` at all (`LoadCaseManagerPanel.tsx:38-45`).
- Water-filled weight cannot be generated. The self-weight generator uses the section's single `contents_density` (`pp:5414-5434`), so the user must edit the section, generate the loads, then edit it back.

**Why it matters to a stress engineer**
- Hydrotest is a required design case: the hydrotest stress check (B31.3 para. 345 with para. 302.3.6) and water-filled support loads.
- Entering a hydrotest pressure and getting a clean "solved" result with no pressure effect is materially misleading.

**Recommended fix**
- Treat hydrotest pressure as a pressure load: thrust as in LOAD-10 and stresses as in LOAD-01.
- Or reject a hydrotest load with the pressure dimension with a blocking diagnostic.
- Add a hydrotest weight generator that fills with a user-specified test fluid density (water by user input). Combine it with per-case support status so that springs are locked (LOAD-05).

---

### LOAD-07: Thermal α and E come from one basis per load case, so expansion stress uses the hot modulus and the α basis is ambiguous

**Severity:** High

**Evidence**
- `pp:6664-6713` (`build_thermal_element_loads`) takes strain α·ΔT and axial load E·A·α·ΔT, where α and E are those of the load case's effective material.
- The effective material is chosen once per case, for all materials, by `modulus_basis_ref` (an exact point) or `modulus_basis_temperature` (linear interpolation of E, G and α between points) (`pp:5882-6216`).
- The stiffness matrix for the case is rebuilt with the same basis (`pp:992-1034`).
- Under interpolation, α is interpolated at the same solve temperature as E (`pp:6097-6102`). No installation temperature or reference temperature is recorded anywhere.
- The primitive magnitude is only a ΔT (`pp:9628`). There is no T_install or T_operating per pipe.
- `append_combination_modulus_basis_records` (`pp:6252-6320`) only records the basis of each operand when `result_state_subtraction` mixes bases, for example OPE with hot E minus SUS with cold E. It does not warn.

**Why it matters to a stress engineer**
- B31.3 para. 319.3.2 / 319.4.4 compute the displacement stress range with the reference (cold) modulus Ea. B31.1 uses the cold modulus too.
- Hot reactions use E_hot (B31.3 para. 319.5), or the cold reaction scaled by E_hot/E_cold.
- If the user solves OPE at the hot basis, as they must to get a sensible interpolated α, then S_E and reactions both use E_hot. S_E is then unconservative by E_cold/E_hot, typically 10 to 15 % for carbon steel at 300 to 400 °C.
- If they solve at the cold basis instead, α is the cold value and the growth is underestimated.
- Linear interpolation of α is only meaningful for mean coefficients from a common reference temperature. The code cannot tell mean from instantaneous coefficients, and ignores T_install ≠ 21 °C.
- OPE − SUS with different bases leaves an E-mismatch residual of the weight solution in the "expansion" result.

**Recommended fix**
- Separate the expansion strain from the modulus. Store per material the total unit expansion e(T) (mm/m from the reference temperature, as the codes tabulate), and compute ε = e(T_op) − e(T_install) per pipe.
- Solve expansion stress ranges with Ea (the cold or reference modulus). Report hot reactions from a separate solve with E_hot, or with the para. 319.5 scaling.
- Warn when a subtraction or range mixes modulus bases, or subtract with a common basis.

---

### LOAD-08: Pipe-by-pipe ΔT entry with one material basis per case; no per-pipe T1/T2/T3 or P1/P2/P3 tables

**Severity:** Medium

**Evidence**
- Thermal and pressure loads are primitives, one element target each (`prim:2546-2557`). There is no generator for them; only self-weight has a generator (`self_weight.rs`).
- A 300-element model needs 300 thermal primitives per thermal case and 300 pressure primitives per pressure case.
- The per-case modulus basis applies the same temperature point or temperature to every material (`pp:5897-5901`). A line with a 350 °C header and a 120 °C branch in one operating case cannot use α and E at each pipe's own temperature. The only workaround is duplicate materials whose points share an id.
- Multiple thermal states (T1, T2, T3) are possible only as separate load cases, each authored in full.

**Why it matters to a stress engineer**
- This is a significant authoring burden and error source; a missed pipe silently gets ΔT = 0.
- Mixed-temperature systems are common: bypasses, branches, tracing, and startup versus normal operation.

**Recommended fix**
- Add per-pipe (or per-run) operating conditions: T1..Tn, P1..Pn and Phyd, inherited along the run as in CAESAR II and AutoPIPE.
- Generate the thermal and pressure primitives per case from those conditions, with each pipe's α and E at its own temperature.
- Flag pipes that have no temperature in a thermal case.

---

### LOAD-09: Hoop and longitudinal pressure formulas use the mean radius and the mill-tolerance wall, with no corrosion allowance

**Severity:** Medium

**Evidence**
- `pp:6369` sets `membrane_radius: (od - thickness) / 2.0`.
- `core/loads/stress_recovery/src/lib.rs:951-954`: `hoop = p*r/t`, longitudinal `hoop / 2`, which gives P(D−t)/2t and P(D−t)/4t.
- `t` is nominal minus mill tolerance (`pp:6340-6356`). There is no corrosion or erosion allowance input.
- Probe: hoop = 11.335 MPa, against Barlow PD/2t = 11.835 MPa.
- The longitudinal row, when shown, is P(D−t)/4t = 5.668 MPa, against PD/4t = 5.918 MPa. As LOAD-01 shows, it is effectively never shown.
- There is one pressure magnitude per load case. Design and operating pressure differ only by the user building separate cases.

**Why it matters to a stress engineer**
- B31.1 para. 104.8 uses PD_o/4t_n. B31.3 para. 320 uses the stated longitudinal pressure stress on the code wall basis, and the code sustained check uses the corroded wall.
- The mean-radius form is 4 to 5 % lower for D/t ≈ 24. It is lower still for thin-wall pipe once mill tolerance and corrosion are removed from t but not from D.

**Recommended fix**
- Let the rule pack choose the pressure-stress formula and the wall basis: nominal, less mill tolerance, less corrosion allowance.
- Add a corrosion allowance input.
- Keep the Lamé or mean-radius values only as labelled mechanics rows.

---

### LOAD-10: Pressure elongation is always on and ignores the Poisson effect; no Bourdon effect and no pressure stiffening of bends

**Severity:** Medium

**Evidence**
- The thrust pair of `pp:6826-6853` imposes the axial strain p·A_i/(E·A_m) with no hoop-Poisson contraction, and there is no switch to turn it off.
- On macro bends, `pp:6868-6892` applies end caps plus a consistent radial wall load. This is a self-equilibrated membrane system that only elongates the arc (the comment at `pp:6856-6867` says so). It does not open the bend (Bourdon).
- The bend flexibility factor is purely user-entered (`pp:311`, `flexibility_factor_user_value`). No pressure correction is applied.
- A search for "Bourdon" in `core` finds nothing.

**Probe result**
- 10 m of 6 in. Sch 40 at 1 MPa, free end: ux = 0.2589 mm.
- The Poisson-corrected elongation, ε = (σ_L − ν·σ_H)/E = (1 − 2ν)·PD/(4tE), is 0.118 mm. The program overstates it by a factor of 2.2.
- In the anchored L, this spurious elongation produced 0.4 to 0.8 MPa of bending (see LOAD-01).

**Why it matters to a stress engineer**
- On long, high-pressure lines, overstated pressure growth distorts displacements, reactions and range calculations.
- The Bourdon effect matters for large-D/t bends at high pressure (for example, main steam), and pressure stiffening reduces bend flexibility per the code's pressure correction to k and i.
- None of these is available, and the elongation that is modelled cannot be switched off.

**Recommended fix**
- Make pressure elongation an option, default off to match common practice. When on, apply the Poisson-corrected strain (1 − 2ν)·σ_L/E as an initial strain, or the full Bourdon formulation for straight pipe and bends.
- Add an optional pressure correction of bend k and i, following the code's pressure-correction note.
- Keep the self-equilibrated arc load vector. It is correct.

---

### LOAD-11: Wind load is not projected onto the pipe, has no height profile, and uses one direction per case

**Severity:** Medium

**Evidence**
- `prim:2045`: `magnitude = pressure * shape_factor * exposed_diameter`, applied as a global uniform intensity along the chosen axis over the full pipe length (`pp:6376-6392`), whatever the pipe's orientation.
- The exposed diameter is OD + 2·insulation (`pp:5696-5704`).
- The inputs are a single uniform pressure and a single axis (`pp:471-491`).

**Probe results** (p = 1 kPa, Cs = 0.7)
- Wind along the axis of an X-pipe: reaction 1178.1 N, applied axially. It should be about 0.
- Pipe inclined 30° to the wind: 1178.1 N. The normal-drag value is 589.0 N.

**Why it matters to a stress engineer**
- Axial "wind" on runs parallel to the wind, and full load on inclined runs, is wrong. The user must hand-mark only perpendicular spans, and cannot model inclined runs correctly at all.
- Wind loads vary with elevation (ASCE 7 exposure profiles and similar). Here a pressure-versus-height profile needs one case per band plus a linear combination.

**Recommended fix**
- Apply w = q(z)·Cs·D_exposed·|sin θ| normal to the pipe, in the plane of the pipe axis and the wind vector (the standard projected-area approach). Here θ is the angle between the pipe and the wind.
- Accept a user q(z) table, or a velocity profile with user coefficients.
- Accept an arbitrary wind direction vector.
- Optionally auto-mark all pipes as exposed, with user exclusions.

---

### LOAD-12: Seismic static load acts in one sense per case with all axes simultaneous, and omits concentrated masses

**Severity:** Medium

**Evidence**
- `pp:5509-5582` and `prim:1935` apply every entered g-factor (X, Y, Z) together in the same case, with the entered sign. Probe: 0.3 g X and 0.3 g Z applied together.
- Negative factors are accepted (probe 8b solves), so ±X, ±Z and similar need one case each, entered by hand.
- There is no directional combination (SRSS or 100-40-40) and no automatic ± envelope.
- The mass is only pipe metal, contents and insulation (see LOAD-04).
- Gravity must be entered again for seismic, separately from the self-weight gravity.

**Why it matters to a stress engineer**
- Standard practice runs +X, −X, +Z, −Z (and ±Y where required) and takes the absolute or SRSS envelope.
- Applying X and Z together in one case is neither the code method nor conservative for all members.
- Missing valve masses under-predict support and nozzle seismic loads.

**Recommended fix**
- Generate one case per axis and sense automatically from the factors, and provide an envelope and SRSS combination.
- Include lumped component masses (LOAD-04).
- Share one gravity definition across the model.

---

### LOAD-13: Cold spring and cut-short are not supported

**Severity:** Medium

**Evidence**
- A search for cold spring, cut-short or cold-pull elements in `core` returns nothing.
- The only mechanisms are thermal ΔT on an element and user forces, and imposed displacements do not work (LOAD-03).

**Why it matters to a stress engineer**
- Cold spring is used to reduce hot nozzle loads on turbines and compressors. It must be modelled for reaction evaluation (B31.3 para. 319.5.1). The codes give no credit for it in the stress range.
- The ΔT-on-a-short-element workaround is possible, but it is hidden and easily mis-signed.

**Recommended fix**
- Add a cold-spring element: a user gap length on a named element, applied as an initial strain −gap/L in the cases the user selects (installed and operating). Exclude it from the range, and report hot and cold reactions per para. 319.5.1.

---

### LOAD-14: Load-case `kind` is ignored and nothing helps the user build the standard SUS, OPE, EXP and OCC set

**Severity:** Medium

**Evidence**
- `PreviewLoadCase` (`pp:418-444`) has no `kind` field, so serde discards the UI's free-text `kind` (`LoadCaseManagerPanel.tsx:495-498`, default `primitive_user_load` at `:1285`). The self-weight generator writes the same value (`self_weight.rs:370`).
- Load categories only gate targets. They are otherwise metadata (`pp:9599-9620`), and the "distributed_force → weight" and "concentrated_force → occasional" mappings produce warnings on every solve.

**How a user must build the standard set today**
1. **W**: case from the self-weight generator.
2. **SUS**: one case holding the W primitives and the pressure primitives on every pipe.
3. **OPE**: one case holding W, pressure and thermal ΔT on every pipe, solved directly.
   - A direct solve is required with nonlinear supports and friction, because the friction normal force comes from the same case's reaction.
   - It is also required because of LOAD-02.
4. **EXP**: `result_state_subtraction` OPE − SUS.
5. **OCC**: separate wind and seismic cases per direction (LOAD-11, LOAD-12), added to SUS by a `mechanics` combination.
   - Occasional cases must not be superposed onto cases with constant-effort supports (LOAD-02).
   - Superposition is invalid with one-way or friction supports.

**Why it matters to a stress engineer**
- Every step is manual. The traps are silent:
  - the constant-effort force is added in every case;
  - modulus bases can be mixed across the subtraction;
  - the pressure term is dropped;
  - superposition is used with nonlinear supports.
- The case's engineering type is not carried to the rule layer, so code-check classification must be re-entered elsewhere.

**Recommended fix**
- Make case type a validated enum: W, P, T, D, HYD, WIN, SEI, HGR, and so on, or SUS / OPE / EXP / OCC / HYD for combinations.
- Provide recommended-case generation in the CAESAR II style, using direct nonlinear solves for OPE and occasional cases where supports are nonlinear.
- Block linear combination of cases that contain nonlinear supports, or that each carry support preloads.

---

### LOAD-15: Concentrated moments created in the UI are rejected by the solver

**Severity:** Medium

**Evidence**
- The UI directions for `concentrated_moment` are `rotation_x|y|z` (`LoadCaseManagerPanel.tsx:2523`). The operation applier requires exactly these (`operation_applier/src/lib.rs:5048-5049`).
- The solver's `parse_direction` accepts only `global_*` or `RX..RZ` (`pp:9590-9597`, `:9540-9550`).
- Probe: `rotation_z` gives blocking `LOAD_INPUT_INVALID` and the whole model stops. `RZ` solves.

**Why it matters to a stress engineer**
- Every moment load authored through the product UI blocks the solve.

**Recommended fix**
- Normalize `rotation_*` to `RX..RZ` in `parse_direction`, or make the UI and applier emit `RX..RZ`.
- Add a UI-to-solver round-trip test for every category the UI offers.

---

### LOAD-16: Self-weight generator is a stale snapshot; no refractory or lining input; the bend chord model concentrates weight at the corner

**Severity:** Low

**Evidence**
- `self_weight.rs:333-364` writes fixed `N/m` values into a new case. There is no link back to the section, so later changes to OD, wall, density, insulation or contents are not reflected, and there is no staleness diagnostic.
- Only one insulation layer is supported. There is no refractory or lining layer: the mass is metal + contents + insulation only (`pp:5412-5456`).
- Gravity has no default and must be entered for the self-weight request and again for seismic (`pp:461`).
- On bends not realized as macro elements, the model is two straight legs to the tangent intersection. For 90°, the leg length is 2R against an arc of πR/2, so the weight in the bend region is overstated by 27 % and lumped at the corner. Macro bends correctly integrate over arc length (`core/solver/curved_bend/src/lib.rs:260-275`).

**Recommended fix**
- Generate weight at solve time from the current section, with the case storing only the gravity vector and the contents state (empty, operating, hydrotest).
- Add refractory and lining layers, as thickness and density inside the pipe, reducing the fluid area.
- Use one model gravity vector.

---

## Cross-area note (verified, outside the loads remit)

`pp:1058-1063` builds `summary.max_displacement` and `summary.max_open_formula_stress` from the **first** load case only. A user who reads the summary sees nothing from OPE or occasional cases unless they are listed first.

## Friction interaction (for the friction reviewer)

- The friction normal force is taken from a named support's reaction in the same case.
- A T-only case therefore has zero friction, and a W + T combination of separately solved cases also has no friction.
- The OPE case must be solved directly with W + T. See LOAD-14.

---

## Done well

- **Consistent (not lumped) distributed loading on straight pipe.** It uses fixed-end forces and moments with section recovery that subtracts the equivalent loads (`pp:6546-6661`, `pp:1584-1605`). Partial-span loads are handled correctly with the lever rule and the station extrema search (`pp:6461-6544`).
- **Curved-bend macro elements** receive arc-length-consistent uniform loads, including weight per unit arc length (`curved_bend:260-275`), and exact free-expansion thermal loads K·u_free (`pp:6928-6957`).
- **The pressure thrust system is self-equilibrated** on straight elements and arcs (cap forces at the true end tangents plus the consistent radial wall load). Probe 12 showed that omitting pressure on one leg creates no spurious external force. Reducer and tee junction forces come out naturally.
- **Thermal restraint is exact for a straight pipe.** Anchored at both ends, ΔT = 200 °C gives −480.0 MPa, which is exactly E·α·ΔT.
- **The mass per length is correct**: metal (over the mill-tolerance wall), contents over the inner area, and insulation annulus. Missing and partial inputs are blocked rather than defaulted.
- **Wind partial-span marking and overlap checks** are careful. Seismic and wind generation block on missing inputs rather than inventing values.
- **The per-case modulus basis** rebuilds the stiffness consistently with the thermal E, so the E·α·ΔT in the load vector matches K. The problem is the policy (LOAD-07), not the arithmetic.
- **The kernel already has the pieces for imposed displacements** (`reduce_system_with_prescribed_displacements`, `SupportFamily::ImposedDisplacement`). Wiring them into the live path is mostly integration work.


---

## Appendix — C. Supports, restraints, hangers and nonlinear support behaviour (SUP-)

Scope: the live solve path in `core/product_physics/src/lib.rs` (abbreviated `pp`), `core/solver/linear_supports/src/lib.rs` (`ls`), `core/solver/nonlinear_supports/src/lib.rs` (`ns`), `core/solver/nonlinear_integration/src/lib.rs` (`ni`), and the desktop authoring in `apps/desktop/src/features/support-configuration/SupportConfigurationForm.tsx` (`form`) and `apps/desktop/src/features/hanger-selection/*`.

Every probe below went through the public entry point `run_linear_static_preview`. The probe crate and model generators are in
`/private/tmp/claude-501/-Users-ryan/08937a6e-e544-4c91-b97d-34e629979c7f/scratchpad/review/sup_probe/` (files `p1.py` to `p9.py`). The probe model is a 6" pipe (OD 168.3 × 7.11 mm, E = 200 GPa, α = 1.2e-5 /°C) with Z up. `sup_probe_cap/` is the same crate built against a scratch copy of `product_physics` whose only change is the iteration cap raised from 4 to 100 at `pp:67`.

## Summary table

| ID | Title | Severity |
|---|---|---|
| SUP-01 | Active-set iteration cap of 4 blocks the whole analysis on ordinary models | Critical |
| SUP-02 | A support with a `nonlinear` block silently loses all its linear restraints | Critical |
| SUP-03 | Variable spring hangers carry no preload: installed, cold and hot loads are metadata only | Critical |
| SUP-04 | Constant-effort force is applied in every load case, doubles in combinations, and is reported as a 0 N reaction | High |
| SUP-05 | Friction is per global DOF (square law), not vector Coulomb: up to 1.41 μN, in the wrong direction | High |
| SUP-06 | Friction cannot sit on a lift-off (one-way) support; the normal must come from a bilateral linear restraint | High |
| SUP-07 | No hanger design: no sizing, no catalog selection, no load-variation or travel check for variable springs | High |
| SUP-08 | Support loads are reported only as a force-resultant magnitude: no signed components, no moments, no local axes | High |
| SUP-09 | Restraints only along global axes: no skewed or pipe-local restraints, no connected-node restraints | High |
| SUP-10 | No imposed displacements (nozzle or anchor movements), no nozzle flexibility, no nozzle-load report | High |
| SUP-11 | Linear combinations of nonlinear load cases are superposed silently | High |
| SUP-12 | No per-load-case support status: no operating-state sustained case, no locked springs for hydrotest, no snubbers | High |
| SUP-13 | No double-acting gap or limit stop; the two-support workaround blocks or double-reports | Medium |
| SUP-14 | Stiffness entered on anchor, guide, stop or rest supports is silently ignored | Medium |
| SUP-15 | Family semantics: `vertical_support` is locked to UZ, and rotational restraint needs the anchor family | Medium |
| SUP-16 | Friction solve cost grows as roughly n⁵: iterations ≈ shoes + 1, with a dense solve per sliding shoe | Medium |
| SUP-17 | Stability diagnostics: "singular system at pivot 9", misleading missing-DOF lists | Medium |
| SUP-18 | Convergence is an exact state count with zero tolerance; no force-residual gate and no anti-chatter | Low |
| SUP-19 | Authoring inconsistencies (spring DOF versus restraints, sign-convention burden, duplicate behaviours) | Low |

---

## Findings

### SUP-01: Active-set iteration cap of 4 blocks the whole analysis on ordinary models (Critical)

**Evidence**
- `pp:67`: `const DEC_046_PRODUCT_PREVIEW_ACTIVE_SET_MAX_ITERATIONS: usize = 4;` with `..._RESIDUAL_TOLERANCE: f64 = 0.0` (`pp:68`), passed to the loop by `product_preview_convergence_control` (`pp:2236`).
- Convergence needs one extra confirming iteration with zero state changes (`ns:528`, `converged: residual_norm <= input.tolerance`). So only 3 state-changing passes are available.
- Non-convergence is fatal for the whole run. `pp:1377-1397` pushes `SOLVER_SYSTEM_BLOCKED` "nonlinear selected mechanics state is unavailable or did not converge" and returns an empty case. The caller then returns `blocked_envelope` (`pp:1049-1053`), so no results are produced for any load case.
- Probe `p2.py` is an anchored 10–40 m riser into a horizontal run on one-way +Z rests every 1–3 m, far end anchored, with W + ΔT in one case:

| Model | Rests | Result with cap 4 | Iterations actually needed (cap 100) |
|---|---|---|---|
| H = 10 m, ΔT = 200 °C, 3 m spans | 10 | converged in exactly 4 | 4 |
| same, rests seeded `inactive` | 10 | **blocked** | 5 |
| H = 15 m, ΔT = 250 °C, 2 m spans | 15 | **blocked** | 6 |
| H = 30 m, ΔT = 350 °C, 2 m spans | 20 | **blocked** | 8 |
| H = 30 m, ΔT = 350 °C, 1 m spans | 40 | **blocked** | 11 |
| H = 40 m, ΔT = 350 °C, 1 m spans | 60 | **blocked** | 14 |

- The friction probe `p3.py` is a straight 3 m-span rack line anchored at one end with bilateral +Z supports and UX/UY friction (μ = 0.3). **Five shoes already fail** with cap 4. They need 6 iterations; 10 shoes need 11; 40 shoes need 41; 80 shoes need 81. The stick-slip front advances one shoe per iteration.
- With the cap raised, the converged states are physically correct. For the 15-rest case, rests R1–R4 are lifted with u_z = +51.7 / +29.5 / +12.2 / +2.7 mm and R = 0. All other rests have u = 0 and R > 0. The algorithm is sound; the cap is what fails.
- The verification suite never catches this. Every nonlinear hand calc in `validation/hand_calcs/*/assembled_*` expects 2–4 iterations on 1–4 supports.

**Why it matters.** Most real hot lines have several resting supports that lift off near risers or at loops, and many shoes with friction. With cap 4, almost any real operating case with lift-off or friction produces no results at all. The only remedy offered is "raise max_iterations", which is a compile-time constant the user cannot change.

**Recommended fix.**
- Raise the default cap to about 100 and expose it as a user setting. Commercial tools use caps of this order and let the user raise them.
- Keep the loud failure, but report the per-support state history of the last iterations, so the user can see oscillating supports.
- Add safeguards against cycling for large models:
  - detect oscillation (the same support flipping two or more times) and freeze or under-relax that support;
  - release only the most-tensile contact per pass when a cycle is detected;
  - apply a small force and gap tolerance (see SUP-18).
- Add a benchmark with at least 20 resting supports and at least 20 friction shoes to the validation suite.

### SUP-02: A support with a `nonlinear` block silently loses all its linear restraints (Critical)

**Evidence**
- `pp:3517`: `if support.nonlinear.is_some() { return None; }` drops the support from the linear set. `build_nonlinear_supports` (`pp:3134-3382`) reads only `nonlinear.dof`, so the `restraints` array is never used.
- There is no diagnostic in `validation.rs` or in the authoring validator. `core/model_operations/operation_applier/src/rich_authoring.rs:409-420` validates the nonlinear block but does not require `restraints` to be empty.
- The desktop form invites exactly this pattern. It shows "Restrained degrees of freedom" checkboxes (`form:196`) and an "Include nonlinear behavior" section (`form:252`) on the same support record.
- Probe `p9.py`: a 6 m cantilever tip support with `restraints: ["UY"]` and a nonlinear one-way `UZ` rest, loaded with W and a 1 kN +Y tip force. The result is **u_y = 30.7 mm** at the "guided" tip, with no warning. The UY guide simply does not exist in the solve.

**Why it matters.** The most common physical support in a pipe rack is a guided resting shoe: +Y rest (nonlinear) plus a lateral guide. An engineer who models it as one support gets an unguided line. Lateral displacements, anchor loads and stresses are wrong, and nothing on screen says so.

**Recommended fix.** Either model a support as a set of restraint rows, each linear or nonlinear, so one support can be "+Y rest with friction, guided in X" as in CAESAR II and CAEPIPE. Or, at minimum, make a non-empty `restraints` array on a nonlinear support a blocking diagnostic in both the solver and the authoring validator.

### SUP-03: Variable spring hangers carry no preload; installed, cold and hot loads are metadata only (Critical)

**Evidence**
- The hanger is assembled as a bare ground spring. At `pp:3547-3567`, a support of family `spring` or variable hanger becomes `LinearSupport::spring(...)` with stiffness only. `installed_load`, `cold_load` and `hot_load` are never read by any solve path.
- The only consumer of those fields is `append_spring_hanger_user_input_results` (`pp:8063-8200`), which echoes them as "spring_hanger_user_input_review" rows.
- The spring reaction is reported as `-k·u` (`pp:1478`): `vector[...] = -spring.stiffness.value * displacements[global]`.
- Validation nonetheless makes the three loads **mandatory** (`validation.rs:787-797`, `SPRING_HANGER_LOAD_MISSING`, blocking). The user must enter them, and nothing uses them.
- Probe `p1.py`: a 6 m cantilever, w = 300 N/m, variable spring at the tip, k = 50 kN/m, installed = cold = hot = 675 N (the propped-cantilever reaction 3wL/8):

| Quantity | SWBPIPE | Correct hanger with 675 N preload |
|---|---|---|
| Hanger load | **408.9 N** (= k × 8.18 mm) | 675 N |
| Tip vertical displacement | **−8.18 mm** | 0 mm |
| Anchor reaction | **1391 N** | 1125 N |
| Anchor moment | 2947 N·m | 2700 N·m (wL²/2 − 675 × 6) |

**Why it matters.** A variable spring is designed to carry its hot load at the hot position, set by its cold (installed) preset. Without the preset force the hanger is only a soft spring. The pipe sags by W/k at every hanger. Sustained stresses and adjacent-support and nozzle loads are wrong, typically unconservative for nozzles. The error grows with softer springs, where W/k can reach tens of millimetres. The practitioner sees "hot load 675 N" in the results next to a solve that used 409 N.

**Recommended fix.** Use the standard hanger representation (CAESAR II / CAEPIPE / AutoPIPE):
- The spring has stiffness k, plus a preload force equal to the installed (cold) load. The preload acts opposite to gravity and is referenced to the installed position.
- The preload is applied in every weight-bearing case (the "H" load component in CAESAR II terms) and not in thermal-only or occasional-only primitive cases.
- The operating hanger force is then F = P_cold − k·Δ_travel.
- Report the computed hot load and travel, and compare them with the entered hot load.

### SUP-04: Constant-effort force is applied in every load case, doubles in combinations, and is reported as a 0 N reaction (High)

**Evidence**
- `add_constant_effort_support_loads(&mut force, model)` runs unconditionally in `solve_load_case` (`pp:1301`) for every load case, whatever its content. The sign convention string says so plainly (`pp:8306`): "applied along the positive axis ... in every solved load case".
- The constant-effort support is excluded from the linear support set (`pp:3520`), so its reaction vector stays [0, 0, 0].
- Probe `p4.py`: 6 m cantilever with a 675 N constant-effort support at the tip.

| Case | Result |
|---|---|
| W | tip u_z = 0 (correct) |
| T (ΔT = 100 °C, axial only) | tip **u_z = +20.7 mm**, anchor reaction 675 N. A pure axial thermal case must give zero vertical movement. |
| Combination W + T | constant-effort applied load **1350 N**, tip **+20.7 mm**, anchor **450 N**. Correct values: 675 N, 0 mm, 1125 N. |
| `reaction_resultant` for the constant-effort support | **0.0 N** in every case and combination |

**Why it matters.** Users who build primitive cases and combine them (as the combination feature invites) get a phantom upward load in the thermal case and a doubled hanger force in the combination. The support-load summary shows 0 N at a support that carries 675 N, so structural loads passed to civil engineering are wrong.

**Recommended fix.**
- Treat the constant-effort force as the hanger ("H") load component, applied only in weight-bearing cases or where the user includes it explicitly.
- Report it as that support's reaction, as a signed global component and in the support-load table.
- Also add the standard constant-effort check: computed travel within the rated travel plus the manufacturer's overtravel allowance. The existing travel warning is a good start.

### SUP-05: Friction is per global DOF (square law), not vector Coulomb (High)

**Evidence**
- Each friction "support" acts on one global DOF: `NonlinearSupport { dof, ... }` (`ns:38-46`).
- The slip limit is scalar per DOF: `let limit = coefficient * normal.abs(); if tangential.abs() <= limit` (`ns:668`). The sliding force is applied per DOF: `-candidate.direction * candidate.coefficient * normal.abs()` (`ni:866`).
- Two-dimensional friction on a rest therefore needs two independent supports (UX and UY) sharing the same normal. Each takes the full μN, and each decides stick or slip on its own.
- Probe `p3.py` (10 shoes, L-shaped line, μ = 0.3), results at shoe N10:
  - N = 2784.8 N, so μN = 835.4 N.
  - FX = −835.4 N and FY = +835.4 N. The resultant is **1181.5 N = 1.414 μN**, acting at 45°.
  - The sliding direction is (u_x, u_y) = (71.8, −21.6) mm, which is **16.7°** from X.
  - Vector Coulomb friction would give about (−800, +240) N. The lateral friction force is overstated by about 3.5×.

**Why it matters.** Friction loads on guides and anchors, and the resulting restraint of thermal growth, drive rack design and anchor or nozzle loads. A square friction law overstates friction by up to 41% in magnitude. It also puts the force in the wrong direction for any movement not aligned with a global axis. The friction models in CAESAR II, AutoPIPE and CAEPIPE apply the friction force in the plane of the support, opposing the direction of movement.

**Recommended fix.** Make friction an attribute of the contact (rest or guide) support, not a separate per-DOF support:
- Take the normal N from that support's own reaction.
- The tangential plane is perpendicular to the restraint direction.
- Use a circular Coulomb cone: stick when |F_t| ≤ μN, and use a single stick or slip state per contact.
- When sliding, the force vector is μN opposing the tangential displacement vector.
- A friction stiffness (regularised stick) is the common robust implementation.

### SUP-06: Friction cannot sit on a lift-off (one-way) support (High)

**Evidence**
- `pp:3040-3050` reports that "friction normal_reaction_source must reference a linear support restraint". A nonlinear source is blocked. The desktop picker filters out nonlinear supports as sources: `!record(s).nonlinear` (`form:97`).
- The only alternatives are a fixed user-entered normal force (constant across all load cases) or a bilateral linear restraint, which can pull the pipe down.
- Any friction support also disables the singular-seed recovery path (`ni:396-397` returns `None` from `eligible_contact_dofs`). The preliminary linear solve then fails hard if the base system without nonlinear supports is singular (`pp:1323-1343`, `Err(error) => return Err(error)`).
- The same `None` removes all one-way supports from the minimum-restraint count (`pp:888-912`). That can block a model that is actually stable.

**Why it matters.** Practitioners overwhelmingly model a rack shoe as "+Y with μ = 0.3": it can lift off, and it carries friction only while in contact. Here the only way to get derived friction is a bilateral vertical restraint. That restraint holds the pipe down where it would lift, and it reports friction from a tensile "normal" force, taken as |N| at `ni:1933`. So lift-off and friction, the two dominant nonlinear effects, cannot be combined on the same support.

**Recommended fix.** Implement friction on one-way and gap supports, with the normal taken from the same contact. Friction then switches off when the contact opens, which is standard in CAESAR II and CAEPIPE. Allow singular-seed recovery with friction present, and count one-way contacts in the restraint check.

### SUP-07: No hanger design, sizing or catalog selection (High)

**Evidence**
- The hanger-selection panel states it outright: "Selection does not size a hanger or establish solve readiness" (`apps/desktop/src/features/hanger-selection/HangerSelectionPanel.tsx:103`).
- `buildHangerSelectionBatch` (`hangerSelection.ts`) copies a user-imported record's fixed values into the support.
- There is no restrained-weight case, no free-operating travel case and no spring-rate or range selection anywhere in `pp`.
- The only hanger diagnostics are input-presence checks (`SPRING_HANGER_*` in `validation.rs:743-830`).
- For variable springs there is no comparison of computed travel with the travel range, and no load-variation check. The only travel comparison is for constant-effort supports (`pp:8517-8556`).

**Why it matters.** Hanger design is a core daily task in flexibility analysis. CAESAR II, CAEPIPE and AutoPIPE all size springs automatically. The user must already know the hot and cold loads and the spring rate, which requires the very analysis the program is meant to perform. There is also no guard against a spring that bottoms out or tops out, or that exceeds the load variability limit.

**Recommended fix.** Implement the standard hanger-design sequence:
1. Solve a restrained-weight case with each hanger location replaced by a rigid +vertical support. That support's load is the hot (design) load.
2. Solve an operating case with the hangers replaced by their hot loads as applied upward forces, to get the vertical travel at each hanger.
3. Select from a user-imported catalog the spring size and rate whose working range contains both the hot load and the cold load (hot load plus k × travel, with sign). Enforce load variability k·|Δ| / hot load ≤ 25% (MSS SP-58 practice, user-settable). Fall back to constant-effort when travel or variability exceeds the limits.
4. Re-solve with the selected springs and their preloads (SUP-03).
5. Report a hanger table: size, rate, hot and cold load, travel and variability.

### SUP-08: Support loads are reported only as a force-resultant magnitude (High)

**Evidence**
- `solve_load_case` builds a 3-component force vector per support and keeps only translational slots (`if slot < 3`, `pp:1467` and `pp:1487`).
- It emits a single row, `kind: "reaction_resultant"`, with `value: magnitude` (`pp:1492-1503`).
- Combinations do the same, and the code comment says "No new public component rows: reaction vectors are internal selected support actions" (`pp:9068`).
- Probe `p1.py` lists every result row for the anchor: only `reaction_resultant = 1391.13 N`. There are no FX, FY, FZ, MX, MY or MZ values. The 2947 N·m anchor moment is obtainable only indirectly, from element end forces in element-local axes.
- Signed nonlinear reactions exist per nonlinear DOF (`nonlinear_support_final_reaction`, `pp:2160-2178`), but only on the nonlinear path.

**Why it matters.** The deliverables of a stress analysis include a restraint-load summary: signed forces and moments per support, per case, with maximum and minimum envelopes. These go to the civil and structural engineers, to vendors (nozzle loads) and to hanger manufacturers. A magnitude cannot be used to design steel (it does not separate vertical from lateral) or to check nozzle allowables (it has no moments). The sign needed to verify a +Y rest is actually in compression is also lost.

**Recommended fix.** For every support and every case or combination, emit signed FX, FY, FZ, MX, MY and MZ in global axes. Also emit them in a support-local or pipe-local frame (axial, lateral, vertical), and for nozzles in the nozzle frame. Include spring, hanger and constant-effort forces. Provide a restraint-summary table with maximum and minimum envelopes across operating, sustained and occasional cases, as CAESAR II's Restraint Summary does.

### SUP-09: Restraints only along global axes; no skewed, pipe-local or connected-node restraints (High)

**Evidence**
- `parse_dof` accepts only UX to RZ (`pp:9540-9550`).
- `LinearSupport` is `{node_index, restrained_dofs: Vec<FrameDof>, ...}` with no direction vector and no second node (`ls:117-125`). `NonlinearSupport` has one global `dof` (`ns:38-46`).
- Every restraint is to ground (`prepare_boundary`, `ls:386-428`). There is no direction-cosine restraint, no "guide perpendicular to the pipe axis" option and no connecting node.

**Why it matters.**
- A guide on a line that runs skewed in plan (say 30°) cannot be modelled. The engineer must restrain both UX and UZ, which also creates an axial stop and invents large thermal loads, or leave it unguided.
- Line stops on skewed lines have the same problem.
- Guides on sloped lines are fine for the horizontal lateral direction. A "vertical" rest on a steep slope, however, cannot be made normal to the pipe.
- Supports from pipe to pipe, trunnion to header, shoe on flexible steel, or pipe to vessel cannot be represented, because every restraint is to rigid ground.

**Recommended fix.**
- Add a restraint direction vector (direction cosines) for translational and rotational restraints, implemented by transforming the constrained DOF (constraint transformation) or by a stiff element along the direction.
- Offer pipe-local shortcuts: guide = the two directions perpendicular to the element axis; line stop = the element axis.
- Add connected-node restraints: a restraint between node i and node j, with rigid, spring, gap, one-way and friction behaviours applied to relative displacement.

### SUP-10: No imposed displacements, nozzle flexibility or nozzle-load reporting (High)

**Evidence**
- The `linear_supports` crate supports imposed displacements (`SupportFamily::ImposedDisplacement`, `prepare_imposed_displacement`, `ls:593-632`), but the product never creates one:
  - `rigid_linear_support_from_preview` hard-codes `imposed_displacement: None` (`pp:3639`);
  - `validate_support_family_tokens` has no `imposed_displacement` family (`pp:3589-3617`);
  - `parse_category` has no `imposed_displacement` category (`pp:9599-9612`);
  - the solve uses `reduce_system(stiffness, &force, restrained_dofs)` with zero prescribed values (`pp:1310`).
- There are no nozzle, WRC 297 or equipment concepts anywhere in `pp`. A nozzle can only be approximated as an anchor plus up to six separate single-DOF ground springs, and those springs have no coupling terms.

**Why it matters.** Almost every analysed system ends at equipment. Thermal growth of the pump, vessel or tank nozzle (anchor movements), tank settlement and bulging, and nozzle flexibility (WRC 297, API 650 Annex P) routinely govern both nozzle loads and pipe stresses. Nozzle loads then have to be reported in the nozzle's local axes to check against API 610, API 617 or NEMA SM-23 allowables or a WRC 107/297 evaluation. None of this can be done: the program always reports rigid, stationary anchors.

**Recommended fix.**
- Wire per-load-case prescribed displacements and rotations at restraint DOFs (the kernel function `reduce_system_with_prescribed_displacements` already exists).
- Add a nozzle element: a 6×6 (or axial plus two bending) stiffness at a node, from WRC 297, API 650 Annex P or user input, with a local frame given by the nozzle axis.
- Report nozzle loads in that frame.

### SUP-11: Linear combinations of nonlinear load cases are superposed silently (High)

**Evidence**
- `append_combination_results` (`pp:9073-9223`) and `append_combined_vector_magnitude` (`pp:8970-9071`) combine any solved rows linearly. There is no check whether the source cases contained nonlinear supports in different states.
- Probe `p5b.py` (riser-plus-rack model, 10 rests), displacement at N1 near the riser:

| Case or combination | u_z at N1 |
|---|---|
| OPE (W + T in one nonlinear solve), the correct answer | **16.6 mm** (R1 and R2 lifted) |
| SUS | 0 mm (all rests active) |
| THERM alone | 45.4 mm (**all** rests lifted, since the pipe floats without weight) |
| `mechanics` combination SUS + THERM | **45.4 mm**, with no warning |

- By contrast, `result_state_subtraction` OPE − SUS gives the correct nonlinear expansion displacement of 16.6 mm.

**Why it matters.** The superposition error here is 2.7× on displacement, with correspondingly wrong reactions and stresses. The UI encourages building primitive cases and combining them. For nonlinear systems that is invalid, and an experienced engineer knows to avoid it; a new user will not be warned.

**Recommended fix.** When any source case of a linear `mechanics` combination has nonlinear supports whose final states differ between the operands, or any friction or gap, emit a blocking or prominent warning. Point the user to the correct pattern:
- solve OPE as a single nonlinear case containing all loads;
- solve SUS the same way;
- take EXP = OPE − SUS algebraically, as CAESAR II does.

### SUP-12: No per-load-case support status: sustained in operating state, hydrotest locking, snubbers (High)

**Evidence**
- Initial states are one value per support for all cases (`NonlinearSupportInput.initial_state`, `pp:390-409`). Each load case restarts from them (`pp:1957-2030`).
- The classifier is designed so the converged state does not depend on the seed (`ns:567-580` doc comment). A user therefore cannot force a lifted support to stay out of the SUS case.
- There is no concept of support behaviour per load case: no "rigid in hydrotest" for springs, no "active only in occasional cases" for snubbers, and no "use the support status from case X".
- In probe `p5b.py` the SUS case carries R1 = 2004 N and R2 = 1738 N at rests that are lifted off in operation.

**Why it matters.** ASME B31.3 (para. 320 and its Appendix S example of a support that lifts off) and current B31.1 practice require the sustained-stress evaluation to consider the operating support configuration. That means a sustained case with lifted-off supports removed, as in CAESAR II's hot-sustained approach, in addition to the installed state. The installed-state-only sustained case can be badly unconservative next to risers. Hydrotest cases need springs locked (rigid). Seismic and slug cases need snubbers active only in occasional cases.

**Recommended fix.** Allow a per-load-case support-status override:
- "Take nonlinear states from case OPE and hold them fixed" (hot sustained).
- Per-case activation of supports (snubbers; temporary hydrotest supports).
- A locked-spring option for hydrotest.
- Automatically generate SUS(installed) and SUS(operating) when nonlinear supports are present.

### SUP-13: No double-acting gap or limit stop; the two-support workaround blocks or double-reports (Medium)

**Evidence**
- `Gap { closes_when }` is one-sided (`ns:30-35`, `ns:604-636`). A guide or stop with clearance on both sides needs two nonlinear supports on the same node and DOF.
- Seeded `active/active`, as a user would naturally do for a stop, the run blocks with a cryptic error. `ni:1428` reports "active boundary DOF 12 is repeated" (probe `p7.py`, `NONLINEAR_SUPPORT_LOOP_BLOCKED`).
- Seeded `inactive/inactive`, it solves correctly (u_x = 5.0 mm, stop closed). **Both** supports then report the same −684 kN reaction, including the open negative-side stop. Reactions are attributed per node DOF, not per support (`pp:1484-1490`, and `pp:2160-2178` reads `solve.reactions[global]`).

**Why it matters.** "Guide with ±3 mm gap" and "line stop with 25 mm gap" are everyday supports. The workaround either fails with an unexplained error or reports a large load at a stop that is open.

**Recommended fix.** Add a double-acting gap behaviour (+gap and −gap on one restraint) with states open, closed-positive and closed-negative. Attribute each reaction to the contact that actually carries it, not to the node DOF.

### SUP-14: Stiffness entered on anchor, guide, stop or rest supports is silently ignored (Medium)

**Evidence**
- `build_model` uses `support.stiffness` only when `family == "spring"` or the support is a variable hanger (`pp:3547`). Every other family goes to `rigid_linear_support_from_preview`, which discards stiffness (`pp:3619-3641`). There is no diagnostic.
- The desktop form offers "Add support stiffness" on every family (`form:213`).
- Probe `p6.py`: a tip support with k = 1000 N/m on UZ. As `guide` or `vertical_support` it gives u_z = 0 and R = 675 N (rigid). As `spring` it gives u_z = −20.1 mm and R = 20.1 N.

**Why it matters.** Engineers routinely give guides and stops a finite stiffness to represent flexible steel. An entry the program quietly ignores is worse than no entry.

**Recommended fix.** Either honour the stiffness on any restraint row (rigid when absent, finite spring when present, including on one-way and gap contacts), or block with a clear message.

Note that rigid restraints use exact DOF elimination (`reduce_system`), which is good (see "Done well").

### SUP-15: `vertical_support` is locked to UZ, and rotational restraint needs the anchor family (Medium)

**Evidence**
- `ls:657`: `SupportFamily::VerticalSupport => dof == FrameDof::Uz`. Probe: a `vertical_support` on UY blocks with "Uy is not valid for VerticalSupport". The gravity axis, however, is user-selectable (`SelfWeightPlanPanel.tsx:85`), and Y-up is the default convention in CAESAR II and CAEPIPE.
- Guide and line-stop families allow only translations (`ls:652-660`). A support with no family and fewer than 6 DOFs falls back to `Guide` (`pp:3630`), so any rotational restraint on it is blocked. Partial rotational restraints are possible only by naming the support an "anchor".
- The spring fallback DOF is `unwrap_or(FrameDof::Uz)` (`pp:3565`).

**Why it matters.** Models imported from or modelled in Y-up conventions cannot use the vertical-support family. The family names imply physics the code does not implement consistently.

**Recommended fix.** Take the vertical axis from the model's declared gravity axis. Let family be a label only, with the restraint rows as the authority. Allow rotational restraints on any rigid support.

### SUP-16: Friction solve cost grows as roughly n⁵ (Medium)

**Evidence**
- For each sliding friction support whose normal is derived, `solve_iteration_with_sliding_friction` performs one extra full solve of the complete system, **forced to dense** (`LinearSolveMode::DenseScrutiny`, `ni:895` and `ni:927`), plus the base solve and the final solve.
- Probe `p3.py` needed about n + 1 iterations for n shoes. Timings with cap 100:

| Shoes (nodes) | Iterations | Wall time |
|---|---|---|
| 20 (24) | 21 | 0.11 s |
| 40 (44) | 42 | 1.75 s |
| 80 (84) | 81 | **38 s** |

- The pattern is O(n) iterations × O(n) solves × O(N³) dense factorisation. A 500-node rack model with 150 shoes would be out of reach.

**Why it matters.** Friction on every shoe is standard for rack lines, so this becomes the limiting cost for real models once SUP-01 is fixed.

**Recommended fix.**
- Factor the reduced stiffness once per active set (sparse LDLᵀ) and reuse it for all unit-load influence right-hand sides.
- Better: adopt a friction-stiffness or return-mapping formulation in which slip is a local update and no influence matrix is needed.
- Allow many simultaneous stick-to-slip transitions per iteration.

### SUP-17: Stability diagnostics are poor (Medium)

**Evidence**
- Probe `p8b.py`: a straight line with translational guides only (torsion about the pipe axis is free). The only message is `SOLVER_SYSTEM_BLOCKED` "singular system at pivot 9". That is an index into the reduced system, not a node or DOF.
- Probe `p8.py` (5 ground DOFs) reports "missing global rigid-body DOF classes: RX, RY, RZ". RY and RZ are in fact restrained by the two translational supports; only RX is a mechanism. The check at `pp:902-922` counts directly restrained DOF classes, not rigid-body modes.
- With friction present, one-way supports are excluded from the count (see SUP-06).

**Why it matters.** Finding the unrestrained mode is the first debugging step on any new model. A pivot index leaves the user guessing.

**Recommended fix.** On a singular or near-zero pivot, map the pivot back to node and DOF. Better, compute the rigid-body null space of the reduced stiffness and report the mechanism: which nodes move, and in which translational or rotational sense. Count nonlinear contacts as potential restraints in the pre-check.

### SUP-18: Convergence is an exact state count with zero tolerance (Low)

**Evidence**
- The residual is the number of supports whose state changed (`ns:494`). The tolerance is 0.0 (`pp:68-69`).
- Contact decisions use strict signs with no tolerance: `reaction > 0.0` (`ns:720-731`) and `displacement < 0.0` (`ns:735-746`).
- Force, work and energy residuals are computed but only reported (`ni:652-663`, first two assumptions).

**Why it matters.** For supports that just touch, with a reaction near 0 N or a gap near zero, floating-point noise can flip the state on every pass and exhaust the cap. This is not demonstrated here; see "Suspected". A state count also gives no measure of how close the solution is.

**Recommended fix.** Use small user-settable contact tolerances: a force tolerance on the reaction sign, and a gap or penetration tolerance in length units. Accept convergence on "no state change AND the displacement change is below tolerance".

### SUP-19: Authoring inconsistencies (Low)

- For springs and variable hangers the acting DOF comes from `stiffness.dof`. The `restraints` array is ignored even when it disagrees (`pp:3547-3567`). The hanger-selection panel asks the user to confirm restraint directions (`HangerSelectionPanel.tsx`) that the solver never reads.
- One-way support sense must be authored as `active_when: positive_reaction` or `negative_reaction` in the global sign convention. There is no "+Y rest" or "−Y hold-down" vocabulary, and no check against the gravity direction.
- `OneWay` and `LiftOff` have identical classification logic (`ns:597-603` and `ns:637-643`), which duplicates concepts in the UI.
- A friction support with an explicit `normal_reaction` uses the same fixed normal force in every load case, including cases without weight.

---

## Done well

- **Rigid restraints are exact constraint elimination.** `reduce_system` removes restrained DOFs, so there are no penalty springs and no conditioning problems from 1e12-type stiffnesses. Repeated restraint on the same DOF is blocked (`ls:634-650`).
- **The active-set classification is correct in principle** (DEC-067, `ns:567-680`). Engaged contacts test the reaction sign, and released contacts test penetration or clearance. With enough iterations the probes converge to states that satisfy complementarity: every lifted rest has R = 0 and u > 0, and every active rest has R > 0 and u = 0.
- **Failure is loud.** An unconverged nonlinear state is never used silently (`pp:1377-1397`), and every non-converged exit carries a diagnostic (`ni:614-632`).
- **Per-support nonlinear evidence** is emitted: state code, signed displacement and signed reaction at the nonlinear DOF, iteration count and residual rows (`pp:2034-2200`).
- **Sticking friction is an exact constraint, and sliding is a bounded ±μN force** with admissibility checks. The coupled derived-normal system is solved simultaneously rather than lagged (`ni:843-1058`). The physics is careful even though the friction law is per-DOF (SUP-05).
- **Springs, curved-bend macro-elements and user-stiffness elements** enter the nonlinear loop consistently with the linear path (`pp:1970-2025`).
- **Nonlinear-appropriate EXP.** `result_state_subtraction` (OPE − SUS) is available and gave the correct expansion displacement in probe `p5b.py`.
- **The constant-effort user-limit travel warning** (`pp:8517-8556`) is the right kind of check; it should be extended to variable springs.
- **No-defaults input discipline.** Hangers, friction and gaps all require explicit user values. The friction normal-source validation is thorough.
- **Every nonlinear support class has a hand calculation** in `validation/hand_calcs/*/assembled_*`. These are small, but they are the right foundation for the larger benchmarks recommended in SUP-01.

## Suspected, not verified

- **Chatter at near-zero contact reactions (SUP-18).** With a strict `> 0.0` sign test and zero tolerance, a support at incipient contact could oscillate between active and inactive until the cap. No model reproduced this, because the probes had clear-cut contact.
- **Derived-normal branch switching.** When a sliding shoe's derived normal changes sign between iterations, `derived_normal_branch_admissible` (`ni:1252-1269`) forces a retry. Coupled with SUP-01 this may add non-convergence on models where the bilateral vertical source goes into tension. This was not isolated in a probe.
- **Dense full-model stiffness in the nonlinear loop.** `assemble_global_stiffness_with_user_elements` returns a dense N × N matrix that is used for every reaction recovery (`ni:1505-1509`). This is probably a memory and time limit for large models independently of SUP-16. It is likely covered by the solver-performance review, and was not measured here beyond the timings above.


---

## Appendix — D. Load cases, combinations, stress recovery and results (STR-)

Reviewer scope: the live path in `core/product_physics/src/lib.rs` (`solve_load_case`, stress and combination code), `core/loads/stress_recovery`, and `core/loads/load_case_algebra`. There is also a short look at how the desktop results panel presents the rows.

All line numbers refer to the read-only checkout at `/Users/ryan/dev/chirality-review/projects/chirality-piping/`. Paths are relative to that checkout.

Probe programs are in `/private/tmp/claude-501/-Users-ryan/08937a6e-e544-4c91-b97d-34e629979c7f/scratchpad/review/str_probe/`:

- `src/main.rs` is a thin binary around `run_linear_static_preview`.
- `gen.py` builds the models.
- `p1.py` to `p7.py` are the individual probes.

Every probe uses the same pipe: a 6 in STD line with OD 168.3 mm and t 7.11 mm, E 200 GPa and G 77 GPa. Its derived properties are:

| Property | Value |
|---|---|
| A | 3.600e-3 m² |
| Z | 1.3923e-4 m³ |
| A_i | 1.8646e-2 m² |

---

## Summary table

| ID | Title | Severity |
|---|---|---|
| STR-01 | Longitudinal pressure stress is suppressed whenever pressure is applied, and straight pipe and bend disagree | Critical |
| STR-02 | SIF review row multiplies stress by SIF × k, with a single SIF, no torsion, and only at one node | Critical |
| STR-03 | SIF review rows are combined, subtracted and enveloped as if they were linear | Critical |
| STR-04 | Pressure "thrust" pair is applied to every pressurised element: an always-on pressure elongation without Poisson, and spurious anchor loads | High |
| STR-05 | `open_formula_stress_summary` sums \|σby\| + \|σbz\| (up to √2 high) and ignores torsion (zero for pure torsion) | High |
| STR-06 | No code stress categories, stress ranges or allowables, and load cases carry no type | High |
| STR-07 | Post-solve superposition across cases with nonlinear supports, with no warning | High |
| STR-08 | Reactions are published only as a translational resultant magnitude, with no components and no moments | High |
| STR-09 | Combination algebra is too thin for practice: no nesting, no reuse of loads, no ABS or SRSS, no true range, no modulus check | Medium |
| STR-10 | Summary maximum displacement and stress come from the first load case only | Medium |
| STR-11 | One reduced wall is used for stiffness and every stress; no corrosion allowance, no branch Ze, and a mean-radius pressure formula | Medium |
| STR-12 | Results are a flat, paginated row list with none of the tables a practitioner expects | Medium |
| STR-13 | Fixed 6-decimal publication rounding loses precision on small rotations | Low |
| STR-14 | The curved-bend maximum is sampled at 5 stations only | Low |
| STR-15 | Per-support reaction double-reports when two supports share a node DOF, and a spring overwrites a rigid component | Low |

---

## Findings

### STR-01: Longitudinal pressure stress is suppressed whenever pressure is applied, and straight pipe and bend disagree

**Severity:** Critical

**Evidence**

1. Every genuine element pressure load becomes a `PressureThrustLoad`. This happens at `lib.rs:6715-6752`, with `axial_load: load.magnitude.value * section.internal_area` at `lib.rs:6746`. It is not limited to expansion joints.
2. `pressure_for_pipe` (`lib.rs:9415`) uses the same selector. So any pipe that has a pressure also has an active thrust.
3. `lib.rs:1754-1756` then does the following:
   ```
   let pressure_thrust_active = pressure_thrust_for_pipe(...) != 0.0;
   let include_pressure_longitudinal = !pressure_thrust_active;
   ```
   In practice, the pressure-longitudinal row and its contribution to the summary are never produced for a pressurised pipe.
4. On straight elements, `corrected_local_forces_for_axial_effects` (`lib.rs:6959-6978`, `corrected[UX] += axial_load`) subtracts the thrust pair in the same way as a thermal equivalent load. The reported axial force is therefore the effective force, N_true − p·A_i, not the wall force.
5. The curved-bend macro span subtracts only the radial wall-load vector and keeps the caps (`lib.rs:6980-6992`). So a bend reports the true wall tension +p·A_i.

**Probe (`p2.py`, `p7.py`, P = 5 MPa)**

| Case | Reported σ_axial | Pressure-longitudinal row | Summary | Correct longitudinal pressure stress |
|---|---|---|---|---|
| Anchor to free end, straight 3 m | 0.000 MPa | absent | 0.000 MPa | +25.9 MPa (p·A_i/A), or PD/4t = 29.6 MPa |
| Anchor to anchor, straight 3 m | −25.894 MPa (compression) | absent | 25.894 MPa | tension (code PD/4t = 29.6 MPa) |
| Straight → macro bend → straight, same pressure | straight 0.000, bend +25.894 | absent | 0.0 / 25.9 / 0.0 | uniform ≈ +25.9 to 29.6 MPa |

The hoop row is published correctly (56.677 MPa = p(D−t)/2t).

**Why it matters.** Longitudinal pressure stress is usually the largest single term in sustained stress. A flexible, well-supported pressurised line reports zero longitudinal pressure stress. An anchored run reports it with the wrong sign. The stress also steps from 0 to 25.9 MPa at every bend tangent point. Any sustained check built on these rows is unconservative.

**Recommended fix**

- Do not apply pressure as element end-cap pairs in the general frame solve. Keep the frame axial force as the mechanical force Fa and always add the longitudinal pressure stress S_lp as a separate term.
- Offer S_lp as either PD/4t or the exact p·d²/(D² − d²), selected by the user. This is the practice in B31.1 and B31.3 and in CAESAR II, AutoPIPE and CAEPIPE.
- Apply pressure thrust as an external load only where the system is not self-restrained: at untied expansion joints (thrust = p × effective bellows area), open ends and user-declared thrust points.
- Make bend and straight recovery follow one convention. Publish the wall force and the effective force as separately named rows if both are wanted.

### STR-02: SIF review row multiplies stress by SIF × k, with a single SIF, no torsion, and only at one node

**Severity:** Critical

**Evidence**

1. `append_component_stress_multiplier_result`, `lib.rs:7842-7846`:
   ```
   let multiplier = if modifier.flexibility_in_assembled_stiffness { modifier.sif } else { modifier.sif * modifier.flexibility };
   ```
   The default `mechanics_geometry_only` mode multiplies by the flexibility factor.
2. The base value is `open_formula_summary_mpa` (see STR-05). The whole summary is scaled, including the axial and pressure terms.
3. `ComponentModifierInput` (`lib.rs:303-329`) has a single `sif_user_value` for bends, and one header SIF and one branch SIF for tees. There are no in-plane and out-of-plane (ii, io) or torsional (it) values.
4. `bend_stress_modifier` ignores `pipe_id`. It is applied to every pipe that has an endpoint at `component.node` (`lib.rs:7687`), and only to endpoint stresses. It is never applied to the far tangent point or to interior bend stations.

**Probe (`p3.py`)**

In both cases the model is an L-shaped line, 3 m + 3 m, anchored at A, with −1000 N Fz at the free end. The bend component has SIF = 2 and k = 5.

- **Sharp corner (geometry-only mode).**
  - Leg 2 at the corner: summary 21.547 MPa, review row **215.47 MPa** (× 10).
  - Leg 1 at the same corner carries the same 3000 N·m as torsion (torsional shear 10.77 MPa). Its review row is **0.000 MPa**.
- **Macro curved bend.**
  - The SIF is applied at the near tangent (bend end-i and the adjoining straight end-j).
  - The far tangent end-j and the arc stations have no SIF row, although the far end summary is 21.7 MPa.

**Why it matters.** For a 6 in STD long-radius elbow, h = tR/r² ≈ 0.25, k ≈ 6.6 and i_i ≈ 2.3. The default mode therefore over-states elbow stress about 6.6-fold. It also reports zero where the elbow moment arrives as torsion from the adjoining leg, which is out-of-plane bending on the elbow. The row is neither conservative nor unconservative in any predictable way.

**Recommended fix**

- Remove k from stress entirely. k belongs only in the element stiffness, through the bend flexibility of the macro element or an equivalent flexible-joint model.
- Carry separate user inputs ii, io and, optionally, it and ia.
- Resolve the element moments at the component into in-plane (Mi), out-of-plane (Mo) and torsion (Mt), using the bend plane or the branch/header plane.
- Form the bending stress as √((ii·Mi)² + (io·Mo)²) / Z, where Z is the branch Z or Ze on the branch side.
- Include torsion by the code method, as in the expansion-stress formula of B31.3 319.4.4 or the resultant moment of B31.1 104.8.
- Apply bend SIFs at both tangent points and at all arc stations. Apply tee SIFs only to the pipe named for that side.

### STR-03: SIF review rows are combined, subtracted and enveloped as if they were linear

**Severity:** Critical

**Evidence**

- `append_combination_results` (`lib.rs:9073-9223`) skips only rows of kind `open_formula_stress_summary` (`lib.rs:9105`).
- `component_user_stress_multiplier_review` rows are absolute-value quantities, but they fall through to the scalar linear, subtraction and envelope algebra.

**Probe (`p5.py`).** There are two cases: down, −1000 N at the tip, and up, +1000 N at the tip.

| Combination | Signed bending σz (correct algebra) | SIF review row | Correct SIF-applied value |
|---|---|---|---|
| down + up (zero net load) | 0.000 | **430.94 MPa** | 0 |
| down − up (a range) | −43.094 | **0.000 MPa** | 2 × 43.09 = 86.2 MPa (SIF only) |

**Why it matters.** Result-state subtraction is the only way to form an expansion range. On the only SIF-applied stress row, it gives zero. A factored sum gives a large non-zero stress for a load-free state.

**Recommended fix**

- Never combine derived nonlinear rows: summaries, SIF-applied stresses, or magnitudes other than the vector-handled ones. Add these kinds to the exclusion list now, as a stop-gap.
- In the proper fix (STR-06), combine the signed force and moment components per element end and station first. Then compute SIF-applied and code stresses from the combined components.

### STR-04: Pressure "thrust" pair is applied to every pressurised element: an always-on pressure elongation without Poisson, and spurious anchor loads

**Severity:** High

**Evidence**

- `add_pressure_thrust_loads` (`lib.rs:6826-6866`) adds −pA_i·x at node i and +pA_i·x at node j of every pressurised element.
- This is mechanically identical to an imposed axial strain of p·A_i/(EA) ≈ σ_L/E. The hand calculation `validation/hand_calcs/mechanics/tp_phys_008_thermal_pressure_axial_effects.md` treats it exactly like thermal expansion.

**Probe (`p2.py`, 5 MPa)**

- **Free-end elongation over 3 m.** The model gives **0.388 mm**. Real closed-end elongation is (σ_L − ν·σ_h)·L/E. Using 28.3 MPa and 56.7 MPa with ν = 0.3, that is **0.170 mm**, so the model is 2.3 times too high.
- **Straight run anchored at both ends.** Each anchor reaction is **93,229 N**. This is the full p·A_i. The value is about 0 in the default practice of commercial tools (no Bourdon effect). It is about 41 kN with the Bourdon and Poisson effects included.
- **L-shape anchored at both ends.** Each anchor reports 570 N, and the leg carries a 604 N·m bending moment, from pressure alone.

**Why it matters.** Nozzle and anchor loads on pressurised lines are overstated. The straight anchor-to-anchor case is overstated grossly. The effect cannot be switched off.

**Recommended fix.** See STR-01.

- Make pressure elongation (the Bourdon axial effect) an explicit option. Use the closed-end strain (σ_L − ν·σ_h)/E, or zero when the option is off, and include the bend-opening part if bends are modelled.
- Apply true thrust only at expansion joints and declared open ends.

### STR-05: `open_formula_stress_summary` sums |σby| + |σbz| and ignores torsion

**Severity:** High

**Evidence**

- `lib.rs:7632-7656`: `bending_total = |σby| + |σbz|`, and the value is `max(|σa + σlp ± bending_total|)`.
- `stress_recovery/src/lib.rs:1041-1054` (`summarize_components`) does the same.
- Torsion is not used.

**Probe (`p1.py`).** The model is a 3 m cantilever.

| Load | Reported summary | Correct value |
|---|---|---|
| Fy = Fz = 1000 N (My = Mz = 3000 N·m) | **43.094 MPa** | Resultant √(My² + Mz²)/Z = **30.472 MPa**, so the summary is √2 high |
| Pure torque 3000 N·m | **0.000 MPa** | Shear stress Mt/(2Z) is 10.77 MPa. Expansion practice gives √(Sb² + 4St²) = 21.55 MPa, or B31.1 Mc/Z = 21.55 MPa |

**Why it matters.**

- This summary is the only per-element stress, the value in `max_open_formula_stress`, and the base of the SIF row.
- For a circular section, the extreme fibre stress comes from the resultant moment. The absolute sum is conservative by up to 41% in skewed bending.
- Torsion-dominated legs, which are common next to elbows in 3-D layouts, show zero.

**Recommended fix**

- Use Mb = √(My² + Mz²) and σ = σa + σlp ± Mb/Z.
- Report a combined stress that includes torsion: the maximum shear or Tresca stress intensity, or the code expansion form √(Sb² + 4St²).
- The code-specific forms belong in STR-06.

### STR-06: No code stress categories, stress ranges or allowables, and load cases carry no type

**Severity:** High

**Evidence**

- `PreviewLoadCase` (`lib.rs:418-442`) has no case type such as sustained, operating, expansion, occasional or hydrotest.
- Primitive-load `category` is parsed (`lib.rs:9599-9612`) but drives nothing in stress. Concentrated forces and moments are silently mapped to `occasional`.
- `recover_stress_range` in `stress_recovery` (`lib.rs:696-782`) exists but is not called from the live path.
- A diagnostic at `lib.rs:1138-1145` (`RULE_CHECK_INPUTS_MISSING`) states that no compliance result is produced. The UI shows `GoverningRatioState ratioCount={0}` (`apps/desktop/src/features/results/ResultsPanel.tsx`).

**Why it matters.** A stress engineer cannot qualify a line. There is no sustained, expansion-range or occasional stress, no stress ratio, and no place to enter allowables.

**Recommended fix.** A minimal framework can stay code-neutral and still be usable:

1. **Case typing.** Every basic load case carries a type: W, P, T, D (imposed displacement), occasional (wind, seismic, other) or hydrotest. Operating cases are solved as nonlinear states: W + P + T (+ D).
2. **Sustained (SUS).** Solve W + P, with the nonlinear supports in their SUS state.
   - Stress: SL = Sa + Sb + S_lp, with torsion handled by the user-selected form.
   - Sa = ia·|Fa|/A, using the mechanical Fa and excluding thrust.
   - Sb = √((ii·Mi)² + (io·Mo)²)/Z, with Z on the corroded or eroded wall.
   - S_lp as in STR-01.
3. **Expansion range (EXP).**
   - Form it as the algebraic difference of signed element-end force and moment components between two solved states. The default is OPE − SUS; the thermal-only case T1 is an alternative.
   - Then compute Se = √(Sb² + 4St²), with Sb from the SIF-multiplied Mi and Mo and St = Mt/2Z. B31.1 uses i·Mc/Z instead.
   - Use Z on the nominal wall, and the cold (installed) modulus Ea for the range solve.
   - The same form applies to a displacement stress range between any two user-selected states, such as OPE1 and OPE2, for a range that crosses zero.
4. **Occasional (OCC).**
   - The occasional-only effect is OPE+OCC minus OPE, solved nonlinearly, or a linear OCC case where there are no nonlinear supports.
   - Then SUS + |OCC| (absolute summation), with SRSS as an option for multiple seismic directions.
5. **Allowables.** The user enters allowables per category and per material or temperature: Sh for sustained, SA for expansion (with the f factor and the liberal SA option), and k·Sh for occasional.
   - Output the stress, the allowable, the ratio and the governing location per case.
   - Keep all code-specific values user-entered, which is consistent with the project's no-protected-data stance.

### STR-07: Post-solve superposition across cases with nonlinear supports, with no warning

**Severity:** High

**Evidence**

- `append_combination_results` (`lib.rs:9073`) linearly combines published per-case rows. It uses `evaluate_linear_combination`, `lib.rs:8931`.
- There is no check that an operand case was solved with nonlinear supports in a different active set. There is no diagnostic in `validation.rs:1808+` or in the combination code.

**Probe (`p4.py`).** The model is a 6 m cantilever with a one-way rest at the tip. Case W is −1000 N and case U is +3000 N.

| Result | Superposed W + U | Solved WU case (truth) |
|---|---|---|
| Tip uz | **92.18 mm** | 61.45 mm |
| Reaction at the lifted rest | **1000 N** | 0 N |
| Anchor reaction | 3000 N | 2000 N |

The only diagnostic is a generic `LOAD_COMBINATION_SOURCE_RESULT_MISMATCH` on the nonlinear-support rows.

**Why it matters.** Lift-off, gaps and friction are the norm on real racks. Superposed results can show a support carrying load after it has lifted off.

**Recommended fix**

- Emit a named warning whenever a mechanics-basis combination includes a case whose nonlinear supports ended in different states from the other operands. Better, block it.
- Steer users to solving the combined load set as its own nonlinear case, which is the operating-case practice.
- Allow algebraic combination only for range forming (STR-06), where subtraction of solved states is the accepted method.

### STR-08: Reactions are published only as a translational resultant magnitude

**Severity:** High

**Evidence**

- `lib.rs:1456-1504` builds a 3-vector from translational slots only (`if slot < 3`, at `lib.rs:1467` and `1487`). It publishes one row, `reaction_resultant` = |F|.
- The comment at `lib.rs:9068` says: "No new public component rows: reaction vectors are internal".
- Anchor moments are never output, and the component forces are not output in any frame.

**Probe (`p1.py`).** A cantilever with a 3000 N·m tip torque reports the anchor reaction as **0.0 N**.

**Why it matters**

- Nozzle and equipment checks need Fx, Fy, Fz, Mx, My and Mz in global and in nozzle-local axes. Examples are the API 610 and API 617 checks, NEMA SM23, and vessel local-stress methods such as WRC 537 and WRC 297.
- Support design needs the signed vertical and lateral loads.
- A magnitude alone cannot be checked against anything.

**Recommended fix**

- Publish signed reaction components for every restrained DOF, including moments, per support and per case or combination.
- Give them in global axes and, optionally, in support-local or nozzle-local axes.
- Combine the components algebraically, and envelope the signed maximum and minimum.

### STR-09: Combination algebra is too thin for practice

**Severity:** Medium

**Evidence**

- `validate_combinations` (`validation.rs:1808+`) requires every operand to be a load case. Combinations of combinations are not possible.
- Primitive-load IDs must be unique across the whole model. The probe hit `DUPLICATE_ID` when one load was reused in two cases. So every operating case must re-author all of its W, P and T loads.
- `RangeMode` offers only min, max, min_abs and max_abs (`load_case_algebra/src/lib.rs:35-66`). The "range_envelope" basis selects one operand. It does not compute a max − min range.
- There is no ABS or SRSS summation method.
- `append_combination_modulus_basis_records` (`lib.rs:6252-6320`) records the modulus basis of each operand. It does not warn when a subtraction mixes hot-modulus and cold-modulus states.

**Why it matters.** Standard load-case sets are tedious to author. The occasional SUS + |OCC| rule and SRSS seismic directions are not expressible. An expansion range formed from a hot-modulus operating case is unconservative relative to the cold-modulus basis the codes use.

**Recommended fix**

- Let cases reference basic loads by ID.
- Allow combinations to reference combinations, with cycle checks.
- Add ABS, SRSS and true range (max − min) methods.
- Warn when range operands use different modulus bases, and state the code convention: the range uses Ea, and hot reactions are scaled by Em/Ea.

### STR-10: Summary maximum displacement and stress come from the first load case only

**Severity:** Medium

**Evidence**

- `lib.rs:1058-1063` sets `max_displacement` and `max_stress` from `load_case_solves.first()`. They are never updated from the other cases or from combinations.
- `HIGH_DISPLACEMENT_REVIEW` (`lib.rs:1128-1137`) uses the same value, against a fixed 5.0 mm threshold.

**Probe (`p5.py`).** A 10 N first case and a 1000 N second case give a summary of **0.215 MPa and 0.23 mm**. The real maxima are 21.5 MPa and 22.6 mm.

**Why it matters.** The headline numbers in the report and summary panels can be two orders of magnitude low, depending only on the order of the cases.

**Recommended fix**

- Compute the maxima across all cases and combinations, and label each with its case.
- Remove or parameterise the 5 mm threshold. Thermal lines routinely move 50 mm or more.

### STR-11: One reduced wall is used for stiffness and every stress

**Severity:** Medium

**Evidence**

- `derive_pipe_section` (`lib.rs:6321-6372`) subtracts the optional mill tolerance.
- The single `DerivedSection` feeds both the element stiffness (`lib.rs:3442-3461`) and all stress recovery (`lib.rs:7602-7630`).
- There is no corrosion or erosion allowance input.
- Stress uses Z = I/(D/2) of the run pipe. There is no effective branch section modulus (Ze) for reduced outlets.
- Hoop stress uses the mean radius, `membrane_radius: (od - thickness)/2` (`lib.rs:6369`), and the longitudinal pressure stress is taken as hoop/2 (`stress_recovery/src/lib.rs:951-957`).

**Why it matters**

- Code practice is stiffness on the nominal wall, sustained stress on the wall less allowances, and expansion stress on the nominal wall. Mixing these under-predicts thermal loads through the reduced stiffness, and gives the wrong stress bases.
- Codes such as B31.1 use a reduced-outlet Ze for branch stresses.

**Recommended fix**

- Carry the nominal thickness, the mill tolerance and the corrosion or erosion allowance separately.
- Use the nominal wall for stiffness, the corroded wall for sustained and occasional Z and A, and the nominal wall for the expansion Z.
- Add a user Ze option on the tee branch side.
- Let the user choose PD/4t or p·d²/(D² − d²) for S_lp.

### STR-12: Results are a flat, paginated row list with none of the tables a practitioner expects

**Severity:** Medium

**Evidence**

- `apps/desktop/src/features/results/ResultsPanel.tsx` groups rows only by family and pages them 50 at a time.
- Each straight element produces about 55 rows per case: 12 endpoint force rows, 18 station force rows, about 25 stress rows and a summary.
- Hanger output is an echo of the user inputs (`append_spring_hanger_user_input_results`, `lib.rs:8063+`).

**What is missing**

- A node displacement table: DX, DY, DZ, RX, RY and RZ per node, per case.
- A restraint summary: signed forces and moments per support, per case, with the operating/sustained envelope.
- An element force and moment table per case.
- A code compliance table: stress, allowable, ratio and SIF per node, per category, with the governing ratio.
- A hanger table: hot and cold loads, travel and load variation.
- A nozzle load summary in nozzle axes.
- Maximum and minimum envelope tables with the governing case named.

**Recommended fix.** Build those standard report tables over the existing row store, and add CSV or XLSX export of each table.

### STR-13: Fixed 6-decimal publication rounding loses precision on small rotations

**Severity:** Low

**Evidence**

- `round6` (`lib.rs:9746-9758`) is applied to every published value (`lib.rs:1158-1163`).
- Rounding correctly happens after all the algebra.

**Probe (`p6.py`).** A 1 N·m tip moment on a 3 m cantilever gives an exact rotation of 1.28e-6 rad. It is published as **1e-6 rad**, a 22% error.

Displacements are in mm, where 1e-6 mm is harmless. Stresses in MPa and forces in N are also fine.

**Recommended fix**

- Round to significant figures, for example 6 or more, or publish full precision and round only in the display.
- Consider publishing rotations in mrad or degrees for the display.

### STR-14: The curved-bend maximum is sampled at 5 stations only

**Severity:** Low

**Evidence**

- Straight elements use an analytic interval search (`straight_summary_extrema`, `lib.rs:6461-6544`), which is good.
- Macro bends use only the ends plus the 0.25, 0.5 and 0.75 stations (`lib.rs:7168`). The bend branch never calls an extremum search (`lib.rs:1880-1896`).

**Why it matters.** For a 90° bend with a sinusoidal moment variation, the worst-case miss is about 1 − cos 11.25° ≈ 2%. This is small, but not zero, and the bend is where SIFs apply.

**Recommended fix.** Use the closed-form a + b·cos θ + c·sin θ variation of the arc moments to find the true maximum. Alternatively, add stations at 1/8 arc spacing.

### STR-15: Per-support reaction double-reports when two supports share a node DOF, and a spring overwrites a rigid component

**Severity:** Low

**Evidence.** In `lib.rs:1458-1491`, each support reads `reactions[node_dof]` for its restrained DOFs. Two supports on the same node and DOF therefore each report the full nodal reaction. A spring entry on the same support overwrites the rigid slot.

**Recommended fix.** Split the nodal reaction among coincident restraints: report it once with a diagnostic, or keep a per-restraint constraint force.

---

## Suspected, not verified

- **The meaning of `active_when` for one-way supports looks inverted relative to intuition.** With `negative_reaction`, a downward tip load let the rest go inactive (−30.7 mm). This belongs to the supports reviewer's area, and I did not trace the sign convention.
- **`core/product_physics/src/pressure_exact.rs`** is labelled "Dormant exact-annulus pressure mechanics". It contains wall and fluid force bookkeeping that could underpin the STR-01 fix, but it is not wired in. I did not assess its correctness.

## Done well

- **Component stress formulas are right, clearly separated and unit-safe.**
  - Axial N/A, bending M/Z per axis and torsion T·r/J.
  - Hoop and longitudinal pressure stresses come from explicit inputs.
  - Missing inputs block with named findings instead of defaulting (`stress_recovery/src/lib.rs:490-540`, `874-960`).
- **Section-cut resultants use one documented j-side convention.** They come from stiffness-recovered end actions, with the consistent distributed-load correction. The straight-element interval search for the governing station is analytic (`lib.rs:6461-6544`).
- **The DEC-070 curved-bend path puts k in the stiffness and applies the SIF only.** Its no-double-counting rule is the right idea. Extend it to all components and drop the k multiplier entirely.
- **Reactions are computed as K·u − F with the equivalent loads included.** This is correct for thermal loads and distributed weight at restrained nodes.
- **Combined displacement and reaction magnitudes use vectors.** The signed x, y and z components are combined first and the magnitude is taken afterwards (`append_combined_vector_magnitude`, `lib.rs:8970-9071`). This is correct.
- **The open-formula summary is excluded from combinations, with a warning.** Signed component rows are combined instead.
- **Rounding happens only at publication, after all algebra.**
- **The per-case modulus basis is recorded on each range operand.** Combination rows carry full source-reference provenance, which is good for audit.


---

## Appendix — E. Verification and validation suite (prefix VER-)

Reviewer scope: `validation/benchmarks/*`, `validation/hand_calcs/*`,
`docs/validation_manual/cases/*`, the tests in `core/product_physics/src/lib.rs`,
and `tests/`. All paths below are relative to
`/Users/ryan/dev/chirality-review/projects/chirality-piping/` unless given in full.

## How I checked

- Read the four benchmark crates, their `Cargo.toml` dependency lists, the hand-calculation notes, the generated validation-manual pages, and the 176 unit tests in `core/product_physics`.
- Ran every suite on current `main`. All pass: `physics_audit_regression` 10, `mechanics` 41, `stress` 23, `nonlinear` 19 and `core/product_physics` 176.
- Wrote a probe crate that calls the live entry point `run_linear_static_preview_with_mode` with realistic magnitudes: 8 in. Sch 40 carbon-steel pipe, D = 0.2191 m, t = 8.18 mm, E = 200 GPa, alpha = 12e-6 /K. The probe and two independent Python force-method scripts are at `/private/tmp/claude-501/-Users-ryan/08937a6e-e544-4c91-b97d-34e629979c7f/scratchpad/review/ver_probe/` (`src/main.rs`, `lbend.py`, `oop.py`).

Probe results used below:

| Probe (live path unless stated) | Program | Independent reference | Verdict |
|---|---|---|---|
| Anchored L-bend, ΔT = 150 K, bend macro-element, k = 1, anchor force resultant | 13842.208549 N | 13842.208549 N (my force method, bending + axial) | Agrees to 10 figures |
| Same L-bend, k = 5, 10 and 20 | 8597.231306, 6955.535380, 5897.295400 N | 8597.231306, 6955.535380, 5897.295400 N | Agrees |
| Cantilevered L-bend, 1000 N out of plane at the tip, k = 1 and k = 3 | 21.211457, 23.656673 mm | 21.211457, 23.656673 mm (bending + torsion) | Agrees |
| Inclined straight run on three translation-only supports, 1000 N·m moment (a torsional mechanism) | `MECHANICS_SOLVED`, rotations about 1.2e12 rad, reactions 256 and 512 N | Should be blocked as singular | **Wrong** (VER-01) |
| Straight cantilever, P = 5 MPa | Longitudinal pressure stress not reported; stress summary 0 MPa | σ_L ≈ 29.8 MPa (P·di²/(D²−di²)) or 33.5 MPa (P·D/4t) | **Wrong** (VER-02) |
| Straight pipe anchored at both ends, P = 5 MPa | Anchor load 161,413 N each; axial stress −29.78 MPa (compression) | Wall axial stress ≈ +ν·σ_h ≈ +16 MPa (tension). Anchor load ≈ 57–74 kN if pressure elongation is modelled, 0 if not | **Wrong** (VER-02) |
| Anchored L-bend, P = 5 MPa | Straight legs carry 0.6–1.0 kN axial force and 4.7–6.6 MPa summary stress; the bend carries 113 kN and 34.4 MPa | Every leg should show the same σ_L of about 30 MPa | **Inconsistent** (VER-02) |

The last three rows are a solver defect. I include them because the verification suite should have caught them and did not: the hand calculation that "verifies" pressure encodes the same model (VER-02).

---

## Coverage map

"Independent" means compared against a closed-form or hand-derived value. "Self" means compared against another part of the same code, or against the same function's output.

| Feature | Component crates (`validation/benchmarks`) | Live path (`core/product_physics`) | Status |
|---|---|---|---|
| Straight beam: tip load, UDL, partial UDL, axial, torsion | Independent, with invented magnitudes | Independent (`physics_audit_regression`: one 2 m cantilever at real section size), plus the p5 tests | Adequate for a single member |
| Inclined member transform | Direction cosines and symmetry only (`validate_transform_fixture`) | Equilibrium-only p5 test | Weak |
| Multi-member frame assembly | Portal frame is repeatability only. Branch case is a 2-DOF spring series | None independent | **Gap** |
| Curved bend, in-plane | Independent: L-bend force-method witness, bending-only, k sweep | Self: compared with the same `CurvedBendMacroElement` crate (`lib.rs:16593`). Anchored thermal only asserts `> 1.0` N (`lib.rs:17000`) | My probe shows it is correct, but there is no suite case |
| Curved bend, out-of-plane | Independent for a distributed load only (CBDFE) | None independent | My probe shows it is correct, but there is no suite case |
| Default bend mode (`mechanics_geometry_only`, straight chord) | None | Only "unchanged" regression tests | **No physical check of the default** |
| Bend k and SIF from geometry (h = tR/r²) | None. k is an opaque user number | None | **Gap** |
| Pressure thrust, straight pipe | tp_phys_008, which encodes the questionable model | Fixed-fixed test asserts compressive stress (`lib.rs:14838-14840`) | **Verifies a wrong model** (VER-02) |
| Pressure thrust at a bend | Arc self-equilibrium witness (CBPT) | Self: compared with the crate | No system-level check |
| Uniform thermal | Fixed-fixed formula check only (no solve); L-bend | Fixed-fixed straight pipe | Thin |
| Temperature-dependent E, α, G | Single-element torsion (DEC092) | Interpolation on a fixed-fixed straight pipe | **No 3-D system case** |
| Deadweight | Load preparation; bend UDL against the crate | Cantilever UDL | No multi-span support-load case |
| Springs and constant-effort hangers | Superposition identity (6 N and 9 N loads) | Spring in parallel (1e6 N/m); constant-effort superposition | No variable-spring hanger design check |
| Imposed anchor or nozzle movement | `validate_imposed_displacement_fixture` multiplies 150 × 0.04 | **Not supported**: `imposed_displacement: None` (`lib.rs:3639`) | **Gap** |
| Gaps, one-way supports, lift-off | Independent; 1 mm members at 100 N/mm | A few tests | Tiny models only |
| Friction | Independent; single axial DOF with the normal force supplied as input | Tests with explicit or derived normal force | **No real-geometry case** |
| Tees and branches | Trivial 2-DOF network | User SIF "review rows" only | **Gap** |
| Code stresses (SL, SE, i·M/Z, sqrt(Sb² + 4St²)) | None. `stress_recovery` has no combined-stress formula | None | **Gap** |
| Stress range | Component-wise absolute difference of invented stresses | Algebraic combination tests | Not a code (vector) range |
| Load combinations | n/a | Signed sum, subtract and envelope algebra | No code-case (sustained, expansion, operating) check |
| Wind and seismic equivalent static | Independent | Independent | Adequate |
| Singularity or mechanism detection | Axis-aligned and tiny-magnitude cases only | ≥ 6 ground-DOF count only | **Broken at real scale** (VER-01) |
| Support reaction components and moments | Component level only | **Not published**; only the force-resultant magnitude | **Cannot be verified** (VER-05) |

---

## Findings

### VER-01: Real-scale mechanisms solve and publish results; the absolute pivot guard is hidden by invented tiny values and axis-aligned fixtures

**Severity: Critical**

**Evidence**

- `core/solver/frame_kernel/src/lib.rs:21` sets `const DENSE_SOLVE_ZERO_PIVOT_GUARD: f64 = 1.0e-12;`. It is used at `:935` as `if pivot_value <= DENSE_SOLVE_ZERO_PIVOT_GUARD`.
- `core/solver/sparse_direct/src/lib.rs:21` sets `SPARSE_SOLVE_ZERO_PIVOT_GUARD = 1.0e-12`. It is used at `:418`. Both are absolute values in N/m and N·m/rad.
- The only stability pre-check in the live path is a count of at least six ground DOFs (`core/product_physics/src/lib.rs:902-922`). Its own comment says "Passing this check never establishes stability: the assembled solve does."
- Probe results:
  - **Live path.** A straight line along (0.6, 0.8, 0) with UX, UY and UZ restrained at three nodes, loaded by a 1000 N·m moment about X. Both sparse and dense modes return `MECHANICS_SOLVED` with rotations of −1.07e12 to −1.61e12 rad and force reactions of 256 and 512 N for a pure-moment load. The sparse and dense answers differ from each other (support-3 gets 0 N in one, 256 N in the other). No blocking diagnostic is raised in either mode; the only diagnostic is the usual rule-check warning. Dense mode publishes an 11 % parity delta, but nothing is gated on it.
  - **Component level** (`solve_dense`), same mechanism:
    - Real-magnitude inclined line: solves, max rotation 1.6e12 rad.
    - Real-magnitude line along X: `SingularSystem`, because the round-off is exactly zero.
    - Invented E = 1200, A = 2, I = 3, inclined: `SingularSystem`.
    - Invented E = 1500, A = 12, I = 25, inclined: solves, 1.0e14 rad.
- Every singular-system and under-restraint test uses axis-aligned geometry or invented magnitudes (for example `under_restrained_model_reports_solver_diagnostic`, `lib.rs:16466`; `frame_kernel/src/lib.rs:1646`).

**Why it matters to a stress engineer.** A forgotten rotational restraint on a skewed line is a very common modelling slip. The program then reports a "solved" model with nonsense displacements and reactions and gives no warning. Commercial tools stop on this with a singularity or zero-pivot message that names the node and DOF.

**Recommended fix**

- Replace the absolute guard with a relative one: pivot ≤ ε·max|K_ii|, or pivot/K_ii (original diagonal) < about 1e-10.
- Always compute the residual ‖K·u − f‖/‖f‖ and global equilibrium (sum of reactions plus applied loads = 0, for forces and moments about a point). Block the result above about 1e-8.
- Name the node and DOF of the failing pivot.
- In the benchmarks, add a mechanism set:
  - an inclined torsion mechanism;
  - a hinge-like chain;
  - a disconnected sub-model.
- Add a scale-invariance test: multiply E by 1e-3 and 1e3, and switch units between SI-mm and US-in. Displacements must scale exactly by 1/λ, forces must be unchanged, and singular cases must stay singular.

### VER-02: Pressure gives wrong stresses and anchor loads, and the "independent" hand calculation encodes the same model

**Severity: Critical**

**Evidence**

- `validation/hand_calcs/mechanics/tp_phys_008_thermal_pressure_axial_effects.md` adds "closed-end pressure thrust" p·A_i to the thermal restraint force E·A·α·ΔT as a same-sign axial eigen-load on a fixed-fixed pipe (3.0 N + 9.0 N = 12.0 N, with E = 1000 Pa and p = 90 Pa).
- The live test `pressure_thrust_applies_axial_fixed_end_correction_without_longitudinal_rows` (`core/product_physics/src/lib.rs:14804-14848`) asserts that the end-i axial stress is `−p·A_i/A`, that is compressive (`:14838-14840`), and that no longitudinal pressure row exists.
- `lib.rs:1754-1756` sets `include_pressure_longitudinal = !pressure_thrust_active`. The stress summary (`open_formula_summary_mpa`, `lib.rs:7632-7656`) then drops σ_L.
- Probe with P = 5 MPa on 8 in. pipe:
  - **Cantilever.** Hoop stress 64.46 MPa is reported. Longitudinal pressure stress is absent and the summary is 0 MPa. Correct σ_L is about 29.8 MPa (P·di²/(D²−di²)) or 33.5 MPa (P·D/4t, the B31.3 sustained basis).
  - **Anchored at both ends.** Each anchor reports 161,413 N, which is exactly p·A_i. Axial stress is reported as −29.78 MPa.
    - Correct wall stress for a fully restrained closed pipe is +ν·σ_h, about +16 MPa tension (ν = 0.25 for the probe material).
    - Correct anchor load is (σ_L − ν·σ_h)·A, about 74 kN, or 0 if pressure elongation is not modelled (the usual commercial default).
  - **Anchored L-bend.** The straight legs report 0.6–1.0 kN axial force and 4.7–6.6 MPa summary stress; the bend reports 113 kN and 34.4 MPa. On a real system the longitudinal pressure stress is uniform in every leg.
- The invented magnitudes (9 N of "thrust" on a 4 m² member with E = 1000 Pa) give no engineering feel. At real scale, a 161 kN anchor load from 5 MPa and a compressive pressure stress would be obvious to any reviewer.

**Why it matters.** Sustained stress SL is under-reported by roughly 30 MPa on every straight pipe that carries pressure. Anchor and nozzle loads in straight anchored runs are overstated about twofold, and the sign of the wall stress is wrong. This is a wrong-answer defect in the most common load in piping.

**Recommended fix (verification side)**

- Rewrite tp_phys_008 against real physics and a real code basis:
  - Pressure end-force is carried by the wall and is not an external load, except at unrestrained expansion joints (thrust = P × effective area).
  - σ_L = P·D/4t (B31.3 §319/§320 sustained basis), or the Lamé/closed-end mean value.
  - Optional pressure elongation (σ_L − ν·σ_h)/E, reported as a user option the way commercial tools treat the Bourdon/pressure-elongation setting.
- Add live-path cases for:
  - a free cantilever;
  - a pipe anchored at both ends;
  - an anchored L-bend;
  - a pipe with an untied bellows between anchors (thrust = P × A_eff on the anchors).
- Each case should check σ_L, anchor forces and the direction of the anchor load. The solver fix belongs to the pressure review area.

### VER-03: Benchmarks exercise component crates, not the live solve path; the only live-path suite is one 2 m cantilever

**Severity: High**

**Evidence**

- `validation/benchmarks/{mechanics,stress,nonlinear}/Cargo.toml` depend on `frame_kernel`, `straight_pipe`, `curved_bend`, `stress_recovery`, `nonlinear_integration` and similar. None of them depends on `open_pipe_stress_product_physics`.
- These component tests bypass the product's own pieces: model parsing, unit normalisation, support-family mapping, bend realisation, thermal and pressure load generation, the sparse solver, recovery and publication.
- Only `validation/benchmarks/physics_audit_regression` calls `run_linear_static_preview_with_mode`, and all 10 of its tests use one two-node straight cantilever (`src/lib.rs:33-43`).
- The live-path curved-bend tests compare against "the same invented arc built directly on the curved-bend crate" (`core/product_physics/src/lib.rs:16593`). That checks assembly against the component, not physics.
- The expansion-loop benchmark (`mechanics/src/lib.rs:3385-3512`) assembles its own global matrix. It drives temperature with f = K·u_free instead of the product's thermal-load generator, and scales the axial area by 1e5 to match a bending-only witness.

**Why it matters.** A stress engineer relies on what the app computes. None of the multi-element, bend, pressure or thermal benchmarks passes through the app's code path. VER-02 is a direct consequence: component checks pass while the live answer is wrong.

**Recommended fix**

- Make every benchmark a live-path model file: a JSON `PreviewModel` in `validation/benchmarks/live/`, loaded through `run_linear_static_preview_with_mode` in both sparse and dense modes.
- Keep component tests as unit tests only.
- The L-bend and out-of-plane probes above can go in as the first two cases. They already match independent force-method values to 8–10 significant figures.

### VER-04: No verification at all for tees, code stresses, vector stress range, imposed movements, real-geometry friction, hangers, or temperature-dependent 3-D systems

**Severity: High**

**Evidence**

- **Tees.** `MECH-BRANCH-ASSEMBLY-THREE-MEMBER` restrains every DOF except UY, which reduces the tee to two springs in series (`mechanics/src/lib.rs:6950-6968`). Its "stiffness" outputs are copied from the expected struct (`:3122-3123`). The live path treats branch SIFs as review rows only (`lib.rs:7771-7780`). There is no branch flexibility and no SIF-intensified stress.
- **Code stresses.** `core/loads/stress_recovery/src/lib.rs` contains no combined-stress formula (no `sqrt`). The stress benchmarks cover F/A, M/Z, T·r/J and P·r/t with invented values: E = 1500 Pa class inputs, area 12 m² (`stress/src/lib.rs:1195`).
- **Stress range.** `STRESS-RANGE-MECHANICS-ORIGINAL` takes the component-wise absolute difference of each stress (`stress/src/lib.rs:609-653`). A code range is taken on the moment vector difference before the SE combination.
- **Imposed displacements.** The live path hard-codes `imposed_displacement: None` (`lib.rs:3639`). The benchmark `validate_imposed_displacement_fixture` (`mechanics/src/lib.rs:4326-4347`) only multiplies 150 × 0.04 and never solves.
- **Friction.** The nonlinear cases are one-DOF, 1 mm, 100 N/mm models with the normal force supplied as input (`hand_calcs/nonlinear/assembled_friction_sliding.md`).
- **Hangers.** Spring-hanger tests check review-row emission only (`lib.rs:12647`, `:12733`).
- **Temperature dependence.** Checked only on a single straight element (`lib.rs:14305-14346`; DEC092).

**Why it matters.** Tees, code stress, expansion range, equipment nozzle movements, friction on long sliding runs and spring-hanger selection drive most real design decisions. None of them has an independent check.

**Recommended fix.** See the prioritised programme below: items P1-3 to P1-9.

### VER-05: Support reactions are published only as a force-resultant magnitude, so anchor loads cannot be benchmarked

**Severity: High**

**Evidence.** In `core/product_physics/src/lib.rs:1440-1503`, only the three translational reaction components are gathered, and only `magnitude = vector[0].hypot(vector[1]).hypot(vector[2])` is published, with kind `reaction_resultant`. No Fx, Fy, Fz, Mx, My or Mz rows exist (the only reaction kind in the file is `reaction_resultant`). My L-bend probe had to be compared on |R| only, which cannot distinguish H_B from V_B errors and says nothing about the anchor moment M_A.

**Why it matters.** Nozzle-load qualification (API 610/617, NEMA SM23, WRC 537 and WRC 297 local stress) needs signed components and moments in global axes. Without them the user cannot check nozzle loads, and the verification suite cannot check the most important output of a flexibility analysis.

**Recommended fix.** Publish signed global reaction components, forces and moments, for every support and every load case and combination. Then add reaction-component assertions to every benchmark. A witness table like the expansion loop's (H_B, V_B, M_B, M_A) is the right shape.

### VER-06: No cross-check against commercial software; the "CAEPIPE" modules use invented formats

**Severity: High**

**Evidence**

- `core/handoff/caepipe_mbf/package.py:1-8` states: "renders a deliberately narrow, invented smoke subset … does not invoke CAEPIPE, parse CAEPIPE output". `render_caepipe_mbf_text` (`:227-284`) writes headed "MBF-style" records (`NODE`, `PIPE`, `SUPPORT`, `LOADCASE`) with `SECTION-TBD` and `MATERIAL-TBD` placeholders, not a real CAEPIPE input.
- `core/handoff/caepipe_external/run.py:1-7` parses an invented CSV (`fixtures/caepipe_external/invented/caepipe_results.csv`).
- There are no CAEPIPE or CAESAR II result sets anywhere under `validation/`. `validation/evidence/comparison_measurement/DEL0904_VD_20260811/OWNER_VALUE_PACKET.md` reports 25 fixtures, of which 11 matched and 14 were blocked, and it compares the program only with itself.

**Why it matters.** Independent agreement with an established program on realistic models is the fastest route to real confidence, and the owner has CAEPIPE.

**Recommended fix.** Run the CAEPIPE comparison programme described in the section on commercial tools below.

### VER-07: Absolute tolerances are unit- and scale-dependent

**Severity: Medium**

**Evidence**

- `mechanics/src/lib.rs:52` and `stress/src/lib.rs:35` set `INTERNAL_ASSERTION_EPSILON = 1.0e-9`, absolute.
- `recorded_comparison_holds` (`mechanics/src/lib.rs:1437-1441`) is the comparison the headless runner and validation manual use for every fixture except four.
- At real magnitudes, 1e-9 absolute is either impossible or meaningless:
  - A bending stress of 55.27 MPa in Pa (M = 15.23 kN·m on 8 in. pipe) has one ulp of 7.45e-9 Pa. M/Z and M·c/I differ by exactly that, so a correct implementation fails the check through round-off alone.
  - At the other extreme, a section property I ≈ 3e-5 m⁴ would pass with a 3e-5 relative error.
- `physics_audit_regression` uses `ROUND6_ALLOWANCE = 0.5e-6 + 1e-10` absolute on values published to 6 decimals (`src/lib.rs:11`). Fixed-decimal publication gives rotations such as 0.000002 rad (one significant figure, from the L-bend probe). A 5e-7 allowance on that value is a 25 % relative tolerance.
- The product records "accepted" absolute nonlinear thresholds that are sized to the invented fixtures, for example 50 mm, 110,000 N and 0.0 N (`core/product_physics/src/lib.rs:71-87`). These are descriptive strings only (`:2835-2843`).

**Why it matters.** The tolerances only work because the fixtures are tiny. They cannot be carried over to realistic cases, and they either hide or falsely flag errors.

**Recommended fix**

- Use |a − e| ≤ rtol·max(|e|, S_q), where S_q is a per-quantity scale for the case: max |reaction|, max |displacement|, or characteristic stress. The expansion-loop and curved-bend fixtures already do this (`mechanics/src/lib.rs:3559-3561`, `:4034`).
- Tiers:
  - rtol 1e-9 for identical-model analytic checks;
  - 1e-6 for independent discretisations;
  - 0.5–1 % against commercial tools with aligned assumptions;
  - 2–5 % where k, SIF or pressure conventions differ.
- Publish results with significant-figure rounding (for example 8 significant figures), not fixed decimals.

### VER-08: Invented tiny magnitudes and axis-aligned geometry hide scale-dependent and physics defects

**Severity: Medium**

**Evidence.** Typical inputs:

- E = 1200 Pa, I = 4 m⁴, P = 6 N (`cantilever_tip_force_fixture`, `mechanics/src/lib.rs:1539-1545`);
- E = 2000 Pa, A = 3 m² (fixed-fixed thermal, `:1843-1846`);
- E = 1000 Pa, p = 90 Pa (tp_phys_008);
- a 1.0 mm member with 100 N/mm stiffness (nonlinear notes);
- section 12 m² and 25 m³ (`stress/src/lib.rs:1195`).

Almost all members lie along global X.

Both Critical findings above are invisible at these scales: VER-01 depends on absolute round-off, and VER-02 produces physically absurd numbers that nobody notices at 9 N. The expansion loop, the curved-bend witnesses and `physics_audit_regression` use real pipe sizes, and those are the cases that carry real information.

**Recommended fix**

- Re-base every benchmark on real sizes: NPS 2, 8 and 24; Sch 40/80; E at 21 °C and at design temperature; α about 12e-6 /K; ΔT 100–300 K; P 1–10 MPa; spans 3–12 m.
- Orient each model off-axis, for example rotated 30° about Z and 20° about X. Run the same model axis-aligned and rotated, and require identical invariants: support-load magnitudes, stresses and the tip-displacement norm.
- Keep one tiny-magnitude twin of each case only to prove scale invariance.

### VER-09: Several cases labelled as verification are tautological or repeatability-only

**Severity: Medium**

**Evidence**

- `fixture_observations` for the cantilever returns the literal `observation("fixed_end_moment_z", 6.0 * 10.0)` rather than a solved value (`mechanics/src/lib.rs:811`).
- The portal frame expected value is `solve_portal_frame_sway()` itself (`:1742-1764`; test `portal_frame_fixture_solves_repeatably`, `:7667`). The validation manual labels this honestly.
- The fixed-fixed thermal case checks only the load-preparation formula E·A·α·ΔT; no system is solved (`:1719-1740`).
- For tp_phys_002/008/009/014 and the stress integrated/004/009/015 fixtures, `expected_values` are produced by calling the solver or recovery function (`mechanics/src/lib.rs:1889`, `:1969`, `:2119`, `:2519`; `stress/src/lib.rs:656-657`, `:705-706`). The crate's own `validate_*` functions do compare against hand values (for example `:4519-4546`). However, the headless runner and validation-manual comparison (`fixture_recorded_comparison_holds`) compare observed values against these self-generated expected values, which is repeatability.
- The evidence packet reports 14 of 25 fixtures blocked and 115 of 206 recorded values unobserved (`validation/evidence/comparison_measurement/DEL0904_VD_20260811/OWNER_VALUE_PACKET.md` §1).

**Why it matters.** The published validation manual overstates independent coverage.

**Recommended fix.** Store expected values as literal numbers taken from the hand-calculation note, never computed by the code under test. Have the runner compare against those. Mark each case in the manual as "independent", "self-consistency" or "repeatability", with the repeatability and self-consistency cases counted separately.

### VER-10: Rust tests and benchmarks are not run by CI

**Severity: Medium**

**Evidence.** The monorepo workflows (`/Users/ryan/dev/chirality-review/.github/workflows/*.yml`) and `tools/ci/e2e_plan.py` contain no `cargo` invocation. The piping workflow runs only `tests/test_ci_e2e_plan.py` and the Playwright plan (`piping-desktop-e2e.yml:46,84,128`). `tests/test_nonlinear_support_regression.py:176-181` would run `cargo test` for one crate, but no workflow runs it.

**Why it matters.** A solver change can break every benchmark without anyone noticing.

**Recommended fix.** Add a CI job that runs `cargo test` for `core/product_physics`, all solver and load crates, and all four benchmark crates, in both solver modes. Fail on any mismatch.

### VER-11: Test effort goes to dormant code and to floating-point edge cases rather than engineering coverage

**Severity: Low**

**Evidence.** `core/product_physics/src/pressure_exact.rs:1-5` calls itself "Dormant exact-annulus pressure mechanics … does not … participate in product-preview assembly, solve, recovery". Nothing outside the file calls it (checked with grep). It still carries 27 tests on subnormal and overflow arithmetic (`:830-1098`).

Of the 83 Python tests in `tests/`, most check schemas, governance and claims language. None asserts a solver number.

**Recommended fix.** Put the effort into the benchmark programme below. Either delete the dormant module or wire it in, and test it with engineering cases.

---

## Suspected, not verified

- The Playwright end-to-end runs may call the Rust solver through Tauri or WASM. I did not trace whether any end-to-end test asserts solver numbers.
- The exact scope of the static load cases in the NRC/BNL reports below is from memory. Confirm it when obtaining the documents.
- I did not test the nonlinear loop at realistic scale. The live path caps active-set iterations at 4 (`lib.rs:66`). That looks too low for systems with dozens of one-way supports, and belongs to the nonlinear review area.

---

## Industry benchmarks to adopt (by name; no tables reproduced)

1. **NRC/BNL piping benchmark problems.**
   - NUREG/CR-1677, "Piping Benchmark Problems" (Bezler et al., BNL): Vol. I, uniform support-motion response spectrum; Vol. II, independent support motion. These are mainly dynamic, but they give complete, publicly documented piping-system models. Use them for deadweight and thermal cross-checks against CAEPIPE now, and for modal checks later.
   - NUREG/CR-6049 (ABWR) and NUREG/CR-6414 (SBWR) benchmark problems. These are standard-design piping systems which, as I recall, include deadweight and thermal expansion cases alongside the dynamic ones.
   - Vendors traditionally document agreement with these problems in their QA manuals.
2. **Classic textbook flexibility problems.**
   - M.W. Kellogg Co., *Design of Piping Systems* (general analytic and elastic-centre methods): the L-bend, Z-bend, U-loop and 3-D two-anchor systems, with and without elbow flexibility.
   - Spielvogel, *Piping Stress Calculations Simplified*: the same planar shapes and quick chart methods.
   - The guided-cantilever approximation, as presented in Peng & Peng, *Pipe Stress Engineering* (ASME Press), which also has worked examples of support loads, friction and spring hangers.
   - Roark's *Formulas for Stress and Strain*: curved beams and rings, for bend-element checks.
3. **Bend and tee flexibility and SIFs.**
   - ASME B31J: SIFs, flexibility factors and the test method.
   - ASME B31.3 Appendix D, in the editions before B31J.
   - von Kármán flexibility (k = 1.65/h) for an elbow-rotation check.
   - Rodabaugh–George pressure stiffening.
   - WRC Bulletin 329 for SIF and k background.
4. **Vendor verification and QA manuals.** CAEPIPE, CAESAR II and AutoPIPE each publish verification or QA documentation with textbook and NRC comparisons. The owner can obtain CAEPIPE's under licence, use its problem statements as test definitions, and compare with their own CAEPIPE runs rather than copying vendor tables.

---

## Recommended test programme, in priority order

**P0: before anyone relies on a result**

1. **Solve-integrity gates on every live solve** (VER-01):
   - relative pivot test;
   - relative residual ‖K·u − f‖/‖f‖;
   - global force and moment equilibrium of applied loads plus reactions;
   - sparse/dense parity as a blocking check in scrutiny mode.

   Add the inclined-torsion mechanism, the disconnected sub-model and the scale-invariance tests (E × 1e±3; SI-mm and US-in).
2. **Pressure correctness set** (VER-02): free cantilever, anchored straight pipe, anchored L-bend, and untied bellows between anchors. Check σ_L = P·D/4t (or the declared basis), σ_h, anchor forces with sign, and the optional pressure-elongation switch.
3. **Publish signed reaction components and moments** (VER-05), and assert them in every case.
4. **CI job** running all Rust suites (VER-10).

**P1: textbook set through the live path, at real magnitudes, off-axis**

1. The L-bend and 3-D out-of-plane L already match my force method to 8–10 figures. Commit them with literal expected values: H, V and M at both anchors, T2 displacement, and the k sweep. Reproduce the same numbers with Kellogg's elastic-centre method as a second reference.
2. Z-bend, symmetric U expansion loop, and a 3-D two-anchor system with two bends. Force-method references, cross-checked with the guided-cantilever approximation and Spielvogel charts to within their stated accuracy.
3. Elbow flexibility: an isolated 90° long-radius elbow under in-plane and out-of-plane moments. Check the rotation against k·M·R·θ/(E·I), with k computed from h per B31J, once the program computes k.
4. Tee: header plus branch under branch-end loads. Check branch flexibility (if modelled) and SIF-intensified stresses i_o·M_o and i_i·M_i per B31J.
5. Code stresses: SL (sustained), SE = sqrt(Sb² + 4St²) (expansion range on the vector moment difference), allowable S_A (B31.3 Eq. 1a/1b), and occasional stresses. Use one straight pipe and one elbow with hand-computed values.
6. Multi-span deadweight: a continuous three- and five-span line on rigid supports. Support loads and midspan and support moments from standard continuous-beam coefficients.
7. Friction: a long straight line anchored at one end and growing thermally over N sliding supports (μ = 0.3, normal loads from deadweight). The anchor load equals μ·ΣN_i, limited by E·A·α·ΔT when the line is fully restrained. Also cover partial sticking near the anchor.
8. Hangers: a variable-spring hanger sized from the hot (operating) load. Check cold load, variability and the effect on neighbouring rigid supports. Also a constant-effort hanger at real load (kN, not 6 N).
9. Temperature-dependent properties in a 3-D system: the U-loop at ΔT = 250 K with E and α from a temperature table. Check hot and cold reactions (B31.3 §319.5 reaction relationships) and the expansion range using the cold modulus for flexibility.
10. Imposed anchor and nozzle displacements, once supported: an L-bend with a thermal nozzle movement at one anchor, checked by superposition.

**P2: public benchmark systems.** Model two or three NRC/BNL problems (NUREG/CR-1677; NUREG/CR-6049/6414 static cases) in both SWBPIPE and CAEPIPE. Compare deadweight and thermal displacements, element forces, support loads and code stresses.

**P3: routine regression.** Freeze all P1 and P2 models as live-path JSON with literal expected values. Run them in CI in both solver modes.

---

## Cross-checking with CAEPIPE (what the owner can do now)

1. **Build the models.** Enter the P1 set and two or three real plant lines in CAEPIPE, ideally lines already analysed for design:
   - a line with 50–200 nodes;
   - 3-D routing;
   - several elbows and tees;
   - guides, limit stops, one-way supports with friction;
   - a variable spring;
   - pressure and two thermal cases.
2. **Align the assumptions explicitly, and record them per model:**
   - code and edition;
   - bend k and SIF source (B31J or code appendix); pressure stiffening on or off;
   - Bourdon and pressure elongation on or off; whether axial force F/A is included in SL;
   - cold or hot modulus for expansion; friction on or off;
   - tee type and branch flexibility;
   - rigid-element weights;
   - how load cases and ranges are combined.
3. **Collect the results.** Export CAEPIPE results (displacements, element forces and moments at both ends of each element, support loads, code stresses) to text or CSV. Write a small parser for the real CAEPIPE output layout. The current `caepipe_external` CSV is an invented format.
4. **Compare automatically**, node by node and element by element:
   - relative error with a per-quantity scale floor;
   - target ≤ 0.5 % on displacements and support loads where assumptions are aligned;
   - ≤ 2–5 % where k, SIF or pressure conventions differ, with every difference explained.
   Keep the CAEPIPE outputs as frozen reference files under `validation/benchmarks/caepipe/`, with a note of the CAEPIPE version and options.
5. **Use the MBF exporter only after it is real.** `render_caepipe_mbf_text` should only be used once it writes a CAEPIPE-readable model with real section and material data. Until then, build the comparison models by hand, or from CAEPIPE's own batch-file format following its documentation.

---

## Done well (keep)

- **The live L-bend (curved-bend macro-element) and the out-of-plane cantilever L are correct.** My independent force-method results matched to 10 and 8 significant figures (13842.208549 N at k = 1; 21.211457 mm tip deflection). The bend element, thermal load generation for bends, and assembly are sound for these cases.
- **The expansion-loop witness is correct.** `validation/hand_calcs/mechanics/expansion_loop_curved_bend_thermal.md` is a real independent force-method derivation. I reproduced H_B = −11636.12 N, V_B = −7635.24 N and M_B = 15230.57 N·m at k = 1 exactly. Its benchmark uses relative tolerances with documented floors and a recorded boost study (`mechanics/src/lib.rs:3160-3198`), which is the right model for all tolerances.
- **`physics_audit_regression` is well built.** It drives the real entry point in both solver modes, uses a real pipe section, checks units on every row, and includes negative controls.
- **The validation manual is honest.** Pages label repeatability-only cases as such (for example `mech-portal-sway-original.md`). Fixture provenance is recorded consistently.
- **Nonlinear hand calculations are explicit.** They give states, iterations, and bounded friction force (for example `assembled_friction_sliding.md`). They are a good base once rebuilt at real scale.
- **Distributed-load consistency is tested.** Wind, seismic and sub-span wind generation are checked against independent lever-rule values in both component and live paths.


---

## Appendix — F. Desktop app workflow, model data, libraries and exports (prefix APP-)

Reviewer focus: can a stress engineer build a real 50 to 200 node process piping model in the desktop app, solve it, and hand it off (report, CAEPIPE, PCF, CSV)?

All paths are relative to `/Users/ryan/dev/chirality-review/projects/chirality-piping/`. Probe programs and models are in
`/private/tmp/claude-501/-Users-ryan/08937a6e-e544-4c91-b97d-34e629979c7f/scratchpad/review/app_probe/`. The probe calls `run_linear_static_preview` from `core/product_physics`, which is the same entry point the Tauri command `run_preview_mechanics_with_solver_mode` uses (`apps/desktop/src-tauri/src/lib.rs:1527-1561`). That command passes the app model straight through with `materials: vec![]`.

## Verdict

As built, the answer is no. Three problems each stop the workflow on their own:

1. **Real data cannot be solved.** The solver refuses any model whose records are not labelled "invented" or "cleared".
2. **Bends created in the app are ignored.** They solve exactly as if no bend were there.
3. **Restraint loads come out only as a force magnitude.** There are no moments and no components.

Beyond those, the CAEPIPE `.mbf` file is not CAEPIPE syntax and carries no engineering data. The report is an audit document, not a stress calculation. There are no pipe-size or material tables. Thermal and pressure loads must be typed per pipe, per load case.

---

## Findings

### APP-1 Provenance substring gate blocks every model built from real project data (Critical)

**Evidence**
- `core/product_physics/src/validation.rs:1320-1334`, `expect_public_preview_provenance`:
  ```
  let normalized = value.to_ascii_lowercase();
  if !(normalized.contains("invented") || normalized.contains("cleared")) {
      diagnostics.push(provenance_diag(entity, id));
  ```
  The diagnostic is `PROVENANCE_INPUT_MISSING` with severity `"blocking"` (1336-1344).
- It applies to every node, pipe, support, component, material, load case, primitive load, equivalent-static block and combination (`validate_provenance`, validation.rs:136-210).
- The app's own defaults fail the gate. Every default it writes is the string `"user_entered_local_preview"`:
  - components: `apps/desktop/src/features/component-creation/componentIntent.ts:106,123`
  - sections, materials and supports: `features/model-tree/PropertyInspector.tsx:2961,2979,3007`
  - load cases, primitive loads and combinations: `features/load-cases/LoadCaseManagerPanel.tsx:1287,1304,1329`
  - Viewport routes leave provenance empty and require the user to type it (`features/viewport/routeDraft.ts:147,187`).
- The app does not warn before solving (`workspaceSession.ts` `handleRun` has no provenance check), so the user finds out only after the run.

**Probe** (the repo's `fixtures/product_preview/invented_preview_model.json`, provenance strings edited):

| Provenance change | Mechanics status | Blocking diagnostics | Result rows |
|---|---|---|---|
| none (fixture as shipped) | `MECHANICS_SOLVED` | 0 | 830 |
| every string set to the app default `user_entered_local_preview` | `MODEL_INCOMPLETE` | 30 | 0 |
| one node set to `Client P&ID rev C, line 6-P-1001` | `MODEL_INCOMPLETE` | 1 | **0** |
| one node set to `not cleared for release` | `MECHANICS_SOLVED` | 0 | 830 |

**Why it matters.** A user cannot solve a model built from their own real data unless they type "invented" or "cleared" into every one of hundreds of records. That means mislabelling real data to get past the gate. A single honest provenance note anywhere removes every result. Because the check is a substring match, "not cleared" passes, so the gate also fails at its own purpose.

**Recommended fix**
- Make provenance a closed enumeration, not free text, for example `project_user_data`, `invented_example`, `cleared_public`, `library:<id>`, plus a separate free-text note.
- Never gate mechanics on provenance. `project_user_data` must always be solvable.
- Apply the public-example rule only when publishing fixtures or examples, as an export check.
- Default new records to `project_user_data`.

### APP-2 Bends created in the app are straight chords: no flexibility, no SIF, identical to having no bend (Critical)

**Evidence**
- The bend payload the app writes has no `mechanics_interface` and no `modifiers`.
  - `buildCreateComponentIntent` (componentIntent.ts:263-271) writes `{ id, label, kind, node, geometry, ...(modifiers), provenance }`.
  - `componentModifiers` (componentIntent.ts:372) does `if (draft.kind !== "expansion_joint") return null;`.
- The Property Inspector cannot set the mode either. It only displays it (`PropertyInspector.tsx:1371-1372`). SIF and flexibility factor are editable (1809, 1818), but the solver mode is not.
- Without the mode, the solver falls back to the chord model.
  - `core/product_physics/src/lib.rs:3756-3769`: `component_solver_consumption(component, "mechanics_geometry_only")`.
  - The curved-bend element is used only for `"curved_bend_macro_element"` (lib.rs:94).
- The only feedback is a *warning*, `BEND_USER_MODIFIER_INPUT_MISSING` (validation.rs:1056 onward).

**Probe.** Model `bend_*.json`:
- 6" pipe (OD 168.3 mm, wall 7.11 mm), anchored at both ends.
- 1 m leg, then a 45° bend with R = 228.6 mm, then a 1 m leg.
- ΔT = 100 °C, E = 200 GPa, α = 1.2e-5 /K.

| Bend as authored | N2 Δx (mm) | Anchor S1 reaction (N) | Max stress (MPa) / location |
|---|---|---|---|
| App-style bend (geometry only, no mode or modifiers) | 0.99965 | 156,158 | 271.9 at P1 |
| No bend component at all | 0.99965 | 156,158 | 271.9 at P1 |
| `curved_bend_macro_element`, k = 2.0, SIF 1.5 | 1.01922 | 140,903 | 257.5 at P3 |

The app-style bend gives results **identical to the digit** to no bend. The anchor load is 11 % too high even in this mild 45° case. For 90° elbows, where k is typically 5 to 15, the error in thermal reactions and stress distribution is much larger. The bend SIF is also absent, so stress at the elbow, which usually governs, is not intensified.

**Why it matters.** Every elbow in a model built in the app is modelled wrongly, with no blocking message. Thermal flexibility and anchor loads are wrong.

**Recommended fix**
- Default every bend or elbow component to the curved-bend element.
- Compute the flexibility characteristic h = T·R/r² and the in-plane and out-of-plane k and i by the code method selected in the rule pack. At minimum, block the solve (not warn) when k or i is absent.
- Remove the chord mode, or make it an explicit expert option labelled "stiff chord, no bend flexibility".

### APP-3 The curved-bend element can hardly be authored: the chord must match R and θ to 1e-6 rad, and bends must be pre-cut by hand (High)

**Evidence**
- `lib.rs:95-97`: `DEC_070_CURVED_BEND_ANGLE_MATCH_TOLERANCE: f64 = 1.0e-6`.
- `lib.rs:3956-3960` compares the user angle with `2·asin(chord/2R)`.
- The bend must be its own pipe span between the tangent points (`bend_pipe_ref`).
- The route form has no bend-at-corner option and no bend radius. Bend angle and radius start empty and are never derived from geometry (`componentIntent.ts:77-78`).
- Coordinates are absolute only. There is no DX/DY/DZ entry in `routeDraft.ts` or `PipeViewport.tsx`.

**Probe** (same 45° model):

| Input | Result |
|---|---|
| Exact coordinates, angle 45 deg | solves |
| Coordinates rounded to 0.001 mm | solves |
| Coordinates rounded to 0.01 mm | **blocked**: `CURVED_BEND_GEOMETRY_INCONSISTENT … 0.785398 rad disagrees with … 0.785386 rad` |
| Angle typed as 0.7854 rad | **blocked** |

At R = 228.6 mm the tolerance corresponds to a chord accuracy of about 0.00025 mm.

**Why it matters.** To model an elbow correctly the engineer has to:
- hand-compute near and far tangent points to better than a micron;
- insert them as nodes;
- create a separate chord pipe for the arc;
- add the component;
- set the solver mode in a JSON file (see APP-2).

No practitioner will do this for 30 to 60 elbows in a model.

**Recommended fix**
- Use the convention of CAEPIPE, CAESAR II and AutoPIPE: route to the tangent-intersection (corner) node and attach a bend radius (long radius 1.5·D by default from the size table, short radius 1.0·D, or user value) at that node.
- The program then computes the near and far points, the included angle and the arc.
- Offer optional intermediate nodes at user angles.
- Derive the angle from geometry and drop the user-angle cross-check, or loosen it to about 1e-3 relative as a warning.

### APP-4 Restraint loads are reported only as a force-resultant magnitude; moments and components are discarded (Critical)

**Evidence**
- `core/product_physics/src/lib.rs:1465-1502`. Only slots below 3 (forces) are copied: `if slot < 3 { vector[slot] = reactions[...] }`.
- The result is `magnitude = vector[0].hypot(vector[1]).hypot(vector[2])`, emitted as `kind: "reaction_resultant"`, `unit: "N"`.
- The fixture solve returns 21 `reaction_resultant` rows and no restraint force-component or moment rows. Tallied from the full 830-row output in `base_out.json`.
- The report and the stress-neutral CSV carry the same magnitude only (`features/report/renderableReportInput.ts:462-471`; `features/stress-neutral/StressNeutralExportPanel.tsx:12-24`).

**Why it matters.** The main hand-offs from a stress analysis are:
- nozzle loads (FX, FY, FZ, MX, MY, MZ) for vendor allowables, for example API 610 and NEMA SM23;
- anchor and support loads by direction for civil and structural design;
- guide and line-stop loads;
- hanger loads.

A signless magnitude without moments serves none of these. The anchor moments, which usually govern nozzle checks, are not output at all.

**Recommended fix**
- Report six signed components per restraint, per load case and per combination, in global axes, plus support-local axes for skewed restraints.
- Report the same for anchors and equipment boundaries.
- For combinations, combine components algebraically (or by the combination's rule for envelopes). Never combine magnitudes.

### APP-5 The CAEPIPE `.mbf` export is not CAEPIPE syntax and carries no engineering content (High)

**Evidence**
- Python renderer: `core/handoff/caepipe_mbf/package.py:227-283`. Desktop renderer: `apps/desktop/src/features/caepipe-mbf/CaepipeMbfExportPanel.tsx:450-475`.
- The file is a header comment block, then the project's own records `UNIT`, `NODE`, `PIPE`, `SUPPORT`, `LOADCASE` and `END`, comma-separated.
- The module's docstring states "It does not invoke CAEPIPE … or claim target compatibility", and the target version is carried as `TBD-17-01-001`.

What is written, from the desktop payload builder (`CaepipeMbfExportPanel.tsx:326-388`):
- **Nodes.** Renumbered `N001…` by *lexicographic sort of internal IDs*, so `node:N-10` comes before `node:N-2`. Absolute X, Y, Z are written.
- **Unit conversion.** Coordinates are converted by `convertLengthToMillimeters`, where `const factor = unit === "m" ? 1000 : 1;` (745-751). Feet or inches are written unscaled but labelled mm.
  - A blocking diagnostic is raised (593-603), but the "MBF text" download link is rendered unconditionally (81-89).
- **Pipes.** `PIPE,P001,N001,N002,section:<pipe id>,<material id>`. The section and material names are never defined anywhere in the file.
  - No OD, wall, corrosion allowance, insulation, contents, E, α, density or allowables are written.
- **Supports.** `supportKind` (696-702) returns `ANCHOR` for all six DOF and `GUIDE` for *any* partial restraint.
  - A +Y resting support (`["UY"]`) is exported as GUIDE.
  - Spring hangers, constant supports, one-way, gap and friction supports (which have empty `restraints`) are exported as `SUPPORT`.
  - No direction, stiffness, gap, friction, or hanger load/rate is written.
- **Load cases.** Only an ID and `loadCase.kind`, which is `primitive_user_load` for app-created cases. No temperatures, pressures or weights.
- **Components.** Bends, tees, valves, flanges, reducers and expansion joints are omitted entirely (the panel's own loss line at 110 says `omitted=components`).

**How far from a usable hand-off.** Essentially the whole distance. Nothing in the file can be imported into CAEPIPE, and even if the syntax were right, the content would be a wireframe with no properties or loads. It would have to be rewritten, not extended.

**What a real CAEPIPE model batch file must contain.** This is described in general terms from knowledge of the format; exact keywords and column order must be taken from the vendor's batch-file documentation. The file is an ordered, sectioned text image of the CAEPIPE model:
1. **Title and options:** job title, unit system and per-quantity units, piping code and edition, vertical axis, ambient (installation) temperature, number of thermal and pressure cases, and analysis switches (hot modulus, pressure stiffening/Bourdon, liberal allowable, friction, and so on).
2. **Layout:** one row per element, "from/to node, DX, DY, DZ" as *incremental* offsets, element type (pipe, bend, valve, reducer, rigid, expansion joint, cold-spring cut), and the current material, section and load names. Properties carry forward down the layout until changed.
3. **Bends:** radius (long, short or user), bend type or flexibility option, optional intermediate nodes or angles, and miter data where relevant.
4. **Branches/tees:** tee or branch type for SIF purposes (welding tee, reinforced or unreinforced fabricated tee, weldolet, sweepolet and so on) with pad and crotch data.
5. **Valves, flanges and rigid elements:** weight, length, flange type, rigidity, insulation factor. **Reducers:** concentric or eccentric, second diameter and thickness.
6. **Materials table:** name, density, Poisson's ratio, E vs temperature, thermal expansion vs temperature, and code allowables (cold/hot, or per code).
7. **Sections table:** name, nominal size and schedule (or OD and thickness), corrosion allowance, mill tolerance, insulation thickness and density, lining.
8. **Loads table:** T1..Tn, P1..Pn, hydrotest pressure, contents specific gravity, additional weight, wind/seismic flags.
9. **Supports:**
   - anchors (rigid or with stiffness), and restraints with direction and stiffness;
   - limit stops and guides with gaps and friction;
   - rod hangers, snubbers;
   - spring hangers (designed or user-defined: manufacturer, hot/cold load, spring rate, number of springs);
   - nozzle flexibilities.
10. **Imposed loads:** anchor/support displacements per thermal case, concentrated forces and moments, wind, seismic g-loads or spectra, and user load combinations.

**Recommended fix**
1. Implement the real format from the vendor's published batch-file specification, organised as the property-carrying layout above.
2. Verify it by round trip: have CAEPIPE export a set of reference models to `.mbf`, re-import the SWBPIPE export, and compare displacements, restraint loads and stresses.
3. Until then, remove the `.mbf` download or label it "not importable".
4. Block every export while a blocking diagnostic exists.

### APP-6 No real import or comparison of CAEPIPE results; the desktop "external run" panel fabricates rows against the user's model (High)

**Evidence**
- `core/handoff/caepipe_external/run.py:206-300` parses an invented CSV with header `section,stable_id,target_id,load_case,ux,uy,uz,axial,shear_y,shear_z,unit` (sections `NODE_DISPLACEMENTS` and `ELEMENT_FORCES` only). No CAEPIPE output file is read.
- `apps/desktop/src/features/caepipe-external/CaepipeExternalHarnessPanel.tsx:298-319` (`parserRows`) builds rows with hard-coded values, `{ ux: 0.001, uy: 0, uz: 0 }` and `{ axial: 12.5, shear_y: 0.1, shear_z: 0.2 }`, keyed to the *user's* first two node IDs and first pipe ID.
- `core/comparison/analysis_run/engine.py:1-6` states it does not ingest external solver files.
- `services/previewService.ts:272-330` compares two bases within one run, hard-coded to the fixture IDs `load:L-100` and `combination:C-OPER-ALT`.

**Why it matters**
- Benchmarking against CAEPIPE or CAESAR II is the first thing an engineer does before trusting a new solver, and there is no route for it.
- Showing invented "external" numbers next to real node IDs can be misread as comparison evidence.

**Recommended fix**
1. Delete the fabricated rows.
2. Implement a parser for CAEPIPE's results export (text or CSV) and for the CAESAR II output report or neutral file.
3. Map node numbers through the sidecar ID map.
4. Compare per load case: displacements (6 DOF), restraint loads (6 components), and code stress with ratio, with stated absolute and relative tolerances.

### APP-7 Libraries cannot populate the model; there are no pipe schedule, fitting or material property tables (High)

**Evidence**
- Material, section and component libraries are validated, stored and indexed only (`apps/desktop/src/services/libraryImportService.ts:68-80,171-219,284-391`).
  - The Library Manager's "Apply … to draft" buttons edit the *library document*, not the model (`features/library/LibraryManagerPanel.tsx`, `applySectionQuantityDraft` around 1126-1197).
- The only library-to-model paths are:
  - hanger records → support hanger fields (`features/hanger-selection/hangerSelection.ts:40-46`);
  - material, section and component library slots → *rule-check inputs only* (`apps/desktop/src-tauri/src/lib.rs:2859-2969`, `resolve_library_value_bindings`).
- Nothing writes library values into `pipe_segments[].section` or `materials[]`. Model sections and materials are typed in by hand (`PropertyInspector.tsx` `buildCreateSectionIntent` ~2523, `buildCreateMaterialIntent` ~2595). Material temperature points are typed row by row (`features/material-temperature/MaterialTemperatureForm.tsx`).
- **No NPS/schedule table anywhere.**
  - Searched for B36, NPS, schedule, sch40, STD, XS, nominal and long_radius across `core`, `apps`, `schemas`, `fixtures` and `examples`.
  - The only hits are a deny-list in `core/reporting/protected_content_linter/src/lib.rs:489-510` that flags the tokens `"b36.10"`, `"b36.19"` and `"b16."` as suspected protected content.
  - The PCF export writes `nominal_size: "TBD_SOURCE_REQUIRED"` (`features/pcf-export/PcfExportPanel.tsx:34,369`).
- **Material data bundled:** one invented material (E 200 GPa, G 77 GPa, α 1.2e-5, no temperature table) in `fixtures/product_preview/invented_preview_model.json`.
- **Weights:** valve and flange weights are typed per component. The self-weight generator covers pipe mass only: "Components, supports and equipment are excluded." (`features/self-weight-authoring/SelfWeightPlanPanel.tsx:80`).

**Why it matters.** Every pipe's OD and wall, every material's E(T) and α(T), and every fitting's weight must be typed by hand. That is slow and the main source of input error in stress models.

On protection: ASME B36.10M/B36.19M dimensions (NPS/DN → OD and schedule wall) are dimensional facts, reproduced in every pipe manufacturer's and distributor's catalogue. They are not comparable to code allowable tables. The linter treating "B36.10" as protected content also flags ordinary references such as "pipe to ASME B36.10M" in a user's report.

**Minimum a practitioner needs**
1. A pipe size table (NPS/DN → OD; schedule or STD/XS/XXS → wall) selectable per section, including insulation and contents fields.
2. A material library with density, E vs T, and mean thermal expansion from installation temperature vs T. It can be user-imported, with an "assign to pipes" action; the allowables can stay user or rule-pack supplied.
3. Fitting data: elbow radius by type (LR 1.5D, SR 1.0D), and tee/branch type for SIF selection.
4. A user valve/flange weight-and-length library applied to rigid elements.
5. A spring hanger catalogue or user table, which already exists.

**Recommended fix**
- Ship an open pipe-size table from public dimensional data (or seed it from a manufacturer datasheet with attribution).
- Add "apply library record to selected pipes/components" operations for section, material and component records.
- Remove the B36/B16 tokens from the protected-content linter, or narrow them to reproduced allowable tables only.

### APP-8 Temperature and pressure are per-pipe primitive loads per load case, entered as ΔT; there are no operating conditions per line and no temperature-dependent E or α per element (High)

**Evidence**
- `features/load-cases/LoadCaseManagerPanel.tsx:2513-2514`: `primitiveLoadUsesPipeTarget` returns true for `"distributed_force" || "pressure" || "thermal"`. Each primitive targets one pipe (`{ type: "element", pipe: draft.targetPipe }`, 1614-1616).
- Thermal is a `temperature_interval` magnitude that still carries a direction. The fixture uses `"magnitude": 12.5 degC`, `"direction": "global_z"`.
- New primitives default to `magnitude: "250"` (1302).
- E, G and α are chosen per *load case*, not per element (`PreviewLoadCase.modulus_basis_ref` / `modulus_basis_temperature`, `lib.rs:418-445`).
- There is no installation temperature and no operating temperature per element.
- There is no copy of a load to other pipes or other cases (see APP-11).

**Why it matters**
- A 100-pipe model with T1, T2, P1 and a hydrotest needs about 400 separate primitive entries.
- OPE and SUS each need their own duplicated primitives, because EXP must be built by subtracting two *load cases* (UI restriction, `LoadCaseManagerPanel.tsx:1404-1408`).
- A system with hot and cold branches of the same material gets one E and one α for the whole case, which misstates thermal expansion and hot-modulus reactions.

**Recommended fix**
- Adopt the CAEPIPE/CAESAR property-carrying model: per-element operating sets T1..Tn, P1..Pn, hydrotest pressure and fluid density, inherited down the run until changed.
- Use a global installation temperature.
- Look up α(T) and E(T) per element from the material table.
- Generate the thermal strain and pressure loads internally for each operating case.

### APP-9 No code load cases; EXP, SUS and OPE must be hand-assembled (High)

**Evidence**
- The load case "Kind" is free text (default `primitive_user_load`, `LoadCaseManagerPanel.tsx:1285`).
- There are no W, P, T, OPE, SUS, OCC or EXP templates (searched SUS, OPE, EXP, sustained, operating, template).
- Combination bases are `mechanics`, `result_state_subtraction` and `range_envelope`. Subtraction operands must be load cases (1404-1408).

**Why it matters**
- B31.1 and B31.3 flexibility analysis always needs the same set: W+P (SUS), W+T+P (OPE), OPE−SUS range (EXP), with hot and cold modulus as the code requires, plus OCC.
- Building these by hand for each model invites errors, for example EXP computed with the wrong modulus or with friction left in.

**Recommended fix.** Add a "recommended load cases" generator driven by the operating sets in APP-8:
- W+T1+P1 (OPE), W+P1 (SUS), OPE−SUS (EXP, as an algebraic range), and occasional cases combined per the code's rule;
- user-editable afterwards;
- nonlinear supports taken into account through the operating state.

### APP-10 Unit system: a blank project is fixed SI metres, the app's default thermal unit is rejected by the solver, and common engineering units are missing (Medium)

**Evidence**
- A blank project sets `length: "m", force: "N", moment: "N*m", pressure: "Pa", stress: "Pa", temperature: "C"` (`apps/desktop/src/services/projectService.ts:468-475`). No UI changes project units.
- The thermal primitive defaults to the project temperature unit (`LoadCaseManagerPanel.tsx:2474-2486`), which is `"C"`.
  - The solver catalog has `degC`, not `C`.
  - Probe `thermal_C.json`: `UNIT_INPUT_INVALID … got C: C is not in the catalog for temperature_interval`, solve blocked.
- Units accepted by the solver are listed in `core/units/src/lib.rs:390-600`. Probe results:

| Unit | Result |
|---|---|
| `bar` | blocked |
| `kN` | blocked |
| `1/degF` | blocked |
| `in/in/degF` | blocked |
| `N/mm` spring rate | blocked |
| `kN/mm` spring rate | blocked |
| `lbf/in` spring rate | accepted and converted correctly (hanger reaction changed from 160.1 N to 446.8 N as expected) |

  kN·m and GPa are not in the catalogue either.
- Display units are a selector: `Entered`, `SI`, `US` (`features/display-units/index.tsx:58-64`), with "Editing uses entered units." Solver results are fixed in mm, MPa and N.

**Why it matters**
- US-customary users cannot enter α in the unit every material table uses.
- Metric users cannot enter bar, kN, or spring rates in N/mm.
- A new project's default thermal unit fails the solve.

**Recommended fix**
- Add kN, kN·m, bar (gauge noted), GPa, N/mm, kN/mm, lbf·in/deg and 1/degF (in/in/°F) to the catalogue.
- Change the blank-project temperature unit to `degC`.
- Offer project unit systems (SI-mm, SI-m, US) used for both input and output, including reports and exports.

### APP-11 Model-building throughput is far too low for a 50 to 200 node model in a day (High)

**Evidence**
- **Routing.**
  - Coordinates are absolute only. There are no DX/DY/DZ offsets.
  - Node ID, label, X, Y, Z, unit and provenance are retyped for every new end (`PipeViewport.tsx:1590-1596`, `setNewEndDraft(emptyNodeDraft(...))`). The pipe ID is also retyped.
  - Each segment needs Add, then Apply.
  - Any *warning* stops a route (`routeDraft.ts:205`: `hasWarningOrBlockingDiagnostic(...)) return false`).
- **Copy/mirror** exist in geometry tools, but "Transforms block attached supports, components, loads, wind and unknown references." (`features/geometry-tools/GeometryToolsPanel.tsx:56`). Every copied node and pipe needs a hand-typed ID and provenance.
- **Multi-select editing** is disabled: "Property mutation is unavailable for multi-selection." (`PropertyInspector.tsx:402,432`). There is no bulk edit.
- **Table view** (`features/model-tree/ModelTree.tsx:1205-1232`):
  - The node grid has label, X, Y, Z and provenance.
  - The pipe grid does not include OD, wall, density or insulation.
  - There is no add-row and no paste from a spreadsheet.
- **Pipe supplements** (densities, insulation, contents) are edited one property per queued task.
- **No line numbers, line classes, piping specs or groups** (searched line_number, lineNumber, line class, spec, group).
- **No geometry import:** no PCF, neutral-file or CSV import command in `src-tauri/src/lib.rs`.
- **Required free-text references on every component:**
  - tee reinforcement reference;
  - valve/flange `connectionEndAReference`, `connectionEndBReference`, `stiffnessBehaviorReference`;
  - bend plane orientation as free text (`componentIntent.ts:186-241`).
- **Supports** use global DOFs only. There are no skewed restraints or direction cosines (`SupportConfigurationForm.tsx:5`), no hanger design (sizing from load and travel), and the family name does not preset restraints ("Family names do not supply restraints or engineering values.", `SupportConfigurationForm.tsx:172`).

**Why it matters.** A practitioner building from an isometric enters incremental offsets down a line, with properties inherited, tees and bends at corners, and supports picked by type. Here each of those steps is manual and multi-click, and the loads multiply per pipe (APP-8). A 100-node model is days of work with a high error rate.

**Recommended fix**
- Incremental DX/DY/DZ routing with auto-numbered nodes (step 10), auto-generated pipe IDs, and inherited section, material, operating set and provenance.
- An "insert bend at corner" option.
- Support types that preset restraints (+Y rest, guide, line stop, limit stop with gaps, anchor, spring).
- Editable spreadsheet grids with paste.
- Multi-select property assignment.
- Line number as a first-class attribute used for grouping, selection and reports.
- A PCF import for geometry.

### APP-12 The calculation report is not issuable: no input echo, no code stress vs allowable, no restraint-load or hanger tables (High)

**Evidence** (`apps/desktop/src/features/report/renderableReportInput.ts`)
- The load case summary has only ID, label and `basis: loadCase.kind` (396-404). There are no magnitudes, temperatures, pressures or combinations.
- `rule_pack_refs: []` is hard-coded (431), so rule-check results never reach the report.
- The results section is one flat dump of every result row (462-471):
  - the label is `` `${item.kind} (${item.metadata.component})` `` and the value is the raw `` `${item.value} ${item.unit}` ``;
  - there is no node or element column, no case grouping, no sorting and no maximum.
- The "Model Input Summary" lists references and hashes, not coordinates or properties.
- Hangers appear only as a single "user-supplied value" row (227-259).
- Report sections are fixed at eight audit-oriented kinds (`core/reporting/report_generator/src/lib.rs:12-21`).

**Why it matters.** An issuable stress report needs:
1. an input echo: node coordinates, element properties, materials and their temperature data, operating conditions, supports with stiffnesses and gaps;
2. the load case and combination definitions;
3. a code stress table per case (SUS, OCC, EXP) with node, stress, allowable and ratio, sorted with the maximum ratio highlighted;
4. displacements per node per case (6 DOF);
5. restraint and nozzle loads per case (6 components, see APP-4);
6. a hanger table: cold and hot load, travel, spring rate, size;
7. a summary of governing ratios and nozzle checks.

Only item 4 appears, and then only as an unsorted list.

**Recommended fix.** Generate the report from structured tables in the order above, formatted in the project unit system, with the input echo taken from the solved model hash.

### APP-13 PCF export writes only straight PIPE records with a non-numeric bore (Medium)

**Evidence**
- `features/pcf-export/PcfExportPanel.tsx:493-523` and `core/handoff/pcf_export/package.py:230-274` write only `PIPE` blocks with two `END-POINT`s.
- The END-POINT bore field is `TBD_SOURCE_REQUIRED` (369, 511-512). That raises a blocking `PCF-NOMINAL-SIZE-TBD` diagnostic (539), yet the download is still offered.
- No ELBOW/BEND (with CENTRE-POINT), TEE, REDUCER, FLANGE, VALVE or SUPPORT records are written. Supports are listed as omitted.
- Non-standard `OUTSIDE-DIAMETER` and `WALL-THICKNESS` attributes are placed inside PIPE blocks.

**Why it matters.** A PCF with no fittings and a text bore cannot be read by ISOGEN or by stress-program PCF importers. It is only a wireframe.

**Recommended fix**
- Write numeric bore from the size table (APP-7).
- Emit ELBOW with CENTRE-POINT, TEE with CENTRE-POINT and BRANCH1-POINT, REDUCER, FLANGE, VALVE (with weight via item codes), and SUPPORT with co-ordinates.
- Block the download while blocking diagnostics exist.

### APP-14 Native JSON export is a review manifest, not the model; results CSV is a long-format scalar dump (Medium)

**Evidence**
- `features/native-package/NativePackagePanel.tsx:417-419`: `implementation_status: "browser_panel_review_packet_not_native_writer"`. The packet lists `model/project.json` as a member but does not contain nodes, pipes, supports or loads.
- Projects open only from the local store (`open_local_project`). There is no file import.
- The stress-neutral CSV (`features/stress-neutral/StressNeutralExportPanel.tsx:12-24`) has columns `result_id,canonical_ref,row_kind,result_family,load_case_ref,station_ref,component_ref,value,unit,dimension,correlation_status`, one row per scalar. There are no code stress, allowable or ratio columns, and reactions are magnitude only (APP-4).

**Why it matters**
- There is no portable model file to send to a checker or archive with the calculation.
- The CSV needs pivoting before it resembles any usual stress table.

**Recommended fix**
- Make the native export the full model JSON (schema-validated and re-importable).
- Add wide-format CSVs: displacements (node × 6 DOF per case), restraint loads (support × 6 per case), and code stresses (node, case, stress, allowable, ratio).

### APP-15 Minor issues (Low)

- `buildExplicitNodeIntent` (`PipeViewport.tsx:3498-3515`) is dead code. It is the only node path that sets a default provenance.
- MBF node renumbering sorts IDs as strings (`localeCompare`), so node order in the export does not follow the route (`CaepipeMbfExportPanel.tsx:329-331`).
- Undo is capped at 25 steps and lost when the project is reopened (`workspaceSession.ts:1032`; App.tsx:433).

---

## Suspected, not verified

- Editing a bend's SIF or flexibility factor in the Inspector sets `modifiers.sif_user_value.value` and `modifiers.flexibility_factor_user_value.value` (`PropertyInspector.tsx:1809,1818`). For a bend created by the app, which has no `modifiers` object (APP-2), this field-path edit may be rejected by the operation applier. Not run.
- The operation applier may reject the unit `"C"` at queue time, before the solve. The solver certainly does (APP-10). Not traced through `core/model_operations`.
- Combination reaction rows: in the fixture, `C-OPER-ALT` gives 200.14 N for SH-140, against case magnitudes of 160.11 N and 80.06 N. That suggests combinations are formed from vectors internally, but the output is still a magnitude. Not traced further; this belongs to the solver reviewers.

## Done well

- **No silent defaults.** Missing or incompatible inputs produce clear blocking diagnostics that name the record, for example `UNIT_INPUT_INVALID … got bar` and `CURVED_BEND_GEOMETRY_INCONSISTENT` with both angles printed. That is the right instinct; it only needs to be aimed at engineering gaps rather than provenance wording.
- **Unit conversion that is present is correct and explicit.** `lbf/in` spring rates, `mm` and `in` coordinates, `psi` and `degF` intervals convert through one catalogue.
- **The curved-bend element works when fed.** With exact geometry it gives the expected drop in anchor load and moves peak stress, so the physics is ready for proper UI authoring.
- **Rich raw result set.** Nodal displacements and rotations in 6 DOF, element-local forces, moments and stresses at stations, and hoop stress are all there to build proper tables from.
- **Hanger library → support path works.** Imported hanger records populate support hanger fields through a reviewed operation (`hangerSelection.ts`).
- **Every model edit is a validated, undoable operation.** Edits have an audit trail and model hashes. This is a good base for bulk edit and import once the per-field friction is removed.
- **Loss reports are honest.** The MBF and PCF packages record what they omit. The problem is that the download is still offered when those losses make the file useless.
- **Useful building blocks exist.** Self-weight generation from density, gravity and axis; mirror, translate and rotate copy; and a SI/US display toggle are all worth keeping and extending.
