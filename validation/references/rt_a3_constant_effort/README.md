# RT-A3 increment 3 analytical references: constant-effort supports on the exact pressure route

These invented references cover ideal constant-effort supports (DEC-049)
admitted on the exact pressure route (`3.0.0/exact_pressure_v3`): the R-4 S2
cantilever (CE-1), its non-consumption controls (CE-1n), mechanics
combinations under the unit factor-sum rule (CE-2), and a load-state
document under D-2 (CE-3). Requirement RT-V1, exit criterion 2.

Governing source read at `575fcc776`:
- DEL-04-07 `Work/route-unification-r4.md` §2.1, §3.2 (S2: CE-1, CE-2), §4 and
  §4.7/§4.7a, §6.3 D-2;
- D-2 as decided by Agent 0 and supplied in the task brief (not yet in the
  repository text): on load-state documents the constant force is a device
  action of the active support in every state. A held/locked state belongs to
  hydrotest, in the load-case lane, and is not built here;
- `Work/pressure-contract.md`: "Constant-effort combinations: the unit
  factor-sum rule" and "Tees and pressure junctions" (D-1 caps);
- `validation/hand_calcs/mechanics/constant_effort_support_applied_load.md`,
  which gives the DEC-049 semantics and sign convention;
- DEL-05-03 `Work/section-bases-and-timoshenko.md` §1, §2.1 (D-5 bases), §3.1
  (κ_E) and §3.3 (Timoshenko coefficients).

**Status: frozen analytical candidate.** Product agreement and independent
scrutiny are separate checks. Self-consistency does not establish product
conformance, acceptance or release.

## Independent basis

The standard-library generator imports, reads and runs no product code, product
output, fixture, result schema or generated envelope. All input numbers are
invented. No value comes from a standard, catalog or vendor source.

It solves every case by two exact formulations. They share only the input
data, the section formulas and the consumption rule that turns a
constant-effort record into a nodal force.

- **A: force method.** The first anchor is the root of the released tree, and
  every other anchor is released into six redundants. Section resultants come
  from statics. Displacements come from unit-load (virtual-work) integrals:
  ∫ nN/EA + m·M/EI + tT/GJ (+ v·V/(κ_E G A) for Timoshenko) + n·ε₀. These are
  integrated exactly; Simpson's rule is exact for the quadratic integrands.
  Reactions and end actions come from statics.
- **B: direct stiffness.** An independently assembled 3-D frame stiffness
  system. The 12×12 member matrices (Euler–Bernoulli, or Timoshenko with
  φ = 12EI/(G A_s L²)) are rotated by exact direction cosines. Caps and the
  constant force enter as nodal loads, and eigenstrains (Poisson pair,
  thermal) as held end actions. Exact Gauss–Jordan elimination over Fractions
  runs per power of π. Reactions are R = K u − f; end actions are k d plus
  the held actions.

A and B agree exactly on every published displacement, reaction, end action
and wall force of every solved state: 1,503 of the 2,075 exact checks. The
others are:
- node, member and global equilibrium;
- constitutive and closed-form checks: the textbook cantilever laws, R-4's
  (F_ce − W)L³/(3EI), and the Timoshenko shear term PL/(κ_E G A_n);
- superposition and cancellation identities;
- gate decisions;
- the mutation controls.

If any check fails, the generator writes and verifies nothing.

Areas, inertias and moduli are rational coefficients of π. Each published
value is a finite sum of q·π^k or √q·π^k terms. Machin's identity with exact
alternating-series remainders encloses π between rational bounds 10⁻¹¹⁰ apart.
`value` (binary64) is emitted only when both enclosure endpoints round to the
same double. Decimal arithmetic supplies display text only.

## Model and inputs

| Quantity | Value |
|---|---|
| E, ν, G | 200 GPa; 3/10; E/(2(1+ν)) = 10¹²/13 Pa |
| α (CE-3 thermal) | 3/250000 K⁻¹ (12·10⁻⁶) |
| Section (S1 header) | D = 0.2 m, t = 0.01 m, c_m = 0.5 mm, c_a = 1 mm |
| Bases (D-5) | nominal wall t (r_i,n = 0.09 m) for A, I, J and the shear area; reduced wall t_r = (t − c_m) − c_a = 17/2000 m; pressure bore r_i,r = 183/2000 m, A_i = π r_i,r² |
| κ_E (§3.1, exact, nominal section) | 16609827/33068756 (0.50228…); A_s = κ_E A_n |
| Pressure | p = 10 MPa; every region one member, both terminals `transfers_to_wall` |
| Global axes | Z is vertical (up); a weight W acts along −Z |

