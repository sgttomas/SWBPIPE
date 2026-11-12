# RT-A3 increment 4 analytical references: nonlinear supports on the exact pressure route

These invented references cover R-4 stage S5. That stage admits gap,
one-way/lift-off and friction supports on the exact pressure route
(`3.0.0/exact_pressure_v3`). The cases are:
- R-4's G-1, G-2, G-3 and F-1, with their printed values reproduced;
- an explicit-normal friction twin and a Timoshenko twin of F-1;
- O-1, one-way lift-off under pressure through a realized bend, designed here;
- unpressurized controls of G-1, F-1 and O-1;
- mutation controls;
- the `NONLINEAR_COMBINATION_REQUIRES_SOLVE` withholding.

This answers requirement RT-V1 and exit criterion 2. K-1, the PF K-D5 controls
run on EX twins, is not part of this packet.

Governing source read at `6757b2341`:
- DEL-04-07 (R-4):
  - the codex round-1 corrections, which confirmed G-1, F-1 and F-2b, and
    the constitutive law EA ε = N_w − 2νpA_i;
  - order; R-3 comes later;
  - pressure as a fixed right-hand side in the loop, and K-D5;
  - S5 and its references;
  - F-2a/b, the context for derived normals.
- DEL-04-01:
  - the DEC-067 support laws;
  - states and k = 1 Stage A;
  - strict pass conditions;
  - fail-closed reasons.

  The task brief places this text under DEL-04-07; it is in DEL-04-01.
- DEL-04-07:
  - "Accepted T4 direction" H-2 and "T4-U2 as implemented": each realized arc
    owns K_b·u_free(ε_p) − c_b;
  - "Tees and pressure junctions": the caps −p·A_i·s.
- DEL-05-03:
  - section bases;
  - κ_E;
  - the Timoshenko element;
  - realized arcs stay Euler–Bernoulli.
- `validation/references/t4_i7/`: its unit-load engine defines the realized
  arc's flexibility. The integrand is N·n/EA + T·t/GJ + k·(M·m − T·t)/EI, with
  no shear term.
- DEL-05-02, for the combination rule.
- The sibling packets, for conventions:
  - `validation/references/rt_a3_tee/`;
  - `validation/references/rt_a3_reducer/`;
  - `validation/references/kd5_nonlinear/`;
  - `rt_a3_constant_effort/`. This one is not on main at `6757b2341`. It was read
    from commit `09521ebb8` on its own branch.

**Status: frozen analytical candidate.** Product agreement and independent
scrutiny are separate checks. Self-consistency does not establish product
conformance, qualification, acceptance or release.

## Independent basis

The standard-library generator imports, reads and runs no product code,
product output, fixture, result schema or generated envelope. All input
numbers are invented. No value comes from a standard, catalog or vendor source.

Every case is solved by two exact formulations.

- **A: force method with physical pressure loads.**
  - The cantilever is rooted at the anchor.
  - Pressure loads are physical:
    - at each `transfers_to_wall` terminal, a cap p·A_i on the wall along the
      outward axis;
    - on each realized arc, the fluid wall load p·A_i/R along the outward normal;
    - on every member, the Poisson eigenstrain −2νpA_i/(E·A).
  - Section resultants come from statics.
  - Displacements come from unit-load integrals, integrated exactly:
    polynomials on straights, and Wallis integrals of cos^a·sin^b over the
    quarter circle on arcs.
  - Linear restraints and nonlinear supports are redundants.
  - The state is found by hand-calculation trials in a fixed order: open or
    released first; for friction, stick, then slip each way. A takes the
    first state that satisfies every condition.
  - A forms no curved-element stiffness and no K·u_free.
