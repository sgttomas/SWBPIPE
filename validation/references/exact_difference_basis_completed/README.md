# Exact-route references: the declared difference, completed admission policy

Independently derived expectations for a result-state subtraction declared
`operating_minus_sustained` on the exact route:
[DEL-05-02](../../../execution/PKG-05/DEL-05-02/ScopeOfWork.md) REQ-012 and AC-003.
This package is the successor to [`exact_difference_basis`](../exact_difference_basis/README.md).
It carries every row of that package and adds rows.

The package contains:

- the admission policy, stated once (below);
- every row the frozen package has (19 gate rows and 17 numerical pairs), re-derived
  under that policy;
- the added rows W1 to W9, F1, F2 and F2b;
- two builder's witnesses, X1 and X2.

Every row carries two entries: an **admission** verdict and a **mechanical** result.

Nothing here comes from the product. The generator uses the Python standard library
only. It imports, reads and runs no product code or output, and it does not read the
frozen package; the frozen inputs are restated in it. All inputs are invented. No
value comes from a design code, catalogue or vendor, and no code compliance is
claimed.

**Status.** Candidate references, derived and cross-checked by their author, awaiting
independent refutation. Before these verdicts and values are frozen, a fresh reader who
did not write them attempts to refute each row's two entries. No product test reads
the package. It wires nothing. The product's blanket refusal of a declared difference
on the exact route stands, and nothing here admits one.

**Two matters as decided at freezing.** Both are built as the specification states
them. Each stands as decided when the package is frozen:

- stored self-weight and component weight are primitives, compared by stored value
  (see "Primitive loads");
- the stiffness part (G-2) compares the selection record as well as the formed values,
  with the over-refusal stated (see "G-2").

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

The package test is `tests/test_exact_difference_basis_completed_references.py`. It
runs the check mode and the sums, confirms the generator names no product path, and
compares every carried row with `../exact_difference_basis/references.json`. Any
reversal of a frozen verdict or change of a frozen value fails it. A separately written
verifier of the policy, `tests/verifiers/exact_difference_basis.py`, reads the same
rows. Its test, `tests/test_exact_difference_basis_verifier.py`, requires the
verifier's verdict on every row to equal the package's. It also requires each of six
wrong policies to reverse at least one row.

