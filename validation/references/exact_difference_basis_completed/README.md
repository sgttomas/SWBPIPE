# Exact-route references: the declared difference, completed admission policy

Exact analytical values and policy verdicts for the declared difference, computed
without product code: a result-state subtraction declared `operating_minus_sustained`
on the exact route ([DEL-05-02](../../../execution/PKG-05/DEL-05-02/ScopeOfWork.md)
REQ-012 and AC-003). This successor to
[`exact_difference_basis`](../exact_difference_basis/README.md) carries the
predecessor's 19 admission rows and 17 pair comparisons and adds new witnesses. Each
record has an admission entry and a mechanical entry, with `none` for the carried
admission rows and for X5. The references are candidates. S is the specified reading
of the stiffness part, and R is recorded for comparison.

The generator uses the Python standard library only. Its inputs are the predecessor's
inputs, restated in the generator, and inputs invented here. No value comes from a
design code, catalogue or vendor.

**Status.** Candidate references, derived and cross-checked by their author, awaiting
independent refutation. The package exercises its reference calculation and admission
policy on recorded inputs.

**One matter as decided at freezing.** The stiffness part (G-2) is built as the
specification states it, reading S. It stands as decided when the package is frozen.

| File | Role |
|---|---|
| `generators/exact_difference_basis_completed_reference.py` | the generator: exact arithmetic, the frame and bar solutions, the policy, every row |
| `references.json` | the inputs, every row and pair with both entries, and the cross-check counts |
| `SHA256SUMS` | hashes of this README, the generator and `references.json` |

```
python3 -I generators/exact_difference_basis_completed_reference.py          # rebuild in memory, compare byte for byte
python3 -I generators/exact_difference_basis_completed_reference.py --write  # rewrite references.json and SHA256SUMS
shasum -a 256 -c SHA256SUMS
```

The package test (`tests/test_exact_difference_basis_completed_references.py`)
compares in-memory regeneration with the recorded files, verifies the hashes, checks
imports and literal paths by AST inspection, and compares carried verdicts and
analytical values with `../exact_difference_basis/references.json`. The verifier
(`tests/verifiers/exact_difference_basis.py`) shares no code with the generator. Its
test (`tests/test_exact_difference_basis_verifier.py`) compares verdicts and gate
outcomes and checks eight deliberate policy changes. Tests establish separate
implementations and agreement on this corpus. The generator's own checks share inputs
and authorship.