The constant-effort rule follows DEC-049 and the hand calc. A record is
consumed when it declares exactly one translational restraint DOF and a
positive `hanger.constant_load`. It then applies a constant nodal force of that
magnitude along the **positive** global axis of that DOF, in every solved case
and state. It has zero stiffness and adds no restraint row. A record that does
not meet these conditions is not consumed: it applies no force and no
restraint.

**Conventions.**
- A reaction is the force or moment the anchor exerts on the pipe
  (R = K u − f, global axes).
- A member end action is the force or moment the node exerts on the member at
  that end, in global axes. The axial part at the far end is +N_w.
- N_w is the statics wall force, and S = N_w − pA_i.

**Stress rows.** Rows are given per basis b at both member ends. Every member
carries only nodal loads, so its moment is linear and the ends are the
extreme stations. Each end has:
- axial σ = N_w/A_b;
- bending |M_⊥|/Z_b, with √ when two planes are loaded;
- torsion |T|·r_o/J_b.

The member's elastic normal maximum per basis is |N_w|/A_b + |M_⊥|/Z_b at the
governing end. Pressurized cases add the Lamé rows of each basis's own annulus.

The resultant |M_⊥| is frame-invariant. Signed member-local y/z components
depend on the product's local axes and are not fixed here.

## Cases

| Case | Configuration | Key values |
|---|---|---|
| CE1-EB | Cantilever A(0,0,0)→B(5,0,0), anchored at A. Constant-effort support on UZ at B, F = 8000 N (+Z). Tip weight W = 3000 N (−Z). Unpressurized. | tip UZ = (F−W)L³/(3EI) = 1250/(10317π) m (0.0385662); RY = −125/(3439π) rad; R_A: FZ = −5000 N, MY = +25000 N·m; without the record UZ = −250/(3439π); F-only UZ = 2000/(10317π) |
| CE1-EB-P | CE1-EB in one capped region at 10 MPa | Transverse values identical to CE1-EB. N_w = pA_i = 167445π/2 N with or without the support. Tip UX = (1−2ν)pA_iL/(EA_n) = 33489/76000000 m. R_A.FX = 0. σ_m = 837225000/19 Pa (nominal) and 334890000000/6511 Pa (reduced). |
| CE1-T, CE1-T-P | The same, Timoshenko | tip UZ = (F−W)[L³/(3EI) + L/(κ_E G A_n)] = 2949517189/(24275901000π) m; RY unchanged (section rotation); φ(5 m) = 8267189/735312500 |
| CE1-EB-MUT-SIGN | **Mutation control**: force on −Z | Differs from CE1-EB by −2 × the F-only solution |
| CE1-EB-MUT-TWICE | **Mutation control**: force counted twice | Differs by +1 × the F-only solution |
| CE1N-2T-{EB,T} | Record declares UY and UZ (two translational DOFs) | Not consumed: no force, no applied-load row. Tip UZ = −WL³/(3EI) (EB) and its Timoshenko twin. Identical to the model without the record. |
| CE1N-ROT-{EB,T} | Record declares RX only | As CE1N-2T |
| CE2 | CE-1 model (EB), two cases, both carrying the support. Case A: W. Case B: W plus a lateral tip load Q_y = 2000 N (+Y), plus p = 10 MPa. | Combinations below. Case B's bending row is √(M_Y² + M_Z²)/Z_b. |
| CE3-EB | Load-state document. L-line A(0,0,0)→C(4,0,0)→B(4,0,−3): a riser down to anchor B; pipe:2 is authored B→C. Constant-effort support on UZ at the corner C, F = 4500 N. W = 6000 N at C. State S at the reference temperature; state O at +100 K. | The constant-force row is present in both states. O − S equals the thermal-only solution exactly, and is independent of F. Thermal UZ at C = 1455377472/405751901443 m. |
| CE3-EB-P, CE3-T-P | CE3 with p = 10 MPa in two regions in both states. The regions meet at the corner C, a D-1 junction with caps +pA_i X̂ and +pA_i Ẑ at C. | O − S as CE3-EB (EB) or its Timoshenko counterpart. Pressure and weight cancel as well. |
| CE3-EB-MUT-OMIT-O | **Mutation control**: force omitted from state O only | O − S = thermal-only − F-only; detected |

**CE-2 combinations.** Σf is the exact sum of the binary64 factors. A
combination is admitted iff |Σf − 1| ≤ 2⁻⁴⁶, compared exactly with closed
bounds. For an admitted combination:
- the published values are the exact factor sums of the case values;
- the constant force is counted Σf times, and its excess (Σf − 1)·F is
  published;
- the check "combined − the true single-F system with the same factored loads
  = (Σf − 1) × the F-only solution" holds exactly.

