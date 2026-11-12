# Friction support whose normal is a one-way support's reaction

Invented closed-form references for a friction shoe that can lift off. A
friction support takes its normal from the reaction of a one-way (resting)
support at the same node. The friction state therefore depends on a reaction
that the friction force itself changes. When the shoe lifts, the normal is 0.

Governing text:
- `execution/PKG-04/DEL-04-04/ScopeOfWork.md`: the friction law, the
  state-switched test for one-way support, the state names, and REQ-013.
- `execution/PKG-04/DEL-04-07/ScopeOfWork.md`: the nonlinear iteration, where
  a sliding support's bounded force is applied from the previous iterate's
  normal reaction; CLM-002; AC-002.

**Status: frozen analytical reference.** No product comparison has been made.
Self-consistency establishes no product conformance, acceptance or release.

## Independence

`generators/friction_one_way_normal_reference.py` uses only the Python
standard library. It reads, imports and runs no product code, product output,
fixture or schema. All inputs are invented. Every value is exact rational
arithmetic, emitted as the nearest binary64.

**No pressure.** The models carry no pressure. The friction law does not
depend on it: the law relates T, N and the slip direction, whatever the loads.
On either route the document has no pressure region, and the values are the
same on both routes.

## Model

The model is planar in X–Z, with Z up. Every out-of-plane displacement (UY, RX,
RZ), reaction (FY, MX, MZ) and end action is exactly 0, and none is listed.

| Item | Value |
|---|---|
| Nodes | A(0,0,0), B(0,0,3), S(2,0,3) m |
| Members | `riser` A→B (h = 3 m), `run` B→S (a = 2 m) |
| Anchor | A, all six DOF |
| One-way support | at S on UZ, sense + (bears upward only). Ground at u_z = 0, with no imposed movement. |
| Friction support | at S on UX, μ = 3/8. Its normal reaction source is the one-way support at S. |
| Section | Annulus, D = 1/4 m, t = 1/64 m, zero corrosion allowance and mill tolerance. A = 15π/4096 m², I = 1695π/67108864 m⁴. |
| Section basis | The nominal and reduced bases coincide (zero allowance and tolerance), so the stiffness is the same on either. |
| Material | E = 2·10¹¹ Pa. ν = 1/4 is not exercised, because nothing twists. |
| Shear model | `euler_bernoulli` (no shear deformation). A product document must select it explicitly. |
| Loads | Nodal forces only: Gx at B, and Px, Pz at S. No self-weight. |

Every input is exactly representable in binary64, including each load.

EI = π·82763671875/16384 N·m² and EA = π·732421875 N.

**Conventions.**
- A reaction is the support-on-pipe force (K u − f) at the support's DOF, in
  global axes.
- N is the one-way reaction FZ while that support is active, and 0 while it is
  inactive.
- T is the friction reaction FX.
- An end action is the force or moment the node exerts on the member at that
  end, in global axes.
- MY is about +Y. A positive rotation turns Z toward X.
- Displacements are exact rationals divided by π. Reactions and end actions are
  rational, because π cancels between the two flexibilities.

**States (DEL-04-04).**
- One-way support: `active` or `inactive`.
- Friction support: `inactive` (the normal does not bear), `stick`, `slide+` or
  `slide-`. The sign is the sign of the slip u_x(S).

## State law, as checked

Each deciding quantity must lie strictly on its state's side.

| State | Condition |
|---|---|
| one-way active | N = R_z > 0 and u_z(S) = 0 |
| one-way inactive | R_z = 0 and separation u_z(S) > 0 |
| friction inactive | The normal does not bear, so the one-way support is inactive. T = 0. |
| friction stick | The normal bears; u_x(S) = 0; \|T\| < μN |
| friction slide d (d = ±1) | The normal bears; T = −d·μ·N, with N this state's own one-way reaction; d·u_x(S) > 0 |

## Closed form (formulation A), for re-derivation by hand

Root the frame at A, so that it is a cantilever. Let (Fx, Fz) be the total
force at S, meaning applied loads plus support forces, and Gx the force at B.
Let X = Fx + Gx. Unit-load integrals over the two members (M²/2EI + N²/2EA)
give:

```
u_x(S) = (X h³/3 − a Fz h²/2)/EI + Fx a/EA
u_z(S) = (−a h² X/2 + a² h Fz + a³ Fz/3)/EI + Fz h/EA
θ_y(S) = (X h²/2 − a Fz h − Fz a²/2)/EI
u_x(B) = (X h³/3 − a Fz h²/2)/EI
u_z(B) = Fz h/EA
θ_y(B) = (X h²/2 − a Fz h)/EI
```

The flexibility at S is:

```
f_xx = h³/(3EI) + a/EA          = 147682/(82763671875 π)  m/N
f_xz = −a h²/(2EI)              = −49152/(27587890625 π)  m/N
f_zz = (a³/3 + a² h)/EI + h/EA  = 721913/(248291015625 π) m/N
```

f_xz < 0, so a +X force at S or B presses S down and raises N, and a −X force
lifts S. The friction force therefore changes its own normal.

Let (u0x, u0z) be the displacement at S under the applied loads with no
support at S. That is the formula above with Fx = Px and Fz = Pz. The support
forces at S are (R_x, R_z):

| State pair | Support forces |
|---|---|
| active / stick | [f_xx f_xz; f_xz f_zz]·(R_x, R_z) = −(u0x, u0z) |
| active / slide d | R_z = −u0z / (f_zz − d·μ·f_xz), R_x = −d·μ·R_z |
| active / inactive | R_x = 0, R_z = −u0z / f_zz. This always violates the friction law when R_z > 0. |
| inactive / inactive | R_x = R_z = 0, u = u0 |
| inactive / stick | R_z = 0, R_x = −u0x/f_xx. This always violates the law, because the normal does not bear. |
| inactive / slide d | R_x = R_z = 0. This always violates the law, because the normal does not bear. |

Every other quantity follows by statics:
- anchor: FX = −X, FZ = −Fz, MY = −(h X − a Fz);
- riser end actions: at A (−X, −Fz, aFz − hX), at B (X, Fz, −aFz);
- run end actions: at B (−Fx, −Fz, aFz), at S (Fx, Fz, 0);
- axial force, tension positive: riser Fz, run Fx.

All of these use the totals Fx = Px + R_x and Fz = Pz + R_z.

**Worked example, SLIDE-POS-B.** Gx = 8000 N and Pz = −10000 N.
1. With EI and EA as above, u0z = (−a h²·8000/2 + (a²h + a³/3)(−10000))/EI + 3(−10000)/EA.
2. For slide+, R_z = −u0z/(f_zz − μ f_xz) = 10758074000/887801 N (12117.66 N).
3. T = −(3/8)·R_z = −4034277750/887801 N.
4. u_x(S) = u0x + f_xx R_x + f_xz R_z = 464735333836/(195940455078125 π) m, which
   is greater than 0, so the slide is +X, as assumed.
5. Stick fails here: the stick solve gives |R_x| greater than μR_z.

## Cases

The case ids are those in `references.json`. In every case Px = 0 except
SLIDE-POS-S.

| Case | Loads (N) | States (one-way / friction) | N (N) | T (N) | Deciding margin |
|---|---|---|---|---|---|
| STICK | Gx 2000; Pz −10000 | active / stick | 207018674458000/20691869929 (10004.83) | −41220587520000/20691869929 (−1992.12) | μN − \|T\| = 1759.70 N |
| SLIDE-POS-B | Gx 8000; Pz −10000 | active / slide+ | 10758074000/887801 (12117.66) | −μN | u_x(S) = 0.000755 m |
| SLIDE-NEG-B | Gx −8000; Pz −10000 | active / slide- | 147207440/22241 (6618.74) | +μN | u_x(S) = −0.00121 m |
| SLIDE-POS-S | Px 6000; Pz −10000 | active / slide+ | 9873338000/887801 (11121.12) | −μN | u_x(S) = 0.000403 m |
| SLIDE-UNLOAD | Gx −3000; Pz −10000 and +6000 (two loads) | active / slide- | 62421920/22241 (2806.61) | +μN | u_x(S) = −0.000427 m |
| LIFT-0 | Pz +2000 | inactive / inactive | 0 | 0 | separation 0.00185 m |
| LIFT-POS | Gx +3000; Pz +2000 | inactive / inactive | 0 | 0 | separation 0.000150 m |
| LIFT-NEG | Gx −20000; Pz +2000 | inactive / inactive | 0 | 0 | separation 0.0132 m |
| NEAR-STICK | Gx 964677/256; Pz −10000 | active / stick | 10009.103… | −3753.413… | μN − \|T\| = 3.74e-4 N |
| NEAR-SLIDE | Gx 964678/256; Pz −10000 | active / slide+ | 8886093584/887801 | −μN | u_x(S) = 6.30e-10 m |
| NEAR-BEAR | Gx −8000; Pz −1254957/256 | active / slide- | 108741/142342400 (7.64e-4) | +μN | contact 7.64e-4 N |
| NEAR-LIFT | Gx −8000; Pz −1254956/256 | inactive / inactive | 0 | 0 | separation 3.07e-9 m |

