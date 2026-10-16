# T4-U6 and T4-U8: section bases and the Timoshenko straight member

This is the technical design for the first post-T3 shared epoch (b7-plan, "After B7b", item 1). It applies the rulings in [section-and-shear-decisions.md](section-and-shear-decisions.md) and changes nothing they decide. Pressure-free documents (no `pressure_contract`) keep their single basis, their Euler–Bernoulli frame and their bytes (H-1).

Governing owner words: D-5 and D-6, "D-5 and D-6 as recommended" (2026-10-10); O2, "Required; absent = refused"; O3, "Refuse by name"; O4, "Scope extension after B7b" (2026-10-12). HELP_HUMAN's H1, H4 and H5 (2026-10-12) are applied as recorded in the rulings file.

Vetted physics bases (read from Git history, not restored into the tree):
- SHEAR_REFERENCE at `f2211ecef6`, `…/CONTINUATION_2026-09-24/CORRECTNESS_DESIGN/SHEAR_REFERENCE/` (CONTRACT.md §§1–6; INDEPENDENT_REFUTATION.md; `_run_records/shear_refutation/reference_arithmetic.py`).
- The repaired T4-I8 freeze at `5b07d85cf3`, `…/PRESSURE_STRESS_T4/T4-I8/` (tracked copy: `validation/references/t4_i8/`), including MILLTOL's `refreeze.after_D5_as_recommended` block and T4-RV4's shear-affected marks.
- Plan 01 at `f2211ecef6`, §3.1 and §5 rows for T4-U6 and T4-U8.

No corpus equation artifact is used (DEC-043).

## 1. Section geometry under D-5

For an exact-contract pipe section with outside diameter D, wall t, mill tolerance c_m and corrosion allowance c_a (each a finite length, c_m, c_a ≥ 0, c_m + c_a < t; the existing refusals apply unchanged):

| Name | Wall | Inner radius | Used for |
|---|---|---|---|
| **Nominal section** | t | r_i,n = D/2 − t | stiffness (A, I, J and the Timoshenko shear area), pipe metal mass, the area As in ε_p = (1−2ν)pAi/(E·As), nominal-basis stresses |
| **Reduced section** | t_r = t − c_m − c_a | r_i,r = D/2 − t_r | reduced-basis stresses |
| **Pressure bore** | — | r_i,r | end-cap and joint-face area Ai = π r_i,r², contents mass, the equal-bore region check, `source_bore_key` |

r_o = D/2 for both sections. Each section's A, I, J = 2I and Z = I/r_o come from `SourceAnnulus::from_od_wall(D, wall)` with that section's wall. Today's single section uses t − c_m, so with c_a = 0 the reduced section is bit-identical to today's section, and with c_m = c_a = 0 both sections are.

