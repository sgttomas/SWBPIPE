# Eccentric reducer offset (W-15 slice c)

This design realizes the "eccentric offset vector" of PRD §12.3 (Reducers) for
the reducer component. It is W-15 slice c in `docs/COMPLETENESS_MAP.md` §4.3,
in the owner's approved W-15 scope: OD-2 (owner, 2026-10-23, "I approve OD-1
through OD-5 as you recommended") selected DEC-045 stiffness scaling for rigid
and semi-rigid components, and W-15 slice c carries the reducer's offset. It
replaces the outline in `rigid-component-stiffness-scaling.md` §4. The model
document part, version 0.8.0, is DEL-02-01 Design
[`model-document-0.8.0.md`](../../../PKG-02/DEL-02-01/Design/model-document-0.8.0.md).

**Status.** Design pass at main `2d4b1bb4bb` (2026-10-26). Nothing here is built.
One point is reserved for the owner (§10, R-c1). The engineering-judgment points
for Agent 0 are in §9. Standard claim fence applies (F-PIP-2; claims taxonomy
per DEC-081): the benchmark is a candidate reference, and no output here is
engineering approval.

Paths are relative to `projects/chirality-piping/`. PP is
`core/product_physics/src`.

## 1. What it is

An eccentric reducer joins two pipes of different size whose centerlines are
parallel but not collinear: one side of the body is flat, so the small end's
centerline is offset from the large end's. The offset e is the vector from one
end's centerline to the other's, perpendicular to the run. A concentric
reducer has e = 0.

In a centerline model each node already sits on its own pipe's centerline. An
eccentric reducer's two end nodes therefore differ by the axial run plus e.
Its mechanical effect follows from that geometry. An axial force P carried
through the step acts on a line offset by e from the upstream centerline, so
the upstream pipe carries the moment P·e. Thermal growth of the body moves the
downstream run along and across the axis.

## 2. The authored member

