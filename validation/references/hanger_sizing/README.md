# Hanger-sizing arithmetic reference (R-3, R-3b, R-4/R-4b arithmetic)

Independent reference values for DEL-04-11 hanger sizing.

- **Basis:** the DEL-04-11 hanger-sizing design, §4.2 (one origin,
  the installed position), §6 (loads, variation, travel and allowance, fit tests,
  family policy and selection) and §7 (the verification pass). It covers R-3
  (the arithmetic, the fit tests and both selection rules), R-3b (refusal when
  W ≤ 0) and the arithmetic part of R-4 and R-4b (verified loads, variation,
  departure, failed tests, verification standing). It does not cover the derived
  solves themselves.
- **Independence:** `generate_r3.py` was written from that design text and the
  task brief only. It reads, searches and imports no product code (`core/`,
  `apps/`, `tests/`) and no other generator under `validation/`. It uses only the
  Python 3 standard library.
- **Precision:** inputs are exact rationals of decimal strings (or stated
  fractions). Results are computed exactly with `fractions.Fraction` and emitted
  rounded to the nearest binary64. Fit tests, tie-breaks and verification tests
  are decided on exact values. Boundary-equality cases use inputs that binary64
  represents exactly. The generator checks that every discrete outcome stays the
  same when each input is replaced by the exact value of its emitted binary64.
  It also checks each case's intended outcome and the values stated in the brief.
- **Conventions:** SI units (N, m, N/m). `design_index` is 0-based into
  `u_operating_m`.
- **Absent verified ratios:** in a `verification` block, an entry of the
  `variation` list is `null` where the verified reference load is not positive
  (F_i^v ≤ 0 under `operating_load`). No ratio is formed for that case, V0
  reports it, and V is judged only on the ratios that are formed (§7). The
  sizing-time `variation` list is never `null`: sizing forms no ratio with a
  non-positive reference, because V0 fails the record first.
- **Invented data:** every value is invented. No supplier catalogue or standards
  data is used.

## Regenerate and check

```sh
python3 validation/references/hanger_sizing/generate_r3.py          # writes r3_arithmetic.json
python3 validation/references/hanger_sizing/generate_r3.py --check  # exit 1 if the frozen file differs
```

# Derived-solve reference (R-1, R-2, R-5)

`generate_r125.py` writes `r125_derived_solves.json`. It holds the sizing quantities
of the derived solves of §4.2 for each candidate: W (S_W), u_C (S_C) and u_i (S_Oi).
It also holds each lift-off support's state in every solve (§5).

- **Basis:** `hanger-sizing.md` §4, §5 and §12, the task brief and classical
  Euler–Bernoulli beam and frame mechanics, with axial deformation (EA) included and
  shear deformation neglected. The cases are:
  - R-1a, R-1b and R-1c: continuous beams with two equal, three equal and two
    unequal spans (three-moment equation);
  - R-2: a free riser with u_i = α·ΔT·H;
  - R-5a: a lift-off support that is active in S_W and S_C and inactive in S_O;
  - R-5b: the same beam with a P too small to lift the support.
- **Independence:** written from that text and the brief only. It reads, searches
  and imports no product code (`core/`, `apps/`, `tests/`) and uses only the Python 3
  standard library. It shares no code with `generate_r3.py`.
- **Precision:** every W and every u_C is exact (`fractions.Fraction`), and so is
  every u_i except R-5a's. R-5a's u_i is a rational divided by π. It is evaluated
  with π to 101 digits in 80-digit decimal arithmetic. All values are emitted
  rounded to the nearest binary64.
- **Self-checks:** each closed form must agree exactly with an exact-rational
  plane-frame stiffness solve written in the generator, which uses active-set
  iteration for lift-off. Every solve's vertical equilibrium is asserted. R-5's
  active-set consistency is asserted: a compressive reaction while the support is
  active, and a zero reaction with an upward gap while it is inactive. The lift-off
  states must not change when every input is replaced by its binary64 value.