- **B: direct stiffness with the H-2 ledger.**
  - An independently assembled 3-D frame:
    - straight members use the standard 12×12 Euler–Bernoulli or Timoshenko
      matrices;
    - each realized arc's 12×12 matrix is the inverse of a closed-form
      quarter-circle cantilever flexibility, derived by hand and mapped by
      equilibrium.
  - Pressure enters as the product's ledger representation:
    - Poisson pairs ±2νpA_i on straights;
    - caps −p·A_i·s at `transfers_to_wall` terminals;
    - K_b·u_free(ε_p) − c_b on each arc, where u_free is the stress-free
      homothetic expansion: relative end translation ε_p·chord, no relative
      rotation.
  - Every combination of support states is solved, including both signs β of
    a derived normal. Exactly one combination must satisfy every strict
    condition.
  - A derived-normal sliding force is solved as the coupled affine system
    φ = −dir·μ·β·R_n(u). That is DEL-04-01's intended system, with k = 1.

**What A and B share:**
- the input data and section formulas;
- the DEC-067 state laws and strict pass conditions (DEL-04-01), which
  define the problem;
- the T4-I7 arc flexibility definition, which A integrates generically and
  B writes out in closed form;
- the exact number class: rational functions of π over the rationals;
- the Gauss–Jordan helper.

Their pressure load formations differ. Their agreement therefore also checks
the H-2 ledger (K_b·u_free − c_b, Poisson pairs and caps) against the physical
wall loads.

A and B agree exactly on every published value of every case:
- each displacement and rotation;
- each reaction, including nonlinear supports and linear restraints;
- all six wall end-action components at both ends of every member;
- the wall force N_w.

That is 1,004 of the 1,510 exact checks. The others are:
- B's state enumeration: exactly one admissible state, and it equals A's;
- equilibrium:
  - global balance of physical loads and support actions;
  - node balance of physical loads, support actions and wall end actions;
  - member balance of wall end actions against the physical arc wall load;
  - the straight constitutive law EA(ε − ε₀) = N_w;
  - zero free-DOF residuals;
- complementarity:
  - gap/one-way: separation ≥ 0, contact ≥ 0, product = 0, and strict
    complementarity (not weakly active);
  - friction: |R_t| ≤ μN; stick has u_t = 0 and margin > 0; slip has
    sign(u_t) = dir and force −dir·μN;
- the derived-coupling identity φ = −dir·μ·β·R_n;
- B's own matrices:
  - its flexibility-to-stiffness map reproduces the standard straight 12×12;
  - the arc matrix is symmetric and annihilates all six rigid modes;
- the closed forms below, including every value R-4 prints;
- the identities:
  - G-3 = −G-1/10;
  - pressure does not move the explicit-normal transverse response;
  - the pressure contributions to F-1 and O-1;
- the combination illustrations and the mutation controls.

If any check fails, nothing is written and the exit status is 1.

**Number form.** Values are rational functions of π. Laurent forms are printed
as sums of q·π^k, and other forms as `(polynomial)/(polynomial)`. Machin's
identity with exact alternating-series remainders encloses π between rational
bounds 10⁻¹¹⁰ apart. `value` (binary64) is emitted only when both enclosure
endpoints round to the same double. Decimal arithmetic supplies display text
only.

**Generator self-test (not frozen).** Eight faults were injected into copies of
the generator. Seven made the run fail:
- the arc wall-load sign in A;
- an arc flexibility coupling sign in B;
- the Poisson-pair sign in B;
- a doubled u_free in B;
- a 1 % error in ε_p;
- a non-strict gap test;
- a wrong Wallis recursion.

The eighth survived: k applied to the arc's axial flexibility along its first
basis vector e₁. In every O-1 case no effective force crosses the bend in that
direction, so that entry cannot affect a published value. The out-of-plane arc
block is likewise unexercised, because all O-1 loading is in plane. That arc
element is covered by T4-I7, not here.

## Model and inputs

