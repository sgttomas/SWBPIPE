# RT-A3 S6 references: nonlinear supports with connectors and on load/reference-state documents

Invented analytical references for R-4 stage S6 (DEL-04-07):
**C-1**, a pressurized objective joint
between two anchors with a one-way guide on one side, with its controls; and
**LS-1**, W-16 stage 2's references run as exact-route load/reference-state
twins. The consumer is
`core/product_physics/src/rt_a3_s6_tests.rs`, in both solver modes.

**Status: frozen analytical candidate.** Product agreement and independent
scrutiny are separate checks. Nothing here records or implies acceptance,
qualification, issuance or release (F-PIP-3).

## Independent basis

`generators/rt_a3_s6_reference.py` uses the Python standard library only. It
imports, reads and runs no product code, product output, fixture outside the
W-16 packet or generated envelope. Its only inputs besides its own invented
numbers are the frozen W-16 request documents and oracle
(`../w16_nonlinear_motion/requests/*.json`, `references.json`), each checked
against that packet's `SHA256SUMS` and oracle digest. The W-16 packet is not
modified.

Every case is solved twice in exact rational arithmetic (`fractions.Fraction`):

- **A, force method with physical loads.**
  - LS-1: the cantilever clamped at the (possibly moved) anchor; the tip support
    actions are the redundants; flexibilities are the exact unit-load integrals
    ∫m·m/EI (+ ∫v·v/(κ_E·G·A) for Timoshenko), with RY the section rotation;
    the anchor's motion is a rigid-body field w = w_A − θ_A·x, θ = θ_A; a
    spring acts with −k·(u − g_k). Anchor reactions come from statics.
  - C-1: axial free bodies. Each pipe wall carries N = EA·ε + 2νpA_i (Lamé);
    the fluid presses each bellows face annulus with p(A_e − A_i) away from the
    joint; the joint carries F_j = k_j·(u_C − u_B); the caps at A and D push
    −pA_i and +pA_i on the closures carried by the wall.
- **B, direct stiffness with the product's ledger representation.**
  - LS-1: the Euler–Bernoulli or Timoshenko (φ = 12EI/(κ_E·G·A·L²)) element in
    the loaded x–z plane; partitioned prescribed values (moved anchor, rigid
    restraint at its base value, an active one-way/lift-off at its moved
    ground); a moved spring base enters as the load k·g with k on the diagonal.
  - C-1: the ledger of DEL-04-07: caps −pA_i at
    A and +pA_i at D, Poisson pairs ±2νpA_i on each pipe, and at the joint the
    effective-area summands −pA_e at B and +pA_e at C with the face caps
    +pA_i at B and −pA_i at C.
  - B enumerates every state of every one-way, lift-off or gap support and keeps
    those satisfying DEC-067's laws on δ = u − g (strict: every margin > 0).

A and B share the input reading, the section leaves and the DEC-067 state laws.
The run fails (nothing written, exit 1) unless: B finds exactly one strict
state and no weak one; A at that state equals B on every displacement and
support action; the C-1 support actions balance (the closed fluid system
exerts no net force); every Euler–Bernoulli LS-1 value lies in W-16's frozen
binary64 enclosure for the same case (a second, separately written
derivation); and every control below holds. 895 exact checks are recorded in
`references.json` (`crosscheck`).

Every LS-1 out-of-plane component (UY, RX and RZ) and every C-1 non-axial
component is exactly zero by
decoupling: the loads lie in the x–z plane (LS-1) or along the run (C-1),
the members are axis-aligned and collinear, and the joint's midpoint relative
translation along the run is u_Cx − u_Bx.

## Inputs and reference object