- **Support model:** the R-1 ends are `rigid_uz`, with a `guide` at one end on UX,
  UY and RX and a `guide` at the other end on UY. No end restrains RY. RX is needed
  because no translational restraint on collinear nodes stops rotation about the
  beam axis. The out-of-plane DOFs carry no load, so no value depends on the shear
  modulus.

```sh
python3 validation/references/hanger_sizing/generate_r125.py          # writes r125_derived_solves.json
python3 validation/references/hanger_sizing/generate_r125.py --check  # exit 1 if the frozen file differs
```

# Exact-route sizing reference (EXS-A, EXS-B)

`generate_ex_sizing.py` writes `ex_sizing.json`. It describes each model as a
physical document and gives the sizing quantities a product test compares after
building that document: the rigid reactions and W, the positions of every force
solve, and, for each sizing run, every load, variation, fit outcome, selection
and the verification pass. EXS-A carries internal pressure, a ledger of stored
loads per case and moved non-candidate supports. EXS-B is a free riser on a
document with no pressurised region.

- **Basis:** DEL-04-11 CLM-001, REQ-002 to REQ-005 and REQ-010,
  `docs/theory/hanger_sizing.md` and the task brief. Classical Euler–Bernoulli
  beams with axial deformation, shear deformation neglected, small
  displacements, one straight circular tube section: D = 0.25 m, t = 0.01 m,
  E = 200 GPa, Poisson's ratio 0.3 (stated for the shear modulus the product
  derives; no value depends on it), α = 1.2e-5 1/K. d = D − 2t,
  A = π(D² − d²)/4, I = π(D⁴ − d⁴)/64, so EI = 3.462e6·π N·m².
- **Conventions:** Z is vertical, positive up. A candidate restrains UZ and
  its force acts along +Z. Every position is the UZ displacement from the drawn
  geometry, the installed position. A spring set to F_cold carries
  F_cold − k·u. Every load is a stored nodal force along Z; no self-weight,
  contents weight or line load is generated. A case includes stored loads by id
  at a stated factor (`includes`) and nothing else (`excluded_stored_loads`).
  Temperatures are in °C; each case's ΔT is its temperature minus the model's
  installation temperature.
- **The derived solves.** `weight_rigid`: the weight case with every candidate
  a rigid UZ restraint; W is its reaction. Force solves (W, O1, O2): every
  candidate removed and replaced by a force W along +Z, which is a further
  source of that case at factor 1 (`added_force_sources`). Verification: each
  selected variable record a spring of its rate plus the force F_cold; each
  constant selection and each unfitted candidate a force W.

## EXS-A: pressurised run, two candidates, moved supports

Nodes N1..N7 at x = 0, 2, …, 12 m on the X axis. S-1 (N1) and S-7 (N7) are
rigid UZ restraints that are never candidates. G-1 (N1: UX, UY, RX) is the only
axial restraint; G-7 (N7: UY) completes the out-of-plane restraint. Candidates
H-3 (x = 4) and H-5 (x = 8).

| Case | Ledger | Temperature | Imposed (non-candidate) | Pressure |
|---|---|---|---|---|
| W | F-1 ×1, F-2 ×1.5, F-3 ×1, PR-1 ×1 | 20 °C (installation) | none | 3.5 MPa |
| O1 (design) | as W, plus F-5 ×0.25 | 260 °C | S-1 −0.012 m, S-7 +0.012 m | 3.5 MPa |
| O2 | as W | 180 °C | S-1 +0.009 m, S-7 −0.009 m | 3.5 MPa |

F-4 (−10000 N at N4) is stored and included by no named case; F-5 (+2000 N at
N4) is included only by O1, at 0.25. F-2's factor 1.5 makes the midspan load of
span 2 9000 N.

