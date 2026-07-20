# T3 Rationale — DEL-04-01 Arc Pressure-Thrust for Curved-Bend Macro Spans

**Run:** `HELP-HUMAN-PIPING-20260719-MECHANICS-CAMPAIGN-R14` / W1 / T3
**Author:** WORKING_ITEMS (Agent 1, PKG-04 package manager)
**Candidate brief:** `execution/_Coordination/CANDIDATE_BRIEF_2026-07-19_T3_DEL-04-01_ARC_PRESSURE_THRUST.md`
**Lane:** D-54/`DEC-087` reasoned selection on the D-52/`DEC-085` overlay
**Date:** 2026-07-19

**Amendment record (v2):** the v1 fresh-context verifier
(`instances/W1/T3/VERIFY_BRIEF.md`, preserved) returned `BLOCK`, refuting
the v1 physics by direct integration: the radial wall load's resultant is
`pA·(t_i − t_j)`, the v1 end-tangent pair was its equilibrant rather than
its equivalent, and the v1 per-force zero-moment claim was false. The
verifier also found the fence lacked the two mirror-README inventory
lines the suite's readiness test enforces, and that the pinned fixture's
bend is not macro-realized (correcting the v1 consequence statement). The
brief was amended (v2) to the complete self-equilibrated design: end-cap
pair PLUS exact work-equivalent consistent radial wall-load vector,
equilibrium-consistent recovery with the `+pA` local-tangent
station-tension identity as the sharp check, fail-closed if the exact
consistent vector is not derivable, mirror-README lines fenced, and the
pinned-fixture statement corrected. The verifier's worked integration is
adopted as accepted basis. D-54 basis for the amended selection: the
complete system is the one project-grounded outcome that is exactly
derivable, produces the real deformation field, and reduces to the chord
treatment at small angle; material alternatives (pair-only, static
lumping, multiplier, chord retention) are rejected in §3. Claims C6, C10,
C11 restated in v2 form; C15–C17 added.

## 1. Ten-Class Fast-Reject Screen (D-52 §4.1, item by item)

1. **Irreducible owner preference / two-defensible-outcomes stop:** not
   hit. The Remaining row names the work; treatment-shape multiplicity is
   D-54 design selection (§3). No method fork requiring an owner ruling is
   touched: the arc treatment is exact statics on already-accepted
   geometry, not a new engineering method choice (contrast the friction
   path-history D-XX, which stays parked).
2. **Professional/safety/legal accountability:** not hit. Preview
   mechanics on invented fixtures; human-review posture preserved; no
   reliance outcome; hand-calc and benchmark values are regression
   evidence under the existing claim posture.
3. **Conflict ruling not determined by the authority chain:** not hit.
   The straight-chord treatment is a recorded interim decision with its
   own recorded residual directing this improvement; no accepted artifact
   defends chord-forever.
4. **Scope/boundary change, new normative content, new acceptance
   criteria:** not hit. Recorded deliverable scope; DEC-026 tier reused as
   recorded; no threshold/criterion introduced; recovery basis strings are
   truth-maintenance, not policy.
5. **Lifecycle/stage/release/acceptance/evidence-posture act:** not hit.
   `IN_PROGRESS` unchanged; suite claim posture unchanged.
6. **External/procurement/publication action:** not hit. Local, offline;
   no push/PR/merge in the tranche.
7. **Merge authority / destructive action:** not hit. Ordinary wave-branch
   commits; existing fixtures and witnesses untouched; any pinned-fixture
   output change is captured before/after with historical witnesses
   preserved.
8. **Protected/private data exposure:** not hit. No code-book SIF or
   protected tables; elementary statics and invented geometry only
   (expressly excluded in brief §9).
9. **Evidence unavailable / stale basis / claim beyond warrant:** not hit.
   The chord treatment, recovery inheritance, curved-bend API, fixture
   exposure, and suite tier facts were verified live; the §4.2 derive-first
   gate stops the tranche if the closed-form derivation cannot be
   completed exactly — the brief never lands an unwitnessed treatment.
10. **Protected domain-engine paths / prover / higher-order boundaries:**
    not hit.

**Screen result: PASS.** The deliverable is fully DAG-unblocked (all eight
execution-upstream rows `SATISFIED`), so no dependency-posture caveat is
needed for T3.

## 2. Four-Lens Analysis

- **Ontology.** The curved-bend macro element, its validated arc geometry,
  the pressure-thrust load class, and the recovery/station frames all
  already exist as governed entities; the tranche replaces one recorded
  interim treatment (chord axial) with the exact treatment on the same
  entities. No new entity, method-fork ruling, or authority class is
  created; the owner-gated second Remaining row stays a distinct object.
