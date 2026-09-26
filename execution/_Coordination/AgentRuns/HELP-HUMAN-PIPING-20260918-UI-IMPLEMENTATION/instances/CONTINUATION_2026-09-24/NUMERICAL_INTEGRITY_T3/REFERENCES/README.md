# T3 R1: independent references (VP-ORACLES and VP-ROBUST)

This is the return of Type 2 TASK R1 for tranche T3 (numerical integrity, precision and scale), 2026-09-26. The brief is `T3/TASK_BRIEFS/R1_REFERENCES.md`, read with `_COMMON.md`. The package holds hand-derived references for T3's general accuracy method, range support and sparse scale. It was written without reading product code. **Status: candidate.** The references are frozen only when ROOT selects them after the independent refutation (V2). No tolerance is proposed here. Every comparison uses the unchanged form

```
|observed - expected| <= 1e-9 * max(|expected|, scale)
```

The `scale` is the published class scale of each value. See §4.

## 1. Summary

| | |
|---|---|
| Cases | 216 (208 with expected values, 8 mechanisms that must be refused) |
| Published expected values | 27893 (40 significant digits), plus represented-input values for 2 cases |
| Negative controls | 724 (550 discriminate under the criterion; the rest are reported as non-discriminating) |
| Families | RF-CHAIN, RF-SKEW, RF-WEAK, RF-LARGE, RF-INVARIANCE, RF-RANGE, RF-ZERO, RF-FINITE, RF-MECH, and RF-CANCEL (addendum `R1_ADDENDUM_CANCEL.md`: 41 cases, 1585 values) |
| Arithmetic | Exact rationals. π is replaced by the exact rational of a 190-digit Machin value. One case uses 110-digit decimal |
| Author self-check | 196 cases and 52490 quantities agree exactly at two rational stand-ins for π, with 0 failures; nullity confirmed for all 8 mechanisms; represented-input comparison exact for 9 cases |
| Python | Python 3.11.15 (CPython), standard library only |

Files. The hashes are in `_run_records/SHA256SUMS`.

- `references.py`: the derivation. `python3 references.py` regenerates `references.json` and prints the run summary. `python3 references.py --full <case id>` prints every value of one case, including the values that large cases leave out of the JSON. `python3 references.py --model <case id>` prints the complete model of a case. The JSON abbreviates the models of cases with more than 1000 members.
- `references.json`: the candidate frozen values, with case ids, units, model inputs, class scales and their derivations, reference accuracy, negative controls, the finite-input comparison and, where relevant, the represented-input basis, range envelope or null motions.
- `_run_records/references.stdout.txt`: the stdout of the generating run.
- `_run_records/selfcheck_stiffness.py` and `selfcheck.stdout.txt`: the author's own direct-stiffness check. This is not V2.
- `_run_records/PYTHON_VERSION.txt` and `SHA256SUMS`.

Findings the designers should see (§7 has the detail):

1. **Input rounding can defeat an intended-input comparison.** `RF-SKEW-A-CANT-AX-122-r1e-12` authors a soft axial load of 4e-8 N and a 9 N perpendicular load in one global vector. Rounding each component to binary64 moves the soft displacements by 8.4e-9 of their scale, which is above 1e-9. The RF-FINITE rule therefore makes the represented input its basis. Coordinates offset by 1e6 m with 1/30 m members do the same (`RF-FINITE-THIRTIETHS-O1e6`). The 1/10 m and 1/3 m offset cases stay below 1e-9, at 9.3e-10 and 2.3e-10.
2. **At k/a ≈ 1e-4 the soft cases are continuity controls, not defect detectors.** A lost soft contribution changes k by only about 1e-12 relative there. The published negative controls say which cases discriminate which defect: from about 1e-8 downward, the lost-contribution, stored-assembly and subtract-rounded controls fail by factors from 1.1 up to 1.1e6.
3. **Two RF-RANGE vectors (`LEF-small`, `LEF-large`) keep every input and output normal, but GJ or EI (2^-1078, 2^1122) falls outside the binary64 range.** A solver that forms section stiffness naively cannot pass them without scaling. The case text says what a correct solve publishes. Whether a named range refusal is acceptable is a design decision, and nothing here decides it.
4. **Cases that need a spring along a non-global direction are flagged `needs_directional_spring`.** R1 did not check whether the product can author such springs.
5. **RF-CANCEL: left-to-right binary64 summation of cancelling contributions.**
   - For (G, n, −G) and (n, G, −G) with n = 0.3, the net stays inside the criterion at G = 1e5 and 1e6. It fails by 2.5× at G = 1e7 and 9.9× at G = 1e8, and is lost entirely at G = 1e80.
   - The order (G, −G, n) is exact at every magnitude.
   - For the two uniformly loaded spans, the binary64 equivalent-moment assembly fails at w = 1e8 N/m (1.0× and 2.5×, depending on order) and loses the net at 1e80.
   - The scale question matters. The recommended net-governed scale detects every such loss. The family-wide class (norm) scale hides it whenever an ordinary load is present, and a gross-governed scale hides it always. §6 recommends the net-governed scale for these cases.