| Item | Rule |
|---|---|
| Name | `geometry.eccentric_offset`, on the component's `geometry` record |
| Shape | the existing `VectorQuantity`, `{x, y, z, unit}`. Each component is a finite number. `unit` is a length unit of the units catalog. |
| Frame | the model's global X/Y/Z axes, the frame of node positions |
| Origin and sign (R-c1, §10) | Recommended (option A): from the centerline at the component's `node` to the centerline at the mapped body member's other end. This mirrors the R10 centre-of-gravity convention (measured from the component's node). |
| Kind | `reducer` only. On any other kind it is refused by name (§3.4). |
| Absence | Absent means no declared offset. The reducer is then taken as authored, and no offset check runs. A present zero vector declares a concentric reducer and is checked. |
| Default | none. The product never writes the member and never supplies a value (CLM-004). |
| Units | normalized to metres with the other component geometry (PP `lib.rs` `normalize_model_units`, beside `center_of_gravity` at ~9559) |

The body length `geometry.rigid_body_length` and the end sizes stay
descriptive, as for every rigid-family kind. The solve does not compare them
with the node geometry (§9, P4).

## 3. How it enters the solve

### 3.1 Mechanism: the existing body member between the two centerlines

There is no new element and no rigid offset link. The reducer's body is its
mapped straight member (W-15 §1), and the member's end nodes are the two
centerline points. The member runs along the chord D = x_far − x_node between
them. Its length is ‖D‖ and it is inclined to the run by atan(‖D⊥‖ / (D·t)).
Both routes form it in the one shared function, `build_model_for_members` (PP
`lib.rs` ~8115), with the existing direction cosines and `y_reference`.

Why this mechanism:
- **It exists on both routes.** Offsets exist only on the exact route's
  objective connectors and tie rods (`frame_kernel/src/connector.rs`,
  `tie_rod.rs`), and the pressure-free route refuses connectors. The straight
  member is the only formation shared by both routes. Using it touches no
  frame-kernel file.
- **It gives the eccentric step's statics exactly.** The member joins the two
  centerlines, so an axial force P through the step has the lever e about the
  upstream centerline. The upstream pipe carries M = P·e. No load is added.
- **Its rigid limit is the rigid offset link.** With DEC-045's scale s → ∞ the
  body's compliance vanishes. End B then moves as a rigid body with end A:
  u_B = u_A + θ_A × D. This is the kinematics of a rigid offset link from A to
  B (§7.3 checks it).
- **It differs from a rigid link only in a semi-rigid body's compliance.** At
  finite s the body bends and stretches along the chord rather than along the
  axis. That is the DEC-045 approximation the product already states (a scaled
  reference section, W-15 §2.4) (§9, P1).

The offset member itself enters the solve only through a consistency check.
It does not place, move or re-form anything.

### 3.2 The consistency check

The check is in PP `rigid_component.rs`. It runs on every route after unit
normalization, so nodes, offset and tolerance are all in metres. It sits
immediately after `normalize_model_units` and its blocking return (PP `lib.rs`
~2805), before formation. It applies to each `reducer` that carries
`eccentric_offset`, unless the reducer's `solver_consumption` is
`not_solver_consumed`. That mode is honoured, as in W-15 §2.3: the member is
not read and no diagnostic is stated (§9, P6).

1. **Ends.** N is the component's `node`. The mapped member is
   `geometry.rigid_pipe_ref`, which must exist and end at N (W-15's mapping
   rule). F is the member's other end. D = x_F − x_N.
2. **Run axis t.** t is the unit direction of the one straight member, other
   than the body, that ends at N. If N has none or more than one, t comes from
   the one such member at F. A realized curved bend member and a span replaced
   by an objective connector are not candidates: a chord is not a tangent.
   t is oriented so that D·t ≥ 0.
3. **Requirements:**
   - D·t > 0: the body has length along the run;
   - ‖D⊥ − e‖ ≤ τ, where D⊥ = D − (D·t)·t is the transverse separation of
     the two centerlines and τ is the model's node-proximity tolerance
     (`PreviewModel::node_proximity_tolerance_m`: 5 mm unless
     `model_settings.node_proximity_tolerance` sets another). Since D⊥ ⟂ t,
     this bounds e·t by τ as well, so no separate perpendicularity test is
     needed.
4. **Arithmetic.** Each component is formed in binary64 from the normalized
   coordinates. The comparison is `≤` on the Euclidean norm.

The check uses no body length and no section. It does not test that the far
end's run is parallel to t (§9, P3).

### 3.3 Effects

| Effect | Treatment |
|---|---|
| Stiffness | The body member A→B along D, with its section's s·K when the W-15 scale is eligible and plain K otherwise. Nothing else changes. |
| Statics | An axial force through the step gives M = P·e in the upstream run. The body carries N = P·(D̂·t̂), a shear P·‖D⊥‖/‖D‖, and a moment falling linearly from P·‖e‖ at N to 0 at F (§7). |
| Self-weight | The member's distributed self-weight acts over ‖D‖, not over `rigid_body_length` (§9, P5). Slice b replaces it when the component's weight is generated (W-15 §3). |
| Component weight | Slice b's wrench at the component's node, with the user's centre of gravity (R10). It is independent of the offset. |
| Thermal | The body's free growth is αΔT·D, an isotropic growth of the whole body, offset included. A scaled body keeps that growth (W-15 §2.1). |
| Pressure | PF: no pressure. EX: a reducer whose member lies in a pressure region is refused (`EXACT_PRESSURE_REDUCER_IN_REGION`, W-15 §2.7). An eccentric reducer is no exception. Outside every region it is admitted like a valve. |
| Other element loads | As authored on the body member, over its length ‖D‖ |
| Reactions, displacements | Unchanged meaning. The node motions are the centerline motions at N and F. |
| Stress rows | Rows of the mapped pipe section along the chord. A scaled body's rows carry slice d's `component_body` record and are excluded from the headline (W-15 §2.4, §5). An unscaled reducer's rows are ordinary pipe rows, as for any unscaled rigid-family member today (§9, P11). |

### 3.4 Diagnostics

| Code | Severity | When |
|---|---|---|
| `RIGID_COMPONENT_ECCENTRIC_OFFSET_INVALID` (new) | blocking | A present offset cannot be checked. The message names the component and the first reason, in this order: the kind is not `reducer`; a component of the offset is not finite after conversion; the mapping is invalid; the run axis is undefined (no unique straight member at either end); D·t ≤ 0. |
| `RIGID_COMPONENT_ECCENTRIC_OFFSET_MISMATCH` (new) | blocking | ‖D⊥ − e‖ > τ. The message names the component, N, F, e and D⊥ in metres, the residual and τ. |
| `RIGID_COMPONENT_ECCENTRIC_OFFSET_REALIZED` (new) | info | On a published result only, like slice a's applied-scale info (PP `rigid_component.rs` `append_formed_diagnostics`). It states the offset as entered and D⊥, and that the eccentricity is carried by the body member between the two centerlines (its length and inclination). It also states that no rigid offset link is added. |

The check and the info take the component order of the model and allocate
nothing when no reducer carries an offset.

### 3.5 The exact route and the retained route

The exact route forms the same member in the same function, with
Euler–Bernoulli or `timoshenko_annulus_energy_v1` (PP `pressure_runtime.rs`
~49). Under Timoshenko the body's shear P·‖D⊥‖/‖D‖ adds its shear compliance;
§7's generator includes it. The reducer-in-region refusal is unchanged. The
retained and source sub-routes still refuse every component (W-15 §2.7), so
they never meet an offset.

### 3.6 Not done in slice c

These are recorded as remaining items, not built:
- a rigid offset link element;
- a desktop "place end F from N, t and e" helper. The user positions nodes
  with the existing node edits, and the check reports any disagreement. This
  replaces the outline's "the desktop can place B from A, t, L and e";
- a body-length check against the geometry;
- an `eccentric_offset` field kind in the component library schema
  (`schemas/component.schema.yaml` `ComponentFieldKind`) and in library
  apply;
- reducer SIF or other local modifiers (PRD §12.3 "optional user-supplied
  local modifiers").

## 4. Authoring

- **Applier.** A field rule `geometry.eccentric_offset` of kind
  `OptionalVectorQuantity { dimension: "length" }`, beside `center_of_gravity`
  (`operation_applier/src/lib.rs` ~652). `resolve_field` refuses it on any kind
  other than `reducer`, as it refuses `solver_consumption` on non-rigid kinds.
  The before value and equality follow the centre-of-gravity edit (W-15 §3,
  "Applier"). Reducer creation accepts an optional exact `{x, y, z, unit}`; it
  is never defaulted.
- **Geometry operations.** Wherever an operation carries a component's
  centre-of-gravity vector, it carries the offset the same way. The
  attachment refusals that block moving a component's nodes are unchanged.
- **Desktop.** Reducer creation (`componentIntent.ts`, `RigidDraftFields.tsx`)
  and the property inspector (`PropertyInspector.tsx`) offer the offset with
  the centre-of-gravity vector editor, on reducers only. The text states the
  convention: "global X/Y/Z, from this component's node to the other end of
  its body". The solve's findings are shown in the diagnostics list as usual.
- **Document validator.** The validator checks the slot's shape only, through
  the schema conformance rules. The geometry check is the solve's, on both
  routes.

## 5. The model document

The member is new on the closed component geometry record
(`schemas/model.schema.yaml` ~355). It arrives in model document 0.8.0 under
the one-current-version rule: 0.7.0 is refused by name, and nothing is
migrated. The version ripple, refusals, persistence and open paths, the D3
chain and the reserved version-token text are in DEL-02-01 Design
[`model-document-0.8.0.md`](../../../PKG-02/DEL-02-01/Design/model-document-0.8.0.md).

## 6. The demo and fixtures

- The bundled demo (`fixtures/product_preview/invented_demo_model.json`) and
  every other committed current document move to 0.8.0 by version token only,
  through their maintained writers (DEL-02-01 Design `model-document-0.8.0.md`
  §5). No physical value moves.
- **The demo does not gain an eccentric reducer.** Adding one would change the
  demo's geometry, results and native-smoke observations, and those are pinned
  by the owner-reserved `expected.json`. The eccentric case is covered by §7's
  invented benchmark and by the tests in §12. A desktop test model is built
  inline in its test, labelled invented.

## 7. Benchmark W15C (PRD §22.2 rigid-link family)

The frozen references go in `validation/references/w15c_eccentric_reducer`,
following W-15 (`validation/references/w15_rigid_scaling`): an exact-rational
generator that imports, reads and runs no product code, closed forms with an
exact direct-stiffness cross-check, `references.json` and `SHA256SUMS`. The
product binding is `core/product_physics/tests/w15c_eccentric_reducer_benchmarks.rs`.
Every value is invented. No equation, value or example comes from protected
content.

### 7.1 Model (invented)

| Item | Value |
|---|---|
| Nodes | N0 (0, 0, 0) anchored; A (1, 0, 0); B (1.12, −0.05, 0); C (2.12, −0.05, 0) free |
| Members | P1 N0→A, section S1. Body A→B, section S1, mapped by the reducer. P2 B→C, section S2. Each has `y_reference` (0, 1, 0). |
| Reducer | at A, `rigid_pipe_ref` the body, `mechanics_geometry_only`, scale s with a source reference, `eccentric_offset` {0, −50, 0, mm} (entered in mm to exercise normalization), `rigid_body_length` 0.12 m |
| Run axis | P1 at A, t = (1, 0, 0). D = (0.12, −0.05, 0), D·t = 0.12, D⊥ = (0, −0.05, 0) = e, so the residual is 0. |
| Body geometry | ‖D‖ = 0.13 m, D̂ = (12/13, −5/13, 0). This is a 5-12-13 triangle, so every direction cosine is rational. |
| Material | E = 200 GPa, ν = 1/4, G = 80 GPa, α = 12e-6 1/K (W-15's) |
| S1 | OD 0.2 m, ID 0.18 m: A₁ = 0.0019π m², I₁ = 8.5975e-6π m⁴ |
| S2 | OD 0.1 m, ID 0.09 m: A₂ = 0.000475π m², I₂ = 5.3734375e-7π m⁴ |

The step's half-diameter difference, (0.2 − 0.1)/2 = 0.05 m, equals |e|. This is
an invented flat-side reducer.

### 7.2 Case W15C-P: axial tip force (the M = P·e case)

Load: Fx = P = 10 000 N at C. The cantilever is statically determinate. The
section actions follow from statics, and the motions from unit-load virtual
work (Euler–Bernoulli; Timoshenko & Gere, *Mechanics of Materials*, and
Hibbeler, *Structural Analysis*, virtual work, as W-15 cites).

**Section actions.** The moment at a cut X is the moment of the tip load about
X: M_z = (C − X) × F = (0.05 + X_y)·P.

The table gives magnitudes. Signs follow W-9's conventions, which the
generator fixes. The body's local axes are x = (12/13, −5/13, 0),
y = (5/13, 12/13, 0) and z = (0, 0, 1).

| Member | Axial N | Shear V (local y) | M_z |
|---|---|---|---|
| P1 | P = 10 000 N | 0 | 0.05·P = 500 N·m, constant |
| Body | (12/13)P = 9 230.769 230 769… N | (5/13)P = 3 846.153 846 153… N | 500·(1 − ξ) N·m, 500 at A, 0 at B |
| P2 | P = 10 000 N | 0 | 0 |

**Reactions at N0** (on the structure): Fx = −10 000 N, Fy = Fz = 0,
Mz = −P·|e| = −500 N·m, other moments 0. These are independent of s.

**Tip motions.** The unit-load fields at C:
- unit x: n = 1, 12/13, 1, and m = 0.05, 0.05(1−ξ), 0;
- unit y: n = 0, −5/13, 0, and m = 2.12 − x, 1.12 − 0.12ξ, 1 − ξ;
- unit z-moment: n = 0, and m = 1.

Then:
- δx = P[L₁/(EA₁) + (144/169)·L_b/(sEA₁) + L₂/(EA₂)] + P·e²[L₁/(EI₁) + L_b/(3sEI₁)]
- δy = −(60/169)·P·L_b/(sEA₁) + P·|e|·[1.62/(EI₁) + 0.6·L_b/(sEI₁)]
- θz = P·|e|·[L₁/(EI₁) + L_b/(2sEI₁)]

Here L₁ = L₂ = 1 m and L_b = 0.13 m. The terms at s = 1, in m or rad:

| Term | Value |
|---|---|
| P·L₁/(EA₁) | 8.3765759522e-6 |
| (144/169)·P·L_b/(EA₁) | 9.2786687471e-7 |
| P·L₂/(EA₂) | 3.3506303809e-5 |
| P·e²·L₁/(EI₁) | 4.6279425150e-6 |
| P·e²·L_b/(3EI₁) | 2.0054417565e-7 |
| −(60/169)·P·L_b/(EA₁) | −3.8661119779e-7 |
| P·|e|·1.62/(EI₁) | 1.4994533749e-4 |
| P·|e|·0.6·L_b/(EI₁) | 7.2195903234e-6 |
| P·|e|·L₁/(EI₁) | 9.2558850301e-5 |
| P·|e|·L_b/(2EI₁) | 6.0163252695e-6 |

| Run | δx (m) | δy (m) | θz (rad) |
|---|---|---|---|
| s = 1 | 4.7639233326e-5 | 1.5677831661e-4 | 9.8575175570e-5 |
| s = 1e3 | 4.6511950687e-5 | 1.4995217047e-4 | 9.2564866626e-5 |
| rigid limit (s → ∞) | 4.6510822276e-5 | 1.4994533749e-4 | 9.2558850301e-5 |

Exact forms: every motion is q/π with q rational. For example, δx at s = 1 is
(62728/419128125)/π m.

**Rigid-link check.** With the body rigid, u_A = (P·L₁/(EA₁), P·|e|·L₁²/(2EI₁))
and θ_A = P·|e|·L₁/(EI₁). B moves as u_B = u_A + θ_A ẑ × D, which adds
θ_A·(0.05, 0.12, 0). P2 carries no moment, so it turns rigidly by θ_A and
stretches by P·L₂/(EA₂). This gives:
- δx = P·L₁/(EA₁) + 0.05·θ_A + P·L₂/(EA₂) = 4.6510822276e-5 m;
- δy = (0.5 + 0.12 + 1.0)·θ_A = 1.62·θ_A = 1.4994533749e-4 m;
- θz = θ_A = 9.2558850301e-5 rad.

These equal the s → ∞ limit above. As
in W-15, each motion satisfies m(s) = m_rigid + c/s with c independent of s.
The generator checks this exactly at s and 2s.

**Concentric control (W15C-P0).** The same model with B = (1.12, 0, 0),
C = (2.12, 0, 0) and e = (0, 0, 0). Then δy = θz = 0 (zero rows) and
δx = P[L₁/(EA₁) + 0.12/(sEA₁) + L₂/(EA₂)]. The difference between the two
cases is the offset's effect: at s = 1e3, δy is 1.4995e-4 m against 0.

### 7.3 Case W15C-T: free thermal growth

ΔT = 100 K on all three members, anchored at N0 only, s = 1e3. Free growth
gives the motions u_B = αΔT·(1.12, −0.05, 0) = (1.344e-3, −6.0e-5, 0) m and
u_C = αΔT·(2.12, −0.05, 0) = (2.544e-3, −6.0e-5, 0) m. All rotations are 0,
and so are all reactions and section actions. The zero scale of the forces is
the body's held force s·E·A₁·αΔT, as in W15-T-FREE. This checks that the
offset grows with the body. PF uses element `thermal` primitives; EX uses the
load/reference-state member pair.

### 7.4 Case W15C-TF: thermal, both ends anchored

The same model anchored at N0 and C, ΔT = 100 K, s = 1 and 1e3. This case is
statically indeterminate to degree 3 in plane. The generator gives the
reactions and motions by the force method, with an exact direct-stiffness
cross-check. No hand value is given here; the frozen reference is the
generator's.

### 7.5 Runs and criterion

| Case | Routes |
|---|---|
| W15C-P, -P0, -TF Euler–Bernoulli | PF; EX (`3.0.0/exact_pressure_v3`, `pressure_regions: []`) |
| W15C-P, -TF Timoshenko | EX (`timoshenko_annulus_energy_v1`) |
| W15C-T | PF (element thermal); EX (load/reference-state group) |

Both solver modes run against the same reference. s = 1 and 1e3 both lie below
the precision threshold (W-15 §2.5, 1e5). The criterion is DEC-024/DEC-026's:
a nonzero row passes when |obs − exp| ≤ 1e-9·|exp|, and a zero row when
|obs| ≤ 1e-9·zero_scale. A run also asserts the `..._REALIZED` info once, and
the scaled body's `component_body` record.

## 8. D3 (retained-memory pricing)

Re-pricing and re-registration are needed. They run in the solver lane's D3
queue, in one turn with the 0.8.0 ripple.

Priced files touched (`tools/retained_pricing/base/crate_dirs.txt`):
- **For the offset (this design):**
  - PP `lib.rs`: the wire field on `ComponentGeometryInput` (~507), its
    normalization (~9559) and the call after `normalize_model_units`;
  - PP `rigid_component.rs`: the check and the info. It is pinned by
    `w15_owners_model.py`.
- **For the version ripple** (DEL-02-01 Design `model-document-0.8.0.md` §7):
  - PP `model_format.rs`;
  - `result_export` `physics_source.rs` and `retained_precision.rs`, which
    hold version literals;
  - `semantic_contract.rs`;
  - `retained_memory.rs`, for the reviewed-input digests and registration;
  - comments in `exact_admission.rs`.
- **If #1630 lands first:** PP `model_fields.rs`. The component geometry's
  declared key set gains `eccentric_offset`, and its `VectorQuantity` test set
  gains it too. The scan's key-loop bound rises by one (31 → 32, the component
  geometry's field count; #1630's SD-NR).

What is expected to move:
- **D1 admits no component (P2).** The new check and info therefore run over
  zero reducers on every priced point. Their TEXT sites are ordinary-route
  diagnostics, priced like W-15's (§8.21).
- **The wire.** The added `Option<VectorQuantity>` enlarges the component
  record. On D1 there is no component, so its priced contribution is zero by
  premise. The registration states this premise rather than assuming it.
- **The version token.** It has the same length as before ("0.7.0" →
  "0.8.0"), so no byte-length literal moves. The reviewed-input digests and
  the DEF-E-bound successor hashes move. W-13 S1b's token epoch priced
  `identical_but_basis`, and the same is expected for the token here.
- **Bound and fit.** P_FINAL is unchanged. Every phase must still fit
  M = 12 GiB. The tightest phase at §8.21 had +2,364,000,893 B of headroom, and
  a misfit stops for the owner (decision 22).
- **Witnesses.** The AC-004 stack witnesses run under an exclusive lane
  granted by Agent 0.

## 9. Precision and detail matters (Agent 0, engineering judgment)

Each is a practical resolution within normal centerline-model practice.
Resolutions that change numerical meaning carry §12's regression checks and
independent scrutiny.

| # | Matter | Recommended resolution | Reason |
|---|---|---|---|
| P1 | Body realization: a chord member, or an axial member plus a rigid offset link | The chord member between the two centerlines (§3.1) | It exists on both routes, gives exact statics and the exact rigid limit, and adds no element. It differs only in the distribution of a semi-rigid body's compliance, within DEC-045's stated approximation. |
| P2 | Check tolerance | The model's node-proximity tolerance τ (5 mm default, user-settable), on ‖D⊥ − e‖ in metres | It is the model's own length tolerance for node placement, which the user already controls. |
| P3 | Source of the run axis | The unique other straight member at the component's node, else at the far end; refuse when neither exists. No parallelism test of the far run. | The outline's relation x_B − x_A = L·t + e with t = d/‖d‖ checks e's magnitude only through |e|/L and leaves its direction about D unchecked. With L = 0.12 m and |e| = 0.05 m, a 10 mm error in |e| passes a 5 mm tolerance. A geometric run axis checks every component of e to first order. |
| P4 | Body length against geometry | Not checked, for any rigid-family kind | The check does not need it. Checking it would newly refuse models: the demo valve's body length is 0.72 m on a 2.2 m member. |
| P5 | Self-weight of a chord body without component weight | Over ‖D‖ (0.13 m against 0.12 m in §7) | It is the member's own weight model. Slice b's component weight replaces it when entered. The info states the body length. |
| P6 | `not_solver_consumed` reducer with an offset | Honoured: not read, no diagnostic. Absent or any other mode is checked. | This is W-15 §2.3's honouring rule. An absent mode assumes nothing about the geometry, and the geometry is solved regardless. |
| P7 | Offset on a non-reducer | Blocking `..._INVALID` | No slot is accepted and silently left unread. |
| P8 | Present zero vector | Checked as concentric (D⊥ within τ of 0) | It is the user's explicit declaration. |
| P9 | D·t ≤ 0 | Blocking `..._INVALID` | The body has no length along the run. |
| P10 | Thermal growth of the offset | αΔT·D, offset included | This is isotropic growth of the body, the member's own free-strain model. |
| P11 | Stress rows of an unscaled eccentric reducer | Ordinary pipe rows along the chord, as for any unscaled rigid-family member. The info states that the eccentricity is carried by the chord member. | This keeps slice d's rule that only an applied scale identifies a body. |
| P12 | Version bump while #1630 closes the record | Bump to 0.8.0 | DEL-02-01 Design `model-document-0.8.0.md` §1 |

## 10. Reserved for the owner, and ScopeOfWork drafts

**R-c1: the offset's coordinate convention.** DEL-03-05 ScopeOfWork line 312
reserves "coordinate conventions other than the centre-of-gravity convention
ruled under R10" to human/project authority, and AC-001 preserves "unresolved
coordinate ... decisions". The offset's frame and origin are such a
convention. The options:
- **A (recommended):** global X/Y/Z, from the centerline at the component's
  node to the centerline at the body's other end, in model length units. This
  mirrors R10. It does not depend on the member's from/to direction, so
  reversing the member does not flip e.
- **B:** global X/Y/Z, from the mapped member's `from` end to its `to` end (the
  outline's "A to B"). Reversing the member flips e.
- **Consequence:** the build needs the choice. Option A is assumed throughout
  this design. Option B changes only §2, §3.2 step 1 and the inspector text.

**ScopeOfWork drafts (DEL-03-05).** These are applied only after R-c1 is
ruled, with the ruling quoted. They record the convention as R10 is recorded.

1. CLM-003 slot table. Insert after line 58 (the "Center of gravity input" row):
   - **New row:** `> | Eccentric offset input | PRD §12.3; R12 | `eccentric_offset` on a reducer is a vector in the model's global X/Y/Z axes, from the centerline at the component's node to the centerline at its body's other end, in model length units (owner ruling R12, <date>: "<quote>"). The solve checks it against the node geometry and does not place nodes (Design `eccentric-reducer-offset.md`). |`
2. CLM-010 requirements. Insert after line 144 (R10):
   - **New row:** `> | R12 | Define the eccentric reducer offset's coordinate convention and reference frame before offset values are accepted for mechanics use. | PRD §12.3; SOW-009; OPS-K-UNIT-1 | Owner ruling, <date>: "<quote>". The convention is global X/Y/Z axes, from the centerline at the component's node to the centerline at its body's other end, in model length units (Design `eccentric-reducer-offset.md` §2). Tests check accepted and refused offsets along each axis and an independent eccentric-step benchmark on both routes. |`
3. Line 312:
   - **Old:** `Human/project authority must still decide coordinate conventions other than the centre-of-gravity convention ruled under R10, accepted source catalogs,`
   - **New:** `Human/project authority must still decide coordinate conventions other than the centre-of-gravity convention ruled under R10 and the eccentric reducer offset convention ruled under R12, accepted source catalogs,`
4. Line 64, reconciliation once built:
   - **Old:** `solver consumption beyond W-15 slices a, b and d,`
   - **New:** `solver consumption beyond W-15 slices a, b, c and d,`
   - The "Current implementation" paragraph at line 49 gains one sentence on
     the offset check. That is the manager's reconciliation text.

No acceptance criterion changes. AC-001 holds as written: a ruled convention
is recorded, not invented.

## 11. `apps/desktop/native-smoke/expected.json`

No value is expected to move. The demo gains no reducer (§6). The version
token moves no row, probe value or standing. The file pins no version or hash,
and its newer-version refusal case uses 9.0.0, which is still newer than 0.8.0
(DEL-02-01 Design `model-document-0.8.0.md` §9). A native smoke run on the
build head confirms this and reports the counts. If any value moves, it is
reported to the owner and not edited.

## 12. Tests and regression list

**Numerical (independent):**
- W15C-P, -P0, -T and -TF against `validation/references/w15c_eccentric_reducer`
  (§7), on PF and EX, Euler–Bernoulli and Timoshenko, both solver modes, at
  s = 1 and 1e3;
- the rigid-limit identity m(s) = m_rigid + c/s, and the generator's
  direct-stiffness cross-check.

**Check behaviour** (`core/product_physics/tests/w15c_eccentric_reducer.rs`):
- accepted: the §7 geometry; e entered in mm and in m giving bit-identical
  results; a present zero vector on collinear nodes; a residual just inside τ
  (e = (0, −47, 0) mm, residual 3 mm);
- refused by `..._MISMATCH`: sign flipped (residual 0.1 m); wrong axis
  (e along z, residual 0.0707 m); magnitude 40 mm (residual 10 mm); the 47 mm
  case with τ set to 1 mm through `model_settings` (the user's tolerance
  governs);
- refused by `..._INVALID`, one test per reason in the order of §3.4:
  - an offset on a valve;
  - a non-finite converted value;
  - no mapped member, an unknown member, or a member not ending at the node;
  - an axis undefined (a body between two anchors with no other member; a tee
    at N with nothing at F);
  - D·t ≤ 0;
- axis fallback: a realized bend at N, so the axis comes from F's run;
- `not_solver_consumed` with a contradictory offset: no diagnostic, and the
  same solve as without the offset;
- the info appears only on a published result, never on a blocked one;
- the scale's eligibility is unaffected by the offset, and an unscaled
  eccentric reducer solves as chord pipe.

**Exact route:** an eccentric reducer outside every region is admitted, and
one inside a region is refused by `EXACT_PRESSURE_REDUCER_IN_REGION`
(`tests/exact_admission_seam.rs`).

**Authoring:**
- the applier sets, edits and clears the vector, with exact before/after and
  inverse; it refuses the field on non-reducers and refuses non-length units;
- the desktop creation and inspector vitests, with the convention text;
- geometry operations carry the offset wherever they carry the centre of
  gravity.

**Format:** DEL-02-01 Design `model-document-0.8.0.md` §12. These cover the
schema conformance of a document with the offset, a 0.7.0 refusal on every
surface, and the leaf comparison of regenerated artifacts.

**Regression:**
- the W-15 and W-15b suites (`w15_rigid_scaling_benchmarks.rs`,
  `w15_component_weight*.rs`, `w15_component_body.rs`) are unchanged;
- the bundled demo and preview fixtures change only by token and hash;
- `exact_admission_seam.rs`;
- `test_component_section_schema.py`;
- the affected checks selected by `select_affected_checks.py`;
- the native smoke compared with main.

**Scrutiny:**
- independent numerical scrutiny of the benchmark and the check;
- independent review of the D3 registration.

## 13. Reconciliation at build

- **DEL-03-05:** this Design's "as built" notes, `deliverable.yaml` code paths
  and checks (the new tests and the reference generator), and the
  ScopeOfWork drafts of §10 after R-c1.
- **DEL-02-01, DEL-02-05, DEL-04-08 and DEL-03-01:** through
  `model-document-0.8.0.md` §10.
- **`docs/COMPLETENESS_MAP.md`:** rows §12.3 reducers, FR-SOL-004/012 and W-15 c.
