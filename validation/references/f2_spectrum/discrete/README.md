# F2 response-spectrum references: discrete frame models

These are frozen, independently derived expectations for DEL-04-10, the
response-spectrum analysis
([scope](../../../../execution/PKG-04/DEL-04-10/ScopeOfWork.md)). They serve
REQ-008 and AC-006, and supply the discrete cases of AC-002 and AC-003.

- The cases are small frame models built on the DEL-04-09 formulation. A
  product built to the two scopes should reproduce every published number to
  rounding.
- C8, C9 and C9-CHAIN take the shapes the cross-engine benchmark can carry.
- The sibling package `../closed_form/` holds the oscillator, lumped-chain and
  continuum cases.

**Status: references with a refutation record.** Their author TASK derived
and cross-checked them. A second TASK, reading no product code, rebuilt
72,820 published reals by its own routes and refuted no value: the largest
disagreement was 4.9e-40. Nine descriptive statements of this README were
refuted (R1 to R9) and are corrected here ([REFUTATION.md](REFUTATION.md)). No
product output has been compared with these references.

## Independence

- The generator imports, reads and runs no product code: nothing under
  `core/`, `apps/`, `api/`, `tests/`, `schemas/`, `fixtures/` or `tools/`.
- Its only other inputs are four frozen reference-side modules of
  `../../f1_modal/mass_discrete/generators/`, imported unmodified:
  - `elements.py`: the straight-member consistent mass and stiffness;
  - `model.py`: the PF section basis, mass assembly and `form_stiffness`;
  - `eigen.py`: Cholesky–Jacobi, Sturm counts and the canonical basis of a
    repeated eigenvalue;
  - `numeric.py`: the exact a + b·π field, the Decimal field and π.
- **How those modules are protected.**
  - Their SHA-256 values are pinned, and the generator stops if one changes.
  - The f1 directory is placed first on the import path, ahead of this
    directory, so no same-named file elsewhere can shadow a pinned module.
  - The generator asserts that each module was loaded from that directory.
- No CAEPIPE material, catalog or code value was read or used. All model data
  are invented. Every formula below is restated from the DEL-04-10 and
  DEL-04-09 scopes and from standard structural dynamics.

## Files

| File | Content |
|---|---|
| `generators/f2_spectrum_discrete.py` | driver and method: system assembly, modal basis, spectra, recovery, combinations, cross-checks, check and write modes |
| `generators/spectrum_cases.py` | the invented models, spectra and requests |
| `c1_one_element.json` | C1 (free tip) and C1C (guided tip) |
| `c2_ground_spring.json` | C2 |
| `c3_cantilever_3d.json` | C3, with mutation data |
| `c4_close_pair.json` | C4, with mutation data |
| `c5_stiff_and_l_line.json` | C5A and C5B |
| `c6_snubbers.json` | C6A (rigid snubbers) and C6B (finite snubbers) |
| `c7_repeated_pair.json` | C7, the exactly repeated pairs, with the basis witness W1 |
| `c8_pipe_x.json` | C8, the benchmark-shaped pipe along X, with its mesh series |
| `c9_pipe_diagonal.json` | C9 (a close pair on the space diagonal) and C9-CHAIN |
| `c10_cluster_witnesses.json` | W2 (distinct modes inside one cluster) and W3 (a cluster straddling f_r) |
| `REFUTATION.md` | the refuting TASK's record; not generated |
| `SHA256SUMS` | hashes of the files above (`shasum -a 256 -c SHA256SUMS` in this directory) |

## Reproduce

```
python3 -I -B generators/f2_spectrum_discrete.py           # check: regenerate in memory, compare byte for byte
python3 -I -B generators/f2_spectrum_discrete.py --write   # rewrite the JSON files and SHA256SUMS
```

The generator needs Python 3 and the standard library only, runs in one
process and takes about 14 s. Its output is deterministic.

- **Hashed, not generated.** `SHA256SUMS` also carries `README.md` and the two
  generators, hashed from disk. The generator's `RECORDS` holds the
  records it verifies by hash but does not generate, as
  `../../f1_modal/mass_discrete` does with its `REFUTATION.md`.
- **Re-freeze rules.** No value changes to match a product output. A
  disagreement goes to the owner of these references and to an independent
  refuter. A change of method, premise or convention is a re-freeze, with a
  stated reason and a new refutation.

## Number format

- **Inputs.** Every input number is a binary64 value. Each file's
  `inputs_binary64` lists each one as its shortest round-trip decimal, its hex
  form and its exact rational. The generator computes with the exact
  rational. The damping ratio is binary64 0.05 =
  3602879701896397/72057594037927936.
- **Working precision.**
  - Where every member length is rational (every case except C9 and
    C9-CHAIN), K and M are formed exactly in ℚ + ℚπ, using f1's field, then
    converted to 80-digit Decimal.
  - Where a length is irrational, the same f1 element functions are evaluated
    in f1's 80-digit Decimal field.
  - Everything after formation is computed at 80 digits.
