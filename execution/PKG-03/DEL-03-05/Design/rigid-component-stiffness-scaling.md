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

Otherwise the member solves as unscaled pipe, and a warning names the first
unmet condition (§2.6). This includes an absent consumption declaration, which
gets its own warning, since no mode is assumed. `not_solver_consumed` is
honoured: the member solves as pipe with no rigid warning beyond the existing
unsupported-interface diagnostics (PP `validation.rs` ~1227-1253). Three
mappings that today pass silently are refused (blocking, by name):
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

Because the reference section can govern only by being an artificial choice,
the meaning travels with the result:
- **Rows.** Every stress row of a scaled member carries a `component_body`
  record: `component_ref`, `stiffness_scale` and
  `section_role: "reference_pipe_section"`. This holds on both routes and
  every section basis.
- **Maximum and headline.** The case stress maximum and the headline exclude
  scaled members, and the maximum record lists them in
  `component_body_pipe_ids`, beside the existing `unavailable_pipe_ids` (PP
  `lib.rs` ~6662). An excluded member is not an unavailable one, and the
  maximum's completeness is unchanged by it. The local-FEA selection, which
  takes the headline member as its region (`LocalFeaHandoffPanel.tsx`
  ~282-319), therefore never selects a reference section.
- **Rule binding.** A rule bound to a tagged row reports not-applicable with
  the named reason `COMPONENT_BODY_REFERENCE_SECTION`, instead of evaluating
  the reference-section stress (`result_export/src/semantic_contract.rs`
  ~683).
- **Exports and readers.** Exports carry the record, and the model-bound
  readers check it.

An info diagnostic per scaled component names the member and the scale.

### 2.5 Numerical bound

The product accepts any finite positive s; there is no engineering bound. A
large s does not by itself lose accuracy: a grounded scaled member recovers its
load accurately. Loss arises when the body's small deformation is recovered
from much larger nodal motions, or when large elastic and eigenload terms
nearly cancel. From s ≥ 1e8, half of binary64's decimal digits, a warning,
`RIGID_COMPONENT_SCALE_PRECISION`, states that the scaled member's forces may
carry reduced precision. This is a conservative numerical heuristic, not an
accuracy bound or an engineering limit. The existing screens remain the
qualification: the pivot screen (FK `structural.rs` ~1747), the sparse pivot
ratio (DEC-050), and formation and recovery qualification, including
force-scaled recovery after a range failure (PP `lib.rs` ~4017-4027). None of
them proves member-force accuracy, and the warning does not replace them. An
s·E·A outside binary64's range fails at stiffness formation, where it is
refused by name.

### 2.6 Diagnostics

| Code | Severity | When |
|---|---|---|
| `RIGID_COMPONENT_GEOMETRY_INPUT_MISSING` | warning | as today |
| `RIGID_COMPONENT_MAPPING_INPUT_INVALID` | warning | mapped member missing or not ending at the node; the member solves as pipe, unscaled |
| `RIGID_COMPONENT_CONSUMPTION_UNDECLARED` (new) | warning | rigid-family component with a scale but no `solver_consumption`; the member solves as pipe, unscaled |
| `RIGID_COMPONENT_MAPPING_CONFLICT` (new) | blocking | §2.3's three refusals |
| `RIGID_COMPONENT_STIFFNESS_INPUT_MISSING` | warning | the scale or its source reference is missing; the member solves as pipe, unscaled |
| `RIGID_COMPONENT_STIFFNESS_INPUT_INVALID` | warning | the scale, or a supplied linear or rotational stiffness, is not finite positive; the member solves as pipe, unscaled |
| `RIGID_COMPONENT_STIFFNESS_SCALING_APPLIED` (replaces `..._REVIEWED`) | info | the scale is applied: names the member and s, and states that stress rows use the pipe section and linear/rotational stiffness are not applied |
| `RIGID_COMPONENT_SCALE_PRECISION` (new) | warning | s ≥ 1e8 (§2.5) |

