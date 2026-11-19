# Reference system: per-load and warning references for RS-WIN, RS-EQX, RS-EQY, RS-EQZ

Per-load and warning references for RS-WIN, RS-EQX, RS-EQY and RS-EQZ.

## Relation to the pending-cases package

Use [`reference_system_pending_cases`](../reference_system_pending_cases/README.md) for EQ-6 as restated at
`e494a5925`. This generator compares the four analytic resultants and three pipes-only twins coefficient by
coefficient with constants transcribed from that package. The maintained test checks those constants against the
sibling files. Both use 9.80665 m/s² for seismic gravity. The pending-cases package assumed it, and §8.1 states it.

## Files

| File | Role |
|---|---|
| `generators/rsgl_reference.py` | the generator: Python 3 standard library only, exact rational arithmetic, controlled evaluation of π and √n |
| `staged/reference_system_model_members.json` | an input staged from `fixtures/reference_system/reference_system.json` as committed at `b5569a8215847bedcceac7de7a1b8429a5fde41b` (git blob `8a9729b033e9929a820b4c5b96a040f4458da1df`): its members `nodes`, `pipe_segments`, `components` and `pressure_contract`, copied as they stand, every number with the same exact value. It holds no `load_cases`, `combinations`, `analysis_status` or `diagnostics` |
| `references.json` | the inputs read, sections, members, arcs, component qualification, per case the generated loads, resultant (with the EQ-6 entries checked equal to the pending-cases package), floors, pipes-only twin and model-check record, a pointer for EQ-4, the comparison rule, the open items and 24 cross-checks |
| `SHA256SUMS` | hashes of the four files above, written by the generator |

`python3 -I generators/rsgl_reference.py` rebuilds `references.json` and `SHA256SUMS` in memory from this package's
files alone and compares them byte for byte with the committed files. `--write` rewrites both.
The maintained tests
([`tests/test_reference_system_generated_loads_references.py`](../../../tests/test_reference_system_generated_loads_references.py))
check byte reproduction with package-only inputs, manifest membership and hashes, static imports and selected call
and path literals, eight specification excerpts and the sibling constants. They check
recorded load counts, component encodings against stored values, warning record consistency and resultant floors.

## What the generator reads and does not read

The generator imports only standard-library modules. The arithmetic uses package inputs: the generator, this README
(for the number-presence check) and the staged model members. The staged copy contains only the four selected model
members plus staging metadata. Live mode opens the fixture, parses its JSON and projects the four model members
before comparing them. The generator does not read `self_weight_capture.json`, `authoring.json`,
`key_results.json`, product source, product tests or product output.

The generator transcribes the rest, with its source. From the specification: the case inputs (§8.1), the weight
gravity (§2), the bend corners (§3.1), the member order (§3.2) and the statements it quotes in `governs.quoted`.
From DEL-03-05: the rigid-family kinds. From three sibling packages: the frozen values its cross-checks compare
against.

## The staged input and the live comparison

    python3 -I generators/rsgl_reference.py --check-live

The live check compares typed leaves in the four selected model members, numeric values exactly, unique ID-keyed
list entries by ID, and list lengths. It reports changed compared paths and exits 1 on a mismatch. Excluded
top-level fields, reordering of ID-keyed lists, empty-object additions and changes to numeric spelling or whitespace
can pass. The maintained whole-file SHA-256 check
([`tests/test_reference_packages_live.py`](../../../tests/test_reference_packages_live.py)) detects byte changes
against the registered digest. Review the fixture diff before updating that digest.

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
record gives the analytic value on the entered decimals (`value_N`, `value_N_m`), the formed primitive
(`binary64_by_formation`) with its bits, and whether the scope requires it to be nonzero (k and the corresponding
coordinate both nonzero). The formed component primitives equal their analytic values in these four cases. The required-nonzero moment
components are 3 in RS-EQX, 5 in RS-EQY and 4 in RS-EQZ. The other transverse components are exactly zero. The scope
does not fix the sign of a zero.

**Analytic resultants** (`resultant`). The force is Σ q·L·e plus Σ F. The moment is about the global origin, with each force at its point
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

Each nonzero difference is approximately 4.85 million to 47.26 million times the component tolerance
(`discrimination_at_the_comparison_rule`), so a resultant comparison at the rule below distinguishes the rule from
its absence on this model, in every case. The components that are exact zeros are zero under both, so those
components do not discriminate. They are the force components off the case's axis and the moment component about
it.

## Seismic model-check findings

The expectations rest on four decisions SWBPIPE's Agent 0 made on 2026-11-18 (`seismic_warning_decisions`):
1. The weight finding names entered weight-category loads, including entered distributed member loads, and on the
   pressure-free route names added weight taken from an operating condition.
2. Its exemption covers fresh applied generated self-weight. The reason: seismic generation uses the model's pipe
   mass and the entered component force W directly.
3. Manual overrides are named.
4. The gravity comparison uses fresh applied component-weight primitives.

