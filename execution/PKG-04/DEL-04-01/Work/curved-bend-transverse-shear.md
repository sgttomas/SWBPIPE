# Curved-bend transverse shear (candidate design)

This is the RT-K1/RT-A1 candidate for transverse shear deformation in the
realized curved-bend macro-element. It covers the formulation, the analytic
results and the minimal implementation boundary. Nothing here is implemented
or accepted. The independent reference is
[`validation/references/curved_bend_shear/`](../../../../validation/references/curved_bend_shear/README.md).

## 1. Identities

- **Absent and `euler_bernoulli`.** These stay the default. Their meaning,
  bytes and results do not change.
- **`timoshenko_annulus_energy_v1`.** Its persisted meaning does not change:
  straight members are Timoshenko with κ_E and realized arcs are
  Euler–Bernoulli. Its bytes, results, diagnostics (including
  `PF_TIMOSHENKO_REALIZED_ARCS_EULER_BERNOULLI`) and semantic identities do not
  change either.
- **New `timoshenko_annulus_energy_curved_v2`.** Straight members are formed
  exactly as under v1, bit for bit. Each realized arc adds the shear term of
  §3, with A_s = κ_E·A formed by the same per-route rule as that route's
  straight members. No fitted constant, alpha_b2, finite-anchor 1e12, vendor
  value or data enters.

## 2. Source and translation

**Formulation.** EDF Code_Aster R3.08.01, *Éléments "exacts" de poutres
(droites et courbes)*, revision 13337, 9 July 2015, §8 "Poutre courbe",
pp. 45–52, GNU FDL. It was read from
<https://ericca.uqtr.ca/fr12.4/man_r/r3/r3.08.01.pdf> (SHA-256
`57a9fa3f…ebc7`; full hash in the reference README). The pages were inspected
as rendered images, not only as extracted text.

- p. 45: a constant-radius, constant-section arc with transverse shear taken
  into account ("avec prise en compte du cisaillement transverse"). It has no
  distributed load. The stiffness follows from flexibility by Castigliano.
- p. 47: J(θ) transports the end actions at P to the section by equilibrium
  ("Figure 8-d" axes). The energy is
  ½∫(N²/ES + V₁²/(k₁SG) + V₂²/(k₂SG) + M_T²/· + M₁²/EI₁ + M₂²/EI₂) ds.
- pp. 48–49: C = ∫₀^β J_θᵀ diag(1/ES, 1/(k₁SG), 1/(k₂SG), 1/GC, 1/EI₁, 1/EI₂)
  J_θ R dθ.
- p. 49: J splits into an in-plane block (N, V₁, M₂ from F_x, F_y, M_z) and an
  out-of-plane block (M_T, M₁, V₂ from M_x, M_y, F_z).
- p. 52: for pipe bends, the user flexibility coefficients divide I_y and I_z
  only.

**Shear coefficient.** EDF R3.08.03, revision 10726, 25 March 2013, §2.1.3,
p. 10, eqs. [4]–[5] (SHA-256 `9fb4e49b…a147f5`). The coefficient is defined by
shear energy: U = ½V²/(kSG), so k = V²/(S∫τ²dS). SWBPIPE's adopted κ_E
(`frame_kernel::shear`, DEL-05-03 `section-bases-and-timoshenko.md` §3.1) is
the Saint-Venant shear energy of the homogeneous isotropic annulus. It is a
coefficient of exactly this energy kind, and it is inherited unchanged.
R3.08.03's own circular, Cowper and Jourawski values (§§2.1.1–2.2) are not
used.

**Printed typographical errors.** These are recorded so that nobody adopts
them. None affects the integral definition, which is the only thing adopted.

- p. 47 prints the torsion energy over EI; pp. 48–49 print 1/GC.
- p. 50's annex prints ∫₀^β sin²θ dθ = ½(β + sin β cos β). Its own second
  form, ¼(2β − sin 2β), is the correct one.
- p. 51 prints C¹₁₂'s shear term as R³ sin²β/(2k₁SG). The printed integrand
  gives R sin²β/(2k₁SG).
- p. 51 prints C²₃₃'s bending term as −R³/(2EI)[6β − 8 sin β + sin 2β] and its
  shear term as "Rb/(k₂SG)". The integrand gives +R³/(4EI)(…) and Rβ/(k₂SG).