**Pressure independence, by design.** The run lies exactly along X, has one
section, is closed at both ends and is restrained axially only at N1. Internal
pressure therefore produces only axial member strain (any Poisson or Bourdon
elongation) and end-cap thrusts p·A_i at N1 and N7, which are equal, opposite
and along X, so they cancel and G-1 carries nothing. In a straight member
with small displacements the axial and bending responses are uncoupled, so no
vertical reaction, vertical position, hanger load or sizing outcome depends on
the pressure or on whether pressure elongation is modelled. The package
exercises the bookkeeping of a pressurised document (support states per case,
the added-force ledger entry, the route) and does **not** test pressure
mechanics. No case's sized quantities depend on pressure. Uniform thermal
growth is likewise free along X and changes no vertical quantity.

**weight_rigid.** Three equal spans L = 4 m with midspan loads P = 8000, 9000,
5000 N. Three-moment equation with zero end moments:
4M₂ + M₃ = −(3L/8)(P₁ + P₂), M₂ + 4M₃ = −(3L/8)(P₂ + P₃), so M₂ = −5400 and
M₃ = −3900 N·m. Span end shears V_l = P/2 + (M_r − M_l)/L,
V_r = P/2 − (M_r − M_l)/L give S-1 2650, H-3 10225, H-5 7600 and S-7 1525 N,
independent of E, A and I.

**Force solves.** With both candidates replaced by forces the run is a simply
supported 12 m span, which is statically determinate. Each position is the
settlement line g₁(12 − x)/12 + g₇x/12 plus the deflection of the case's
net forces, with f(x, a) = b·x·(L² − b² − x²)/(6EIL), b = L − a, x ≤ a
(symmetric in x and a). W's loads with the forces W give zero deflection at
both candidates, so u_C = 0 and O2 is pure settlement: u = +0.003 m at H-3 and
−0.003 m at H-5, exact. O1 adds +500 N at x = 6:
500·f(4, 6) = 500·f(8, 6) = 15333⅓/EI ≈ 1.40981 mm, so u_O1 = −0.004 + that
at H-3 and +0.004 + that at H-5. End reactions are statics of the span.

**Failure signature.** `failure_signature_positions_m` gives each candidate's
position if the added forces W did not act: the same span carrying only the
case's loads and settlement (about −44 mm in W). These are not expected values.
A product that leaves the added force out of a case's ledger fails the position
comparison by about that much.

**Runs.** The catalogue is invented. H-3 is sized with k = 100000 N/m records
whose ranges discriminate the two load designs; VS-V (k = 800000) fails V;
VS-T fails T; VS-A fails L; VS-UX fails E2; VS-NR fails E1.

| Run | Choice | H-3 | H-5 | Verification |
|---|---|---|---|---|
| A-HOT | hot, operating load, λ = 0.25, a = 1 mm, smallest capacity | VS-B, F_cold = W + k·u_O1 | constant: CE-1 | CE-1 fails T (verified travel 11.42 mm > 11 mm) |
| A-COLD | as A-HOT but cold | VS-C, F_cold = W; VS-B fails L high | constant: CE-1 | u_C^v = 0; CE-1 fails T |
| A-NOFIT | as A-HOT, H-5 variable | VS-B | no record fits (L, V) | conditional; H-5 verified as a force W |

The spring at H-3 opposes H-3's motion, and through the span pushes H-5 the
other way. In O1 and O2 H-3 and H-5 move in opposite senses, so the spring
widens H-5's travel, which is the mixed variable and constant reference of
AC-005. In a linear model the departure is exactly zero under either design: a
spring set by hot design carries W at u_O1, which is the design force solve's
equilibrium; set by cold design it carries W at u_C = 0. A departure of zero is
a closed-form zero and is judged with the absolute floor.

## EXS-B: free riser, no pressurised region

N1 (0, 0, 0), N2 (0, 0, 4), N3 (0, 0, 8); anchor A-1 at N1 (all six DOFs);
candidate H-3 at N3 (UZ). F-T −9000 N at N3 and F-M −4000 N at N2 in every
case. Installation temperature 15 °C; W at 15 °C, O1 at 315 °C (ΔT = +300 K),
O2 at −35 °C (ΔT = −50 K).