| Quantity | Value |
|---|---|
| E, ν, G | 5·2^35 Pa; 1/4; E/(2(1+ν)) = 2^36 Pa (W-16's E and G; ν reproduces G exactly) |
| Section | OD 0.206 m, wall 0.011 m, corrosion allowance 0. A, I, J are W-16's declared binary64 leaves. κ_E (DEL-05-03 §3.1) is exact in the decimal radii and ν: 363779329/723755144 |
| C-1 bore | A_i = fl(π/4·0.184²) = 0.026590440219984007 m² (a binary64 leaf) |
| C-1 joint | k_j = 2^20 N/m axial (other midpoint coefficients 2^22 N/m, 2^12 N·m); A_e = 5/128 m²; untied, zero offsets, stress free |
| C-1 tie rods (control) | two bilateral rods at ±0.15 m in y, 2^24 N/m each, zero reference extension |
| C-1 geometry | A(0) – pipe L – B(2) – joint (replaces pipe J) – C(2.5) – pipe R – D(4.5) m along +X; anchors at A and D; one pressure region over L, R and the joint with `transfers_to_wall` terminals at A and D |
| C-1 pressure | +1 MPa (`load:PRESSURE`) and −1 MPa (`load:VACUUM`) |

**Reference object.** The exact rational response of the system formed from
the binary64 leaves above. The exact route forms its section constants by
D-5's sequence, which can differ from these leaves in the last place
(DEL-04-07); that changes no state and moves no value by
more than about 10⁻¹⁵ relative, far inside the criterion. Values are emitted as
exact fractions with their correctly rounded binary64.

**Criterion.** The consumer requires 1e-9·|expected| for a nonzero value and
1e-9 times the case's family zero scale for an exact zero. This is no looser
than 1e-9·max(|expected|, family zero scale). The family scale is the largest
expected magnitude in that family (translations m, rotations rad, forces N,
moments N·m); no tolerance was derived from an observed error.

## Cases

**C-1** (`requests/c1*.json`, model 0.9.0, written by the generator):

| Request | Guide at C (UX) | Cases and states |
|---|---|---|
| `c1` | one-way, contact for a positive reaction | PRESSURE: the joint thrust moves C toward D, the guide is **inactive**; VACUUM: C moves toward B, the guide is **active** |
| `c1_linear` | none | the no-nonlinear control; PRESSURE equals `c1`'s exactly, VACUUM differs |
| `c1_tied` | as `c1` | tie rods stiffen the joint (k_j + 2k_r); same states, other values; rod forces k_r·(u_C − u_B) |
| `c1_timoshenko` | as `c1` | Timoshenko contract; the axial response equals `c1`'s exactly (a confirming Timoshenko connector case) |
| `c1_gap` | gap closing in +, c = 2^-15 m | PRESSURE closes it, VACUUM leaves it open; R1/R-3: the product keeps such a case unqualified, the values are inspection evidence |
| `c1_ls` | as `c1`, on a load/reference-state document | anchor D moved by −2^-13 m (`boundary_motion`): the guide is active in both cases; dropping the motion reopens it under PRESSURE |

Ledger mutants (each joint term exactly once): dropping the face caps
(the bore thrust counted twice), doubling them, or dropping the effective-area
summand each moves u_C by more than 10⁶ criteria.

**LS-1** (`requests/ls1_*.json`): W-16's `a1` (R-a, a moved anchor: UZ rise
and RY rotation), `d3` (R-b, a rotational lift-off's moved ground, with R-d,
its record's rigid UZ moving) and `c5` (R-b with R-c, a lift-off's moved ground
and a separate spring's moved base), each case except the weak boundaries, as
twins: the same nodes, members, supports, loads and units; the v3 contract;
E/ν; explicit zero corrosion allowance; and per case an `analysis_state` whose
load sources are the non-motion primitives at factor 1 and whose support
states carry each motion primitive as the law of the DOF it names
(rigid or nonlinear DOF: `boundary_motion`; spring axis: `base_motion`), with
meaning `absolute_reference_displacement`. `d3` and `c5` also have Timoshenko
twins, whose values differ from the Euler–Bernoulli ones by more than 10³
criteria.

**Dropped-motion mutants.** For every motion entry of every LS-1 case (and
C-1's moved anchor), the generator solves the case with that entry removed.
Each strict mutant is separated from the reference by more than 10⁶ criteria.
Three mutants fall on an exact weak boundary of the W-16 designs and are
recorded with `weak_boundary: true` and not run by the consumer: `d3`
LIFTED without the ground (A-1's boundary; the reference's separation g is
nonzero) and `c5` ACTIVE and UNEQUAL without the spring base (R = 0 exactly;
the reference's reaction differs by more than 10⁹ criteria).

## Files

| File | Role |
|---|---|
| `generators/rt_a3_s6_reference.py` | the generator; `python3 -I generators/rt_a3_s6_reference.py --pressure-elongation all` rebuilds every output in memory and compares it byte for byte; `--write --pressure-elongation all` rewrites them. The switch is there because the committed requests name every pressurised load case in `model_settings.pressure_elongation` (DEL-04-07); without it the generator builds them without that member and reports them stale. |
| `requests/*.json` | the product request documents (model 0.9.0) |
| `references.json` | per request and case: the strict state, displacements, support actions, nonlinear relative displacements, C-1 joint/wall/rod forces, family zero scales and the dropped-motion mutants; the inputs; the cross-checks |
| `SHA256SUMS` | every file above (`shasum -a 256 -c SHA256SUMS` here) |
