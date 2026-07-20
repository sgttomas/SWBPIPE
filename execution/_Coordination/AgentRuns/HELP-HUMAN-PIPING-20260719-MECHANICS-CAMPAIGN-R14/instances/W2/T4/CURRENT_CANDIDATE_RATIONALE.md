# T4 Rationale — DEL-05-01 Sub-Span (Partial-Extent) Wind Exposure

**Run:** `HELP-HUMAN-PIPING-20260719-MECHANICS-CAMPAIGN-R14` / W2 / T4
**Author:** WORKING_ITEMS (Agent 1, PKG-05 package manager)
**Candidate brief:** `execution/_Coordination/CANDIDATE_BRIEF_2026-07-20_T4_PKG05_SUBSPAN_WIND.md`
**Lane:** D-54/`DEC-087` reasoned selection on the D-52/`DEC-085` overlay
**Date:** 2026-07-20

**Amendment record (v2):** the v1 fresh-context verifier
(`instances/W2/T4/VERIFY_BRIEF.md`, preserved unsoftened) returned
`BLOCK` on exactly one premise set: the v1 §3.6/§5-item-5 "additive
only / no existing assertion weakened" test-edit constraint is
unsatisfiable with the live `tests/test_model_schema.py` wind
required-set assertion (lines 450–458), which the brief's own `anyOf`
required-set relaxation forces to be rewritten. The brief was amended
(v2) to authorize exactly ONE named, equal-or-stronger, non-additive
assertion replacement (three-key required subset + exact `anyOf` shape +
both `minItems` + `additionalProperties: false`), with everything else
additive-only. The verifier confirmed all other claims, independently
re-derived the lever-rule statics, and verified the fence's path
coverage in both directions. D-54 basis for the amended selection: the
equal-or-stronger replacement is the one project-grounded outcome that
keeps the canonical schema truthful (at least one marking form) without
weakening total verification strength; the material alternative —
keeping `exposed_element_refs` always-required — defeats the selected
sub-span capability (a spans-only document would stay invalid) and is
rejected. Claims C11 and C16 are restated/added in v2 form below.

## 1. Ten-Class Fast-Reject Screen (D-52 §4.1, item by item)

1. **Irreducible owner preference / owner-only choice:** not hit. The
   Remaining row names the work verbatim; the design multiplicity
   (extent representation, lumping tier, bend handling) is D-54 design
   selection among internal, fence-respecting means (§3). No engineering
   method fork requiring a human ruling is touched: the lever rule is
   exact elementary statics at the tier the preview path already uses,
   not a competing model choice.
2. **Professional/safety/legal accountability:** not hit. Preview
   mechanics from user-entered inputs on invented fixtures; human-review
   posture preserved; hand-calc and benchmark values are regression
   evidence under the existing claim posture; no reliance outcome.
3. **Conflict ruling not determined by the authority chain:** not hit.
   DEC-068 item 2 rules the wind-generation class; the landed tranche's
   own boundary record names sub-span marking as the deliberate residual;
   no accepted artifact defends whole-span-forever.
4. **Scope/boundary change, new normative content, new acceptance
   criteria:** not hit. The Remaining row (Receipt 6 residual /
   TP-PMM-P3-OCCLOADGEN-001 §Boundaries / DEC-068 item 2) is recorded
   deliverable scope with no `(gated:)` suffix; the extension stays
   inside "user-marked spans" under the ruled shape; fraction vocabulary
   is reused from the accepted canonical `$defs/ElementLoadSpan`; no
   threshold, criterion, or code content is introduced; the canonical
   schema edit is additive with every currently-valid document remaining
   valid.
5. **Lifecycle/stage/release/acceptance/evidence-posture act:** not hit.
   `IN_PROGRESS` unchanged; suite claim posture unchanged; DEC-026 tier
   reused as recorded.
6. **External/procurement/publication action:** not hit. Local, offline;
   no push/PR/merge inside the tranche (the W2 manager commits locally;
   HELP_HUMAN performs fan-in acts).
7. **Merge authority / destructive action:** not hit. Ordinary
   wave-branch commit after independent verification; frozen witnesses
   and existing recorded values untouched by predicate.
8. **Protected/private data exposure:** not hit. No code-book wind
   coefficients, exposure categories, or protected tables — expressly
   excluded (brief §9); elementary statics and invented geometry only.
