# Rigid and semi-rigid components: DEC-045 stiffness scaling

This design realizes FR-SOL-004 and FR-SOL-012 for valves, flanges, reducers,
rigid and specialty components. The basis is DEC-045 Option C: no new element
type; the component's mapped frame member is solved with its stiffness scaled
by the user's factor. OD-2 (owner, 2026-10-23, "I approve OD-1 through OD-5 as
you recommended") selected it. It is W-15 in `docs/COMPLETENESS_MAP.md` §4.3.

Paths below are relative to `projects/chirality-piping/`. PP is
`core/product_physics/src`, FK is `core/solver/frame_kernel/src`.

## 1. The model a user authors

A rigid-family component (`kind` valve, flange, reducer, rigid or specialty)
sits at `node` and maps one generic straight member, `geometry.rigid_pipe_ref`,
that ends at that node. The member is the component's body. Its scale is
`modifiers.stiffness_scaling_user_value` (dimensionless, finite, > 0) with
`modifiers.source_reference`. A scale above 1 stiffens the body against the
mapped pipe section; a scale below 1 softens it (semi-rigid). The software
supplies no scale, weight, centre of gravity or stiffness value.

`linear_stiffness_user_value` and `rotational_stiffness_user_value` remain
recorded provenance quantities. Under DEC-045 one uniform scale realizes the
component (a directional stiffness set is the expansion joint's macro-element,
DEL-03-06), so the solve does not read them. They become optional (PRD §12.3
"optional stiffness values"): a missing value no longer raises
`RIGID_COMPONENT_STIFFNESS_INPUT_MISSING`; a supplied value is still checked
finite and positive, and an info diagnostic states that it is recorded and not
applied.

## 2. Stiffness scaling (slice a)

### 2.1 Meaning

For a mapped member with scale s, the element stiffness is s·K, where K is the
member's 12×12 frame stiffness from its pipe section. Free strains keep their
meaning:

| Effect | Meaning under scale s |
|---|---|
| Element stiffness | s·K (axial, both bendings, torsion; Timoshenko shear) |
| Free thermal strain αΔT | unchanged: a stiffened body still expands with temperature, so its equivalent load is s·E·A·αΔT and its free growth αΔT·L |
| Pressure axial strain (straight member, EX) | an elastic response, so s times smaller: the Poisson contraction −2νpAᵢ/(s·E·Aₙ), and with closed ends the free total (1−2ν)pAᵢ/(s·E·Aₙ); multiply by L for the length change. The load pair ±2νpAᵢ (PP `pressure_exact/source_geometry.rs:314-323`) has no E and is unchanged; the recovery Nw = Nm + 2νpAᵢ (`:349-379`) is unchanged |
| Member end forces and section actions | from s·K·Δu minus the scaled fixed-end loads; equilibrium is unchanged |
| Stress rows | recovered from the forces on the mapped pipe section, which is a reference section, not the body's (§2.4) |
| Reported material E, G, ν | the material's, unscaled |

### 2.2 Where it enters

Both routes build members in one function, `build_model_for_members` (PP
`lib.rs` ~7948; PF and EX callers at ~2735, ~2852). The scale enters there as
E·s and G·s when the member's `StraightPipeSectionProperties` is formed (~8126).
Every assembly (sparse, dense), the force-scale census and the certified frame
binding then see s·K. Phi = 12EI/(κGAL²) is unchanged in exact arithmetic; with
binary64, the products sE and sG are each rounded once.

Two thermal sites re-read E from the material instead of the section. They must
read the member's section E so the free thermal growth stays αΔT·L, and so the
certified re-formation's `FreeStrainStraight` E·A·ε (FK
`structural/formation_check.rs` ~2244-2262) agrees with the ledger's:
- the element thermal load (PP `lib.rs` ~11647, ledger push ~11686-11702);
- the load-state thermal load from the member pair (~3923).

