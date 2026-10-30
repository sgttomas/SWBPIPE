# T4-U4a arc frames and signs: frozen independent references

Frozen expectations for T4-U4a, the unit that publishes every arc row family in
one frame and sign convention ([DEL-04-07 Design, arc-frames-and-signs.md](../../../execution/PKG-04/DEL-04-07/Design/arc-frames-and-signs.md)).
They were written before any U4a product code and by a separate agent from
the one implementing it. Standard claim fence applies (F-PIP-2; claims
taxonomy per DEC-081): invented models, internal verification evidence only.

## Files

| File | Role |
|---|---|
| `u4a_engine.py` | The independent engine: unit-load (flexibility) method, 90-digit `decimal`, standard library only |
| `u4a_generate.py` | Defines the 20 invented cases, writes `requests.json` and `oracle.json`, and checks them |
| `requests.json` | One product document per case: preview-physics-1 cases are 0.2.0 pressure-free documents; pressure-1 cases are 0.4.0 documents under `3.0.0/exact_pressure_v3` |
| `oracle.json` | Expected rows per case, the discriminators, and the cross-checks; `requests_sha256` binds the exact bytes of `requests.json` |
| `SHA256SUMS` | Digests of the five files above |

Verify with `python3 -I -B u4a_generate.py` in this directory (it regenerates
both JSON files in memory and compares them byte for byte, about 20 s; `-B`
keeps the engine's import from writing `__pycache__` here) and
`shasum -a 256 -c SHA256SUMS`. `--write` replaces them and is used only for an
intended change of the references, never to absorb a product result.

## D-5 re-freeze of the pressure-1 cases (T4-U6)

The owner ruled D-5 on 2026-10-10 ("D-5 and D-6 as recommended"; DEL-05-03
Design `section-bases-and-timoshenko.md` §§1, 2.1 and 7). The eight
pressure-1 cases (section OD 0.1143 m, wall 0.00602 m, mill tolerance
0.0007 m; no corrosion allowance in the requests, which the current-format
re-authoring writes as 0) are re-frozen:

- the engine's `section_bases = "d5"` puts EA, EI, GJ and the As of the
  Poisson strain on the nominal wall and p·Ai on the pressure bore
  OD/2 − ((wall − mill) − allowance), as before;
- `derived` gives `section_bases` (nominal and reduced), `pressure_bore` and
  the stiffness section;
- the stresses are recovered on each basis in
  `stresses_SI_by_section_basis.{nominal_section, reduced_section}`, which
  replaces `nominal_stresses_SI` on pressure-1 (that key's "nominal" never
  meant a section basis);
- all rows, zero scales, discriminators and the objectivity and quadrature
  checks are recomputed. In the free-tip cases every force, moment and
  reduced-basis stress is unchanged; displacements, the nominal-basis
  stresses and the anchored cases' actions move;
- the zero-scale rule is unchanged (per member, 1e-9 × that member's own
  largest force or moment). Where the re-frozen restraint actions rose, so did
  the floors: 19 of the 152 member scales rose, by at most ×1.115
  (`U4A-PR1-Z-ANCH-NEGP`, `pipe:BEND1`, force). This is not a loosening: each
  floor stays tied to its member's own maximum.

The twelve preview-physics-1 cases keep their bytes (one section, as the
preview route keeps), and `requests.json` is unchanged, so
`requests_sha256` is unchanged. The I7 cross-check now binds the re-frozen
T4-I7 file. The re-freeze was derived without reading product code and
awaits refutation by a separate TASK.

## Cases

Twelve preview-physics-1 cases (eleven linear and the gap case) and eight
pressure-1 cases cover planar
left-turn and right-turn L lines, an L with the arc and a straight authored
against the traversal, rigidly rotated ("skew") L and U lines (including one
at 5e6 m coordinates), a U (two left turns), a Z (a left and a right turn) and
a non-planar line whose second bend leaves the first bend's plane. Loads are
tip forces and moments, weight, an in-plane side load on arcs, an interior
nodal force, uniform temperature change and, on pressure-1, 4 MPa or -0.6 MPa
with transferring or separately supported closures. Free-tip lines are
statically determinate; anchored lines are indeterminate and depend on the
bends' k. `U4A-PP1-L-LEFT-GAP` closes a one-way vertical contact at its gap
(the nonlinear route, which publishes binary64 recovery). It is the only
binary64-route case: pressure-1 refuses nonlinear supports, so all eight
pressure-1 cases are certified (see the Design's Status section).
`U4A-PP1-T0R-B1-K2` and `-K4` reproduce the mechanics of the product preview
test model `arc_model` (k = 2, 4), so that test's end-row assertion can move
here. They are not literal copies of its request: they omit its
`sif_user_value` 1.3, its orientation text and its component node, none of
which enters these rows.

## Basis and independence

- **Method.** The line is cantilevered from its first node. A section action is
  the closed-form resultant of every load beyond the cut, plus the redundant
  reaction of a restrained last node. The redundants and the displacements come
  from virtual-work integrals of N n/(EA) + T t/(GJ) + k (M·m − T t)/(EI) + n ε0.
  These use 48-point Gauss–Legendre quadrature per member. It is exact on
  straights. On arcs, two indeterminate cases agree with 64 points to 9e-88.
  There is no stiffness matrix, no curved element and no rotation of product rows.
- **Pressure.** The wall formulation of T4-I7 is used: the wetted-wall load
  pAi/R on arcs, terminal caps along the end tangents, kink forces at interior
  nodes, and the Poisson strain −2ν pAi/(E As). The section's wall force is
  N_w, and S = N_w − pAi.
- **Frames.** The engine forms the chord and tangent frames from the
  documented arc rule: the arc bows toward +y_reference projected normal to the
  chord. It also checks the Design's closed-form rotation (below) against them
  to 1e-80.
- **No product code.** The engine imports nothing from the product and reads no
  product file. Node coordinates, y_reference components and load components
  are the binary64 values written in `requests.json`, used as exact inputs.
  Section, material, radius, k, pressure and temperature are exact decimal
  targets.
- **Cross-check against T4-I7.** `u4a_generate.py` recomputes all 79 valued
  T4-I7 cases (`../t4_i7/u2_reference_cases.json`, SHA-256 `4bb9d0fa…ab66`
  after its D-5 re-freeze, read as data), both under D-5. T4-I7 was derived
  independently by a different method implementation and reviewed by T4-RV3.
  The check covers every station, every arc's `tangent_frame` and
  `chord_frame_elastic` end rows, and support A: 13,536 values. The largest
  difference is 4.2e-20 relative (4.7e-20 before D-5), against a 1e-18 bound.
  So T4-I7's `tangent_frame` block, `F_y` to `M_z`, is itself the post-U4a
  expectation for pressure-1 arc end rows. Its 79 cases need no copy here.
- **Admission check.** Before freezing, a scratch harness ran every request
  through the product of that time (main `6aec8cfd6c`, `openpipestress-runner solve`)
  in both modes. All requests were admitted. The 19 linear cases are published
  Passed; the gap case publishes binary64 recovery. The unchanged rows
  (displacements, stations, endpoint stresses, the pressure family) and
  the chord-frame arc end rows of that time all agreed with the oracle within
  4e-4 of the criterion. No product value was used to form any expectation.
  That agreement is with the pre-D-5 single section: the re-frozen pressure-1
  values differ from a product without D-5 by design, and no product run was
  made for the re-freeze.

## Oracle content (per case)

- `members.<pipe>`:
  - `frames`: chord and tangent frames at each end.
  - `chord_to_tangent_rotation`: rows are the tangent axes in chord components.
    At end_i the rows are [[c, s, 0], [s, −c, 0], [0, 0, −1]]; at end_j they are
    [[c, −s, 0], [−s, −c, 0], [0, 0, −1]], with s = L/(2R) and c = √(1 − s²).
  - `cuts_tangent_frame_j_side` at the five sections. Pressure-1 uses N_w and S.
  - `end_rows_node_on_element_elastic.<end>.tangent_frame`, which U4a
    publishes, and `.chord_frame`, today's rows.
  - `wall_end_action_along_end_tangent` (pressure-1).
  - `nominal_stresses_SI` (preview-physics-1, one section) or
    `stresses_SI_by_section_basis.{nominal_section, reduced_section}`
    (pressure-1, D-5).
  - `zero_scale`.
  - `tangent_axes_45_digits` and `binary64_tangent_axes_inexact` on arcs.
- `node_displacements` and `support_A_on_pipe` hold the support action at A.
  Restrained-last cases also give `redundant_support_D_on_pipe`.
- `discriminators`: for each wrong frame or sign, the largest distance from
  the oracle, in tolerances, over the case's published arc end rows. The
  pressure-1 published rows are `F_y` to `M_z`; preview-physics-1 adds `F_x`.
  Up to six listed rows lie at least 1e3 tolerances away. The mutants are:
  - `chord_frame_unrotated`: today's rows, and also the chord rotation
    applied twice, because the rotation is an involution;
  - `frame_swap_between_ends`;
  - `cut_sign_at_end_i` and `sign_flip_at_end_j`;
  - `end_rows_swapped`;
  - `inplane_rotation_without_flip`;
  - `full_angle_instead_of_half`.
  In every case each reaches at least 1.16e9 tolerances, except
  `full_angle_instead_of_half`, whose smallest is 4.0e8 tolerances
  (`U4A-PR1-Z-ANCH-NEGP`).
- `binary64_tangent_axes`: CB's binary64 tangent used in place of the
  p-formed one. It is 1e-7 tolerances at most, so the values cannot
  discriminate it. The control for that mutant is the FK enclosure test below.
- `objectivity_controls`: a rigidly rotated model with rotated loads has the
  same frame-local rows to about 5e-16. The 5e6 m case differs by 1.2e-9:
  one ulp of 5e6 m is 9.3e-10 m against R = 0.375 m. That case sits at the
  objectivity edge on purpose: one ulp of an input coordinate moves its rows
  by about 1.2 tolerances. Its expectations are for the authored binary64
  coordinates taken as exact, so the case tests that frames are formed from
  those coordinates with no further loss of the order of an input ulp. A
  miss of about one tolerance there is investigated as coordinate rounding,
  never absorbed by relaxing the criterion.

## Criterion

A row passes when |observed − expected| ≤ 1e-9 · max(|expected|, zero_scale)
in both solver modes and through both entries.

- `zero_scale` is per member and per family: the largest force (N, S, V) or
  moment (T, M) over its five cuts. On pressure-1 the member force scale uses
  S, not N_w; N_w enters only the case-wide fallback used when a member's own
  scale is zero.
- Displacements and supports use the case's largest value of that family.
- The product publishes translations in mm and element stresses in MPa;
  compare after exact conversion.

This is the per-group rule of T4-I7 with K-D5's member-relative scale. The
linear cases must also keep their K-D5 standing, `NUMERICAL_INTEGRITY_CHECKS_PASSED`.
A status change is a declared result to investigate, never a reason to relax
the criterion. A mismatch is the product's to fix, never the reference's.

## How a PP test consumes these

1. `include_str!` both JSON files and assert `sha256(requests.json bytes) ==
   oracle.requests_sha256`.
2. For every case, run the document unchanged through
   `run_linear_static_preview_value_with_mode` and the typed entry, in
   `SparseInteractive` and `DenseScrutiny`. On the linear cases, assert
   `NUMERICAL_INTEGRITY_CHECKS_PASSED`.
3. On every arc, assert the published `element_local_*` rows at `end_i` and
   `end_j` against `tangent_frame` (pressure-1: shear, torsion and bending only;
   `element_local_axial_force` stays absent). Assert `coordinate_system ==
   "element_local"` with the Design's arc end-row sign texts. No row may carry
   `arc_chord_frame`.
4. For each discriminator's `listed_rows`, assert that the published row is not
   within 1e3 tolerances of `mutant_value`. In particular it is not within
   1e3 tolerances of today's `chord_frame` value.
5. Assert the unchanged families at the same criterion: station cuts,
   endpoint stresses, pressure-family rows including the wall end action,
   straight members' end rows, displacements and support reactions (on
   pressure-1, each stress row against its own basis in
   `stresses_SI_by_section_basis`). On
   pressure-1, `element_local_axial_force` is absent on straights as well as
   on arcs (the `bend_pressure.rs` retain filter), so the oracle's elastic F_x
   end rows of pressure-1 straights are not asserted. This step catches a
   rotation leaking into the stations (for example a wrong inverse where the
   binary64 path rebuilds the global end-j force), or one applied on only one
   route. The gap case is the binary64 route; the others are certified. The
   gap case is preview-physics-1, so it controls pressure-1's binary64 route
   only through code shared by both routes.
6. FK enclosure control for `binary64_tangent_axes`, on arcs with
   `binary64_tangent_axes_inexact`. Each tangent-axis coefficient the certified
   arc recovery uses must be a ball containing the 45-digit value. A binary64
   tangent lifted with radius zero fails this.
