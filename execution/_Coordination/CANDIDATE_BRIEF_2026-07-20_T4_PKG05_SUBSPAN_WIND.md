---
doc_id: CB-2026-07-20-T4-PKG05-SUBSPAN-WIND-001
doc_kind: coordination.candidate_brief
status: adopted_effective_execution_released
prepared: 2026-07-20
package_id: PKG-05
deliverable_id: DEL-05-01
decision_basis: DEC-068 item 2 (D-36 ruling), DEC-018 (unit normalization), DEC-026 (reused comparison tier), DEC-046 (preserved gate), DEC-065, DEC-025, DEC-081, DEC-085, DEC-087
agent_classification: classify_eligible
rule_activation: activate_owner_standing_approval
---

# CANDIDATE Brief — T4 DEL-05-01 Sub-Span (Partial-Extent) Wind Exposure in Occasional-Load Generation

**Status:** `EFFECTIVE (v2) — EXECUTION RELEASED BY W2 MANAGER UNDER THE R14 CAMPAIGN CHAIN (V2 COMMIT-SAFE)`

**Amendment record (v2, 2026-07-20):** the first fresh-context verifier
(`instances/W2/T4/VERIFY_BRIEF.md`, preserved) returned `BLOCK`, refuting
one premise set: the v1 §3.6 "additive assertions … no existing assertion
is weakened" condition (mirrored in fence §5 item 5 "additive only") is
jointly unsatisfiable with the live `tests/test_model_schema.py` wind
required-set assertion (lines 450–458, which asserts
`exposed_element_refs` is a member of `WindEquivalentStaticInput`'s
`required`), because the §3.6 `anyOf` relaxation necessarily removes that
key from `required` and therefore forces a rewrite of that one existing
assertion. §3.6 and §5 item 5 are amended to authorize exactly ONE named,
equal-or-stronger, non-additive assertion replacement in
`tests/test_model_schema.py` (specified in §3.6); everything else remains
additive-only. All other v1 claims, predicates, fence lines, screens, and
the lever-rule physics were confirmed by the verifier's independent
re-derivation and live-tree checks. No other section changed in meaning.

**Prepared by:** WORKING_ITEMS (W2, PKG-05 package manager) for HELP_HUMAN

**Current run:** `HELP-HUMAN-PIPING-20260719-MECHANICS-CAMPAIGN-R14`, wave W2, tranche T4

**Selected work item:** the DEL-05-01 `_STATUS.md ## Remaining` sole item:
"Extend wind marking to sub-span (partial-extent) exposure in
occasional-load generation — whole-span exposed_pipe_refs marking landed
via TP-PMM-GUIEMIT-001 (source: Receipt 6 residual /
TP-PMM-P3-OCCLOADGEN-001 §Boundaries / DEC-068 item 2)".

This brief is authored under the D-54/`DEC-087` reasoned-selection lane on
the D-52/`DEC-085` standing-approval overlay. Adoption remains the human
owner's conditional act under the standing rule; this document classifies
and proposes only. The adoption effect is `HELD` until independent
refutation returns `COMMIT-SAFE` and the W2 manager progresses the chain
under the R14 campaign-plan execution rules. No execution writes are
authorized by this document in its current state.

## 1. Purpose and Accepted Basis

Purpose: extend the DEC-068 item-2 wind static-equivalent occasional-load
generation so a user may mark a partial extent of a pipe span as
wind-exposed — not only whole spans — with the generated load applied as
the statically-equivalent end-node forces of the partial uniform intensity
(lever rule), whole-span behavior byte-unchanged, curved-bend
macro-realized spans fail-closed for partial extents, all inputs
user-entered, and no code wind profile, exposure category, coefficient
catalog, threshold, or default introduced anywhere.

The later executor must resolve paths from the active checkout:

```text
REPO_ROOT=$(git rev-parse --show-toplevel)
WORKING_ROOT=${REPO_ROOT}/projects/chirality-piping
```

All relative paths below are relative to `WORKING_ROOT` unless stated.

Accepted basis, verified against the live tree at brief preparation
(HEAD `581a15b1c718fd444870f13e75fc7cd974518670`, branch
`claude/piping-r14-pkg05-loads`, post-wave-1 main including the merged
R14-W1 PKG-04 tranches and Receipt-61):

- root and project `AGENTS.md`; active workplan
  `loop/WORKPLAN_2026-07-18b_piping_loop.md`; R14 campaign plan
  (`execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260719-MECHANICS-CAMPAIGN-R14/ORCHESTRATION_PLAN.md`,
  W2 = PKG-05 manager; serialized author→verify→execute→verify chains;
  HUMAN selection authority for the mechanics-lane queue);
