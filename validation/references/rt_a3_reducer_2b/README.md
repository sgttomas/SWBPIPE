# RT-A3 reducer 2b reference package (candidate)

**Status: CANDIDATE ONLY.** This package is not adopted, not frozen, and not
product evidence. It confers no family, readiness, adoption, acceptance or release
standing. Before any product test may consume it, the additions described here
need focused independent delta scrutiny and manager integration. The 26 cleared
frame cases, their checks and their values are carried unchanged.

**Consumer.** These are future RT-A3 increment 2b product bindings, for the
bounded eccentric reducer under DEL-03-05 `Work/eccentric-reducer-offset.md`. No
product test consumes this package yet.

Read for the declared model only: selected source `74e57d55ffd4cb09b68680e36bcaca06f4ad81eb`
(branch `codex/rt-a3-reducer-2a`) on integrated baseline `2656dcb33`.

## Files

| File | Role |
|---|---|
| `generators/rt_a3_2b_reference.py` | Standard library only. The first part is the cleared frame mathematics, unchanged. The PACKAGE LAYER at the end holds every addition |
| `references.json` | Deterministic output: the cleared blocks, plus `section_recovery` and `zero_scales` in every case, one appended C3-analogue case, and the top-level `package` block |
| `SHA256SUMS` | Hashes of the generator, `references.json` and this README |

## Reproduce

```sh
python3 -I -B generators/rt_a3_2b_reference.py --check     # default; writes nothing
python3 -I -B generators/rt_a3_2b_reference.py --summary   # counts and mutant sensitivity
python3 -I -B generators/rt_a3_2b_reference.py --write     # intentional re-freeze of references.json and SHA256SUMS
```

`--check` recomputes every value and compares the bytes with `references.json`.
It also verifies `SHA256SUMS` for the three files and writes nothing. It exits
nonzero on any failed check, any undecided binary64 value, a stale
`references.json`, or a missing or mismatched hash. Paths resolve relative to the
generator. It needs no product code, no Chirality checkout, no host wrapper and
no temporary path. `--write` refuses to write while any check fails or any value
is undecided. README integrity is a byte/hash check, not semantic regeneration:
`--write` hashes its current bytes, so the stated counts and tables still need
source review when the reference contents change.

Counts: 27 cases; 822 cleared checks plus 1,275 package checks, with 0 failures;
0 undecided binary64 values (12,487 cleared emitted leaves, plus 9,072 recovery
values, 594 zero-scale values and 114 C3 result values).

## Model and basis

The mechanics are those of the cleared packet, unchanged:
- small-displacement, linear-elastic straight 3-D frame members on the actual RS-R1
  four-node neighbourhood (A = RS-170, N = RS-160, F = RS-150, C = RS-140);
- invented E = 200 GPa and ν = 0.3, with G = E/(2(1+ν));
- sections S270 (body and nbrF) and S220 (nbrN), with D-5 nominal, reduced and
  pressure-bore geometry and κ_E;
- s·E and s·G on the nominal section of the mapped body;
- caps −p·A_i·s_into at every transferring terminal, along each member's actual
  direction and at its actual node;
- the Poisson pair ±2ν·p·A_i, unscaled;
- N_w = N_m + 2ν·p·A_i and S = N_w − p·A_i.

There is no averaged axis, no projection to a declared offset and no added p·A·e.
Each member is its own pressure region, so a case is a control, not the full RS
system. The body's forces and stresses are those of the equivalent reference
chord pipe under DEC-045 (component_body attribution). They are not cone, taper
or body stresses.

### Section actions (new)

The section action at each station is the **j-side section action**: the action
exerted on the from-side part by the to-side part across the cut, in the
element-local frame. That frame is x = (x_to − x_from)/L; y is y_reference with
its x component removed, then normalized; z = x × y.

At end_i the j-side action is **minus** the from-end action on the member. At
end_j it equals the to-end action on the member. This matches the product's
straight-member "j-side section action" convention. The cleared packet's
`end_wrench_on_member_*` fields are node-on-member end actions, a different
object. The stations are end_i (0), quarter_1 (1/4), midspan (1/2), quarter_3
(3/4) and end_j (1) of the chord length.

The section actions come from two independent routes:
- **K route:** member statics from the K from-end action f_i, with no interior
  load. That gives N = −Fx_i, V = −(Fy_i, Fz_i), T = −Mx_i,
  M_y(ξ) = −My_i − ξL·Fz_i and M_z(ξ) = −Mz_i + ξL·Fy_i.