The other printed closed forms are not used.

**Axis map to the product.** The product's local frame
(`curved_bend::ArcGeometry`) is x radial outward at node i, y tangent at node i
toward j, and z = x × y. The source's Figure 8-d puts x_s along the tangent at
P, y_s along the outward radius at P, and θ from y_s toward x_s, so
z_s = −z. Substituting F_xs = F_y, F_ys = F_x, F_zs = −F_z into J_θ gives:

- N = −F_x sin θ + F_y cos θ. This is the product's existing axial row
  ("N = −sin θ, cos θ, 0"), which independently confirms the map.
- V₁ = F_x cos θ + F_y sin θ = V_r, the in-plane radial shear.
- V₂ = −F_z. Its sign drops out of the quadratic energy.

So k₁ is the in-plane shear coefficient and k₂ the out-of-plane one. For the
axisymmetric annulus, k₁S = k₂S = A_s.

## 3. Element mechanics

### 3.1 End flexibility

The flexibility is node j's with node i clamped, in the local frame, with
S = sin φ and C = cos φ. Under a unit tip load, the section at θ carries the
tip force unchanged, so V_r = F_x cos θ + F_y sin θ and V_z = F_z. Moments
produce no shear. Then

ΔF = R/(GA_s) ∫₀^φ (V_r,a V_r,b + V_z,a V_z,b) dθ, which gives

- ΔF₀₀ = R(φ + SC)/(2GA_s)
- ΔF₀₁ = RS²/(2GA_s)
- ΔF₁₁ = R(φ − SC)/(2GA_s)
- ΔF₂₂ = Rφ/(GA_s)

and every other entry, including all rotational rows, is zero. The v2
flexibility is F_EB(k_in, k_out) + ΔF. For a quarter arc the translational
block is (R/GA_s)[[π/4, ½, 0], [½, π/4, 0], [0, 0, π/2]]. The oracle confirms
this; case `Q1` is exactly π/2.

### 3.2 Uniform load

The intensity w is constant in the local frame and given per arc length. The
tip-side segment of the free (node-i clamped) arc transmits
F(θ) = R(φ − θ)w. By the unit-load theorem with ψ = φ − θ, the shear part of
the free-tip deflection is

δ_shear = (R²/GA_s) [[(φ² + S²)/4, (φ − SC)/4, 0],
[(φ − SC)/4, (φ² − S²)/4, 0], [0, 0, φ²/2]] · w,

and its rotations are zero. The three in-plane integrals are already in the
kernel's `LoadIntegrals`: `acc` = ∫ψ cos²θ = (φ² + S²)/4,
`asc` = ∫ψ sin θ cos θ = (φ − SC)/4 and `ass` = ∫ψ sin²θ = (φ² − S²)/4. The
oracle derives them independently (by nested quadrature and closed form) and
checks them by Betti reciprocity, through double quadrature over interior
displacement fields. The consistent vector is formed exactly as now: X = −K_g δ₀,
p_j = −X and p_i = H X + W. Its stiffness, load vector, load certificate and
re-formation therefore come from one energy.

### 3.3 Properties

- **Straight limit.** At a fixed arc length L = Rφ, ΔF → diag(L, 0, L)/GA_s
  and δ_shear,xx = δ_shear,zz → wL²/(2GA_s). These are the straight Timoshenko
  cantilever values. The relative deviation is −φ²/3 for ΔF_xx and −φ²/6 for
  δ_xx.
- **Separation from k.** k_in and k_out scale only the bending terms, as in
  the source. The shear block is the same for any k (oracle `Q1K`).
- **No double counting.** N uses the tangent component of the section force.
  V_r and V_z use the radial and binormal components. At every section,
  N² + V_r² + V_z² = |F|² (checked at every quadrature node), so each force
  component enters once and the torsion and bending terms are untouched.
- **Pressure and thermal loads.** `FreeStrainCurved` and
  `PressureFreeStrainCurved` are K·u_free with u_free = ε·d. That is a
  stress-free homothety of the arc, so it involves no constitutive term. Under
  v2 they read the v2 K through `CurvedFormation` and stay self-equilibrated.
  No shear term enters pressure.