**Every geometry site.** The producer derives the three geometries once per pipe on the exact route (`derive_pipe_section`'s exact branch and `build_model_for_members`) and threads them by name. No site re-derives a wall from `wall − mill_tolerance` on the exact route after this change.

| Site (today) | Basis after D-5 |
|---|---|
| `build_model_for_members` stiffness section, `StraightPipeSectionProperties::new`, `CurvedBendMacroElement::new` (arc A, I, J) | nominal |
| Thermal/fit eigenloads E·A·ε (`build_thermal_element_loads`, `load_state_eigen_loads`, `FreeStrainStraight`) | nominal A |
| Straight Poisson eigen pair 2νpAi, terminal caps ±pAi, joint face p(Ae − Ai), `PressureOperand`, `certified_pressure_area_product`, `pressure_adjustment`, `arc_pressure_wall` | Ai on the pressure bore |
| Arc `PressureFreeStrainCurved` / `arc_pressure_strain` | Ai on the pressure bore, As = nominal arc area (the element's own area, N-3); `ARC_PRESSURE_STRAIN_ROUNDINGS` is re-derived for two walls |
| `recover_wall_effective_membrane`, `surface_stresses`, `straight_stress_spans`, `exact_straight_summary_extrema`, `fallback_circular_maximum`, `recover_section_stress`, `append_stress_mpa`, endpoint/station stress pushes, arc membrane | once per basis (section 2) |
| `stable_mass_per_length` (exact branch), FK `MassPrimitive` / `mass_reformation` | metal area on the nominal wall; contents on the pressure bore. A new method id names the split; the preview branch keeps `source_od_effective_wall_areas/v1` |
| `PRESSURE_REGION_BORE_MISMATCH`, `source_bore_key` | pressure bore |
| Evidence `pipe_sections` / `exact_section_evidence` | all three, by name (section 2.3) |
| Rust readers (`physics_evidence.rs`, `physics_source.rs`, `retained_precision.rs` G5b/G8) and Python verifiers (`pressure_1.py`, `physics_evidence.py`, `physics_source.py`, `retained_precision.py`) | rederive all three from the captured invocation and bind them bit for bit |

The preview route's `derive_pipe_section` (wall − mill for every use), its mass method and every preview row stay exactly as they are. The exact branch is separate code, so route L's pricing decisions can zero it.

**Consequences for the one mechanical solution.** Forces, moments, displacements, reactions, N_w = N_el + pAi and S = N_el + (2ν−1)pAi are basis-independent: one solve with nominal stiffness and pressure-bore loads. Because As in ε_p is the stiffness As, the restrained wall force −(1−2ν)pAi does not depend on As, so N_w and S do not move when the stiffness section moves from t − c_m to t (T4-I8 RV1 N-5).

## 2. Outputs

### 2.1 Stress rows on two bases

On an exact-contract document every row whose pressure-1 table family is `stress` is published twice, once per section basis, at the same entity, location and component. That covers `element_local_axial_normal_stress` (members outside every pressure region), `element_local_bending_normal_stress_y/z`, `element_local_torsional_shear_stress`, `pipe_axial_membrane_stress_v2`, `pipe_lame_radial_stress_v2`, `pipe_lame_hoop_stress_v2` and `pipe_elastic_normal_stress_maximum_v2`, on straight members and realized arcs alike. Both families are published even where the bases coincide (H4). Force, moment, displacement, rotation, reaction, wall-force and effective-force rows are published once.

Per basis b (section A_b, Z_b, J_b, r_i,b):
- axial normal σ = N/A_b; membrane σ_m = N_w/A_b;
- bending σ = M_y/Z_b, M_z/Z_b; torsional shear τ = T·r_o/J_b;
- Lamé, the closed annulus solution of the basis's own section with internal pressure p at r_i,b: radial −p (inner) and 0 (outer); hoop at the outer surface 2p·π r_i,b²/A_b and at the inner surface that plus p. The pressure-load bore governs Ai in the loads; it does not move the stress solution of the nominal section onto another annulus;
- the member maximum max over stations of |N_w/A_b| (or |N/A_b|) + hypot(M_y, M_z)/Z_b, with the existing certified circular-maximum machinery evaluated with that basis's A_b and Z_b. Arcs keep the withheld maximum and its warning.

**Naming.** Each stress row names its basis in `metadata.basis`, by appending `; section_basis=nominal_section` or `; section_basis=reduced_section` to its existing recovery token (the solver-mode row already uses this `; key=value` form). For example `recovered_from_open_mechanics_stress_components; section_basis=reduced_section`, `recovered_from_local_element_stiffness; section_basis=nominal_section`, and on a selected source case `retained_source_straight_stress_v1; section_basis=…` and `retained_source_endpoint_normal_max_v1; section_basis=…`. Stress row ids gain a final segment, `:nominal_section` or `:reduced_section` (ids with hyphenated segments use `:nominal-section` / `:reduced-section`), for both families, so neither family is privileged. Non-stress ids and tokens are unchanged.

### 2.2 Maxima and headlines

Each basis has its own member maxima and its own headline. On an exact-contract document:
- `summary.max_open_formula_stress` is `null`; the product does not choose a governing basis;
- `summary.max_open_formula_stress_nominal_section` and `summary.max_open_formula_stress_reduced_section` are always present, each a `LocatedQuantity` bound to a maximum row of its own basis or `null` when that basis's coverage is incomplete (the existing withholding rules apply per basis).

On a pressure-free document the two new keys are absent (serde: an outer `Option` skipped when `None`), so those bytes do not move. The per-case internal maximum becomes one per basis, and `maximum_across_cases` runs per basis.

### 2.3 Evidence

`pipe_sections` (results.v0.3 `PhysicsGeometry`; `results_pressure_1.v0.3.schema.json`) becomes, per pipe: `pipe_id`, `geometry_basis = "authored_od_wall_d5_sections_v1"`, `outside_diameter_m`, `ro_m`, `wall_thickness_m`, `mill_tolerance_m`, `corrosion_allowance_m`, `nominal_section {wall_m, ri_m, As_m2, I_m4, J_m4, Z_m3}`, `reduced_section {wall_m, ri_m, As_m2, I_m4, J_m4, Z_m3}`, `pressure_bore {ri_m, Ai_m2}`, and `beam_theory`. `beam_theory` is `{"model": "euler_bernoulli"}` or `{"model": "timoshenko_annulus_energy_v1", "shear_factor": κ_E, "shear_area_m2": A_s}` (section 3.1). Realized arcs carry `beam_theory = euler_bernoulli` in their arc evidence whatever the document selects. `pipe_stress_extrema` and the per-case stress-maximum coverage records become per basis. The receipt's section evidence (`physics_source_recovery.schema.json`) and W1-E's `section_terms` keep one section, because both routes admit only documents whose bases coincide (section 5); their readers bind that section to both `nominal_section` and `reduced_section` bit for bit.

### 2.4 Tables, schemas and limitations

- **pressure-1 table** (`fixtures/results/semantic_contract_v0_3_pressure_1.json`): every stress kind gets two signatures discriminated by `source_basis` (the mechanism pressure-source-1 and preview-physics-1 already use), plus the `source_basis_signature_policy` they use; `canonical_metadata_vocabulary.basis` gains the tokens. The pinned counts (88 signatures, 59 kinds, `semantic_contract.rs`) change with it. **pressure-source-1** and **physics-retained-1** follow (physics-retained-1 by its inherited hash).
- **Schemas**: `results.v0.3.schema.yaml` (basis enum, `PhysicsGeometry`, physics-retained-1's const limitations), `results_pressure_1.v0.3.schema.json` (basis enum, `pipe_sections`, summary keys), `analysis_run.v0.3` and `stress_neutral_export.v0.3` (table hash consts), `model.schema.yaml` (the `PressureContract` description; the 0.5.0 field shapes are unchanged, H1). Python analysis-run and stress-neutral packages for pressure-1 and pressure-source-1 carry both families through the table-generic matcher; their carriers are regenerated.
- **Limitations** (`exact_admission.rs`, whole static strings, positions unchanged). The identity limitation (index 0) replaces "(Euler-Bernoulli; source OD/effective wall define the single section basis; G is derived from E/nu)" with:

  > (shear deformation as the contract declares: euler_bernoulli, or timoshenko_annulus_energy_v1 with the energy-matched annular shear factor, whose nodal rotations are section rotations; G is derived from E/nu). Section bases: stiffness and pipe metal mass use the nominal wall; contents mass and pressure end-cap areas use the reduced bore; every stress row is published on the nominal section and on the reduced section (wall minus mill tolerance and corrosion allowance) and names its basis, and each basis has its own maximum and headline; no governing basis is chosen.

  and, for pressure-1, "realized curved bends are Euler-Bernoulli under either shear model and, with expansion joints authored as objective connectors, are admitted only as the bend and connector limitations state." The pressure-source-1 identity says its selected response is Euler–Bernoulli with coincident bases.

  Limitation 7 (pressure-1, joined unselected):

  > Every member section declares its corrosion allowance and the contract declares its shear model. This envelope publishes ordinary structural recovery. Retained-source recovery is available only for euler_bernoulli documents with zero mill tolerance and zero corrosion allowance, and only for straight members, linear restraints or springs, nodal force and moment loads and explicitly empty pressure regions; an authored load/reference-state group includes resolved eigenloads and prescribed motions. Timoshenko documents, nonzero mill tolerance or corrosion allowance and other source families are declined by name. No code compliance or professional acceptance is produced, and actual numerical admission remains separate.

  Limitation 7 (pressure-source-1, selected) states the same admission: the selected response is euler_bernoulli with zero mill tolerance and zero corrosion allowance, where the two bases coincide. The held variant (`selected = None`), whose text says no allowance or shear selection exists, is removed if unreachable, else rewritten to the current contract.

### 2.5 Identity and stored results

`pressure-1`, `pressure-source-1` and `physics-retained-1` carry the change under their current versions: result format 0.3, model format 0.5.0, the same identities, with revised tables, schemas and limitation text, re-pinned and re-registered (as B7a and B7b revised these tables in place). `pressure-2` and `preview-physics-2` remain reserved for U4b/U5b on 0.6.0 and are not introduced. Under the owner's one-current-version direction, a stored result written before this revision is read against the current table and is refused (basis contradiction or headline binding), and an analysis-run record binding the old table hash is refused with `ANALYSIS_SEMANTIC_CONTRACT_MISMATCH`; the user re-runs. No migration or legacy acceptance is added.

## 3. The Timoshenko straight member (T4-U8)

### 3.1 Section shear rigidity

For `timoshenko_annulus_energy_v1`, from the nominal section's binary64 leaves r_i = r_i,n, r_o and the member's selected ν (the same E/ν basis that gives G; per resolved case on load-state documents):

```
m  = r_i / r_o;  m2 = m*m;  a = 1 + m2;  b = 1 - m2;  n1 = 1 + nu
c  = (8*nu + 14)*nu + 7
D  = c*(b*b) + 48*(m2*(n1*n1))           // SHEAR_REFERENCE's cancellation-free form
kappa = (6*(n1*n1))*(a*a) / D             // kappa_E, energy-matched (not Cowper, not 5/6)
A_s = kappa * A_n                         // shear area leaf; Ks = G * A_s
```

The operation order is fixed and uses no fused multiply-add, so the Rust and Python readers reproduce κ and A_s bit for bit from the evidence. A_s is a binary64 leaf like A and I: the intended K-D5 system is formed exactly from it. A missing ν or G basis blocks a Timoshenko solve by name; no default factor exists.

### 3.2 Representation in FK (H5: a re-priced FrameElement)

`FrameSection` gains `beam_theory: BeamTheory`, an enum `EulerBernoulli | Timoshenko { shear_area: f64 }`. `FrameSection::new` keeps building Euler–Bernoulli sections, so its 72 call sites and every Euler–Bernoulli formation are unchanged bit for bit. `FrameSection::timoshenko(E, G, A, Iy, Iz, J, shear_area)` checks a finite positive shear area. `StraightPipeSectionProperties` and `StraightPipeElement` carry the same leaf into `frame_section()`, so `pipe.frame_element()`, every load primitive's frame copy and every recovery frame carry it. The layout growth (FrameSection 48 → 64 B, FrameElement 136 → 152 B) is priced (section 9).

### 3.3 Stiffness

Per bending plane with I (Iz for v = u_y, θ = r_z; Iy for v = u_z, θ = −r_y, as in the existing frame convention), φ = 12EI/(G·A_s·L²):

```
k_vv = 12EI/(L³(1+φ))   k_vθ = 6EI/(L²(1+φ))   k_θθ = (4+φ)EI/(L(1+φ))   k_θθ' = (2−φ)EI/(L(1+φ))
```

placed exactly where `add_bending_z` / `add_bending_y` place the Euler–Bernoulli coefficients. Axial EA/L and torsion GJ/L are unchanged (A is the metal area, never A_s; κ never touches GJ). Each intermediate is a named K2a checked formation. The difference (2 − φ) may be exactly zero (φ = 2), so it and its product are checked for finiteness only; a zero there is not an underflow. The Euler–Bernoulli branch is the existing code. The 12×12 element keeps exactly six rigid modes.

### 3.4 Load equivalents for every admitted load family

| Exact-route family | Timoshenko equivalent |
|---|---|
| Full-span uniform member load (authored, self-weight, contents weight) | [wL/2, wL²/12, wL/2, −wL²/12]: identical to Euler–Bernoulli for a prismatic member (SHEAR_REFERENCE §4), so the existing terms are used unchanged in assembly, recovery, the S11-G load-row guard and K-D5's `uniform_straight_balls` |
| Partial-span uniform member load | Not reachable on the exact route (extents arise only from equivalent-static wind, which the exact route refuses). The straight_pipe primitive still implements the exact Timoshenko vector: with R_ss = W1(L)/L and the Euler–Bernoulli V0_EB, ΔV0 = φ/(1+φ)·(R_ss − V0_EB) and feq_T = feq_EB + ΔV0·[1, L/2, −1, L/2] (derived from §4's integrated solution; distal half at φ = 1 gives [7/64, 13/384, 25/64, −19/384]). K-D5 forms the same correction in ball arithmetic or records a Timoshenko partial-span primitive as unavailable (fail-closed); either way never the stale Euler–Bernoulli vector |
| Point member force | No such load exists on any route (element loads are per length); the Timoshenko primitive refuses it by name rather than using Hermite terms |
| Thermal/fit eigenstrain, pressure Poisson pairs, caps, joint faces | Axial; unchanged |
| Prescribed motions | Partitioned elimination through the assembled K, so Timoshenko K carries them; unchanged code |
| Nodal forces and moments, springs, connectors | Unchanged |

### 3.5 Recovery and interior reconstruction

End actions are f = K_T·d − feq with the same feq used in assembly, in binary64 (`recover_local_forces`, `exact_straight_end_forces`) and in K-D5's certified recovery (`frame_matrix`). Station resultants V and M are statics from the i-end action plus load integrals, which hold for any beam theory, so `station_resultants_from_i_end_with_spans`, the certified section rows and the Bernstein stress spans are unchanged. No interior displacement or rotation is reconstructed anywhere today, and none is added: there is no Hermite displacement interpolation to replace. Published nodal rotations of a Timoshenko member are section rotations θ, not slopes v′ (disclosed in limitation 0).

### 3.6 Scope of the selection

The selection is per document and applies to every straight member. Realized arcs stay Euler–Bernoulli under either selection and say so in their evidence and in limitation 0. Connectors, springs and supports are unaffected. Euler–Bernoulli remains an explicit legitimate choice. The W1 retained element (`structural/retained/assemble.rs`) and the source method stay Euler–Bernoulli; both decline Timoshenko before formation (section 5), and the retained assembler refuses a Timoshenko leaf if one ever reaches it.

## 4. The K-D5 certificate with a Timoshenko element

The certificate bounds published member, section, support and pipe-wall groups against the intended system K_int u = f_int formed exactly from the binary64 leaves. Its only premise is A1 (the Hager-estimate inverse bound), and the formation error ε_E is measured exactly after the fact; no a-priori per-entry bound enters. Nothing in the bound uses Euler–Bernoulli structure. With the changes below it stays sound:

1. `frame_matrix` (formation_check.rs) forms φ, 1+φ, 4+φ and 2−φ at p from the frame's own leaves when `beam_theory` is Timoshenko. It feeds ρ, s, s₂, ε_E, certified recovery, support reactions and the stress spans, so all of them see the same intended element. ε_E absorbs any binary64 Timoshenko formation error, including cancellation in (2−φ); a larger ε_E fails closed (Sensitive), never open.
2. **Frame binding.** `frame_binding` includes the leaf, and a new fail-closed check requires every straight load primitive's frame and every certified-recovery frame to be bit-equal to the FormationSource frame on its node pair. This closes the split-source hazard (a primitive or recovery frame without the leaf would certify a mixed system).
3. **K2b force scaling.** A_s is an area, so φ is bit-invariant under the exact 2^b scaling of E and G; `ForceScaleCensus::frame` predicts the Timoshenko intermediates.
4. **Load equivalents.** Full-span uniform loads are unchanged (section 3.4), so the S11-G guard and `uniform_straight_balls` keep their exact formulas. A Timoshenko partial span cannot reach the exact route; if the K-D5 implementation records it as unavailable, that is fail-closed.
5. **M3-T, evidence that the certified gate holds.** The certificate certifies against K_int and cannot see a wrong but well-defined intended system (for example stale Euler–Bernoulli vectors with Timoshenko K). An independent exact Timoshenko reference therefore judges the integrated producer: a generator that does not import or read product code forms the SHEAR_REFERENCE element in exact or 80-digit arithmetic and solves invented exact-route models (cantilevers, L-lines, skew multi-span runs, interior springs, prescribed motions, self-weight; φ from about 1e-6 to above 2, including φ = 2), in both solver modes and both typed and captured entries. Acceptance is the K-D5 M3 criterion: 0 judged member, section, support or stress breaches on Passed cases and 0 strict coverage failures. The Euler–Bernoulli path is bit-identical, so the existing M3 evidence (RV30, realistic, RV29, W1) stays valid for it.

If M3-T finds a judged breach on a Passed Timoshenko case that a sound repair cannot remove, the epoch stops and returns to HELP_HUMAN; the gate is never loosened.

## 5. Admission, declines and readers

**Admission.** `EXACT_PRESSURE_CORROSION_ALLOWANCE_UNSUPPORTED` (lib.rs `derive_pipe_section`) and `EXACT_PRESSURE_SHEAR_MODEL_UNSUPPORTED` (pressure_runtime.rs `validate_profile`) are removed. `EXACT_PRESSURE_*_REQUIRED`, `*_WITHOUT_PRESSURE_CONTRACT`, the unknown-shear refusal, the section-sum check and every quantity refusal stay. The retirement diagnostic text that recommends `shear_model euler_bernoulli` (pressure_runtime.rs, embedded in pressure-free fixtures) is left unchanged: it describes a legitimate re-authoring and keeps pressure-free bytes.

**O4 interim declines.** `prepare_sources` declines, by name and after the existing D7 declines, a document whose contract selects `timoshenko_annulus_energy_v1` ("timoshenko_annulus_energy_v1 straight members are not a retained-source formulation"), a pipe section with nonzero mill tolerance ("nonzero mill tolerance is not a retained-source section basis") and one with nonzero corrosion allowance ("nonzero corrosion allowance is not a retained-source section basis"). The messages avoid "coincident" and "transform", which select other receipt codes. As with D7, a declined case publishes ordinary pressure-1 rows; because these are document facts, such a document never becomes a pressure-source-1 composite. W1-E declines the same documents at its G-A admission (`family_clauses` / `namespace_branch`), under named family facts (`ShearModel`, `SectionAllowance`) rather than the generic `PressureContract`, so the published ordinary bytes are exactly the ordinary result. Nonzero mill tolerance, admitted on both retained routes today with one effective wall, is declined from now on, because under D-5 those routes have no second basis.

**Readers (Rust and Python, the matching current contract).**

| Identity | Contract checks | Geometry and rows |
|---|---|---|
| pressure-1 | shear_model ∈ {euler_bernoulli, timoshenko_annulus_energy_v1}; allowance present, ≥ 0, c_m + c_a < t | Rederive the nominal, reduced and pressure-bore geometry and `beam_theory` (κ, A_s bit for bit; arcs Euler–Bernoulli) from the invocation; check each stress row against its basis's section; require both families for every stress signature; per-basis maxima and headlines (`max_open_formula_stress` null, both per-basis keys present, each bound to its own basis and ≥ every maximum of its basis) |
| pressure-source-1 | euler_bernoulli (existing), zero allowance (`PHYSICS_SOURCE_ACTUAL_CORROSION_ALLOWANCE`, existing), and zero mill tolerance on every pipe section (`PHYSICS_SOURCE_ACTUAL_MILL_TOLERANCE`, new) | Source stress checks per basis: the reduced row binds the same functional and section as the nominal row and is bit-equal to it; per-basis composite headlines |
| physics-retained-1 | G8: euler_bernoulli and zero allowance (existing) and zero mill tolerance (new, named) | G5b/G8 bind the one receipt section to both bases; the successor overlays each reduced row with its nominal row's retained value; G7's base-reader rerun applies the pressure-1 rules |

DEF-E (`retained_precision_prepared_exact_v1.json`, reviewed input 7) states zero mill tolerance with zero allowance and euler_bernoulli, and lists the reduced-basis rows.

## 6. Authoring (D-6/O3)

From U8 on, activating the exact contract writes `shear_model = timoshenko_annulus_energy_v1` by default: desktop `PRESSURE_V3`, `BLANK_LOAD_STATE_PRESSURE_CONTRACT` and the operation applier's profile gate (`pressure_authoring.rs`), which admits both values. The pressure panel gains a shear-model selector (Timoshenko with the energy-matched annular factor, the default; Euler–Bernoulli as an explicit choice) and states that realized arcs are Euler–Bernoulli. `pressureProfileTarget` preserves a document's existing selection instead of rewriting it. Corrosion-allowance authoring is unchanged: the inspector already writes nonzero values, activation still requires the field on every section, and nothing manufactures a value. Save stays version-only. Desktop output surfaces for the exact identities stay refused (owner, "Merge with the gap"). The frozen UI benchmark (`apps/desktop/e2e/ui-foundation/fixtures/`) is not touched. The Rust test helper `current_contract()` keeps writing an explicit Euler–Bernoulli selection, which keeps every Euler–Bernoulli reference on its own basis; the desktop L-line fixture follows the desktop default.

## 7. References

| Reference | Change | Method |
|---|---|---|
| T4-I8 MILLTOL (`validation/references/t4_i8/`) | Re-authored with the true nominal wall 0.01 m, explicit allowance 0.002 m and mill tolerance 0.00125 m, and re-frozen on D-5: nominal stiffness and ε_p area, pressure-bore Ai, both stress families. `cap_area_on_nominal_bore` becomes active | The I8 generator (`_run_records/derive_rebuilt_references.py`, exact rationals, no product code), extended for D-5; the result must agree with the frozen `refreeze.after_D5_as_recommended` prediction (Nw, S, reduced σz and Lamé unchanged; displacements and nominal stresses move). MILLTOL has no transverse load, so it serves both shear models: one re-freeze |
| T4-I8 case 2 and twin A | Unchanged for Euler–Bernoulli. A Timoshenko variant is frozen for the marked rows (u_y at B, C, D; twin A tip deflection), with every other row asserted unchanged | The new t4_u8 generator (below), independent of product code |
| T4-I7 (`validation/references/t4_i7/`, 72 sketches with mill tolerance 0.000889 m) | Re-frozen on D-5 (stiffness on the nominal wall; both stress families). Arc and zero-mill cases keep their values | `u2_engine.py` / `u2_generate.py`, extended for D-5 |
| T4-U4a (`validation/references/t4_u4a/`, 8 pressure-1 cases with mill tolerance 0.0007 m) | Re-frozen on D-5; its I7 cross-check SHA follows I7 | `u4a_engine.py` / `u4a_generate.py`, extended |
| VP-STATIC exact_pressure package | `MANIFEST.json` regenerated; the independent review of these re-freezes is bound in `ADMISSION.json` as its README requires | `generate_package.py --write` |
| New `validation/references/t4_u6/` | The same model on each basis: a pressurized cantilever with mill tolerance and allowance, self-weight, contents and a transverse tip force, frozen for displacements (nominal stiffness), reactions (metal on the nominal wall, contents on the pressure bore), pressure thrust (pressure-bore Ai), restrained wall force (ε_p on the stiffness As) and every stress row on both sections; a zero-allowance twin whose families coincide; discriminators for each wrong mapping; Euler–Bernoulli and Timoshenko variants | New generator, exact rationals/Decimal, no product code |
| New `validation/references/t4_u8/` | SHEAR_REFERENCE §5: S-A rational controls (including the distal half, 53/384 vs the stale-vector 54/384), S-B physical cantilevers at L = 1, 2, 10 m with reversed and rotated companions, S-C coefficient checks (m → 0, m → 1, ν = 0, two nonzero ν, m = 0.9, unit scaling), S-D slender limit and 1/2/4/8 subdivision, S-E interior spring (rational and physical); the I8 case-2 and twin-A Timoshenko rows; the M3-T set | New generator from SHEAR_REFERENCE's formulas and its reference arithmetic, no product code |

Cases with zero allowance, zero mill tolerance and Euler–Bernoulli keep their reference files and values; their consuming tests assert both stress families against the one expected value where the documented bases coincide. `validation/references/t3_r1/` contains no exact-pressure, section-basis or shear family and is not touched (its J7 census consumer re-authors to explicit Euler–Bernoulli). T4-U5a's references are basis-neutral and unchanged; T4-I12's stand under explicit Euler–Bernoulli.

**SSLL106.** Plan 01 names SSLL106's transverse cases for U8. Code_Aster's reference uses Timoshenko with a shear area equal to the section area (κ = 1); with m = 0.9 and ν = 0.3 that differs from κ_E by about 2.2e-4 in tip deflection, and its assets are not in the repository (catalogue: `source_packet_prepared_not_scoring_ready`). It is therefore not a formulation-matched oracle for `timoshenko_annulus_energy_v1` and is not used to judge it. S-B's physical cantilevers carry the published-geometry role. A formulation-matched published benchmark remains open.

## 8. Mutants

Each is killed by the named test.

| Mutant | Killing test |
|---|---|
| Mill tolerance leaking into stiffness (stiffness on t − c_m) | `u6_bases_stiffness_uses_the_nominal_wall` (t4_u6 displacements; MILLTOL displacements) |
| Allowance leaking into stiffness | `u6_bases_stiffness_uses_the_nominal_wall` |
| Contents on the nominal bore; metal mass on the reduced wall | `u6_bases_mass_metal_nominal_contents_pressure_bore` |
| Cap area on the nominal bore | `i8_milltol_d5_refreeze` (`cap_area_on_nominal_bore` discriminator) |
| ε_p area on the reduced section | `u6_bases_restrained_wall_force_uses_the_stiffness_area` |
| One basis for both families (reduced rows copied from nominal) | `u6_bases_stress_rows_on_both_sections`; reader mutation `pressure_1_reader_refuses_cross_basis_stress` |
| A headline bound across bases | reader mutation `pressure_1_reader_refuses_cross_basis_headline` |
| Stale Euler–Bernoulli load equivalents with Timoshenko K | `timoshenko_distal_half_load_kills_the_stale_vector` (54/384) |
| θ = v′ | `timoshenko_s_a_rational_cantilever_controls` (tip θ = 1/2, slope 7/12) |
| κ applied to GJ; A_s in EA | `timoshenko_axial_and_torsion_are_unchanged` |
| Swapped bending-plane signs | `timoshenko_bending_planes_and_rotation_covariance` |
| Cowper, solid-circle or 5/6 factor | `timoshenko_kappa_is_energy_matched` (S-B κ_E, S-C limits) |
| Timoshenko leaf missing from a recovery or primitive frame | `kd5_straight_frames_are_bound_to_the_formation_source` |
| prepare_sources or W1-E admitting a Timoshenko, mill-tolerance or allowance document | `o4_prepare_sources_declines_by_name`, `o4_w1e_declines_at_admission` |
| Authoring writing Euler–Bernoulli by default, or re-queuing overwriting a selection | desktop `PressureAuthoringPanel` and applier `exact_authoring` tests |

## 9. Blast radius and D3

**Reviewed inputs** (12, unchanged list): #7 DEF-E, #8 physics-retained-1 table, #9 pressure-source-1 table, #10 `results_pressure_1.v0.3.schema.json`, #11 pressure-1 table, and #1 `physics_source_recovery.schema.json` if the receipt's section evidence changes. **Priced layouts**: FrameSection, FrameElement, StraightPipeSectionProperties, StraightPipeElement (LoadFormula's largest variant is the connector's, so it should not grow; confirmed from the native profile). **TEXT**: the limitation literals, row tokens, decline messages and K2a names.

**Fixtures and corpora regenerated by their own writers**: the pressure-1 and pressure-source-1 raw twins and carriers (`SWBPIPE_PRESSURE_SOURCE_FIXTURES_WRITE`, `LOAD_REFERENCE_WRITE_FIXTURES`, `LOAD_REFERENCE_SOURCE_WRITE_FIXTURES`, `STRESS_NEUTRAL_WRITE_FIXTURES`), `pressure_*_mechanics_*` (`exact_pressure_connected` example), the v3 straight/arc/connector/joint reader corpora, J2 (`retained_precision_corpus.py pack`), J7 expectations, the W1 exact successors and derivatives, RV120 shapes, the R1 and R2 reader records and the desktop WASM equivalence inputs, the L-line and contract-corpus fixtures, the generation records, and the hash pins (Rust, Python, schemas, desktop tests). Tests that look rows up by (kind, entity, location, component) take the basis as a key. Pressure-free fixtures must come out byte-identical; any pressure-free byte change is a defect.

**D3.** M = 12 GiB, 0.9 M, R = 64 MiB and the 5 % TEXT reserve are unchanged. Measured on scratch pricing of `dff1ff2a08` (the tool's per-member row premise varied): one extra stress row per member per case on route E costs 7.53 MB of merged dense W3; doubling route E's 21 stress rows per member, with per-basis extrema evidence, leaves merged dense W3 about 145 MB short and X1 about 51 MB short. The budget is met by byte-preserving source changes that remove resident copies, each with its own adjudication rule, source pin and independent review:
- **L1**: `retained_wire.rs` `domain_hash` serializes a borrowed `{domain, payload}` wrapper instead of copying the payload into a new Value (identical text and hash). Measured: +420.6 MB in merged dense W3.
- **L3**: the `rows_by_base_id` clone is built only when the preview exists (route E never has one). Measured: +104.2 MB in merged dense W3 under the dual family, and it restores X1.
- The reduced family is emitted only from exact-route code (zeroed on route L), and the tool's row premise P_final becomes route-specific (route E 72m in place of 51m, census-checked).

Estimated merged dense W3 headroom after L1 and L3 with the dual family: about +390 MB, X1 about +53 MB (the tighter phase), FK layout growth about −29 kB. If the actual-source D3 does not pass all 70 evaluations, the epoch returns to HELP_HUMAN with the measured shortfall; M, the margin, the reserve and the budget model are not changed.

## 10. Verification

Proportionate to a numerical-meaning change: targeted regression checks and independent scrutiny by separate agents of the element formulation and load equivalents, recovery, both D-5 bases, the re-frozen references against their vetted basis, the certificate (M3-T) and the mutants. Checks are selected with `select_affected_checks.py`; per CI_STRATEGY the selected numerical suite runs locally in the registered build and in an emulated unregistered build before the retained-source and registration-dependent changes are pushed, and the hosted `harness` must pass on the exact head.
