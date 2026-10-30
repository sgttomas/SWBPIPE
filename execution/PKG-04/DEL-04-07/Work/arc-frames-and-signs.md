# Arc frames and signs (T4-U4a)

This is the governing design for T4-U4a. On a realized arc, it publishes every
row family in one frame and one sign convention, on the exact `pressure-1`
route and on the ordinary `preview-physics-1` route. It changes no product
code by itself. The frozen independent references are in
[`validation/references/t4_u4a/`](../../../../validation/references/t4_u4a/README.md).
Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).

## Status

**Both stages on main (2026-10-17).** Stage 1 (pressure-1) reached main in #1333. Stage 2, the
preview-physics-1 flip with the O-13 adapter, reached main with B7a's H-4 C5 in #1387 (merge
`64c22d5510`), after B7a's actual-source D3 registration and its one current-corpus refresh: preview-physics-1
arc end rows are published in the tangent frame with §8's LIMITATIONS[3] sentence, `arc_chord_frame` is
gone from both preview tables and `results.v0.3`, and `certify_arc_recovery` takes no end-frame
parameter. B7b re-authored the pressure-1 invocations in model 0.5.0. The remaining-work list below is
therefore complete; the paragraphs that follow record the 2026-10-13 state. What stayed for U4b is §9,
which T4-U4b has since implemented ([u4b-u5b-implementation.md](u4b-u5b-implementation.md) §2). That
epoch also renamed what this file names as current: the ordinary identities are `pressure-2` and
`preview-physics-2` (`pressure-1` and `preview-physics-1` are refused by name), the verifier is
`tests/verifiers/pressure_2.py` and model 0.6.0 is current (§1 there). The text below keeps the names
U4a used.

The owner resumed T4-U4a from the controlled pause. Stage 1 is implemented
and checked; the independently reviewed stage-2 preview flip and O-13
adapter are implemented in B7a's H-4 wave. Their source, sentence, schema
and live table linkage travel together. The current B7a assembly remains
UNREGISTERED; its actual source D3 precedes the one public corpus regeneration,
D8 and consuming checks because the W1 public corpus writers require G0 admission.
These checks do not establish acceptance.

**Designed.** Sections 1–10: the tangent-frame, node-on-element convention
for arc end rows (§2), the closed-form chord-to-tangent rotation and its
formation on the certified route at p and on the binary64 route (§3), the
K-D5 group changes, the metadata and the retirement of `arc_chord_frame`
(§4), the two-stage landing (§5), consumers and blast radius (§§6–7), the
B7a H-4 delta for T3 (§8), what stays for U4b and the open directional-k
unit (§9), and the evidence plan with mutants M1–M12 (§10).

**Frozen and independently verified.** The references in
`validation/references/t4_u4a/` (frozen at `f7b1996a73`: 20 invented cases,
SHA-bound requests and oracle) were refuted by a separate TASK that did not
write them, using a different method: a direct-stiffness solve written from
the mechanics, reading the request documents directly, with member
flexibilities and load resultants by Clenshaw–Curtis quadrature (not
Gauss–Legendre) at 60 digits and its own arc frames. Result: all 20 cases
CONFIRMED. The worst agreement is 4.6e-13 tolerances (about 4.6e-22
relative). Independence and the conventions are confirmed: end_i = −cut(0),
end_j = +cut(1), y toward the centre, R_i and R_j as in §3 to 1.5e-59, and on
pressure-1 S = N_w − pA_i. Mutant separation: M1–M6 at least 4.0e8
tolerances (M6, `full_angle_instead_of_half`, is the smallest, 4.0e8 on
`U4A-PR1-Z-ANCH-NEGP`); M7 is not value-discriminable, as §10 declares; M12
is caught by the absent axial row.

