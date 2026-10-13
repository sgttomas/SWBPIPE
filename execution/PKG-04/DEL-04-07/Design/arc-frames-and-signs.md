# Arc frames and signs (T4-U4a)

This is the governing design for T4-U4a. On a realized arc, it publishes every
row family in one frame and one sign convention, on the exact `pressure-1`
route and on the ordinary `preview-physics-1` route. It changes no product
code by itself. The frozen independent references are in
[`validation/references/t4_u4a/`](../../../../validation/references/t4_u4a/README.md).
Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).

**Owner.** DEL-04-07 owns the unit.

- DEL-04-07's code path `core/product_physics/src/lib.rs` forms and
  publishes the arc end rows (`recover_curved_bend_local_forces`,
  `append_element_force_results`).
- This deliverable's [pressure contract](pressure-contract.md) already
  carries arc recovery and H-4.
- The certified arc recovery is DEL-04-01's (FK `certify_arc_recovery`; CB's
  arc frame). That is a supplier need, stated in `deliverable.yaml`.
- The preview-physics-1 registration rides DEL-05-02's B7a wave, which is
  also stated there.

## Rulings

- **Owner steer:** "T4: K-D5 option (d), then U2/O-13, nonlinear K-D5 and
  U4/U5/U6/U8."
- **HELP_HUMAN technical rulings, 2026-10-12:**
  - T4-U4 is split into U4a and U4b. U4a covers frames and signs. U4b
    covers quantities: the certified arc maximum, the toroidal hoop and radial
    values, directional-SIF intensified bending (never combined), the VP-STATIC
    Q1 Saint-Venant transverse shear and signed fibre stresses, and the
    algebra-class marks.
  - Directional k is taken out of T4-U4 and deferred to its own later unit
    (§9).
  - U4a's preview-physics-1 LIMITATIONS and table edit rides T3's B7a H-4
    wave. U4a delivers the exact sentence and table delta (§8).
  - Batch 5 is nonlinear K-D5, U4a and U5a.
- **Earlier rulings this applies:**
  - H-4: the preview-physics wording and any LIMITATIONS change occur together
    in B7 ([pressure contract](pressure-contract.md)).
  - RV1 S-7: the pressure family's end rows on arcs are published in the
    tangent frame.
  - D7: realized arcs are declined by name in `prepare_sources`.

## 1. Today

| Arc row family | Rows | Frame today | Sign today |
|---|---|---|---|
| End-force rows | `element_local_{axial_force, shear_force_y, shear_force_z, torsional_moment, bending_moment_y, bending_moment_z}` at `end_i`/`end_j`. On pressure-1 the axial row is removed (the pressure family replaces it) | **Chord frame** of the replaced span: x = d/L, y = y_reference projected (the bow direction), z = x × y | Node-on-element, the element-local DOF |
| Station rows | The same six kinds at `quarter_1`, `midspan`, `quarter_3` | Tangent frame at the station | j-side section action |
| Endpoint cuts | Feed the end stresses and the pressure family at `end_i`/`end_j`; not published as force rows | Tangent frame at the end | j-side |
| Stresses | `element_local_*_stress` at the five sections (preview-physics-1 basis `nominal_straight_beam_formula_on_arc_resultants`) | Tangent frame | j-side, signed with the resultant |
| Pressure family (pressure-1) | `pipe_wall_axial_force_v2`, `pipe_effective_axial_force_v2`, `pipe_axial_membrane_stress_v2` at five sections | Tangent | j-side, tension positive |
| Wall end action (pressure-1) | `pipe_wall_endpoint_action_v2` at `end_i`/`end_j`, = −N_w(0), +N_w(1) | Along the end tangent | Node-on-element |

So at one arc end, the force rows and the stress rows use different frames.

- **preview-physics-1** discloses this:
  - its relabel (`preview_physics.rs` `ARC_COORDINATE`, :25, :600–613)
    marks the end-force rows `arc_chord_frame`;
  - LIMITATIONS[3] says "Endpoint force rows are in the chord frame".