| Quantity | Value |
|---|---|
| E, ν, G | 200 GPa; 3/10; E/(2(1+ν)) = 10¹²/13 Pa |
| Section | S1 header: D = 0.2 m, t = 0.01 m, c_m = c_a = 0 (r_i = 0.09 m). The nominal and reduced bases coincide, so each stress value is the value of both basis families. |
| Section properties | A_n = 19π/10⁴ m²; I = 3439π/(4·10⁸) m⁴; J = 2I; A_i = 81π/10⁴ m²; A_i/A_n = 81/19 |
| Pressure | p = 10 MPa unless stated. One region over every member. ε_p = (1−2ν)pA_i/(E·A_n) = 81/950000 at 10 MPa. N_w = S + pA_i. |
| Shear model | `euler_bernoulli`, except the F1-T cases (`timoshenko_annulus_energy_v1`, κ_E = 16609827/33068756 exact). Realized arcs are Euler–Bernoulli under either model. |
| Loads | Nodal forces only; no self-weight |

**Conventions.**
- A reaction is the support-on-pipe force r = (K·u − f)_d in global axes. Here
  f includes any cap at that node (R-4 §2.1). For example, G-1's gap reaction
  is −13,400π N and its contact force is +13,400π N.
- The contact force is the magnitude: −r for a gap closing in +, and +r for one
  closing in − or for a one-way support of sense +.
- A wall end action is the force or moment that the node exerts on the
  member's wall at that end, in global axes. N_w is its component along the
  outward end tangent, and S = N_w − pA_i.
- State names:
  - `closed`/`open` for gaps;
  - `active`/`inactive` for one-way supports;
  - `stick`, `slide±` (the slip sense on the friction DOF) and `inactive` for
    friction.

  A derived normal appends `/b±`, where β = sign(R_n) of the source restraint.
  F-1 slides as `slide+/b-`: R_n = −32,400π N.
- An inactive support's reaction is exact 0. DEL-04-01 R4 publishes it as +0.

## Cases and key values

