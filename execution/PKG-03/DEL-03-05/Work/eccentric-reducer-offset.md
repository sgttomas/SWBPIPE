# Eccentric reducer offset (W-15 slice c)

This design realizes the "eccentric offset vector" of PRD §12.3 (Reducers) for
the reducer component. It is W-15 slice c in `docs/COMPLETENESS_MAP.md` §4.3,
in the owner's approved W-15 scope: OD-2 (owner, 2026-10-23, "I approve OD-1
through OD-5 as you recommended") selected DEC-045 stiffness scaling for rigid
and semi-rigid components, and W-15 slice c carries the reducer's offset. It
replaces the outline formerly in `rigid-component-stiffness-scaling.md` §4. The
model document part, version 0.8.0, is DEL-02-01 Design
[`model-document-0.8.0.md`](../../../PKG-02/DEL-02-01/Work/model-document-0.8.0.md).

**Status.** Design at main `2d4b1bb4bb` (2026-10-26), PR #1649, revised after
Codex's design reviews and numerical scrutiny r1 and r2, with Agent 0's
rulings of 2026-10-26.
- **Owner's ruling** (2026-10-26, verbatim): "R-c1 A, 0.8.0 a2".
  - R-c1 A fixes the offset convention (§2, §3.6, §10).
  - 0.8.0 a2 governs stored work and recorded runs (DEL-02-01 Design
    `model-document-0.8.0.md` §3).
- **Agent 0's resolutions:** the engineering-judgment points are recorded in
  §9.
- **Build status:** the convention-neutral parts and the option-A convention
  are built (§12a). The schema member and the 0.8.0 ripple come last, after
  #1625 and #1630.

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
  means P·‖D⊥‖ of the actual geometry; e agrees with the entered D⊥ within
  the placement allowance (§3.2). That allowance bounds the residual on the
  entered nodes only; it gives no certified bound on the true placement (§9,
  rule B).

## 2. The authored member

| Item | Rule |
|---|---|
| Name | `geometry.eccentric_offset`, on the component's `geometry` record |
| Shape | the existing `VectorQuantity`, `{x, y, z, unit}`. Each component is a finite number. `unit` is a length unit of the units catalog. |
| Frame | the model's global X/Y/Z axes, the frame of node positions |
| Origin and sign | R-c1, ruled A by the owner (2026-10-26, "R-c1 A, 0.8.0 a2"; §10): from the centerline at the component's `node` to the centerline at the mapped body member's other end. The convention is isolated in one place (§3.6). |
| Kind | `reducer` only; on any other kind it is refused by name (§3.4) |
| Absence | Absent means no declaration: no offset check runs, and the reducer solves as its nodes place it. An absent offset does not declare a concentric reducer (§3.2), and an undeclared reducer is never run evidence for a neighbour (§3.3). |
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

**The three states of the slot.**
- *Absent*: no declaration. Nothing is checked, and the reducer solves as its
  nodes place it. An absent offset is **not** a declaration of concentricity.
- *Explicit zero*: declares a concentric reducer, and is checked.
- *Nonzero*: declares an eccentric reducer, and is checked.

**First pass.** An allocation-free scan over the components looks for any
component, of any kind, whose `geometry.eccentric_offset` is present.
- If there is none, the check returns before any index, map or text is built
  (§8).
- Otherwise every non-reducer carrying the slot is refused (`not_reducer`)
  before any reducer-specific processing, whatever its consumption mode.

**Applicability.** The reducer processing then covers each `reducer` that
carries the slot, unless its `solver_consumption` is `not_solver_consumed`.
That mode is honoured: the offset is not read and no offset diagnostic is
stated. Ordinary structural and unit validation still applies (§9, P6).

1. **Ends.** The ends come from the convention function (§3.6): the origin
   end N and the far end F of the mapped member `geometry.rigid_pipe_ref`. The
   member must exist and end at the component's node (W-15's mapping rule).
   Then D = x_F − x_N.
2. **Run axis** (§3.3). This gives an undirected axis chosen independently of
   which end is N, then oriented by D so that D·t > 0, with its direction
   uncertainty β_run. Or it gives a named refusal.