- **pressure-1 does not disclose it** (finding). Its chord-frame end rows
  carry `coordinate_system: "element_local"` and the straight sign text
  "positive value follows the element-local DOF at the i-end force vector".
  Their metadata cannot be told apart from tangent-frame rows. The PP T4-I7
  test compares them to T4-I7's `chord_frame_elastic` block.

## 2. The convention

**Frame.** Every row of an arc is expressed in the arc's local frame at that
row's own section, the tangent frame at arc angle ψ from authored node i:

- x = the arc tangent toward authored j;
- z = the bend-plane normal (x_i − c) × (x_j − c), normalised;
- y = z × x, toward the arc centre.

This is CB's section frame (`CURVED_BEND_SECTION_SIGN_CONVENTION`). The end
rows use it at ψ = 0 and ψ = φ.

**Signs.**

- **Cut families** (stations, endpoint cuts, the pressure family, stresses)
  keep the j-side section action: the action of the authored-j side on the
  authored-i side, with positive N as tension.
- **End families** (`element_local_*` at `end_i`/`end_j`, and the wall end
  action) are node-on-element actions in the tangent frame at their own end.
  For the elastic actions:

  end_i = −cut(0) and end_j = +cut(1).

This is the definition straight members already have, because the tangent
frame of a straight is its element-local frame at both ends. So
`element_local_*` at a member end means one thing for every member: the
node-on-element action in the member's local frame at that end.

**Why the tangent frame.**

1. **It is the section's own frame.** It gives the axial force, the shears,
   the torsion, and the in-plane (M_z) and out-of-plane (M_y) bending of the
   pipe at the weld. The chord frame differs from it by a half-turn about the
   bisector of chord and tangent. That mixes N with V_y and T with M_y,
   through φ/2 (45° on a 90° bend), and reverses V_z and M_z.
2. **One frame per location.** The end rows then agree with the end stresses,
   the endpoint cuts and the wall end action, which are already in the
   tangent frame, and with the adjacent straight's end row at the shared node.
   With exact tangency, the two members share x, so node equilibrium can be
   read across members.
3. **K-D5's groups mean what they say.** The groups |N|, hypot(V_y, V_z), |T|
   and hypot(M_y, M_z) are section quantities. In the chord frame, the "axial"
   group of an arc end holds shear.
4. **U4b needs it.** Directional SIF and the arc maximum use the in-plane and
   out-of-plane split at arc ends.
5. **The chord is not a material direction.** The chord frame depends on
   y_reference only through the bow direction.

**Why the end rows stay node-on-element, not j-side.**

- The kinds and locations are shared with straight members.
- Consumers, readers and combination algebra treat `element_local_*` at
  `end_i`/`end_j` uniformly.
- Straight members keep their bytes.
- Making arc end rows j-side would give one kind two sign conventions,
  depending on member type.

## 3. The transformation

Let (x_c, y_c, z_c) be the chord frame. y_c is the bow direction, which the
arc rule fixes from the same y_reference. Let s = L/(2R) = sin(φ/2) and
c = √(1 − s²). Then:

- at i: t_i = c x_c + s y_c, y_i = s x_c − c y_c, z = −z_c;
- at j: t_j = c x_c − s y_c, y_j = −s x_c − c y_c, z = −z_c.

The force block and the moment block of each end row transform alike,
f_tangent = R_end f_chord, where:

```
R_i = [[c,  s,  0],      R_j = [[ c, -s,  0],
       [s, -c,  0],             [-s, -c,  0],
       [0,  0, -1]]             [ 0,  0, -1]]
```

R is symmetric, orthogonal, has determinant +1 and squares to I, so it is its
own inverse. This is why "the chord rotation applied twice" reproduces today's
rows (mutant M1). In components:

- F_x^t = c F_x^c ± s F_y^c;
- F_y^t = ±s F_x^c − c F_y^c;
- F_z^t = −F_z^c;
- the moments transform the same way (+ at i, − at j).

The generator checks this closed form against independently formed frames to
1e-80, on all 28 arcs of the reference set.

**Certified route (K-D5, pre-guard Passed linear cases; FK
`certify_arc_recovery`).**