## 2. Theory, conventions and inputs

This is the implemented theory as the brief states it: a small-displacement, linear-elastic Euler–Bernoulli space frame with no shear deformation, circular annular sections (Iy = Iz = I, J = 2I), rigid restraints on global DOFs, grounded linear springs and nodal loads only. These are references for that theory. They are not physical validation.

- Global right-handed X, Y, Z, with DOFs UX, UY, UZ, RX, RY, RZ. Lengths are in m, forces in N, moments in N·m and rotations in rad. The mm/N/MPa cases use mm and N·mm.
- The N-series section: E = 200 GPa, G = 80 GPa, OD = 0.2 m, ID = 0.18 m. That gives A = 0.0019π, I = 0.0000085975π and J = 0.000017195π, so EA = 380000000π N, EI = 1719500π N·m² and GJ = 1375600π N·m². The script asserts these three coefficients exactly. The soft coupling section in RF-WEAK scales E and G by ρ. The PHYS-R4-geometry cases state E and ν, with G = E/(2(1+ν)).
- **Published quantities** are all convention-free:
  - `u.<node>.U*` and `th.<node>.R*`: global nodal translations and rotations.
  - `R.<node>.<DOF>`: rigid-restraint actions on the structure.
  - `S.<node>.<i>.F*|M*`: global components of spring i's action on the structure, −k (d·u) d/|d|².
  - Per member: `N` is the axial force, tension positive, = EA·ext/L. `T` = GJ·tw/L. `Mb.i`, `Mb.mid` and `Mb.j` are the bending magnitudes hypot(My, Mz) at authored end i, the midpoint and end j. `tw` = (θj − θi)·e_ij and `ext` = (uj − ui)·e_ij.
  - N, T, tw and ext do not change when i and j are swapped.
  - Loads plus support actions sum to zero in force and moment. The script checks this exactly for every solved case.
- **Model inputs** are exact intended values. A decimal string is exact. `p/q` is an exact rational. `d*2^k` is a power-of-two multiple. Spring directions are exact vectors, which the product normalizes. All inputs are invented, and no material, component or code data is used.

## 3. Methods (how every value is derived)

No direct-stiffness assembly produces any expected value. Four closed-form routes do.

**M1. Tree integration.** Used for determinate structures whose one supported node has, for translations and for rotations separately, three mutually orthogonal supported directions, each rigid or sprung. Statics of every cut gives the subtree resultant F and its moment M_c about the child node c. For member p→c, with unit vector e and length L, the section moment is M(x) = M_c + (L−x) e×F. The curvature is C·M(x) with C = e eᵀ/GJ + (I − e eᵀ)/EI. Integrating along the member:

```
Δθ = C (L M_c + L²/2 e×F)
Δu = L²/2 (C M_c)×e + L³/3 (C(e×F))×e + L (e·F)/EA e
θ_c = θ_p + Δθ,   u_c = u_p + θ_p×(x_c − x_p) + Δu
```

The root reaction is minus the total resultant. Each root spring along d contributes a motion −d (R·d)/(k|d|²). Member actions follow from the cut: N = e·F, T = e·M_c, and bending is the perpendicular part of M(x) at x = 0, L/2 and L.

**M2. Force method.** Tree integration from a determinate root. Every other rigid restraint or spring becomes a redundant action X_i d_i, with compatibility d_i·u_i + X_i|d_i|²/k_i = 0 (k = ∞ for a rigid restraint). The flexibility matrix is built from unit-action tree solves and solved exactly. This route covers W-AX, W-3D and ZERO-SYM (6 redundants at the far fixed node), the relabelled continuous beam and the author's cross-checks.

**M3. Pure torsion with translation-pinned collinear nodes.** Used for INVARIANCE-PINTOR. Every node is translation-pinned, a rotational spring k along the line sits at the root, and a torque T acts along the line at the tip. Then every member carries T, θ_n = (T/k + Σ T L/GJ) e, and every translation, force and bending moment is zero. Nodal moment equilibrium holds, the field is compatible with the pins and has no curvature, and the structure is stable. So this field is the solution.

**M4. One member pinned in translation at both ends.** Used for SKEW-T-PIN-* and the RF-MECH stable companion. At the root there is one rotational spring k, or one rigid rotation restraint, along d with c = d·e ≠ 0. A tip moment M1 = M1e e + M1p acts at the tip. Let w = d − c e and s = d·θ_root. Moment equilibrium at both nodes, split along e and the plane normal to e, with rotational stiffness GJ/L along e and EI/L·[[4,2],[2,4]] across it, gives:

```
s = M1e |d|²/(c k)                         (0 for a rigid restraint)
θ_root⊥ = −L/(3EI) (M1p/2 + (M1e/c) w),   θ_root·e = (s − w·θ_root⊥)/c
θ_tip⊥ = (M1p L/EI − 2 θ_root⊥)/4,        θ_tip·e = θ_root·e + M1e L/GJ
restraint moment −(M1e/c) d;   R_tip = e×(M_0 + M1)/L = −R_root
```

Here M_0 = −M1e e + EI/L (4θ_root⊥ + 2θ_tip⊥). For a pure tip torque T e this reduces to θ_root = [T/(c²k) + (1−c²)TL/(3c²EI)] e − TL/(3cEI) w and R_tip = T (d×e)/(cL). The root bending magnitude is then |T/c|·sqrt(1−c²), falling linearly to zero at the tip.

**M5. Three-moment equation.** Used for the continuous beams. Supports are equally spaced (span ℓ), S0 is fixed in all six DOFs, S1..Ss are translation-pinned, and point loads p_j act at the midspans. Take m = EI v'' for each transverse plane:

```
2 m_0 + m_1 = (3ℓ/8) p_1;   m_{i−1} + 4 m_i + m_{i+1} = (3ℓ/8)(p_i + p_{i+1});   m_s = 0
v_mid,j = −ℓ²(m_{j−1} + m_j)/(16EI) + p_j ℓ³/(48EI);   v'_mid,j = ℓ (m_{j−1} − m_j)/(24EI)
v'(support i) = −(ℓ/EI)(m_i/3 + m_{i+1}/6) + p_{i+1}ℓ²/(16EI)   (last support: (ℓ/EI)(m_{s−1}/6) − p_sℓ²/(16EI))
R_i = (m_{i+1} − 2m_i + m_{i−1})/ℓ − (p_i + p_{i+1})/2;   S0 moment reaction from global statics
```

The two transverse planes are independent, since the section is circular. Axial and torsional actions are zero. θ_z = v' and θ_y = −w' in the local frame, which is rotated exactly by Q for the rotated cases.

**M6. Two uniformly loaded fixed-ended spans at a pinned shared node.** Used for RF-CANCEL-UDL. S0 and S2 are fixed, S1 is translation-pinned, the spans have equal length L and carry uniform loads w_A and w_B (+y), and S1 carries a moment Mz. With m = EI v'' (m'' = w), each span is the fixed-ended solution plus the effect of the shared-node rotation θ:

```
θ = (Mz + (w_B − w_A) L²/12)/(8EI/L)
m_A(x) = w_A (x²/2 − Lx/2 + L²/12) + EIθ(−2/L + 6x/L²)
m_B(x) = w_B (x²/2 − Lx/2 + L²/12) + EIθ(−4/L + 6x/L²)
S0: FY = −w_A L/2 + 6EIθ/L², MZ = −m_A(0);   S1: FY = −(w_A + w_B)L/2;   S2: FY = −w_B L/2 − 6EIθ/L², MZ = m_B(L)
```

The fixed-end moments of the two spans reach S1 with opposite signs, so equal loads in the same direction cancel there.

**Transformed cases** (rotation, offset, relabelling, units, range scaling and the load scaling of the comb trees) are computed by transforming the base solution. Each is asserted identical to a direct M1–M5 solve of the transformed model.

## 4. Scales (VP-ORACLES zero scales)

Every published value belongs to a class: translation, rotation, force, moment, twist or extension. RF-WEAK adds regions, which give classes with suffixes `@coupling` and `@far`. Each class carries one scale, published with its derivation.

- **Non-empty class:** the scale is the largest magnitude of that class in the case (or region). Nodal translations, rotations and support actions are measured as vector norms, so the scale is invariant under rotation. Member values are measured as absolute values or bending magnitudes. This follows the case's load path directly: for example, the tip rotation T/k + ΣTL/GJ of a soft chain, or the coupling force of a weak link.
- **All-zero class:** the scale comes from load and geometry. L_c is the longest member of the region. F_ref and M_ref are the largest applied force and moment norms.
  - Force scale: moment scale/L_c, or max(F_ref, M_ref/L_c) when force and moment both vanish.
  - Moment scale: force scale·L_c, or max(M_ref, F_ref·L_c).
  - Translation scale: rotation scale·L_c. Rotation scale: translation scale/L_c.
  - Twist scale: moment scale·max(L/GJ). Extension scale: force scale·max(L/EA).
- **Relative motions have their own classes.** Twists and extensions are therefore compared relatively even when they are 1e-12 of the nodal rotations. This is the N05-class requirement.
- **Weak links get their own regions.** A far-side response proportional to a 1e-12 coupling cannot hide under a near-side scale.
- Each case reports `nonzero_below_class_scale`. This counts nonzero values smaller than their class scale, which the criterion therefore compares against an absolute floor of 1e-9 times the scale. Typical examples are near-root displacements of long cantilevers. RF-ZERO-TINY shows the reverse case: its 1.7e-12 m and 4.6e-10 rad values are the class scales themselves, so they are compared relatively although a naive absolute 1e-9 floor would accept zero.