- weight_rigid is a fixed-fixed axial bar: W = 9000 + 4000/2 = 11000 N; A-1's
  UZ reaction 2000 N, independent of EA.
- In each force solve the riser is a cantilever. The upper half carries
  +2000 N and the lower half −2000 N, so the net shortening is zero and
  u = α·ΔT·H from the installation temperature: 0, +0.0288 m, −0.0048 m.
- Run B-COLD: cold design, design-load variation (reference W), λ = 0.3,
  a = 2 mm, least variation. VS-R1 and VS-R2 fit; least variation takes VS-R1,
  where smallest capacity would take VS-R2; VS-R3 fails V. Verification, with
  one spring on an axial bar: u^v = (α·ΔT·H + (F_cold − W)·H/EA)/(1 + k·H/EA),
  so u_C^v = 0 and the departure is 0.
- Failure signature: u = α·ΔT·H − (2·9000 + 4000)·H/(2EA).

## Readings taken

- **"A non-candidate rigid support that stays"** is read as a non-candidate
  restraint that stays rigid in every derived solve. S-1 and S-7 both stay,
  and both are moved in O1 and O2. The riser's anchor never moves.
- **Design-load reference** is the hot-load basis W, as in the theory note and
  R-3.
- **Verified tests** follow R-4: V0 on the verified loads, L on the envelope
  of the verified span widened by k·a, T on the verified travel, V on the
  ratios that are formed.
- **An unfitted candidate's verification** is null (H-5 in A-NOFIT). The null
  means that this reference supplies no expectation for it. The scope requires
  verified values for every selected family and says nothing of an unfitted
  candidate. A product comparison must therefore not assert that the product
  publishes nothing there. The run's `unfitted_verification_note` says the
  same. The candidate is verified as a force W, listed in
  `unfitted_verified_as_force_w`, and that force enters the other candidates'
  verified values.
- **`f_cold_n`** of a constant selection is its constant load W, as in R-3.
  `first_failed_test` lists each record of the families tested, "fits" or the
  first test it fails (REQ-004).
- **Pressure in the weight case.** W carries the same pressure as the
  operating cases (a sustained case). This changes nothing, by the argument
  above.

## What a product test must map

The reference states physics, not the product's representation. A test must
map:
- the support kinds: rigid single-DOF restraints, the UX/UY/RX guide, the
  anchor and the placeholder candidates;
- each case's ledger: inclusion by id and factor, and the stored load that is
  not included;
- PR-1 as the product carries internal pressure (a ledger load or an
  operating-condition set), with the end caps closed;
- each case's temperature as an operating temperature against the installation
  temperature, or as the equivalent uniform ΔT;
- the imposed displacements as the non-candidate supports' per-case states,
  with each candidate's state in every named case active and unmoved;
- the section with nominal wall, so that EI and EA are those of the stated tube
  (no corrosion allowance or mill tolerance), and no generated self-weight or
  contents weight;
- the identifiers the product gives the added forces in each derived ledger.

## What the package shows and what it does not

It shows W, positions, loads, variations, fit outcomes, selections and the
verification on a pressurised document with a case ledger, moved non-candidate
supports and a reference temperature; the dependence of every expected value on
the ledger (F-2's factor enters W; F-5 enters O1 only; including F-4 or F-5
anywhere else changes W or the positions); and riser growth from the
installation temperature. It does not test pressure mechanics, nonlinear
supports (R-5 does), the boundary equalities of the fit tests (R-3 does), or
whether the product's EX section basis equals the stated tube.

- **Independence:** written from the governing texts above and the brief. It
  reads, searches and imports no product code and no other generator, and
  shares no code with any. Standard library only.
- **Precision:** inputs are the exact rationals of the decimals in the
  generator. W, the EXS-A O2 positions and every riser sizing value are exact
  rationals. Values that involve EI or EA contain π. They are computed exactly
  at rational lower and upper bounds of π from Machin's formula (120 digits,
  with an integer error bound). Every emitted value is the nearest binary64,
  and it is the same at both bounds.
