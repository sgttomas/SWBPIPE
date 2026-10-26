# Eccentric reducer offset (W-15 slice c)

This design realizes the "eccentric offset vector" of PRD §12.3 (Reducers) for
the reducer component. It is W-15 slice c in `docs/COMPLETENESS_MAP.md` §4.3,
in the owner's approved W-15 scope: OD-2 (owner, 2026-10-23, "I approve OD-1
through OD-5 as you recommended") selected DEC-045 stiffness scaling for rigid
and semi-rigid components, and W-15 slice c carries the reducer's offset. It
replaces the outline formerly in `rigid-component-stiffness-scaling.md` §4. The
model document part, version 0.8.0, is DEL-02-01 Design
[`model-document-0.8.0.md`](../../../PKG-02/DEL-02-01/Design/model-document-0.8.0.md).

**Status.** This is revision 2 of the design pass at main `2d4b1bb4bb`
(2026-10-26), PR #1649. It resolves Codex's design review r1 and applies Agent
0's rulings of 2026-10-26. Nothing here is built.
- **Owner's choices:**
  - R-c1, the offset convention (§10);
  - the stored-data and recorded-run questions of 0.8.0 (DEL-02-01 Design
    `model-document-0.8.0.md` §3).
- **Agent 0's resolutions:** the engineering-judgment points are recorded in §9.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081): the
benchmark is a candidate reference, and no output here is engineering approval.

Paths are relative to `projects/chirality-piping/`. PP is
`core/product_physics/src`.

## 1. What it is

An eccentric reducer joins two pipes of different size. The centerlines at its
two ends are parallel but not collinear: one side of the body is flat, so the
small end's centerline is offset from the large end's. The offset is the
transverse vector from one end's centerline to the other's. A concentric
reducer has a zero offset.

In a centerline model each node already sits on its own pipe's centerline.
- **The geometry is in the nodes.** An eccentric reducer's two end nodes
  differ by the axial run plus a transverse separation D⊥.
- **D⊥ governs the mechanics, not the declared offset.** An axial force P
  through the step acts on a line displaced by D⊥ from the upstream
  centerline. The upstream run therefore carries the moment P·‖D⊥‖. Thermal
  growth of the body moves the downstream run along and across the axis.
- **The declared offset e** records the reducer's eccentricity as a property of
  the component. The solve checks it against D⊥ (§3.2) and never uses it in
  place of the node geometry. Every "M = P·e" statement in this design
  means P·‖D⊥‖ of the actual geometry; e agrees with D⊥ within the placement
  tolerance.

## 2. The authored member

| Item | Rule |
|---|---|
| Name | `geometry.eccentric_offset`, on the component's `geometry` record |
| Shape | the existing `VectorQuantity`, `{x, y, z, unit}`. Each component is a finite number. `unit` is a length unit of the units catalog. |
| Frame | the model's global X/Y/Z axes, the frame of node positions |
| Origin and sign | R-c1 (§10), owner. Recommended option A: from the centerline at the component's `node` to the centerline at the mapped body member's other end. The convention is isolated in one place (§3.6). |
| Kind | `reducer` only; on any other kind it is refused by name (§3.4) |
| Absence | Absent means no declared offset. No offset check runs, and the reducer solves as its nodes place it. |
| Zero vector | Declares a concentric reducer, and is checked (§3.2). |
| Default | none. The product never writes the member and never supplies a value (CLM-004). |
| Units | normalized to metres with the other component geometry (PP `lib.rs` `normalize_model_units`, beside `center_of_gravity`, ~9559) |

The body length `geometry.rigid_body_length` and the end sizes stay
descriptive, as for every rigid-family kind. The solve does not compare them
with the node geometry (§9, P4).

## 3. How it enters the solve

### 3.1 Mechanism: the existing body member between the two centerlines

There is no new element and no rigid offset link. The reducer's body is its
mapped straight member (W-15 §1). The member's end nodes are the two centerline
points, so it runs along the chord D = x_F − x_N. Its length is ‖D‖, and it is
inclined to the run by atan(‖D⊥‖ / (D·t)). Both routes form it in one shared
function, `build_model_for_members` (PP `lib.rs` ~8115), with the existing
direction cosines and `y_reference`.

Why this mechanism:
- **It is available on both routes.** Offsets exist only on the exact route's
  objective connectors and tie rods (`frame_kernel/src/connector.rs`,
  `tie_rod.rs`), and the pressure-free route refuses connectors. The straight
  member is the only formation both routes share. Using it touches no
  frame-kernel file.
- **It gives the step's statics from the actual geometry.** The member joins
  the two centerlines, so an axial force P through the step has the lever ‖D⊥‖
  about the upstream centerline. The upstream run carries P·‖D⊥‖. No load is
  added.
- **Its rigid limit matches a rigid offset link, with qualifications.** As
  DEC-045's scale s → ∞ the body's elastic compliance vanishes, and end F
  follows end N as a rigid body: u_F = u_N + θ_N × D. This kinematic
  equivalence holds only for these cases:
  - **Zero body eigenstrain.** A body with free strain does not match a
    rigid link. Thermal growth stays αΔT·D at every scale, the offset
    included, while a rigid link grows only if it is given a strain.
  - **Equivalent loading.** Load resultants differ unless matched. The
    chord's distributed self-weight acts over ‖D‖ and along the chord, while
    a rigid link carries no distributed load unless one is placed on it.

  §7.2 checks the mechanical-load limit.