Today the end rows are `evaluate_ends` over
`local_operator(K_int,arc, chord_axes)` minus `local_loads(…, chord_axes)`.
U4a forms the end operator and equivalent loads with the tangent axes at each
end instead: rows 0–5 at end i, rows 6–11 at end j. The axes are formed at p
in ball arithmetic from the binary64 coordinates, R and y_reference,
exactly as `PressureDirection::Tangent` forms its tangent in the formation
check: `ball_chord_axes`, then s = l/(2r), c = √(1 − s²), t = c·x_c ± s·y_c.
So the pressure closure's tangent operands and the end rows share one
formation. Each published tangent end row is then a certified row of K_int,arc
under K-D5 §3.1, with its own m, r and R. The law is unchanged: the rotation
enters the operator before `evaluate_row`.

Not allowed:

- rotating already-rounded rows: that rounds twice and loses the certificate;
- lifting CB's binary64 `end_tangents()` as exact, radius-zero, coefficients
  (mutant M7);
- touching the cut rows: `sections[0..5]` already use the arc's own ball
  frame and are unchanged.

The same law applies to nonlinear K-D5's certified route (batch 5) when it
certifies arcs.

**Binary64 route (Sensitive, demoted and nonlinear-support cases).**

- `recover_curved_bend_local_forces` keeps its exact per-row sums of the
  global end forces. It rotates them with CB's binary64 arc frame at each end
  (tangent, inward and normal, the frame `arc_section_resultants` uses for the
  stations) in place of the chord transform: one formed transform, as today.
- `curved_bend_section_resultants` must rebuild the global end-j force from
  the new rows with the tangent frame at j, not with today's chord transpose.
  The cleaner choice is to pass the global end-j force and not round-trip
  through published rows. A chord transpose left in place silently rotates
  every binary64 station and endpoint cut (mutant M8).

**Groups (K-D5 §3.2).** Groups are rotation-invariant only as full 3-norms,
and R mixes x with y.

- **Change:** the four `end_i`/`end_j` groups of every arc (axial, shear,
  torsion, bending). Their q, q̂ and B are re-formed from the tangent rows.
- **Unchanged:** the predicate law and the criterion; the arc `cut_i`…`cut_j`
  groups; the wall groups (already tangent, from the cuts by
  `arc_pressure_wall`); straight, connector, support and displacement groups.
- **Also unchanged:** on pressure-1 the arc's axial end group is still formed
  from the elastic axial end row, which is not published. That is today's
  practice.

A tangent end group nearly equals its cut group, since end_i = −cut(0) by
element equilibrium, and the cut groups already pass. Status changes on arcs
are still possible, because the bounds differ. M3 is therefore re-measured on
RV30, W1 and the realistic set, in both modes. Every Passed ↔ Sensitive move
is declared, never silenced.

## 4. Metadata and labels

- **Arc end rows** carry `coordinate_system: "element_local"` and
  `basis: "recovered_from_local_element_stiffness"`, with new static sign texts:
  - end_i: "positive value follows the element-local DOF at the i-end force
    vector, in the arc tangent frame at end i (local x is the arc tangent
    toward j; local z is the bend-plane normal; local y is z cross x toward
    the arc centre); it equals minus the j-side section action at end_i"
  - end_j: the same with "j-end", "end j" and "it equals the j-side section
    action at end_j".

  Straight end rows keep their texts. The phrase "force vector" is kept, so
  existing text predicates still select end rows.
- **`arc_chord_frame` is retired.** Delete `ARC_COORDINATE` and the
  `END_FORCE_KINDS` relabel branch in `preview_physics.rs`. Remove the enum
  value with B7a (§8).
- **Unchanged texts:** `CURVED_BEND_SECTION_SIGN_CONVENTION`,
  `ARC_BASIS`, `bend_pressure`'s `WALL_END_SIGN` and the other pressure-family
  signs, and `BEND_LIMITATION`.
- **Comments.** The comments that justify the chord frame retire: lib.rs at
  `recover_curved_bend_local_forces` and :5475–5480, and the
  `bend_pressure.rs` module comment.
- **Pricing.** New static texts are priced under O-10 in T3's TEXT chain;
  `tools/retained_pricing/base/kd5_text_model.py` names the sign constants.

## 5. Staging