3. **Requirements**, in this order. The tolerance constants are in §3.3.
   - **Axial length.** D·t > 0, so the body has length along the run.
   - **Precision** (nonzero e only). The declared offset must be resolvable:
     a_e = ρ_base·‖e‖ + (√3/2)·q + ‖e‖·sin β_run must be < ‖e‖. When
     a_e ≥ ‖e‖ the transversality test cannot distinguish any direction of
     e, so the offset is refused, with a precision code (§3.4). The user
     declares a zero offset or corrects the geometry.
   - **Transversality** (nonzero e only). |e·t| ≤ a_e. This is a numerical
     agreement rule on the declared vector, independent of the placement
     tolerance. Without it, a collinear e = (5, 0, 0) mm on concentric
     geometry would pass placement with a residual of exactly 5 mm.
   - **Placement** (rule B, §9). ‖D⊥ − e‖ ≤ τ + ‖D‖·sin β_run, where
     D⊥ = D − (D·t)·t, τ is the model's node-proximity tolerance
     (`PreviewModel::node_proximity_tolerance_m`: 5 mm unless
     `model_settings.node_proximity_tolerance` sets another; τ remains the
     user's control), and ‖D‖·sin β_run is the run-axis allowance: the body
     length times the sine of the governing axis's own uncertainty, carried as
     the transversality allowance carries ‖e‖·sin β_run. On a tie each end uses
     its own β (J3). A residual within τ is accepted as before. A residual
     above τ but within the allowance is accepted with the named warning
     `..._AXIS_ALLOWANCE_USED` (§3.4). Beyond it, `..._MISMATCH`. The check
     bounds the residual on the entered nodes, not the true placement: no
     certified bound on true placement is claimed (§9, rule B).
4. **Arithmetic.** Every quantity is formed in binary64 from the normalized
   values. Every comparison is `≤`, except the precision test's strict `<`.

The actual geometry governs. A declared offset that passes is a statement about
the reducer, checked against the entered nodes within these allowances. The lever arm, the stiffness and
every result come from the nodes. The solve is bit-identical to the same model
without the offset member (§12).

### 3.3 The run axis, its uncertainty and the refusal boundary

§1 defines the end centerlines as parallel. The run axis must therefore be
supported at the ends, and must agree where both ends support it.

**Tolerance constants** (HELP_HUMAN, 2026-10-26, under the owner's delegation;
§9, P2 and P3):
- ρ_base = 1e-3 and θ_base = 1e-3 rad.
- q = 1 mm is the declared coordinate-rounding allowance. It is a fixed
  constant of the product (`COORDINATE_ROUNDING_ALLOWANCE_M`), not a user
  setting. It reflects the common practice of entering coordinates, and the
  offset's components, rounded to the millimetre. Each entered coordinate is
  taken to lie within q/2 of its true value, so a point lies within (√3/2)·q
  of its true position, and the separation of two points within √3·q.

**Evidence at an end.** At each end X ∈ {N, F}, consider every member other
than the body that ends at X. Each is classified:

| Adjoining member | Class | Direction d (undirected) | Uncertainty β |
|---|---|---|---|
| a straight pipe mapped by no component as a body, bend, expansion-joint member or connector span (a branch component's header or branch pipe is such a pipe) | run | its axis | asin(√3·q/L), L its length |
| the straight body of a valve, flange, rigid or specialty component | run | its axis | asin(√3·q/L) |
| a member realized as an arc by a DEC-070 bend macro-element | run | the arc's tangent at X: chord ĉ and in-plane normal n̂ (`y_reference` less its chord component), half-angle φ/2 = asin(c/(2R)), tangent cos(φ/2)·ĉ ± sin(φ/2)·n̂ (+ at the arc's `from` node, − at its `to` node; the arc bows toward +n̂, PP `lib.rs` ~8715) | see below |
| the body of any other reducer: absent, zero or nonzero offset | unsupported | none. An undeclared reducer's chord may be slanted. A declared one is judged by its own check, which cannot be the evidence for this one. Eccentric reducers in series therefore stay refused in this slice (§3.8). | — |
| a member mapped by a bend component and not realized as an arc | unsupported | none: a chord is not a tangent | — |
| a member mapped by an expansion joint, or a span replaced by an objective connector | unsupported | none | — |
| any run-class member whose β is not defined (√3·q ≥ L or c, or a total β ≥ π/2) | unsupported (`unresolvable_run_evidence`) | none: the rounding admits no direction; the message names which part of the bound fails (below) | — |

**Realized arc uncertainty: a finite bound** (revised after Codex numerical
scrutiny r1, P2). The tangent is cos h·ĉ ± sin h·n̂, with:
- chord ĉ;
- in-plane normal n̂ (the `y_reference` y less its chord part);
- half-angle h = asin(c/(2R)).

Each endpoint lies within (√3/2)·q of its true position, so the true chord
differs from the entered one by at most ε = √3·q. The bound holds over that
whole region, not to first order:

| Part | Finite bound | Defined while |
|---|---|---|
| chord direction | α_c = asin(ε/c); ‖ĉ′ − ĉ‖ ≤ d_c = 2·sin(α_c/2) | ε < c (else `SHORT_SPAN`) |
| half-angle, chord length in c ± ε | Δh = max(asin((c + ε)/(2R)) − h, h − asin(max(0, c − ε)/(2R))), since asin is increasing | c + ε < 2R (else a semicircle is admissible: `NEAR_SEMICIRCLE`) |
| plane normal | with u = y/‖y‖ and sin φ = ‖u⊥‖: ‖u⊥′ − u⊥‖ ≤ ‖ĉ′ − ĉ‖ + \|u·(ĉ′ − ĉ)\| ≤ d_n = d_c + (1 − cos α_c) + sin φ·sin α_c; the normal turns by at most α_n = asin(d_n / sin φ) | d_n < sin φ (else the plane is unresolved: `ILL_CONDITIONED_PLANE`) |
| frame (ĉ, n̂) → (ĉ′, n̂′) at a fixed half-angle | ‖Δt‖/2 ≤ cos h′·sin(α_c/2) + sin h′·sin(α_n/2) ≤ √(sin²(α_c/2) + sin²(α_n/2)), so the angle is at most 2·asin of that | — |

β_arc = Δh + 2·asin(√(sin²(α_c/2) + sin²(α_n/2))), required < π/2. This
follows from the triangle inequality: change the half-angle in the entered
frame, then change the frame.

A bend whose bound is undefined is unresolved run evidence, refused by
`..._INVALID` with reason `unresolvable_run_evidence`. The message names the
member and which part fails, each with its own explanation:
- `SHORT_SPAN`: ε ≥ c;
- `NEAR_SEMICIRCLE`: c + ε ≥ 2R;
- `ILL_CONDITIONED_PLANE`: d_n ≥ sin φ;
- `TOTAL_UNCERTAINTY`: every part is defined, but β_arc ≥ π/2 (the total
  end-tangent uncertainty within the rounding reaches π/2). Codex's case
  (numerical scrutiny r2, P3): chord (0.005, 0, 0) m, R = 0.0033660304037844385
  m, y = (0, 1, 0); d_n = 0.760 < sin φ = 1, total β_arc = 1.674.

The user's radius and `y_reference` are entered values, taken as exact.

**The bound's limit is a conservative certificate boundary** (J1, §9). A bend
whose β_arc reaches π/2, or whose parts are undefined, is refused even where
its true geometry would resolve the axis: the check certifies only what the
rounding region as a whole supports, with no extra margin. Probe variants'
true-geometry controls are therefore refused, not resolved. Ordinary bends
resolve: Codex's sweep (numerical scrutiny r2) of every 90° and 45° elbow at
R = 1.5D, D = 100–600 mm, mm-rounded, with a global-Y reference, resolves with
β_arc 0.0049–0.0296 rad at 90° and 0.0069–0.0412 rad at 45°. A single 180°
return is refused (`NEAR_SEMICIRCLE`); two 90° arcs work. The near-return
cutoff is about 167.68° at D = 100 mm and 174.97° at D = 600 mm.

**Coverage, with margin.**
- **Codex's probe 1** (plane reference nearly parallel to the chord):
  d_n / sin φ = 1.636. The plane is unresolved, and the bend is refused by
  name instead of the false `run_directions_disagree`.
- **Probe 2** (near semicircle): c + ε − 2R = 0. It is refused by name.
- **Admissible variants of both probes, the quarter circle and an oblique
  bend:** every corner of the ±q/2 box on both endpoints and 20,000 interior
  samples stay within β_arc. The worst sampled change is 0.08, 0.09, 0.83,
  0.75, 0.38 and 0.39 of β_arc respectively
  (`w15c_eccentric_offset_tests.rs`, `the_arc_bound_covers_the_finite_rounding_region`).

**Short spans give wide uncertainty.** A 1 m span has β = 1.732e-3 rad. A
100 mm span has 1.732e-2 rad. Below √3 mm a span gives no direction. A wide β
widens the agreement bound, the transversality allowance and the placement's
run-axis allowance ‖D‖·sin β_run (§3.2, rule B). The
precision test (§3.2) refuses an offset that the resulting allowance cannot
resolve.

**The end's status:**
- *none*: no adjoining member, a terminal end;
- *direction*: exactly one adjoining member, of class run;
- *ambiguous*: more than one adjoining member, such as a tee or branch;
- *unsupported*: exactly one adjoining member, of class unsupported.

**Decision.** Ambiguous and unsupported are checked first, at N then F.

| N | F | Outcome |
|---|---|---|
| ambiguous (either end) | any | refuse, `ambiguous_run`, naming the end and its candidates |
| unsupported (either end) | any | refuse, `unsupported_run_evidence` (or `unresolvable_run_evidence`), naming the end and member |
| none | none | refuse, `standalone_body` |
| direction | direction | Agreement: θ = atan2(‖d_N × d_F‖, |d_N·d_F|), the angle after aligning signs, must satisfy θ ≤ θ_base + β_N + β_F. Otherwise refuse, `run_directions_disagree`. Then the better-resolved end governs (J2, §9): u = the direction with the smaller β, and β_run = that β. On an exact tie β_N = β_F no consensus is formed (J3, §9): the offset is judged against each end's own (d, β), with that β in each end's own transversality and placement allowances, refused if either judgment refuses, and the reported judgment follows the tie precedence below. |
| direction | none, or none | direction | u = the one direction; β_run = its β |

**Tie precedence (J3).** Of the two judgments at a tie, the reported one is:
a refusal before an acceptance; then the larger residual ‖D⊥ − e‖; then the
earlier refusal in the check's order (no axial length, below precision, not
transverse, mismatch); then the larger |e·t|, D·t and allowance; and last the
axis t and D⊥, each multiplied by the sign of D's first nonzero component,
compared component by component. The outcome and its reported text are
unchanged when the ends' names are swapped. Whenever D·t > 0 the precedence
keys are also literally unchanged. Signed zeros are the one exception IEEE
negation leaves there (a D·t of −0 under one naming is +0 under the other;
Codex numerical scrutiny r4), so on the tie path every key and every reported
quantity is canonicalised to +0. When D·t = 0 (`no_axial_length`) the axis is
not oriented by D, and an internal key can differ under the swap (Codex r5:
tied u = (−0, −1, −0), D = (0.75, 0, 0), e = 0, where the key's axis Y
component is −1 under one naming and +1 under the other), while the outcome,
`no_axial_length` with D·t = 0e0, does not (`judge_run`,
`rigid_component.rs`).