- `execution/_DAG/_LATEST.md` → approved `DAG-007`;
- DEL-05-01 deliverable folder (under
  `execution/PKG-05_Loads, Load Cases, and Stress Recovery/1_Working/DEL-05-01_Primitive load case engine/`):
  `_STATUS.md` (`IN_PROGRESS`; single Remaining item, quoted above),
  `MEMORY.md` (D-41 R5 T7 PDU-054 current declaration: "The
  primitive-load slice now includes explicit loads and the DEC-068
  equivalent-static wind, seismic, and occasional generators"),
  `Dependencies.csv` (five root `EXECUTION UPSTREAM` constraint rows
  `DAG-002-E0130..E0134` all `SATISFIED`; the sole `PENDING` execution row
  `TP-DAG-004-DEL-05-01-E001` is a DOWNSTREAM interface row toward
  DEL-05-02, not an upstream blocker; retired legacy rows are `RETIRED`),
  and `_run_records/WORKING_ITEMS_RUN_2026-07-10_TP-PMM-P3-OCCLOADGEN-001.md`
  (records the landed whole-span design and the deliberate boundary:
  "Wind marking is whole-span (per pipe segment); sub-span wind marking is
  a possible future extension, deliberately not invented here");
- ruling basis: `DEC-068` item 2 (D-36; user-entered wind pressure/shape
  parameters project onto exposed diameter per user-marked spans, pure
  mechanics from user inputs, blocking diagnostics on missing inputs) —
  sub-span marking is still user-marked-span wind generation under this
  ruling; dynamics disposition stays with `D-12` and is untouched;
- generation ground truth (live source):
  - `core/loads/primitive_loads/src/lib.rs` —
    `WindEquivalentStaticBasis`, `ElementExposedDiameter { element_index,
    exposed_diameter }`, `generate_wind_equivalent_static_loads`
    (whole-element `PrimitiveLoad::uniform_element_load` per marked span,
    id `{case_ref}:generated:wind:{axis}:element-{i}`, intensity
    `w = pressure * shape_factor * exposed_diameter`), `PrimitiveLoad`
    (helper-constructed everywhere outside the crate; struct literals only
    in-crate), `ElementUniformLoadContribution`, `prepare_loads` /
    `prepare_element_load`, and `prepare_lumped_nodal_loads` (50/50
    half-total end lumping over `ElementLoadSpan.span`);
  - `core/product_physics/src/lib.rs` — `WindGenerationInput { pressure,
    shape_factor, direction, exposed_pipe_refs }` (serde-default preview
    input), DEC-018 normalization of the wind quantities
    (`normalize_dimensionless_quantity` for `shape_factor`),
    `append_equivalent_static_generated_loads` (missing-input aggregation
    `EQUIVALENT_STATIC_INPUT_MISSING`, unknown-ref blocking
    `EQUIVALENT_STATIC_INPUT_INVALID`, exposed diameter = outside diameter
    plus twice insulation thickness, one-element-per-pipe-segment index
    map), `add_uniform_element_loads` (straight spans: 50/50 static end
    shares `w·L/2`; curved-bend macro-realized spans: exact arc-consistent
    `consistent_uniform_nodal_loads` with blocking fallback, no silent
    drop), `curved_bend_uniform_intensity_by_pipe` (bend recovery
    intensity map), and `prepare_loads` consumption at the solve seam;
  - `core/product_physics/src/validation.rs` — equivalent-static
    provenance is required at the block level
    (`expect_public_preview_provenance("equivalent-static-generation", …)`)
    and the at-least-one-load check tests only `equivalent_static`
    presence; neither enumerates wind sub-fields;
- canonical-schema ground truth: `schemas/model.schema.yaml` —
  `WindEquivalentStaticInput` (required `pressure`, `shape_factor`,
  `direction`, `exposed_element_refs` minItems 1) and the EXISTING
  partial-extent vocabulary `$defs/ElementLoadSpan { start_fraction,
  end_fraction }` with `$defs/FractionQuantity`, already consumed by
  `ElementUniformDistributedForceLoadRecord.span` (DEL-05-05 lineage) —
  the accepted project shape for sub-span extents on an element;
- partial-span precedent at the solver tier (read-only context, not this
  tranche's surface): `core/solver/straight_pipe` `UniformLoadSpan` /
  `SpannedGlobalUniformLoad` and the witnessed
  `validation/hand_calcs/mechanics/tp_phys_006_partial_span_load_to_resultant.md`;
  `core/loads/user_loads` span-validated user loads. The preview solve
  path deliberately remains at the statically-equivalent force-lumping
  tier for generated distributed loads (no fixed-end moments), and this
  tranche stays on that tier;
- mechanics evidence homes: `validation/hand_calcs/mechanics/` (existing
  witness `tp_pmm_p3_occloadgen_equivalent_static.md`) and
  `validation/benchmarks/mechanics/` (fixture
  `MECH-TP-PMM-P3-OCCLOADGEN-EQUIVALENT-STATIC` consumes the
  primitive_loads generators and lumping; suite claim posture: recorded
  comparison values are regression evidence at the recorded DEC-026
  analytic-class relative tier; thresholds/tolerances/CI policy remain
  `TBD` owner-gated; the two README inventory mirrors are enforced by the
  suite's readiness test — T2/T3 precedent);
- frozen witness surfaces (read-only): the five
  `validation/witness/inputs/del1005_payload_binding_*_input.json` /
  `validation/witness/generated/del1005_payload_binding_*.json` pairs and
  the three historical `tp_runner_015_final_cli_*` witnesses; NO witness
  input contains an `equivalent_static` block (verified by repo-wide grep
  at preparation), so no witness-facing solve change is expected from this
  tranche;
- GUI-emit boundary (read-only): the DEL-07-02 TP-PMM-GUIEMIT-001 surface
  (`core/model_operations/operation_applier` field path
  `equivalent_static.wind.exposed_pipe_refs`) emits whole-span marking
  only; sub-span GUI/operation emission is a reported follow-on for a
  separate lawful selection, not this tranche;
- governance: `DEC-018`, `DEC-065`, `DEC-025` (single wave-level sweep at
  W2 closeout per the controlling HELP_HUMAN W2 dispatch, mirroring the
  recorded W1 refinement), `DEC-026` (reused analytic tier), `DEC-046`
  (untouched), `DEC-081`, D-52/`DEC-085`, D-54/`DEC-087`, D-12
  (dynamics disposition, untouched), and the R14-W1 tranche briefs as
  current structural models;
- `software-workflow.json` and the root tools
  `tools/software_workflow/run_registered_checks.py` and
  `tools/software_workflow/validate_change_scope.py`.

## 2. Live Selection Facts

- DEL-05-01 is `IN_PROGRESS`; the selected Remaining item is its sole
  Remaining row, an ordinary (ungated) recorded residual of the landed
  TP-PMM-P3-OCCLOADGEN-001 tranche, named by Receipt 6 and the DEC-068
  item-2 boundary record.
- The whole-span limitation is confirmed live: `ElementExposedDiameter`
  carries no extent; `generate_wind_equivalent_static_loads` emits only
  whole-element uniform loads; `WindGenerationInput` has only
  `exposed_pipe_refs`; the canonical `WindEquivalentStaticInput` has only
  whole-element `exposed_element_refs`. No partial-extent wind path
  exists anywhere in the generation or preview-application surfaces.
- The project already owns the sub-span extent vocabulary: canonical
  `$defs/ElementLoadSpan` fractions (DEL-05-05 lineage) and the
  solver-tier `UniformLoadSpan`. Reusing the fraction shape is
  vocabulary reuse, not invention.
- The preview application tier for generated distributed loads is
  statically-equivalent force lumping (50/50 on straight spans; exact
  arc-consistent vectors on macro-realized bends). The
  statically-equivalent end shares of a partial uniform load on a
  straight span are the lever-rule closed forms (§3.3) — elementary
  statics, exactly reducing to 50/50 at full extent.
- Wave-1 landings (producer binding, spring hanger, arc pressure-thrust)
  touched `core/product_physics` in surfaces disjoint from the wind
  generation path (`append_equivalent_static_generated_loads`,
  `add_uniform_element_loads` straight-span branch); the executor
  freeze-check re-verifies the §1 symbol facts at execution.
- Receipt cursor is `Receipt-61`; the branch for this wave is
  `claude/piping-r14-pkg05-loads` at `581a15b1c`.

## 3. Objective and Acceptance Predicates

Implement sub-span wind exposure so that all of the following hold on the
implementation head:

1. **Extent vocabulary exists once, validated.**
   `core/loads/primitive_loads` gains one public extent type (for example
   `LoadExtent { start_fraction, end_fraction }`, Copy, with a validating
   constructor: both fractions finite, `0.0 <= start < end <= 1.0`).
   `ElementExposedDiameter` gains `exposed_extent: Option<LoadExtent>`
   (`None` = whole span). `PrimitiveLoad` and
   `ElementUniformLoadContribution` gain `extent: Option<LoadExtent>`;
   every existing helper constructor sets `None`; a new helper (for
   example `partial_uniform_element_load`) is the only way to construct an
   extent-bearing load. No `LoadTarget` enum variant is added and no
   existing helper signature changes.
2. **Generator emits partial-extent wind loads deterministically.**
   `generate_wind_equivalent_static_loads` accepts exposed entries with
   optional extents: entries with `None` produce byte-identical loads and
   ids to the current behavior; entries with `Some(extent)` produce one
   wind load per entry carrying the extent, with a deterministic load id
   that extends the existing
   `{case_ref}:generated:wind:{axis}:element-{i}` pattern with a stable,
   distinct extent-bearing suffix (deterministic across reruns for
   identical inputs; distinct for distinct extents). Invalid extents
   (non-finite, out of `[0,1]`, `start >= end`) are blocking findings via
   the existing `InvalidGenerationInput` code family; no new finding code
   is introduced (`core/solver/diagnostics` untouched).
3. **Lever-rule static equivalence at the existing preview tier.** For a
   partial uniform intensity `w` (force/length) over fractions `[a, b]`
   of a straight span of length `L`, the applied end forces are the
   statically-equivalent shares of the exposed-segment resultant
   `W = w·(b−a)·L` located at centroid fraction `c = (a+b)/2`:
   `R_i = W·(1−c)` at node i and `R_j = W·c` at node j. At `(a,b) =
   (0,1)` these reduce exactly to the existing `w·L/2` whole-span shares.
   Both extent-aware application seams implement exactly these closed
   forms: `prepare_lumped_nodal_loads` (extent-aware for `Some`,
   byte-identical behavior for `None`) and the product-physics
   `add_uniform_element_loads` straight-span branch. No fixed-end moment
   is introduced anywhere (the preview tier stays statically-equivalent
   force lumping, consistent with every existing generated load).
4. **Curved-bend macro spans fail closed for partial extents.** In
   `add_uniform_element_loads`, an extent-bearing load targeting a
   curved-bend macro-realized span produces a blocking diagnostic (the
   existing `LOAD_INPUT_INVALID` family; message directs whole-span
   marking or separate pipes) — never a silent chord approximation, a
   partial-arc invention, or a drop. Whole-span wind on macro-realized
   spans keeps the existing exact arc-consistent path byte-unchanged.
   Extent-bearing loads never enter `curved_bend_uniform_intensity_by_pipe`
   or any bend recovery/station intensity map.
5. **Preview input surface is additive and fail-closed.**
   `WindGenerationInput` gains a serde-default `exposed_spans` vector
   whose entries carry a pipe reference and user-entered dimensionless
   `start_fraction` / `end_fraction` quantities (DEC-018-normalized like
   `shape_factor`; missing or non-dimensionless entries are blocking).
   Marking rules, all blocking via the existing
   `EQUIVALENT_STATIC_INPUT_MISSING` / `EQUIVALENT_STATIC_INPUT_INVALID`
   codes: at least one marked span overall (`exposed_pipe_refs` or
   `exposed_spans`); unknown pipe references block; the same pipe named
   in both `exposed_pipe_refs` and `exposed_spans` blocks (no double
   exposure); overlapping extents on the same pipe block (multiple
   disjoint extents per pipe are allowed, each generating its own load);
   invalid fractions block. Exposed diameter for a partial extent uses
   the same section basis as whole-span marking (outside diameter plus
   twice insulation thickness when supplied). Seismic generation is
   untouched.
6. **Canonical schema extension reuses the accepted vocabulary.**
   `schemas/model.schema.yaml` `WindEquivalentStaticInput` gains an
   optional `exposed_spans` array (minItems 1) whose items reference an
   element (`$defs/ElementReference`) and carry a `$defs/ElementLoadSpan`
   extent — the existing fraction vocabulary, not a new shape. The
   required set is relaxed to require at least one marking form (via
   `anyOf` over `exposed_element_refs` / `exposed_spans`); every
   currently-valid document remains valid; `additionalProperties: false`
   is preserved; no other `$defs` change. Test coverage (v2):
   `tests/test_physical_to_analytical_transform.py` gains additive
   round-trip coverage only. In `tests/test_model_schema.py`, exactly ONE
   existing assertion — the wind required-set subset assertion (currently
   lines 450–458) — is REPLACED, as a named non-additive edit, by an
   equal-or-stronger assertion set that must verify all of: (i)
   `{"pressure","shape_factor","direction"} <= set(wind["required"])`;
   (ii) the exact `anyOf` shape enforcing at least one of
   `exposed_element_refs` / `exposed_spans`; (iii)
   `exposed_element_refs` retains `minItems == 1` and the new
   `exposed_spans` carries `minItems == 1` and the
   `ElementLoadSpan`-based item shape; and (iv) `additionalProperties`
   remains `False`. Every other assertion in both files is additive-only
   and no other existing assertion is weakened or removed.
7. **Derivation witnessed.** A new hand-calc witness
   `validation/hand_calcs/mechanics/tp_pmm_p3_subspan_wind_exposure.md`
   records: the partial-extent intensity (`w = p·G·D_exposed` over
   `[a,b]`), the exposed-segment resultant and centroid, the lever-rule
   end-share derivation, the exact reduction to the 50/50 whole-span
   shares at `(0,1)`, the multiple-disjoint-extent superposition, and the
   closed-form values used by the benchmark fixture. Elementary statics
   and invented geometry only; no code wind content.
8. **Benchmark evidence.** One new invented fixture in
   `validation/benchmarks/mechanics` (for example
   `MECH-TP-PMM-P3-SUBSPAN-WIND-EXPOSURE`) exercises partial-extent wind
   generation and lever-rule lumping against the hand-calc closed forms
   at the already-recorded DEC-026 analytic-class relative tier —
   regression evidence only; no claim-posture change, no policy JSON, no
   new tolerance constant. The fence permits ONE additive inventory-line
   entry in EACH of the two mirrors
   (`validation/benchmarks/mechanics/README.md`,
   `validation/hand_calcs/mechanics/README.md`) plus the conventional
   fixture-count assertion bump. If a lawfulness obstacle beyond that
   emerges, the fixture is dropped, the obstacle recorded, and
   predicate-7 hand-calc + unit/integration-test evidence stands
   (recorded fallback, not failure).
9. **Existing behavior preserved.** Whole-span wind generation (loads,
   ids, diagnostics), seismic generation, weight/thermal/pressure paths,
   and every solve without an extent-bearing load are unchanged; the
   existing occloadgen fixture and every other benchmark fixture pass
   with recorded values untouched; product-physics integration tests
   demonstrate (a) a partial-extent wind case matches hand-computed
   lever-rule nodal forces to the occloadgen-precedent 1e-9 relative
   comparison, (b) mixed whole-span + partial-extent marking on distinct
   pipes superposes correctly, (c) each §3.5 blocking rule blocks, and
   (d) a whole-span-only wind case solves identically to the pre-tranche
   behavior. The five committed `del1005_payload_binding_*` witnesses
   remain byte-identical at the implementation head (no witness input
   uses `equivalent_static`; any drift is a failure, not a recordable
   consequence). `CliOutput` and every serialized runner surface are
   untouched.
10. **Bounded state update.** On success only: DEL-05-01 `_STATUS.md`
    strikes exactly its sole Remaining item; one new History entry;
    updated `Last Updated`; one new `MEMORY.md` entry recording the
    sub-span design, the curved-bend fail-closed boundary, and the
    GUI-emit follow-on; one new
    `_run_records/WORKING_ITEMS_RUN_2026-07-20_R14_W2_T4_SUBSPAN_WIND.md`.
    The wording records truthfully that GUI/operation emission of
    sub-span marking (DEL-07-02 surface) is reported to HELP_HUMAN as a
    follow-on, not performed here.
11. **Checks.** The full §6 validation plan passes.

A successful run closes only the named Remaining item. It does not touch
DEL-05-02 (D-45 gated), DEL-05-04's conditional row, any DEL-07-02 or
operation-applier surface, dynamics (D-12), or any threshold/tolerance
record.

## 4. Selected Design (D-54 Reasoned Selection) and Bounded Tasks

Where several shapes were defensible, the selection below was made under
D-54/`DEC-087`; the four-lens analysis and materially rejected
alternatives are recorded in
`execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260719-MECHANICS-CAMPAIGN-R14/instances/W2/T4/CURRENT_CANDIDATE_RATIONALE.md`.
That fact alone is not a referral condition.

Selected shape:

- **Optional extent field, not a new load target class.** One validated
  `LoadExtent` fraction type threaded as `Option` through
  `ElementExposedDiameter`, `PrimitiveLoad`, and
  `ElementUniformLoadContribution` (`None` = whole span everywhere), so
  every existing constructor, consumer, and match stays valid and
  whole-span behavior is structurally byte-identical.
- **Fraction vocabulary reused from the accepted canonical shape**
  (`$defs/ElementLoadSpan` / `FractionQuantity`, DEL-05-05 lineage), on
  both the canonical schema and the preview input, with DEC-018
  dimensionless normalization mirroring `shape_factor`.
- **Lever-rule static equivalence at the existing preview tier** —
  the exact statically-equivalent end shares of the partial resultant —
  rather than fixed-end (work-equivalent) vectors, keeping one uniform
  rigor tier for all generated distributed loads on straight spans.
- **Fail-closed on curved-bend macro spans** for partial extents
  (blocking diagnostic; whole-span arc-consistent path untouched); a
  partial-arc consistent integration is a possible future refinement,
  deliberately not invented here.
- **Evidence = hand-calc witness + additive benchmark fixture (+ two
  mirror inventory lines) + unit/integration tests**, all invented
  content under the existing claim posture.

Bounded tasks for the executor child:

### 4.1 Freeze the execution basis

- Begin on the wave branch `claude/piping-r14-pkg05-loads` in the
  integration checkout; record the base commit before any durable write;
  verify the tree is clean apart from this run's lawful pre-existing
  state (the R14 W2 instance directory, the T5 verification record, and
  this brief).
- Re-verify the §1 symbol-level facts (generator shape, application
  seams, schema shapes, absence of `equivalent_static` in witness
  inputs). Stop if the DAG pointer, the deliverable's lifecycle/Remaining
  text, the named sources, or `software-workflow.json` changed
  materially. Do not silently reinterpret scope.

### 4.2 Derive, then implement

- Write the §3.7 hand-calc derivation FIRST. If any closed form cannot
  be completed from elementary statics, STOP and return the tranche as
  blocked — do not implement an unwitnessed treatment.
- Implement §3.1–§3.5 in `core/loads/primitive_loads/src/lib.rs` and
  `core/product_physics/src/lib.rs` (and `validation.rs` only if an
  additive edit proves strictly required — expected: not required, since
  provenance and load-presence checks are block-level).
- Apply the §3.6 schema extension and additive test coverage.
- Unit tests (primitive_loads): extent validation, deterministic
  partial-load ids, whole-span invariance, lever-rule shares in
  `prepare_lumped_nodal_loads` including the `(0,1)` reduction.
  Integration tests (product_physics): the §3.9 set.

### 4.3 Evidence

- Add the §3.8 benchmark fixture (or record the fallback); run the
  suite; verify both README mirrors carry exactly one additive line.
- Verify del1005 five-case byte-identity at the implementation head.

### 4.4 Update deliverable state and close out

On success only: apply §3.10, then run §6 in order. On failure or block:
leave deliverable state unchanged (or record the truthful partial state),
write truthful evidence and `EXECUTE_RETURN.md` under `instances/W2/T4/`,
and return to the W2 manager. The executor does not commit; the W2
manager commits after independent implementation verification.

## 5. Exact Write Fence for the Later Execution

While the adoption effect is held: no execution writes are authorized.

After the adoption chain becomes effective, durable writes are limited to
(paths relative to `WORKING_ROOT` unless noted):

1. this candidate brief, only for the governed status record or a later
   superseding hold/rejection record;
2. `core/loads/primitive_loads/src/lib.rs` (and its `Cargo.toml`/
   `Cargo.lock` only if strictly required);
3. `core/product_physics/src/lib.rs` and
   `core/product_physics/src/validation.rs` (additive-only, and
   `validation.rs` only under the §4.2 strict-necessity condition), plus
   `Cargo.toml`/`Cargo.lock` only if strictly required;
4. `schemas/model.schema.yaml` (the §3.6 additive extension only);
5. `tests/test_model_schema.py` and
   `tests/test_physical_to_analytical_transform.py` (additive only,
   EXCEPT the single named wind required-set assertion replacement in
   `tests/test_model_schema.py` authorized by §3.6 (v2), which must be
   equal-or-stronger; never weaken any other existing check);
6. NEW file
   `validation/hand_calcs/mechanics/tp_pmm_p3_subspan_wind_exposure.md`;
7. `validation/benchmarks/mechanics/src/lib.rs` (additive fixture +
   registration + conventional fixture-count assertion bump, plus the
   minimal literal-construction updates required by the §3.1 additive
   fields), `validation/benchmarks/mechanics/README.md` and
   `validation/hand_calcs/mechanics/README.md` (ONE additive
   inventory-line entry each; no claim-posture, tolerance, `TBD`, or
   note-text change), and the suite `Cargo.lock` if required;
8. DEL-05-01 deliverable folder
   (`execution/PKG-05_Loads, Load Cases, and Stress Recovery/1_Working/DEL-05-01_Primitive load case engine/`):
   `_STATUS.md`, `MEMORY.md`, one new
   `_run_records/WORKING_ITEMS_RUN_2026-07-20_R14_W2_T4_SUBSPAN_WIND.md`;
9. the tranche instance directory
   `execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260719-MECHANICS-CAMPAIGN-R14/instances/W2/T4/**`;
10. no evidence-sweep artifact in this tranche (single wave-level DEC-025
    sweep at W2 closeout per the controlling HELP_HUMAN W2 dispatch,
    mirroring the recorded W1 refinement).

Ephemeral writes: task-local Cargo target dirs and scratch captures
outside durable project paths.

No other project file is writable. In particular: no
`core/solver/**` (including `diagnostics`, `straight_pipe`,
`curved_bend`), `core/loads/user_loads/**`, `core/runner/**`,
`core/model_operations/**`, `core/reporting/**`, `core/gui/**`, Python
core, apps, or any other crate; no schema other than
`schemas/model.schema.yaml`; no `validation/witness/**`, reproduction
bundle, policy JSON, or existing hand-calc/benchmark recorded value; no
`docs/**` (a validation-manual consequence, if discovered, is reported
and returned, not fixed here); no other deliverable or package folder,
register, DAG, decomposition, decision packet, PRD/PLAN, workplan, or
receipt; no root governance, `_DomainEngines/**`, app-dev, PEC, or
external path. No push, pull, fetch, PR, or merge.

## 6. Evidence and Validation Plan

In sequence from `WORKING_ROOT` unless noted; every failure stops
subsequent state-changing closeout; all cargo offline
(`CARGO_NET_OFFLINE=true`, `--offline`); no check may install, fetch, or
update anything (a missing local prerequisite is a truthful blocked
result):

- `cargo fmt --manifest-path core/loads/primitive_loads/Cargo.toml --check`
  and `cargo test --offline --manifest-path core/loads/primitive_loads/Cargo.toml`;
- `cargo fmt --manifest-path core/product_physics/Cargo.toml --check`
  and `cargo test --offline --manifest-path core/product_physics/Cargo.toml`;
- `cargo fmt --manifest-path validation/benchmarks/mechanics/Cargo.toml --check`
  and `cargo test --offline --manifest-path validation/benchmarks/mechanics/Cargo.toml`
  (fmt/test still run even on the §3.8 fallback path, because the §3.1
  field additions touch the suite's fixture constructors);
- when the fixture path is taken: verify both inventory mirrors list the
  new fixture and each README diff is exactly one additive line;
- `cargo test --offline --manifest-path core/solver/diagnostics/Cargo.toml`
  (read-only regression: finding-code mapping must still be exhaustive
  with no new code introduced);
- `cargo test --offline --manifest-path core/runner/headless/Cargo.toml`
  (read-only downstream regression over the changed solve inputs);
- `python3 -m pytest -q tests/test_model_schema.py tests/test_physical_to_analytical_transform.py`;
- `python3 tests/test_headless_runner_contract.py` (read-only contract
  guard; `CliOutput` is untouched);
- del1005 five-case byte-identity (runner stdout at the implementation
  head vs the five committed generated witnesses);
- `python3 tools/validation/validate_claims_language.py --repo-root .`
  and `python3 tools/validation/validate_path_anchors.py . --text` from
  `REPO_ROOT`;
- `git diff --check` from `REPO_ROOT`; JSON parsing for every new/changed
  `.json` file;
- changed-path containment:
  `python3 tools/software_workflow/validate_change_scope.py "$REPO_ROOT"
  --base <base-commit> --allowed <each §5 path>` from `REPO_ROOT`,
  persisting JSON stdout to
  `execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260719-MECHANICS-CAMPAIGN-R14/instances/W2/T4/CHANGE_SCOPE_CONTAINMENT.json`.

The branch-level registered checks (`piping-pytest`, `evidence-sweep`,
`harness-pytest`, `harness-self-check`) run once at W2 wave closeout,
before any push or fan-in, per the controlling HELP_HUMAN W2 dispatch
(the single evidence sweep must yield exactly one new
`validation/evidence/sweeps/SWEEP_*.json`, committed as the DEC-025
pre-push gate for the whole code-touching branch — the same wave-level
refinement recorded in the W1 T1 brief §6 and W1 return).

## 7. Defect and Failure Disposition

- Fail closed. Any §3 predicate failure, §6 check failure, del1005 byte
  drift, unexpected changed path, recorded-value change in any existing
  fixture, or an incomplete derivation stops closeout; record truthful
  evidence under `instances/W2/T4/` and return to the W2 manager. An
  unwitnessed or silently-approximated treatment is never landed.
- No scope drift: no code wind profile/exposure-category/coefficient
  content, no dynamics (D-12), no fixed-end/work-equivalent tier change,
  no partial-arc bend integration, no GUI/operation-applier emission, no
  threshold/tolerance creation or promotion, no
  lifecycle/stage/release/acceptance act, no D-45 or DEL-05-04 work.
- A repair need outside the §5 fence is reported and returned for a new
  lawful selection, not fixed here.
- Dirty checkout beyond lawful R14 state: stop and return the condition.

## 8. Rerun Triggers

A rerun (new execution record, same governed brief unless superseded) is
required when any of these changes after the implementation base commit:
the generation/application surfaces (`core/loads/primitive_loads`,
`core/product_physics`, `schemas/model.schema.yaml`); the DEL-05-01
Remaining scope or lifecycle state; applicable DAG-007 rows or the DAG
pointer; `software-workflow.json`; or a prior failed/blocked result after
its condition resolves. A material governance change (a new decision
touching the wind-generation surface; supersession of the D-52/D-54
lanes) returns the brief itself to HELP_HUMAN before any rerun.

## 9. Exclusions and Preserved Gates

This brief does not authorize:

- any DEL-05-02 work (D-45 `AWAITING_RULING`; no temperature-indexed G
  slot, interpolation, or synthesized value), any DEL-05-04 work (the
  conditional PDU-037 row's condition is separately verified NOT MET in
  `instances/W2/T5/CONDITION_VERIFICATION.md`), or any owner-gated row
  anywhere;
- code-content wind/seismic coefficients, exposure categories, gust or
  importance factors, catalogs, or defaults of any kind — all values
  remain user-entered;
- dynamics content (response spectra, time histories, modal analysis) —
  disposition stays with D-12;
- GUI, operation-applier, or editor emission of sub-span marking
  (DEL-07-02 surface; reported follow-on);
- promotion of release thresholds, final tolerance policy, CI gate
  policy, or any DEC-046 record; new tolerance constants, acceptance
  criteria, or normative content (the DEC-026 analytic tier is reused as
  recorded, never redefined);
- edits to `core/solver/**`, frozen witness surfaces, reproduction
  bundles, policy JSON, existing recorded comparison values, or docs;
- lifecycle transition, stage/milestone advancement, issuance, release,
  packaging, publication, push, PR creation/merge, hosted CI, network
  use, or any external commitment;
- professional approval, certification, sealing, authentication, or
  code-compliance claims.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).

## 10. Owner Adoption by Standing Approval — Attribution and Effect Record

```text
OwnerStandingApproval: DEC-085 / D-52 §2, as prospectively refined by DEC-087 / D-54 §1
AgentClassification: CLASSIFY_ELIGIBLE (W2 manager, R14 campaign)
RuleActivation: ACTIVATE_OWNER_STANDING_APPROVAL
ClassifiedBy: WORKING_ITEMS (Agent 1), HELP-HUMAN-PIPING-20260719-MECHANICS-CAMPAIGN-R14 / W2 / T4
AgentJudgment: SELECT_AND_ADVANCE (D-54 §3.3; selected shape per §4)
SelectedOutcome: optional validated LoadExtent threaded through the wind generation and preview lumping seams with lever-rule static equivalence on straight spans, fail-closed curved-bend partial extents, canonical ElementLoadSpan vocabulary reuse, hand-calc witness, and additive benchmark fixture per §3–§4 within the §5 fence
JudgedBy: WORKING_ITEMS (Agent 1), HELP-HUMAN-PIPING-20260719-MECHANICS-CAMPAIGN-R14 / W2 / T4
AdoptionAuthority: HUMAN_OWNER_BY_STANDING_APPROVAL (DEC-085 / D-52 §2, durably SHA-bound at governance commit f14fa77518a06f112ae72a8fcce4de0fab958d47)
OwnerCaseSelection: NONE
RejectedAlternatives: recorded in the rationale artifact (intensity scaling without centroid placement; direct nodal-force emission from the generator; work-equivalent fixed-end tier upgrade; partial-arc bend integration; element subdivision/meshing; new LoadTarget variant; deferral)
RationaleArtifact: execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260719-MECHANICS-CAMPAIGN-R14/instances/W2/T4/CURRENT_CANDIDATE_RATIONALE.md
IndependentVerifier: COMMIT-SAFE — `instances/W2/T4/VERIFY_BRIEF_V2.md` (16/16 claims confirmed; independent ten-class re-screen pass; lever-rule statics independently re-derived; v1 cure verified as jointly satisfiable and equal-or-stronger); history: v1 BLOCK at `instances/W2/T4/VERIFY_BRIEF.md` (test-edit constraint unsatisfiable with the live wind required-set assertion; cured by the v2 amendment), preserved unsoftened
EffectStatus: EFFECTIVE (v2) — EXECUTION RELEASED BY W2 MANAGER UNDER THE R14 CAMPAIGN CHAIN (V2 COMMIT-SAFE)
PreservedGates: D-45 (DEL-05-02); DEL-05-04 conditional row (condition NOT MET, verified); D-12 dynamics; DEC-046 tolerance promotion; GUI-emit follow-on (DEL-07-02); lifecycle/stage/issuance/release/acceptance; reproduction acceptance and evidence-posture promotion; prover activation/correlation; publication/external action; merge authority; D-05b; F-PIP-1..5
```

Adoption is the owner's conditional act under the standing rule; the
agent classifies, selects among defensible shapes under D-54, and
proposes. The W2 manager progresses `EffectStatus` only after the
independent refutation returns `COMMIT-SAFE`, under the R14 campaign
plan's execution rules and the durably landed D-52/D-54 records. No
execution is released by this document in its current state.