**Stage 1 implemented.** FK forms the tangent end operator and equivalent
loads in ball arithmetic at p before evaluating the row; PP publishes the
tangent end rows with §4's signs on pressure-1. Preview keeps the reviewed
chord convention. Binary64 section recovery uses the transpose of the same
end-j frame as the published rows. The 79 T4-I7 cases pass in both modes and
document versions. The 20 U4a cases pass in both modes and both entries;
all 48 preview envelope serializations equal the independently sealed
pre-U4a baseline bytes. FK's 31 certified recovery tests pass, including
end/cut equilibrium, norm preservation with the scalar norm's own bound,
and actual coefficient enclosure against independent 512-bit circle
geometry on all nine arcs marked as binary64-inexact. PP tests recompute
the four end groups from the actual tangent row certificates.

**Stage-1 binary64 condition met by the intended forced-demotion hook.**
No pressure-1 case runs the binary64 route: pressure-1 refuses nonlinear
supports, so all eight PR1 cases are certified, and in stage 1 the gap case
(preview-physics-1) stays byte-identical. M8 and M9 therefore have no value
control on pressure-1 in stage 1. Stage 1 may land alone only once a
pressure-1 control on the binary64 route exists: a pressure-1 case that
publishes binary64 recovery, or a forced-demotion test hook on the pressure-1
route. The test-only hook removes `member_bound` before any certified
publication. All eight unchanged PR1 requests run through both modes and
entries, assert no certified recovery records and the named binary64
diagnostic, and check tangent end rows and unchanged cuts, stresses,
pressure, displacements and supports against the frozen oracle. No oracle
values or request bytes changed.

**Minor findings of the refutation and where they are handled.** None blocks.
- Fixed in the references' README (text only; no request, oracle value or
  generator byte changed; only the README line of `SHA256SUMS` is
  regenerated): there are twelve preview-physics-1 cases (eleven linear and
  the gap case), not eleven; the 5e6 m case sits at the objectivity edge on
  purpose (one input ulp is about 1.2 tolerances); step 5 states that
  `element_local_axial_force` is absent on pressure-1 straights too
  (`bend_pressure.rs` retain filter), so the oracle's elastic F_x there is not
  asserted; the pressure-1 member `zero_scale` uses S, not N_w; the T0R cases
  reproduce `arc_model`'s mechanics but are not literal copies of it (no
  `sif_user_value` 1.3, orientation text or component node); verification is
  run as `python3 -I -B`; M6's separation is 4.0e8, not 1e9.
- Fixed in this Design: §2 item 2's wording (only F_x and M_x are shared
  directly across a tangent node).
- Left for the next intended rewrite of the reference set, because it would
  change generator bytes: the unused `Fraction` import in `u4a_engine.py`.
- The two T0R 45-digit axis blocks assume decimal R = 0.2, whereas the
  certificate uses the authored binary64 R. Independent direct-circle
  evaluation confirms their difference (up to 5.56e-17 near zero and
  1.39e-32 near one). Those frozen bytes stay. The FK control uses exact
  binary64 operands in its independent 512-bit circle construction and
  separately derived 100-digit axis pins for those two cases; the other
  seven marked arcs retain the frozen 45-digit pins. This changes no bound
  or product comparison criterion.

**M3 re-measurement (2026-10-13).** The independently audited public-status
census compares the retained B4 `norule` captures with the immutable
evidence-only C5 tree `54334bccd60f3b94b8f52f2e93fcf9160f8a6ccb`.
It covers 22,560 RV30 arc models, 492 realistic arc models and 1,422 W1
straight models, each through the genuine typed and captured public entries
in both modes: 97,896 records with no duplicate or missing key. There are
88,716 Passed, 7,904 Sensitive and 1,276 refused records. Every status,
integrity-code, blocking-code and diagnostic-code list matches the retained
baseline: **zero Passed ↔ Sensitive moves** in either mode and zero
missing, other-integrity or runtime-error records. Refusals remain separate:
4 `PIPE_ELEMENT_INPUT_INVALID`, 216 `LOAD_INPUT_MISSING` and 1,056
`PRESSURE_REGION_NONCOLLINEAR`.

