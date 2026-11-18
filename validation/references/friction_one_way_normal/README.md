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

## Classification and strict confirmation

The package keeps two questions apart, and each case record answers both. The
answers are in `classification` and `strict confirmation`, and per state pair
under `enumeration`.

**Classification** asks which state the classifier returns for a trial. A
state pair is consistent when the classifier returns the trial pair itself.
The rules are DEL-04-04's law as written:

| Trial | The classifier returns |
|---|---|
| one-way active | Active while it bears (R_z > 0), otherwise inactive. Whether R_z = 0 "bears" is not settled by the wording; see open reading 1. |
| one-way inactive | Active when it would penetrate (u_z(S) < 0), otherwise inactive. u_z(S) = 0 is touch, not penetration. |
| friction, normal does not bear | Inactive |
| friction stick (or inactive) trial, normal bears | Stick while \|T\| ≤ μN, **non-strict as written**. Otherwise slide in the direction −sign(T). |
| friction slide d trial, normal bears | The trial carries μN opposing its motion. It stays slide d while d·u_x(S) > 0, and otherwise returns stick. |

A slide trial has \|T\| = μN by construction, so the law's comparison can
only be applied to it through its motion. The slide-trial row follows from "a
sliding support carries a tangential force of magnitude μ·N opposing its
motion".

**Strict confirmation** asks whether a pair is established. This is DEL-04-04's
admissibility confirmation (REQ-013). It also follows DEL-04-01 REQ-016, as
quoted in the repair brief: "An enclosure that touches its state's boundary
does not establish that state." This package checks the exact values, so the
test is that every deciding quantity lies strictly on its state's side:

| State | Strict condition |
|---|---|
| one-way active | N = R_z > 0 and u_z(S) = 0 |
| one-way inactive | R_z = 0 and separation u_z(S) > 0 |
| friction inactive | The normal's source is established as released, which is the one-way inactive row above. T = 0. |
| friction stick | The normal bears; u_x(S) = 0; \|T\| < μN |
| friction slide d (d = ±1) | The normal bears; T = −d·μ·N, with N this state's own one-way reaction; d·u_x(S) > 0 |

The friction rows follow the decided text of DEL-04-04's state-law parenthesis
(see "Decided reading" below). For a friction support that sticks or slides,
the normal derived from the reaction must be established as nonzero. A
friction support whose normal's source is established as released is
inactive. The order matters: the source is established first, released by
strict separation or engaged by a strictly positive reaction, and only then is
the friction support read.

**Two confirmation predicates.** Formulation A decides confirmation with
`confirmation_failures`, which lists every condition a pair fails.
Formulation B decides it with `b_establishes`, which is written separately and
follows the establishment order above. The two must give the same set of
confirmed pairs in every case, both under the decided text and under the
literal alternative. Both were written by the same author from the same text,
so their agreement catches a coding slip but not a misreading of the text. The
reading itself rests on the quoted decision.

Away from a boundary the two answers coincide. In every case except the two
boundary witnesses, exactly one pair is consistent with the classification law
under either reading of a zero reaction, and strict confirmation establishes
that same pair and no other. At the boundary witnesses they differ, as stated
below.

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
| active / inactive | R_x = 0, R_z = −u0z / f_zz. When R_z > 0 this is never consistent, because the normal bears. |
| inactive / inactive | R_x = R_z = 0, u = u0 |
| inactive / stick | R_z = 0, R_x = −u0x/f_xx. This is never consistent, because the normal does not bear. |
| inactive / slide d | R_x = R_z = 0. This is never consistent, because the normal does not bear. |

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
| STICK-BOUNDARY | Px 3000; Pz −8000 | active / stick (classification; not confirmed) | 8000 | −3000 | μN − \|T\| = 0 exactly |
| LIFT-BOUNDARY | Gx −721913/128; Pz −3456 | not settled (open reading 1) | 0 | 0 | contact 0 and separation 0 exactly |

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