| Combination | Exact Σf | Gate |
|---|---|---|
| 0.3·A + 0.7·B | 1 − 2⁻⁵⁴ = 18014398509481983/18014398509481984 | admitted |
| 1.2·A − 0.2·B | 1 − 2⁻⁵⁴ | admitted |
| fl(1/3)·A + fl(2/3)·B | 1 − 2⁻⁵⁴ | admitted |
| (1 ± 2⁻⁴⁶)·A | 1 ± 2⁻⁴⁶ (closed bounds) | admitted |
| 2·A | 2 | withheld, `CONSTANT_EFFORT_COMBINATION_REQUIRES_SOLVE` |
| (1 + 2⁻⁴⁵)·A | 1 + 2⁻⁴⁵ | withheld |
| 2⁵²·A − 2⁵²·B | 0 (2⁵² < 2⁵³, so the value entry admits it) | withheld |
| 2⁵³·A − 2⁵³·B | 0 | refused upstream by the value entry (`CHECKED-JSON-UNSAFE-INTEGRAL-FLOAT`); a typed entry reaching the gate is withheld |

Combinations publish linear quantities only: motions, reactions, end actions,
N_w and N_w/A_b. No |M| magnitude, maximum or Lamé row is published for a
combination. Whether the product emits an applied-load row on a combination is
not fixed here; Σf·F is given for a reader.

## Choices and ambiguities (stated, not silently resolved)

1. **CE-1 orientation.** R-4 §3.2 says "a vertical cantilever", but its
   formula (F_ce − W)·L³/(3EI) is a transverse tip deflection, and a UZ force
   on a vertical member would be axial. This packet therefore uses a
   horizontal cantilever along +X, with the support on UZ at the free tip.
   R-4's formula then holds exactly (checked).
2. **CE-3 geometry.** The L-line lies in the vertical X–Z plane, with the
   riser running down from the hanger corner to a second anchor. Thermal
   growth then moves the hanger node *along the support's DOF* (UZ at C is
   nonzero, checked), which shows the zero stiffness of the ideal support.
   Weight, constant force and thermal response couple in one plane through
   frame action; all are linear. A cantilever with only axial growth was
   rejected because it would not exercise the support DOF. W is a nodal
   force; self-weight is not modelled.
3. **D-2.** D-2 is taken from the brief, as Agent 0's decision; it is not
   yet in the repository text. The force is present in each state, and O − S
   cancels it exactly (R-4 §3.2 "Interactions"). The O − S block is published
   as exact state differences.
4. **Non-consumed records.** No force, stiffness or restraint follows from a
   non-consumed record. Sources: the hand calc; the 2026-07-19 T2 brief, where
   such a record "remains user-data review evidence"; and the task's
   −WL³/(3EI). The warning code `SUPPORT_CONSTANT_EFFORT_NOT_CONSUMED` is the
   name recorded in `docs/validation_manual/headless_runner_reproduction.md`.
   It is not derived here.
   - A record declaring one translational plus one rotational DOF (e.g. UZ,
     RX) has "exactly one translational DOF". Whether it is consumed is not
     settled by the governing text, and no reference is given for it.
   - Non-positive `constant_load` is also outside these cases.
5. **Pressure on load-state documents.** CE3-EB-P and CE3-T-P assume that EX
   load-state documents admit per-state pressure regions. If the product
   refuses them, those two cases are unexercised; CE3-EB stands on its own.
6. **Timoshenko factor.** κ_E is the exact rational value of §3.1's formula.
   The product forms a binary64 leaf, which differs by about 10⁻¹⁶
   relative, below any binding tolerance.
7. **Section bases.** Every case has nonzero c_m and c_a.
   - Per basis: the axial, bending and torsion rows, the elastic maximum and
     the Lamé rows.
   - Once: motions, reactions, end actions, N_w and S.
   - The pressure bore (reduced) sets A_i in caps and the Poisson pair.
     Stiffness and thermal use the nominal section.
8. **Combination gate extras.** The ±2⁻⁴⁶ boundary pair and 1 + 2⁻⁴⁵ are
   added to R-4's CE-2 pair to pin the closed bound.

## Scope exclusions

The following are out of scope:
- held/locked devices (HYDROTEST H-3, `locked_equivalent_support`), which
  belong to the load-case lane;
- nonlinear supports, and a constant-effort record with a `nonlinear` field;
- combinations of other support families, and connector/tie-rod gates;
- spring hangers;
- travel and movement-limit comparisons;
- bends, tees and reducers;
- self-weight, and per-state E;
- code stresses.

## Reproduce

```sh
python3 -I -B generators/rt_a3_constant_effort_reference.py --check
python3 -I -B generators/rt_a3_constant_effort_reference.py --summary
shasum -a 256 -c SHA256SUMS
```

`--write` intentionally regenerates `references.json` and `SHA256SUMS`. Normal
execution only checks their bytes. A disagreement is investigated against the
independent basis and review, never repaired by matching a product output.