The 3,952 typed/captured differences are inherited diagnostic-code-only
Sensitive/Sensitive pairs, with zero new or changed pairs. The captured
entry adds `SOURCE_BLOCK_RECOVERY_UNAVAILABLE` in 2,937 pairs and
`EXACT_PRESSURE_V3_RETAINED_SOURCE_NOT_JOINED` in 1,015 pairs, exactly as
the retained baseline does. All 63 retained raw captures, frozen inputs,
normalized projections, candidate records and source/binary bindings were
independently checked. The census report SHA256 is
`7a2d0aa68afff9fea370bba0bf24e259eb0afdb706df229fb38d682b44c79477`.

This is a status/diagnostic comparison against retained B4 evidence. Both
public entries use `retained_entry=None`; W1 names the frozen straight-model
corpus here, and retained admission/certificates are not exercised. It is
not a current-main re-run or a full envelope/physical-value comparison.
The old full Decimal comparison helpers were unavailable. Numerical accuracy
continues to rest on the independently reviewed frozen references,
coefficient enclosures, actual group checks, closure checks and U5a's separate
unchanged-path preservation comparisons. C5 and O-13 remain in B7a's H-4
wave; this evidence tree does not activate them in the pressure-only batch.

Thirteen seeded representatives covering M1–M12 were behaviorally killed
and restored at `da6aa10898`; independent scrutiny checked their actual
production patches, failures and source restoration. No test seam or oracle
change remains. The VP-STATIC tangent mapping has its current independent
review bound by `ADMISSION.json`; the historical RV28 review is preserved
for the unchanged basis. The generator reproduces 841 files with zero
differences, all 24 package checks pass, and normalized numerical values and
rules remain identical across the review-binding regeneration. Independent
review reported no blocking findings on stage 1 `51fa5e8ac4`, stage 2
`d253b5aea2` and the O-13 adapter-only repair `1d3f1aeef0`. A separate
100-digit replay of six frozen O-13 entries and 48 groups measured at most
2.30e-7 of the unchanged 1e-9 criterion from the adapter's rounding; product
code and frozen bytes are unchanged. The current VP mapping review is
`validation/qualification/fixtures/exact_pressure/INDEPENDENT_REVIEW_T4-U4A.md`.

**Remaining work.**
1. T3 completes B7a's integration of the reviewed stage-2 commit and O-13
   adapter with H-4 C4–C6 (§§5, 8), actual source D3, the single public corpus
   regeneration and consuming checks. The remaining fixture sentence copies
   and public arc/L-line outputs are regenerated from that actual source;
   historical stored outputs are not current producer evidence.
2. B7a's test-only current-format helper verifies the frozen request SHA,
   then re-authors only each copied preview invocation's version to 0.5.0.
   The frozen requests, oracle, physical operands, group criteria and
   mandatory pressure-1 binary64 controls remain unchanged. B7b must likewise
   express the pressure-1 invocations in its one current format after the
   pressure-format retirement; it does not imply a product compatibility
   path or permission to change the frozen reference values.

**Dependencies.**
- DEL-04-01 (FK): the certified tangent end operator in
  `certify_arc_recovery`; nonlinear K-D5 (batch 5) adopts the same law when it
  certifies arcs.
- T3, DEL-05-02 B7a: the H-4 C4–C6 wave that carries stage 2, its
  registration and re-pinning (§8); B7a and B7b for the model 0.5.0
  re-expression of the requests.
- Batch 5 (nonlinear K-D5, U4a, U5a) re-registers once (H7).

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
   With exact tangency, the two members share x at that node, so F_x and M_x
   can be compared directly across members. Their y and z axes differ by a
   rotation about the shared x unless the straight's y_reference makes them
   coincide, so node equilibrium of the shear and bending rows is read after
   that rotation.
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
- **Stage 2:** the B7a C4–C6 radius in [b7-plan.md](../../DEL-04-08/Work/b7-plan.md):
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