| Case | Configuration | State | Key exact values |
|---|---|---|---|
| **G1** (R-4 G-1) | Pipe N0(0,0,0)→N1(10,0,0). N0 anchored and `separately_supported_or_compensated`; cap at N1. Gap on UX at N1, closing in +X, clearance g = 0.5 mm. | closed | Free elongation 81/95000 m; overlap 67/190000 m; contact **13,400π N**; **N_w = 67,600π N**; **σ_a = 676,000,000/19 Pa**; anchor R_X = −67,600π N; u_X(N1) = 1/2000 m |
| **G2** (R-4 G-2) | G1 with g = 1 mm | open | Contact **+0**; u_X(N1) = **81/95000 m**; separation 7/47500 m; N_w = 81,000π N |
| **G3** (R-4 G-3) | G1 with p = −1 MPa and a gap on UX at N1 closing in −X, g = 0.05 mm | closed | Free contraction −81/950000 m; overlap **67/1,900,000 m** (0.0353 mm); contact **1,340π N** (signed reaction +1,340π); **N_w = −6,760π N**; σ_a = −67,600,000/19 Pa. Exactly −1/10 of G1. |
| G1-P0 | G1 with p = 0 and no regions | open | Every motion and reaction is exactly 0; separation 1/2000 m |
| G1-HALF | G1 at p = 5 MPa (combination illustration) | open | u_X = 81/190000 m; contact +0 |
| **F1-SLIDE** (R-4 F-1) | Cantilever N0(0,0,0)→N1(3,0,0). N0 anchored and compensated; cap at N1. Linear UX restraint at N1 is the normal source. Friction on UZ at N1, μ = 0.3. F_z = 50 kN at N1. | slide+/b- | R_n = −32,400π N; **N = 32,400π N**; **μN = 9,720π N**; **u_z = 27(50,000 − 9,720π)/(3·1,719,500π) m** = −4374/85975 + 900/(3439π) (32.4277 mm); friction **−9,720π N**; anchor R_X = **−48,600π N** (wall tension); **σ_a = 486,000,000/19 Pa**; anchor R_Z = 9,720π − 50,000 N, M_Y = 150,000 − 29,160π N·m |
| **F1-STICK** (R-4 F-1) | F1 with F_z = 25 kN | stick | **u_z = 0**; **R_t = −25,000 N**; stick margin 9,720π − 25,000 N (5,536.28 N) |
| F1E-SLIDE / F1E-STICK | Explicit-normal twin: N = 100 kN entered (μN = 30 kN); F_z = 50 / 25 kN | slide+ / stick | Slide: u_z = 360/(3439π) m, friction −30,000 N. Stick: R_t = −25,000 N, margin 5,000 N. Axial values as F1. |
| F1-T-SLIDE / F1-T-STICK | F1 with Timoshenko straight members | as F1 | Slide: u_z = (50,000 − 9,720π)·[L³/(3EI) + L/(κ_E·G·A_n)]; the tip rotation equals Euler–Bernoulli's (a section rotation). Stick as F1. |
| F1-P0-SLIDELOAD / -STICKLOAD | F1 with p = 0 and no regions; F_z = 50 / 25 kN | inactive | R_n = 0 exactly, so N = 0 and there is no friction. u_z = F_z·L³/(3EI) = 900/(3439π) and 450/(3439π) m. |
| F1E-P0-SLIDE / -STICK | Explicit twin with p = 0 | slide+ / stick | Transverse values identical to F1E (pressure does not enter an explicit normal) |
| F1-MID | F1 with F_z = 37.5 kN (combination illustration) | slide+/b- | u_z = −4374/85975 + 675/(3439π) m |
| **O1-LIFT** (O-1, designed here) | See below. W = 100 N at S, p = 10 MPa. | inactive | Free pressure lift at S **ε_p(h+R) = 1053/9,500,000 m**; f_SS below; **u_Z(S) = ε_p(h+R) − W·f_SS = 509469/5502400000 − 286543/(2063400000π) m** (0.0483868 mm); rest reaction +0 |
| **O1-CONTACT** | O1 with W = 400 N | active | **R_S = W − ε_p(h+R)/f_SS = (4,584,688,000 − 312,274,800π)/(1,506,429π + 11,461,720) N** (222.5256 N) |
| O1-VAC | O1 with p = −1 MPa, W = 0 | active | R_S = 91,484,640π/(1,506,429π + 11,461,720) N (17.7474 N), exactly −1/10 of the 10 MPa pressure contribution |
| O1-P0-W100 / -W400 | O1 with p = 0 and no regions | active | R_S = W exactly (100 N and 400 N): the support is at the loaded node. The pressure contribution is therefore R(O1-CONTACT) − 400 = −ε_p(h+R)/f_SS. |

**R-4's printed values are reproduced exactly:**
- G-1: 81/950,000, 81/95,000, 67/190,000, 13,400π, 67,600π, 676,000,000/19;
- G-2: +0 and 81/95,000;
- G-3: −81/950,000 and the 0.0353 mm overlap (67/1,900,000);
- F-1: 32,400π, 9,720π, u_z, −9,720π, 48,600π, 486,000,000/19, 0 and −25,000.

R-4 does not print G-3's contact force or wall force. They are completed here:
1,340π N and −6,760π N.

Each case in `references.json` publishes:
- inputs and the expected standing;
- each support's state and deciding quantities;
- all nodal motions;
- every reaction;
- per member and end: the wall end action, N_w, S and N_w/A_n;
- the closed forms;
- for pressurized cases, the Lamé rows, which supports do not change.

## O-1: design, formulation relied on, and what is closed-form

R-4 §3.5 leaves O-1 "to be built". It asks for "a horizontal pipe on a one-way
vertical support near a capped end, with an axial elbow-free configuration
chosen so that pressure thrust through a realized bend lifts the support", in
which "the bend's K_b·u_free term enters".

**Geometry** (X–Z plane, Z up; h = 1 m, R = 0.3 m (1.5D), a = 1 m, c = 0.5 m).
- A(0,0,0) is anchored.
- The riser A→B1 runs up to B1(0,0,1).
- The realized 90° bend B1→B2 has its center at (0.3,0,1), turns from +Z to +X
  and ends at B2(0.3,0,1.3). Its k = 2 (a user value).