Every other later reader is consistent without change. The stress recovery uses
the geometric `DerivedSection` (~12662). The reported E is the material's or
the pair's (~6665, ~3935, ~10809). The PF Poisson ratio is read back from
the section as E/(2G)−1 (~6485 → `u4b_rows.rs:573-575`). A PF material need
not supply ν (the bundled demo gives E and G only), and the independent reader
recomputes ν from the selected unscaled E/G and checks the shear rows bit for
bit (`core/reporting/result_export/src/u4b_evidence.rs:530-553`). The PF ν
of a scaled member is therefore derived from the selected unscaled E/G pair
(material, named or interpolated temperature basis), never from the scaled
section; EX keeps its selected E/ν. The load-state function's fit eigenstrain
uses the same section E as its thermal term.

No FK file changes. The certified binding (FK `formation_check.rs`
~1220-1376) binds each straight load primitive to its member's frame and does
not require frame E to equal material E; thermal re-formation uses that frame's
E (~2244-2262), and pressure primitives take their frame from `built.pipes`
(`pressure_runtime.rs` ~1274).

### 2.3 Which members are scaled

One eligibility predicate decides both formation and the diagnostics. A
component's mapped member is scaled when all of these hold:
- the component is rigid-family;
- `mechanics_interface.solver_consumption` is `mechanics_geometry_only`, the
  DEC-045 mode for this family;
- the mapping is valid (§2.6);
- the scale is present, finite and positive, with a nonblank
  `modifiers.source_reference`;
- every supplied linear or rotational stiffness is finite and positive.

Otherwise the member solves as unscaled pipe, and one warning names the first
unmet condition (§2.6), in the order listed. An absent consumption declaration
gets its own warning when the component carries a scale, since no mode is
assumed; without a scale, the first unmet later condition is named instead
(normally the missing scale). An absent or blank `rigid_pipe_ref` is a missing
mapped member, so the mapping warning is named beside the geometry warning.
Within the scale condition, presence (scale and source reference) is checked
before validity. `not_solver_consumed`, like any other declared mode, is
honoured: the member solves as pipe with no rigid warning beyond the existing
unsupported-interface diagnostics (PP `validation.rs` ~1227-1253). The
predicate is PP `rigid_component.rs`, read by formation (`lib.rs`
`build_model_for_members`) and by `validation.rs` alike. Three
mappings that today pass silently are refused (blocking, by name), counting
every rigid-family component that maps a member, whether or not it is eligible
(a `not_solver_consumed` record sharing a member with an active one is a
conflict):
- the mapped member is a realized curved bend (PP `lib.rs` ~8171);
- the mapped member is a span replaced by an objective connector (~7982, ~8173);
- two components map the same member.

### 2.4 Stress rows on a scaled member

The rows remain published with their values: they are the stresses of the
mapped pipe section under the body's actual forces. They are not a stress of
the valve or flange body, and the product supplies no body stress. Where the
body joins a single pipe with no external load at the joint, nodal equilibrium
makes its end forces equal that pipe's. At a branch or a loaded junction only
the nodal sum balances.

Before W-15 the demo valve's member already published ordinary pipe stress
rows. In a statically determinate chain the body's forces are fixed by
equilibrium alone, so scaling does not change them. In an indeterminate system
scaling redistributes force onto the stiffer body. Take two collinear members
of equal E·A meeting at a loaded node, each anchored at its far end, with the
body twice the pipe's length. At s = 1 the pipe carries 2F/3 and governs; at
s = 4 the body carries 2F/3. Scaling can therefore newly move the headline, a
headline-bound rule and the headline-derived local-FEA region onto a reference
section. Slice a ships only with slice d's identification (§5), in one
implementation. An info diagnostic per scaled component,
`RIGID_COMPONENT_STIFFNESS_SCALING_APPLIED`, names the member and the scale.
It is stated only on a published result whose member was formed with the
scale; a refused or blocked model states no applied scale.

### 2.5 Numerical bound

The product accepts any finite positive s; there is no engineering bound. A
large s does not by itself lose accuracy: a grounded scaled member recovers its
load accurately. Loss arises when the body's small deformation is recovered
from much larger nodal motions, or when large elastic and eigenload terms
nearly cancel. From s ≥ 1e5 (`PRECISION_WARNING_SCALE`,
`rigid_component.rs`), a warning, `RIGID_COMPONENT_SCALE_PRECISION`, states
that the scaled member's forces may carry reduced precision. The threshold is
measured: it is the lowest decade at which the W-15 rigid-link benchmark (§6)
has a row outside the DEC-026 1e-9 criterion. Worst relative row error of
that benchmark (zero rows stay below 2.5e-15 of their scale throughout):

