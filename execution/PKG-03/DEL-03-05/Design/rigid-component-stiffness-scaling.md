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
| Pressure Poisson elongation (straight member, EX) | the elastic response 2νpAᵢ/(s·E·Aₙ): s times smaller, as for any elastic strain of a stiffer body. The load pair ±2νpAᵢ (PP `pressure_exact/source_geometry.rs:314-323`) has no E and is unchanged; the recovery Nw = Nm + 2νpAᵢ (`:349-379`) is unchanged |
| Member end forces and section actions | from s·K·Δu minus the scaled fixed-end loads; equilibrium is unchanged |
| Stress rows | recovered from the forces on the mapped pipe section, as for pipe (§2.4) |
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
the pair's (~6665, ~3935, ~10809). The PF Poisson ratio is read back as
E/(2G)−1 (~6485 → `u4b_rows.rs:573-575`); under scaling it can differ from the
material's ν in the last bits, so the scaled member's ν is taken from the
material directly.

No FK file changes.

### 2.3 Which members are scaled

Scaling applies when the component is rigid-family, its mapping is valid, and
its scale is present, finite and positive. Otherwise the member solves as pipe
and the existing warnings say so in plain terms (§2.5). Three mappings that
today pass silently are refused (blocking, by name):
- the mapped member is a realized curved bend (PP `lib.rs` ~8171);
- the mapped member is a span replaced by an objective connector (~7982, ~8173);
- two components map the same member.

### 2.4 Stress rows on a scaled member

The rows remain published: they are the mapped pipe section's stresses under
the body's actual forces, which at the member ends equal the adjacent pipe's.
They are not a stress of the valve or flange body, and the product supplies no
body stress. An info diagnostic per scaled component names the member and says
so. The rows stay in the case maximum. The rule-check boundary is unchanged;
rule packs decide whether those members are checked.

### 2.5 Numerical bound

The product accepts any finite positive s; there is no engineering bound. For
large s, the member's forces come from s·K·Δu with Δu of order 1/s, so about
log₁₀ s decimal digits of the member's force are lost to cancellation. From
s ≥ 1e8 (half of binary64's digits) a warning, `RIGID_COMPONENT_SCALE_PRECISION`,
states this. The existing pivot screens (FK `structural.rs` ~1747; sparse pivot
ratio, DEC-050) still refuse a singular or ill-conditioned system. Ranges that
overflow E·A stay refused as `PIPE_SECTION_INPUT_INVALID`.

### 2.6 Diagnostics

| Code | Severity | When |
|---|---|---|
| `RIGID_COMPONENT_GEOMETRY_INPUT_MISSING` | warning | as today |
| `RIGID_COMPONENT_MAPPING_INPUT_INVALID` | warning | mapped member missing or not ending at the node; the member solves as pipe, unscaled |
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

A reducer stays refused on EX while it sits inside a pressure region: the
equal-bore guard (PP `pressure_runtime.rs` ~684-690) refuses it, and the
reducer's thrust from a bore change is not modelled. A reducer outside every
pressure region is admitted like a valve.

The retained and source sub-routes still refuse every component
(`retained_memory.rs` ~922, `source_recovery.rs` ~874) and are not touched.
Both assert frame E equal to material E, which a scaled member would break, so
the refusal is kept deliberately.

### 2.8 Retained-memory pricing (D3)

Slice a changes the priced files PP `lib.rs`, `validation.rs`
(`validate_components` is body-pinned by `w8_topology_model.py`) and
`exact_admission.rs`, so it needs re-pricing and re-registration under
`tools/retained_pricing/README.md` in the solver lane's D3 turn. It changes
no FK file and no P_FINAL term. The scale adds no allocation: it is one f64
read per member at formation. The registration must still show every priced
phase fits M = 12 GiB; a misfit stops for the owner (decision 22).

## 3. Component weight and centre of gravity (slice b)

The component's weight W (a force) acts at its centre of gravity. The
self-weight generation (PP `self_weight.rs`) emits, for each selected
rigid-family component with weight and centre of gravity, two node-targeted
primitives at `component.node` in the same case: a force F = W·ĝ along the
gravity axis, and a moment M = r × F. Here r is the centre-of-gravity vector
and ĝ the case's gravity direction. Both routes already solve nodal forces and
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
length units. That is the convention the bundled demo's invented values
already read naturally, and it needs no component-local frame.

## 4. Eccentric reducer offset (slice c)

PRD §12.3 requires an eccentric offset vector for reducers. No slot exists.
The centerline model already places each node on its own centerline, so the
offset is geometry the nodes must agree with. The proposed field is
`geometry.eccentric_offset`: a length vector in global axes from the upstream
centerline to the downstream centerline. Validation then compares it with the
mapped member's end positions, within the model's node-proximity tolerance, and
blocks on a mismatch. The desktop can derive the downstream node from it.

The model document's component geometry record is closed in the schema
(`schemas/model.schema.yaml` ~355). The product's decoder ignores unknown keys,
so an older build would silently drop the field. Adding it therefore needs a
document version (0.8.0) and its schema, validator and fixture regeneration,
which is a shared interface with W-6 and W-12. Slice c waits for that
coordination.

## 5. Verification

- **Rigid-link benchmark** (PRD §22.2), following W-9
  (`validation/references/w9_benchmarks`, `tests/w9_solver_benchmarks.rs`). An
  invented cantilever has a scaled body member between two pipe members, with
  a tip force and a tip moment. An exact rational generator, which runs no
  product code, gives tip displacement and rotation in closed form for finite
  s. It also checks the s → ∞ limit against the rigid-link closed form; the
  product runs at s = 1e3 and 1e6 against the finite-s form. The benchmark runs
  on both routes and in both solver modes, at the 1e-9 relative criterion.
- **Thermal check.** A fixed-fixed member with a scaled body under ΔT: the
  reactions follow the series stiffness of the body and the pipe, and a free
  scaled body's growth is αΔT·L.
- **EX pressure check.** On a closed-end cantilever with a scaled body, the wall
  axial force on the body is unchanged and its Poisson elongation is s times
  smaller.
- **Regression.** The bundled demo's valve C-130 (s = 15, `pipe:P-130`) moves
  the demo and preview-physics result fixtures. These are regenerated by their
  generators with the cause stated. The native smoke's row counts and standing
  are expected unchanged; any `apps/desktop/native-smoke/expected.json` value
  that moves is reported, not edited.
- **Independent numerical scrutiny** of each slice, and of the D3 registration.
