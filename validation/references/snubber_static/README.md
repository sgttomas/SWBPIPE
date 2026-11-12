# Snubbers in static load cases

Invented closed-form references for one statement. A snubber is a restraint in
dynamic analysis only, so every static load-case solve leaves it out and
discloses it. Every static result of a model with snubbers therefore equals
the same model's result without them.

Governing text:
- `execution/PKG-04/DEL-04-10/ScopeOfWork.md`: REQ-005, AC-003, REQ-007.
- `execution/PKG-04/DEL-04-07/ScopeOfWork.md`: CLM-001, "every static
  load-case solve leaves it out and discloses it".
- `execution/PKG-04/DEL-04-03/ScopeOfWork.md` REQ-010, for what a skew record
  means in the negative controls.

These references do not cover dynamics. Snubber modal and spectrum references
belong elsewhere.

**Status: frozen analytical reference.** No product comparison has been made.

## Independence

`generators/snubber_static_reference.py` uses only the Python standard
library. It reads, imports and runs no product code, product output, fixture
or schema. All inputs are invented.

Values are exact elements of Q(π), ratios of polynomials in π with rational
coefficients. That form is needed because a snubber stiffness combines with
EI and EA, which carry π. Each value is emitted as the nearest binary64. The
generator decides that rounding from a Machin enclosure of π.

**No pressure.** The models carry no pressure. The snubber's static absence
does not depend on it, because it is a rule about which restraints enter a
static solve. On either route the document has no pressure region, and the
values are the same on both routes.

## Models

| Item | Value |
|---|---|
| Nodes | A(0,0,0), M(2,0,0), N(3,0,0), S(4,0,0) m, on one straight line along X (L = 4 m) |
| Members | A-M, M-N, N-S |
| Anchor | A, all six DOF |
| Linear model (SL) | Rigid two-way restraint on UZ at S (vertical support) |
| Nonlinear model (SN) | One-way support on UZ at S, sense + (bears upward). Ground at u_z = 0. |
| Section | Annulus, D = 1/2 m, t = 1/32 m, zero corrosion allowance and mill tolerance. A = 15π/1024 m², I = 1695π/4194304 m⁴. The nominal and reduced bases coincide. |
| Material | E = 2·10¹¹ Pa |
| Shear model | `euler_bernoulli` |
| Loads | Nodal only: −P or +P on Z at N (P = 20000 N), and Q = +50000 N on X at S |

The model is planar in X–Z, with Z up. Every out-of-plane quantity is exactly
0. Every input is exactly representable in binary64.

**Snubber records** used in the variants:

| Id | Node | Direction d (not normalized) | Stiffness |
|---|---|---|---|
| SNB-1 | S | (1, 0, 0), axis UX | rigid |
| SNB-2 | M | (0, 0, 1), axis UZ | 2·10⁸ N/m |
| SNB-3 | M | (1, 0, 2), skew | rigid |
| SNB-4 | S | (2, 0, 1), skew | 10⁹ N/m |

**Conventions.**
- A reaction is the support-on-pipe force in global axes.
- A skew restraint's force is λ·d along its authored d. Its components are
  published, because they are rational where |d| is not.
- An end action is the force or moment the node exerts on the member, in
  global axes.
- MY is about +Y, with Z turning toward X positive.

## What a product must give

| Case | Model, loads | One-way state | Expected for each of V1, V2, V3, V4, V-ALL |
|---|---|---|---|
| SL-1 | SL; −P at N, +Q at S | (linear) | The base result, unchanged |
| SN-BEAR | SN; −P at N, +Q at S | active | The base result, unchanged, including the state |
| SN-LIFT | SN; +P at N, +Q at S | inactive | The base result, unchanged, including the state |

The variants are:
- V1, V2, V3 and V4: SNB-1, SNB-2, SNB-3 and SNB-4, each alone;
- V-ALL: all four snubbers together.