**Withheld for U4b** (published since T4-U4b, [u4b-u5b-implementation.md](u4b-u5b-implementation.md)
§2; `withheld_on_arcs` and `ARC_WITHHELD_REASON` are removed):

- the certified arc maximum (`pipe_elastic_normal_stress_maximum_v2` and the
  preview maximum and headline on arcs);
- the toroidal membrane hoop and the radial surface values (`withheld_on_arcs`,
  `ARC_WITHHELD_REASON`);
- directional-SIF intensified bending on arcs and markers, never combined;
- the Q1 Saint-Venant transverse shear and signed fibre stresses;
- the algebra-class marks.

U4a supplies their frame: in the tangent frame, M_z is in-plane and M_y is
out-of-plane at every arc section, ends included.

**U4b's selected idealizations** (carried in the owner's 2026-10-14
resumption steer). U4b discloses two idealizations. HELP_HUMAN's ruling of
2026-10-17 fixes how each is read.

- **`bore_load_transferred_toroidal_membrane_v1`** gives the toroidal
  membrane hoop and the radial values on realized arcs.
  - The hoop at section angle β is the area average of the hoop stress over
    the wall cut at β. β is measured from +y (toward the bend centre) toward
    +z: the intrados is at 0°, the crowns at ±90° and the extrados at 180°.
  - The value comes from exact equilibrium of the pressure p on the bore,
    transferred to that cut.
  - The arc is idealized as a complete torus in an axisymmetric membrane
    state: no transverse-shear resultant on the cuts, no tangent-end effects,
    reference geometry and small strain.
  - On each section basis b the bore is that basis's own bore r_i,b, with wall
    t_b and r_o = D/2:
    σ_h(β) = (p·r_i,b/t_b)·(2R − r_i,b·cos β)/(2R − (r_i,b + r_o)·cos β), for
    R > r_o.
  - The radial values are −p at that basis's bore and 0 at the outer surface.
  - At the crowns σ_h = p·r_i,b/t_b, which is the straight member's Lamé wall
    mean on the same basis. σ_h tends to that mean as R → ∞, and in the
    thin-wall limit it reduces to the classical torus membrane hoop.
  - The nominal basis uses its own bore r_i,n (ruling (a)). The basis for this
    is D-5 as built on main: each basis's Lamé stresses are those of its own
    annulus. Arcs and straights therefore agree at the crown on each basis.
    Loading the nominal wall from the pressure bore r_i,r would break that
    agreement, so the references carry it only as a discriminator.
- **`local_straight_annulus_on_arc`** gives the Saint-Venant (SV) transverse
  shear.
  - It is the straight-annulus SV flexure field (SHEAR_REFERENCE refutation
    §2) on each basis's own annulus.
  - The tangent-frame V_y and V_z at each section drive it by superposition.
    No curvature correction is applied.
  - Its maximum, |V|·((3+2ν)·r_o² + (1+2ν)·r_i,b²)/(4(1+ν)·I_b), lies on the
    inner surface at the two fibres normal to V.
  - On straight members the same field is exact within Saint-Venant theory.
  - Torsional shear stays a separate quantity.

**Qualification (2026-10-17).** Selection was not qualification. The
references in
[`validation/references/t4_u4b/`](../../../../validation/references/t4_u4b/README.md)
supply it for both definitions and for the other quantities above where a
vetted basis exists. Their bases:
- an independent derivation of both definitions, which the generator checks
  by 3D quadrature of the bore load (hoop) and by quadrature, equilibrium and
  compatibility checks of the field (shear);
- STRESS_REFERENCE §§1 and 3–6 for fibre stresses, σ⁺/σ⁻, intensification and
  maxima (S1, S2, I1, X1, C1, C2);
