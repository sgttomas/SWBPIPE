# Reference system: per-load and warning references for RS-WIN, RS-EQX, RS-EQY, RS-EQZ

Per-load and warning references for the wind case RS-WIN and the static seismic cases RS-EQX, RS-EQY and RS-EQZ
of the invented reference system
([`fixtures/reference_system/SPECIFICATION.md`](../../../fixtures/reference_system/SPECIFICATION.md) §8.1). They
supplement [`../reference_system_pending_cases/`](../reference_system_pending_cases/README.md), which remains the
reference for EQ-6 (§9.2). For each case they give every generated load one by one. For each component force and
moment they give the binary64 bits the formation rule fixes. For each seismic case they also give the seismic
model-check findings the case must carry. They follow the generation rule of
[DEL-05-01](../../../execution/PKG-05/DEL-05-01/ScopeOfWork.md) ("Static-equivalent generation", "Mass basis",
REQ-008, REQ-015, AC-007). They were frozen before the engine work they judge, and no product test reads this
package yet.

Status: candidate per-load and warning references, derived and cross-checked by their author, not yet independently
refuted.

## Relation to the pending-cases package

**Values both packages hold.** Both hold the four resultants (force and moment about the origin of RS-WIN, RS-EQX,
RS-EQY and RS-EQZ) and the three pipes-only twins. That package holds the twins as its control "the rule before
2026-11-12". This generator checks every one of them equal to that package's frozen value, rational coefficient by
coefficient (`crosscheck`), so the two cannot part silently. The pending-cases package is the reference for EQ-6.
This package does not replace that reference or restate it as its own: its `EQ-6` entries name that package as the
reference of record and are values checked equal to it.

**What only this package holds:**
- each generated load one by one, with its target, kind, direction or axis and its intensity or value;
- the formed binary64 bits of each component force and moment;
- the comparison rule and each case's floors;
- the twin's difference measured against that rule;
- the seismic warning expectation;
- inputs read from the fixture's model members, staged here, where that package holds the specification's inputs
  as constants.

**Stated assumptions.** That package's README treats the seismic gravity magnitude as an open reading (OR-G) and
assumes 9.80665 m/s². This package takes 9.80665 m/s² from §8.1, which now states it. No other stated assumption
differs.

## Files

| File | Role |
|---|---|
| `generators/rsgl_reference.py` | the generator: Python 3 standard library only, exact rational arithmetic, controlled evaluation of π and √n |
| `staged/reference_system_model_members.json` | an input staged from `fixtures/reference_system/reference_system.json` as committed at `b5569a8215847bedcceac7de7a1b8429a5fde41b` (git blob `8a9729b033e9929a820b4c5b96a040f4458da1df`): its members `nodes`, `pipe_segments`, `components` and `pressure_contract`, copied as they stand, every number with the same exact value. It holds no `load_cases`, `combinations`, `analysis_status` or `diagnostics` |
| `references.json` | the inputs read, sections, members, arcs, component qualification, per case the generated loads, resultant (with the EQ-6 entries checked equal to the pending-cases package), floors, pipes-only twin and model-check record, EQ-4's statement, the comparison rule, the open items and 24 cross-checks |
| `SHA256SUMS` | hashes of the four files above, written by the generator |

`python3 -I generators/rsgl_reference.py` rebuilds `references.json` and `SHA256SUMS` in memory from this package's
files alone and compares them byte for byte with the committed files. `--write` rewrites both.
[`tests/test_reference_system_generated_loads_references.py`](../../../tests/test_reference_system_generated_loads_references.py)
runs that check. It also checks the manifest, the generator's imports, the transcribed governing text and sibling
values, the comparison rule and the floors.

## What the generator reads and does not read