- **FM route:** the frame-free resultant, about the cut point, of every load on
  the C side of the cut, including the solved redundant. It is signed +1 when the
  member's to node lies on the C side, and −1 otherwise.

Both routes agree exactly at all five stations in all six components, and so do
the frame-free invariants N, T, |V|² and |M_⊥|². The checks also confirm:
- j-side at end_j equals the K to-end action;
- the axial action equals N_w at every station;
- the station membrane stress equals the cleared `axial_membrane_stress_Pa`;
- rotation objectivity: local actions are identical under the rational rotation
  plus translation;
- reflected actions at mirrored stations for the reversed body,
  [N, −V_y, V_z, T, −M_y, M_z];
- determinate controls: scale independence, EB equal to Timoshenko, j-side action
  at A equal to the anchor reaction plus nbrN's cap at A, and DET-COMP nbrN with
  no torsion and constant |M_⊥| = p·A_F·0.025 m;
- nonzero bending, torsion and out-of-plane components where the case is
  eccentric or out-of-plane.

### Stress rows (new; reference section, both D-5 bases)

These follow DEL-05-03 `section-bases-and-timoshenko.md` §2.1 and the T4-U4b
section quantities. For basis b, with A_b, I_b, Z_b = I_b/r_o, J_b = 2I_b and r_i,b:

| Field | Product kind | Value and sign |
|---|---|---|
| `axial_membrane_Pa` | `pipe_axial_membrane_stress_v2` (pressurized), `element_local_axial_normal_stress` (outside every region, where N_w = N) | N_w/A_b |
| `bending_normal_y_Pa` | `element_local_bending_normal_stress_y` | M_y/Z_b; positive means tension at the +z outer fibre |
| `bending_normal_z_Pa` | `element_local_bending_normal_stress_z` | M_z/Z_b; positive means tension at the −y outer fibre |
| `torsional_shear_Pa` | `element_local_torsional_shear_stress` | T·r_o/J_b, signed with T |
| `normal_extreme_plus_Pa` / `_minus_Pa` | `element_local_normal_stress_extreme` | σ_N ± hypot(σ_y, σ_z). The fibre ±r_o(−σ_z, σ_y)/hypot is not emitted |
| `transverse_shear_maximum_Pa` | `element_local_transverse_shear_stress_maximum` | \|V\|((3+2ν)r_o² + (1+2ν)r_i,b²)/(4(1+ν)I_b) ≥ 0 |
| `max_abs_normal_Pa` | informational per station | \|σ_N\| + hypot(σ_y, σ_z) |
| `member_maximum_abs_normal_Pa` | the member maximum (`pipe_elastic_normal_stress_maximum_v2` on ordinary exact straight members) | max over the member of \|N_w\|/A_b + hypot(M_y, M_z)/Z_b |

The outer-fibre normal stress is σ_x(y, z) = σ_N + σ_y·z/r_o − σ_z·y/r_o. σ± are
signed normal-stress envelopes. Neither they nor the member maximum is a
combined, Tresca, equivalent or code stress; torsional and transverse shear are
separate rows. Values are in Pa. The product's element stress rows are in MPa, so
a binding compares after exact conversion.

**Maximum location (proof, not sampling).** No member carries an interior load:
caps and the Poisson pair are end actions. So N_w, V and T are constant, and
M_⊥(ξ) is affine. ξ ↦ |M_⊥(ξ)| is the norm of an affine map, hence convex. With a
constant |N_w|/A_b, the member maximum is attained at an endpoint. The generator
decides the governing endpoint from the exact sign of |M_⊥(0)|² − |M_⊥(1)|² in the
case field:
- if the two are equal, both endpoints govern;
- if M_⊥ is constant, every station ties.

The same convexity puts max σ⁺ and min σ⁻ at an endpoint, while T and |V| are
constant. Over the 81 members, the governing location is end_i in 25, end_j in 27
and a full tie in 29. The ties are the determinate free and compensated pipes
whose M_⊥ is constant or zero.

### Exactness and emission