- **It differs from a rigid link at finite s.** A semi-rigid body bends and
  stretches along the chord rather than along the axis. This is within the
  DEC-045 approximation the product already states: a scaled reference
  section (W-15 §2.4) (§9, P1).

The offset member itself enters the solve only through the check in §3.2. It
does not place, move or re-form anything.

### 3.2 The consistency check

The check is in PP `rigid_component.rs`. It runs on every route after unit
normalization, so nodes, offset and tolerance are all in metres. It sits
immediately after `normalize_model_units` and its blocking return (PP `lib.rs`
~2805), before formation.

**Applicability.** The check applies to each `reducer` that carries
`eccentric_offset`, unless the reducer's `solver_consumption` is
`not_solver_consumed`. That mode is honoured: the member is not read and no
offset diagnostic is stated. Ordinary structural and unit validation still
applies (§9, P6).

**Early return.** A first pass over the components returns before any index or
map is built when no component is applicable (§8).

1. **Ends.** The ends come from the convention function (§3.6): the origin
   end N and the far end F of the mapped member `geometry.rigid_pipe_ref`.
   The member must exist and end at the component's node (W-15's mapping
   rule). Then D = x_F − x_N.
2. **Run direction** (§3.3). This gives t, oriented so that D·t > 0, or a
   named refusal.
3. **Requirements**, in this order:
   - **Axial length.** D·t > 0, so the body has length along the run.
   - **Transversality.** |e·t| ≤ ρ·‖e‖ with ρ = 1e-3, so the declared
     offset lies within 0.057° of the plane transverse to the run. A zero e
     passes. This is a numerical agreement rule on the declared vector
     itself, independent of the placement tolerance. Without it, a collinear
     e = (5, 0, 0) mm on concentric geometry would pass the placement test
     with a residual of exactly 5 mm (§9, P2).
   - **Placement.** ‖D⊥ − e‖ ≤ τ, where D⊥ = D − (D·t)·t and τ is the model's
     node-proximity tolerance (`PreviewModel::node_proximity_tolerance_m`:
     5 mm unless `model_settings.node_proximity_tolerance` sets another).
4. **Arithmetic.** Every quantity is formed in binary64 from the normalized
   values. The comparisons are `≤` on the Euclidean norm and on |e·t|.

The actual geometry governs. A declared offset that passes is a statement
about the reducer, verified to within τ. The lever arm, the stiffness and every
result come from the nodes, and the solve is bit-identical to the same model
without the offset member (§12).

### 3.3 The run direction (refusal boundary)

§1 defines the end centerlines as parallel, so the run direction must be
supported at the ends and must agree where both ends support it.

**Evidence at an end.** At each end X ∈ {N, F}, consider every member other
than the body that ends at X. Each is classified:

| Adjoining member | Class | Direction it gives |
|---|---|---|
| straight pipe mapped by no component | run | its axis |
| straight body of a rigid-family component with no nonzero offset (valve, flange, rigid, specialty, concentric reducer) | run | its axis |
| curved member realized by a DEC-070 bend macro-element | run | the arc's end tangent at X, from the realized arc geometry (PP `lib.rs` ~8715, chord, radius and `y_reference`) |
| body of another reducer carrying a nonzero offset | unsupported | none: its chord is slanted, not a run |
| member mapped by a bend component and not realized as an arc | unsupported | none: a chord is not a tangent |
| span replaced by an objective connector | unsupported | none |

**The end's status:**
- *none*: no adjoining member, a terminal end;
- *direction*: exactly one adjoining member, of class run;
- *ambiguous*: more than one adjoining member, such as a tee or branch;
- *unsupported*: exactly one adjoining member, of class unsupported.

**Decision:**

| N | F | Outcome |
|---|---|---|
| direction | direction | Directions must agree: ‖t_N × t_F‖ ≤ ρ (ρ = 1e-3 rad, §9, P3). Then t = t_N. Otherwise refuse with reason `run_directions_disagree`. |
| direction | none | t = t_N |
| none | direction | t = t_F |
| none | none | refuse, `standalone_body` |
| ambiguous (either end) | any | refuse, `ambiguous_run`, naming the end and its candidates |
| unsupported (either end) | any | refuse, `unsupported_run_evidence`, naming the end and member |

Ambiguous and unsupported are checked before none and direction. A
neighbouring concentric body counts as run evidence, so flange–reducer–flange
is accepted. Two eccentric reducers in series are refused. So is a reducer
between two members mapped by bend components that are not realized as arcs,
since neither end gives a tangent.

### 3.4 Diagnostics