The default check and `--write` read only this package: the generator, this README (for the quotation check) and
the staged model members. They open no file outside the package. The staged file holds the four model members the
derivation needs and nothing else. The captured self-weight primitives, which are in the fixture's `load_cases`,
were never staged. The generator does not read `self_weight_capture.json`, `authoring.json`, `key_results.json`,
product source, product tests or product output.

The generator transcribes the rest, with its source. From the specification: the case inputs (§8.1), the weight
gravity (§2), the bend corners (§3.1), the member order (§3.2) and the statements it quotes in `governs.quoted`.
From DEL-03-05: the rigid-family kinds. From three sibling packages: the frozen values its cross-checks compare
against. The maintained test checks every quoted statement against the current specification and every sibling
value against the committed file it names.

## The staged input and the live comparison

    python3 -I generators/rsgl_reference.py --check-live

This separate check is not run by the default check. It opens the live fixture, keeps the same four members, and
compares them with the staged copy leaf by leaf: numbers by exact value, records in a list by their id, and each
list by its length. It names every leaf that differs and exits nonzero ("SOURCE MOVED ON"). Its result does not
change the committed output, which still stands for its staged input.

[`tests/test_reference_packages_live.py`](../../../tests/test_reference_packages_live.py) runs it. That test also
holds the digest of the fixture's whole bytes, because the leaf comparison cannot see everything:
- a change outside the four members, such as a load case added, a primitive load edited or a member added beside
  them, passes the leaf comparison;
- a change the product reads differently from how it reads here, such as a new member a solve gives precedence,
  also passes the leaf comparison.

Any change to the file fails the digest until a person has looked at its diff against this package.

When the fixture's geometry, sections or components change, the answer is a successor package or a regeneration,
decided at that time. It is not an edit of this package in place.

## Mass bases

The reference system is on the exact route (§2: `exact_pressure_v3`). Its mass basis is
`source_od_nominal_wall_pressure_bore_areas/v1` (DEL-05-01 "Mass basis"). The constituents are those of DEL-03-08's
"Mass per length", taken on the sections DEL-05-03's section bases give. As
[`../reference_system/README.md`](../reference_system/README.md) ("Mass bases") and
[`../w16_exact_equivalent_static/README.md`](../w16_exact_equivalent_static/README.md) state them: metal on the
nominal wall, contents on the bore reduced by mill tolerance and corrosion allowance, and insulation from OD to
OD + 2t_i. With R = D/2:

    μ = π [ ρ_m (R² − (R − t)²) + ρ_c (R − ((t − c_m) − c_a))² + ρ_i ((R + t_i)² − R²) ]   (kg/m)

| Section | Metal | Contents (820) | Insulation (140) | μ (kg/m) | D + 2t_i (m) |
|---|---|---|---|---|---|
| RS-S270 | 57.92987 | 42.53731 | 8.708495 | 109.17567 | 0.39 |
| RS-S220 | 41.82591 | 28.07791 | 7.389026 | 77.29284 | 0.34 |
| RS-S170 | 28.13877 | 16.61096 | 4.838053 | 49.58779 | 0.27 |

The pressure-free route's basis is not referenced here, because the reference system is specified on the exact
route only.

## Derivation

**Wind** (DEL-05-01, "Wind"; §8.1). On each marked span, over the marked fraction, the load per length is
q = p·C_s·(D + 2t_i), with p = 900 Pa, C_s = 0.7, along +X. The marked members are RS-P070 … RS-P200 in §3.2's order
except the replaced span RS-P180: 14 members, each over its whole length (fraction 0 to 1). That gives 245.7 N/m on
S270 and 214.2 N/m on S220. RS-P150 takes its own section, S270. Every marked member, arc RS-P085 included, lies in
the plane X = 6 m, so the wind is normal to each member along its whole length.