**Re-freeze rule** (the frozen package's). No value changes to match a product output.
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

- In every frozen model, every terminal is `transfers_to_wall`. The frozen README
  says so in "Which constraint each model uses", and says that its balance "holds
  because every terminal transfers to the wall".
- The frozen member states share one reference configuration and have no fit.
- The frozen numerical pairs have no generated loads and no prescribed motions.

The generator confirms this: no carried row is reversed (see "Carried rows").

## The admission policy

A declared subtraction S → O is admitted only when all three parts hold. S is the
subtrahend ("sustained") and O the minuend ("operating"). The parts are evaluated in
this order:

1. **Restraint:** O and S have the same restraints and devices.
2. **G-1:** S's actions are contained in O's, by sameness and with multiplicity.
3. **G-2:** S and O have the same modulus basis.

The verdict names the first part that fails: `admitted`, `refused_restraint`,
`refused_g1` or `refused_g2`. Multiplicity means that, for every action, S's count does
not exceed O's. Each row's `gates` records the outcome of all three parts.

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

Stored self-weight and component weight are primitives under this rule. An authoring
plan writes them into the document as stored values, so two plans with different
recipes that write equal stored values apply the same action. Content generated at
solve time, by contrast, is compared by recipe (see "Generated loads"). This is the
first of the two matters as decided at freezing.

### Pressure

A region is the same action when it has the same:

- sorted pipe members;
- sorted connector members;
- sorted terminals as (node, closure transfer): `transfers_to_wall` or
  `separately_supported_or_compensated`;
- pressure (binary64 bits, Pa).

**Elongation** is an action of its own. It is counted once when the case is listed for
elongation **and** carries at least one region. Switching elongation on adds an
action, so such a pair is admitted, as the frozen package has it.

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
  generator reads.

Generated loads are counted with multiplicity, one per marked span or generated
primitive.

**This rule is conservative.** Two recipes that differ are not the same action, even
when they give equal values, and the pair is refused. So a pair refused here may have
an equal mechanical result (W8).

### Restraints, then motions

**Restraint first.**

- Each support DOF has a resolved law in each operand: free, rigid, spring (with its
  identity, participation and stiffness) or nonlinear (with its identity and support
  law).
- The restraint part holds only when every support DOF has the same law, with the
  same identity, participation and parameters, in both operands. A DOF an operand does
  not list is free.
- A free DOF in one operand and a DOF fixed at zero in the other differ (W6).
- A removed or held spring is a different case (W7a), and so is a different spring at
  the same node (W7b).
- The same holds for a nonlinear support's law.

**Then motion increments.**

- Once the same restraint is established in both operands, a motion that compares
  equal to zero is the absence of a motion increment. This holds for either sign, and
  whether the zero is authored explicitly or defaulted (W9).
- A non-zero motion is an action. It is compared by its target (support, DOF and
  field: rigid boundary, nonlinear ground or spring base) and by its value bits in
  canonical units.
- S's non-zero motions must be contained in O's.

**Spring base and nonlinear ground offset.** The same spring, with the same
participation and stiffness, must hold first. A zero base movement in S against a
non-zero one in O is an added excitation, and the pair is admitted (W4). A nonlinear
ground offset is treated the same way, under its support law.

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

**Omission is fail-closed.** A state is not an action only when its source is
established exactly zero. Both of these must hold:

- the thermal part is `unchanged_reference`, a zero strain, a zero coefficient, a zero
  interval, or an installation temperature equal to the operating temperature;
- the fit is none, a zero strain or a zero change.

A total that rounds to zero never omits a state.

### Fixed loads

These loads are present in every case of a model and are not actions:

- constant-effort forces;
- a connector's installed reference;
- a tie rod's reference extension;
- a spring's cold load, where the model applies it.

Each counts 1 − 1 = 0 in the difference. The two operands' fixed-load inventories must
be equal. An unequal inventory is outside this policy, and the row is refused under
restraint, with that reason.

### G-2: the stricter stiffness policy

Two operands have the same basis when every member has equal values of all of these:

- E, ν and G;
- the material and the constitutive basis;
- the selection kind and the case basis label;
- the selection record: its kind, selection temperature, interpolation fraction, and
  selected E and ν;
- the section record.

**The over-refusal is stated.** Suppose two cases have identical formed elastic inputs
and identical thermal-action content, but their selection records differ. The pair is
refused, although its mechanical difference may exist. This is the second of the two
matters as decided at freezing.

## Two entries per row, never merged

**`admission`** has these fields:

- `verdict`;
- `reason`, one sentence;
- `gates`, with each part's `holds` and why;
- `gate_only`, with its `gate_only_reason`.

`gate_only` is true when ordinary exact-route validation would stop the document before
the gate. Such a row tests the policy function only.

**`mechanical`** has a `kind`, a `meaning`, and either `rows` (the carried pairs) or
`values` (the added rows). Each value gives `exact`, `value` (the nearest binary64) and
`unit`. The kinds are:

- `linear_increment`: both operands are on one common linear stiffness and restraint
  operator. The value is the response of that operator to the difference of the two
  load vectors.
- `state_difference`: the operands are on different operators. The value is the
  difference of two independently solved states, with no claim of superposition.
- `none`: no numerical result is defined. This is the case for the carried gate rows,
  for which the frozen package gives none.

**Where the two entries diverge, and why.** A common linear operator gives a response
to the difference of *any* two load vectors. That response alone cannot tell an
authored addition from a removed action, a changed wind extent disguised by nodal
loads (W5), or cancelling actions of different multiplicity. So a `mechanical` entry
can exist while `admission` refuses. The rows diverge for three reasons:

- that response, as above (W5, X1);
- the conservative recipe rule (W8);
- the stricter stiffness policy, which may refuse a pair whose exact mechanical
  difference exists.

On different operators the mechanical entry is a `state_difference` (W6, W7a, W7b and
the mixed-basis pair). It is not an increment of anything.

## Exact form

Each value is a finite sum of rational multiples of integer powers of π, written as in
`3/4*pi + 1/2 + 5/7*pi^-1`. This is the frozen package's convention. Each value's
`value` is the nearest binary64. The rounding is decided from rational bounds on π
about 2⁻⁶³⁰ apart (Gauss's arctangent formula, with a proven error).

## The input format

Each operand is a case side, in the format of the specification's section 4. Units are
canonical (N, m, rad, K, Pa). Numbers are exact rationals written as strings, with the
binary64 bits beside them where the policy compares bits. A side has these fields:

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

A load that is a multiple of π, such as W6's 7π N, has `value` `7*pi` and the bits of
its nearest binary64.

## Carried rows

The frozen inputs are restated in the generator. The README of the frozen package gives
them under "Inputs" and "Which constraint each model uses".

**What the restated cases represent.**

- Every frozen terminal is `transfers_to_wall`.
- Each anchor's six DOFs are rigid.
- The modulus basis of each member is either:
  - `REF`: the exact basis reference to the 300 K point;
  - `HOT`: a solve temperature of 600 K, interpolated at the fraction 3/4.
- The frozen authored member state is a `free_length_state`, operating at 600 K, or at
  the next binary64 above 600 K. It uses the material's table law and has no fit. Its
  installation temperature is resolved from the one shared reference configuration as
  300 K. That value is invented here, and it is the same in every carried case.
- The frozen preload is a constant-effort fixed load, the same in both operands.

**The verdict mapping.** Each carried row's verdict equals the frozen `outcome`, mapped
as follows:

| Frozen `outcome` | Completed verdict |
|---|---|
| `published` | `admitted` |
| `refused_g1` | `refused_g1` |
| `refused_g2` | `refused_g2` |
| `refused_g1_and_g2` | `refused_g1` (G-1 is ordered first) |

**Carried mechanical values.** Each carried pair has `rows`: every row of the frozen
pair's values field, re-derived by direct stiffness with exact arithmetic. The values
field is:

- `difference` for an admitted pair;
- `undeclared_subtraction` for a pair refused under G-1;
- `withheld_would_be_difference` for the pair refused under G-2.

Each row gives `exact`, `value` and `unit`. It does not carry the frozen
`from_published_operands`, which is a product publication rule and stays in the frozen
package.

The pairs on one basis are `linear_increment`. The mixed-basis pair is
`state_difference`. Reading J is carried as the frozen package marks it:

- the rows where it differs analytically from reading W, under
  `reading_J_alternative`, for the six pressure-change pairs;
- the statement that it is analytically identical, for the other same-basis pairs.

**What the generator checks.**

- Equilibrium of every case (66) and every difference (17).
- The wall form of DEL-04-07's pressure loads equals the closed-end free strain on
  every pressurised case with elongation on (19).
- The M1 hand formula, on both bases.

The package test then compares all 1 806 rows and the 531 reading-J rows with the
frozen file. Every value is equal and no verdict is reversed.

`gate_only` is false on every carried row. The frozen package states each case as an
exact-route document. Whether validation would stop one before the gate is not
re-examined here, because that would need product code.

## Added rows

**Shared inputs for W1 to W9.**

- One straight member AB along +x, with L = 1 m and A at the origin.
- A is anchored: all six DOFs are rigid.
- Outside diameter 1/4 m, wall 1/32 m, no allowances. So r_i = 3/32 m and the wall
  area is 7π/1024 m².
- E = 2²⁰ Pa and ν = 1/4, so EA = 7168π N.
- Axial response only.

The solutions use an exact axial bar: rigid supports, prescribed motions, springs with
a base, and consistent nodal loads. Each value is checked against its closed form.

In the table, "verdict" is the admission verdict and "mechanical" is the result
(O − S unless the row says otherwise).

| Row | S | O | Verdict | Mechanical | Gate-only |
|---|---|---|---|---|---|
| W1 elongation added | region AB, both terminals `transfers_to_wall`, p = 1024 Pa, elongation off | the same, elongation on | `admitted`: elongation is an added action | `linear_increment`: Δu_B = 9/14336 m | no |
| W2 shared-terminal grouping | AB and BC collinear (C at x = 2 m), ν_AB = −1/8, ν_BC = 1/4, elongation on; region AB with terminals A and B | that region, plus region BC with terminals B and C, at the same pressure | `admitted`: BC's region is an added action | `linear_increment`: Δu_B = 0, Δu_C = 9/14336 m; N_mech of BC in O = 9π/2 N = ½·p·π·r_i² | no |
| W3 operating-only rigid motion | rigid line stop at B.UX, no motion | the same stop, B.UX moved +1/1024 m | `admitted`: the same restraint; S's zero is no increment | `linear_increment`: Δu_B = 1/1024 m, N = 7π N, reaction at A −7π N | no |
| W4 operating-only spring base | spring K at B.UX, k = EA/L, base unmoved | the same spring, base moved +1/1024 m | `admitted`: the same spring; an added excitation | `linear_increment`: Δu_B = 1/2048 m; spring force on the pipe 7π/2 N | no |
| W5 wind extent substitution | wind along +x, 32 Pa, shape factor 1, exposed diameter 1/4 m (q = 8 N/m), span [0, 1/2] | wind on [1/2, 1], plus nodal +3 N at A.UX and +1 N at B.UX | `refused_g1`: S's recipe is not in O; same-bit nodal forces are not the same action | `linear_increment`: EA·Δu_B = 3 N·m (the nodal forces alone give 1 N·m); consistent A/B forces (3, 1) N in S and (1, 3) N for O's wind | yes ¹ |
| W6 free DOF against one fixed at zero | B.UX free; +7π N at B | B.UX rigid, zero motion; the same load | `refused_restraint` | `state_difference`: u_B(S) = 1/1024 m, u_B(O) = 0, difference −1/1024 m | yes ² |
| W7a removed spring | spring K at B.UX, k = EA/L; +7π N at B | no spring | `refused_restraint` | `state_difference`: u_B(S) = 1/2048 m, u_B(O) = 1/1024 m, difference +1/2048 m | yes ² |
| W7b different spring at the same node | spring K1, k = EA/L; +7π N at B | spring K2, the same k; the same load | `refused_restraint`: another identity is another case | `state_difference`: 0 m; the two states are equal ³ | yes ² |
| W8 two recipes, equal values | whole-span wind 0.1 Pa (its binary64), shape factor 10, D = 1/4 m | whole-span wind 1 Pa, shape factor 1 | `refused_g1`: the recipes differ (conservative) | `linear_increment` of the formed loads: 0 (binary64 intensity 1/4 N/m in both, in either order of multiplication); the exact recipe difference at B is −2⁻⁵⁷ N | yes ¹ |
| W9 explicit zero against default zero | line stop at B.UX with authored motion −0.0 m | the same stop, no authored motion | `admitted`: a zero of either sign normalizes with the default | `linear_increment`: 0 | no |
| F1 terminal witness | pressurised cantilever, free-end terminal `transfers_to_wall` | the same, free end `separately_supported_or_compensated` | `refused_g1` | `linear_increment`: Δu_B = −p·A_i·L/(E·A) = −64607/81065856 m ⁴ | no |
| F2 fit witness | free-length state: datum and installation 300 K, operating 500 K, α = 2⁻¹⁶ /K, fit strain 2⁻¹⁰ | the same, no fit | `refused_g1` | `linear_increment`: strain increment −8217/8388608; on a fixed-fixed bar of the W section, N = 57519π/8192 N | no |
| F2b installation-only variant | as F2's O: installation 300 K, no fit | installation 400 K, no fit | `refused_g1` | `linear_increment`: strain increment −205425/134422528; N = 1437975π/131272 N on the same bar ⁵ | no |

1. Whether exact-route wind generation admits a load along the member axis is not
   established here, because establishing it would need product code. The row
   therefore tests the policy function only.
2. Supports are model-level on the exact route. A per-case free or fixed DOF, or a
   per-case spring, cannot be authored.
3. The specification gives no load for W7b. The common +7π N of W7a is used, and the
   difference is zero with or without it.
4. F1 uses the frozen package's section, E = 194 750 000 000 Pa and p = 5 750 000 Pa,
   with L = 15/4 m. It is solved with elongation on in both operands. The increment
   depends on neither the setting nor ν.
5. The specification gives F2b's value and "installation 400 K in O". The stated value
   is the increment with no fit in either operand, so that is how F2b is built. The
   secant strain of F2 is
   (1 + α·(T_op − T_inst)/(1 + α·(T_inst − T_datum)))·(1 + fit) − 1.

W2's formation note in the specification (a grouped binary64 force at B that differs
by −2⁻⁴⁹ N from the separately formed extra region) concerns the order in which a
product forms binary64 values. It is not derived here and is not an entry.

## Builder's witnesses

The six wrong policies of the verifier test must each reverse a row. The added rows
contain no ledger factor and no second reference-configuration identifier, so the
builder added two rows on the W bar. Both follow from the policy as written.

| Row | S | O | Verdict | Mechanical | Gate-only |
|---|---|---|---|---|---|
| X1 same stored primitive, other ledger factor | +1024 N at B.UX, ledger factor 1 | the same primitive, ledger factor 3/2 | `refused_g1`: the factors' bits differ | `linear_increment`: Δu_B = 1/(14π) m | yes: not established here whether the exact route accepts a ledger on this document |
| X2 equal state, two reference-configuration identifiers | `constant_alpha_interval` on AB, α = 2⁻¹⁶ /K, 64 K, identifier RC-1 | the same operands, identifier RC-2 | `admitted`: no identifier is compared | `linear_increment`: 0 | yes: not established here whether one document can carry two reference configurations for one member |

## What this package does not prove

- It says nothing about what any solve consumes.
- It says nothing about a consumed-term check, an evidence record, or how any reader
  is wired.
- Its mechanical results are exact references, not product outputs.
- It admits no declared difference on the exact route.

## Limits

The limits of the frozen package's models carry over. They are linear Euler–Bernoulli
models without bends, and no certificate bound is given. The added rows are axial
only. The policy's seismic, nonlinear and natural-length-change paths are stated but
not exercised by any row.