| s | PF EB sparse | PF EB dense | EX EB sparse | EX EB dense | EX TIM sparse | EX TIM dense |
|---|---|---|---|---|---|---|
| 1e3 | 9.4e-16 | 9.4e-16 | 4.4e-16 | 4.4e-16 | 4.7e-16 | 4.7e-16 |
| 1e4 | 9.4e-16 | 9.4e-16 | 4.7e-16 | 4.7e-16 | 4.7e-16 | 4.7e-16 |
| 1e5 | 3.8e-9 | 3.8e-9 | 3.8e-9 | 1.6e-9 | 4.3e-16 | 4.3e-16 |
| 1e6 | 5.3e-8 | 7.6e-9 | 6.1e-8 | 3.4e-8 | 2.7e-8 | 7.9e-9 |
| 1e7 | 4.1e-7 | 7.8e-7 | 4.1e-7 | 1.2e-7 | 6.1e-8 | 2.4e-7 |
| 1e8 | 3.4e-6 | 1.5e-6 | 2.0e-6 | 3.9e-6 | 2.0e-6 | 2.1e-6 |

The existing screens mark the same cases: each case at and above the first
failing decade is `sensitive` (Euler–Bernoulli from 1e5, Timoshenko from
1e6), every case below it `checks_passed`. The threshold remains a numerical
heuristic measured on one benchmark, not an accuracy bound or an engineering
limit; another model may lose digits elsewhere, and a grounded body may keep
them (the free body of §6 meets 1e-9 at s = 1e6). The existing screens remain the
qualification: the pivot screen (FK `structural.rs` ~1747), the sparse pivot
ratio (DEC-050), and formation and recovery qualification, including
force-scaled recovery after a range failure (PP `lib.rs` ~4017-4027). None of
them proves member-force accuracy, and the warning does not replace them. An
s·E, s·G or a product formed from them outside binary64's normal range
(overflow, or underflow for a small s) fails at section or stiffness
formation, where it is refused by name: an s·E or s·G that is not a normal
binary64 value is the blocking `PIPE_SECTION_INPUT_INVALID` of that member.

### 2.6 Diagnostics

| Code | Severity | When |
|---|---|---|
| `RIGID_COMPONENT_GEOMETRY_INPUT_MISSING` | warning | as today |
| `RIGID_COMPONENT_MAPPING_INPUT_INVALID` | warning | mapped member missing (no `rigid_pipe_ref`, or an unknown member) or not ending at the node; the member solves as pipe, unscaled |
| `RIGID_COMPONENT_CONSUMPTION_UNDECLARED` (new) | warning | rigid-family component with a scale but no `solver_consumption`; the member solves as pipe, unscaled. Without a scale, the first unmet later condition is named instead |
| `RIGID_COMPONENT_MAPPING_CONFLICT` (new) | blocking | §2.3's three refusals |
| `RIGID_COMPONENT_STIFFNESS_INPUT_MISSING` | warning | the scale or its source reference is missing; the member solves as pipe, unscaled |
| `RIGID_COMPONENT_STIFFNESS_INPUT_INVALID` | warning | the scale, or a supplied linear or rotational stiffness, is not finite positive; the member solves as pipe, unscaled |
| `RIGID_COMPONENT_STIFFNESS_SCALING_APPLIED` (replaces `..._REVIEWED`) | info | the scale is applied: names the member and s, and states that the member's stress rows are the reference pipe section's (not a body stress) and that linear/rotational stiffness are not applied |
| `RIGID_COMPONENT_SCALE_PRECISION` (new) | warning | s ≥ 1e5 (§2.5), on a published result |