- **No libm value.** π comes from Machin's series (f1). Square roots, ln and
  exp come from the `decimal` module, which rounds them correctly at the
  working precision. No libm value enters any published number.
- **Published reals.** Each is a pair: the decimal to 40 significant digits,
  then the nearest binary64.
- **Flushing.** In an analysis, a value below 1e-50 of the largest magnitude
  of its unit is printed as `0`. Such values are zero by symmetry or
  decoupling. The generator refuses to write any value lying between 1e-50
  and 1e-35 of that scale. For the member-axes block (R4), the scale is taken
  over the member-axes values.
- **Units.** SI throughout: m, rad, kg, N, N·m, s, Hz and m/s².

## Model formulation (the DEL-04-09 basis, PF route)

- **Members.**
  - Straight Euler–Bernoulli members on a circular annulus.
  - Mass:
    - Hermitian consistent translation mass;
    - linear axial and torsional mass;
    - Rayleigh rotary inertia ī/(30L)·[…].
  - Contents count in all three translations and are excluded from j̄ and ī
    (j̄ = ρ_m·J_metal, ī = j̄/2).
  - Stiffness: the 12×12 with A, I and J = 2I on the wall t − c_m.
  - These are exactly f1's `straight_mass` and `straight_stiffness` (see the
    f1 README for the bending-plane sign map). The assembled K is checked
    against f1's `form_stiffness`.
- **Section `s1`** (C1 to C7, C10), invented:
  - OD = 0.21875, t = 0.0078125, c_m = 0.0009765625 and c_a = 0.001953125,
    all in m;
  - contents 1000 kg/m³, no insulation;
  - steel: E = 2e11 Pa, G = 7.8125e10 Pa, ρ_m = 7850 kg/m³.
- **Section `s2`** (C8, C9, C9-CHAIN), invented:
  - OD = 0.0603 and t = 0.00391, both binary64, in m;
  - c_m = c_a = 0;
  - metal only: no contents, no insulation;
  - the same steel.
- **Supports.**
  - Rigid supports (anchors, guides, directional restraints) restrain global
    DOFs.
  - A linear spring on a global translation adds k on the diagonal.
  - A skewed spring adds k·d dᵀ/(dᵀd) to the node's translations.
  - A snubber is a restraint of the dynamic stiffness; every solve here is
    dynamic. A rigid snubber on a global axis restrains that DOF. A finite
    snubber adds k·d dᵀ/(dᵀd).
  - A rigid snubber along a skew d is the exact constraint dᵀu = 0. The free
    translations of its node are replaced by two rational vectors spanning
    d⊥: w1 = d × e_m, with m the axis of the smallest |d_m| and the lowest
    such index, and w2 = d × w1.
- **Reduced eigenproblem.** K_r = TᵀK_ffT and M_r = TᵀM_ffT, with T that
  basis and the identity elsewhere. Every published vector is the physical
  vector u = Tq on all DOFs, with zeros at restrained DOFs.
- **Geometry.**
  - **Axis-aligned members** (every case except C9 and C9-CHAIN): lengths
    are rational and each member's frame is a signed permutation of the
    global axes.
  - **C9 and C9-CHAIN** lie on the space diagonal. Their node coordinates
    (t_k, t_k, t_k) are inputs: t_k is the binary64 nearest to 15k/(8√3),
    rounded correctly through 60-digit decimal arithmetic. The member lengths
    are √3·Δt, equal to 1.875 m only to rounding. Their frames have the rows
    (1,1,1)/√3, (−1,−1,2)/√6 and (1,−1,0)/√2, which are not a signed
    permutation.
  - Every member's frame (x from i to j, y = y_reference made normal to x,
    z = x × y) is published in `derived_model.member_frames`.

## Method (DEL-04-10 CLM-001, as applied)

### Modal basis (DEL-04-09 REQ-005)

- **Eigenpairs.** Cholesky K_r = RRᵀ, then cyclic Jacobi on R⁻¹M_rR⁻ᵀ at 80
  digits (f1 `route_b`), followed by a Rayleigh quotient. Blocks that K_r and
  M_r leave exactly uncoupled (exact zeros), such as the planes of a straight
  pipe along X, are solved separately. Every finite mode is computed.
- **Two kinds of grouping**, published under two keys:
  - `modal_basis.clusters` are **exact-repeat groups**: eigenvalues equal
    within 1e-30 relative, the bending pairs of a circular section in C1,
    C5A and C7. The basis of such a group is arbitrary; the canonical f1 basis
    is used. No shape, normalization or per-mode Γ is published for a group of
    more than one mode, only its frequency and effective-mass sums.
  - `modal_basis.scope_clusters` are **the scope's clusters** (DEL-04-09
    REQ-002): adjacent computed eigenvalues chained while
    λ_{i+1} − λ_i ≤ δ_c·λ_{i+1}, with δ_c = 2⁻²⁰. Each lists its modes and
    its standing (lowest) frequency.