| Code | Severity | When |
|---|---|---|
| `RIGID_COMPONENT_ECCENTRIC_OFFSET_INVALID` (new) | blocking | A present offset cannot be checked. The message names the component and the first reason, in this order: `not_reducer`; `not_finite` (a component of e after conversion); `mapping_invalid`; `ambiguous_run`; `unsupported_run_evidence`; `standalone_body`; `run_directions_disagree`; `no_axial_length` (D·t ≤ 0). |
| `RIGID_COMPONENT_ECCENTRIC_OFFSET_NOT_TRANSVERSE` (new) | blocking | |e·t| > ρ·‖e‖. The message names e, t, the axial share |e·t|/‖e‖ and ρ. |
| `RIGID_COMPONENT_ECCENTRIC_OFFSET_MISMATCH` (new) | blocking | ‖D⊥ − e‖ > τ. The message names N, F, e and D⊥ in metres, the residual and τ. |
| `RIGID_COMPONENT_ECCENTRIC_OFFSET_REALIZED` (new) | info | On a published result only, like slice a's applied-scale info (PP `rigid_component.rs` `append_formed_diagnostics`). See below. |

The `..._REALIZED` info states:
- the offset as entered, and D⊥ as realized;
- that the realized D⊥ governs;
- that the eccentricity is carried by the body member between the two
  centerlines, with its length ‖D‖ and its inclination;
- that no rigid offset link is added;
- that the member's stress rows are those of the mapped (reference) pipe
  section along the chord, whether or not a scale is applied (§9, P11).

### 3.5 Effects

| Effect | Treatment |
|---|---|
| Stiffness | The body member N→F along D, with its section's s·K when the W-15 scale is eligible and plain K otherwise. |
| Statics | An axial force through the step gives P·‖D⊥‖ in the upstream run. The body carries N = P·(D̂·t), a shear P·‖D⊥‖/‖D‖, and a moment that falls linearly from P·‖D⊥‖ to 0 along the chord (§7). |
| Self-weight | The member's distributed self-weight acts over ‖D‖ (§9, P5). Slice b replaces it when the component's weight is generated (W-15 §3). |
| Component weight | Slice b's wrench at the component's node, with the R10 centre of gravity. It is independent of the offset. |
| Thermal | The body's free growth is αΔT·D, offset included, at every scale (§9, P10). |
| Pressure | PF: none. EX: a reducer whose member lies in a pressure region is refused (`EXACT_PRESSURE_REDUCER_IN_REGION`, W-15 §2.7). Outside every region it is admitted like a valve. |
| Other element loads | As authored on the body member, over ‖D‖ |
| Displacements, reactions | Unchanged meaning. Node motions are the centerline motions. |
| Stress rows | Rows of the mapped pipe section along the chord. A scaled body's rows carry slice d's `component_body` record and are excluded from the headline (W-15 §2.4, §5). An unscaled reducer's rows are ordinary pipe rows, and the reference-section meaning is disclosed in the info, the UI and the report (§4, §9, P11). |

### 3.6 Convention isolation (R-c1)

The convention lives in exactly one place per language:
- **Rust:** `rigid_component::eccentric_offset_ends(component, member) -> (N, F)`
  is the only reader of the origin rule. Option A gives N = the component's
  node and F = the member's other end. Option B gives N = the member's `from`
  and F its `to`.
- **TypeScript:** `ECCENTRIC_OFFSET_CONVENTION` (one module) holds the text the
  inspector, creation form and report show.

The convention's tests are separate (`tests/w15c_offset_convention.rs` and one
vitest), and §7.5 holds the convention-dependent benchmark rows. A B ruling
changes the function body, the text constant and those tests' expected signs,
and nothing else.

### 3.7 The exact route and the retained route

The exact route forms the same member in the same function, with
Euler–Bernoulli or `timoshenko_annulus_energy_v1` (PP `pressure_runtime.rs`
~49). Under Timoshenko the body's shear adds its shear compliance, and §7's
generator includes it. The reducer-in-region refusal is unchanged. The retained
and source sub-routes still refuse every component (W-15 §2.7).

### 3.8 Not done in slice c

These are recorded as remaining items:
- a rigid offset link element;
- desktop node placement from the offset. Placement stays manual: the user
  positions nodes with the existing node edits, and the check reports any
  disagreement;
- a body-length check;
- vector authoring of the offset in the component library schema
  (`schemas/component.schema.yaml` `ComponentFieldKind`). Library apply still
  accounts for an entered offset (§4);
- reducer SIF or other local modifiers.

## 4. Authoring, adoption, reporting

- **Applier.**
  - A field rule `geometry.eccentric_offset` of kind
    `OptionalVectorQuantity { dimension: "length" }`, beside
    `center_of_gravity` (`operation_applier/src/lib.rs` ~652).
  - `resolve_field` refuses the field on kinds other than `reducer`.
  - Before value, equality and inverse follow the centre-of-gravity edit
    (W-15 §3, "Applier").
  - Reducer creation accepts an optional exact `{x, y, z, unit}`; it is never
    defaulted.
- **Types and adoption checks.**
  - `apps/desktop/src/types.ts` gains `eccentric_offset?: VectorQuantity` on
    component geometry.
  - The descriptor checks gain `eccentric_offset: "point"`: the webview's
    `COMPONENT_DESCRIPTORS` (`features/model-file/modelDocumentShape.ts` :38)
    and its native twin (`src-tauri/src/model_document_shape.rs` :86,
    `Member::Point`).
  - Focused tests in both cover a well-formed vector, a malformed one, and
    absence.
  - The document validator (`operation_applier/src/document_validator.rs`)
    does not run JSON Schema and does not validate component geometry (its
    scope, :1). It gains nothing. The slot's shape is judged by the
    descriptor checks at adoption, by the solve wire, and by the schema
    conformance test at test time.