Field values (actions, σ_N, σ_y, σ_z, τ_T) are exact elements of the case's
multi-quadratic field times π^k, and their `exact` text is given. Hypot-type rows
have the form (a + c·√q)·π^k, with q = M_y² + M_z² or V_y² + V_z² at the same
station. Their `exact` is omitted, and their binary64 value is decided from
rigorous rational enclosures, at 70 and up to 560 digits, only when both
endpoints round to the same binary64. Comparisons that choose maxima and scales
are exact field signs, or disjoint enclosures. `decimal` strings are
informational midpoints. Zero scales retain exact expressions, with the whole
additive expression grouped before its π factor.

**Exact decimal input model, not product binary64.** The mathematical object is
the exact model with the declared decimal inputs. It is not the system the
product assembles from binary64 leaves. Product binding therefore needs the
governed scaled tolerance below, not bit equality. The geometry controls
re-evaluate decisions in Decimal on binary64-rounded inputs. That is a check of
the decision's robustness to input rounding, not product binary64 arithmetic.

## Criterion and zero scales (new)

A row passes when |observed − expected| ≤ 1e-9·max(|expected|, zero_scale). The
governing 2a tests use the same 1e-9 relative criterion, and this package
changes neither their fallbacks nor native-smoke `expected.json`.

Each scale is the largest |expected| among its family's rows in the stated
scope. A family whose rows are all exactly zero takes a dimensioned fallback
built from independent inputs and other scales, never another unit's number:

| Family (unit) | Scope and rows | All-zero fallback |
|---|---|---|
| translation (m) | case: u_x, u_y, u_z of unanchored nodes | none needed |
| rotation (rad) | case: r_x, r_y, r_z | translation / L_ref (L_ref = longest member chord) |
| reaction_force (N) | case: reaction forces | largest member-force scale of the case |
| reaction_moment (N·m) | case: reaction moments | reaction-force scale × L_ref |
| member_force (N) | member: end-wrench forces (local and global), N_w, N_m, S, station N_w, V_y, V_z | none needed |
| member_moment (N·m) | member: end-wrench moments (local and global), station T, M_y, M_z | member-force scale × member chord |
| normal_stress (Pa) | member and basis: σ_N, σ_y, σ_z, σ±, member maximum | member-force scale / A_b |
| shear_stress (Pa) | member and basis: τ_T, transverse shear maximum | member-force scale / A_b |
| pressure_stress (Pa) | Lamé block: hoop inner (≥ \|p\|) | — |

Fallbacks occur only where a family is exactly zero by statics:
- DET-FREE and the C3 analogue: rotation, reaction force and reaction moment;
- moment-free members: member moment;
- shear-free members: shear stress;
- the unloaded DET-COMP nbrF: normal stress.

Every scale is emitted with its unit and rule under `zero_scales` in each case.
No scale is chosen from product output or observed error.

**Mutant tolerance sensitivity.** The eight cleared mutants are re-solved with
the cleared K formulation and judged against the reference by the criterion
above. Each is judged across every binding row: motions, reactions, end
wrenches, N_w, N_m, S, station actions, all stress rows and maxima. Lower-bound
separations are in tolerances, and every row is decided.

| Mutant | Rows outside / compared | Largest separation in an intended family |
|---|---|---|
| drop body cap at F | 258 / 411 | 2.3e9 |
| extra p·A·e moment at N | 258 / 411 | 3.8e9 |
| Poisson pair scaled by s | 258 / 411 | 8.4e8 |
| caps on nominal bore | 258 / 411 | 4.8e7 |
| κ = 5/6 instead of κ_E | 258 / 411 | 1.7e7 |
| body caps along run axis (averaged axis) | 258 / 411 | 3.0e9 |
| nbrF cap projected to declared offset point | 258 / 411 | 4.2e8 |
| NONPAR nbrN cap along chosen axis | 411 / 411 | 2.6e8 |

In HELD-TIMO-S2, the 153 rows that are exactly zero stay exactly zero under the
first seven mutants: those are the out-of-plane components of a planar case. A
check requires each mutant to exceed 1e3 tolerances in a family fixed in
advance. Per-family counts and the strongest rows are in
`package.mutant_tolerance_sensitivity`.

## Case coverage

These are the 26 cleared cases:
- HELD: EB and Timoshenko at s = 1 and 2, plus EB at s = 4;
- DET-FREE: EB and Timoshenko at s = 1, 2 and 4;
- DET-COMP: EB and Timoshenko at s = 1 and 2;
- HELD-REV: EB and Timoshenko at s = 2;
- MECH, a general six-component load at N and F: EB and Timoshenko at s = 1 and 2;
- NONPAR, the actual tilted nbrN direction: EB and Timoshenko at s = 2;
- HELD-UNEQ: D-1 unequal closure, Timoshenko at s = 2;
- ROT-HELD and ROT-MECH: a rational rotation plus a far translation.