- **Every combination uses the scope's clusters**; see
  [Repeated modes and clusters](#repeated-modes-and-clusters-the-adopted-rule).
- **Where the two groupings coincide.** They coincide in every case except:
  - **C9:** its unextracted pairs (25, 26), (27, 28), (30, 31) and (32, 33)
    are distinct modes inside δ_c. Over the extracted modes the two
    coincide.
  - **C10:** distinct modes inside δ_c, by design.

  The generator asserts each of these statements.
- **Refused gaps.** The generator refuses an extracted gap between δ_c/2 and
  2δ_c, where a binary64 computation might chain differently.
- **Normalization of a mode not in an exact-repeat group.**
  - Unit maximum translation, the largest component +1.
  - A mode with no translation uses unit maximum rotation instead: case (b),
    when the largest translation ≤ τ_t·ℓ·(largest rotation), with
    τ_t = 1e-6 and ℓ the bounding-box diagonal.
  - Ties within 1e-25 relative go to the lowest DOF index and are listed in
    `normalization_tie_dofs`.
  - Ties occur only in C9 and C9-CHAIN: 25 modes each, with |uy| = |uz| at B,
    by the model's y ↔ z symmetry. Their shapes' and Γ's signs follow the
    tie rule; their contributions do not depend on it.
- **Participation.**
  - Generalized mass m_i = φᵀMφ.
  - p_k = (M R_k)_f, with R_k the rigid unit translation of all DOFs.
  - Γ_ik = φᵀp_k/m_i and m_eff = (φᵀp_k)²/m_i.
  - T_k = R_kᵀMR_k.

### Spectra

- **The table.** Ordinates are pseudo-spectral accelerations in m/s². The
  abscissa is the table's own: frequency in Hz, or period in s, in which case
  x = 1/f_i.
- **Interpolation.** For x in [x₁, x₂]:
  - t = (x − x₁)/(x₂ − x₁) for a linear abscissa, or (ln x − ln x₁)/(ln x₂ − ln x₁)
    for a logarithmic one;
  - S = S₁ + t(S₂ − S₁) for a linear ordinate, or exp(ln S₁ + t(ln S₂ − ln S₁))
    for a logarithmic one.
- **Range.** Nothing is extrapolated. A rigid mode outside the table is
  marked as such and never used.
- **The four rules:**
  - linear/linear: `S-LL`;
  - log/log: `S-GG`;
  - period abscissa, linear in period and log ordinate: `S-PLG`;
  - log abscissa, linear ordinate: `S-GL`, used in C4.

  `S-FLAT` is linear/linear and constant above 50 Hz. The C8 and C9 tables
  are described with their cases.

### Modal responses

- **Displacement.** u_ik = Γ_ik·s_k·S_k(f_i)/ω_i²·φ_i on all DOFs, relative
  to the supports. It does not depend on the normalization.
- **Recovery.** Every quantity is recovered from u and the inertia field
  g = ω_i²·u, the inertia load being F = Mg:
  - **Element end forces** f_e = K_e·u_e − M_e·g_e, here (K_e − ω²M_e)u_e.
  - **Rigid restraints:** the residual r = K·u − M·g at the restrained DOF,
    here (K_cf − ω²M_cf)u. K is the total dynamic stiffness. No compliant
    support sits on a restrained node.
  - **Rigid skew snubber:** r at its node's three translations, which is
    parallel to d.
  - **Compliant supports** (springs, finite snubbers): −K_s·u.

### Rigid response (missing-mass correction)

- **Periodic set.** The modes of a scope cluster whose lowest frequency is
  below f_r are all periodic. The modes of every other cluster, extracted or
  not, are all rigid.
- **The static solve.** For each direction:
  - a_r,k = s_k·S_k(f_r);
  - g = a_r,k·(R_k − Σ_periodic Γ_ik·φ_i) and F_k = M·g, over all DOFs;
  - solve K_r·q = TᵀF_f, then u = Tq.
- **Recovery.** By the same formulas: f_e = K_e·u_e − M_e·g_e, where M_e·g_e
  is the element's own share F_e of F_k, and reactions K·u − F_k at the
  restrained DOFs.

### Combinations

They are applied per response quantity. R_i is the signed periodic value of
one combination unit: a scope cluster, whose modes' contributions are summed
with their signs (a single mode is a cluster of one).

- **Each mode keeps its own frequency** f_i and ω_i² for its spectral
  ordinate, its displacement and its recovery.
- **The cluster's lowest frequency** stands for the cluster only in ρ, in the
  closeness tests of GROUPING and PAIRWISE_TEN_PERCENT, and in the
  periodic/rigid classification.

| Rule | Formula |
|---|---|
| SRSS | √(Σ R_i²) |
| CQC | √(Σ_i Σ_j ρ_ij R_i R_j), ρ_ij = 8ζ²(1 + r)r^{3/2} / ((1 − r²)² + 4ζ²r(1 + r)²), r = ω_j/ω_i (symmetric in i, j) |
| GROUPING | units ascending; a group starts at its lowest f_g and takes each next unit with (f − f_g)/f_g ≤ 0.1; √(Σ R_i² + 2 Σ_groups Σ_{i<j} \|R_i R_j\|) |
| rigid with periodic | SRSS √(R_p² + R_r²), or absolute sum R_p + \|R_r\| |
| directional | SRSS √(Σ R_d²), or 100-40-40: max_a (R_a + 0.4 Σ_{b≠a} R_b) with 0.4 = 2/5 exactly |
| *not in the scope today* (C3, C4, C8, C9, C9-CHAIN, C10) | ABSOLUTE_SUM over units Σ\|R_i\|; PAIRWISE_TEN_PERCENT √(Σ R_i² + 2 Σ_{i<j, (f_j − f_i)/f_i ≤ 0.1} \|R_i R_j\|); ABSOLUTE_SUM over directions |

The generator checks that no frequency ratio lies within 1e-6 of the 10 %
boundary.

## Conventions a product must match or map

These are fixed here, because the scope does not fix them in words, and are
applied in every file as stated.

1. **Reaction sign.** A reaction is the generalized force that the support
   applies **to the piping**, in global axes: the residual
   (K_cf − ω²M_cf)u, or K·u − F, at rigid restraints, and −K_s·u at
   compliant supports.
   - The total reaction along k in the rigid response is
     −a_r,k·(T_k − Σ_periodic m_eff), and −a_r,k·T_k when no mode is
     periodic.
   - A product reporting the force on the support has the opposite sign of
     every signed reaction. Combined magnitudes are unaffected.
2. **End-force frame and ordering.**
   - Every element end force published in the existing blocks, signed and
     combined, is in **global axes**. It is the 12 generalized forces acting
     **on the element** at its end nodes: node i first, then node j, each in
     the order Fx Fy Fz Mx My Mz. Keys are `P1.i.Fx` and so on.
   - An internal section force at end i is the negative of the end force
     there.
   - **Combined magnitudes depend on the axes.** A combined magnitude is a
     property of the axes it is combined in. A product working in member axes
     must rotate each signed contribution to global axes before combining, to
     compare with these values.
   - Where a member's frame is a signed permutation of the global axes (every
     case except C9 and C9-CHAIN), the combined magnitudes map component to
     component.
   - **C9 and C9-CHAIN** also publish `element_end_forces_member_axes`: the
     signed contributions rotated into each member's own axes (frame stated),
     and the magnitudes combined from them under every published rule. Keys
     are `P1.i.Fx_m` and so on, units in `derived_model.member_axes_quantities`.
     The two sets differ, because these frames are not signed permutations and
     combination does not commute with the rotation.
   - C9-CHAIN publishes no global-axis end forces: its global blocks are
     limited to B, SA and S1.