**Orientation and naming independence.** t = u, negated if u·D < 0.
- The choice is symmetric in N and F up to sign: the smaller-β rule does not
  depend on which end is N, and the tie's outcome and reported text are
  unchanged by the swap (its precedence keys too, whenever D·t > 0).
- Swapping the ends' names negates D and t exactly. Under the other
  convention it also negates e.
- So |e·t|, ‖D⊥ − e‖, a_e and every outcome are bit-identical under a
  sign-only convention change. §12's convention tests assert this.

Accepted configurations include flange–reducer–flange and an arc at one end
with a pipe at the other. Two reducers in series are refused (unsupported).
So is a reducer between two members mapped by bend components that are not
realized as arcs, since neither end gives a tangent.

### 3.4 Diagnostics

| Code | Severity | When |
|---|---|---|
| `RIGID_COMPONENT_ECCENTRIC_OFFSET_INVALID` (new) | blocking | A present offset cannot be checked. The message names the component and the first reason, in this order: `not_reducer`; `not_finite` (a component of e after conversion); `mapping_invalid`; `ambiguous_run`; `unsupported_run_evidence`; `unresolvable_run_evidence`; `standalone_body`; `run_directions_disagree` (stating θ and its bound); `no_axial_length` (D·t ≤ 0). |
| `RIGID_COMPONENT_ECCENTRIC_OFFSET_BELOW_PRECISION` (new) | blocking | a nonzero e with a_e ≥ ‖e‖. The message names ‖e‖, a_e, q and β_run, and says to declare zero or correct the geometry. |
| `RIGID_COMPONENT_ECCENTRIC_OFFSET_NOT_TRANSVERSE` (new) | blocking | |e·t| > a_e. The message names e, t, |e·t| and a_e. |
| `RIGID_COMPONENT_ECCENTRIC_OFFSET_MISMATCH` (new) | blocking | ‖D⊥ − e‖ > τ + ‖D‖·sin β_run (rule B). The message names e and D⊥ in metres, the residual, the placement allowance, τ, the run-axis allowance and β_run. |
| `RIGID_COMPONENT_ECCENTRIC_OFFSET_REALIZED` (new) | info | On a published result only, like slice a's applied-scale info (PP `rigid_component.rs` `append_formed_diagnostics`). See below. |
| `RIGID_COMPONENT_ECCENTRIC_OFFSET_AXIS_ALLOWANCE_USED` (new, rule B) | warning | τ < ‖D⊥ − e‖ ≤ τ + ‖D‖·sin β_run, on a published result only, beside the `..._REALIZED` info (`append_eccentric_offset_realized`). The message names the declared offset, the realized offset D⊥ from the node geometry, the residual, τ, the run-axis allowance and β_run, and says that the node geometry was used for the solve. |

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
  is the only reader of the origin rule. The ruled option A gives N = the
  component's node and F = the member's other end. (Option B, not ruled,
  would have given N = the member's `from` and F its `to`.)
- **TypeScript:** `ECCENTRIC_OFFSET_CONVENTION` (one module) holds the text the
  inspector, creation form and report show.

The convention's tests are separate (`tests/w15c_offset_convention.rs` and one
vitest), and §7.5 holds the convention-dependent benchmark rows.

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
- reducer SIF or other local modifiers;
- a certified bound on true placement. Rule B (§9) bounds the residual on the
  entered nodes; within the coordinate-rounding region the true placement can
  exceed τ and still pass (with the `..._AXIS_ALLOWANCE_USED` warning above τ).
  Sampled maxima are in §9; no certified bound is claimed;