## Boundary witnesses

Both witnesses use inputs that are exact in binary64.

**STICK-BOUNDARY** (Px = 3000 N, Pz = −8000 N, Gx = 0). Every load acts at S.
The stick trial holds S in both X and Z, so F_SS·R = −F_SS·P and R = −P:
- T = −3000 N and N = 8000 N;
- every displacement and rotation is 0, and the anchor reaction is 0;
- μN = 3000 N = |T|, so the stick margin is exactly 0.

The slide trials differ:
- **slide+** (T = −μN, u_z = 0). The stick solution R = −P satisfies
  T = −μN, so it is this trial's solution too. u_x(S) = 0, so the trial does
  not move in its own direction.
- **slide−** (T = +μN). Solving T − μN = 0 with u0z + f_xz·T + f_zz·N = 0
  gives N = 284096320/22241 N, T = 106536120/22241 N, and
  u_x(S) = 662139837728/(122716455078125 π) m > 0. The motion opposes the
  assumed negative slide direction, so the classifier returns stick and the
  pair is neither consistent nor confirmed.
- **inactive trials.** The free u_z(S) < 0 would penetrate.
- **Classification** under DEL-04-04's law as written: **stick**, because
  |T| ≤ μN holds with equality. (active, stick) is the only consistent pair,
  under either reading of a zero reaction.
- **Strict confirmation** does not establish stick, because its margin is
  exactly 0 (DEL-04-01 REQ-016). It establishes no other pair either.

**LIFT-BOUNDARY** (Gx = −721913/128 N, Pz = −3456 N, Px = 0). The free
vertical displacement at S is:

u0z·EI = −(a h²/2)·Gx + (a²h + a³/3 + h·EI/EA)·Pz = −9·Gx + (721913/49152)·Pz

using EI/EA = 113/16384. That is 0 for Gx = (721913/442368)·Pz = −721913/128 N.
The free u_x(S) = 9(Gx − Pz)/EI = −21469056/(5517578125 π) m. So:
- u_z(S) = 0, N = T = 0, and every reaction at S is 0;
- the anchor carries FX = 721913/128 N, FZ = 3456 N and MY = 1281003/128 N·m;
- the active/stick compatibility solve gives T = 116240929680960/20691869929 N
  and N = 71229175234560/20691869929 N. Then |T| > μN, so that pair violates
  the friction bound and is not consistent;
- the active/slide− trial solves to R_z = 0;
- the inactive/inactive trial has u_z(S) = 0;
- the classification-consistent pairs under the two readings,
  (inactive, inactive) and (active, slide−), give identical physical values.

What DEL-04-04's wording gives:
- On an **inactive trial**, u_z(S) = 0 is touch, not penetration, so the
  released support stays released. (inactive, inactive) is consistent.
- On an **active trial** with a reaction of exactly 0, "it stays engaged while
  it bears" does not say whether a zero reaction bears.
  - If it does not bear, the support releases, and (inactive, inactive) is the
    only consistent pair.
  - If it does bear, (active, slide−) is also consistent: with a zero normal,
    the slip is −X in its own direction.

The wording does not settle this tie. It is open reading 1.
- **Classification:** not settled by the scope. Both readings are published.
- **Strict confirmation** establishes no pair. The active trial's reaction and
  the inactive trial's separation are each exactly 0, so each touches its
  boundary (DEL-04-01 REQ-016).

The physical values of both witnesses are unique and published like any other
case. Only the state labels and the standing are at issue.

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

**Robustness of the discrete outcome.** The generator checks that the
classification and confirmation outcome does not change when either of the
following is done:
- each input (E, ν, D, t, h, a, μ and every load) is replaced by its binary64
  value. This is checked for every case. All the inputs are dyadic, so it is
  an identity check that the emitted inputs are the inputs.
- EA and EI are each perturbed independently by ±1e-9 relative, in all four
  combinations. This far exceeds the rounding of a binary64 section
  computation. It is checked for every case except the two boundary
  witnesses, which sit on their boundary by design.