3. **Grouping boundary.**
   - GROUPING's closeness test is inclusive, (f − f_g)/f_g ≤ 0.1, with f_g the
     group's lowest frequency. PAIRWISE_TEN_PERCENT uses
     (f_j − f_i)/f_i ≤ 0.1, also inclusive.
   - In both, a scope cluster is one unit represented by its lowest
     frequency.
   - No unit lies within 1e-6 of the boundary.
4. **100-40-40 with fewer than three directions.** The rule is
   max_a (R_a + 0.4 Σ_{b≠a} R_b) over the directions requested. With two
   directions this is max(R_a + 0.4R_b, R_b + 0.4R_a), as in C4 and C10; with
   one it is R_a.
5. **Interpolation abscissa.** Interpolation is on the table's own abscissa:
   period when the table is entered in period (`S-PLG`), frequency otherwise.
   A logarithmic axis gives the same ordinate in either; a linear one does
   not.
6. **Contributions of extracted rigid modes.**
   - An extracted rigid mode whose frequency lies in the table's range has its
     per-mode contribution published, with `status: rigid (extracted)`. One
     outside the range has its ordinate published as null and no contribution.
   - These contributions are the data for the correction-off variant
     (`periodic_modal_combined_correction_off`, and the OFF mutations). The
     base result never uses them.
7. **The fraction moving with the supports** is 1 − p_kᵀM_ff⁻¹p_k/T_k: the
   mass no finite mode reaches. That includes mass at restrained DOFs, its
   consistent coupling, and mass held by a rigid skew restraint at a free
   node.

**Published components of supports and skew records** (as in
`derived_model.quantities`):

- **Rigid support** (anchor, guide, directional restraint, rigid axis
  snubber): one component per restrained DOF, `.Fx` … `.Mz`.
- **Axis spring or finite axis snubber:** its one component, for example
  `S1.Fy`, `SN1.Fz`.
- **Finite skew spring or snubber:** the global components `.Fx`, `.Fy`,
  `.Fz` only where d has a nonzero entry, plus `.Fd`, the signed component
  along d/|d|. A component where d is zero is identically zero and is not
  published. Examples: K1 and K2 of C4 have no `.Fz`, SN2 of C6B has none,
  and K1 of C10 has no `.Fx`.