- **Epistemology.** The residual is recorded in the deliverable's own
  MEMORY ("Pressure thrust keeps the recorded straight-chord axial
  treatment on macro spans (stations inherit it)"). The replacement rests
  on a closed-form static identity (resultant p·A·(t_j − t_i); zero moment
  about the arc center for both systems) that the hand-calc witness must
  derive before implementation; benchmark values are tied to those closed
  forms and stay regression evidence. Claims never exceed that warrant.
- **Praxeology.** Derive-first, fail-closed ordering; single geometry
  source (the build-time validated macro element); straight-span
  invariance guards; nonlinear parity through the shared force vector;
  bounded optional refinement with a recorded fallback; truthful
  before/after capture of any pinned-fixture change. Every path is
  executable offline inside the fence.
- **Axiology.** Advances the adopted physical-model-correctness program
  (the exact treatment replaces a recorded approximation) while preserving
  prohibition integrity (no protected content, no thresholds), truthful
  attribution, reversibility, and the claim fence.

All four lenses support the same bounded outcome: author and advance this
tranche brief through the governed verify→execute→verify chain.

## 3. Materially Important Rejected Alternatives

1. **Keep the chord treatment and only document it better.** Rejected:
   the Remaining row directs treatment "beyond the recorded straight-chord
   treatment"; documentation alone discharges nothing.
2. **Single lumped bisector force** (net 2·p·A·sin(θ/2) applied at one
   point). Rejected: statically equivalent in resultant but misplaces the
   load path (all thrust at one node or mid-span), distorting reactions
   and station resultants; the end-tangent pair is equally exact in
   resultant AND moment and preserves the two-node load path.
3. **Mandatory work-equivalent (fixed-end) refinement.** Rejected as a
   requirement: the rotating radial intensity is outside the existing
   constant-vector `consistent_uniform_nodal_loads` machinery; demanding a
   new force-method derivation as a gate would over-scope the tranche.
   Kept as a bounded optional path with recorded fallback (§3.5).
4. **Multiplier-style correction** (scale the chord force by a geometry
   factor). Rejected: direction, not magnitude, is what the chord
   treatment gets wrong; a multiplier cannot produce the correct
   end-force directions and would be an invented approximation.
5. **Implement inside `core/solver/curved_bend` as load-vector API and
   rewire product_physics wholesale.** Rejected as a requirement: the
   assembly seam in product_physics is where the chord decision lives;
   additive crate helpers (now including the consistent radial-load
   vector) are the bounded path; a crate-level rewiring expands the blast
   radius without evidence gain.
6. **Defer despite no defect.** Rejected: owner-directed queue; recorded
   residual; satisfied dependency basis; no supporting risk record.
7. **(v2) Pair-only end-tangent treatment (the v1 design).** Rejected:
   refuted by direct integration — the pair is the wall load's
   equilibrant; landing it would apply inverted physics with
   self-consistent but wrong evidence.
8. **(v2) Static lumping of the complete system to the nodes.** Rejected:
   cancels identically to zero and produces no deformation — the
   spatial distribution is the physics; only the work-equivalent
   consistent vector carries it.
9. **(v2) Park the tranche on the physics BLOCK.** Rejected: the v1
   verifier's worked integration supplies the exact corrected basis, the
   required consistent-vector derivation lies inside the crate's existing
   closed-form trigonometric machinery, and the derive-first fail-closed
   gate plus a fresh v2 verification round protect against a second
   pre-specification error; parking would abandon a completable
   owner-queued item.

## 4. Enumerated Refutable Claims (for the fresh-context verifier)

- C1. DEL-04-01 `_STATUS.md ## Remaining` contains exactly two items; the
  first matches the brief's selected item; the second is the owner-gated
  G1/G2/G4+M2/M3 row the brief excludes; lifecycle is `IN_PROGRESS`.
- C2. DEL-04-01 `MEMORY.md` records the TP-PMM-P1-CURVEDBEND-004 entry
  with the exact residual sentence about pressure thrust keeping the
  straight-chord treatment on macro spans, stations inheriting it, and
  D-38 untouched.
- C3. In live `core/product_physics/src/lib.rs`,
  `add_pressure_thrust_loads` applies ±axial_load along the span's chord
  `local_x` for all spans with no macro-span branch, and an inline comment
  near the load-case assembly records the straight-chord decision.
- C4. `corrected_local_forces_for_axial_effects` applies the pressure
  axial in chord `UX`, and the bend recovery/station paths
  (`recover_curved_bend_local_forces`, `curved_bend_station_resultants`)
  receive `pressure_thrust_loads`; `pressure_thrust_for_pipe` gates
  `include_pressure_longitudinal`.
- C5. `core/solver/curved_bend` exposes validated arc geometry
  (`geometry()`, `radius()`, `included_angle()`, `orientation()`,
  `arc_section_resultants()` with x-tangent section frame) sufficient to
  derive unit end tangents without new geometry sources.
- C6 (v2). The corrected statics in brief §2 are mathematically exact for
  a circular arc: the radial wall load `q(θ) = (pA/R)·n(θ)` has resultant
  `pA·(t_i − t_j)` and zero total moment about the arc center; each
  end-cap force separately carries moment `pA·R` about the center with
  only the pair-sum vanishing; the cap pair plus wall load is exactly
  self-equilibrated; static lumping of the complete system to the two
  nodes cancels to zero; segment equilibrium of the completely loaded arc
  yields `+pA` axial along the local tangent at every interior station;
  and the treatment reduces to the chord treatment as the included angle
  goes to zero. (Consistent with the v1 verifier's worked integration.)
- C7. All eight DEL-04-01 deliverable-local `EXECUTION UPSTREAM` rows are
  `SATISFIED` (five constraints + three prerequisites).
- C8. No existing `validation/benchmarks/**` fixture combines pressure
  loading with a realized curved-bend macro span, and no
  `del1005_payload_binding_*` surface solves such a model outside the
  suites, so §3.7's del1005 byte-identity predicate is satisfiable.
- C9. The mechanics suite already records the DEC-026 analytic-class
  relative tier as its comparison basis, so the new fixture is reuse, not
  tolerance creation.
- C10 (v2). The §5 fence covers every §4 task and nothing materially
  more, including the two mirror-README inventory lines the suite's
  readiness test enforces (T2 precedent); `core/solver/curved_bend`
  writes are additive-only by predicate.
- C11 (v2). The §1 screen, read with the v2 amendment, is complete
  against D-52 §4.1 / D-54 §3.1 with no class hit: the class-9 defect the
  v1 verifier found (erroneous physics identities anchoring the v1
  gates) is cured by adopting the verifier's worked integration as the
  §2 basis, and the arc treatment remains exact statics, not an
  owner-gated method fork (friction D-XX, G1/G2/G4+M2/M3, DEC-046
  promotions untouched).
- C12. The §6 plan is executable offline in this worktree.
- C13. §10 keeps owner standing approval and agent classification
  distinct, `OwnerCaseSelection: NONE`, `EffectStatus: HELD`.
- C14. The R14 campaign plan authorizes W1 to run this tranche chain with
  per-tranche commits and no push/PR/merge/receipt at manager level; the
  single wave-level DEC-025 sweep refinement recorded in the T1 brief §6
  applies wave-wide.
- C15 (v2). The radial wall intensity's components lie inside the
  curved-bend crate's existing closed-form trigonometric integration
  basis (the `{1, cos, sin, θ, θ·cos, θ·sin}` family recorded in the
  TP-PMM-P1-CURVEDBEND-004 landing), so the exact work-equivalent
  consistent nodal vector required by brief §3.1(b) is derivable with
  existing machinery, and the fail-closed gate is real, not decorative.
- C16 (v2). The pinned tp_runner_015 solve fixture's bend is not
  macro-realized on the live tree (`mechanics_geometry_only`; no
  bend-realization data), so brief §3.7's expectation of an UNCHANGED
  pinned-case output is the truthful consequence statement, with any
  change a failure.
- C17 (v2). The two mirror READMEs
  (`validation/benchmarks/mechanics/README.md`,
  `validation/hand_calcs/mechanics/README.md`) are the only fixture
  inventory mirrors (per the T2 v3 verification at the same wave), both
  now list 22 fixtures after T2, and the v2 fence permits exactly one
  additive line in each.

## 5. Attempted Failure Mode

Attempted refutation before dispatch: classify the arc treatment as an
owner-gated method fork (class 1/class 4) analogous to the friction
path-history D-XX. The attempt fails: the friction row is gated because
competing engineering models (path-dependence) require a human method
ruling; the pressure-thrust direction on an arc has no competing model —
it is exact statics on already-validated geometry, its residual is
recorded as ordinary deliverable scope with no `(gated:)` suffix, and the
derive-first gate stops the tranche if exactness fails. A second attempt —
treating the benchmark fixture as evidence-posture promotion (class 5) —
fails because the fixture adds regression evidence under the suite's
existing recorded claim posture without touching thresholds, README
posture, or any acceptance surface.

## 6. Classification, Effect, and Preserved Gates

- **Classification:** `STANDING_APPROVAL_ELIGIBLE`; **Agent
  classification:** `CLASSIFY_ELIGIBLE`.
- **Rule activation:** `ACTIVATE_OWNER_STANDING_APPROVAL`; adoption is the
  owner's conditional act under DEC-085/D-52 §2 as refined by
  DEC-087/D-54 §1; `OwnerCaseSelection: NONE`.
- **Effect:** `HELD` pending the fresh-context verifier
  (`instances/W1/T3/VERIFY_BRIEF.md`).
- **Preserved gates:** as enumerated in brief §9/§10.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