- **κ_E is a straight-annulus approximation.** The energy coefficient is the
  straight prismatic Saint-Venant value. The curved section's shear stress
  distribution differs at order r/R. This is the source's model too (one k
  per section). It is the engineering approximation adopted here (AGENTS.md
  engineering-judgment delegation), and no claim of curvature-corrected shear
  energy is made.

### 3.4 Recovery and stress

`arc_section_resultants` and the arc recovery are equilibrium only. They take
the node-j force K d − p and the intensity, so they are unchanged, and they
recover tangent-frame actions consistently with the v2 K and p. The
O-13 recovery certificate shares `arc_flexibility` with the load certificate,
so it encloses the v2 element once that function carries shear. The stress
field stays the existing straight-annulus recovery. No curvature-corrected
stress is claimed.

## 4. Stable evaluation

- **Below the series switch** (s < ½), every quantity is either an existing
  stable form or free of cancellation. The existing forms are φ − SC
  (`phi_minus_sin_cos`) and the series `LOAD_ACC`, `LOAD_ASC` and `LOAD_ASS`,
  so φ² − S² needs no new series. The cancellation-free ones are φ + SC, S²,
  φ and φ².
- **Above the switch,** φ² − S² ≥ (π/3)² − ¾ > 0.34, so the Gram closed forms
  do not cancel.
- **Expected behaviour.** The oracle's sweep crosses s = ½ on both sides and
  runs from φ ≈ 2e-8 to π − 6.3e-4. Its values are exact to more than 30
  digits, so a kernel test can measure relative error directly. Binary64
  agreement should be a few ulp of the largest entry. The consumer test sets
  the tolerance.

## 5. Minimal implementation boundary

This boundary was audited at `5f2946ef12`. Step A, the kernel and
certification, is the next authorized slice. Step B, product admission, is
reserved until Agent 0 reviews its actual diff.

### Step A: kernel and certification (DEL-04-01; FK structural)

1. **`core/solver/curved_bend/src/lib.rs`.**
   - Add `transverse_shear_area: Option<f64>` to `CurvedBendMacroElement`.
     `new` keeps its signature and sets `None`. A new
     `with_transverse_shear_area(self, a_s)` validates a positive finite A_s and
     returns `Self`.
   - The only struct literal is `new`'s `Self { … }`. Every other
     construction site, in tests, PP, NI and the benchmarks, calls `new`.
   - `flexibility_of`: only when the value is `Some`, append the shear term
     inside the existing bracket, last. That is f₀₀ += cos_sq/GA_s,
     f₀₁ += sin_cos/GA_s, f₁₁ += sin_sq/GA_s and f₂₂ += φ/GA_s, each inside the
     `r * (…)`, with GA_s = G·A_s. `None` executes the current expressions
     unchanged.
   - `tip_deflection_by_series`: add a shear family per row: row 0
     r(w_x·acc + w_y·asc), row 1 r(w_x·asc + w_y·ass), row 2 r·w_z·φ²/2, rows
     3–5 zero. It is divided by GA_s and appended last.
   - `tip_deflection_by_gram`: add **two** families, never one.
     - V_r: unit Fx = [0, 1, 0], Fy = [0, 0, 1]; load
       [0, Rφw_x, Rφw_y, 0, −Rw_x, −Rw_y].
     - V_z: unit Fz = [1, 0, 0]; load [Rφw_z, 0, 0, −Rw_z, 0, 0].
     - One combined trig vector would add a false F_x–F_z coupling of
       ∫cos θ dθ = S.
   - The stiffness, load vector, H, recovery and arc frame need no other
     change.
2. **`frame_kernel/src/structural/formation_check.rs`.**
   - `CurvedFormation` gains `shear_area: Option<f64>`.
   - `curved_binding` appends `[TIMOSHENKO_BINDING_TAG, A_s bits]` only when
     the value is `Some`. The tag is the NaN pattern already used by
     `frame_binding`, prefix-free by the same argument, so `None` bindings are
     unchanged.
   - `CurvedLeaves` gains `shear_area() -> Result<Option<Wide2>, _>`.
   - `curved_matrix_with`'s `unit_load_actions` becomes six families: the
     four existing ones plus V_r [0,1,0]/[0,0,1] for rows 0–1 and V_z [1,0,0]
     for row 2.
   - The shear quads are added after the axial term only when the value is
     `Some`.