- **Rigid skew snubber** (SN2 of C6A): all three of `.Fx`, `.Fy`, `.Fz`,
  zero where d is zero, plus `.Fd`, plus the node's motion:
  - `.u_along_d`, zero in every response;
  - `.u_perp1` and `.u_perp2`, along the unit vectors in
    `derived_model.skew_snubber_motion_axes_unit`.

**Quantity keys.**

- Displacements: `N2.ux` … `N2.rz`, for every node; zero at restrained DOFs.
- Reactions: `<support id>.<component>`.
- The list and units are in `derived_model.quantities`.

**C8-III-TRUNCATION extracts 3 modes, not 2.** With f_r between modes 2 and
3, a 2-mode set does not reach f_r, which REQ-001's extraction premise
refuses (`SPECTRUM_EXTRACTION_BELOW_RIGID_FREQUENCY`). Mode 3 is a rigid
extracted mode and enters nothing. So every published number equals what a
2-mode request would give if it were admitted.

## Contents of an analysis

- `request`: the spectra, scale factors, ζ and every method choice. Nothing
  is defaulted.
- `mode_status` and `spectral_ordinates`: per direction, S(f_i) and
  s·S(f_i).
- `rigid_acceleration_m_per_s2`.
- `contributions_signed`: per direction and mode, every quantity. A mode in
  an exact-repeat group appears only through the group's signed sum.
- `rigid_response_signed`: per direction.
- `cqc_correlation`: ρ over the extracted scope clusters.
- `grouping_groups`: for the periodic set and for the correction-off set.
- `periodic_modal_combined` and `periodic_modal_combined_correction_off`:
  each rule, per direction.
- `rigid_magnitude`.
- `per_direction_combined` and `combined`: the request's own rules.
- `mutations`:
  - **C3 and C4.** The 18 in-scope combinations: modal SRSS, CQC or GROUPING;
    correction SRSS, ABSOLUTE_SUM or OFF; directional SRSS or 100-40-40.
    Plus three labelled *not in the scope today*, each replacing one choice
    of the base request.
  - **C8, C9, C9-CHAIN, C10.** A grid of every modal rule × the stated
    correction and directional rules. Each entry has `in_scope`; an entry out
    of scope is labelled *not in the scope today*.
- `missing_mass_disclosure`, per direction:
  - T_k;
  - the periodic cumulative effective-mass fraction, and the fraction it
    does not represent;
  - the same with every extracted mode periodic (correction off);
  - the all-finite-modes fraction, and the fraction moving with the supports.
- `cross_checks`.
- **Case-level blocks:**
  - `frequency_list` (C8, C9, C9-CHAIN, C10): every mode, ascending, with its
    scope cluster (and its plane, in C8); the smallest relative eigenvalue
    gap; adjacent pairs within 1 % in frequency; and adjacent pairs within
    δ_c;
  - `mesh_series` (C8).
- **Analysis-level blocks:**
  - `single_direction_check` (C8);
  - `close_pair` (C9);
  - `chain` (C9-CHAIN);
  - `element_end_forces_member_axes` (C9, C9-CHAIN);
  - `basis_witness` (C7);
  - `cluster_witness`, `not_the_adopted_reading_mode_by_mode`,
    `rigid_load_signed` and `comparability` (C10).

## Cases