"Unchanged" covers every displacement and rotation at M, N and S, every
reaction and every member end action. No snubber publishes a static reaction.

**Expected disclosure** is an outcome, not a code name. In each static case,
the product gives one non-blocking finding per snubber, naming it, with the
wording "dynamic-only restraint, not active in static cases". V-ALL therefore
gives four.

## Base results in closed form

Every value follows from the cantilever influence functions of the frame
rooted at A. For a force (Fx, Fz) at xf, the displacements at xp are:

```
u_x = Fx·min(xp, xf)/EA
w   = Fz·xp²(3xf − xp)/(6EI)      for xp ≤ xf
w   = Fz·xf²(3xp − xf)/(6EI)      for xp ≥ xf
θ_y = −dw/dx                      slope Fz·xp(2xf − xp)/(2EI), or Fz·xf²/(2EI) beyond xf
```

Support forces are redundants solved from compatibility at their points.
EI = 2·10¹¹·1695π/4194304 N·m² and EA = 2·10¹¹·15π/1024 N.

**SL-1 and SN-BEAR** have the same values. The structure is a propped
cantilever with the load at a = 3 and b = 1:

| Quantity | Value |
|---|---|
| R_S | P a²(3L − a)/(2L³) = 81P/128 = 50625/4 N (12656.25 N); in SN-BEAR this is > 0, so the support bears |
| Anchor | FX = −50000, FZ = 29375/4 N, MY = −P a b (L + b)/(2L²) = −9375 N·m |
| u_x(S) | Q L/EA = 16/(234375 π) m |
| u_x(M), u_x(N) | Q x/EA |
| u_z(M) | −44032/(397265625 π) m |
| u_z(N) | −4992/(44140625 π) m |
| u_z(S) | 0 |

The rotations are in `references.json`.

**SN-LIFT.** Take the support as active, and the compatibility solve gives
R_S = −81P/128 < 0, which is inadmissible. Take it as inactive, and S is a
free cantilever tip:
- u_z(S) = P a²(3L − a)/(6EI) = 147456/(44140625 π) m, which is > 0, so the
  support is separated;
- R_S = 0;
- anchor: FX = −50000 N, FZ = −20000 N, MY = 60000 N·m.

Exactly one state is admissible in each nonlinear case, and in each control.

## Negative controls (wrong static answers, labelled)

Each control is the static result the model would have if a snubber acted in
the static solve:
- a rigid snubber acts as a rigid restraint d·u = 0;
- a finite snubber acts both as a rigid restraint and as a spring
  k·d dᵀ/(dᵀd);
- in V-ALL, every snubber acts as authored.

Each control is solved with the one-way state re-determined. It differs from
the base by more than 5e-2 relative, which is larger than any tolerance number
stated in the docs. Key support forces:

| Control | SL-1 / SN-BEAR | SN-LIFT |
|---|---|---|
| V1 rigid | SNB-1 FX = −50000 N; u_x = 0 along the line | SNB-1 FX = −50000 N |
| V2 rigid | R_S = 55000/7, SNB-2 FZ = 107500/7 N | SNB-2 FZ = −35000 N |
| V2 spring | SNB-2 FZ = (22544384/1017)/(917504/635625 + π) N (4834.7 N) | SNB-2 FZ ≈ −23711.3 N |
| V3 rigid | R_S = 20403875/2348 N; SNB-3 = (3725200, 7450400)/587 N | SNB-3 = (−9310640, −18621280)/527 N |
| V4 rigid | R_S = 150625/4 N; SNB-4 = (−50000, −25000) N | **the one-way support becomes active**, R_S = 49375/4 N |
| V4 spring | SNB-4 ≈ (−12899.2, −6449.6) N | SNB-4 ≈ (−24390.8, −12195.4) N |
| V-ALL as authored | R_S ≈ 7912.9 N; SNB-1 ≈ −53774 N | SNB-1 ≈ −32719 N, SNB-3 ≈ (−9632, −19264) N |

