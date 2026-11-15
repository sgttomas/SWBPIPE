# F2 response-spectrum references W2: realized arc, face pipes, massless connector and rods

These are candidate references for DEL-04-10, the response-spectrum analysis
([scope](../../../../execution/PKG-04/DEL-04-10/ScopeOfWork.md); REQ-002 to
REQ-004 and REQ-008), on DEL-04-09's modal basis. They cover the signed
periodic recovery and the missing-mass recovery, both inertia-inclusive,
of one small invented model family:
- a realized 90° Euler–Bernoulli arc, whose half-arc lump inertia is
  attributed to its ends, with moments and end-tangent frames;
- massive straight face pipes;
- a massless objective connector that replaces a span;
- two explicit offset linear rods.

The sibling packages `../discrete/`, `../closed_form/` and `../benchmark_pack/`
cover straight members and the combination rules.

**Status: candidate, not refuted.**
- No refutation record exists yet. A separate, source-independent refuter
  commissioned by the integrating agent examines these files.
- Until then the outputs are reviewable and may be corrected. After that
  boundary, a correction is a recorded re-freeze, never a silent rewrite.
- No product output has been compared with them.
- They establish no acceptance, issuance or code compliance.

Standard claim fence applies (claims taxonomy per DEC-081).

## Independence and provenance

- **What the author read.** The author read no product implementation:
  nothing under `core/`, `apps/`, `api/`, `schemas/`, `tests/` or `tools/`, no
  fixture JSON and no product output. It read:
  - the DEL-04-09, DEL-04-10, DEL-03-03 and DEL-03-06 scopes;
  - the release target;
  - the READMEs of `../discrete/`, `../../f1_modal/mass_discrete/`,
    `../../t4_u4a/`, `../../t4_i7/`, `../../t4_i12/` and `../../t4_u5b/`;
  - the connector functions of `../../t4_i12/generators/u3_reference.py`.
- **What the generator reads and imports.** It imports, reads and runs no
  product code. Besides its own files, it imports four frozen reference-side
  modules of `../../f1_modal/mass_discrete/generators/`, unmodified:

  | Module | SHA-256 | Used for |
  |---|---|---|
  | `elements.py` | `c2a7b890c90d8c2d016fc72036aad36165173cb9f146158bea7629364bd696f0` | straight K and M, arc K, half-arc lumps |
  | `model.py` | `c219d158aa2aa260153dff303514508a4c41e4983b2b1c5236a2beb79c046761` | section bases, `form_mass`, `form_stiffness` (cross-check) |
  | `eigen.py` | `22ae605c36ae47ad7e80369ecbee422646bef67acb1d46294a744228589d17c6` | Cholesky–Jacobi, Sturm counts |
  | `numeric.py` | `e08ff24066f636f0d6c76e38ae95de2ef42e9e1c0f651714910034575d5dd1e7` | ℚ + ℚπ field, Decimal field, π |

  - The generator checks each hash before import and stops on a change.
  - It puts the f1 directory first on the import path, and asserts each
    module's file origin.
  - These are the same pins `../discrete/` uses. The f1 package has its own
    refutation record.
- **The connector's B.** It is re-derived here from the T4-I12 kinematic
  contract restated in that generator's docstring (symmetric midpoint, small
  rotation). It is written as new code; nothing is imported from T4-I12.
- **The rods.** They are re-derived from the T4-U5b rod definition in its
  README.
- **The arc end-tangent rotations.** They are the ones the `../../t4_u4a/`
  README states. `t4_u4a/u4a_generate.py` imports repository tooling
  (`tools.validation`), so it was neither run nor imported.
- **Other material.** No CAEPIPE material, held source, exchange folder or
  external data was used. Every model value is invented.
- **What the parent decided.** The parent (the dynamics Type 1) settled the
  premises of the five questions asked before generation:
  - the U4a end-tangent frames as the second frame for arc ends, with
    global rows kept as independent observables;
  - the connector observables g = K B u and Bᵀg, and the rod T with its end
    actions, labelled as incremental dynamic quantities;
  - the fixed-state exclusion control in its limited form;
  - the dyadic substitutions and the zero allowances;
  - no compliant support.

## Files

| File | Content |
|---|---|
| `generators/w2_recovery.py` | driver and method: assembly, connector and rods, modal basis, spectra, recovery, combinations, controls, publication, check and write |
| `generators/w2_cases.py` | the invented model, variants, spectra, requests and held state |
| `DERIVATION.md` | the mathematics restated, step by step |
| `w2_inputs.json` | every input, with `inputs_binary64` (decimal, hex, rational); the connector's exact derived Q, B, K_c and K_e; the rods' exact offsets, n and b |
| `w2_v_arc.json` | V-ARC: the arc between straights, anchored at A and D |
| `w2_v_joint_0.json`, `w2_v_joint_r.json` | V-JOINT: face pipes and the connector, without and with rods, anchored at C and F |
| `w2_v_comb_0.json`, `w2_v_comb_r.json` | V-COMB: the whole line, without and with rods, anchored at A and F |
| `w2_controls.json` | frequency ladders, the fixed-state exclusion control, the replaced-span mutants, the arc closed-form weight, rounding and precision convergence |
| `SHA256SUMS` | hashes of every file above |
| `REFUTATION.md` | absent; when the refuter supplies it, the generator hashes it and requires it in `SHA256SUMS` |