**Seismic** (DEL-05-01 "Static-equivalent generation", REQ-015, AC-007; §8.1). Each case enters g = 9.80665 m/s² and
one positive factor: k_x = 0.30 (RS-EQX), k_y = 0.30 (RS-EQY) or k_z = 0.20 (RS-EQZ). Positive loads along the
positive axis ("The factor's sign is the direction's sign"). RS-P180, the span the objective connector RS-EJ1
replaces, carries no seismic load. The generator applies REQ-015's ordered rules to every rigid-family component
(DEL-03-05: valve, flange, reducer, rigid, specialty) and records each condition's outcome in
`component_qualification`. RS-F1, RS-V1, RS-F2 and RS-R1 pass every condition. On their bodies RS-P020, RS-P030,
RS-P040 and RS-P150 the only seismic load is then the force F = k·W along the axis at the component's node, with
the couple r × F about that node; r is the entered global centre-of-gravity offset. Every other member carries
q = k·g·μ along the axis: 21 members. The bends and the tee are not rigid-family and keep their members' loads.

**Component force and moment bits.** The scope fixes the binary64 formation: k, W and r are each normalized to
binary64, F = RN(k·W), and each transverse moment component is RN(±r_c·F), with r_c the corresponding coordinate of
r, rounded to nearest with ties to even. For F along X the components are M_y = RN(r_z·F) and M_z = −RN(r_y·F).
For Y they are M_x = −RN(r_z·F) and M_z = RN(r_x·F). For Z they are M_x = RN(r_y·F) and M_y = −RN(r_x·F). Each load
record gives the exact value on the entered decimals, the formed binary64 with its bits, and whether the scope
requires it to be nonzero (k and the corresponding coordinate both nonzero). The required-nonzero moment
components are 3 in RS-EQX, 5 in RS-EQY and 4 in RS-EQZ. The other transverse components are exactly zero. The scope
does not fix the sign of a zero.

**Resultants.** The force is Σ q·L·e plus Σ F. The moment is about the global origin, with each force at its point
of action: the moment of a distributed load is (∫ r ds) × q·e, and the moment of a component force with its couple
is (node + r) × F. For a straight member ∫ r ds = L·(A + B)/2. A 90° arc with centre C = A + B − corner, and e₁, e₂
the radius vectors from C to its from-node and to-node, has length R·π/2 and ∫ r ds = R[(π/2)C + e₁ + e₂]. EQ-6
expects the reaction resultant of RS-dWIN, RS-dEQX, RS-dEQY or RS-dEQZ, force and moment about the origin, to be
minus the resultant of its case. The pending-cases package is its reference, and `cases.<case>.EQ-6` holds the
same values, checked equal to it.

| Case | Force (N) | Moment about the origin (N·m) |
|---|---|---|
| RS-WIN | (8274.00962903341, 0, 0) | (0, 54013.28236095945, -99699.62081912628) |
| RS-EQX | (14010.967471694581, 0, 0) | (0, 73835.75668234036, -132703.4004272364) |
| RS-EQY | (0, 14010.967471694581, 0) | (-73835.75668234036, 0, 74963.32462992267) |
| RS-EQZ | (0, 0, 9340.64498112972) | (88468.93361815759, -49975.54975328178, 0) |

**Pipes-only twin.** The twin is the same seismic case with every non-replaced member carrying k·g·μ and no
component force. The table gives the rule's resultant minus the twin's:

| Case | Force difference (N) | Moment difference (N·m) |
|---|---|---|
| RS-EQX | (662.1968, 0, 0) | (0, 664.3105, 782.3136) |
| RS-EQY | (0, 662.1968, 0) | (−664.3105, 0, 363.4160) |
| RS-EQZ | (0, 0, 441.4645) | (−521.5424, −242.2773, 0) |

Each nonzero difference is between 4.8 × 10⁶ and 4.7 × 10⁷ times the tolerance of its component
(`discrimination_at_the_comparison_rule`), so a resultant comparison at the rule below distinguishes the rule from
its absence on this model, in every case. The components that are exact zeros are zero under both, so those
components do not discriminate. They are the force components off the case's axis and the moment component about
it.