- The run B2→S goes to S(1.3,0,1.3), with a one-way rest support on UZ, sense
  +, which can only push up.
- The run continues to the cap C(1.8,0,1.3), 0.5 m beyond S.
- One region covers every member, `transfers_to_wall` at A and at C.
- The chain is tangent-continuous, so there are no kink or mitre terms.

**Realized-bend formulation relied on**, quoted from DEL-04-07
("T4-U2 as implemented"): "Each arc owns K_b·u_free(ε_p) − c_b, with
ε_p = (1−2ν)pAi/(E·As) formed in Scaled arithmetic and c_b its end caps along
the end tangents." The batch-4 dependency adds that the arc pushes
"`PressureFreeStrainCurved` to the same K_int,arc and raw (1−2ν)pAi/(EA) law".
Section-bases §1 adds: "As = nominal arc area".

This packet takes u_free(ε_p) to be the stress-free expansion of the arc: a
relative end translation of ε_p·chord, with no relative rotation. That is what
a uniform axial strain gives in the unit-load integral, ∫ n·ε₀ ds = ε₀·chord.

**Why it is closed-form.**
- With both terminals transferring to the wall, pressure acts on the pipe only
  as the eigenstrain ε_p. In effective terms, the wall tension pA_i balances
  every cap and bend wall load.
- With the support inactive, the line is a determinate cantilever, so the
  eigenstrain produces free deformation only. S rises by the riser's growth
  ε_p·h plus the bend chord's vertical growth ε_p·R. The bend's term is exactly
  its K_b·u_free.
- This lift, ε_p(h+R), is independent of k, of the arc stiffness and of the
  bend's flexibility. The mutant that omits K_b·u_free loses exactly ε_p·R
  (checked).
- What depends on the arc stiffness is the weight's deflection at S, through
  the vertical flexibility:

  f_SS = [a³/3 + k·R(a²π/2 + 2aR + R²π/4) + h(a+R)²]/EI + (Rπ/4 + h)/EA

  This is closed-form at the quarter circle, using ∫cos² = π/4. It enters the
  lift-off separation and the contact reaction.
- All loading is in plane, so only the arc's in-plane bending (with k) and its
  axial flexibility matter. How k is applied out of plane cannot affect O-1.

**Why the physical "thrust" is not an external force here.** In a closed
region the bend thrust p·A_i(t_out − t_in) is equilibrated by the wall tension.
What lifts the support is pressure growth through the bend and riser. R-4's
"axial elbow-free" wording conflicts with "through a realized bend". This
design keeps the realized bend and has no mitre or kink (see choice 3).

**Alternative considered, not built.** A run anchored at its far end, with the
bend turning down into a riser ending at a `separately_supported_or_compensated`
terminal. That terminal's unbalanced thrust +pA_i·Ẑ would lift the support
through the bend. Its support state is K_b-independent, but there it is the
bend's c_b part that lifts, not K_b·u_free. The design above matches R-4's
"K_b·u_free term enters".

## Mutation controls

Each mutation is solved with formulation B, every admissible state is
enumerated, and the result is compared with the reference. Detection means the
state changes, or a published value moves by more than 10³ tolerances, where a
tolerance is 1e-9·max(|ref|, the largest |ref| in its group).