- SHEAR_REFERENCE's refutation §2 for the field;
- plan 01 §5's method for the arc maximum: an analytic extremum and a
  dense-sampling bound. Each maximum is also certified at the criterion by a
  second-derivative bound.

A separate TASK refuted the set, using independent statics, an independent
Saint-Venant boundary-value solve and independently certified maxima. It
confirmed every derivation and value. A second separate TASK confirmed the
repairs and the added case.

The set has eight invented, statically determinate lines. Their resultants are
pure statics, independent of k, the shear model and the solver. They cover,
on both D-5 bases and on the preview single section:
- the hoop and radial values;
- SV shear on arcs and straights;
- signed fibre stresses, with σ⁺/σ⁻ and their fibres;
- the arc maximum and per-basis headlines;
- directional-SIF intensified bending with invented user factors: in-plane on
  M_z, out-of-plane on M_y, never combined.

The cases are request-agnostic. Once model 0.6.0 is defined, a builder
authors the requests and the U4b design maps the quantity keys to row kinds.
Oracle values do not move.

This qualifies each definition as the stated idealization only. It grants no
arc-source admission; D7's decline of realized arcs in `prepare_sources`
stands. Still open:
- **No vetted error bound.** No vetted basis bounds either idealization's
  error against shell or curved-beam theory: ovalization and the hoop bending
  that moments cause, bending boundary layers, tangent-end effects, and the
  curvature correction of the shear field. The references state this gap and
  add no inferred bound.
- **Markers.** Directional SIF at geometry-only markers has no vetted
  in-plane axis. Only the component-frame control I1 is frozen, and the
  marker mapping is the U4b design's. As built, a directional pair off a
  realized arc is refused (`BEND_DIRECTIONAL_SIF_NOT_REALIZED_ARC`,
  u4b-u5b-implementation.md §2.6).
- **For the U4b design** (decided there: no hoop or radial rows on an arc
  outside every pressure region, §2.5; the inferred conventions, §§2.2–2.6;
  the algebra-class marks, §2.7):
  - whether hoop and radial rows are published or omitted on an exact-route
    arc outside every pressure region;
  - the conventions the references' README lists as inferred: a uniform arc
    load per unit arc length, station-independent hoop rows, fibre names and
    the SIF input names;
  - the algebra-class marks. These are a classification: STRESS_REFERENCE §5
    never combines maxima or intensified measures.

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
     coefficient ball contains the independently calculated axis for its
     exact authored binary64 operands. The frozen 45-digit pins apply where
     that operand basis agrees; the two T0R decimal-radius targets use the
     independent exact-binary64 supplements described in Status instead.
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
| M8 station or cut rows rotated, or the chord transpose left in the binary64 inverse | The unchanged cut and stress families, on the binary64 gap case and on the certified cases. The gap case is preview-physics-1, so on pressure-1 in stage 1 this needs the binary64 control of the Status section |
| M9 rotation on one route only (certified or binary64) | The gap case (binary64) together with the Passed cases (certified); on pressure-1 in stage 1, as for M8 |
| M10 relabel without rotation | M1's control with the metadata assertion |
| M11 end groups formed from chord rows while tangent rows are published | The FK record check (3) |
| M12 wall force or cap folded into the published elastic end rows | Pressure-1 tangent `F_y`…`M_z` (the cap is axial in the tangent frame and is not published), with the `element_local_axial_force` absence on arcs |

## Open questions

1. **Sign at arc ends.** This design keeps node-on-element at the ends, for
   uniformity with straights, rather than the j-side cut. If the manager wants
   the j-side convention at the ends, it would apply to every member type, so
   straight end rows would move too. This design does not recommend it.
2. **Staging.** Two stages or one change depends on B7a's timing (§5). This
   is the T4 manager's call, not a reserved choice. The manager has set its
   condition: stage 1 alone needs the pressure-1 binary64-route control
   (Status section).
3. **Reader enforcement.** Making the pressure-1 readers enforce the arc
   end-row sign texts in stage 1 is recommended but optional.