- **Self-checks, on every run:**
  - each closed form above agrees exactly with an exact-rational plane-frame
    stiffness solve written in the generator;
  - every solve balances vertically and horizontally;
  - every EXS-A solve is repeated with internal pressure modelled as end-cap
    thrusts and a Poisson strain, and every vertical quantity is unchanged;
  - every discrete outcome (fit, first failed test, selection, verified failed
    tests, standing) is the same at both bounds of π and when each input is
    replaced by the exact value of its binary64;
  - the selections, the verified T failure, the zero departures and the values
    stated above are asserted.
  No case has a boundary equality.
- **Invented data:** every value is invented. No supplier catalogue or
  standards data is used.

```sh
python3 validation/references/hanger_sizing/generate_ex_sizing.py          # writes ex_sizing.json
python3 validation/references/hanger_sizing/generate_ex_sizing.py --check  # exit 1 if the frozen file differs
```

# Spring preload reference (P-A, P-B)

`generate_preload.py` writes `preload.json`. The cases are closed forms for a
variable spring hanger modelled as a spring of its rate k plus its cold load
F_cold as a constant force along +Z in every load case (DEL-04-03 REQ-014 and
AC-009; DEL-04-11 REQ-014 and AC-014). Each case gives every node's UZ
displacement and every **published support force**. The spring's published
force includes the preload: F_cold − k·(u − g), where g is the base
displacement along the spring's axis. A product test runs them on both solve
routes; the test cases run on the exact route only.

- **Basis:** the scopes above, DEL-05-01's definitions of the test condition
  and held support, and the brief. The beams and section are those of the
  sizing package: horizontal Euler–Bernoulli beams along +X, rigid UZ ends, UX
  at N1 only, UY at both ends and RX at N1. Every published force is the force
  on the pipe, positive along +Z.
- **P-A:** an 8 m span (N1..N5, 2 m apart) with spring hanger SH-3 at midspan,
  k = 150000 N/m, F_cold = 6000 N. Closed form, simply supported span with
  f(x, a) as above:
  u₃ = (Σ_j f(4, x_j)·F_j + f(4, 4)·(F_cold + k·g)) / (1 + k·f(4, 4)),
  then the spring force, and the end reactions by statics.

  | Case | Loads | Exercises |
  |---|---|---|
  | PA-WT | F-A −5000 N at N2, F-B −7000 N at N4 | weight with preload |
  | PA-EMPTY | none | the cold load alone lifts the span |
  | PA-BASE | as PA-WT, SH-3's base moved −0.004 m | F_cold − k·(u − g) |
  | PA-B, PA-AB | F-C −3000 N at N4; PA-WT's loads plus F-C | superposition |
  | PA-HT | PA-WT's plus F-H1, F-H2 (−4000 N at N2 and N4), test pressure 6 MPa, springs held | held spring |
  | PA-HT-NONE | the same, held supports "none" | the unheld contrast |

  Held: SH-3 is a rigid UZ restraint at zero, and neither k nor F_cold acts.
  The model is then two 4 m spans with midspan loads P₁ = 9000 N at N2 (F-A
  plus F-H1) and P₂ = 11000 N at N4 (F-B plus F-H2):
  M_B = −3L(P₁ + P₂)/32 = −7500 N·m. The reactions are P/2 + M_B/L at the ends
  and (P₁ + P₂)/2 − 2M_B/L at SH-3: S-1 2625, SH-3 13750 and S-5 3625 N, exact.
- **P-B:** a 12 m span with SH-3 at x = 4 (k = 120000, F_cold = 5000 N) and
  SH-5 at x = 8 (k = 200000, F_cold = 7500 N), loads −6000, −9000 and −4000 N
  at x = 2, 6 and 10. Closed form: the two equations
  u_i + Σ_j f_ij·k_j·u_j = Σ_l f_il·F_l + Σ_j f_ij·(F_cold,j + k_j·g_j),
  solved by Cramer's rule. Cases PB-WT, PB-EMPTY and PB-BASE (SH-5's base
  moved +0.003 m).