| Control | Mutation | Detected by |
|---|---|---|
| MUT-GAP-SIGN | G1 with the gap closing in −X | State closed → open; u_X = 81/95000 m instead of 1/2000 m (7.1e8 tolerances) |
| MUT-NORMAL-SOURCE-SLIDE | F1-SLIDE with N from the anchor's UX reaction (48,600π) instead of the N1 UX restraint | u_z = −6561/85975 + 900/(3439π) m (7.8e8 tolerances) |
| MUT-NORMAL-SOURCE-STICK | The same on F1-STICK | It still sticks with R_t = −25,000 N. Detected only by the published derived-normal row (48,600π vs 32,400π N) and the stick margin. |
| MUT-NORMAL-NO-PRESSURE-SLIDE / -STICK | N formed without the pressure terms (N = 0) | State → inactive; u_z = 900/(3439π) and 450/(3439π) m. The stick case moves a reference group that is exactly 0. |
| MUT-ARC-UFREE-LIFT | O1-LIFT without the bend's K_b·u_free term (c_b kept) | The lift falls by exactly ε_p·R = 243/9500000 m (1.5e8 tolerances) |
| MUT-ARC-UFREE-CONTACT | O1-CONTACT without K_b·u_free | R_S rises by exactly ε_p·R/f_SS (2.3e8 tolerances) |
| MUT-ARC-ALL-LIFT | O1-LIFT without the whole bend term K_b·u_free − c_b | The unbalanced caps pull the run down: the support stays active, R_S ≈ 88 kN |

## Combinations

In a model with any nonlinear support, a `mechanics` combination is withheld as
`NONLINEAR_COMBINATION_REQUIRES_SOLVE` (the PF rule,
DEL-05-02; S5 lifts it onto EX, where
DEL-05-02 says it "cannot arise" today), with one adopted
exception. Decision OQ-1 (Agent 0, 2026-10-31; DEL-05-02)
admits it exactly when one term
names a solved load case with factor exactly 1, it has at least one other term,
and every other term names a combination whose κ is exactly 0 (a difference, an
absolute sum or an SRSS of κ-zero operands, or a combination of those; an
undefined κ does not qualify). Such a combination is formed from rows with
operand standing and reads "a solved state plus derived differences or bounds;
not an equilibrium state". For example, with D = result_state_subtraction(O1-CONTACT,
O1-LIFT) and E = srss(D), mechanics(1·O1-CONTACT + 1·E) is admitted. Outside
that shape the withholding is a model condition, independent of factors,
states and pressure. These references give no values for an OQ-1 combination.

Withheld:
- every flat mechanics combination over the G, F or O models (all terms name
  load cases), including the singleton mechanics(1·G1), which OQ-1 excludes
  because it has no other term;
- unit-sum combinations, such as 0.3·O1-LIFT + 0.7·O1-CONTACT;
- combinations whose operands share a state;
- absolute-sum/SRSS forms outside OQ-1's admitted shape
  (DEL-05-02 V-9).

Formed as derived results with operand standing: `result_state_subtraction`
and `range_envelope`.

`references.json` shows why mechanics combinations are withheld:
- 0.5·G1 publishes contact −6,700π N and u_X = 1/4000 m, but G-1 solved at
  5 MPa is open with u_X = 81/190000 m.
- 0.5·F1-SLIDE + 0.5·F1-STICK publishes R_t = −4,860π − 12,500 N, but the
  solved F_z = 37.5 kN state slides at −9,720π N.

## Expected standing

| Cases | Expected label |
|---|---|
| G1, G2, G3, G1-P0, G1-HALF (gap-bearing) | `NUMERICAL_INTEGRITY_RECOVERY_BASIS_UNQUALIFIED`: R1's unqualified branch. No confirmation runs until R-3 (R-4 §3.5; DEL-04-01 R1). No qualification is claimed. |
| F1-P0-SLIDELOAD, F1-P0-STICKLOAD | `NUMERICAL_INTEGRITY_SENSITIVE`, with "support support:FRICTION: friction normal not established". A derived-normal friction support in state Inactive fails closed (DEL-04-01). The rows are the loop's. |
| All other cases | K-D5 confirmation applies: k = 1 Stage A for derived F cases; k = 0 (Stage B only) for explicit F and O cases. Its outcome is the product's and is not asserted here. Every deciding quantity is strictly signed, so no case is weakly active. |

## Choices and ambiguities (stated, not silently resolved)

1. **Section bases.** c_m = c_a = 0 reproduces R-4's A_i = 81π/10⁴. Product
   documents must enter a zero corrosion allowance explicitly, which the
   contract requires, and select `euler_bernoulli` except in F1-T. R-4's F-1
   u_z formula is Euler–Bernoulli, and the authoring default is Timoshenko.