9. **Evidence unavailable / stale basis / claim beyond warrant:** not
   hit. Every outcome-determining premise was verified live at HEAD
   `581a15b1c` (generator/application/schema shapes, witness-input grep,
   dependency rows, D-45 register state); the derive-first gate (§4.2)
   stops the tranche if any closed form cannot be completed; the
   whole-span invariance and del1005 byte-identity predicates keep
   claims within warrant.
10. **Protected domain-engine paths / prover / higher-order boundaries:**
    not hit.

**Screen result: PASS.** Dependency posture: the five root `EXECUTION
UPSTREAM` constraint rows (`DAG-002-E0130..E0134`) are `SATISFIED`; the
single `PENDING` execution row (`TP-DAG-004-DEL-05-01-E001`) is a
DOWNSTREAM interface obligation toward DEL-05-02 (boundary preservation),
which this tranche honors by not touching load-case algebra; it is not an
upstream blocker.

## 2. Four-Lens Analysis

- **Ontology.** Wind occasional-load generation, the marked-span
  exposure concept, the fraction-extent vocabulary
  (`ElementLoadSpan`), the statically-equivalent preview lumping tier,
  and the curved-bend macro realization all already exist as governed
  entities. The tranche extends one existing entity (exposure marking)
  with an existing vocabulary (fraction extents) and applies existing
  statics; it creates no new entity class, method fork, authority class,
  or acceptance object. Whole-span marking, sub-span marking, and the
  bend fail-closed boundary remain distinct, named things.