These replace the stale texts at PP `validation.rs` ~1386 and ~1426 ("no
stiffness scaling is silently applied without a valid mapping"; "carries ...
stiffness scaling ... under DEC-045"), which implied a scaling that no route
applied.

### 2.7 The exact route

EX admits valve, flange, rigid and specialty components under
`3.0.0/exact_pressure_v3` with the same scaling. The admission table and its
texts (PP `exact_admission.rs` ~197-251, unit test ~312-350) change, and so
does the "Admitted families" sentence of the pressure-2 semantic contract
(`fixtures/results/semantic_contract_v0_3_pressure_2.json` ~2187). That
sentence is authored by hand (`tools/serialization/semantic_contract_tables.py`
~4). Its derived tables and pins are then regenerated with `derive` and
`repin`, followed by the required carrier regeneration.

A reducer whose mapped member belongs to any pressure region in any case is
refused on EX, by an explicit, case-specific refusal,
`EXACT_PRESSURE_REDUCER_IN_REGION`. The reducer's thrust from a bore change is
not modelled. The equal-bore guard (PP `pressure_runtime.rs` ~684-694) cannot
stand in for this refusal: it compares the bores of traversed members only,
never reads the reducer, and has nothing to compare in a single-member region.
A reducer whose mapped member lies outside every region is admitted like a
valve, including when it is mapped at a region's boundary node but its own
member is outside the region. A reducer marker with no mapped member is not
refused: it solves as pipe with the mapping warning (§2.6), no bore change is
modelled, and the equal-bore guard still governs any actual bore difference
between region members. The refusal names (case, region, component, member). Admitted valves and flanges are an equivalent
pipe of the represented bore: bore continuity, cap thrust and the annular κ are
those of the mapped section.

The retained and source sub-routes still refuse every component
(`retained_memory.rs` ~922, `source_recovery.rs` ~874) and are not touched.
Each asserts frame E equal to material E (`retained_product.rs` ~2093,
`source_recovery.rs` ~1659), which a scaled member would break, so the refusal
is kept deliberately.

### 2.8 Retained-memory pricing (D3)

Slice a changes the priced files PP `lib.rs`, `validation.rs`
(`validate_components` is body-pinned by `w8_topology_model.py`) and
`exact_admission.rs`, so it needs re-pricing and re-registration under
`tools/retained_pricing/README.md` in the solver lane's D3 turn. It changes
no FK file. The retained route refuses components, so its rows and P_FINAL
census are unchanged; slice d's optional row and coverage members appear only
on results with an applied scale. The eligibility predicate and the
mapping-conflict check are pricing inputs, to be priced or bounded in the
registration, not assumed free. The registration must still show every priced
phase fits M = 12 GiB; a misfit stops for the owner (decision 22).

## 3. Component weight and centre of gravity (slice b)

The component's weight W (a force) acts at its centre of gravity. The
self-weight generation (PP `self_weight.rs`) emits node-targeted primitives at
`component.node` in the same case for each selected rigid-family component with
weight and centre of gravity. They are the Cartesian components of the wrench:
a force F = W·ĝ along the gravity axis, and a moment M = r × F. Here r is the
centre-of-gravity vector and ĝ the case's gravity direction. A primitive
carries one magnitude and one direction (PP `lib.rs` ~788-805), so the
generator emits the force primitive plus one moment primitive per nonzero
component of M. With axis-aligned gravity that is at most two moments; for
example r = (1, 2, 3) m and F = (0, 0, −10) N give M = (−20, 10, 0) N·m.
Identities are deterministic and follow the generator's length-prefixed
convention (`self_weight.rs` ~376), with the component id, the wrench part
(force or moment) and the axis. Both routes already solve nodal forces and
moments (PP `lib.rs` ~802, ~14322; EX `tests/pressure_runtime.rs` ~984). The
pair is statically exact for the body; for a semi-rigid body it places the
weight at the body's node rather than along the member.

The mapped member's own distributed self-weight is not generated when its
component's weight is generated: the user's weight is the component's total,
including any contents the user includes. Generation records the method, the
component and the omitted member in its evidence rows. Lineage and staleness
checks extend to node-targeted generated loads.

**Centre-of-gravity convention (DEL-03-05 R10).** The owner ruled on
2026-10-24: "For the centre-of-gravity I approve option A as recommended."
Option A: `center_of_gravity` is r, a vector in the model's global X/Y/Z axes,
measured from the component's `node`, in model length units. Weight is the
nodal force F at that node plus the moments r × F. The convention is
unambiguous and needs no component-local frame. A model transformation rotates
r without translating it. Slice b's tests include a nontrivial offset along
each axis, checking both the force and the moment.

## 4. Eccentric reducer offset (slice c)

PRD §12.3 requires an eccentric offset vector for reducers. No slot exists.
The centerline model already places each node on its own centerline, so the
offset is geometry the nodes must agree with.

- **Ends.** The mapped member's `from` node is the reducer's upstream end A and
  its `to` node the downstream end B. `end_a_size` and `end_b_size` bind to A
  and B. The upstream and downstream section assignments are the sections of
  the pipes adjoining A and B. The body member keeps its own section as the
  reference section (§2.4).
- **Field.** `geometry.eccentric_offset` is e: a length vector in global axes
  from A's centerline to B's centerline, perpendicular to the reducer axis.
- **Relation.** The axis t (unit) and body length L = `rigid_body_length`
  satisfy x_B − x_A = L·t + e. With d = x_B − x_A − e, validation requires
  |‖d‖ − L| and |e·d/‖d‖| each within the model's node-proximity tolerance,
  and blocks on a mismatch. A zero or non-finite ‖d‖ is refused before
  normalizing, as is an end with no adjoining pipe or with more than one
  candidate adjoining section. A concentric reducer has e = 0 or no field.
- **Desktop.** It can place B from A, t, L and e.

The model document's component geometry record is closed in the schema
(`schemas/model.schema.yaml` ~355). The product's decoder ignores unknown keys,
so an older build would silently drop the field. Adding it therefore needs a
document version (0.8.0) and its schema, validator and fixture regeneration,
which is a shared interface with W-6 and W-12. Slice c waits for that
coordination.

## 5. Identifying reference-section results (slice d)

A scaled member's stress rows are stresses of a reference pipe section. Slice
d makes that meaning travel with the result, and ships with slice a (§2.4).
These are the decisions; the consumer lists come from round-2 scrutiny (Codex).

- **Applies to.** Only results with an applied scale. A result without one keeps
  its shape byte for byte, so slice d itself moves no golden without a scaled
  component. The exact-identity re-pins are separate and required: §2.7's
  corrected admission and "Admitted families" text changes the pressure-2
  identity limitation every exact-route result restates, so the exact bases
  and their binding hashes are re-pinned for that text (the retained corpus's
  six exact bases among them, each at that one identity field), not for any
  slice-d member.
- **Row record.** `component_body`: {`component_ref`, `stiffness_scale`,
  `section_role: "reference_pipe_section"`}. It is an optional, closed member
  on every stress row, stress extremum and maximum row of a scaled member, on
  both routes, every section basis and combination rows. Model-bound readers
  derive the expected set from the model's eligibility predicate. They refuse
  a missing, forged or inconsistent record, and refuse disagreement between
  basis twins. Each slice-d member is permitted on its own object only: the
  record on a row or extremum, `component_body_pipe_ids` on a coverage entry.
  Either one on the other object is a closed-shape refusal.
- **Maximum.** Per-member maxima and extrema stay. The case maximum and the
  headline range over non-body members. The coverage entry carries
  `component_body_pipe_ids` when nonempty. Completeness is unchanged. A case
  whose every available member is a body has a null headline.
- **Rules.** A rule bound to a body row reports `RULE_INPUTS_INCOMPLETE` with
  reason `COMPONENT_BODY_REFERENCE_SECTION`. No new outcome kind is added. A
  pack that binds every member is therefore incomplete while a scaled body is
  among them, which is the conservative reading. If rule scope cannot exclude
  members, that consequence goes to Agent 0 as an open choice, not worked
  around.
- **Exports.** Derivative quantity rows and source annotations copy the record
  verbatim. Both exporters (Rust `derivative.rs`, desktop
  `resultExportAdapter.ts`) validate it against the source row.
- **Desktop.** The stress headline and the local-FEA region from it never name
  a body. A body included for another reason, such as adjoining the
  displacement node or the fixed P-120 preference, is shown with its
  identification.

What each decision touches:

1. **Maximum and coverage.** Per-member maxima and `pipe_stress_extrema` stay
   published for every available member, so the coverage and row-binding
   invariants hold. The case maximum and headline are taken over the
   remaining, non-body members. Each coverage entry lists `component_body_pipe_ids`.
   If no eligible member remains, the headline is null. Consumers include:
   - Rust readers: `result_export/src/section_basis.rs` ~497/552,
     `physics_evidence.rs` ~561/583/1140/1519,
     `preview_physics_evidence.rs` ~420/508/854;
   - Python verifiers: `tests/verifiers/section_basis.py` ~341/370,
     `preview_physics_evidence.py` ~199/229/350, `pressure_2.py` ~1105;
   - closed schemas: `schemas/results.v0.3.schema.yaml` ~2427/3493,
     `PhysicsStressMaximumCoverageEntry`, and
     `schemas/stress_neutral_export.v0.3.schema.json` ~2697/3075;
   - `joint.rs` ~1008 and `tools/validation/qualification_physics_structure.py`
     ~465;
   - shared retained and source shapes: `physics_source.rs` ~278,
     `retained_precision.rs` ~6751, `retained_product.rs` ~2919/3069,
     `source_receipt/composite.rs` ~535. Their component refusals stay, but
     any unconditional new field changes their output shape.
2. **Rows and exports.** The record must have one explicit representation,
   preserved and validated in the source row, in derivative quantity rows
   (`result_export/src/derivative.rs` ~291, desktop `resultExportAdapter.ts`
   ~113, the closed `QuantityResult`/`PressureQuantityResult`/metadata shapes)
   and in source annotations. Missing, forged and inconsistent tags are
   refused, including on basis twins and combination rows. Existing bitwise
   checks keep their strength.
3. **Rules.** The rule runner has no not-applicable outcome
   (`rule_check_runner/src/lib.rs` ~123/184/283/551). Slice d either uses
   `RULE_INPUTS_INCOMPLETE` with the refusal reason
   `COMPONENT_BODY_REFERENCE_SECTION`, or adds an applicability outcome. The
   second touches aggregation, counts, declared statuses, persistence, the
   native binding (`src-tauri/src/lib.rs` ~3432), `ruleCheckService.ts`,
   `knownSemanticLimitations.ts`, `core/analysis_runs/compatibility.py` and
   `schemas/rule_pack.schema.yaml` ~102.
4. **Desktop.** The local-FEA handoff also selects pipes adjoining the
   displacement-headline node, and prefers a fixed P-120 row
   (`LocalFeaHandoffPanel.tsx` ~287/296/348). Selection and disclosure must
   carry the identification wherever a body member is included. The same
   applies to `ResultsPanel.tsx` ~232, `PhysicsSourceEvidencePanel.tsx` ~46,
   `ReportPanel.tsx` ~292, `HandoffPanel.tsx` ~335 and
   `NativePackagePanel.tsx` ~795.
5. **Semantic-contract tables.** The pressure-2 table and its pins are
   regenerated with `tools/serialization/semantic_contract_tables.py`
   (`derive`, `repin`), alongside the retained inherited-table pins.

As implemented (`core/product_physics/src/rigid_component.rs`,
`core/reporting/result_export/src/component_body.rs`, mirrored in
`tests/verifiers/component_body.py`):

- The stress rows are the row kinds whose `source_physical_semantic_dimension`
  is `stress` in the pressure-2 and preview-physics-2 tables (17 kinds); a unit
  test pins the producer's list to the tables.
- A source's bodies are read from its coverage entries, which must agree
  across cases and section bases; a record is refused as forged off a listed
  body's stress row, missing on one, or inconsistent across that body's rows,
  extrema and basis twins (`SOURCE_COMPONENT_BODY_*`). The model-bound
  readers compare the bodies with the model's eligibility predicate
  (`SOURCE_COMPONENT_BODY_MODEL_MISMATCH`). The preview reader takes these
  checks after its blocked-envelope rule.
- The preview transport check admits the coverage list and the extremum
  record. A derivative quantity and its source annotation carry the row's
  record; it is compared by value, the scale bit for bit, so a JSON form
  change does not unbind it.
- The pressure-2 table gains one limitation sentence naming the record and
  the rule refusal; the preview-physics-2 table is unchanged.

## 6. Verification

- **Rigid-link benchmark** (PRD §22.2), following W-9
  (`validation/references/w9_benchmarks`, `tests/w9_solver_benchmarks.rs`).
  - **Model.** An invented cantilever has a scaled body member between two
    pipe members, loaded by a tip force (transverse and axial) and a tip
    torque.
  - **Reference.** An exact rational generator, which runs no product code,
    gives in closed form for finite s: tip displacements and rotations, every
    member's end forces and the support reactions. It also checks the s → ∞
    limit against the rigid-link closed form.
  - **Runs.** s = 1 (equal to plain pipe), a soft s = 0.25, and every decade
    from 1e3 to 1e8. Euler–Bernoulli on both routes, and Timoshenko on EX,
    where `timoshenko_annulus_energy_v1` is selectable (PP `pressure_runtime.rs`
    ~49). Both solver modes. Below the precision threshold (§2.5, 1e5) every
    run asserts the 1e-9 relative criterion with W-9's zero-scale rule and no
    precision warning. At and above it a run asserts no accuracy: only the
    warning, once for the body, the readers' admission and the screens'
    unchanged standing of the case; its measured errors are printed. The
    decades are the measurement that sets the threshold.
- **Reference formulas** (independent generator):
  - transverse tip load F: δ = F·Σᵢ∫(L−x)²/(sᵢEᵢIᵢ)dx, plus F·Σᵢ Lᵢ/(sᵢκᵢGᵢAᵢ)
    for Timoshenko. Axial and torsional cases use their compliance sums. In
    the rigid limit the body's compliance is removed and its moment arms kept.
  - thermal and fit series, tension positive:
    N = −Σᵢ(εᵢLᵢ) / Σᵢ[Lᵢ/(sᵢEᵢAᵢ)].
  - EX closed end: Nm = (1−2ν)pAᵢ, Nw = pAᵢ, ε = (1−2ν)pAᵢ/(sEAₙ).
- **Headline switch.** The indeterminate two-member case of §2.4 at s = 1 and
  s = 4, through the public readers. Its checks:
  - the headline stays on the pipe;
  - the body's rows carry `component_body`;
  - the coverage entry lists the body;
  - a rule bound to the body reports `COMPONENT_BODY_REFERENCE_SECTION`;
  - both exporters preserve and validate the record;
  - forged, missing and inconsistent records are refused;
  - an all-body case has a null headline.
- **Thermal series case.** Frozen independent expectations, with signs and
  zero scales. It includes nonzero fit strain, fit-only loading and
  thermal/fit cancellation. A member anchored at both ends, a scaled body in series with
  pipe, under ΔT: the reactions follow the series stiffness. A free scaled
  body's growth is αΔT·L. Both thermal producers are covered (the element
  thermal load and the load-state member pair with its fit eigenstrain), on
  material, named and interpolated bases.