U4a's code is shared by both routes. H-4 holds the preview-physics-1 values
until their LIMITATIONS sentence can change with them, so U4a lands in two
stages.

- **Stage 1, the U4a PR (pressure-1).**
  - Arc end rows move to the tangent frame on pressure-1, which is not a
    reviewed table, with the §4 metadata.
  - The certified and binary64 recoveries take the end frame as a parameter
    (`Tangent` on pressure-1, `Chord` on preview-physics-1). Preview-physics-1
    is byte-identical in this stage.
  - The pressure-1 readers may then enforce the arc end-row sign texts
    (recommended, §6).
- **Stage 2, riding B7a's H-4 C4–C6 (preview-physics-1).**
  - The parameter, the `Chord` frame and `arc_chord_frame` are deleted.
  - Preview-physics-1 arcs publish the tangent rows, with §8's sentence,
    table and schema delta re-registered and re-pinned.
  - If B7a is ready when U4a is, the two stages may land as one change.

## 6. Consumers that change

- **PP.**
  - The member loop (lib.rs :5470–5790): the end frame, and the binary64
    inverse in `curved_bend_section_resultants`.
  - `certified_publication::member`, which passes the end frame to FK.
  - `preview_physics.rs` (stage 2).
  - Tests:
    - `t4_u2_pressure_references.rs`: arc end rows go to T4-I7's
      `tangent_frame` `F_y`…`M_z`. The `if arc { continue; }` at end
      stations goes, so arc end-station actions are asserted through the end
      rows, as on straights.
    - `preview_physics_runtime.rs` `b1_…`: the end rows go to the U4a oracle's
      `U4A-PP1-T0R-B1-K2/K4`. T0R's frozen chord block becomes a negative
      control, and its bytes stay unchanged.
    - lib.rs tests at :14353 and :15548.
    - `formation_check_runtime.rs` :411, which detects arcs by
      `arc_chord_frame`. Detect them by the arc stress basis.
    - `pressure_arc_readers.rs` and the corpus generator tests.
- **FK.** `certify_arc_recovery` with the tangent end operator and its unit
  tests; nonlinear K-D5's arc certificate. **CB:** none. The arc frame and
  `end_tangents` already exist.
- **Readers.**
  - The Rust reader (`result_export` `physics_evidence`/`arc_evidence`) and the
    test verifier `tests/verifiers/pressure_1.py` check only `_v2` sign texts
    on arcs, so pressure-1 needs no change.
  - Recommended in stage 1: both require the §4 texts on arc `element_local_*`
    end rows. Then the frame becomes a reader-checked statement, not
    metadata-only.
  - `preview_physics_evidence` takes the enum from the schema and table in
    stage 2.
  - The Python product paths take the Rust verdict and do not change.
- **Report package.** `wire.rs` `fresh_semantics_metadata` (:677) and its test
  token (:898) drop `arc_chord_frame`. Preview-physics-1 arc envelopes stay
  detected through `ARC_BASIS` on their stress rows.
- **Desktop.** No code. The results panel shows the new sign texts.
  `reportPackageRequest.ts` now passes arc end-row metadata (its allow-list
  never had `arc_chord_frame`); the wire gate still refuses fresh semantics.
- **Python tests.** `test_preview_physics_consumer_contract.py` :219 and :907
  (stage 2).

## 7. Blast radius

**Moves.**

- **Stage 1:**
  - `fixtures/results/pressure_v3_arc_reader_corpus.json` and the
    `sparse_result` of `fixtures/model_operations/pressure_v3_l_line_authoring.json`
    (values and sign texts), each regenerated only by its PP generator test.
  - The VP-STATIC `exact_pressure_1` package (`generate_package.py`
    mapping, `MANIFEST.json`, README):
    - arc end rows move from `chord_frame_elastic` to `tangent_frame`;
    - arc end-station section actions move from `not_published` to
      `implied_by_positive`;
    - the 168 scored cases need an independent re-review before admission
      statuses rely on them.
  - The arc end groups' K-D5 statuses, measured (§3).