2. **Explicit-normal twin.** N = 100 kN is entered rather than 32,400π, which
   binary64 cannot represent. μN = 30 kN keeps 50 kN sliding and 25 kN
   sticking. A twin at fl(32,400π) would agree with F-1 to about 10⁻¹⁶ but has
   no tidy exact form.
3. **O-1 wording.** R-4's "axial elbow-free configuration … through a realized
   bend" is self-conflicting. It is read here as one realized bend, with no
   mitre and no kink. See the O-1 section for the reading of "thrust".
4. **O-1 relies on T4-I7's arc definition** for f_SS: k on bending, axial and
   torsion unmodified, no shear. The free pressure lift and the K_b·u_free
   share do not depend on it. The weight's deflection and the contact reaction
   do.
5. **The support-law text's location.** The brief places
   it under DEL-04-07; it is in DEL-04-01.
6. **The constant-effort sibling** (`rt_a3_constant_effort/`) is not on main at
   `6757b2341`. It was read from commit `09521ebb8`.
7. **Gap reaction sign.** R-4 prints the contact as a magnitude. The product's
   reaction row is signed (K·u − f)_d, for example −13,400π N in G1. Both are
   published.
8. **Derived inactive standing.** Expecting Sensitive for F1-P0 follows
   DEL-04-01's table. If the product publishes another standing for an exactly-zero
   derived normal, that is a question about the governing text, not a
   tolerance.
9. **Mutation "normal from the wrong source".** It is modelled as the anchor's
   UX reaction, the only other linear restraint on that axis. In stick it
   changes no motion or reaction, only the normal row and the margin.
10. **Pressure omitted from the normal.** It is modelled as N taken from the
    unpressurized state, which is 0.

## Scope exclusions

- Connectors and tie rods with nonlinear supports, and load/reference-state
  documents. These are S6 (R-4 §3.7).
- A nonlinear support as a friction normal source, and k > 1. These are S7
  (F-2a/b/3).
- Gap qualification and two-sided stops. These are R-3 and S8. Gap cases carry
  the unqualified label only.
- Held or locked devices, which belong to load cases (HYDROTEST, R-4 §4.9).
- K-1, the PF K-D5 controls on EX twins, which are run by the product binding,
  not produced here.
- Bourdon effects and pressure stiffening (D-3), self-weight, thermal loads,
  tees, reducers and code stresses.
- The out-of-plane arc flexibility block, and the arc's e₁ axial term. These
  are not exercised (see the self-test note).

## Points for a numerical reviewer

- Check the closed-form quarter-circle flexibility in
  `arc_flexibility_local` (B) independently. Its in-plane block is exercised by
  O-1 through A's generic integration. Its out-of-plane block, and f₁₁'s
  dependence on k, are not.
- Check that u_free is the homothetic expansion and that c_b = p·A_i along the
  outward end tangents. A and B agree only if the ledger K_b·u_free − c_b
  equals the physical arc wall load plus Poisson eigenstrain.
- Check the sign convention for β: F-1 has R_n < 0, so β = −1.
- Check that the published wall end action of an arc is K_b(d − u_free) +
  p·A_i·(outward tangent). On straights it is k·d + the Poisson held action.
- Check that every case is strictly complementary. The smallest margins are:
  - stick: 5,000 N (F1E) and 9,720π − 25,000 ≈ 5,536 N (F1);
  - contact: 17.75 N (O1-VAC);
  - separation: 0.048 mm (O1-LIFT).

## Reproduce

```sh
python3 -I -B generators/rt_a3_nonlinear_reference.py --check
python3 -I -B generators/rt_a3_nonlinear_reference.py --summary
shasum -a 256 -c SHA256SUMS
```

`--write` regenerates `references.json` and `SHA256SUMS` on purpose. A normal
run only checks their bytes. Investigate a disagreement against the independent
basis and the review. Never repair it by matching a product output.