## Reproduce

```
python3 -I -B generators/w2_recovery.py           # check: regenerate in memory, compare byte for byte,
                                                  # verify SHA256SUMS and complete file membership
python3 -I -B generators/w2_recovery.py --write   # rewrite the JSON files and SHA256SUMS
```

- **Environment.** Python 3 and the standard library only, in one process.
- **Precision.** Each run builds every output twice, at 80 and at 100
  digits, and refuses to proceed unless every published value agrees (see
  Number format). It takes about 7 s.
- **What check verifies.**
  - Every output regenerates byte for byte.
  - `SHA256SUMS` equals what the generator computes over `README.md`,
    `DERIVATION.md`, both generators, the outputs and `REFUTATION.md` when it
    is present.
  - The files present in this directory are exactly the files listed.
- **What `--write` does not touch.** It never writes `README.md`,
  `DERIVATION.md` or `REFUTATION.md`; it only hashes them.

## Number format

- **Inputs.** Every input number is a binary64 value written exactly in
  decimal (dyadic fractions and integers), taken as an exact rational.
  - The exceptions are ζ, the binary64 nearest 0.05 taken exactly, and the
    connector guide, which is integers.
- **Exact formation.**
  - ℚ + ℚπ for the straight-member stiffness (`K_exact`).
  - ℚ for the connector and rods.
  - 80-digit Decimal for the arc, whose chord R√2, frame and half-angle
    functions are irrational, and for everything after formation.
  - π comes from Machin's series and the arc angle from f1's arctangent
    series. Square roots, ln and exp are Decimal's correctly rounded
    operations. No libm value enters any published number.
- **Published reals.** Each is a pair: a 40-significant-digit decimal, then
  the nearest binary64.
  - A value below 1e-50 of the largest magnitude of its unit in its block is
    printed `0`. Such values are zero by symmetry or decoupling.
  - The generator refuses any value between 1e-50 and 1e-35 of that scale.
- **Convergence and rounding.**
  - All 86,516 published values are identical at 80 and 100 digits, in the
    40-digit decimal and in the binary64.
  - The nearest-binary64 rounding of each published value is decided with a
    relative margin of at least 1.47e-21 from the nearest rounding boundary.
    The generator requires more than 1e-60.
- **Units.** SI: m, rad, kg, N, N·m, s, Hz and m/s².

## The model (invented)

| Item | Value |
|---|---|
| Section `s3` | OD 0.171875 m, wall 0.0078125 m, c_m = c_a = 0, contents 1000 kg/m³, no insulation |
| Steel | E 2e11 Pa, G 7.8125e10 Pa, ρ 7850 kg/m³ |
| Nodes | A (0,0,0), B (1.5,0,0), C (1.875,0.375,0), D (1.875,1.375,0), E (1.875,1.875,0), F (1.875,3.125,0) |
| P1 | straight A–B, 1.5 m along X |
| BEND1 | realized 90° arc B–C, R = 0.375 m, centre (1.5,0.375,0), y_reference (1,−1,0), one flexibility factor k = 4.25 |
| P2, P3 | face pipes C–D (1.0 m) and E–F (1.25 m) along Y |
| J1 | massless connector replacing span D–E (0.5 m): zero offsets, Ls = 1 m, Q from guide (4,0,3), H = diag(150e3, 400e3, 250e3 N/m; 500e3, 1.5e3, 1.0e3 N·m/rad) |
| R1, R2 | linear rods D–E at ±0.1875·Q_y, k = 1.2e8 N/m, e_ref = 0 |

**Why these values.**
- They are dyadic, so every coordinate, sum, chord and offset is exact in
  binary64. They take the place of the reference-system inspirations R 0.4 m,
  k 4.2 and rod offset 0.2 m, which are not exactly representable.
- P2 and P3 are unequal, so the joint variants have no mirror symmetry that
  would zero the connector forces.
- The connector's unequal lateral and bending terms keep the rods-absent
  straight line free of exactly repeated pairs, without adding a support. They
  also make the rotated frame enter K_e itself.
- k is one factor on both bending terms, as f1 forms it. Directional k is
  not covered.

## Variants, requests and what each shows