- **Stage 2:** the B7a C4–C6 radius in [b7-plan.md](../../../PKG-05/DEL-05-02/Design/b7-plan.md):
  - both preview tables and `results.v0.3`;
  - the 27 committed files that carry LIMITATIONS[3]: `preview_physics.rs`,
    both tables, the schema, and 23 result, desktop and retained-precision
    fixtures;
  - the corpus bases.

  No committed preview-physics-1 result contains an arc, so no
  preview-physics-1 fixture value moves; only its text does. The two
  `invented/result_export_v0_2.json` arc rows are a historical snapshot and
  stay.

**Unchanged.**

- All displacements and rotations, and all support reactions.
- Every straight-member row.
- Arc stations and endpoint stresses, the pressure family and the wall end
  action.
- The cut and wall groups.
- Retained and source identities, which publish no arc rows (D7).
- Combination algebra: each end's frame is fixed by geometry, so the rotation
  commutes with linear combination.
- The T4-I7, T0R, T4-I6, T4-I9 and O-13 reference bytes.

## 8. B7a H-4 delta (for T3)

**LIMITATIONS[3]** (`preview_physics.rs` :76; both preview tables'
`supported_profile_limitations[3]`; `results.v0.3.schema.yaml` :1470 and
:1698).

- Old: "On arcs: signed tangent-frame resultants and nominal stress components. Endpoint force rows are in the chord frame. No maximum, and no stress headline for a model containing an arc."
- New: "On arcs: signed tangent-frame resultants and nominal stress components; endpoint force rows are node-on-element actions in the tangent frame at their end. No maximum, and no stress headline for a model containing an arc."

**Table delta** (`semantic_contract_v0_3_preview_physics_1.json` and
`semantic_contract_v0_3_preview_physics_retained_1.json`):

- `canonical_metadata_vocabulary.properties.coordinate_system.enum`:
  `["global", "element_local", "pipe_section", "TBD", "arc_chord_frame"]` →
  `["global", "element_local", "pipe_section", "TBD"]`. The retained table
  changes for vocabulary parity only, because it publishes no arc rows.
- `supported_profile_limitations[3]`: the new sentence.
- `rows`, `source_signature_count` (73) and `source_kind_count` (49) are
  unchanged. No signature depends on the coordinate system or the sign text.
- `hash_vectors` and the contract digests are recomputed by the registration.

**Schema delta** (`results.v0.3.schema.yaml`): `PreviewPhysicsResultMetadata`
`coordinate_system.enum` (:3987–3995) drops `"arc_chord_frame"`, and the two
`limitations` `const` copies take the new sentence. The Rust reader's embedded
schema and table bytes follow.

The values that ride the same wave are §5's stage 2.

## 9. What stays out of U4a

**Withheld for U4b:**

- the certified arc maximum (`pipe_elastic_normal_stress_maximum_v2` and the
  preview maximum and headline on arcs);
- the toroidal membrane hoop and the radial surface values (`withheld_on_arcs`,
  `ARC_WITHHELD_REASON`);
- directional-SIF intensified bending on arcs and markers, never combined;
- the Q1 Saint-Venant transverse shear and signed fibre stresses;
- the algebra-class marks.

U4a supplies their frame: in the tangent frame, M_z is in-plane and M_y is
out-of-plane at every arc section, ends included.

**Open unit: directional k** (deferred by HELP_HUMAN, 2026-10-12; not yet
numbered). Separate in-plane and out-of-plane flexibility factors replace the
one k for both planes. Its blast radius:

- **Solver.**
  - CB: the end flexibility, `K_b`, consistent uniform loads that share its
    strain-energy weights, the free-tip deflections and the series switch.
  - K-D5's p re-formation of `K_int,arc` (`formation_check` curved matrix).
  - The R-1 arc certificate (T4-I9 midpoint-radius enclosure;
    `validation/references/t4_i9/arc_certificate_pins.txt`,
    `arc_load_references.txt`).
- **Frozen arc references.** Every one that assumes one k:
  - T4-I6 U1 (`curved_ref.py`, round 01 and `kd5_models`);
  - T4-I7 (formulation "one k for both planes");
  - O-13's FK references;
  - T0R's B2;
  - the VP-STATIC `exact_pressure_1` package;
  - this unit's indeterminate cases. The free-tip U4a cases are statically
    determinate, so their rows do not depend on k.