The exact value of every displacement, rotation, reaction (anchor, one-way,
friction), end action and axial force is in `references.json`.

**Sliding and the source of N.** In each sliding case, N is the one-way
reaction of the converged sliding state itself. It differs from the weight,
and from the normal of any other state:
- In SLIDE-POS-B, the +X load at B presses S down, so N is about 12118 N
  against a 10000 N weight.
- In SLIDE-NEG-B, the −X load lifts S, so N is about 6619 N.
- In SLIDE-UNLOAD, the uplift and the −X load both unload the shoe, so N is
  about 2807 N against a 10000 N weight.

**Lifted.** When the one-way support is off its seat, N = 0 and the friction
support carries no force and restrains nothing.
- In LIFT-POS, the +X lateral load of 3000 N pushes S down, but not onto its
  seat.
- In LIFT-NEG, the −X load of 20000 N lifts S further.
- Neither lateral load meets any friction resistance, and u_x(S) is the free
  cantilever's.

## Near-threshold pairs

- **Stick to slide, varying Gx.** Pz = −10000 N throughout. The exact threshold
  is Gx* = 12932418705625/3431924736 N (3768.2699 N), where μN = |T| in the
  stick state. NEAR-STICK and NEAR-SLIDE are the two multiples of 1/256 N on
  either side of it.
- **Lift-off, varying Pz.** Gx = −8000 N throughout. The exact threshold is
  Pz* = −3538944000/721913 N (−4902.1752 N), where the free u_z(S) = 0. At that
  point the sliding-state normal and the lifted separation both reach 0.
  NEAR-BEAR and NEAR-LIFT are the two multiples of 1/256 N on either side of
  it. On the bearing side the normal is small and the lateral demand large,
  so the shoe slides.

**Robustness of the discrete outcome.** For every case, not only the pairs, the
generator checks that the state does not change when either of the following
is done:
- each input (E, ν, D, t, h, a, μ and every load) is replaced by its binary64
  value. All the inputs are dyadic, so this is an identity check that the
  emitted inputs are the inputs.
- EA and EI are each perturbed independently by ±1e-9 relative, in all four
  combinations. This far exceeds the rounding of a binary64 section
  computation.

The state decisions do not depend on π: every threshold is π-free. The
generator also checks that reactions are unchanged, and displacements scale as
1/π, when the stiffness is scaled by 3 and by 4.

## Normal from the previous iterate: the fixed point

DEL-04-07 applies a sliding force from the previous iterate's normal. While a
sliding state is held, that update is N_k = N_0 + ρ·N_(k−1), where:
- ρ = d·μ·f_xz/f_zz = ∓165888/721913 (about ∓0.2298), so |ρ| < 1;
- N_0 is the one-way reaction with the friction force set to zero.

The iteration therefore converges to the fixed point N* = N_0/(1 − ρ). That
fixed point is the reference N above: the "current iterate" normal of
DEL-04-04's law. For each sliding case, `references.json` gives:
- ρ;
- N_0;
- N_1 to N_5;
- N*.

The generator checks N_k − N* = ρ^k·(N_0 − N*) exactly.

## Negative controls (wrong answers, labelled)

Each control holds N fixed at a wrong value and applies the friction law with
it. The control sticks if the stick demand is below μN_wrong. Otherwise it
slides with force −d·μ·N_wrong, and the one-way support stays seated. Each
control's state, friction force, one-way reaction, u_x(S) and u_x(B) are
published, with their relative differences from the reference.