| Variant | Free DOFs | Extracted (f_c) | Periodic (f_r) | Shows |
|---|---|---|---|---|
| V-ARC | 12 | 6 (560 Hz) | 4 (400 Hz) | arc end actions with lump inertia, in global and end-tangent frames; half-arc checks |
| V-JOINT-0 | 12 | 5 (768 Hz) | 4 (500 Hz) | connector g and Bᵀg between massive face pipes; two close pairs grouped (86.46/88.17 Hz, 133.0/135.3 Hz; ρ = 0.963 and 0.972) |
| V-JOINT-R | 12 | 5 (768 Hz) | 4 (500 Hz) | rods present: rod T and end actions beside the connector's g |
| V-COMB-0 | 24 | 6 (128 Hz) | 4 (72 Hz) | the full coupled path without rods; one grouped pair (20.18/21.55 Hz) |
| V-COMB-R | 24 | 5 (128 Hz) | 3 (72 Hz) | the full W2 model |

- **Spectra and directions.** Every request uses directions X (`S-W2-LL` ×1),
  Y (`S-W2-GG` ×0.75) and Z (`S-W2-PLG` ×0.5) and ζ as above.
- **Rules.** The base rules are CQC, rigid SRSS and directional SRSS. The
  mutation grid covers 5 modal × 3 rigid × 3 directional rules.
- **Modes.** Every scope cluster is one mode. Extracted rigid modes enter only
  the correction-off data (`periodic_modal_combined_correction_off` and the
  OFF mutations) and the static solve's modal-expansion cross-check.
- **Torsion modes.** The joint variants' modes 5 and 9 are pure torsion,
  normalized to unit maximum rotation, with zero participation.

**Diagnosis chain.** A product that disagrees on V-COMB can localize the
difference:
- V-ARC isolates the arc and its lumps;
- V-JOINT-0 and V-JOINT-R isolate the connector and the rods;
- the 0/R pairs isolate the rods.

## Contents of a variant file

- **`derived_model`.**
  - The DOF order, and the free and restrained DOFs.
  - The section quantities (EX equal to PF, with the exact coefficients of π).
  - Every stiffness constituent's K. Straights also give `K_exact`. The arc
    gives its chord frame, s, c, both chord-to-tangent rotations, both
    end-tangent frames and its centre.
  - Every mass constituent's M. Each half-arc lump gives its node, mass,
    first moment, centroid offset, centreline second moment and section
    rotary inertia.
  - The massless constituents, the assembled `K_full` and `M_full` over all
    DOFs, and the mass by category.
- **`modal_basis`.** Per mode:
  - λ, ω, f and T;
  - the normalization case and DOF, and any tie;
  - the generalized mass, Γ, m_eff, the fraction and the cumulative
    fraction;
  - the shape.

  It also gives T_k and the scope clusters.
- **`analysis`.**
  - The request, the extracted, clustered and periodic modes and the mode
    status.
  - The spectral ordinates, the CQC ρ and the grouping groups.
  - `quantities` (with units) and `quantity_families` (their meaning and
    sign).
  - `contributions_signed` per direction and mode, and
    `rigid_response_signed` per direction.
  - `rigid_load` per direction: the acceleration field g, F over all DOFs,
    and its share per constituent.
  - `periodic_modal_combined` (every modal rule), and the same with the
    correction off.
  - `per_direction_combined` and `combined` under the base rules, and the
    45-entry `mutations` grid.
  - `missing_mass_disclosure`.
- **`cross_checks`.** The generator's own residuals (below). They are not
  reference values.

## Conventions a product must match or map

1. **Reaction sign.** A reaction is the force and moment the anchor applies
   to the piping, in global axes: (K u − M g) at its DOFs. A product
   reporting the force on the support has the opposite sign of every signed
   reaction. Combined magnitudes are unaffected.
2. **End actions.** Every element, connector and rod end action is the
   generalized force acting on that element at its end node (node on
   element), in the order Fx Fy Fz Mx My Mz, node i then node j. An internal
   section force at end i is the negative of the end action there.
3. **Arc end actions.**
   - The total is elastic minus `lump_load`: (K_arc − ω² M_lumps) u per mode,
     and K_arc u − M_lumps g in the rigid response. M_lumps acts only at the
     end node of each lump.
   - The `_t` rows are the same totals in each end's U4a end-tangent frame:
     x the traversal tangent, y toward the centre, z = x × y. The frame rows
     are published per end.
   - Signed contributions are rotated before they are combined. Combined
     magnitudes depend on the axes they are combined in. The global rows are
     kept as independent observables; their presence does not decide what a
     product publishes.
4. **Connector.**
   - g is in connector axes, in the order axial, lateral y, lateral z,
     torsion, bending y, bending z. It is work-conjugate to q = B d.
   - `J1.i.*` and `J1.j.*` are Bᵀg in global axes.
   - These are incremental dynamic quantities. They exclude static pressure
     thrust, preload and reference load, and they are neither wall force nor
     stress.