- **Library apply** (`features/library/libraryApply.ts`).
  - `GROUP_VALUES.rigid` (:596) gains `geometry.eccentric_offset` as a
    retained, non-replaceable value, like `center_of_gravity`. A reducer that
    keeps an entered offset therefore counts it in the retained-value and
    source-reference accounting (:708–:716).
  - A group or replace-all apply cannot replace the group source reference as
    though no user geometry remained.
  - Tests cover keep, group and all on a reducer with an offset.
- **Geometry operations.** Wherever an operation carries a component's
  centre-of-gravity vector, it carries the offset the same way. The
  attachment refusals that block moving a component's nodes are unchanged.
- **Desktop.**
  - Reducer creation (`componentIntent.ts`, `RigidDraftFields.tsx`) and the
    property inspector (`PropertyInspector.tsx`) edit the offset with the
    centre-of-gravity vector editor, on reducers only. Both show
    `ECCENTRIC_OFFSET_CONVENTION`.
  - Placement stays manual.
  - The solve's findings appear in the diagnostics list.
  - Wherever the UI identifies a body row, it also discloses that an
    eccentric reducer's chord rows are reference-section stresses, scaled or
    not.
- **Report.** `ReportPanel.tsx` `reportComponentProvenance` (:341) adds the
  entered offset with its unit and the convention text. It also states that
  the reducer's stress rows are reference-pipe-section stresses along the
  chord, in both the scaled and unscaled cases. The rendered-report section
  that lists component provenance carries the same text.

## 5. The model document

The member is new on the closed component geometry record
(`schemas/model.schema.yaml` ~355). It arrives in model document 0.8.0 under
the one-current-version rule. DEL-02-01 Design
[`model-document-0.8.0.md`](../../../PKG-02/DEL-02-01/Design/model-document-0.8.0.md)
covers:
- the ripple;
- the refusals;
- the owner questions on stored data and recorded runs;
- #1630's closed record;
- the D3 chain;
- the guards;
- the reserved version-token text.

## 6. The demo and fixtures

- The bundled demo and every other committed current document move to 0.8.0
  by version token only, through their maintained writers. No physical value
  moves.
- **The demo does not gain an eccentric reducer.** That would change its
  geometry, results and native-smoke observations, which the owner-reserved
  `expected.json` pins.
- The eccentric cases are covered by §7's invented benchmark and §12's tests.
  Desktop test models are built inline in their tests, labelled invented.
- The emission goldens gain one reducer carrying the offset. That is
  intentional new content, reviewed separately from the unchanged-leaf
  comparison (DEL-02-01 Design `model-document-0.8.0.md` §5).

## 7. Benchmark W15C (PRD §22.2 rigid-link family)

The frozen references go in `validation/references/w15c_eccentric_reducer`,
following W-15:
- an exact-rational generator that imports, reads and runs no product code;
- closed forms (unit-load virtual work) and an independent exact
  direct-stiffness solve, as two formulations;
- `references.json` and `SHA256SUMS`.

The product binding is `core/product_physics/tests/w15c_eccentric_reducer_benchmarks.rs`.
Every value is invented. The values below were hand-computed and agree with
Codex's independent values. **They are frozen only when the generator's two
formulations agree with each other and with these.**

### 7.1 Model (invented)

| Item | Value |
|---|---|
| Nodes | N0 (0, 0, 0) anchored; A (1, 0, 0); B (1.12, −0.05, 0); C (2.12, −0.05, 0) free |
| Members | P1 N0→A, section S1. Body A→B, section S1, mapped by the reducer. P2 B→C, section S2. Each has `y_reference` (0, 1, 0). |
| Reducer | at A, `mechanics_geometry_only`, scale s with a source reference, `eccentric_offset` {0, −50, 0, mm} (option A; entered in mm to exercise normalization), `rigid_body_length` 0.12 m |
| Run direction | P1 at A and P2 at B both give (1, 0, 0) and agree. D = (0.12, −0.05, 0), D·t = 0.12, D⊥ = (0, −0.05, 0) = e, so the residual is 0. |
| Body geometry | ‖D‖ = 0.13 m, D̂ = (12/13, −5/13, 0). This is a 5-12-13 triangle, so every direction cosine is rational. Body local axes: x = D̂, y = (5/13, 12/13, 0), z = (0, 0, 1). |
| Material | E = 200 GPa, ν = 1/4, G = 80 GPa, α = 12e-6 1/K |
| S1 | OD 0.2 m, ID 0.18 m: A₁ = 0.0019π m², I₁ = 8.5975e-6π m⁴, J₁ = 2I₁ |
| S2 | OD 0.1 m, ID 0.09 m: A₂ = 0.000475π m², I₂ = 5.3734375e-7π m⁴, J₂ = 2I₂ |

