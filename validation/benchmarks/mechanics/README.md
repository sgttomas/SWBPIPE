# Mechanics Benchmarks

This crate contains original mechanics verification fixtures for
`DEL-09-01 - Mechanics benchmark suite`.

The fixtures are public project content because their inputs, expected values,
and derivations are generated from elementary open mechanics within this
repository. They do not copy protected standards examples, commercial software
benchmarks, proprietary engineering values, or code-specific acceptance
criteria.

Numerical comparison values here are regression evidence for the current solver
mechanics. Release thresholds, final tolerance policy, CI gate policy, and
professional reliance remain `TBD` pending human approval.

External validation claims and benchmark publication scope also remain `TBD`.
This suite is readiness evidence for invented mechanics fixtures only.

## Fixture Unit Basis

Fixture values carry explicit unit identifiers in code under
`PKG09-FIXTURE-UNITS-EXPLICIT-N-M-RAD-K`. This is a fixture-local basis only:
it records units for evidence review and does not define project conversion
constants or the canonical unit catalog, which remain `TBD`.

## Fixture Families

The explicit source inventory is the `fixture_inventory()` list in
`src/lib.rs`; this table mirrors that inventory for review.

| Family | Fixture IDs |
|---|---|
| Cantilever | `MECH-CANTILEVER-TIP-FORCE` |
| Frame | `MECH-PORTAL-SWAY-ORIGINAL` |
| Branch assembly | `MECH-BRANCH-ASSEMBLY-THREE-MEMBER` |
| Straight pipe | `MECH-STRAIGHT-PIPE-WEIGHT-RECOVERY` |
| Support boundary | `MECH-SUPPORT-BOUNDARY-MIXED` |
| Primitive load | `MECH-PRIMITIVE-LOAD-PREP` |
| Integrated linear static | `MECH-TP-PHYS-002-LINEAR-STATIC-INTEGRATION` |
| Load-to-resultant integration | `MECH-TP-PHYS-004-LOAD-TO-RESULTANT`, `MECH-TP-PHYS-005-ORIENTED-LOAD-TO-RESULTANT`, `MECH-TP-PHYS-006-PARTIAL-SPAN-LOAD-TO-RESULTANT`, `MECH-TP-PHYS-007-STATION-SWEEP-RESULTANTS`, `MECH-TP-PHYS-009-COMBINED-LOAD-AXIAL-EFFECTS`, `MECH-TP-PHYS-014-CANONICAL-ANALYTICAL-PAYLOAD`, `MECH-TP-PHYS-015-CANONICAL-SOLVE-RESULT-ENVELOPE` |
| Thermal growth | `MECH-FIXED-FIXED-THERMAL-AXIAL`, `MECH-TP-PHYS-008-THERMAL-PRESSURE-AXIAL-EFFECTS`, `MECH-TP-PHYS-009-COMBINED-LOAD-AXIAL-EFFECTS` |
| Imposed displacement | `MECH-IMPOSED-DISPLACEMENT-SPRING` |
| Stiffness transform | `MECH-INCLINED-MEMBER-TRANSFORM` |
| Curved-bend expansion loop | `MECH-EXPANSION-LOOP-CURVED-BEND-THERMAL` |
| Curved-bend distributed load | `MECH-CURVED-BEND-DISTRIBUTED-FIXED-END` |
| Equivalent-static generation | `MECH-TP-PMM-P3-OCCLOADGEN-EQUIVALENT-STATIC` |
| Constant-effort support | `MECH-CONSTANT-EFFORT-SUPPORT-APPLIED-LOAD` |

Hand-calculation notes are in `validation/hand_calcs/mechanics/`.

`MECH-EXPANSION-LOOP-CURVED-BEND-THERMAL` (D-34 / DEC-070 exit evidence)
compares the curved-bend macro element in a plane expansion loop against the
independent force-method witness
`validation/hand_calcs/mechanics/expansion_loop_curved_bend_thermal.md`,
including the k-sweep monotonicity evidence for `k in {1, 5, 10, 20}`. The
comparison replicates the witness bending-only flexibility assumption with an
axial-rigidity boost of `1.0e5` (boost study recorded at
`EXPANSION_LOOP_AXIAL_RIGIDITY_BOOST` in `src/lib.rs`). The fixture is
registered in `fixture_inventory()` and in the shared hand-calculation
README inventory under `validation/hand_calcs/mechanics/`.

`MECH-CURVED-BEND-DISTRIBUTED-FIXED-END` (D-34 / DEC-070 arc-residual
closure evidence) compares the arc-consistent distributed-load fixed-end
forces/moments and the interior-station section resultants of the
curved-bend macro element against the independent force-method witness
`validation/hand_calcs/mechanics/curved_bend_distributed_load_fixed_end.md`
for user-entered bending flexibility factors `k in {1, 2}`, in-plane and
out-of-plane uniform loads on a clamped-clamped quarter-circle arc. Both
comparison sides are closed-form; the measured agreement sits inside the
DEC-026 analytic-class `1.0e-9` relative tier (near-zero absolute scale
floor `1.0e-3` N / N-m for the exact-zero midspan torsion row).

## Readiness Boundary

- Fixture inventory: explicit in `fixture_inventory()` and mirrored above.
- Fixture-local unit basis: explicit under
  `PKG09-FIXTURE-UNITS-EXPLICIT-N-M-RAD-K`.
- Provenance: each fixture records project-original public provenance.
- Final tolerance policy, release thresholds, CI gate policy, benchmark
  publication scope, external validation claims, and professional reliance:
  `TBD`.