## Seismic model-check findings

The expectations rest on four decisions SWBPIPE's Agent 0 made on 2026-11-18 (`seismic_warning_decisions`). The
scope's sentences are to be clarified in a later change.
1. The warning names the two kinds of weight REQ-008 states: loads in the weight category and, on the pressure-free
   route, the added weight a case takes from an operating condition. It does not name applied generated
   self-weight. The reason: the warning tells the engineer which weights seismic generation did not scale, and
   generation takes the pipe's mass and the component's W directly.
2. A manual override of generated self-weight is named in the warning.
3. The gravity comparison reads fresh applied component-weight primitives only. An overridden component contributes
   no gravity to it.
4. The weight category is the solver's parsed category, an entered distributed member load included. The text says
   "load in the weight category".

On each of RS-EQX, RS-EQY and RS-EQZ, no weight finding and no gravity finding are required. This expectation holds
only under the condition recorded in each case's `seismic_model_check.condition`: the case's 29 self-weight
primitives are fresh (the self-weight plan's output, matching the model, as §8.1 states them, none overridden), and
the case holds no other load in the weight category.
- **Weight:** under decisions 1, 2 and 4, fresh generated self-weight is not named, and nothing else is in the
  weight category. The case is on the exact route, and no case carries condition added weight (§8). The 29 ids
  follow §8.1's rule and are listed in the record.
- **Gravity:** under decision 3, the baseline is the case's four fresh `…:force:z` component-weight primitives,
  for the components whose forces the case generates. Their gravity magnitude is 9.80665 m/s² (§2, and §8.1: W uses
  "gravity as §2"). The seismic gravity is 9.80665 m/s² (§8.1). They are equal after normalization.

RS-WIN carries no seismic model-check finding (AC-007: "none of those findings otherwise").

Two readings were considered and excluded. Naming the case's own applied self-weight is excluded by decision 1.
Naming every weight-category load in the document is excluded on the assigner's reading of REQ-008's "on each
seismic case" as that case's loads.

## EQ-4

EQ-4 is an identity on published rows, so no values are frozen for it. On each signed component row of RS-EQ-SRSS
and RS-EQ-ABS, take x_X, x_Y and x_Z as the matching rows of RS-dEQX, RS-dEQY and RS-dEQZ as published (factors 1). A
test recomputes ABS = RN(|x_X| + |x_Y| + |x_Z|), the exact sum rounded once, and
SRSS = RN(√(x_X² + x_Y² + x_Z²)), the exact square root of the exact sum rounded once (DEL-05-02 "Absolute sum",
"SRSS"). It then checks SRSS ≤ ABS. Magnitude rows follow the combination's own recomputation rule, and withheld
rows are excluded (§9.2).

## Comparison rule

As the sibling packages state it, with the 1e-9 of DEL-05-01 AC-001:

- a nonzero expected value passes when |observed − expected| ≤ 1e-9·|expected|, relative alone;
- an exact-zero expected value passes when |observed| ≤ 1e-9·zero_scale, where zero_scale is the largest magnitude
  of its family (force or moment) in the item. Each case's `floors` records zero_scale and the floor;
- discrete outcomes compare exactly. A component force or moment whose bits the scope's formation rule fixes may be
  compared bit for bit, except for the sign of a zero.

## Conventions

The units are m, N, N·m, N/m and kg/m. The origin is global (0, 0, 0). A moment is Σ r × F plus the component
couples. Positive g-factors and `global_x` wind act along the positive global axis. A straight member runs from its
`from` node to its `to` node. An arc is parametrized from C by e₁ cos θ + e₂ sin θ, θ ∈ [0, π/2]. `exact` is a sum of
`(q)*sqrt(n)*pi^k` terms with q rational. `decimal` has 40 significant digits, rounded half even. `value` is the
nearest binary64. Both are emitted only when the two ends of a bounding interval give the same result: π is
bounded by Machin's formula and √n by the integer square root, each within 10⁻¹¹⁰.

## Cross-checks

`references.json` `crosscheck` holds 24 cross-checks, each with what it compares and whether it is a distinct
calculation, a transcription check or a consistency check. All agree:

- **transcription (5):** the model's member order equals §3.2's. The replaced span is RS-P180. The marked set
  (RS-P070 … RS-P200 less the connector's span) equals the 14 members listed from §8.1. The admitted components are
  §5's four, and the self-weight inventory is 21 + 2 × 4 = 29.
- **distinct calculations here (7):** every exposed member, the arc's centre and radius vectors included, lies in
  X = 6 m. Each section's μ in radius form equals
  the diameter form (3). π from Machin's bounds equals a Gauss–Legendre AGM value. Each component force and moment
  rounded from the exact rational product (the RN written here) has the same bits as the hardware binary64 product
  of the same operands. Every resultant, the twins' included, is recomputed by quadrature along each member and arc.
  That second route uses the AGM π, an arc centre built from the incoming tangent (C = B − R·t_in), a trigonometric
  parametrization with Romberg integration in 80-digit decimals, diameter-form masses, and each component force at
  node + r. Its largest relative difference is 8e-58 of its family's largest magnitude.
- **distinct calculations by other authors (11):**
  - RS-EQX, RS-EQY and RS-EQZ against
    [`../reference_system_r1_ce/`](../reference_system_r1_ce/README.md)'s frozen weight total W and first moment S:
    the force is k·W·e and the moment k·(S × e), and for Z also −k times its weight moment, all exactly. The mapping
    is exact for three reasons: the seismic gravity equals the weight gravity's magnitude, the same members carry
    distributed load on the same basis, and each component's W is held fixed at the same centre of gravity.
  - RS-EQZ against the tranche-1 [`../reference_system/`](../reference_system/README.md) weight total. That match is
    exact only after RS-R1's substitution (+300 N − g·μ_S270·√257/40), because tranche 1 has no RS-R1.
  - All four resultants against
    [`../reference_system_pending_cases/`](../reference_system_pending_cases/README.md)'s full-system values, which
    assumed the seismic gravity 9.80665 m/s² before §8.1 stated it.
  - The three twins against that package's control "the rule before 2026-11-12".
- **consistency (1):** this README quotes every nonzero resultant value.

The sibling comparisons are exact equalities of the rational coefficients. They share the mass-basis reading with
this package, so they do not check that reading. The pending-cases package takes its inputs from the
specification's text, and this package from the fixture's model members, so agreement also checks the fixture's
transcription of those inputs.

## Open items

`references.json` `open_items` records each of these with a null value:

- the code and words of the seismic findings. No finding is required on these cases, so this matters only for a
  test that asserts absence;
- one finding per load or per case, and their order;
- whether an exactly zero transverse moment component becomes a load record. The values and the required-nonzero set
  are frozen, and the number of moment records is not;
- generated-load identities and their order. No rule is given for them, so loads are keyed by target, kind and axis.

## What this is not evidence of

- Nothing here states how the product behaves today. Every value comes from the governing text and the model's
  members, not from a product run.
- The four cases do not exist in the fixture yet, and no product test reads this package.
- A resultant constrains the total applied load and its line of action, not its distribution. The per-load values
  constrain the generated-load record, not the solved response.
- Displacements, member forces, stresses and support states are not referenced here.
- The warning expectation holds only under its condition: fresh self-weight primitives and no other load in the
  weight category. It does not establish what the warning does with a primitive that no longer matches its model,
  with an overridden one or with an entered weight load.
- The resultants are not a second reference for EQ-6; the pending-cases package is.
- Passing these checks does not make exit criterion 1 or 2 met.
- Agreement is evidence, not acceptance (F-PIP-3). SWBPIPE computes mechanics; it does not determine code compliance
  (F-PIP-2).