### 7.2 W15C-P: axial tip force (P·‖D⊥‖ in the upstream run)

The load is Fx = P = 10 000 N at C. The cantilever is statically determinate,
so section actions follow from statics. Motions come from unit-load virtual
work (Euler–Bernoulli; Timoshenko & Gere, and Hibbeler, as W-15 cites). The
moment at a cut X is M_z = (C − X) × F = (0.05 + X_y)·P.

The table gives magnitudes; signs follow W-9's conventions, which the generator
fixes.

| Member | Axial N | Shear V (local y) | M_z |
|---|---|---|---|
| P1 | 10 000 N | 0 | 500 N·m, constant |
| Body | (12/13)P = 9 230.769 230 769… N | (5/13)P = 3 846.153 846 153… N | 500·(1 − ξ) N·m |
| P2 | 10 000 N | 0 | 0 |

**Reactions at N0** (on the structure): Fx = −10 000 N and Mz = −500 N·m;
everything else is 0. They are independent of s.

**Tip motions.** The unit-load fields at C:
- unit x: n = (1, 12/13, 1), m = (0.05, 0.05(1−ξ), 0);
- unit y: n = (0, −5/13, 0), m = (2.12 − x, 1.12 − 0.12ξ, 1 − ξ);
- unit z-moment: m = 1.

Over the body, ∫₀¹(1−ξ)(1.12 − 0.12ξ)dξ = 0.54. Then:
- δx = P[L₁/(EA₁) + (144/169)·L_b/(sEA₁) + L₂/(EA₂)] + P·e²[L₁/(EI₁) + L_b/(3sEI₁)]
- δy = −(60/169)·P·L_b/(sEA₁) + P·|e|·[1.62/(EI₁) + 0.54·L_b/(sEI₁)]
- θz = P·|e|·[L₁/(EI₁) + L_b/(2sEI₁)]

Here L₁ = L₂ = 1 m, L_b = 0.13 m and |e| = ‖D⊥‖ = 0.05 m. The terms at s = 1,
in m or rad:

| Term | Value |
|---|---|
| P·L₁/(EA₁) | 8.3765759522e-6 |
| (144/169)·P·L_b/(EA₁) | 9.2786687471e-7 |
| P·L₂/(EA₂) | 3.3506303809e-5 |
| P·e²·L₁/(EI₁) | 4.6279425150e-6 |
| P·e²·L_b/(3EI₁) | 2.0054417565e-7 |
| −(60/169)·P·L_b/(EA₁) | −3.8661119779e-7 |
| P·|e|·1.62/(EI₁) | 1.4994533749e-4 |
| P·|e|·0.54·L_b/(EI₁) | 6.4976312911e-6 |
| P·|e|·L₁/(EI₁) | 9.2558850301e-5 |
| P·|e|·L_b/(2EI₁) | 6.0163252695e-6 |

| Run (EB) | δx (m) | δy (m) | θz (rad) |
|---|---|---|---|
| s = 1 | 4.7639233326413e-5 | 1.5605635758029e-4 | 9.8575175570147e-5 |
| s = 1e3 | 4.6511950687106e-5 | 1.4995144850708e-4 | 9.2564866625877e-5 |
| rigid limit | 4.6510822276055e-5 | 1.4994533748698e-4 | 9.2558850300608e-5 |

Every motion is q/π with q rational.

**Rigid-link check** (mechanical load, zero body eigenstrain). With the body
rigid:
- u_A = (P·L₁/(EA₁), P·|e|·L₁²/(2EI₁)) and θ_A = P·|e|·L₁/(EI₁);
- u_B = u_A + θ_A ẑ × D = u_A + θ_A·(0.05, 0.12, 0);
- P2 carries no moment: it turns rigidly by θ_A and stretches by P·L₂/(EA₂).

So δx = P·L₁/(EA₁) + 0.05·θ_A + P·L₂/(EA₂), δy = 1.62·θ_A and θz = θ_A. These
equal the s → ∞ row. As in W-15, m(s) = m_rigid + c/s, checked exactly at s and
2s.

**Concentric control W15C-P0.** Use B = (1.12, 0, 0), C = (2.12, 0, 0) and
e = (0, 0, 0). Then δy = θz = 0 as zero rows, and
δx = P[L₁/(EA₁) + 0.12/(sEA₁) + L₂/(EA₂)]:

| Run | δx (m) |
|---|---|
| s = 1 | 4.2888068875290e-5 |
| s = 1e3 | 4.1883884950139e-5 |
| rigid limit | 4.1882879761025e-5 |

### 7.3 W15C-T and W15C-TF: thermal

- **W15C-T (free).** ΔT = 100 K on all members, anchored at N0 only,
  s = 1e3. Then u_B = αΔT·(1.12, −0.05, 0) = (1.344e-3, −6.0e-5, 0) m and
  u_C = (2.544e-3, −6.0e-5, 0) m. All rotations, reactions and section
  actions are 0. The force zero scale is s·E·A₁·αΔT. This checks that the
  offset grows with the body at every scale, which is where a rigid link
  would differ (§3.1).