The state decisions do not depend on π: every threshold is π-free. The
generator also checks that reactions are unchanged, and displacements scale as
1/π, when the stiffness is scaled by 3 and by 4.

## Normal from the previous iterate: the fixed point

DEL-04-07's text describes a sliding force applied from the previous iterate's
normal. This section illustrates that scheme and commits a product to nothing
about its iteration: a scheme that solves the sliding force and its normal
together reaches N* without passing through this sequence. What the package
commits to is the fixed point N*. While a sliding state is held, the
previous-iterate update is N_k = N_0 + ρ·N_(k−1), where:
- ρ = d·μ·f_xz/f_zz = ∓165888/721913 (about ∓0.2298), so |ρ| < 1;
- N_0 is the one-way reaction with the friction force set to zero.

The fixed point N* = N_0/(1 − ρ) is the exact reference target, whichever
scheme reaches it. It is the reference N above, where the force uses the
normal of the current state, as in DEL-04-04's law.

Finite numerical convergence is what the scopes define: unchanged states with
a residual within the governed tolerance of the support class (DEL-04-04
"Convergence", DEL-04-07 CLM-002). Neither of these is the criterion:
- state stability alone;
- exact equality of successive normals, which a previous-iterate scheme
  reaches only in the limit.

A product's converged normal is compared with N* at the class tolerance, like
every other value. For each sliding case, `references.json` gives:
- ρ;
- N_0;
- N_1 to N_5, the previous-iterate sequence, as illustration and not as an
  expected product path;
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

## Cross-checks (all exact; 536 checks, all pass)

- **A against B.** The direct stiffness formulation (B) uses planar
  Euler–Bernoulli 6×6 elements rotated to global axes. The support states are
  extra equations, and the system is solved by exact Gauss–Jordan elimination.
  B equals A exactly in every displacement, rotation, reaction and end action
  of every case.
- **Classification and confirmation.** In both A and B, all eight state pairs
  are solved. On each pair the classification law is tested under both
  readings of a zero reaction, and the strict confirmation predicate is tested
  too. Except at the boundary witnesses, exactly one pair is consistent and
  the same pair alone is strictly confirmed. At the witnesses the outcomes are
  the stated ones. A and B give the same sets. `references.json` lists, for
  each pair, whether it is classification-consistent, whether it is strictly
  confirmed, and the confirmation conditions it fails.
- **Active-set iteration.** B iterates the classifier above, with the
  non-strict stick test, from (active, stick) and from (inactive, inactive),
  under both readings of a zero reaction. Each run settles on a
  classification-consistent pair. Under the first reading, every case settles
  on the reference pair. At LIFT-BOUNDARY, the second reading settles on
  (active, slide−) from (active, stick). The paths are published as
  illustration, not as an expected product path.
- **Tied pairs.** At LIFT-BOUNDARY, the classification-consistent pairs under
  the two readings give identical physical values.
- **Confirmation by two predicates.** A's `confirmation_failures` and B's
  separately written `b_establishes` give the same confirmed set in every
  case, under both the decided text and the literal alternative. The literal
  alternative differs from the decided text only in the released/inactive
  pair of the four lifted cases.
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

**What is asserted about standing** (DEL-04-04/REQ-013). For every case, the
package asserts the exact confirmation outcome: whether the exact deciding
quantities establish the classified pair, with each strictly on its state's
side.
- **Ground:** the exact values above, the decided text of the state-law
  parenthesis (quoted under "Decided reading"), and DEL-04-01 REQ-016: "An
  enclosure that touches its state's boundary does not establish that state."
- **The outcome:**
  - every case except the two boundary witnesses is established;
  - the two witnesses establish no pair;
  - the four lifted cases are established under the decided text, and
    unconfirmed under the named literal alternative.