## 5. Reference accuracy

Each case states its accuracy. The default: the stated closed form is evaluated exactly in rationals with π → PI_Q (|PI_Q − π| < 1e-189), and every nonzero value is rounded once to 40 significant digits (relative error < 5e-40). Bending magnitudes are square roots of exact rationals, evaluated at 70 digits. Zeros are exact. Represented-input solutions decode every intended input with `Decimal.from_float(float(x))`. Spring directions are exact integer vectors and are not decoded. One represented-input solution uses 110-digit decimal because rounded skew coordinates give irrational lengths (`RF-FINITE-ROT-Q9-NONMULT`). Its accuracy is stated in that case.

Checks inside `references.py`:

- The global equilibrium residual is exactly zero for every solved case.
- Every published value is exact, with no binary float anywhere.
- Every rotated, offset, relabelled, unit-converted, range-scaled and load-scaled expectation equals a direct solve of the transformed model exactly, at every n. For n = 10 the continuous beam is also solved by M2, and it agrees exactly with M5.
- RF-MECH null motions are verified to be admissible zero-energy motions and linearly independent.
- The RF-RANGE inputs, values and scales are all normal binary64 numbers.

The author self-check (`_run_records/selfcheck_stiffness.py`) re-solves every non-mechanism case up to 100 members by exact direct stiffness, at π = 22/7 and π = 333/106. It compares every quantity for exact equality with the closed forms at the same π. It also counts the exact zero pivots of every RF-MECH model, and repeats the comparison on represented inputs for nine cases, including all cases whose basis is represented. For RF-CANCEL-UDL it uses consistent equivalent nodal loads (qL/2 and ±(L²/12) e×q) and recovers member end actions as K_e u_e minus the equivalent loads. Result: 196 cases and 52490 quantities agree exactly at two rational stand-ins for π, with 0 failures; nullity confirmed for all 8 mechanisms; represented-input comparison exact for 9 cases.

## 6. Families

**RF-CHAIN: order > 2 soft chains (30 cases).** Torsion and axial chains of 3, 5 and 10 members along x, with lengths 2, 1, 3, 1.5, 2.5, 0.5, 4, 1.25, 3.5 and 0.75 m. A soft root spring gives k/a₁ ≈ 1e-4, 1e-6, 1e-8, 1e-10 and 1e-12: k = 216 … 2.16e-6 N·m/rad against a₁ = GJ/2, and 59700 … 5.97e-4 N/m against EA/2. The tip load T (or F) = 1e-4·k, so the root rotation or translation is 1e-4. Series compliance gives θ_root = T/k, θ_j = T/k + Σ_{i≤j} T L_i/GJ, tw_i = T L_i/GJ, member torque T and spring action −T. The axial chains are analogous with EA. Negative controls:

- NC-LOST-SOFT: k after one rounded binary64 addition into a₁.
- NC-STORED-ASSEMBLY: the exact solution of the binary64-stored tridiagonal chain matrix, with correctly rounded a_i, one rounded addition per diagonal, and rounded k and load. This is NP-A generalized.
- NC-SUBTRACT-ROUNDED: relative motion from binary64-rounded global values.
- NC-SIGN.

**RF-SKEW: skewed members (36 cases).** Members (0,0,0)→(3,4,0), L = 5, and (0,0,0)→(1,2,2), L = 3, at k/a ≈ 1e-4, 1e-8 and 1e-12 (soft motion 1e-4). Six variants each:

- T-PIN-AX and T-PIN-OFF: both nodes translation-pinned; root rotational spring along e, or along global X at cosines 3/5 and 1/3 (M4).
- T-CANT-AX: root translations fixed; springs k along e and 1e6 N·m/rad along the rational perpendiculars n and m; tip torque plus a perpendicular force.
- T-CANT-OFF: springs along global X (k), Y and Z (1e6).
- A-CANT-AX and A-CANT-OFF: the translational analogues, with root rotations fixed and 1e8 N/m ordinary springs.

M1 or M4 applies. Examples:

- T-CANT-OFF: θ_root = (T e_x/k, T e_y/1e6, T e_z/1e6), θ_tip = θ_root + TL/GJ e and u_tip = θ_root × L e.
- A-CANT-AX: u_root = (F/k) e + (P/1e8) n̂, u_tip = u_root + FL/EA e + PL³/(3EI) n̂ and θ_tip = PL²/(2EI) m̂.

Negative controls:

- NC-LOST-SOFT: one rounded addition into the member's stiffness in the spring direction, GJ/L c² + 4EI/L (1−c²) or EA/L c² + 12EI/L³ (1−c²).
- NC-SUBTRACT-ROUNDED and NC-SIGN.
- NC-WRONG-TRANSFORM: the member axis with its first two components swapped in the constitutive frame.
- NC-DROPPED-COUPLING (T-PIN-OFF): the off-axis spring treated as a torsional spring k c², which drops the bending it induces.

**RF-WEAK: weak coupling (9 cases).**

- W-AX, ρ = 1e-4, 1e-8 and 1e-12: nodes x = 0..5, stiff A1 and A2, soft C (E and G × ρ), stiff B1 and B2, both ends fixed. The load at N2 is (1000, 100, 0) N plus (50, 0, 0) N·m, and the far response is ∝ ρ. Method M2.
- W-L at 1e-4, 1e-8 and 1e-12: the brief's L-frame with a soft root torsional spring, where M2's bending becomes M1's torsion. Method M1.
- W-3D at three ρ: two fixed L-frames joined by a soft member along z. It carries axial force, torsion and bending into the far frame. Method M2.

Regions near, coupling and far have separate class scales. Negative controls: NC-DROPPED-COUPLING, NC-LOST-SOFT (the coupling's retention factor after one rounded addition into the adjacent diagonal), NC-SUBTRACT-ROUNDED, NC-SIGN and NC-WRONG-TRANSFORM (W-L).

**RF-LARGE: scale (24 cases).** n = 10, 100, 1000 and 10000 members, axis-aligned (AX) and rotated by Q3 (ROT, integer coordinates).

- CHAIN: a cantilever of 3 m members with a tip force and moment of power-of-two magnitude (tip rotation ≈ 1e-4). Method M1.
- TREE: a comb, with a spine of n/2 members and one 3 m branch at each spine node (+y at odd, +z at even). Branch-tip forces are 3·((k mod 7) − 3)·2^s, normal to branch and spine. Method M1.
- CONT: n/2 spans of 6 m, with midspan forces 96·((j mod 5) − 2) N along y and 96·((j mod 3) − 1) N along z. Method M5.

For n ≥ 1000 the JSON publishes 8 sampled node and member positions, at indices 0, 1, 2, n/4, n/2, 3n/4, n−1 and n of each chain, spine or span list. `--full` prints every value. Scales always use the full solution. Negative controls: NC-OFF-BY-ONE, NC-DROPPED-LOAD, NC-SIMPLE-SPANS (continuity dropped), NC-UNROTATED and NC-SIGN. These cases are meant for M32 parity and memory runs as well as accuracy. R1 makes no conditioning or solver-difficulty claim.

**RF-INVARIANCE (25 cases).** There are four bases, all with rotation-invariant restraint sets:

- PINTOR: the pinned torsion chain, lengths 18, 9, 27, 9 and 18 m, soft root spring 90000·2^-20 N·m/rad (M3).
- LFRAME: an L-frame with 9 m legs and a soft rotational spring set (M1).
- CONT4: 4 spans of 18 m (M5).
- TREE100: the RF-LARGE comb (M1).

Each base is transformed as follows:

- Rotation by Q3 = (1/3)[[1,2,2],[2,1,−2],[−2,2,−1]] and Q9 = (1/9)[[1,8,4],[8,1,−4],[−4,4,−7]], both checked orthogonal with det +1. Base coordinates and loads are multiples of 9, so rotated inputs are exact binary64 values.
- Offsets (1000, −1000, 1000) and 10⁶·(1, −1, 1) m.
- Reverse node and member numbering, with every member direction reversed.
- mm/N/MPa: E = 200000 MPa, OD 200 mm, moments in N·mm, rotational springs in N·mm/rad, translational springs in N/mm.

Negative controls: NC-UNROTATED, NC-ORIGIN-MOMENTS (support moments taken about the global origin), NC-OLD-NUMBERING, NC-UNSWAPPED-ENDS, NC-WRONG-UNIT and NC-SIGN.

**RF-RANGE (32 cases).** Three bases: RF-CHAIN-T-n05-r1e-08, RF-SKEW-T-CANT-OFF-122-r1e-08 and RF-LARGE-CONT-n00010-AX. Scale factors are 2^pl for lengths including OD and ID, 2^pm for moduli and 2^pf for loads. Moments scale by 2^(pf+pl), translational springs by 2^(pm+pl) and rotational springs by 2^(pm+3pl). The ten (pl, pm, pf) vectors are (∓240, 0, 0), (0, −1000, 0), (0, 960, 0), (0, 0, ∓960), ±(200, 300, 600) and ±(−120, 500, 260), the last pair preserving rotation.

The expected values are the base values times exact powers of two: 2^(pf−pm−pl) for translations and extensions, 2^(pf−pm−2pl) for rotations and twists, 2^pf for forces and 2^(pf+pl) for moments. Each case reports the log2 envelope of its inputs, derived section and element quantities, and published values. Two loaded cantilevers use the PHYS-R4 geometry (OD 4e-77 m, ID 2e-77 m, L = 1 m, ν = 0.1):

- THIN-A: E = 2^200 Pa, loads ≈ 1e-251 N, rotations ≈ 1e-4.
- THIN-B: E = 1 Pa as in PHYS-R4, loads 1e-300, rotations ≈ 1e6 rad. This is linear-theory arithmetic only.

PHYS-R4's own frozen stress reference is not re-derived. What a correct solve publishes is the stated value. A zero, infinity, NaN or unscaled value is a failure. A named range refusal is not a wrong number but does not pass. Negative controls: NC-UNSCALED, NC-ROTATION-SCALED-AS-TRANSLATION and NC-SIGN.

**RF-ZERO (4 cases).** ZERO-SYM is a fixed–fixed beam with a centre load, so centre rotations, axial and torsional values are zero (M2). ZERO-TORSION is a (1,2,2) cantilever under a pure axial torque, so every translation, force and bending moment is zero. ZERO-PLANAR is an L-frame loaded in its plane, so the out-of-plane response is zero. ZERO-TINY has u_x = 2e-3/EA and θ_x = 2e-3/GJ, which are below a naive 1e-9 floor but above, or equal to, their scales. Negative controls:

- NC-SPURIOUS-NONZERO: each structural zero published as 1e-8 of its scale, which fails by a factor of 10.
- NC-ROUND6.
- NC-ZEROED: this reports `naive_absolute_1e-9_floor_accepts_this_defect: true`.
- NC-SIGN.

**RF-FINITE (6 dedicated cases, and every other case too).** Ten-member cantilevers with nodes at x = O + i/3 (O = 0, 1e3, 1e6), O + i/10 and O + i/30 (O = 1e6), plus an L-frame rotated by Q9 with ninth-valued coordinates and loads. Every case in the package carries `finite_input`: the exact solution for the binary64-rounded inputs and the largest per-quantity difference, normalized by the criterion. Where that exceeds 1e-9, the case says it cannot discriminate at 1e-9 on intended inputs, and `expected_represented` becomes its reference basis, as with NP-A. Cases on the represented basis: `RF-SKEW-A-CANT-AX-122-r1e-12`, `RF-FINITE-THIRTIETHS-O1e6`.

**RF-MECH (9 cases).** These must be refused:

- LINE122-TORQUE, -PERP and -UNLOADED: 10 members along (1,2,2), every node translation-pinned. Null motion θ = α(1,2,2), u = 0. The load does work, is orthogonal, or is absent.
- LINE345-RZ: 10 members along (3,4,0), pinned plus RZ at every node, 44 restraints. RZ is normal to the axis. This is the count trap.
- K0: RF-CHAIN torsion with k = 0 exactly.
- DISC-CHAIN100: RF-LARGE-CHAIN-100 plus a disconnected 3-member sub-assembly, 6 rigid modes.
- DISC-CHAIN100-SPRING: the same with one spring, 5 modes.
- LINE-IN-CHAIN1000: RF-LARGE-CHAIN-1000 plus a pinned 5-member (1,2,2) line, 1 mode.

Each lists its null motions, verified admissible and zero-energy, and the work of the load on each. One stable companion, LINE345-RX-COMPANION (RX intersects the axis, M4 with a rigid restraint), must be solved. Its NC-FALSE-MECHANISM is the refusal.

**RF-CANCEL: cancelling load contributions (41 cases, 1585 values; addendum `R1_ADDENDUM_CANCEL.md`).**

Cases:

- **Nodal cancellation (items 1 and 2).** A cantilever N0 (fixed) → N1 (1,0,0) → N2 (2,0,0), N section. The tip DOF, UY in N or RZ in N·m, carries three contributions in authored order (G, n, −G), (G, −G, n) or (n, G, −G): n = 0.3 with G = 1e5, 1e6, 1e7, 1e8, and n = 1e-8 with G = 1e80. The inputs store the contributions separately (`load_contributions_in_authored_order`). The exact response is that of the exact net n. The represented-input basis rounds each contribution separately and sums exactly, so it differs from the intended by about 1e-15 and every case keeps the intended basis.
- **Item 3.** The same, at G = 1e8 and 1e80 in order (G, n, −G), with an ordinary load. `-ORTHO` places F_z = 100 N (or M_y = 50 N·m) at the tip, an orthogonal DOF. `-INPLANE` places F_y = 50 N (or M_z = 20 N·m) at N1, in the same bending plane. Either way the response is no longer a scaling of the net load.
- **Item 4.** `RF-CANCEL-UDL-W1e5`, `-W1e8` and `-W1e80` use M6 with L = 2 m, w_A = W and w_B = W + d (d = 0.875 N/m, exact in binary64 next to W) and Mz = 0.2 N·m. The W1e80 case uses d = 0 and Mz = 1e-8 N·m. The net moment at S1 is Mz + dL²/12. Interpretation: both loads act in the same direction (+y). Only then do their fixed-end moments reach the shared node with opposite signs and cancel; loads of opposite direction would add there. I read the brief's "opposite sign" as the sign of the fixed-end moments at the shared node, and the case states this.

**Scales (brief: say whether gross or net governs, and recommend).** Each RF-CANCEL value row is `[key, expected, class, recommended_scale, gross_scale, governed_by]`.

- `governed_by` is `net` when the value is exactly its response to the net contribution alone, `other loads only`, `mixed` or `zero`.
- **Recommended, net-governed:** the magnitude of the value's response to the net contribution alone, or the class scale where the net does not affect the value or the value is zero. Because the comparison uses max(|exp|, scale), a scale at or below |exp| makes the comparison relative. Every value the net governs is therefore compared relatively, which is what detects a lost or mis-summed net.
- **Alternative, gross-governed:** the value's response to the gross contribution alone. I do not recommend it, because it accepts the defect it is meant to find. That is the NP-A/N05 principle: a small legitimate contribution next to large ones must be preserved.
- The family-wide class rule is also shown.
- Each negative control reports `discriminates` (recommended scale), `discriminates_under_class_scale` and `discriminates_under_gross_scale`. In the ORTHO and INPLANE variants the class (norm) scale is set by the ordinary load and hides the float-summation defect. The recommended scale still catches it, through the values the net alone governs: for INPLANE, the bending moment at N1, where the N1 load has no lever arm. For the spans, the S1 rotation is net-governed while the reactions and end moments are gross-dominated (`mixed`).

**Negative controls:**

- NC-FLOAT-SUM-GnG, -GGn and -nGG: the response to the net obtained by summing the binary64 contributions left to right in each order, marked where that is the authored order.
- NC-SMALL-DROPPED: the response with n dropped.
- For the spans, NC-FLOAT-SUM-A-node-B and -A-B-node: the equivalent end moments −w_A·L·L/12.0 and +w_B·L·L/12.0 plus the nodal moment, summed in binary64 in the stated order. And NC-SMALL-DROPPED (the nodal moment dropped).

## 7. Findings and limits

- **Represented basis.** `RF-SKEW-A-CANT-AX-122-r1e-12`, `RF-FINITE-THIRTIETHS-O1e6` fail the 1e-9 test on intended inputs because of input rounding alone. `SKEW-A-CANT-AX-122-r1e-12` authors a soft 4e-8 N axial load and a 9 N perpendicular load in one global vector. The ulp of each summed component, about 4.4e-16 N, is a 1e-8 relative change of the soft load. `SKEW-A-CANT-AX-345-r1e-12` is just below, at 9.8e-10. This is an input-level analogue of N05. Neither extra precision in the solver nor anything else downstream can recover it.
- **Offsets.** A 1e6 m offset with 1/3 m and 1/10 m members stays below 1e-9, at 2.3e-10 and 9.3e-10 respectively. With 1/30 m members it does not.
- **Non-discriminating controls are reported, not hidden.** Examples: the lost soft contribution at k/a ≈ 1e-4, subtract-rounded controls where no soft mode exists, NC-ORIGIN-MOMENTS for PINTOR (no support forces), and NC-WRONG-TRANSFORM in W-L where the soft rigid mode dominates. A harness should use a case's discriminating controls as its mutation tests.
- **Defect models are models.** NC-LOST-SOFT uses one rounded addition in the frame of the spring. A global-frame assembly also perturbs the other entries. NC-STORED-ASSEMBLY assumes correctly rounded element coefficients. The product's own rounding differs in detail, but not in order.
- **RF-CANCEL scale choice.** A per-value net-governed scale is a choice about the comparison. It is not a new tolerance: the predicate is unchanged, and the scale never goes below |exp|. Whether harnesses adopt it for cancellation cases is for the designers and ROOT. The alternatives are published so the choice can be reviewed.
- **Not covered:**
  - shear deformation, releases, offsets, curved elements, user matrices, element loads, thermal or pressure loads, prescribed displacements, nonlinear supports and load combinations (none are in the brief's formulation);
  - dynamic or buckling behaviour;
  - any tolerance proposal;
  - any statement about which product path can meet 1e-9;
  - conditioning estimates for the large cases.
- **Unknown product authoring.** The product's input model (OD with ID or with wall, springs by direction or by DOF) is unknown to R1. The represented-input basis uses OD and ID as inputs. With wall as the input, section values change by about 1e-15 relative, which is immaterial at 1e-9 unless a case's `finite_input` is already near 1e-9.
- **Existing references.** The frozen N01–N09, R01–R07 and NP-A–NP-D references and the T0R and T1 references were not changed or regenerated. No case here duplicates them. RF-CHAIN generalizes N05 and N06 to longer chains and different lengths.

## 8. Independence, what was read and run

**Independence.** No product source (`P/core/**`, `P/apps/**`), product test, product fixture or `P/validation/benchmarks/numerical_integrity/src/**` was read, imported or called. Every value comes from the stated theory by the closed forms in §3. The author's stiffness self-check is separate code written from the theory, and it is not V2.

**Read** (SHA-256 of the bytes read; C/ = `P/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/`):

| Record | SHA-256 |
|---|---|
| `AGENTS.md` (root) | `c8ce87ef342902cb081bc659b26fc9a4edda1b6dba513814e5cb1e14e0b1dffd` |
| `agents/AGENT_TASK.md` | `1a13a5b00b3ce01ff8519efe6b46bcbe0cd6a5b7985e24282fa7efa2c57c8fb7` |
| `C/NUMERICAL_INTEGRITY_T3/TASK_BRIEFS/_COMMON.md` | `892a2e4e6f8b2e68967bd4c2a92996a141e93bf456bff12690248af91eac8fb3` |
| `C/NUMERICAL_INTEGRITY_T3/TASK_BRIEFS/R1_REFERENCES.md` | `0031453f75f30f342588783351c4db993ff34cebcd85115bf5ad47af55ddf99c` |
| `C/NUMERICAL_INTEGRITY_T3/STAGE1_PLAN.md` (with §8) | `f5c75dcf5bb1bb4a093696edc5f21ad13150fe832ac4d6a3a1f11ea118c476bb` |
| `C/NUMERICAL_INTEGRITY_T3/STAGE0_MAP.md` (at `e14f7fd13`) | `cfdb9dab66b0aa58070d02946c4c1b11abb4e101d4507b786c38d308f06cb0f2` |
| `C/NUMERICAL_INTEGRITY_T3/OWNER_DIRECTION.md` | `7e4068e5841489973ed52454fda84a7c07df12652c90ec2d7ea7b737e21f40db` |
| `C/CORRECTNESS_DESIGN/NUMERICAL_REFERENCE.md` | `67c8aac5798e9663528bea94b58d6d8f2eb8308f94b7ebb78aa22dca4b8f94ca` |
| `C/CORRECTNESS_DESIGN/CONTRIBUTION_PRECISION/CONTRACT.md` | `3e239d5535a1b1d864a8220e3716c42a9bace07bd9bd6376a8ff2d8f40a6ac3a` |
| `C/CORRECTNESS_DESIGN/CONTRIBUTION_PRECISION/INDEPENDENT_REFUTATION.md` | `62c969a407c74cae1db7fea8382331c89c94da42e996679234e2d1605b59d0b0` |
| `P/validation/benchmarks/numerical_integrity/README.md` | `0cb44d1f5fbc1ee0d3138c645b646b74d80346a1099e557a132dba79e5251505` |
| `P/validation/benchmarks/numerical_integrity/COVERAGE.md` | `114b34619dc33515199deeacfa81a757f0feecbac7238b3264f688ca071ea1a2` |
| `P/validation/benchmarks/numerical_integrity/generate.py` | `b491529a4bff8ed0e25de1f85c2488192a8e55677413571da6c260932b1a71a2` |
| `P/validation/benchmarks/numerical_integrity/test_reference.py` | `35b1b81895af8ec42e24fbb03dbec5c0233e5a2a57d1cccbc394cacee1a0c496` |
| `P/validation/benchmarks/numerical_integrity/fixtures.json` (first 3000 bytes read; whole-file hash) | `c061d73481d2ad137f7cef988475721681131789ec222e3e93c5ed3836b2910e` |
| `C/DEFAULT_ROUTE_DESIGN/references.py` (lines 1–200 read; whole-file hash) | `2451162b68ddf6fa2dca652a045689515f234baa5f73245c55f0d9a209569ec4` |
| `C/NUMERICAL_INTEGRITY_T3/TASK_BRIEFS/R1_ADDENDUM_CANCEL.md` (at `065c9ff60`, via git show) | `93c3c85d1c2b1bde80c8f47d7a1f08c29c68542a124e07890ae1c2f69bf46d81` |

`fixtures.json` was read only to its first 3000 bytes, for conventions. `DEFAULT_ROUTE_DESIGN/references.py` was read to line 200, for presentation style. `STAGE1_PLAN.md` was read in its current form, including §8, and `STAGE0_MAP.md` in its `e14f7fd13` form.

**Ran.** Everything ran single-threaded under `nice`, using standard-library Python only (Python 3.11.15 (CPython)), with scratch prototypes in `<scratch>`:

- `python3 -B references.py`: 679 s, exit 0.
- `python3 -B _run_records/selfcheck_stiffness.py`: 11 s, exit 0.
- `sha256sum`.

**Not done.** No cargo, npm or product build or test was run, and no Git write was made. No file outside `T3/REFERENCES/**` was written. No frozen reference, fixture, hash or protected criterion was edited. No tolerance was proposed.