**Re-freeze rule** (the predecessor's). No value changes to match a product output.
A disagreement goes back to this reference's owner and an independent refuter.

## What this supersedes

This package supersedes, for the meaning of G-1 content on the exact route, two
definitions in `validation/references/exact_difference_basis/README.md`, section "G-1:
what 'contained by content and with multiplicity' means":

- "**A pressure region's content** is its members and its pressure's bits."
- "**A member state** is compared as authored: member, kind, expansion law reference,
  and the temperature's bits and unit. It is not compared by the resolved strain."

It also states the meanings of motions and generated loads, which that package left
open. The old package stays byte for byte as history and keeps its own test.

**The reasons.**

- A terminal's transfer changes the pressure action.
- The fit, the installation temperature, an interval strain and a coefficient each
  change the free strain.
- A generated load's extent and recipe change its action.
- A zero motion is meaningful only against the same restraint.

**Why the carried rows keep their verdicts.**

- In every predecessor model, every terminal is `transfers_to_wall`. The predecessor's
  README says so in "Which constraint each model uses", and says that its balance
  "holds because every terminal transfers to the wall".
- The predecessor's member states share one reference configuration and have no fit.
- The predecessor's numerical pairs have no generated loads and no prescribed motions.

## The admission policy

A declared subtraction S → O is admitted only when all three parts hold. S is the
subtrahend ("sustained") and O the minuend ("operating"). The parts are evaluated in
this order:

1. **Restraint:** O and S have the same restraints and devices.
2. **G-1:** S's actions are contained in O's, by sameness and with multiplicity.
3. **G-2:** S and O have the same modulus basis.

The first failed gate determines the refusal, and the record includes each gate
outcome. Multiplicity means that, for every action, S's count does not exceed O's.

### Primitive loads

Two primitive loads are the **same action** when they have:

- equal category, target, direction and dimension;
- equal magnitude in canonical units (the binary64 bits of the normalized value);
- an equal unit after normalization;
- equal ledger factor bits.

The rules:

- A case with no ledger applies every stored primitive once, at factor 1.0.
- With a ledger, only the referenced primitives count, each once per reference.
- The key is never the rounded product of magnitude and factor.
- Identity and provenance are not compared.

Count stored authoring-plan weights, such as self-weight, as primitives: a stored
primitive is an authored operand of the document, whatever wrote it, and the solve
consumes no recipe for it. Compare solve-time generated actions by recipe.

### Pressure

A region is the same action when it has the same:

- sorted pipe members;
- sorted connector members;
- sorted terminals as (node, closure transfer): `transfers_to_wall` or
  `separately_supported_or_compensated`;
- pressure (binary64 bits, Pa).

**Elongation** is an action of its own. When a pressure region is present, switching
elongation on adds one action. W1 satisfies the other gates.

### Generated loads

Two generated loads are the same action when they have the same resolved generation
recipe:

- the kind (`seismic` or `wind`);
- the target member;
- the extent, as resolved start and end fractions. The whole span is [0, 1], and an
  absent extent resolves to it.
- each resolved input, normalized. For seismic: the axis and its factor. For wind: the
  direction, the pressure, the shape factor and the exposed-diameter basis.
- the mass basis: the mass method and every section and component mass input the
  generator reads (X5).

Generated loads are counted with multiplicity, one per marked span or generated
primitive.

Equal formed values can still come from distinct recipes. G-1 fails if the
subtrahend's recipe is missing, as in W8.

### Restraints, then motions

**Restraint first.**

- Each support DOF has a resolved law in each operand: free, rigid (with its support
  identity), spring (with its identity, participation and stiffness) or nonlinear
  (with its identity and support law).
- The restraint part holds only when every support DOF has the same law, with the same
  identity, participation and parameters, in both operands. A DOF absent in a case and
  one declared free normalize to one free law.
- A free DOF in one operand and a DOF fixed at zero in the other differ (W6).
- A removed or held spring is a different case (W7a), and so is a different spring at
  the same node (W7b).
- A rigid restraint with another identity is a different case (X3).
- The same holds for a nonlinear support's law.

**Then motion increments.**

- Once the same restraint is established in both operands, a motion that compares
  equal to zero is the absence of a motion increment. This holds for either sign, and
  whether the zero is authored explicitly or defaulted (W9).
- A non-zero motion is an action. It is compared by its target (support, DOF and
  field: rigid boundary, nonlinear ground or spring base) and by its value bits in
  canonical units.
- S's non-zero motions must be contained in O's.

**Spring base and nonlinear ground offset.** On the same active spring, movement of a
previously stationary base adds an action (W4). The same rule applies to ground
movement under an unchanged nonlinear law.

### Member free-strain states

Two states are the same action when they have the same member, the same kind, and
the same operands as the solve consumes them, in canonical units:

| Kind | Operands |
|---|---|
| `unchanged_reference` | none |
| `explicit_interval_strain` | the strain |
| `constant_alpha_interval` | the coefficient per K and the interval in K |
| `free_length_state` | the installation temperature in K, the operating temperature in K, and the expansion law's content (definition, datum K, constant or table points with interpolation) |

They must also have the same fit, as the solve consumes it:

| Fit | Operands |
|---|---|
| none | none |
| fit strain | the strain |
| natural length change | the change in m and the reference length in m |

No reference-configuration identifier is compared. Two identifiers that resolve to
identical operands give the same action (X2).

**Omission is fail-closed.** The thermal part is omitted for `unchanged_reference`,
zero explicit interval strain, a zero coefficient or interval in
`constant_alpha_interval`, or equal installation and operating temperatures in
`free_length_state`. Omit the state only if the fit is also absent or its strain or
length change is exactly zero. A total that rounds to zero never omits a state. A
`free_length_state` with a constant zero law and unequal temperatures stays an action
(X4).

### Fixed loads

These loads are present in every case of a model and are not actions:

- constant-effort forces;
- a connector's installed reference;
- a tie rod's reference extension;
- a spring's cold load, where the model applies it.

Each fixed source contributes 1 − 1 = 0 to κ. Its response cancels on a shared linear
operator. The two operands' fixed-load inventories must be equal. An unequal inventory
is outside this policy, and the row is refused under restraint, with that reason.

### Two readings of the decided policy, made at freezing

Each reading below is the refusing direction: it can refuse a pair that the wider words
would admit, and never admits one they would refuse. Neither changes a recorded
verdict: the 53 rows recorded before these readings keep every verdict and gate
outcome. Both were confirmed by SWBPIPE's Agent 0 on 2026-11-18.

- **A rigid restraint's identity.** The policy's words (specification section 3.4):
  "The restraint part holds only when every support DOF has the same law, with the same
  identity, participation and parameters, in both operands." Read here: a rigid law is
  compared with its support identity; a free law carries none, and a DOF absent in a
  case and one declared free are one free law. X3 is the control: W3 with only the
  rigid stop's identity changed is refused under restraint.
- **The zero rule, kind by kind.** The policy's words (specification section 3.5): "the
  thermal part is `unchanged_reference`, a zero strain, a zero coefficient, a zero
  interval, or installation equal to operating". Read here as in "Omission is
  fail-closed" above: "a zero coefficient" omits only a `constant_alpha_interval`
  state, so a `free_length_state` whose law has a constant zero coefficient and whose
  temperatures differ stays an action. X4 shows it: refused under G-1, with a zero
  free-strain increment.

### G-2: the stricter stiffness policy

With restraint and G-1 satisfied, S can refuse mechanically identical operands when
their selection differs. R compares E, ν, G, material, constitutive basis and section
on each member. S also compares selection kind, case basis label and the selection
record's kind, temperature, interpolation fraction, selected E and selected ν.

The generator and the verifier each define the stiffness comparison at one point,
with both readings side by side. Every row's `admission` is under S. Every row also has
`stiffness_readings`, which gives the verdict and the G-2 outcome under each reading,
and `differ`, which is true where the two verdicts differ.

### Where the two stiffness readings differ

| Row | Under S | Under R | Fields that differ |
|---|---|---|---|
| K1 | `refused_g2` | `admitted` | selection kind (`exact_basis_reference`, `solve_temperature`); case basis label (`REF`, `T300`); the selection record's kind and interpolation fraction (none, 0) |
| W11 | `refused_g2` | `admitted` | case basis label (`W11_LO`, `W11_HI`); the selection record's selection temperature (250 K, 650 K) |

No other row differs. The mixed-basis pair has another E, so both readings refuse it.
`fails_g1_and_g2` fails G-1 first, so both readings refuse it under G-1.

**Quoted as written.**

- `validation/references/exact_difference_basis/README.md`, section "G-2: one modulus
  basis": "G-2 is read as the two cases selecting the same basis: the same exact
  reference, or the same solve temperature. Whether a reference and a temperature that
  resolve to equal moduli count as one basis is not exercised here."
- The same README, section "Open readings as they now stand": "**One basis.** Whether
  a basis reference and a solve temperature that resolve to equal moduli are one basis
  (G-2)."
- The pressure-free comparison, `docs/theory/load_combinations.md`, section "Declared
  operating comparison (operating-case slice 1b)": "then G-2,
  `EXPANSION_BASIS_MODULUS_MIXED`, when the two cases' modulus-basis keys differ."
- The same note, section "Assumptions: modulus basis, stiffness and nonlinear
  supports": "A linear, absolute-sum or SRSS combination assumes one stiffness: every
  load case it reaches through the operand graph, subtractions and envelopes included,
  must share one modulus basis (and, on the exact route, one resolved member
  stiffness)."

## Two entries per row, never merged

**`admission`** has these fields:

- `verdict`;
- `reason`, one sentence;
- `gates`, with each part's `holds` and why;
- `gate_only`, with its `gate_only_reason`.

A gate-only row supplies a policy verdict and a mechanical idealization where defined.
Its reason states the known obstruction or the unresolved document assumption. A false
mark identifies a proposed document case or a carried case from the predecessor. This
package does not validate document reachability.

**`mechanical`.** Mechanical values are under `rows` or `values`. They are analytical
O − S unless a field names an operand or a control. Each gives `exact`, `value` and
`unit`.

- `linear_increment` gives the shared linear operator's response to the difference in
  effective excitation, including prescribed movement where applicable. F2, F2b and X4
  also give the free-strain increment itself.
- `state_difference` records a subtraction of reference states for W6, W7a, W7b, X3,
  the mixed-basis pair and W11. W7b's spring identities differ but its numerical states
  coincide.
- `none` supplies no numerical result (the carried admission rows and X5).

A policy refusal can coexist with an analytical value, as W5, X1, W8, K1, W11 and X4
show.

## Exact form

Each value is a finite sum of rational multiples of integer powers of π, written as in
`3/4*pi + 1/2 + 5/7*pi^-1`. This is the predecessor's convention. Each value's `value`
is the nearest binary64, ties to even, using rational arctangent bounds on π. The
bounds are 12752·2⁻⁶⁴⁰ apart.

## The input format

Case sides contain normalized policy operands. Numerical values may be exact rational
strings or, where shown, rational multiples of π. The record includes bits for fields
compared by bits. A side has these fields:

- `case_id` and `ledger`;
- `primitives` (`value`, `value_bits`);
- `pressure_regions` (pipe and connector members, terminals, pressure and its bits);
- `pressure_elongation`;
- `generated` (kind, target, `extent`, `inputs`, `mass_basis`);
- `supports` (support, DOF, law, identity, participation, parameters);
- `motions` (support, DOF, field, value and its bits, authored);
- `member_states` (member, kind, operands, fit, `reference_configuration`);
- `fixed_loads`;
- `basis`, per member.

## Carried rows

The predecessor's inputs are restated in the generator. The predecessor's README gives
them under "Inputs" and "Which constraint each model uses".

- Every predecessor terminal is `transfers_to_wall`, and each anchor's six DOFs are
  rigid.
- The modulus basis of each member is `REF` (the exact basis reference to the 300 K
  point) or `HOT` (a solve temperature of 600 K, interpolated at the fraction 3/4).
- The three carried free-length witnesses restate the predecessor's two operating
  temperatures and share an invented 300 K installation temperature and no fit.
- The constant-effort witness has the same fixed force in both operands.

**The verdict mapping.** Each carried row's verdict equals the predecessor's
`outcome`, mapped as follows:

| Predecessor `outcome` | Completed verdict |
|---|---|
| `published` | `admitted` |
| `refused_g1` | `refused_g1` |
| `refused_g2` | `refused_g2` |
| `refused_g1_and_g2` | `refused_g1` (G-1 is ordered first) |

**Carried mechanical values.** Each carried pair names the predecessor's values field
it reproduces (`frozen_values_field`). The carried comparison covers 1 806 analytical
W values and 531 differing analytical J values. For the ten same-pressure, same-basis
pairs it records analytical identity. The predecessor retains its rounded-operand
publication comparison.

**What the generator checks.** The generator checks global force and moment balance
on 66 carried case solves and 17 differences, compares wall form with free strain for
motions, reactions and axial recovery on 19 carried on-setting W solves, and compares
three M1 reactions with the flexibility formula on REF and HOT. With K1 and W11 the
overall counts are 70 and 21.

## Added rows

**Shared inputs for W1 to W9.**

- One straight member AB along +x, with L = 1 m and A at the origin.
- A is anchored: all six DOFs are rigid.
- Outside diameter 1/4 m, wall 1/32 m, no allowances. So r_i = 3/32 m and the wall
  area is 7π/1024 m².
- E = 2²⁰ Pa and ν = 1/4, so EA = 7168π N.

W1–W9, F1, F2, F2b and X1–X4 use axial bar models; K1 and W11 use the M1 frame. The
generator checks selected exact bar results against formulas and identities.

In the table, "verdict" is the admission verdict and "mechanical" is the result
(O − S unless the row says otherwise).

| Row | S | O | Verdict | Mechanical | Gate-only |
|---|---|---|---|---|---|
| W1 elongation added | region AB, both terminals `transfers_to_wall`, p = 1024 Pa, elongation off | the same, elongation on | `admitted`: elongation is an added action | `linear_increment`: Δu_B = 9/14336 m | no |
| W2 shared-terminal grouping | AB and BC collinear (C at x = 2 m), each with its own material record (ν_AB = −1/8, ν_BC = 1/4), elongation on; region AB with terminals A and B | that region, plus region BC with terminals B and C, at the same pressure | `admitted`: BC's region is an added action | `linear_increment`: Δu_B = 0, Δu_C = 9/14336 m; N_mech of BC in O = 9π/2 N = ½·p·π·r_i² | no ⁸ |
| W3 operating-only rigid motion | rigid line stop at B.UX, no motion | the same stop, B.UX moved +1/1024 m | `admitted`: the same restraint; S's zero is no increment | `linear_increment`: Δu_B = 1/1024 m, N = 7π N, reaction at A −7π N | no |
| W4 operating-only spring base | spring K at B.UX, k = EA/L, base unmoved | the same spring, base moved +1/1024 m | `admitted`: the same spring; an added excitation | `linear_increment`: Δu_B = 1/2048 m; spring force on the pipe 7π/2 N | no |
| W5 wind extent substitution | wind along +x, 32 Pa, shape factor 1, exposed diameter 1/4 m (q = 8 N/m), span [0, 1/2] | wind on [1/2, 1], plus nodal +3 N at A.UX and +1 N at B.UX | `refused_g1`: S's recipe is not in O; same-bit nodal forces are not the same action | `linear_increment`: EA·Δu_B = 3 N·m (the nodal forces alone give 1 N·m); consistent A/B forces (3, 1) N in S and (1, 3) N for O's wind | yes ¹ |
| W6 free DOF against one fixed at zero | B.UX free; +7π N at B | B.UX rigid, zero motion; the same load | `refused_restraint` | `state_difference`: u_B(S) = 1/1024 m, u_B(O) = 0, difference −1/1024 m | yes ² |
| W7a removed spring | spring K at B.UX, k = EA/L; +7π N at B | no spring | `refused_restraint` | `state_difference`: u_B(S) = 1/2048 m, u_B(O) = 1/1024 m, difference +1/2048 m | yes ² |
| W7b different spring at the same node | spring K1, k = EA/L; +7π N at B | spring K2, the same k; the same load | `refused_restraint`: another identity is another case | `state_difference`: 0 m; the two states are equal ³ | yes ² |
| W8 two recipes, equal values | whole-span wind 0.1 Pa (its binary64), shape factor 10, D = 1/4 m | whole-span wind 1 Pa, shape factor 1 | `refused_g1`: the recipes differ (conservative) | `linear_increment`: the reference multiplication gives 1/4 N/m for both rounded intensities, so 0. The exact recipe difference at B is −2⁻⁵⁷ N | yes ¹ |
| W9 explicit zero against default zero | line stop at B.UX with authored motion −0.0 m | the same stop, no authored motion | `admitted`: a zero of either sign normalizes with the default | `linear_increment`: 0 | no |
| F1 terminal witness | pressurised cantilever, free-end terminal `transfers_to_wall` | the same, free end `separately_supported_or_compensated` | `refused_g1` | `linear_increment`: Δu_B = −p·A_i·L/(E·A) = −64607/81065856 m ⁴ | no |
| F2 fit witness | fixed-fixed bar (A anchored, rigid stop at B.UX); free-length state: datum and installation 300 K, operating 500 K, α = 2⁻¹⁶ /K, fit strain 2⁻¹⁰ | the same fixed-fixed bar and state, no fit | `refused_g1` | `linear_increment`: strain increment −8217/8388608; N = 57519π/8192 N | no |
| F2b installation-only variant | fixed-fixed bar; as F2's O: installation 300 K, no fit, reference identifier `RC-AB-INSTALLED-300` | the same fixed-fixed bar; installation 400 K, no fit, reference identifier `RC-AB-INSTALLED-400` | `refused_g1` | `linear_increment`: strain increment −205425/134422528; N = 1437975π/131272 N ⁵ | no |
| K1 selection record only | frozen M1: weight, both regions at 5.75 MPa, elongation on, ΔT on every member; basis `REF` (exact 300 K point) | the same actions; basis a solve temperature of 300 K, interpolated between 300 K and 700 K at fraction 0 | `refused_g2` under S (`admitted` under R): the selection differs although E, ν and G are equal | `linear_increment`: 0 on each of M1's 84 rows (common operator, equal load vectors) | yes ⁶ |
| W11 equal moduli, different expansion coefficients | M1, no pressure; weight at B, ΔT = 475/2 K on every member; the invented material's 250 K point (α = 5·2⁻¹⁹ /K) | the same actions; that material's 650 K point (α = 7·2⁻¹⁹ /K), the same E, ν and G | `refused_g2` under S (`admitted` under R): the case basis label and selection temperature differ | `state_difference` on the common operator: the response to (α_O − α_S)·ΔT = 475/524288 on every member; see "W11" below | yes ⁷ |

1. Whether exact-route wind generation admits a load along the member axis is not
   established here, because establishing it would need product code.
2. W6, W7a and W7b compare different model-level support definitions and are
   gate-only.
3. The specification gives no load for W7b. The common +7π N of W7a is used, and the
   difference is zero with or without it.
4. F1 uses the predecessor's section, E = 194 750 000 000 Pa and p = 5 750 000 Pa,
   with L = 15/4 m. It is solved with elongation on in both operands. The increment
   depends on neither the setting nor ν.
5. The specification gives F2b's value and "installation 400 K in O". The stated value
   is the increment with no fit in either operand, so that is how F2b is built. The
   secant strain of F2 is
   (1 + α·(T_op − T_inst)/(1 + α·(T_inst − T_datum)))·(1 + fit) − 1.
6. K1 is a gate-only analogue of W10. It uses M1 and synthetic selection records,
   including fraction 0. W10 specifies a common 1 N bar load and an endpoint record
   without a fraction. K1 establishes S and R's verdicts for its supplied records and
   zero analytical difference on the common M1 operator.
7. Not established here whether the exact route's resolver accepts a material whose
   points share E and G but have different expansion coefficients, and records the two
   selections as given. Establishing it would need product code.
8. AB and BC have distinct material records with the listed moduli. Reachability with
   the auxetic AB material has not been established. Both terminals of each region
   transfer to the wall. The specification's binary64 formation observation is outside
   the analytical comparison.

### W11: equal moduli, different expansion coefficients

W11 uses the predecessor's M1 geometry and section with no pressure. Both operands
carry the same actions: the weight at B and the thermal primitive ΔT = 475/2 K on every
member. They select two points of a new invented, non-standard material,
`invented_w11_equal_moduli_two_alphas`. It is not the predecessor's material and does
not use its bases.

| Point | E (Pa) | G (Pa) | ν | α (/K) |
|---|---|---|---|---|
| 250 K (S selects it, as an exact basis reference) | 187 500 000 000 | 75 000 000 000 | 1/4 | 5·2⁻¹⁹ |
| 650 K (O selects it, as an exact basis reference) | 187 500 000 000 | 75 000 000 000 | 1/4 | 7·2⁻¹⁹ |

**Verdicts.** Under reading S the stiffness part refuses W11 (`refused_g2`): the case
basis label and the selection record's selection temperature differ. Under reading R
the stiffness part and the authored classes hold, so this package's policy admits W11
(`admitted`).

**Mechanics.** In the reference calculation both operands share one elastic operator.
Their identical weight cancels. The direct-stiffness difference equals the response to
the uniform free strain 475/524288 on all 84 output rows. Under R the equal authored
actions give an admission verdict even though the calculation has a nonzero difference
from the two selected expansion coefficients. The package records it as a
`state_difference` under `rows`.

Selection of the two coefficients for the thermal primitive is an assumption of the
reference calculation. How the product's resolver records either selection is not
established here, so W11 is marked gate-only (note 7).

**Closed-form check.** Rounded to two decimals, C carries FX = −11 750.96 N,
FY = −5 473.80 N and MZ = −23 153.85 N·m. The exact values in `closed_form_check`
match the three reaction entries.

## Builder's witnesses

| Row | S | O | Verdict | Mechanical | Gate-only |
|---|---|---|---|---|---|
| X1 same stored primitive, other ledger factor | +1024 N at B.UX, ledger factor 1 | the same primitive, ledger factor 3/2 | `refused_g1`: the factors' bits differ | `linear_increment`: Δu_B = 1/(14π) m | yes: not established here whether the exact route accepts a ledger on this document |
| X2 equal state, two reference-configuration identifiers | `constant_alpha_interval` on AB, α = 2⁻¹⁶ /K, 64 K, identifier RC-1 | the same operands, identifier RC-2 | `admitted`: no identifier is compared | `linear_increment`: 0 | yes: not established here whether one document can carry two reference configurations for one member |
| X3 W3 with only the rigid stop's identity changed | rigid line stop `line_stop:B` at B.UX, no motion | a rigid stop at B.UX with another identity, moved +1/1024 m | `refused_restraint` | `state_difference`: u_B(S) = 0, u_B(O) = 1/1024 m, difference 1/1024 m | yes: compares different model-level support definitions |
| X4 zero-law free-length state | fixed-fixed bar; `free_length_state` on AB with a constant zero law, installation 300 K, operating 500 K, no fit | the same bar, no state | `refused_g1`: the state is an action | `linear_increment`: free-strain increment 0, N = 0 | no |
| X5 two seismic recipes, mass basis only | seismic along x, factor 1/4, on AB; mass basis with contents 8 per m | the same recipe with contents 0 per m | `refused_g1`: the recipes' mass bases differ | `none` | yes: which mass inputs exact-route seismic generation reads has not been checked here |

X1 tests ledger-factor identity and X2 tests reference-configuration identifier
exclusion. Each is a target of one of the verifier test's mutants, as X5 is of the
mutant that ignores the mass basis. X3 is the control for the rigid-identity reading
and X4 for the zero-rule reading. The verifier test's seventh mutant drops the
selection kind, label and record from reading S; K1 and W11 reverse under it.

## Limits

The bar witnesses are axial. K1 and W11 use the M1 frame. Seismic generation has one
policy witness (X5) and no mechanical value. Nonlinear restraints and natural-length
changes have policy entries but no witness here. No certificate bound is supplied.