All 26 carry section recovery and zero scales. The package appends one case:
- **RS2B-C3-JHELD-FREE-TIMO-S2** (C3 analogue): junction N held; A, F and C free;
  1.6 MPa in every member.

K solves the full chain. FM is reused unchanged on the two determinate branches
leaving N. The case confirms:
- K agrees with FM;
- R_N is exactly zero, so no p·A·e moment arises;
- N_w = p·A_i, with no shear, torsion or bending;
- every rotation is zero, and each member stretches purely along its actual chord.

## Geometry scope

The 24 cleared W-15c controls are carried unchanged as a **subset** of admission
behaviour. They are not a complete pressure-admission proof. These boundaries
were reviewer-confirmed:
- agreement dx* = 0.0065837258 m;
- placement allowance 5.26699 mm;
- transversality a_e = 0.9077 mm;
- precision h* = 0.86747 mm.

The Decimal judge is a simplified re-derivation for these controls, not a product
implementation. An absent offset is no_check in the existing OFFSET check. That state alone
proves neither pressure admission nor pressure refusal; the applicable pressure
admission proof still has to pass. The legacy
2a concentric branch, including absent-offset back-to-back bodies, is not W-15c
run evidence. These items are left to the governed W-15c product tests:
- β_arc evidence;
- full J3 signed zero and precedence;
- J2 discrimination;
- split ties;
- not_solver_consumed and not_reducer;
- nonfinite inputs and invalid mapping;
- connectors and joints.

## What changed versus the cleared packet

The cleared mathematics is unchanged. The generator's cleared part differs from
the input generator only by five marked retention lines, which keep the mutant
solutions and the solved cases for the package layer, and by its docstring.
Every one of the 12,487 cleared leaf values reappears unchanged in
`references.json`. This was verified during development against the input
packet; the package does not read that packet at runtime. The new data is
additive:
- `section_recovery` and `zero_scales` in each case;
- one appended case;
- the `package` block, which holds row semantics, the criterion, Lamé zero
  scales, mutant sensitivity, geometry scope, emission and the 1,275 package
  checks.

The cleared `checks` (822) and `emission` blocks are unchanged.

## Independence and its limits

The generator imports, reads and runs no product code, product output or frozen
product reference. Product source was read only to map row names, stations and
signs:
- `core/product_physics/src/lib.rs`: stress-row emission and the j-side sign
  strings;
- `core/product_physics/src/u4b_rows.rs`: extreme and shear kinds;
- `core/loads/stress_recovery/src/section_stress_quantities.rs`;
- `core/product_physics/src/certified_publication.rs`: M/Z and T·r_o/J.

Every expected number comes from the exact mathematics here. The mutant
re-solves and the C3 analogue use the cleared K and FM solvers. The second route
for the section actions is the cleared FM statics, so a defect shared by both
cleared formulations' inputs, such as a wrong declared input, would not be
exposed.

## Remaining consumer work

- Focused independent delta scrutiny of the package layer, then manager
  integration and any freeze into `validation/references/`.
- Product bindings: compare pressurized axial sections to `pipe_wall_axial_force_v2`
  (N_w) and `pipe_effective_axial_force_v2` (S), and node-on-member axial end
  actions to `pipe_wall_endpoint_action_v2`. The ordinary exact straight-member
  maximum is `pipe_elastic_normal_stress_maximum_v2`, in Pa. Element stress rows
  use MPa; membrane, Lamé and member maxima use Pa. These are existing row/unit
  contracts at the selected source, not new decisions.
- Preserve DEL-03-05 §2.4's existing body rule: per-member maxima stay, while
  case maxima and headlines exclude `component_body` members. No body-stress
  or headline eligibility is chosen by this package.
- Fibre coordinates for σ± are not emitted. The values can bind to the public
  sigma_plus/sigma_minus rows; any coordinate comparison needs a separate
  explicit witness (a constant circumferential stress has no unique fibre).
- Full RS with thermal and weight, the governed W-15c product tests listed above,
  and old and new commitment scrutiny are still future work, as is the D3
  sequence (2a, dynamics, S2), where 2a owner adoption (8cd8fcf3) is still
  pending.