| Case | Model | Request | Serves |
|---|---|---|---|
| C1A-AXIAL | one member 4 m along X, N1 anchored, N2 free | X, no correction, all 6 modes | AC-002 consistent-mass witness: axial mode Γ = 3/2, m = T/3 (free mass 1/3, coupling 1/6, excitation 1/2), m_eff = 3T/4; reaction −(3/4)T·S, which contains −ω²M_cf·u |
| C1A-AXIAL-MM-ALL-PERIODIC | same | X, f_r = 320 Hz above every mode | the rigid response with every mode periodic: free-DOF load 0, u = 0, reaction −a_r·T/4 (the mass moving with the support) |
| C1B-RESTRAINED-MASS | same | X, Y, Z; f_r = 4 Hz below every mode; rigid ABSOLUTE_SUM | AC-002 restrained-mass witness: total reaction −a_r,k·T_k in each direction, including the mass at the restrained DOFs |
| C1C-TRANSVERSE | same member, tip guided (uz, ry) | Y, f_r = 40 Hz | transverse one-element response; extracted rigid modes 2–4 in the static solve |
| C2-SPRING | two members along X, spring 262144 N/m on uy at N3 | Y, f_r = 33 Hz, log/log | AC-002 ground-spring reaction beside the anchor reaction, in equilibrium per mode and in the rigid response |
| C3-BASE | four members along X, Y, Z, X from an anchor at N1 (24 DOF) | X `S-LL` ×1, Y `S-GG` ×1, Z `S-PLG` ×0.75; CQC; directional SRSS; f_r = 70 Hz, 8 modes extracted (modes 7 and 8 rigid); rigid SRSS | AC-002 multi-DOF modal responses, every combination, the rigid response with extracted rigid modes, the correction off and its disclosure; mutation data |
| C4-BASE | vertical cantilever 3 m, skew springs at N3: 131072 N/m along (3,4,0) and 196608 N/m along (−4,3,0) | X ×1, Y ×0.75, `S-GL`; CQC; 100-40-40; f_r = 60 Hz; rigid ABSOLUTE_SUM | AC-002 CQC on a close pair (18.75 and 19.57 Hz, ρ₁₂ = 0.843): N3.uy, A1.Fy and A1.Mx get opposite-sign contributions under X, so CQC < SRSS; N3.ux, A1.Fx and A1.My have CQC > SRSS; mutation data |
| C5A-RIGID | vertical cantilever 0.5 m (lowest mode 594.8 Hz) | X, Y, Z, `S-FLAT`; f_r = 100 Hz below every mode | AC-002 stiff model: response = the static response to a_r·M·R_k, shown equal to the full modal expansion over every finite mode |
| C5B-L | L line: 4 m along X, then 3 m along Y, anchors at both ends | as C3's spectra; CQC; 100-40-40; f_r = 120 Hz; rigid ABSOLUTE_SUM | AC-002 piping-shaped case, in plane (X, Y) and out of plane (Z); the unsnubbered basis of C6 |
| C6A-L-RIGID | C5B plus a rigid snubber on uz at N2 and a rigid snubber along (3,4,0) at N4 | as C5B | AC-003 rigid axis and skew snubbers: the modal basis with them (and without them in `modal_basis_without_dynamic_only_restraints`); `SN2.u_along_d` = 0 with `u_perp1`/`u_perp2` free; a rigid response that differs from C5B's |
| C6B-L-FINITE | C5B plus snubbers of 1048576 N/m on uz at N2 and 524288 N/m along (3,4,0) at N4 | as C5B | AC-003 finite snubbers, k·d dᵀ/(dᵀd) |
| C7-PAIRS | two members along X, tip free: exact repeated bending pairs | Y ×1, Z ×0.5; correction off; 4 modes (two pairs) | basis-invariant data only; reading B's SRSS range; W1 |
| C8-I-FLAT | `s2` pipe 15 m along X, 8 members; anchor SA at A; SB at the tip B restrains uz only (Y plane a cantilever, Z plane a propped cantilever) | Y and Z ×1, `S-C8-FLAT` (2.5 m/s² from 0.125 to 1000 Hz); 6 modes, correction off; base CQC and directional SRSS; grid: every modal rule × directional SRSS and ABSOLUTE_SUM | benchmark shape. Each quantity receives contributions from one direction only, so every directional rule agrees (`single_direction_check`) |
| C8-II-TWO-LL, C8-II-TWO-GG | same | the two-segment table (0.125, 1.5), (2, 4.5), (16, 2.25), linear/linear and log/log | the same, on a sloped table |
| C8-III-TRUNCATION | same | `S-C8-TRUNC`: (0.125, 1.25), (0.75, 3.5), (1000, 3.5), so flat from below mode 2; f_r = 1.25 Hz between modes 2 (1.100 Hz) and 3 (1.572 Hz); rigid SRSS, and ABSOLUTE_SUM in the grid | truncation with the correction: 2 periodic modes and the rigid response |
| C8 `mesh_series` | the C8 pipe with N = 2, 4, 8 and 16 members | as C8-I-FLAT | the first six frequencies with their planes; combined B.uy, B.ry, B.rz, SA.Fy, SA.Fz, SA.My, SA.Mz and SB.Fz under every modal rule |
| C9-Y | `s2` pipe 15 m from the origin along (1,1,1), 8 members, anchor SA at A, spring S1 of 8 N/m along global X at B | Y ×1, `S-C9-FLAT` (2.5 m/s²); 6 modes, correction off; base CQC; grid: every modal rule | close pair 0.2509 / 0.2637 Hz (5.1 %), ρ₁₂ = 0.8004 at ζ = 0.05. Same sign: B.uy, B.rz, SA.Fy, SA.Mz (CQC > SRSS). Opposite sign: B.uz, B.ry, SA.Fz, SA.My (CQC < SRSS). Mode 1 is zero in B.ux and SA.Fx; mode 2 is zero in B.rx and SA.Mx (`close_pair`) |
| C9-CHAIN-Y | the C9 pipe with the tip spring at 1048576 N/m | as C9; 23 modes; global-axis quantities B, SA and S1 only | modes 21–23 (79.73, 85.64, 91.09 Hz) form a chain, so GROUPING and PAIRWISE_TEN_PERCENT differ (`chain`) |
| C10-W2 | the C7 model plus a skewed spring of 0.0625 N/m along (0,3,4) at N3 | Y ×1, Z ×0.5, `S-LL`; 4 modes, correction off; grid: every modal rule | W2: two scope clusters of two distinct modes, relative eigenvalue gaps 2.53e-7 and 6.45e-9 |
| C10-W3 | same | the same with f_r = 9.5395413 Hz strictly between modes 1 and 2; rigid SRSS, and ABSOLUTE_SUM in the grid | W3: a cluster straddling f_r is periodic as a whole |