- **Model and text.** The bend modifier schema, authoring
  (`bend_realization`, the inspector forms, the applier), evidence fields,
  and `BEND_LIMITATION` ("one k for both bending planes").

**D7.** No change. Realized arcs stay declined by name in `prepare_sources`.
When `pressure-source-1` admits arcs, it adopts this convention for arc end
rows.

## 10. Evidence plan

Changing a numerical meaning needs targeted regressions and independent
scrutiny by a separate agent. Both the references and their author are
separate from the implementer.

1. **PP U4a reference test.** It consumes `validation/references/t4_u4a/`
   (procedure in its README).
   - Nineteen linear cases must stay Passed, and one nonlinear gap case is the
     binary64 route.
   - It runs both modes and both entries.
   - Arc end rows are checked against `tangent_frame` at
     1e-9·max(|e|, zero_scale(member, family)), with the §4 metadata, and no
     `arc_chord_frame`.
   - Every discriminator's listed rows are checked to lie at least 1e3
     tolerances away.
   - Every unchanged family is checked at the criterion.
   - Stage 1 runs the pressure-1 cases and asserts the preview cases
     byte-identical to today. Today's product matches the oracle's
     `chord_frame` and unchanged rows within 4e-4 of the criterion.
     Stage 2 runs everything on the tangent rows.
2. **T4-I7 switch.** All 79 cases are checked against `tangent_frame`. The
   U4a engine reproduces both T4-I7 blocks to 4.7e-20.
3. **FK.**
   - On arcs whose binary64 tangents are inexact, each tangent-axis
     coefficient ball contains the oracle's 45-digit axis.
   - ‖end_i + cut_i‖ ≤ B(end_i) + B(cut_i) at both ends (certified element
     equilibrium).
   - The force and moment 3-norms of the end rows equal the chord rows' norms
     within their bounds (an orthogonality check).
   - The record's end groups are formed from the published tangent rows.
4. **K-D5 M3 re-measurement** on arcs in both modes, with declared status
   moves.
5. **The VP-STATIC package** is regenerated and independently re-reviewed.

| Mutant | Control that fails it |
|---|---|
| M1 chord frame left, or rotation applied twice | Discriminator `chord_frame_unrotated`, about 1e9 tolerances in every case |
| M2 frame swap between ends | `frame_swap_between_ends` |
| M3 sign flip at the j end; j-side cut sign at the i end | `sign_flip_at_end_j`, `cut_sign_at_end_i` |
| M4 end rows exchanged | `end_rows_swapped` |
| M5 in-plane rotation without the half-turn about x (y away from the centre, chord z) | `inplane_rotation_without_flip` |
| M6 φ instead of φ/2 | `full_angle_instead_of_half` |
| M7 binary64 tangent in place of the p-formed one | Not value-discriminable (at most 1e-7 tolerances); the FK enclosure control on `binary64_tangent_axes_inexact` arcs |
| M8 station or cut rows rotated, or the chord transpose left in the binary64 inverse | The unchanged cut and stress families, on the binary64 gap case and on the certified cases |
| M9 rotation on one route only (certified or binary64) | The gap case (binary64) together with the Passed cases (certified) |
| M10 relabel without rotation | M1's control with the metadata assertion |
| M11 end groups formed from chord rows while tangent rows are published | The FK record check (3) |
| M12 wall force or cap folded into the published elastic end rows | Pressure-1 tangent `F_y`…`M_z` (the cap is axial in the tangent frame and is not published), with the `element_local_axial_force` absence on arcs |

## Open questions

1. **Sign at arc ends.** This design keeps node-on-element at the ends, for
   uniformity with straights, rather than the j-side cut. If the manager wants
   the j-side convention at the ends, it would apply to every member type, so
   straight end rows would move too. This design does not recommend it.
2. **Staging.** Two stages or one change depends on B7a's timing (§5). This
   is the T4 manager's call, not a reserved choice.
3. **Reader enforcement.** Making the pressure-1 readers enforce the arc
   end-row sign texts in stage 1 is recommended but optional.