- **W15C-TF (both ends anchored).** Anchored at N0 and C, ΔT = 100 K,
  s = 1 and 1e3. The case is indeterminate to degree 3 in plane. The
  generator gives the values by the force method and cross-checks them by
  direct stiffness. No hand value is given.

### 7.4 W15C-Z: z-offset with a transverse force (torsion)

The geometry is §7.1's with the offset in z: B = (1.12, 0, −0.05),
C = (2.12, 0, −0.05), and e = {0, 0, −50, mm}. Body local axes with
`y_reference` (0, 1, 0): x = (12/13, 0, −5/13), y = (0, 1, 0),
z = (5/13, 0, 12/13). The load is Fy = Q = 1 000 N at C.

**Statics.** For a cut X on P1, the moment of the load is
(C − X) × F = (0.05·Q, 0, (2.12 − x)·Q). So:
- P1 carries torsion T = Q·‖D⊥‖ = 50 N·m, constant, and bending
  M_z = (2.12 − x)·Q;
- the reactions at N0 are Fy = −1 000 N, Mx = −50 N·m and
  Mz = −2 120 N·m, independent of s;
- on the body, the transmitted force (0, Q, 0) is pure local-y shear, and the
  moment vector resolves into torsion and bending about the body's local
  axes, varying linearly along the chord;
- P2 carries bending only.

The motions (u_y, θx, θz at C, at s = 1 and 1e3, EB and Timoshenko) come from
the generator's two formulations. This case exercises J and the torsion path
that the eccentricity creates.

### 7.5 Convention-dependent rows (R-c1; separate tests)

Under option A these rows must solve identically to W15C-P, and the check
must accept them with residual 0:
- **W15C-R, reversed member.** §7.1 with the body defined B→A (`from` B,
  `to` A) and the reducer at A. Option A: e = (0, −50, 0) mm, unchanged.
  Option B: e = (0, +50, 0) mm.
- **W15C-O, opposite end.** §7.1 with the reducer at B, the same body.
  Option A: e = (0, +50, 0) mm, measured from B. Option B: e = (0, −50, 0) mm,
  from the member's `from`.

Each row's refused counterpart (the other option's sign) is refused by
`..._MISMATCH` with a 0.1 m residual. These rows and their expected signs are
the only benchmark content a B ruling changes.

### 7.6 Runs and criterion

| Case | Routes |
|---|---|
| W15C-P, -P0, -TF, -Z, -R, -O, Euler–Bernoulli | PF; EX (`3.0.0/exact_pressure_v3`, `pressure_regions: []`) |
| W15C-P, -TF, -Z, Timoshenko | EX (`timoshenko_annulus_energy_v1`) |
| W15C-T | PF (element thermal); EX (load/reference-state group) |

Both solver modes run against the same reference. s = 1 and 1e3 both lie below
the precision threshold (W-15 §2.5). The criterion is DEC-024/DEC-026's:
- a nonzero row passes when |obs − exp| ≤ 1e-9·|exp|;
- a zero row passes when |obs| ≤ 1e-9·zero_scale.

Each run also asserts the `..._REALIZED` info, and the scaled body's
`component_body` record.

## 8. D3 (retained-memory pricing)

Re-pricing and re-registration are needed. They run in one D3 turn with the
0.8.0 ripple, after #1625, #1630 and #1615 (Agent 0, 2026-10-26).

**Priced files touched by the offset:**
- PP `lib.rs`: the wire field on `ComponentGeometryInput` (~507) with #1630's
  closed deserializer, its normalization (~9559), and the call after
  `normalize_model_units`;
- PP `rigid_component.rs`, pinned by `w15_owners_model.py`: the check, the
  run-direction classifier and the info;
- PP `model_fields.rs` (#1630): the declared key and the nested scan. The
  key-loop bound rises 31 → 32, the component geometry's field count.

The version ripple's files are DEL-02-01 Design `model-document-0.8.0.md` §7.
They include `retained_wire.rs` and `pressure_runtime.rs` for review.

**What is expected to move:**
- **The check is designed for zero incremental allocation when nothing
  applies.** The first pass is an allocation-free scan for an applicable
  reducer (kind, offset present, mode not `not_solver_consumed`), and it
  returns before any pipe map, adjacency index or diagnostic text is built.
  D1 admits no component (P2), so every priced point takes that return.
- **TEXT sites.** The refusal and info texts are ordinary-route TEXT sites,
  priced like W-15's (§8.21).
- **The wire field.** Its size is a per-component cost, zero on D1 by premise.
  The registration states that premise.
- **`identical_but_basis`** for the token and the offset is a hypothesis. It
  is verified by `compare` on the final merged source, not assumed.
- **Bound and fit.** P_FINAL is unchanged. Every phase must fit M = 12 GiB. A
  misfit stops for the owner (decision 22).
- **Witnesses.** The AC-004 stack witnesses run under an exclusive lane
  granted by Agent 0.

## 9. Precision and detail matters

Each entry is HELP_HUMAN, 2026-10-26, under the owner's delegation (LOOP_INIT,
owner 2026-10-26: engineering judgment for matters of precision or detail).
Resolutions that change numerical meaning carry §12's regression checks and
independent scrutiny.