- **Reading a product's standing against it.** A product's standing takes its
  own numerical error into account.
  - Presenting a pair as confirmed where this package establishes none is
    wrong. That includes the boundary witnesses and, under the literal
    alternative, the lifted cases.
  - Where the product's confirmation runs and encloses the deciding
    quantities, presenting an established pair as unconfirmed agrees with the
    scope only where the product's error bound reaches the margin. Where
    confirmation does not run or cannot enclose them, the scopes require
    unconfirmed standing with their named reason (DEL-04-04 REQ-013,
    DEL-04-01 REQ-016), and that agrees with this package.

  Away from the witnesses, the smallest margins are in the near-threshold
  cases:
  - stick margin 3.74e-4 N;
  - slip 6.30e-10 m;
  - contact 7.64e-4 N;
  - separation 3.07e-9 m.

## Open readings

1. **A zero reaction on an active trial (the lift-off tie).** DEL-04-04 says
   an engaged support "stays engaged while it bears". The wording does not say
   whether a reaction of exactly 0 bears. On a released trial, touch
   (u_z = 0) is not penetration, so the support stays released.

   *Reading (a):* a zero reaction does not bear. At LIFT-BOUNDARY the
   classifier then returns (inactive, inactive) only.
   *Reading (b):* a zero reaction bears. Then (active, slide−) is also
   consistent, and the classification is not unique.

   The physical values are the same under both readings, and strict
   confirmation establishes neither pair. The scope leaves this tie open, and
   this package does not choose. The convergence criterion is not affected.
   The exact analytical reference target is N*. Finite convergence permits a
   residual within the governed tolerance (see "Normal from the previous
   iterate").

## Decided reading

**Standing of a friction support whose source has lifted.** Today's DEL-04-04
text says the confirmation's deciding quantities include "a friction normal
derived from a reaction, which must be established as nonzero". The SWBPIPE
Agent 0 has decided that the state law's parenthesis will read:

> "(contact reaction, separation, stick margin μ·N − |T|, slip direction, and,
> for a friction support that sticks or slides on a normal derived from a
> reaction, that normal established as nonzero; a friction support whose
> normal's source is established as released is inactive)"

**Status:** decided, and not yet in `execution/PKG-04/DEL-04-04/ScopeOfWork.md`.
It lands there with the engine unit this package judges.

The nonzero requirement was written for a normal that can only vanish at a
tie. With a one-way source, the normal is zero because of the source's
established state. Establishment order is part of the decision: the source's
release is established first, by strict separation, and only then is the
friction support read as inactive.

**Primary flags.** In LIFT-0, LIFT-POS, LIFT-NEG and NEAR-LIFT, the flags rest
on the decided text: `strict confirmation` → `establishes: true` with the
established pair (inactive, inactive), and "strictly confirmed: true" on that
pair in `enumeration`. Each of these cases also publishes:
- the decision and its status, under `rests on`;
- the establishment order:
  1. the one-way support is released, established by a strictly positive
     separation, with the separation and its sign;
  2. the friction support is then inactive, with N = T = 0.

**Named alternative (not primary).** Under the literal reading of today's text,
the derived normal is zero and cannot be established as nonzero, so these four
cases are unconfirmed. Each publishes that alternative beside the primary
flags:
- in `strict confirmation`, under
  `alternative (named; not primary): literal reading of today's text`;
- on the affected pair in `enumeration`, under the members prefixed
  "alternative:".

The physical values are the same under both. No other case's flags differ
between the two: in every sticking and sliding case the normal is nonzero, and
the two boundary witnesses are unconfirmed under both.

## Reproduce

```sh
python3 -I -B generators/friction_one_way_normal_reference.py          # check frozen bytes
python3 -I -B generators/friction_one_way_normal_reference.py --write  # rewrite on purpose
shasum -a 256 -c SHA256SUMS
```

`SHA256SUMS` covers this README, the generator and `references.json`. Resolve
any disagreement against the mechanics above. Never resolve it by matching a
product output.