- reducers in series. A later rule could traverse an unbranched chain of
  bodies to independent outer run evidence at each end, then check each
  reducer of the chain against it (Codex r2). This slice refuses any
  neighbouring reducer body as unsupported (§3.3).

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
[`model-document-0.8.0.md`](../../../PKG-02/DEL-02-01/Work/model-document-0.8.0.md)
covers:
- the ripple;
- the refusals;
- stored work and recorded runs under the owner's ruling "0.8.0 a2";
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
- on the body (local axes as above), the transmitted force (0, Q, 0) is
  pure local-y shear Q. The moment resolves into torsion −5000/13 N·m
  (constant) and bending about local z, falling from 13690/13 N·m at A to
  12000/13 N·m at B. Codex r2 confirmed these values;
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
0.8.0 ripple, after #1625 and #1630 (Agent 0, 2026-10-26).

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
  applies.** The first pass is an allocation-free scan for any
  component carrying `eccentric_offset`. It returns before any pipe map,
  adjacency index or diagnostic text is built.
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

**As priced** (DEL-04-08 `retained-pricing-rules.md` §8.26, with the 0.8.0
ripple, on main `289bd8d546`). The hypothesis held: `compare` against main's
registered tree is `identical_but_basis` (1,586 coefficients equal, none
higher or lower), and so is each point's tree. Four adjustments were needed:
- Both entry points read the node-proximity tolerance after the first pass,
  not before the call (`lib.rs` no longer reads it). Read before, the two
  added accessor calls would have priced the accessor's five error texts
  twice more, 6,392 B on each route. The behaviour is unchanged: an unadmitted
  tolerance is refused by `validate_model_settings`, and the check does not
  run on it.
- `validate_profile`'s two version texts, which now format the shared
  constant, are unreachable on D1 (M08-VZ); priced as reached they would
  add 1,980 B on route L and 8,910 B on route E.
- `arc_evidence`'s `min`/`max` receivers are f64, not the certified
  publication's private helpers (W15C-CG).
- Both entry points' work after the first pass is zero on D1 by P2: the
  topology, the judgement, the tolerance read, their loops and texts
  (W15C-NO, W15C-L1, W15C-ID). The SD-NR key-loop bound is 32.

The six AC-004 stack witnesses passed 6/6 in the registered build at
`99005d9745` under the exclusive lane (`check_release_readiness.py --profile
stack-witnesses --execute`; DEL-04-08 `retained-pricing-rules.md` §8.26).
Model document 0.8.0's shear-model member then re-registered the profile
(§8.26, "The shear-model member"), and the six witnesses passed 6/6 again at
head `c83cf828cc` (registered source `95aacdf852`) under the exclusive lane,
by the same command. This is test evidence, not an acceptance.

## 9. Precision and detail matters

Each entry is HELP_HUMAN, 2026-10-26, under the owner's delegation (LOOP_INIT,
owner 2026-10-26: engineering judgment for matters of precision or detail).
Resolutions that change numerical meaning carry §12's regression checks and
independent scrutiny.