| # | Matter | Resolution |
|---|---|---|
| P1 | Body realization | Approved as Codex says. The chord member between the two centerlines (§3.1). It is equivalent to a rigid link mechanically and kinematically only with zero body eigenstrain and equivalent loading. Thermal growth stays αΔT·D, offset growth included, at every scale. Load resultants (chord self-weight over ‖D‖) differ unless matched. |
| P2 | Tolerance and transversality | Revised. The actual node geometry governs the lever arm and the solve. The declared offset is checked against it within τ (the 5 mm node-proximity tolerance, user-settable), and transversality is enforced separately by |e·t| ≤ 1e-3·‖e‖ (§3.2). Every M = P·e claim means P·‖D⊥‖. Regressions: 47 mm declared against 50 mm geometry passes placement, and the solve is that of the 50 mm geometry. The collinear e = (5, 0, 0) mm on concentric geometry is refused by `..._NOT_TRANSVERSE` (§12). ρ = 1e-3 accepts declared values entered to about four significant figures on inclined runs, and refuses any axial share above 0.1 %. |
| P3 | Run direction | Revised (§3.3). Pipe-run evidence is kept apart from other component-body chords, and a neighbouring eccentric body's chord is not a run. Realized bend tangents are used where they apply. Where both ends give a direction, the two must agree within 1e-3 rad. Ambiguous or unsupported topology is refused by name: a tee or branch, a standalone body, unrealized bend members, connector spans, and two eccentric reducers in series. |
| P4 | Body length against geometry | Approved: not checked for any rigid-family kind. The demo valve's body length is 0.72 m on a 2.2 m member. |
| P5 | Self-weight of the chord body | Approved: it acts over ‖D‖, and slice b's component weight replaces it when entered. |
| P6 | `not_solver_consumed` | Approved: honoured, the offset not read and no offset diagnostic. Ordinary structural and unit validation still applies. |
| P7 | Offset on a non-reducer | Approved: blocking `..._INVALID` (`not_reducer`). |
| P8 | Present zero vector | Approved: checked as concentric under the revised rules. Transversality passes trivially, and placement requires ‖D⊥‖ ≤ τ. |
| P9 | D·t ≤ 0 | Approved: blocking `..._INVALID` (`no_axial_length`). |
| P10 | Thermal growth of the offset | Approved: αΔT·D, offset included. |
| P11 | Stress rows of an eccentric reducer | Approved, with disclosure. Rows are the reference pipe section's along the chord. Scaled rows carry `component_body`. The reference-section meaning is disclosed in the info, the UI and the report text in both the scaled and unscaled cases (§3.4, §4). |
| P12 | Version bump while #1630 closes the record | Approved: 0.8.0 (DEL-02-01 Design `model-document-0.8.0.md` §1). |

## 10. Reserved for the owner, and ScopeOfWork drafts

**R-c1: the offset's coordinate convention.** Agent 0 sends this to the owner
with recommendation A (2026-10-26). DEL-03-05 ScopeOfWork line 312 reserves
"coordinate conventions other than the centre-of-gravity convention ruled
under R10" to human/project authority, and AC-001 preserves "unresolved
coordinate ... decisions". The options:
- **A (recommended):** global X/Y/Z, from the centerline at the component's
  node to the centerline at the body's other end, in model length units. It
  mirrors R10, and reversing the member does not flip e.
- **B:** global X/Y/Z, from the mapped member's `from` end to its `to` end.
  Reversing the member flips e.

**Consequence.** The build needs the ruling. Under B, the changes are confined
to §3.6's function and text, §7.5's rows and the convention tests.