These replace the stale texts at PP `validation.rs` ~1386 and ~1426 ("no
stiffness scaling is silently applied without a valid mapping"; "carries ...
stiffness scaling ... under DEC-045"), which implied a scaling that no route
applied.

### 2.7 The exact route

EX admits valve, flange, rigid and specialty components under
`3.0.0/exact_pressure_v3` with the same scaling. The admission table and its
texts (PP `exact_admission.rs` ~197-251, unit test ~312-350) change, and so
does the pinned "Admitted families" sentence of the pressure-2 semantic
contract (`fixtures/results/semantic_contract_v0_3_pressure_2.json` ~2187),
through its maintained generator.

A reducer whose mapped member belongs to any pressure region in any case is
refused on EX, by an explicit, case-specific refusal,
`EXACT_PRESSURE_REDUCER_IN_REGION`. The reducer's thrust from a bore change is
not modelled. The equal-bore guard (PP `pressure_runtime.rs` ~684-694) cannot
stand in for this refusal: it compares the bores of traversed members only,
never reads the reducer, and has nothing to compare in a single-member region.
A reducer whose mapped member lies outside every region is admitted like a
valve, including when it is mapped at a region's boundary node but its own
member is outside the region. Admitted valves and flanges are an equivalent
pipe of the represented bore: bore continuity, cap thrust and the annular κ are
those of the mapped section.

The retained and source sub-routes still refuse every component
(`retained_memory.rs` ~922, `source_recovery.rs` ~874) and are not touched.
Both assert frame E equal to material E, which a scaled member would break, so
the refusal is kept deliberately.

### 2.8 Retained-memory pricing (D3)

Slice a changes the priced files PP `lib.rs`, `validation.rs`
(`validate_components` is body-pinned by `w8_topology_model.py`) and
`exact_admission.rs`, so it needs re-pricing and re-registration under
`tools/retained_pricing/README.md` in the solver lane's D3 turn. It changes
no FK file and no P_FINAL term. The scale adds no new owner on the
admitted domain beyond ordinary component diagnostics and the `component_body`
rows; the eligibility predicate and the mapping-conflict check are pricing
inputs to be priced or bounded in the registration, not assumed free. The registration must still show every priced
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
Identities are deterministic:
`load:<case>:component-weight:<component>:force_<axis>` and
`:moment_<axis>`. Both routes already solve nodal forces and
moments (PP `lib.rs` ~802, ~14322; EX `tests/pressure_runtime.rs` ~984). The
pair is statically exact for the body; for a semi-rigid body it places the
weight at the body's node rather than along the member.

The mapped member's own distributed self-weight is not generated when its
component's weight is generated: the user's weight is the component's total,
including any contents the user includes. Generation records the method, the
component and the omitted member in its evidence rows. Lineage and staleness
checks extend to node-targeted generated loads.

**Reserved.** DEL-03-05 R10 holds the centre-of-gravity coordinate convention
and reference frame for a human ruling. Slice b waits for it. Recommended: r
in the model's global axes, measured from the component's `node`, in model
length units. It is unambiguous and needs no component-local frame. A model
transformation rotates r without translating it.

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
  and blocks on a mismatch. A concentric reducer has e = 0 or no field.
- **Desktop.** It can place B from A, t, L and e.

The model document's component geometry record is closed in the schema
(`schemas/model.schema.yaml` ~355). The product's decoder ignores unknown keys,
so an older build would silently drop the field. Adding it therefore needs a
document version (0.8.0) and its schema, validator and fixture regeneration,
which is a shared interface with W-6 and W-12. Slice c waits for that
coordination.

## 5. Verification

- **Rigid-link benchmark** (PRD §22.2), following W-9
  (`validation/references/w9_benchmarks`, `tests/w9_solver_benchmarks.rs`).
  - **Model.** An invented cantilever has a scaled body member between two
    pipe members, loaded by a tip force (transverse and axial) and a tip
    torque.
  - **Reference.** An exact rational generator, which runs no product code,
    gives in closed form for finite s: tip displacements and rotations, every
    member's end forces and the support reactions. It also checks the s → ∞
    limit against the rigid-link closed form.
  - **Runs.** s = 1 (equal to plain pipe), a soft s = 0.25, s = 1e3 and
    s = 1e6. Euler–Bernoulli and Timoshenko, both routes, both solver modes,
    at the 1e-9 relative criterion with W-9's zero-scale rule.
- **Thermal series case.** Frozen independent expectations, with signs and
  zero scales. A member anchored at both ends, a scaled body in series with
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
    model-bound reader.
  - The `component_body` record, maximum exclusion and rule not-applicable
    through the public readers.
  - The s ≥ 1e8 warning, with formation and recovery qualification unchanged.
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