| # | Matter | Resolution |
|---|---|---|
| P1 | Body realization | Approved as Codex says. The chord member between the two centerlines (§3.1). It is equivalent to a rigid link mechanically and kinematically only with zero body eigenstrain and equivalent loading. Thermal growth stays αΔT·D, offset growth included, at every scale. Load resultants (chord self-weight over ‖D‖) differ unless matched. |
| P2 | Tolerance, transversality and precision | Revised, then decided. The actual node geometry governs the lever arm and the solve, and every M = P·e claim means P·‖D⊥‖. Placement: the declared offset was checked within τ (the 5 mm node-proximity tolerance, user-settable), separately from any rounding allowance; rule B (below) superseded this with τ + ‖D‖·sin β_run. Transversality: |e·t| ≤ ρ_base·‖e‖ + (√3/2)·q + ‖e‖·sin β_run, with ρ_base = 1e-3 and q = 1 mm, a fixed declared coordinate-rounding allowance, not a user setting, reflecting millimetre-rounded entry (§3.2, §3.3). A nonzero offset whose allowance reaches its magnitude is refused by `..._BELOW_PRECISION`. **Why:** with ρ alone, a 30° run entered as (866, 500, 0) mm with offset (−25, 43, 0) mm has axial share 0.003015789 and would be wrongly refused; under the rule it is accepted. Regressions: that case; 47 mm declared against 50 mm geometry (passes, and the 50 mm geometry governs); the collinear e = (5, 0, 0) mm, still refused by `..._NOT_TRANSVERSE`; one case just outside the transversality bound and one just inside the precision bound (§12). |
| P3 | Run axis | Revised, then decided (§3.3). Run evidence comes from ordinary pipes, the bodies of valves, flanges, rigid and specialty components, and realized-arc tangents, each with a direction uncertainty β: asin(√3·q/L) for a straight member; the defined β_arc for an arc. Any other reducer's body (absent, zero or nonzero offset), an unrealized bend member, an expansion-joint member, a connector span and an unresolvably short member are unsupported. Both ends' directions must agree: θ ≤ θ_base + β_N + β_F, with θ_base = 1e-3 rad, after aligning signs. The axis is the better-resolved end's direction (J2); on an exact tie each end's own direction is judged separately (J3). The outcome is independent of endpoint naming, and the axis is oriented by D. Ambiguous or unsupported topology is refused by name. **Why:** with θ_base alone, two parallel 100 mm spans rounded to (87, 50) and (86, 50) mm are 0.005008974 rad apart and would be wrongly refused; under the rule they are accepted. Regressions: that case; one case just outside the agreement bound; Codex's two-reducer chain; and the convention-invariance case (§12). **Realized arcs (Codex numerical scrutiny r1):** β_arc is a finite bound over the whole rounding region, and a bend whose bound is undefined (a reachable semicircle, an ill-conditioned plane) is refused by name (§3.3). |
| P4 | Body length against geometry | Approved: not checked for any rigid-family kind. The demo valve's body length is 0.72 m on a 2.2 m member. |
| P5 | Self-weight of the chord body | Approved: it acts over ‖D‖, and slice b's component weight replaces it when entered. |
| P6 | `not_solver_consumed` | Approved: honoured, the offset not read and no offset diagnostic. Ordinary structural and unit validation still applies. |
| P7 | Offset on a non-reducer | Approved: blocking `..._INVALID` (`not_reducer`). |
| P8 | Present zero vector | Approved: checked as concentric under the revised rules. Transversality passes trivially, and placement requires ‖D⊥‖ ≤ τ + ‖D‖·sin β_run (rule B), with the warning above τ. |
| P9 | D·t ≤ 0 | Approved: blocking `..._INVALID` (`no_axial_length`). |
| P10 | Thermal growth of the offset | Approved: αΔT·D, offset included. |
| P11 | Stress rows of an eccentric reducer | Approved, with disclosure. Rows are the reference pipe section's along the chord. Scaled rows carry `component_body`. The reference-section meaning is disclosed in the info, the UI and the report text in both the scaled and unscaled cases (§3.4, §4). |
| P12 | Version bump while #1630 closes the record | Approved: 0.8.0 (DEL-02-01 Design `model-document-0.8.0.md` §1). |
| J1 | The bound's limit for ordinary bends | Decided (Agent 0, HELP_HUMAN, 2026-10-26, under the owner's delegation): keep. Refuse at the bound's limits, with no extra margin. Codex's numbers (numerical scrutiny r2): every 90° and 45° elbow at R = 1.5D, D = 100–600 mm, mm-rounded, with a global-Y reference, resolves (β_arc 0.0049–0.0296 rad at 90°, 0.0069–0.0412 rad at 45°). A single 180° return is refused; two 90° arcs work. The near-return cutoff is about 167.68° at D = 100 mm and 174.97° at D = 600 mm. The refusal is a conservative certificate boundary: the probes' true-geometry controls are refused rather than resolved (§3.3). |
| J2 (was P13) | How far weak run evidence may steer the axis | Decided (Agent 0, 2026-10-26, approving the recommendation). After the status and agreement checks, the end direction with the smaller β is the axis and its β is β_run. J2 kept the consensus for an exact tie; J3 replaced it. Endpoint and sign symmetry are kept, and no constant is added (§3.3). **Why:** with an equal-weight consensus, a resolved but weak bend can tilt the axis by up to half its β. **Regression** (Codex numerical scrutiny r2), on PF and EX: entered chord (−18.19787406218549, 13.058496225274045, 3.349962055095552) m, true chord the entered plus (0.001, 0.001, 0.001) m, R = 11.36337291391957 m, y_reference (−0.8035474210765018, 0.5765244818323195, 0.1480914039682203), e = (0.00819795250341938, −0.005298081948721869, −0.002173454029030151) m, D = 0.04·t + e, and a 1 m straight run along t, the true chord's far-end tangent. On `4291a7ace8` it gave `..._MISMATCH` (8.086 mm residual; bend β 0.4158, run β 0.0017). Now the run governs and the offset is realized (residual about 2.2e-15 m). Through the solve, the model then stops for a separate reason: the arc's `y_reference` lies within 1.9e-4 of its chord, and the frame kernel refuses its local axes (`PIPE_ELEMENT_INPUT_INVALID`, the same with `eccentric_offset` removed). **Full-solve regression** (Codex numerical scrutiny r3; invented geometry), on PF and EX, sparse and dense: a 0.15 m radius arc with `y_reference` (0, 1, 0), entered chord (0.298, 0, 0) m against a true chord (0.299, 0, 0) m (bend β ≈ 0.08628); a reducer of 100 mm OD / 5 mm wall to 80 mm OD / 4 mm wall with body 0.4·t + e along the true tangent t and e = (0, 0, 10) mm; and a 1 m straight run along t (β ≈ 0.001732). It solves with the offset realized; the equal-weight consensus gave `..._MISMATCH` (6.772065 mm residual). There are two regressions because the first geometry exercises the offset check at a weak bend (β ≈ 0.4158) but cannot reach a solve, and the second reaches a solve with an ordinary arc frame (§12). |
| J3 | The exact tie β_N = β_F | Decided (Agent 0, 2026-10-26, under the owner's delegation), verbatim: "option B. On an exact tie, judge each end with its own (direction, β), refuse if either refuses, and report the larger residual. Reasons: It's safe-side. A placement the stronger evidence refuses is never accepted on weaker evidence; A would allow that. It's symmetric. The decision at the tie matches one of its 1-ulp neighbours, so it adds no new discontinuity beyond the unavoidable switch of governing end." The sign-aligned consensus is removed. The precedence that makes the reported judgment independent of endpoint naming is in §3.3. **Why** (Codex numerical scrutiny r3): (a) two 125 mm runs with opposite 0.9765625 mm lateral differences, a 750 mm body and a 15.625 mm Z offset: the consensus realized the offset exactly at the tie, while either 1-ulp neighbour of the downstream outer node's X selected one end and refused a 5.859196 mm placement residual on PF and EX. (b) Two 2 mm chords, each a common true chord ±(1, 1, 1) mm with β = 60°, admit the true axis, but their consensus lay 90° from it while β_run = 60° was returned. **Regressions:** (a) at the check and through the solve on both routes, with both neighbours and the swapped naming; (b) at the check; the precedence (§12). |
| Rule B | The placement under coordinate rounding | Decided (Agent 0, 2026-10-26, under the owner's engineering-judgment delegation), verbatim: "Placement rule, decided under the owner's engineering-judgment delegation: B ... Reasons: Node geometry governs the solve, so the check guards consistency between the declared offset and the geometry. It does not move the solve. Refusing realistic compact flange–reducer–flange layouts, entered to the mm, blocks real use. Carrying the axis uncertainty in the same way as the transverse allowance is the normative treatment of input rounding. Condition: no silent over-acceptance. When a placement passes only because of the axis allowance (τ < residual ≤ τ + ‖D‖·sin β_run), emit a named warning. It gives the declared offset, the realized offset, the residual and τ, and says that the geometry was used. Below τ, behaviour is as today." **Rule:** placement passes when ‖D⊥ − e‖ ≤ τ + ‖D‖·sin β_run, with D the body vector and β_run the governing axis's own β (each end's own on a tie, J3). Below τ nothing changes. Every acceptance above τ carries `..._AXIS_ALLOWANCE_USED` (warning), which states the declared offset, the realized offset, the residual, τ and the run-axis allowance, and that the node geometry was used for the solve. τ remains the user's control (`model_settings.node_proximity_tolerance`). Node geometry governs the solve: the check never moves the lever arm, the stiffness or any result. **What the rule bounds:** the residual between the declared offset and D⊥ formed from the entered nodes. **What it does not bound:** the true placement. Within the 1 mm rounding region the true residual can exceed τ and still pass, and no certified bound on true placement is claimed. Codex's sampled maxima of the true residual accepted (numerical scrutiny r4; current rule / rule B, mm, with a 0.6 m body), as sampled, not certified: 0.1 m run 11.6/21.7; 0.15 m 10.1/16.4; 1 m 7.0/7.9; 3 m 6.3/6.9; 6 m 6.3/6.3 (the 6.5 mm rule B maximum at 6 m was seen with a 0.2 m body). **Why:** under τ alone, mm-rounded entry of exactly consistent ordinary reducers was refused where the governing run is short against the body (L_body/L_run above about 1.9–2.5; worst entered residual about c·q·(1 + L_body/L_run), c from √2 to √3 with the run's orientation): 181 of about 2.3 million ordinary-size samples (bodies up to 1.0 × the large OD), all between 0.1–0.15 m members, e.g. 400→300 with a 0.4 m body at 30° in plan and 10° slope (5.3196 mm) and 600→400 with a 0.6 m body at 45° in plan (5.0155 mm); Codex's own sweep gave 593 of 1.55 million. Rule B accepts every one. The bundled demo model has no reducer. **Regressions** (§12): both ordinary cases on PF and EX; the 0.1 m run, 0.6 m body case at the high end of the band; the bound from both sides; the J3 tie under B; a tie with one end accepting and one refusing. |

## 10. R-c1 ruled, and the ScopeOfWork drafts

**R-c1: owner, 2026-10-26, verbatim: "R-c1 A, 0.8.0 a2".** The eccentric offset
is a vector in the model's global X/Y/Z axes, from the centerline at the
component's node to the centerline at the body's other end, in model length
units. It mirrors R10, and reversing the member does not flip e.
- It is implemented in `rigid_component::eccentric_offset_ends` and
  `eccentricOffsetConvention.ts` (§3.6).
- The convention tests stay labelled as such (`w15c_offset_convention.rs`;
  the convention case of `reportEccentricOffset.test.ts`).

**ScopeOfWork drafts (DEL-03-05).** These are not applied here. They go through
Codex, then to Agent 0 under the wording grant. Line numbers are at the basis.
Old text is quoted exactly; insertions name their anchor line.

1. **Line 56** (CLM-003, "Geometry slots").
   - **Old:** ``> | Geometry slots | `ComponentFieldKind`; fixture field definitions; strict fixture tests | `rigid_body_length`, `end_a_size`, `end_b_size`, `connection_end_a_reference`, and `connection_end_b_reference` are schema/fixture-recognized slots. Public fixture values remain schema-shape-only. |``
   - **New:** ``> | Geometry slots | `ComponentFieldKind`; fixture field definitions; strict fixture tests; model document 0.8.0 | `rigid_body_length`, `end_a_size`, `end_b_size`, `connection_end_a_reference`, and `connection_end_b_reference` are schema/fixture-recognized slots; a reducer's `eccentric_offset` is a model-document slot (R12). Public fixture values remain schema-shape-only. |``
2. **Insert after line 58** (CLM-003, after the "Center of gravity input" row).
   - **New:** ``> | Eccentric offset input | PRD §12.3; R12 | `eccentric_offset` on a reducer is a vector in the model's global X/Y/Z axes, from the centerline at the component's node to the centerline at its body's other end, in model length units (owner ruling of 2026-10-26: "R-c1 A, 0.8.0 a2"). The node geometry governs: the solve checks the declared offset against it and never places nodes (Design `eccentric-reducer-offset.md`). |``
3. **Line 64** (CLM-003).
   - **Old:** `solver consumption beyond W-15 slices a, b and d,`
   - **New:** `solver consumption beyond W-15 slices a, b, c and d,`
4. **Insert after line 144** (CLM-010, after R10).
   - **New:** ``> | R12 | Define the eccentric reducer offset's coordinate convention and reference frame before offset values are accepted for mechanics use. | PRD §12.3; SOW-009; OPS-K-UNIT-1 | Owner ruling, 2026-10-26: "R-c1 A, 0.8.0 a2". The convention is global X/Y/Z axes, from the centerline at the component's node to the centerline at its body's other end, in model length units (Design `eccentric-reducer-offset.md` §2, §3.6). Tests check accepted and refused offsets, the convention rows and an independent eccentric-step benchmark on both routes. |``
5. **Line 312** (CLM-024).
   - **Old:** `Human/project authority must still decide coordinate conventions other than the centre-of-gravity convention ruled under R10, accepted source catalogs,`
   - **New:** `Human/project authority must still decide coordinate conventions other than the centre-of-gravity convention ruled under R10 and the eccentric reducer offset convention ruled under R12, accepted source catalogs,`
6. **Line 49** (CLM-003, "Current implementation"), applied with the ripple
   when the slot is in the format.
   - **Old (opening):** `> Current implementation (W-15 slices a, b and d, PRs #1604 and #1608; Design `
   - **New (opening):** `> Current implementation (W-15 slices a, b, c and d, PRs #1604, #1608 and #1649; Design `
   - **Appended at the paragraph's end:** `Slice c checks a reducer's declared `eccentric_offset` (R12) against the node geometry, which governs: the run axis comes from run evidence at both ends within the coordinate-rounding allowance, and a mismatch, an offset that is not transverse or not resolvable, or unresolved run evidence is refused by name (`core/product_physics/src/rigid_component.rs`; Design `eccentric-reducer-offset.md`).`

No acceptance criterion changes. AC-001 holds as written: a ruled convention is
recorded, not invented.

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
  - an explicit zero e on collinear nodes;
  - **declared 47 mm against 50 mm geometry:** placement residual 3 mm ≤ τ.
    The results are bit-identical to the same model without the offset member
    and to the W15C-P rows, and the info states declared 47 mm and realized
    50 mm.
  - **the 30° run:** P1 entered as (866, 500, 0) mm, e = (−25, 43, 0) mm, all
    coordinates mm-rounded; axial share 0.003015789 (P2 decision);
  - **the 100 mm spans:** P1 (87, 50) mm and P2 (86, 50) mm, θ = 0.005008974
    rad (P3 decision);
  - just inside each bound: transversality, angle agreement and precision.
- **`..._NOT_TRANSVERSE`:**
  - the collinear e = (5, 0, 0) mm on concentric geometry, whose placement
    residual of exactly 5 mm alone would pass;
  - an axial component just above a_e (1 m runs along X, e = (1.01, −50, 0)
    mm), with (1.00, −50, 0) mm accepted.
- **`..._BELOW_PRECISION`:** e = (0, −0.8, 0) mm with 1 m runs
  (a_e = 0.867 mm), with (0, −0.9, 0) mm accepted on matching geometry.
- **Rule B** (§9; each fails under the τ-only placement):
  - **ordinary compact reducers,** on PF and EX
    (`rounded_compact_reducers_are_accepted_with_the_axis_allowance_warning`):
    400→300 with a 0.4 m body between 0.1 m runs at 30° in plan and 10°
    slope (residual 5.3196 mm) and 600→400 with a 0.6 m body at 45° in plan
    (5.0155 mm), mm-rounded; accepted with the realized info and the warning,
    results bit-identical to the same model without the offset. Under τ alone
    both were `..._MISMATCH`;
  - **the high end of the band** (Codex r4: 0.1 m runs, 0.6 m body, 45° in
    plan, 2° slope, true deviation 21.7 mm; entered residual 15.37 mm):
    accepted with the warning
    (`the_high_end_of_the_axis_allowance_band_is_accepted_with_the_warning`);
  - **the bound from both sides** (0.1 m runs on X, 0.6 m body, allowance
    about 15.4317 mm): 15.4 mm accepted with the warning, 15.5 mm refused as
    `..._MISMATCH` naming the placement allowance, τ and the run-axis
    allowance, and 4.9 mm accepted with no warning
    (`the_axis_allowance_bound_from_both_sides`; unit
    `rule_b_placement_bound_from_both_sides`). Every `assert_accepted` case
    asserts the realized info alone, so a residual within τ carries no warning;
  - **the J3 tie under B:** the 5.859196 mm tie and both neighbours are now
    realized with the warning, under either naming (moved expectation; see
    J3 below);
  - **one end accepting, one refusing** (Codex r4): 4.9 mm at the axis end,
    5.140781 mm at the tilted end, above its own τ + ‖D‖·sin β = 5.13 mm; the
    tie refuses (`j3_tie_precedence_is_refusal_then_larger_residual`).
- **`..._MISMATCH`** (each still beyond τ + ‖D‖·sin β_run):
  - sign flipped (residual 0.1 m);
  - wrong transverse axis (0.0707 m);
  - magnitude 40 mm (10 mm);
  - the 47 mm case with τ set to 1 mm through `model_settings`.
- **`..._INVALID`,** one test per reason in §3.4's order:
  - an offset on a valve. This includes a model with no reducer at all, which
    the first pass must not skip;
  - a non-finite converted value;
  - a mapping that is absent, unknown or not ending at the node;
  - a tee at N;
  - a neighbouring reducer at N, in three cases: absent, zero and nonzero
    offset;
  - **Codex's chain:** an X pipe, then two successive reducer chords
    (0.12, −0.05, 0), the first undeclared and the second with e = 0. This
    must not pass;
  - an unrealized bend member at N, and a connector span at N;
  - a run member shorter than √3·q;
  - a standalone body;
  - run directions just outside θ_base + β_N + β_F (1 m runs:
    4.47e-3 rad refused, 4.46e-3 rad accepted);
  - D·t ≤ 0.
- **Run evidence:**
  - a realized arc at N supplies its tangent, with β_arc;
  - **the finite arc bound:** every corner and 20,000 samples of the rounding
    region stay within β_arc on six bends, including admissible variants of
    both probes;
  - **Codex's probe 1** (plane reference nearly parallel) and **probe 2**
    (near semicircle), each through the solve on both routes: refused by name
    as unresolved run evidence, never `run_directions_disagree`. Both gave the
    false disagreement on `c90a6d61ff`;
  - a concentric flange body at both ends is accepted;
  - a terminal end uses the other end's direction;
  - **total uncertainty (P3, r2):** Codex's bend with every part defined but
    β_arc = 1.674 is refused with the `TOTAL_UNCERTAINTY` explanation;
  - **J2,** in two regressions (§9). The r2 weak-bend case reaches the
    offset check but not a solve: its arc's `y_reference` lies within 1.9e-4
    of the chord, and the frame kernel refuses the arc's local axes, which is
    a check separate from and independent of W-15c. A second, ordinary arc
    geometry (r3) carries J2 through the full solve.
    - **Offset check only:** the r2 case is realized at the check (unit test
      `j2_weak_bend_case_is_realized`, residual < 1e-12 m). Through the solve
      on both routes no offset refusal is stated, and
      `PIPE_ELEMENT_INPUT_INVALID` is the only blocking code
      (`a_weak_bend_does_not_tilt_the_axis_offset_check_only`). With the
      equal-weight consensus it fails with `..._MISMATCH`, residual 8.086 mm.
    - **Full solve:** the r3 case solves with the realized-offset info and no
      blocking code on PF and EX, sparse and dense
      (`a_weak_bend_does_not_tilt_the_axis_full_solve`). With the equal-weight
      consensus it fails with `..._MISMATCH`, residual 6.772065 mm.
  - **J3,** the exact tie (§9). With the old tie consensus restored, (a) and
    (b) fail, and the precedence case reports the smaller residual:
    - **(a) the tie and its neighbours:** at the check
      (`j3_a_tie_is_judged_at_each_end_like_its_neighbours`) the tie and both
      1-ulp neighbours have residual 5.859196 mm, which under rule B is in the
      warning band (realized; under τ alone all three were `..._MISMATCH`);
      the tie's measure is bit-identical to the neighbour whose governing end
      has unchanged evidence, and the swapped naming gives bit-identical
      measures. Through the solve on PF and EX
      (`a_tie_is_judged_at_each_end_like_its_neighbours`) the tie, both
      neighbours and each with the reducer at the other end solve with the
      realized info and the warning stating the same residual. The consensus
      realized the tie with residual 0;
    - **(b) each end's own uncertainty:** both ends keep their own direction
      and β = 60°, and the true axis lies within each
      (`j3_a_tie_keeps_each_ends_own_uncertainty`); the consensus lay 90° from
      it;
    - **precedence:** a tie of two accepted judgments reports the larger
      residual, and one acceptance with one refusal refuses, identically under
      either naming (`j3_tie_precedence_is_refusal_then_larger_residual`).
    - **signed zeros (Codex r4):** tied ends (−0, −1, −0) with D = (0.75, 0,
      0) refuse `no_axial_length` with the identical outcome under either
      naming (`j3_a_tie_reports_signed_zeros_identically_under_either_naming`);
      without the canonicalisation the two namings report D·t = −0e0 and 0e0.
- **Other behaviour:**
  - `not_solver_consumed` with a contradictory offset gives no offset
    diagnostic and the same solve;
  - ordinary validation still reports a malformed unit;
  - the info appears only on a published result;
  - an unscaled eccentric reducer solves as chord pipe, and its info states
    the reference-section meaning;
  - the first pass builds no index when no component carries the slot (a unit
    test on the scan).

**Convention** (`tests/w15c_offset_convention.rs` and one vitest; the only
convention-dependent tests):
- W15C-R and -O are accepted, and their other-option signs refused.
- **Codex's invariance case:** the downstream axis rotated −0.0005 rad and
  e = (0.04, −50) mm, checked as option A with the component at A and as
  option B with the member reversed. The outcome and |e·t|, a_e and
  ‖D⊥ − e‖ are bit-identical.

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

## 12a. As built: the convention-neutral parts (Agent 0, 2026-10-26: "go")

Built on the 0.7.0-era code paths. Held back:
- the schema member and the 0.8.0 ripple;
- any run gate;
- anything in #1648's files.

Option A is implemented behind §3.6's single function and text constant.

- **Check.**
  - PP `rigid_component.rs`: constants and refusal codes; `eccentric_offset_ends`
    (the R-c1 function); run evidence (`straight_beta`, `arc_evidence`, the
    end status); `decide_axis` (one governing axis, or both ends on a tie);
    `judge_offset` (rule B's placement allowance in the measure);
    `judge_run` (the J3 tie judgment and precedence);
    `append_eccentric_offset_refusals`, whose first pass scans every component
    and refuses non-reducers first; `append_eccentric_offset_realized` (the
    info, and rule B's warning above τ).
  - PP `lib.rs`: the wire field `ComponentGeometryInput::eccentric_offset`,
    its normalization, the call after `normalize_model_units`, and the info
    past the last blocked return.
- **Tests.**
  - `src/w15c_eccentric_offset_tests.rs`: the pure rules, the decided examples,
    the bounds from both sides and the naming invariance.
  - `tests/w15c_eccentric_reducer.rs`: the check through the solve on both
    routes.
  - `tests/w15c_eccentric_reducer_benchmarks.rs` with
    `tests/common/w15c_bench.rs`: W15C against
    `validation/references/w15c_eccentric_reducer`.
  - `tests/w15c_offset_convention.rs`: R-c1 rows.
- **Applier** (`operation_applier/src/lib.rs`): the field rule, the non-reducer
  refusal, and the optional creation slot, with tests.
- **Desktop.**
  - `types.ts`;
  - the shape descriptors in `modelDocumentShape.ts` and
    `model_document_shape.rs`, with shared corpus cases
    `eccentric-offset-malformed` and `-valid`;
  - library apply `GROUP_VALUES.rigid`;
  - the inspector's reducer-only offset field with
    `ECCENTRIC_OFFSET_CONVENTION`
    (`features/component-creation/eccentricOffsetConvention.ts`);
  - report component provenance with the offset, the convention and
    `ECCENTRIC_REDUCER_STRESS_BASIS`.

  Not yet built: the offset input on the creation form, which takes it through
  the inspector after creation.
- **Guards** (§5 of DEL-02-01 Design `model-document-0.8.0.md`):
  - `test_model_version_literals.py` scans product-physics and result-export
    production code against a pending-ripple list tied to 0.7.0;
  - `test_model_document_schema.py` gains the explicit-retired-controls rule,
    empty until the ripple.

## 13. Reconciliation at build

- **DEL-03-05:** this Design's "as built" notes, `deliverable.yaml` code paths
  and checks, and §10's drafts after R-c1.
- **DEL-02-01, DEL-02-05, DEL-04-08 and DEL-03-01:** through
  `model-document-0.8.0.md` §10.
- **`docs/COMPLETENESS_MAP.md`:** the rows §12.3 reducers, FR-SOL-004/012 and
  W-15 c.
- **#1648's files:** Agent 0 is told before any of them is touched.