On each of RS-EQX, RS-EQY and RS-EQZ, no weight finding and no gravity finding are required. This expectation holds
only under the condition recorded in each case's `seismic_model_check.condition`: the case's 29 self-weight
primitives are fresh (the self-weight plan's output, matching the model, as §8.1 states them, none overridden), and
the case holds no other load in the weight category.
- **Weight:** under decisions 1 to 3, fresh generated self-weight is exempt, and nothing else is in the weight
  category. The case is on the exact route, and no case carries condition added weight (§8). The 29 ids follow
  §8.1's rule and are listed in the record.
- **Gravity:** under decision 4, the baseline is the case's four fresh `…:force:z` component-weight primitives,
  for the components whose forces the case generates. Their gravity magnitude is 9.80665 m/s² (§2, and §8.1: W uses
  "gravity as §2"). The seismic gravity is 9.80665 m/s² (§8.1). They are equal after normalization.

RS-WIN carries no seismic model-check finding (AC-007: "none of those findings otherwise").

## EQ-4

EQ-4 is an identity on published rows, so no values are frozen for it. Its operand-row identity is the one
[`../reference_system_pending_cases/README.md`](../reference_system_pending_cases/README.md) states ("EQ-4: SRSS and
absolute sum, an identity").

## Comparison rule

Compare load kind, target and direction or axis exactly. Compare every nonzero component force and moment against
its recorded binary64 bits. For zero transverse moments, either sign is valid if a record is emitted. Compare the
remaining nonzero values at 1e-9 relative alone. For an exact-zero resultant component, use 1e-9 times the largest
absolute component in its force or moment vector.

The relative and zero rule is the one
[`../w16_exact_equivalent_static/README.md`](../w16_exact_equivalent_static/README.md) states. The pending-cases
package leaves tolerances to the consumer. The eight stored floors (`floors`) use binary64 multiplication of the
represented family maximum and the represented 1e-9. For example, RS-EQX's force floor is
`1.4010967471694583e-05`. Rounding the exact analytic product instead gives `1.4010967471694581e-05`.

The bits are required because single rounding of the three normalized operands changes six moment results by one
bit and passes the relative comparison. In RS-EQX, the valve's +234 N·m about Y would be `406d3fffffffffff`; the
required encoding is `406d400000000000`.

## Conventions

The units are m, N, N·m, N/m and kg/m. The origin is global (0, 0, 0). A moment is Σ r × F plus the component
couples. Positive g-factors and `global_x` wind act along the positive global axis. A straight member runs from its
`from` node to its `to` node. An arc is parametrized from C by e₁ cos θ + e₂ sin θ, θ ∈ [0, π/2]. `exact` is a sum of
`(q)*sqrt(n)*pi^k` terms with q rational. `decimal` has 40 significant digits, rounded half even. `value` is the
nearest binary64. Both are emitted only when the two ends of a bounding interval give the same result: π is
bounded by Machin's formula and √n by the integer square root, each within 10⁻¹¹⁰.

## Cross-checks

`references.json` `crosscheck` holds 24 checks, and all agree: five transcription or inventory checks, one
wind-plane geometry check, three algebraically equivalent mass-form checks, three numerical method checks (π,
component products, resultant quadrature), eleven comparisons with sibling records, and one README number-presence
check.

The sibling comparisons consist of three scaled weight wrenches
([`../reference_system_r1_ce/`](../reference_system_r1_ce/README.md)), one corrected tranche-1 weight total
([`../reference_system/`](../reference_system/README.md)), four resultants and three controls
([`../reference_system_pending_cases/`](../reference_system_pending_cases/README.md)). They reuse the mass and
component readings. Agreement between packages checks the resultant effect of their shared readings and different
input sources, with no independent coverage of every field. The π check compares Machin evaluation with AGM. The bit
check agrees on the nonzero products and compares zero magnitudes. The quadrature uses Simpson on straight members
and Romberg on bends. The README check finds numeric strings anywhere in the file.

## Open items

`references.json` `open_items` records these, each with a null value. Warning code, wording, grouping and specific
order are open. These cases freeze the absence of both seismic warning families. Emission of zero transverse moment
records is open. Generated wind and seismic identifier syntax and the specific sequence are open. REQ-009 requires
repeatable contribution and finding order, and REQ-007 requires sorted load IDs in the primitive load-case record.

## What this is not evidence of

The package supplies generated-load and warning expectations for the specified cases. For EQ-6 reaction resultants
use `reference_system_pending_cases`.

- Nothing here states how the product behaves today. Every value comes from the governing text and the model's
  members, not from a product run.
- The four cases do not exist in the fixture yet, and no product test reads this package.
- A resultant constrains the total applied load and its line of action, not its distribution. The per-load values
  constrain the generated-load record, not the solved response.
- Displacements, member forces, stresses and support states are not referenced here.
- The warning expectation holds only under its condition: fresh self-weight primitives and no other load in the
  weight category. It does not establish what the warning does with a primitive that no longer matches its model,
  with an overridden one or with an entered weight load.
- Passing these checks does not make exit criterion 1 or 2 met.
- Agreement is evidence, not acceptance (F-PIP-3). SWBPIPE computes mechanics; it does not determine code compliance
  (F-PIP-2).