| Control | Wrong normal | Effect |
|---|---|---|
| CTRL-FREE-NORMAL (all sliding cases) | N_0, from a solve with no friction force, not updated | The friction force differs by exactly ±ρ relative (±23 %). In NEAR-SLIDE the control sticks instead. |
| CTRL-STICK-NORMAL (SLIDE-*) | The stick iterate's normal, used once with no further update | The force differs by −17 % to +51 % relative. In SLIDE-POS-S it equals the applied 10000 N. |
| CTRL-WEIGHT-NORMAL (SLIDE-UNLOAD) | The weight-only reaction, 10000 N, carried over from another load and ignoring the uplift | The state becomes **stick** instead of slide-, with T = +2988.17 N against the reference +1052.48 N. |

A product whose normal comes from a stale or wrong source matches a control,
not the reference. The near-threshold controls differ from the reference by
very little. They show only the direction of the error.

## Cross-checks (all exact; 377 checks, all pass)

- **A against B.** The direct stiffness formulation (B) uses planar
  Euler–Bernoulli 6×6 elements rotated to global axes. The support states are
  extra equations, and the system is solved by exact Gauss–Jordan elimination.
  B equals A exactly in every displacement, rotation, reaction and end action
  of every case.
- **Uniqueness.** In both A and B, all eight state pairs are solved and the
  state law is tested on each. Exactly one pair satisfies it in every case,
  and it is the same pair in A and in B. `references.json` lists each pair's
  violated conditions.
- **Active-set iteration.** B classifies with the state-switched test: an
  active support is judged on its reaction, an inactive one on penetration,
  stick on |T| < μN, and slide on the direction of the slip. It is iterated
  from (active, stick) and from (inactive, inactive), and both starts settle
  on the unique state. The paths are published as illustration. They are not
  an expected product path.
- **Flexibility.** The closed-form f_xx, f_xz and f_zz equal B's response to
  unit loads.
- **Equilibrium.** The checks cover global balance, member balance and the
  node-B balance.
- **Slide relations.** T = −d·μ·N, and the lagged-normal identities above.
- **Other checks.** The robustness checks above, the bracket checks, and that
  each control differs from the reference.

## Tolerance class

DEL-04-04/AC-001 and DEL-04-07/AC-002 apply: support states must equal these
references exactly. Displacements and reactions must agree within the
nonlinear tolerance of their class, in the governed per-class record that
DEL-04-07 refers to (DEL-09-03/REQ-007). This package sets no number. The 2e-2
"Nonlinear" class in `docs/RELEASE_TARGET_v1.0.md` is the cross-engine
screening profile, not this verification.

**Expected standing** (DEL-04-04/REQ-013) is not asserted here. The deciding
quantities and their margins are published. The smallest margins are in the
near-threshold cases:
- stick margin 3.74e-4 N;
- slip 6.30e-10 m;
- contact 7.64e-4 N;
- separation 3.07e-9 m.

Whether the confirmation establishes each of these, given its numerical error,
is for the product to determine.

## Open readings

1. **Which normal "converged" means.**
   - DEL-04-04's law takes N from the current iterate.
   - DEL-04-07 applies the force from the previous iterate's normal.

   *Reading (a):* the iteration has converged only when N has stopped changing,
   that is, at the fixed point N*. The scope's convergence needs both no state
   change and a residual within tolerance.
   *Reading (b):* it has converged when the states stop changing, which can
   leave the force formed from a lagged normal. CTRL-STICK-NORMAL shows what
   that gives after a single slide iterate.

   The values here are computed under (a), which the text best supports.
2. **Standing of a friction support whose source has lifted.** DEL-04-04's
   confirmation requires "a friction normal derived from a reaction, which must
   be established as nonzero".

   *Reading (a):* friction `inactive` is decided by its source's state, and the
   separation is established as greater than 0, so the state can be confirmed.
   *Reading (b):* a derived normal that is exactly 0 cannot be established as
   nonzero, so the LIFT and NEAR-LIFT cases are unconfirmed with a named
   reason.

   Their values are the same under both readings. Only the standing differs,
   and it is not asserted here.

## Reproduce

```sh
python3 -I -B generators/friction_one_way_normal_reference.py          # check frozen bytes
python3 -I -B generators/friction_one_way_normal_reference.py --write  # rewrite on purpose
shasum -a 256 -c SHA256SUMS
```

`SHA256SUMS` covers this README, the generator and `references.json`. Resolve
any disagreement against the mechanics above. Never resolve it by matching a
product output.