- **Epistemology.** The residual is recorded in the deliverable's own
  run record ("sub-span wind marking is a possible future extension,
  deliberately not invented here"), `_STATUS.md`, and the DEC-068
  boundary lineage. The extension rests on closed-form elementary
  statics (partial-load resultant, centroid, lever-rule end shares,
  exact 50/50 reduction at full extent) that the hand-calc witness must
  derive before implementation; benchmark values tie to those closed
  forms and stay regression evidence. The claim never exceeds the
  statically-equivalent tier the preview path already occupies.
- **Praxeology.** Derive-first, fail-closed ordering; `Option`-threaded
  extent so whole-span behavior is structurally unchanged; blocking
  diagnostics (existing code families) for every invalid, conflicting,
  overlapping, or unsupported marking; curved-bend partial extents
  blocked rather than approximated; deterministic ids; bounded fence;
  offline checks; independent verification before commit. Every path is
  executable in this worktree.
- **Axiology.** Advances the adopted physical-model program (users can
  model partially-exposed spans truthfully instead of over- or
  under-marking whole spans) while preserving prohibition integrity (no
  code content, no defaults), evidence over plausibility (witnessed
  closed forms), truthful attribution, reversibility, and the claim
  fence.

All four lenses support the same bounded outcome: author and advance
this tranche brief through the governed verify→execute→verify chain.

## 3. Materially Important Rejected Alternatives

1. **Scale the whole-span intensity by the exposed fraction**
   (emit `w·(b−a)` uniform over the full span). Rejected: preserves the
   resultant but misplaces its centroid — end shares are wrong for any
   asymmetric extent; a silent approximation where the exact lever rule
   is equally cheap.
2. **Emit end nodal forces directly from the primitive_loads generator.**
   Rejected: the generator has no node indices or span lengths; nodal
   emission would bypass the element-load category machinery
   (category checks, application seams, bend detection) and hide the
   load's distributed nature from every downstream consumer.
3. **Upgrade partial-extent wind to work-equivalent (fixed-end) vectors**
   via the `core/solver/straight_pipe` spanned machinery. Rejected: the
   preview tier applies ALL generated distributed loads as
   statically-equivalent forces (50/50 whole-span); introducing fixed-end
   moments only for partial wind creates a mixed-rigor seam and changes
   the established tier without a directing record. A uniform tier
   upgrade would be its own lawful selection.
4. **Partial-arc consistent integration for curved-bend macro spans.**
   Rejected as over-scope: the arc-consistent machinery integrates full
   uniform intensity; a partial-arc closed form is new derivation work
   with no directing residual. Fail-closed blocking preserves truth and
   keeps whole-span bend wind exact.
5. **Subdivide elements at exposure boundaries** (mesh change).
   Rejected: changes node/element identity, result addressing, and every
   downstream surface for a load-application concern; massive blast
   radius for no evidence gain at the preview tier.
6. **New `LoadTarget` enum variant for extent-bearing loads.** Rejected:
   breaks exhaustive matches across in-repo consumers and misstates the
   ontology — the target is still one element; the extent qualifies the
   load, not the target. The additive `Option<LoadExtent>` field keeps
   every existing constructor and match valid.
7. **Mark extents by absolute lengths instead of fractions.** Rejected:
   the accepted canonical vocabulary (`ElementLoadSpan`,
   `FractionQuantity`, DEL-05-05 lineage; solver-tier `UniformLoadSpan`)
   is fractional; absolute lengths add unit/length-consistency failure
   modes and a second vocabulary for the same concept.
8. **Defer despite no defect.** Rejected: owner-directed R14 queue names
   this row; the residual is recorded ordinary scope; constraints are
   satisfied; no risk record supports deferral.

## 4. Enumerated Refutable Claims (for the fresh-context verifier)

- C1. DEL-05-01 `_STATUS.md` is `IN_PROGRESS` with exactly one
  `## Remaining` item, matching the brief's quoted selected item.
- C2. The landed whole-span design and the deliberate sub-span boundary
  are recorded in
  `_run_records/WORKING_ITEMS_RUN_2026-07-10_TP-PMM-P3-OCCLOADGEN-001.md`
  ("Wind marking is whole-span (per pipe segment); sub-span wind marking
  is a possible future extension, deliberately not invented here"), and
  DEC-068 item 2 (SOFTWARE_DECOMP §12) rules the user-marked-span wind
  generation class this extension stays inside.
- C3. In live `core/loads/primitive_loads/src/lib.rs`:
  `ElementExposedDiameter` has exactly `element_index` and
  `exposed_diameter` fields (no extent);
  `generate_wind_equivalent_static_loads` emits only whole-element
  uniform loads with ids `{case_ref}:generated:wind:{axis}:element-{i}`;
  `PrimitiveLoad` is constructed by struct literal only inside the crate
  (helpers everywhere else); `prepare_lumped_nodal_loads` lumps
  `magnitude·span/2` to each end node.
- C4. In live `core/product_physics/src/lib.rs`: `WindGenerationInput`
  has exactly `pressure`, `shape_factor`, `direction`,
  `exposed_pipe_refs`; `append_equivalent_static_generated_loads` blocks
  on missing inputs/unknown refs and computes exposed diameter as
  outside diameter plus twice insulation thickness;
  `add_uniform_element_loads` applies 50/50 static end shares on straight
  spans and exact arc-consistent vectors (with blocking fallback) on
  curved-bend macro-realized spans; pipe segments map one-to-one to
  element indices.
- C5. In live `core/product_physics/src/validation.rs`, the
  equivalent-static provenance requirement and the at-least-one-load
  check are block-level and enumerate no wind sub-field, so the §3.5
  additions require no validation.rs change unless an additive edit
  proves strictly necessary.
- C6. `schemas/model.schema.yaml` already defines
  `$defs/ElementLoadSpan { start_fraction, end_fraction }` over
  `$defs/FractionQuantity`, consumed by
  `ElementUniformDistributedForceLoadRecord.span`; and
  `WindEquivalentStaticInput` currently requires `pressure`,
  `shape_factor`, `direction`, `exposed_element_refs` with
  `additionalProperties: false`. The §3.6 `anyOf` relaxation keeps every
  currently-valid document valid.
- C7. The lever-rule closed forms in brief §3.3 are mathematically exact
  statics: for intensity `w` over fractions `[a,b]` of span `L`, the
  resultant is `W = w·(b−a)·L` at centroid fraction `c=(a+b)/2`, and the
  statically-equivalent end shares are `R_i = W·(1−c)`, `R_j = W·c`,
  reducing exactly to `w·L/2` each at `(0,1)`; disjoint extents
  superpose.
- C8. No `validation/witness/inputs/*.json` contains an
  `equivalent_static` block, so the §3.9 del1005 byte-identity and
  unchanged-witness predicates are satisfiable, and no witness-facing
  solve change is expected.
- C9. The mechanics suite already records the DEC-026 analytic-class
  relative tier as its comparison basis and enforces the two README
  inventory mirrors via its readiness test, so the new fixture plus one
  additive line per mirror is lawful reuse, not tolerance creation
  (T2/T3 precedent at wave 1).
- C10. The five DEL-05-01 root `EXECUTION UPSTREAM` constraint rows
  (`DAG-002-E0130..E0134`) are `SATISFIED`; the sole `PENDING` execution
  row `TP-DAG-004-DEL-05-01-E001` is a DOWNSTREAM interface row toward
  DEL-05-02; `execution/_DAG/_LATEST.md` resolves to approved `DAG-007`.
- C11 (v2). The §5 fence covers every §4 task and nothing materially
  more (including the benchmark-suite literal-construction updates
  forced by the §3.1 additive fields, at the verifier-inventoried
  construction sites), `core/solver/diagnostics` needs no edit because
  no new finding code is introduced, and — with the v2 single named
  assertion-replacement carve-out — the §3.6/§5-item-5 test-edit
  constraints are satisfiable on the live tree.
- C12. The §6 plan is executable offline in this worktree (the same
  registered-check and cargo/pytest surfaces just ran clean for wave 1
  at the merged head).
- C13. §10 keeps owner standing approval and agent classification
  distinct, `OwnerCaseSelection: NONE`, `EffectStatus: HELD`, with
  adoption remaining the owner's conditional act under the SHA-bound
  D-52 §2 record as refined by D-54 §1.
- C14. The R14 campaign plan authorizes W2 to run this tranche chain
  with per-tranche commits and no push/PR/merge/receipt at manager
  level; the single wave-level DEC-025 sweep refinement recorded in the
  W1 T1 brief §6 and W1 return applies to W2 identically per the
  controlling HELP_HUMAN W2 dispatch.
- C15. D-45 is `AWAITING_RULING` (register row; packet
  `D-45_temperature_indexed_shear_modulus.md`), so DEL-05-02 is parked
  and excluded; the DEL-05-04 conditional row's condition is verified
  NOT MET at `instances/W2/T5/CONDITION_VERIFICATION.md`, so no DEL-05-04
  work is selectable this wave.
- C16 (v2). The single authorized replacement of the
  `tests/test_model_schema.py` wind required-set assertion (currently
  lines 450–458) under the §3.6 (v2) equal-or-stronger specification
  preserves or increases total verification strength: every fact the
  current assertion set verifies remains verified except the one fact
  the schema change makes deliberately false (`exposed_element_refs`
  always-required), and the replacement adds verification of the `anyOf`
  marking rule and the `exposed_spans` shape that the current suite
  cannot express.

## 5. Attempted Failure Mode

Attempted refutation before dispatch: classify sub-span exposure as an
owner-gated scope/boundary change (class 4) on the theory that DEC-068
item 2 ruled only whole-span marking. The attempt fails: the ruling's
own language is "per user-marked spans" without a whole-span
restriction; the whole-span limitation was an implementation boundary the
landing tranche recorded as a deliberate residual ("possible future
extension"), the Remaining row directing exactly this extension has no
gate suffix and cites the ruling as its source, and the extension adds
no code content, threshold, or acceptance surface. A second attempt —
classify the lever-rule lumping as a new engineering-method choice
(class 1/3) analogous to the friction path-history D-XX — fails because
the statically-equivalent force-lumping tier is already the recorded
preview treatment for every generated distributed load; the lever rule
is that same tier evaluated exactly for a partial extent, with the
work-equivalent upgrade and partial-arc bend integration expressly
excluded and preserved for future lawful selections.

## 6. Classification, Effect, and Preserved Gates

- **Classification:** `STANDING_APPROVAL_ELIGIBLE`; **Agent
  classification:** `CLASSIFY_ELIGIBLE`.
- **Rule activation:** `ACTIVATE_OWNER_STANDING_APPROVAL`; adoption is
  the owner's conditional act under DEC-085/D-52 §2 as refined by
  DEC-087/D-54 §1; `OwnerCaseSelection: NONE`.
- **Effect:** `HELD` pending the fresh v2 verifier
  (`instances/W2/T4/VERIFY_BRIEF_V2.md`); v1 BLOCK preserved at
  `instances/W2/T4/VERIFY_BRIEF.md`.
- **Preserved gates:** as enumerated in brief §9/§10.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