**Which extracted modes are simple.** In every case except C1, C5A, C7 and
C10, every extracted mode is simple and alone in its scope cluster.

- In C1 and C7 the extracted modes include exactly repeated pairs. C1's
  bending pairs carry no X participation, and C1B has no periodic mode.
- C5A's two extracted modes are an exactly repeated pair, both rigid.
- C10's clusters hold distinct modes.

**The smallest relative gap between extracted distinct frequencies**,
(f_{i+1} − f_i)/f_i:

| Case | C1, C1C | C2 | C3 | C4 | C5B | C6A | C6B | C8 | C9 | C9-CHAIN |
|---|---|---|---|---|---|---|---|---|---|---|
| Smallest gap | 0.15 | 0.014 | 0.076 | 0.0014 (the rigid extracted modes 3 and 4; 0.044 for the pair) | 0.049 | 0.025 | 0.048 | 0.24 | 1.7e-4 | 0.029 |

Unextracted modes can be closer. They enter only the cross-checks and the
published Γ.

**Pairs within 1 % in C9.**

- **Extracted.** The pairs (3, 4) at 1.5723 / 1.5744 Hz (0.13 %) and (5, 6)
  at 4.4047 / 4.4055 Hz (0.017 %) lie within 1 %, but far outside δ_c. Their
  directions are fixed by the model's y ↔ z symmetry about the plane of the
  diagonal and global X.
- **Unextracted.** Exactly four pairs lie inside δ_c: (25, 26), (27, 28),
  (30, 31) and (32, 33) (`frequency_list.adjacent_pairs_within_delta_c`). No
  request extracts them.

**Grouping against the ten-percent rule.** C3, C4, C8 and C9 have no three
units in a chain, so GROUPING and PAIRWISE_TEN_PERCENT agree there. C9-CHAIN
is the case where they differ.

**What is not here.** AC-002 also lists the single-degree-of-freedom response
under each interpolation rule, the pseudo-acceleration convention and a
damped force case. Those are in `../closed_form/`. Here every rule is
exercised on frame models, and the forces are recovered at the displacement
extremum, with the velocity terms neglected as REQ-002 states.

## Repeated modes and clusters (the adopted rule)

**The rule.**

- **Repeated eigenvalues.** For an exactly repeated eigenvalue, every
  M-orthogonal basis of its eigenspace gives the same signed sum of its
  modes' contributions, whatever each shape's normalization.
- **Clusters.** A cluster is adjacent computed eigenvalues chained by
  λ_{i+1} − λ_i ≤ δ_c·λ_{i+1}, with δ_c = 2⁻²⁰. It is a numerical
  convention, not a statement that the eigenvalues are equal.
- **Combination.** The modes of a cluster are summed with their signs first,
  and every modal rule is applied to the cluster sums.
- **Frequencies.** Each mode keeps its own f_i and ω_i² for its ordinate,
  displacement and recovery. The cluster's lowest frequency is used only for
  ρ, for the closeness tests, and for the periodic/rigid classification.

**This package applies the rule.**

- Every `*_combined` block and every mutation entry is formed this way, on
  `modal_basis.scope_clusters`.
- ρ uses the cluster's lowest ω (`cqc_correlation`, by cluster).
- GROUPING and PAIRWISE_TEN_PERCENT test the clusters' lowest frequencies
  (`grouping_groups`).
- The periodic set takes whole clusters (`mode_status`).
- The ordinate, displacement and recovery of each mode use its own f_i and
  ω_i² (`spectral_ordinates`, `contributions_signed`).

**Why the rule matters: C7, exact pairs, with the two readings kept as data.**
Take an exactly repeated pair with M-orthonormal vectors a and b. For one
response quantity with linear recovery L, let α = (aᵀp_k, bᵀp_k),
β = (L·a, L·b) and C = s_k·S_k/ω².

- The pair's sum is invariant: C·αᵀβ.
- Plain SRSS over the individual vectors is not. With A = (αβᵀ + βαᵀ)/2,
  SRSS² ranges over [C²(tr A)²/2, C²‖A‖_F²] as the basis rotates.
- CQC is invariant (ρ = 1 inside the pair).
- GROUPING is not.
- `repeated_cluster_readings` keeps reading A (the adopted rule) and reading
  B's SRSS range. Under Y, reading A gives N3.uz = 0 and A1.Fz = 0, while
  reading B can report N3.uz up to 3.197e-3 m and A1.Fz up to 1251 N.

**W1 (C7, `basis_witness`), the first pair under Y and Z.**

- **basis_1:** the canonical M-orthonormal basis, then normalized to unit
  maximum translation.
- **basis_2:** that M-orthonormal basis rotated by cos = 3/5, sin = 4/5, then
  normalized the same way.
- **What differs and what does not.** The generalized masses (68.68 kg
  against 107.31 kg) and the participation factors differ between the bases.
  The cluster sums agree, and equal the published sum.
- **The Euclidean rotation of basis_2's normalized vectors.** It stays
  M-orthogonal and gives the same sums, because its two vectors have equal
  generalized masses (the pair is isotropic).