3. **`arc_certificate.rs`.**
   - `Actions` and the cases gain the two shear families.
   - `Rigidity` gains `gas: Option<Ball>`, a lifted G·A_s. `energy` takes six
     quads and adds q_r/GA_s + q_z/GA_s (division site "GAs") only when the
     value is `Some`.
   - `curved_uniform_load_balls` adds the two extended-basis load rows of item
     1.
   - Both certificates use `arc_flexibility`, so they stay one element.
4. **`element_reformation.rs`.**
   - `BendOperands` and `LiftedCurved` gain an optional shear area.
   - `reform_bend`'s geometry carrier sets `shear_area: None`.
   - The one production `BendOperands` site, `modal_eigen` `reformation/mod.rs`,
     passes `None`. Modal analysis refuses v2 (Step B), so its arcs are always
     Euler–Bernoulli.
5. **`nonlinear_integration/src/structural_adapter.rs`.** The two
   `CurvedFormation` sites copy `shear_area: m.transverse_shear_area`. At the
   force-scaled site, E and G are scaled by 2^b and A_s is copied unscaled, as
   `area` is. `curved_formation` (the symmetry trace) reads the element's own
   methods and needs no formula.
6. **Mechanical `CurvedFormation` literals.** These gain `shear_area: None`:
   PP `lib.rs` `curved_formation_of` (`Some` only from Step B) and
   `certified_publication.rs`, plus FK `arc_certificate_tests`,
   `arc_recovery_tests`, `certified_recovery_tests`,
   `element_reformation_tests` and `formation_check_tests`.

### Step B: product admission (reserved; DEL-03-03, DEL-05-03, DEL-02-01, DEL-04-07)

- **Selection.**
  - `pressure_runtime::selects_timoshenko` and `pf_timoshenko::selects` admit
    v2 for straight members, with an unchanged formation.
  - A new predicate selects arc shear.
  - The `model_settings` enum and refusals (`ShearModelSetting`, messages) and
    the `pressure_contract.shear_model` vocabulary gain the value.
- **Arc A_s.**
  - In `build_model_for_basis`, a v2 arc's pipe computes A_s by its route's
    straight rule: the exact route uses the nominal annulus with the member's ν
    (`annulus_energy_shear_factor`); PF uses `pf_timoshenko::shear_area` on the
    consumed basis.
  - A_s is stored in a map beside `member_moduli`.
    `build_curved_bend_macro_elements` calls
    `with_transverse_shear_area(a_s)`, and `curved_formation_of` copies it.
  - A missing factor blocks by name, as for straights.
- **Modal M4.** `modal/prepare.rs` refuses v2 under the same code
  (`MODAL_TIMOSHENKO_MASS_UNSUPPORTED`), so an Euler–Bernoulli mass is never
  paired with shear-flexible stiffness. The message names the selected value.
- **Disclosure and identity.**
  - Under v2, `append_arc_disclosure` does not fire.
  - v2 needs its own result-semantics ID, profile and limitation 0, stating
    that straights and arcs use κ_E and that arc stress stays the straight
    annulus recovery.
  - Per-member `beam_theory` (`lib.rs` ~12203) needs an arc record.
- **Consumers.**
  - `schemas/model.schema.yaml` (and its physical-transform twin) and the
    results schema for the new identity.
  - The desktop `ShearModel` types and `ModelSettingsSection`, where the offer
    needs an explicit UI design and wording.
  - `operation_applier` admission lists and authoring diagnostics.
  - The DEL-02-01 result reader (`numericalResultQuality.ts` and the identity
    tests).
  - Each is a reserved shared root until its actual diff is reviewed.
- **MBF.** No MBF change is needed for truthful behaviour.
  - The writer admits only `euler_bernoulli` and v1 in `pressure_contract`
    (`writer/mod.rs` `SHEAR_MODELS`; `sidecar.rs`), so a v2 exact document
    visibly refuses export.
  - A v2 `model_settings` document exports with the existing loss entry
    `MBF-SI-MODEL-SETTINGS-NOT-EXPORTED`, as v1 does today.
  - Import reads no CAEPIPE beam-theory field. It never produces v2: a
    no-pressure import has absent settings (EB), and a pressure import keeps
    v1 or the chosen EB/v1.
  - Any mapping, sidecar format or format-6 dependency belongs to the MBF
    owner through Agent 0.