5. **Rods.** T is tension positive. `R*.i.*` and `R*.j.*` are b·T in global
   axes, with the moment about the node including the offset.
6. **Straight members** are axis-aligned. Their global rows map component to
   component onto any member frame that is a signed permutation of the global
   axes.
7. **Interpolation abscissa, grouping boundary and 100-40-40 with 2/5.**
   These are as in `../discrete/` (conventions 3 to 5), unchanged.

## Cross-checks (largest over all variants)

| Check | Largest |
|---|---|
| Pipe K assembled here vs f1 `form_stiffness` | 3.9e-79 relative |
| Connector B: closed form vs kinematic columns; rigid-body nullity | exact |
| Connector Bᵀg vs K_e u; rod b·T vs K_rod u | 2.7e-80; 5.3e-79 |
| Sturm counts at λ₁/2, each gap's geometric mean and 2λ_max | all agree |
| Eigen residual; M-orthogonality | 1.6e-73; 2.0e-72 |
| Σ m_eff over every finite mode vs p_kᵀM_ff⁻¹p_k | 3.4e-74 relative to T_k |
| Nodal balance (free DOFs; anchor DOFs vs reaction) | 3.5e-74; 0 |
| Global force and moment equilibrium, per mode and rigid response | 1.9e-74; 2.6e-74 |
| Half-arc lump M g vs direct 48-point integral of the rigid half-arc (own geometry, trigonometry, π and nodes) | 2.1e-79 |
| Arc lumps vs closed-form arc weight at its centroid | 5.8e-80 |
| End-tangent frames: U4a rotation rows vs own geometry; orthonormal, right-handed | 3.7e-80; 2.0e-80 |
| Rigid total reaction vs −a_r(T_k − Σ_periodic m_eff) | 3.5e-79 |
| Rigid static solve vs modal expansion over the non-periodic finite modes | 1.2e-73 |
| Rigid-load shares summed vs F | 4.9e-80 |
| T_k vs m̄ × mass-bearing length, span D–E excluded | 9.1e-80 |

## Controls

- **Rods absent and present.** The two variants of each pair differ only by
  the rods. `w2_controls.json` places the frequency ladders side by side.
- **Fixed-state exclusion** (V-JOINT-R, V-COMB-R).
  - The held state is p = 1.5 MPa on A_e = 0.03125 m² (P = 46875 N), a
    nonzero q_ref and a nonzero d0.
  - In all 144 checks, the increments of the affine connector, face and rod
    actions equal K_c B δ, Bᵀ K_c B δ and k b·δ exactly in ℚ.
  - Three mutants add a forbidden constant to every contribution. Their
    smallest relative distances from the reference combined values are:
    - the held connector force: 46.4 on V-JOINT-R, 4.03 on V-COMB-R;
    - the thrust folded into the axial force: 4.0e7 and 4.0e5;
    - the held face action with its thrust: 225 and 21.0.
  - This is a limited control. It shows that the affine terms vanish in the
    increment. It does not establish an EX static equilibrium, a
    pressure-region mapping or a same-run nonlinear state confirmation.
- **Replaced-span double ownership** (V-COMB-R).
  - The span's pipe mass, its stiffness, or both are added.
  - T_k becomes 245.75 kg against 220.36 kg. The first frequency becomes
    19.44, 29.37 or 28.26 Hz against 21.40 Hz.
  - The selected combined values change by at least 0.41 % (mass), 0.39 %
    (stiffness) and 1.2 % (both).
  - The reference publishes no span member key.

## Limits

- **The arc model.** It is DEL-03-03's thin curved-beam model with one k,
  which inherits LIM-021. These references show agreement with that model,
  not its adequacy.
- **Section bases.** PF and EX coincide here (c_m = c_a = 0), so this family
  does not qualify the distinction between the nominal and reduced bases.
- **The connector frame.** It is an exact derived rational frame from an
  integer guide. A product that authors the frame as rounded unit vectors
  forms a slightly different frame. How a product represents the frame is
  outside these references.
- **No exact repeats.** There is no exactly repeated eigenvalue and no
  multi-mode cluster. `../discrete/` C7 and C10 cover those.
- **What is not covered.** Compliant supports, snubbers, nonlinear states,
  Timoshenko members, directional k, nonzero rod pretension, nonzero
  attachment offsets and pressure stiffness.
- **The pressure and reference control** is the fixed-state increment only
  (above).
- **Stresses.** None are formed.
- **Tolerances.** None is set for product comparison. These references state
  only their own cross-check residuals.
- **Comparability.** Every extracted periodic mode is simple and well
  separated, so per-mode contributions and combined magnitudes are both
  comparable at a verification tolerance.