**ScopeOfWork drafts (DEL-03-05)**, applied after the ruling, with it quoted:
1. **CLM-003 slot table.** Insert after line 58 (the "Center of gravity
   input" row):
   - **New:** `> | Eccentric offset input | PRD §12.3; R12 | `eccentric_offset` on a reducer is a vector in the model's global X/Y/Z axes, from the centerline at the component's node to the centerline at its body's other end, in model length units (owner ruling R12, <date>: "<quote>"). The solve checks it against the node geometry, which governs, and does not place nodes (Design `eccentric-reducer-offset.md`). |`
2. **CLM-010 requirements.** Insert after line 144 (R10):
   - **New:** `> | R12 | Define the eccentric reducer offset's coordinate convention and reference frame before offset values are accepted for mechanics use. | PRD §12.3; SOW-009; OPS-K-UNIT-1 | Owner ruling, <date>: "<quote>". The convention is global X/Y/Z axes, from the centerline at the component's node to the centerline at its body's other end, in model length units (Design `eccentric-reducer-offset.md` §2, §3.6). Tests check accepted and refused offsets, the convention rows and an independent eccentric-step benchmark on both routes. |`
   - Under option B the convention clause in both rows reads "from the mapped member's `from` end to its `to` end".
3. **Line 312.**
   - **Old:** `Human/project authority must still decide coordinate conventions other than the centre-of-gravity convention ruled under R10, accepted source catalogs,`
   - **New:** `Human/project authority must still decide coordinate conventions other than the centre-of-gravity convention ruled under R10 and the eccentric reducer offset convention ruled under R12, accepted source catalogs,`
4. **Line 64**, reconciliation once built:
   - **Old:** `solver consumption beyond W-15 slices a, b and d,`
   - **New:** `solver consumption beyond W-15 slices a, b, c and d,`
   - Line 49's "Current implementation" paragraph gains one sentence; that is
     the manager's reconciliation text.

No acceptance criterion changes. AC-001 holds as written.

## 11. `apps/desktop/native-smoke/expected.json`

No value is expected to move:
- the demo gains no reducer (§6);
- the version token moves no row, probe value or standing;
- the file pins no version or hash;
- its newer-version case uses 9.0.0.

A native smoke run on the build head confirms this and reports the counts. If
any value moves, it is reported to the owner and not edited.

## 12. Tests and regression list

**Numerical (independent):**
- W15C-P, -P0, -T, -TF, -Z, -R and -O against
  `validation/references/w15c_eccentric_reducer` (§7), on PF and EX, EB and
  Timoshenko, both solver modes, at s = 1 and 1e3;
- the rigid-limit identity;
- agreement of the generator's two formulations.

**Check behaviour** (`core/product_physics/tests/w15c_eccentric_reducer.rs`):
- **Accepted:**
  - the §7.1 geometry;
  - e entered in mm and in m, with bit-identical results;
  - a zero e on collinear nodes;
  - **declared 47 mm against 50 mm geometry:** placement residual 3 mm ≤ τ.
    The results are bit-identical to the same model without the offset
    member and to the W15C-P rows, and the info states declared 47 mm and
    realized 50 mm.
- **`..._NOT_TRANSVERSE`:**
  - the collinear e = (5, 0, 0) mm on concentric geometry, whose placement
    residual of exactly 5 mm alone would pass;
  - an axial share just above 1e-3, with the share just below it accepted.
- **`..._MISMATCH`:**
  - sign flipped (residual 0.1 m);
  - wrong transverse axis (0.0707 m);
  - magnitude 40 mm (10 mm);
  - the 47 mm case with τ set to 1 mm through `model_settings`.
- **`..._INVALID`,** one test per reason in §3.4's order:
  - an offset on a valve;
  - a non-finite converted value;
  - a mapping that is absent, unknown or not ending at the node;
  - a tee at N;
  - a neighbouring eccentric reducer at N;
  - an unrealized bend member at N;
  - a connector span at N;
  - a standalone body;
  - run directions at N and F 2e-3 rad apart (refused), with 0.5e-3 rad
    accepted;
  - D·t ≤ 0.
- **Run evidence:**
  - a realized arc at N supplies its end tangent;
  - a concentric flange body at both ends is accepted;
  - a terminal end uses the other end's direction.
- **Other behaviour:**
  - `not_solver_consumed` with a contradictory offset gives no offset
    diagnostic and the same solve;
  - ordinary validation still reports a malformed unit;
  - the info appears only on a published result;
  - an unscaled eccentric reducer solves as chord pipe, and its info states
    the reference-section meaning;
  - the early return builds no index when no reducer applies (a unit test on
    the first pass).

**Convention** (`tests/w15c_offset_convention.rs` and one vitest): W15C-R and
-O accepted, and their other-option signs refused.

**Exact route** (`tests/exact_admission_seam.rs`): an eccentric reducer outside
every region is admitted; inside a region it is refused.

**Closed record** (#1630, `silent_drop_guard_runtime.rs`):
- an unknown vector member (`{x, y, z, unit, w}`) is refused as
  `UNKNOWN_MODEL_FIELD`, naming `model.components[i].geometry.eccentric_offset`;
- a malformed shape (missing `z`, or a string) is refused;
- absence gives `None`;
- `null` behaves as `center_of_gravity`'s `null` does (absent at the wire;
  the schema refuses it), asserted side by side.

**Authoring and adoption:**
- the applier sets, edits and clears the vector, with exact before, after and
  inverse; it refuses the field on non-reducers and refuses non-length units;
- the descriptor checks in TS and Rust;
- library apply keep, group and all on a reducer with an offset;
- the inspector and creation vitests, with the convention text;
- the report provenance row, with the offset, the convention and the
  reference-section disclosure;
- geometry operations carry the offset wherever they carry the centre of
  gravity.

**Format:** DEL-02-01 Design `model-document-0.8.0.md` §12.

**Regression:**
- the W-15 and W-15b suites are unchanged;
- the demo and preview fixtures change by token and hash only;
- `test_component_section_schema.py`;
- the checks `select_affected_checks.py` selects;
- the native smoke compared with main.

**Scrutiny:** independent numerical scrutiny of the benchmark and the check,
and independent review of the D3 registration.

## 13. Reconciliation at build

- **DEL-03-05:** this Design's "as built" notes, `deliverable.yaml` code paths
  and checks, and §10's drafts after R-c1.
- **DEL-02-01, DEL-02-05, DEL-04-08 and DEL-03-01:** through
  `model-document-0.8.0.md` §10.
- **`docs/COMPLETENESS_MAP.md`:** the rows §12.3 reducers, FR-SOL-004/012 and
  W-15 c.
- **#1648's files:** Agent 0 is told before any of them is touched.