- **EX pressure case.** Frozen independent expectations. A closed-end
  cantilever with a scaled body: the body's wall axial force is unchanged and
  its pressure axial strain is s times smaller (§2.1).
- **Negative and behaviour cases.**
  - The eligibility conditions of §2.3, one test each. This includes
    `not_solver_consumed`, an absent declaration and an invalid optional
    quantity.
  - The three mapping conflicts.
  - Reducers on EX: inside a single-member region, inside an equal-bore
    region, at a region's boundary, and outside every region.
  - PF ν: absent, and consistent and inconsistent optional ν, through the
    model-bound reader on material and named bases. Interpolated PF ν already
    skips model-bound recomputation (`u4b_evidence.rs` ~523); that existing
    limit is stated, not closed, here.
  - The s ≥ 1e5 warning, with formation and recovery qualification unchanged;
    no applied-scale info or warning on a refused or blocked model (input
    refusal, mapping conflict, scaled moduli outside the normal range).
  - Slice b: weight replacement, staleness, every gravity axis and sign, and
    wrench components.
  - Slice c: eccentric geometry, accepted and refused.
- **Regression.** The bundled demo's valve C-130 (s = 15, `pipe:P-130`) moves
  the demo and preview-physics result fixtures. They are regenerated by their
  generators with the cause stated. This is regression evidence, not an
  independent numerical reference. The native smoke's row counts and standing
  are expected unchanged; any `apps/desktop/native-smoke/expected.json` value
  that moves is reported, not edited.
- **Independent numerical scrutiny** of each slice, and of the D3 registration.