- **The counter-witness, outside the premise:** a sheared basis
  (basis_2's v1, and v2 + v1/2). It spans the eigenspace but is not
  M-orthogonal (v1ᵀMv2 = 39.0 kg), and its sums differ, by 0.33 of the unit
  scale.
- `basis_witness.comparability` names what is comparable: the sums, but no
  per-vector value.

**W2 (C10-W2), distinct modes inside one cluster.**

- **The modes.** The skewed tip spring splits each pair into two distinct
  modes, with relative eigenvalue gaps of 2.53e-7 (9.5395407 / 9.5395419 Hz,
  on S-LL's sloped segment) and 6.45e-9.
- **Published** (`cluster_witness`): each mode's own ordinate and signed
  contribution, and the cluster sum.
- **Labelled "not the adopted reading"**
  (`not_the_adopted_reading_mode_by_mode`): the same modes combined mode by
  mode. Combined under Y and Z with directional SRSS, against the adopted
  SRSS:

  | Quantity | Adopted SRSS | Mode by mode | Ratio |
  |---|---|---|---|
  | N3.uy | 4.522e-3 m | 3.658e-3 m | 0.81 |
  | N3.uz | 2.261e-3 m | 3.489e-3 m | 1.54 |
  | A1.Fz | 884.6 N | 1365.4 N | 1.54 |
  | A1.Mz | 5085.9 N·m | 4114.2 N·m | 0.81 |

  CQC with the modes' own frequencies agrees with the adopted CQC to about
  3e-11 relative, since ρ = 1 − O(1e-12) across the pair.
- **Comparability.**
  - Per-mode values of two modes in a cluster this close cannot be compared
    with a correct product at the verification tolerance. The basis within a
    near-degenerate cluster is ill conditioned, and how far a product's pair
    rotates depends on its eigensolver's route.
  - REFUTATION.md (R5) reports, from 16-digit decimal arithmetic used as a
    binary64 stand-in, a rotation of mode 1 of about 1.7e-9 on a route through
    Cholesky of K, and about 4.2e-6 on a route through Cholesky of M.
  - The cluster sums and every combined magnitude on the adopted reading are
    well conditioned, and are what a product is compared against. For
    example, under Y, N3.uz sums to −4.43e-10 m from contributions of
    ±2.17e-3 m.
  - Each C10 analysis lists both groups of blocks in `comparability`.

**W3 (C10-W3), a cluster straddling f_r.**

- **Classification.** f_r = 9.5395413 Hz lies strictly between modes 1 and 2.
  The cluster's lowest frequency is below f_r, so both modes are periodic:
  their sum enters the periodic part, and both are removed from the rigid
  load.
- **Published:** `rigid_load_signed`, the rigid response, the fractions, and
  both rigid rules (`mutations`).
- **Equilibrium.** The rigid total reaction balances
  −a_r,k·(T_k − Σ over both modes of m_eff).

## Cross-checks

Each check is a second computation, derived differently. The table gives the
largest result over all cases.

| Check | Largest result |
|---|---|
| K assembled here = f1 `form_stiffness` (+ axis springs) | exactly equal in every rational case; 4.1e-83 relative in C9 and C9-CHAIN |
| Sturm counts (LDLᵀ of K_r − σM_r) at λ₁/2, the geometric mean of each gap, and 2λ_max = the number of eigenvalues below | all agree |
| Eigen residual ‖K_rq − λM_rq‖∞/(‖K_r‖∞‖q‖∞) | 7.2e-70 |
| M-orthogonality | 8.7e-69 |
| Σ m_eff over every mode vs p_rᵀM_r⁻¹p_r | 5.4e-72 relative |
| Every mode periodic: free-DOF load p − Σ Γ·M·φ | 5.4e-71 relative |
| Nodal balance: assembled element end forces vs the support forces at each DOF | 4.5e-70 relative |
| Global equilibrium: Σ support forces and moments (with x × F) vs −R_jᵀMg and −Θ_jᵀMg, per mode and per rigid response | 3.1e-71 relative |
| Rigid response: static solve vs a_r·Σ_{every non-periodic finite mode} Γ·φ/ω² | 9.0e-77 relative to the full static response |
| Rigid response: total reaction along k vs −a_r,k·(T_k − Σ_periodic m_eff) | 6.4e-78 relative |
| Rigid skew snubber: \|dᵀu\|/(\|d\|·max\|u_node\|) | 4.0e-80 |
| C7 SRSS range (closed form) vs explicit rotated bases through the full recovery | 3.3e-79 relative |
| C8: directional SRSS vs ABSOLUTE_SUM | 1.1e-80 relative to the unit scale |
| W1: cluster sums, basis 1 vs basis 2 vs the Euclidean rotation vs the published sum | 6.3e-79 relative to the unit scale |

## Out of scope

- Independent support motion (REQ-011).
- Every static load case, and a snubber's absence from it (AC-003, product
  side).
- Refusals (AC-001, AC-005).
- Comparison tolerances. These references state only their own cross-check
  residuals.