- **Two published rows per hanger:** each case gives every hanger's
  `published_support_force_n` (F_cold − k·(u − g), or the held restraint's
  reaction) and its `published_applied_force_n`. The applied force is the
  constant the solve publishes for an acting spring's cold load, along +Z. It
  equals F_cold. It is a statement of what is published, not a computed
  mechanics value. A held spring publishes no applied force: its row is null,
  with the reason in `applied_force_null_reason`. That absence is part of the
  expectation.
- **Force plus spring:** in every unheld case the displacements equal those of
  the same model with each hanger replaced by a plain spring of rate k (force
  −k·(u − g)) and a separately applied force F_cold at its node.
  `force_plus_spring_twin` gives both parts; they add to the published force.
- **Superposition:** for A = PA-WT and B = PA-B, `superposition` gives the
  direct solve of A + B (PA-AB, one cold load), the naive sum of the solved
  states of A and B (factor sum 2, two cold loads), and their difference. The
  difference equals PA-EMPTY exactly, in displacements and in every published
  force: the response to one extra cold load. A combination gate can use it.

## Readings taken and what a product test must map

- **Test condition.** PA-HT and PA-HT-NONE declare a test condition, which
  DEL-05-01 solves on the exact route only. The run's single axial restraint
  and closed ends make the test pressure change no vertical quantity, as in
  EXS-A. The test's contents selection must add no load. The vertical loads
  are only the listed nodal forces.
- **Moved base.** g is a displacement of the spring's base along +Z, entered as
  the product enters an imposed displacement at a spring's base.
- **Published force sign.** Rigid reactions are given as the force on the
  pipe, positive along +Z. A test maps the product's sign convention.
- **Section.** As in EXS-A, the stated tube with nominal wall and no generated
  weight.

## What the package shows and what it does not

It shows the preload in displacement and published force: in a loaded case, in
a case with no other load, with a moved base, with two springs, held and
unheld, the force-plus-spring identity and the doubled preload of a naive sum.
It does not cover the diagnostics, the memory pricing or the reference system
(AC-014's other parts), or the combination gate's rule itself.

- **Independence:** as for the sizing package; it shares no code with any
  other generator.
- **Precision:** displacements and unheld forces contain π through EI. They are
  computed exactly at both bounds of π and emitted as the nearest binary64,
  which is the same at both. The held reactions are exact rationals. Emitted
  values are not compared under binary64-replaced inputs. A few move there by
  one unit in the last place:
  - PA-AB's displacement at N2 and N3, and its hanger row, which equals N3;
  - PB-EMPTY's displacement at N6;
  - the twin's plain-spring force in PA-BASE and in PA-HT-NONE;
  - the matching superposition rows (direct at N2 and N3, naive sum at N4).

  That is far inside the 1e-9 comparison tolerance.
- **Self-checks, on every run:**
  - every closed form, at the hanger and at every node, agrees exactly with an
    exact-rational stiffness solve written in the generator;
  - every solve balances vertically and axially, and the axial restraint
    carries nothing;
  - each test case is repeated with end-cap thrusts p·A_i, and nothing
    vertical changes;
  - each unheld case is re-solved as a plain spring plus a force F_cold, and
    the displacements and reactions are identical;
  - the superposition difference equals PA-EMPTY exactly;
  - with each input replaced by its binary64 value, the stated properties
    are checked again: the held reactions, the held hanger at zero, every
    acting spring's force positive and the unloaded span lifted. Emitted
    values are not compared there.
- **Invented data:** every value is invented.

```sh
python3 validation/references/hanger_sizing/generate_preload.py          # writes preload.json
python3 validation/references/hanger_sizing/generate_preload.py --check  # exit 1 if the frozen file differs
```