## 6. Preservation of bytes and results

Every `None` path executes the present instructions. There is no `+ 0.0`.
Bindings, K-D5 re-formations and certificates skip the shear operations
entirely. The existing pins, `platform_bits_tests`, the T4-U1 references, the
t4_u8 and pf_timoshenko references and the native-smoke expectations should
therefore stay green unchanged, and that is the regression check. A pin that
moves is a defect to find, not a value to update. v2 straight members share
v1's formation. Only arcs differ.

## 7. Route, pricing, layout and witness impact

These were checked against the code, not estimated.

- **Layout.** `CurvedFormation` is held by value in `LoadFormula::UniformCurved`,
  `FreeStrainCurved`, `PressureFreeStrainCurved` and
  `PressureGeometry::Curved`. `size_of::<LoadFormula>()` is an in-build atom
  of the retained-memory profile (`retained_memory.rs` `s(LoadFormula)`,
  coefficient ≥ `LOAD_CASES`). `PINNED_RECORD`
  (`retained_memory_law_tests.rs`, governed by
  [DEL-04-08 `retained-pricing-rules.md`](../../DEL-04-08/Work/retained-pricing-rules.md))
  pins the phase totals in the qualified build.
  - Adding `Option<f64>` (16 B) moves the record only if a curved variant is
    `LoadFormula`'s largest. That depends on the compiler's layout, so it must
    be measured.
  - Step A's first build on `t3_cargo` prints `size_of::<LoadFormula>()` (FK
    `tests/tie_rod_element.rs` `sizes`) at base and candidate.
  - If it moves, the record is re-derived under the pricing rules as its own
    §8 entry, never edited to pass.
  - New Display text in error arms is priced as well (§8.30's precedent), so
    Step B's diagnostics need the same accounting.
- **Witnesses.** The KD5 member bound, the formation closure, the retained
  witness tests and NI's matched-source equality all compare K bits against
  the element's own `global_stiffness`. Re-forming from the same
  `CurvedFormation` keeps them consistent. No coefficient, tolerance or pin
  changes.
- **Routes.** Dense, sparse and nonlinear assembly consume the element's
  matrix unchanged. The retained route stores no curved-specific layout beyond
  `LoadFormula`.

## 8. Verification plan for the implementing slice

- **Step A tests.**
  - Kernel v2 `end_flexibility` against `expected.json`
    `end_flexibility_local_v2`.
  - Kernel `None` against `_euler_bernoulli`. The oracle's EB values agree with
    the present closed forms to 2e-16 of the largest entry in a scratch
    transcription check, which confirms the frame and signs.
  - The shear block against the sweep's `shear_flexibility_over_R_per_GAs` on
    both series branches.
  - Free-tip and consistent load vectors (`consistent_uniform_load_global_v2`),
    the global stiffness (`global_stiffness_v2`), and the S1T/S1P invariance.
  - FK `curved_matrix` v2 enclosing the kernel's K. `certify_curved_uniform_load`
    enclosing the kernel's v2 load vector.
  - `curved_binding` unchanged for `None` and distinct for `Some`.
  - The NI source copy, scaled and unscaled.
- **Step B tests.** Modal v2 refusal, v1 arc disclosure unchanged, v2 straight
  members bit-equal to v1, A_s blocking by name, and the MBF export behaviour
  of §5.
- **Running them.** All pytest runs go through `t3_slot`. Cargo runs only
  through `t3_cargo` in the owned trees. Independent scrutiny is required
  because numerical meaning changes.

## 9. Open choices (Agent 0)

1. **Exact-contract vocabulary.** `pressure_contract` 3.0.0 is a closed
  vocabulary. Adding v2 there is a contract change, while the
  `model_settings` route takes it additively.
  - The recommendation is to admit v2 on both routes in Step B, so that
    pressure documents can take arc shear.
  - The alternative is PF first and the exact route later.
2. **UI wording and the offer.** These need an explicit design. The proposed
  label is "Timoshenko, straight members and bends (energy-matched annular
  factor)".
3. **Result identity names** for v2 (semantic ID and profile).
