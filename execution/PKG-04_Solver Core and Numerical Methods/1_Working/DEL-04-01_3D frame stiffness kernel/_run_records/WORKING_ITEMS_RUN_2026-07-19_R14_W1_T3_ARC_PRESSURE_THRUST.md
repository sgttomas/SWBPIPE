# WORKING_ITEMS Run Record - R14-W1-T3 Arc Pressure-Thrust

Date: 2026-07-19
Agent: T3 executor (governed Agent 2, serialized, non-delegating; branch
`claude/piping-r14-pkg04-mechanics`, base commit `faee4faed`)
Deliverable: DEL-04-01 - 3D frame stiffness kernel
Package: PKG-04 - Solver Core and Numerical Methods
Campaign: `HELP-HUMAN-PIPING-20260719-MECHANICS-CAMPAIGN-R14`, wave W1,
tranche T3
Brief: `execution/_Coordination/CANDIDATE_BRIEF_2026-07-19_T3_DEL-04-01_ARC_PRESSURE_THRUST.md`
(`CB-2026-07-19-T3-DEL-04-01-ARC-PRESSURE-THRUST-001`, v2 COMMIT-SAFE;
v1 BLOCK and v2 COMMIT-SAFE verifier returns preserved at
`execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260719-MECHANICS-CAMPAIGN-R14/instances/W1/T3/`)

## Scope

Close the DEL-04-01 `_STATUS.md ## Remaining` first item: "Treat arc
pressure-thrust beyond the recorded straight-chord treatment for
curved-bend macro spans (source: Receipt 11 named remainder /
TP-PMM-P1-CURVEDBEND-004 boundaries)". The owner-gated second Remaining
item (G1/G2/G4 + M2/M3 re-disposition) is excluded and untouched.

## Selected Design (brief v2)

Complete self-equilibrated arc pressure system on macro-realized bend
spans: end-cap forces `-pA t_i` at node i and `+pA t_j` at node j (unit
end tangents from the validated `CurvedBendMacroBuild`/
`CurvedBendMacroElement` geometry — single source of truth) PLUS the exact
work-equivalent consistent nodal vector of the outward radial wall load
`q(theta) = (pA/R) n(theta)`, derived with the crate's existing
closed-form force-method machinery (the radial load's internal actions lie
in the plain `{1, cos, sin}` sub-basis). Equilibrium-consistent recovery
(equivalent-load subtraction; the ad-hoc chord `UX` pressure correction is
retired for macro spans) and stations from segment equilibrium including
the wall load's far-segment actions. The v1 pair-only design was refuted
by the fresh-context verifier (the pair is the wall load's equilibrant,
not its equivalent); the landed design adopts the verifier's worked
integration.

## Files Touched

- `core/solver/curved_bend/src/lib.rs` (additive: `end_tangents`,
  `consistent_radial_pressure_nodal_loads`,
  `arc_section_resultants_with_radial_pressure`,
  `radial_pressure_load_actions`, seven new closed-form tests; no change
  to existing stiffness/flexibility/load/station behavior or recorded
  values)
- `core/product_physics/src/lib.rs` (macro-span branch in
  `add_pressure_thrust_loads` + new `add_curved_bend_pressure_thrust_load`;
  wall-vector subtraction in `recover_curved_bend_local_forces` with the
  chord-UX pressure correction retired for macro spans;
  `curved_bend_station_resultants` carries the pressure thrust into the
  station equilibrium; truthful review-row basis
  `pressure_thrust_treatment=arc_end_cap_tangent_pair_plus_consistent_radial_wall_load`;
  three new tests + one updated basis-string test)
- `validation/hand_calcs/mechanics/curved_bend_pressure_thrust_arc.md`
  (NEW witness: cap/wall decomposition, worked integrals, exact
  self-equilibrium, zero static lumping, membrane-state station identity,
  small-angle chord reduction, closed-form fixture values)
- `validation/benchmarks/mechanics/src/lib.rs` (additive fixture
  `MECH-CURVED-BEND-PRESSURE-THRUST-ARC`, family variant, solve fn,
  witness table, two tests; fixture-count assertion 22 -> 23)
- `validation/benchmarks/mechanics/README.md` (one additive inventory line)
- `validation/hand_calcs/mechanics/README.md` (one additive inventory line)
- DEL-04-01 `_STATUS.md` (first Remaining item struck; one History entry;
  Last Updated), `MEMORY.md` (one new entry), this run record
- `execution/_Coordination/AgentRuns/.../instances/W1/T3/EXECUTE_RETURN.md`
  and `CHANGE_SCOPE_CONTAINMENT.json`

## Implemented Evidence

- Derive-first: the hand-calc witness was completed exactly (elementary
  statics, invented geometry) before implementation; every identity is
  cross-checked against the preserved v1 verifier integration and the v2
  verifier's independent re-derivation.
- Crate tests (23 passing): end-tangent frame identities; consistent
  radial vector rigid equilibrium (resultant `pA (t_i - t_j)`, zero center
  moment); cap+wall self-equilibrium about an arbitrary point at fp
  precision plus node-by-node zero static lumping; membrane state of the
  anchored arc under the complete system (end forces `+/- pA t`,
  closed-form flexibility-independent tip stretch, station axial `+pA`
  with zero shear/moment for k in {1.0, 2.5/1.75}); zero-thrust station
  reduction to `arc_section_resultants`; small-angle chord reduction;
  non-finite input rejection.
- Product tests (86 passing): assembled complete-system self-equilibrium
  on a macro span with BOTH thrust sources (pipe internal area +
  expansion-joint effective area) receiving the identical treatment;
  no-pressure invariance of the assembly seam; THE SHARP CHECK — the
  end-supported pressurized macro span reports membrane end forces,
  `+pA` station axial with vanishing shear/moment rows, `pA/A_s` station
  stress, closed-form tip stretch, preserved
  `include_pressure_longitudinal` gating (hoop row present, longitudinal
  row absent); nonlinear active-set parity through the shared assembled
  force vector; truthful review-row basis string (old string asserted
  absent). All pre-existing tests unchanged and passing (straight-span
  pressure closed forms; curved-bend no-pressure oracles).
- Benchmark fixture (36 suite tests passing): measured normalized
  deviation ~1.5e-11 at the recorded DEC-026 analytic-class 1.0e-9
  relative tier (fixture-local 1.0 N near-zero floor for exact-zero
  targets — single-ulp closed-form roundoff headroom); readiness mirror
  test enforces the two README inventory lines.
- Guards: pinned `tp_runner_015_final_cli_solve_input.json` stdout
  byte-identical before/after (its bend is `mechanics_geometry_only`, not
  macro-realized); five `del1005_payload_binding_*` outputs byte-identical
  to committed witnesses before and after.

## Boundaries

- No D-38 temperature-semantics change, no DEC-046/tolerance promotion, no
  SIF/flexibility-factor change, no existing-fixture recorded-value change,
  no threshold/tolerance-policy creation (the DEC-026 tier is reused as
  recorded), no owner-gated row work, no protected standards content.
- No lifecycle transition, release-readiness claim, professional approval,
  certification, sealing, authentication, or code-compliance claim.
- No commit, push, PR, merge, or network action by this executor; the W1
  manager commits after independent verification.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