**A hand check of V2 rigid on SL-1.** Take R_M and R_S as redundants. The
cantilever influence coefficients are f(M,M) = 8/3, f(M,S) = 20/3 and
f(S,S) = 64/3, each over EI. The load −P at N gives w(M) = −14P/(3EI) and
w(S) = −27P/(2EI). The two compatibility equations are:

```
8 R_M + 20 R_S = 14 P
20 R_M + 64 R_S = 40.5 P
```

They give R_S = 5.5P/14 = 55000/7 N and R_M = 107500/7 N.

Every control's full displacements, reactions, end actions and one-way state
are in `references.json`, with the largest relative difference from the base.

## Cross-checks (all exact; 434 checks, all pass)

- **A against B.** Formulation A uses closed-form influence functions with
  redundants. Formulation B is direct stiffness: planar Euler–Bernoulli
  elements, springs as k·d dᵀ/(dᵀd), and rigid axis or skew restraints as
  exact constraint rows d·u = 0 with their multipliers. A and B agree exactly
  on every displacement, reaction and end action of every base result and
  every control.
- **Textbook closed forms.** The propped-cantilever R_S and anchor moment, the
  free-cantilever lift, and the axial stretch QL/EA.
- **One-way state.** Each state is the unique admissible one, by enumeration
  with strict tests: R_S > 0 when active, u_z(S) > 0 when inactive.
- **Equilibrium.** Global and member equilibrium for every solve.
- **Identity.** The static solve with each variant's snubbers left out equals
  the base result. That holds by construction: it states the rule.
- **Separation.** Each control is more than 5e-2 relative from the base.

## Tolerance class

- **Base values, model SL:** the analytic tolerance of DEL-04-03/AC-001,
  "1e-9 relative, with its governed absolute floor near zero".
- **Base values, model SN:** the one-way state exactly, and values within the
  nonlinear class tolerance of DEL-04-07/AC-002.
- **The identity** ("unchanged") is discussed under open reading 1.

This package sets no number.

## Open readings

1. **What "unchanged" means.** DEL-04-10/AC-003 says every static result is
   "unchanged by the snubber".
   *Reading (a):* byte-identical to the same model without the snubber.
   *Reading (b):* equal within the case's verification tolerance.

   REQ-005 says that every static solve "ignores" the snubber. That supports
   (a), because nothing of the snubber enters the solve. The references are
   exact, and the expected values are identical under either reading.
2. **"No static reaction".**
   *Reading (a):* the snubber has no reaction row in a static result.
   *Reading (b):* it has a row that is exactly zero.

   "Ignores it" supports (a). No value depends on the choice.
3. **How the disclosure is counted.** "Every static load-case solve …
   discloses it" supports one finding per snubber in each static case. The
   other reading is one finding per snubber per result or model. Each case
   here is a single load case, so both readings give one finding per snubber
   per case.
4. **Skew snubbers in a model with nonlinear supports, or on the exact route.**
   DEL-04-03/REQ-010 leaves undecided whether skewed records are admitted in a
   model with nonlinear supports, and on the exact route.
   *Reading (a):* a snubber's skew record never enters a static solve
   (DEL-04-10/REQ-005, REQ-007), so that undecided set does not reach it. SN
   cases V3, V4 and V-ALL then solve, with the identity.
   *Reading (b):* the static solve still refuses the skew record by name. Those
   SN variants, and every variant on the exact route with SNB-3 or SNB-4, are
   then refusals, not identities.

   The values here are computed under (a), which DEL-04-10's text best
   supports.

## Reproduce

```sh
python3 -I -B generators/snubber_static_reference.py          # check frozen bytes
python3 -I -B generators/snubber_static_reference.py --write  # rewrite on purpose
shasum -a 256 -c SHA256SUMS
```

`SHA256SUMS` covers this README, the generator and `references.json`.
