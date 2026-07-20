# WORKING_ITEMS Run Record - R14-W2-T4 Sub-Span Wind Exposure

Date: 2026-07-20
Agent: T4 executor (governed Agent 2, serialized, non-delegating; W2 PKG-05
manager chain, campaign `HELP-HUMAN-PIPING-20260719-MECHANICS-CAMPAIGN-R14`)
Deliverable: DEL-05-01 - Primitive load case engine
Package: PKG-05 - Loads, Load Cases, and Stress Recovery
Tranche: R14 W2 T4 (brief `CB-2026-07-20-T4-PKG05-SUBSPAN-WIND-001`,
status EFFECTIVE (v2), verifier v2 COMMIT-SAFE; v1 BLOCK preserved)
Base commit: `581a15b1c718fd444870f13e75fc7cd974518670` on
`claude/piping-r14-pkg05-loads`

## Scope

Close the sole DEL-05-01 `_STATUS.md` Remaining item: extend wind marking
to sub-span (partial-extent) exposure in occasional-load generation
(Receipt 6 residual / TP-PMM-P3-OCCLOADGEN-001 §Boundaries / DEC-068
item 2). A user may mark a fraction range `[a, b]` of a straight pipe
span as wind-exposed; the generated load applies the exact lever-rule
statically-equivalent end shares of the partial uniform intensity at the
existing preview force-lumping tier. Whole-span behavior is
byte/behavior-identical; curved-bend macro-realized spans fail closed for
partial extents; all inputs are user-entered with no code wind content.

## Files Touched

- `core/loads/primitive_loads/src/lib.rs` (new validated `LoadExtent`
  fraction type + `PrimitiveLoadError::InvalidExtentBounds`;
  `ElementExposedDiameter` gains `exposed_extent: Option<LoadExtent>`;
  `PrimitiveLoad` and `ElementUniformLoadContribution` gain
  `extent: Option<LoadExtent>` with every existing helper setting `None`
  and new helper `partial_uniform_element_load` as the only
  extent-bearing constructor; `generate_wind_equivalent_static_loads`
  validates extents (blocking `InvalidGenerationInput`) and emits
  deterministic extent-suffixed ids
  `{case_ref}:generated:wind:{axis}:element-{i}:extent-{start}-{end}`;
  `prepare_lumped_nodal_loads` applies lever-rule shares for `Some`
  extents and the byte-identical 50/50 half-total for `None`; 4 new
  tests)
- `core/product_physics/src/lib.rs` (`WindGenerationInput` gains
  serde-default `exposed_spans: Vec<WindExposedSpanInput>` with
  DEC-018 dimensionless normalization of `start_fraction`/`end_fraction`
  mirroring `shape_factor`; `append_equivalent_static_generated_loads`
  enforces the marking rules — at least one marking form, per-span
  required fields, unknown refs, no double exposure, no overlapping
  extents per pipe, invalid fractions — all via the existing
  `EQUIVALENT_STATIC_INPUT_MISSING`/`EQUIVALENT_STATIC_INPUT_INVALID`
  codes, with the same exposed-diameter section basis as whole-span
  marking; `add_uniform_element_loads` straight-span branch applies the
  lever rule for `Some` extents and blocks extent-bearing loads on
  curved-bend macro-realized spans with `LOAD_INPUT_INVALID`;
  `curved_bend_uniform_intensity_by_pipe` never admits extent-bearing
  loads; 8 new tests. `validation.rs` unchanged — its checks are
  block-level, as the brief expected)
- `schemas/model.schema.yaml` (`WindEquivalentStaticInput`: required set
  relaxed to the three parameter keys plus an `anyOf` requiring at least
  one of `exposed_element_refs`/`exposed_spans`; additive optional
  `exposed_spans` array (minItems 1) whose inline items reuse
  `$defs/ElementReference` + `$defs/ElementLoadSpan`;
  `additionalProperties: false` preserved; no other `$defs` change)
- `tests/test_model_schema.py` (the single §3.6 (v2) authorized
  replacement of the wind required-set subset assertion by the
  equal-or-stronger (i)–(iv) set: three-key subset, exact `anyOf` shape,
  both `minItems` plus the `ElementLoadSpan`-based item shape, and
  `additionalProperties` False; every other assertion untouched)
- `tests/test_physical_to_analytical_transform.py` (two additive tests:
  spans-only round-trip through the transform with schema validation,
  and marking-rule rejection cases)
- `validation/hand_calcs/mechanics/tp_pmm_p3_subspan_wind_exposure.md`
  (NEW witness, written before any source edit)
- `validation/benchmarks/mechanics/src/lib.rs` (additive fixture
  `MECH-TP-PMM-P3-SUBSPAN-WIND-EXPOSURE` tied to the witness closed
  forms, inventory registration, count assertion 23 -> 24, two new
  tests, and the two forced `ElementExposedDiameter` literal updates)
- `validation/benchmarks/mechanics/README.md` and
  `validation/hand_calcs/mechanics/README.md` (ONE additive inventory
  line each; no claim-posture, tolerance, `TBD`, or note-text change)
- DEL-05-01 `_STATUS.md` (sole Remaining item removed per the W1
  strike precedent; one new History entry; Last Updated 2026-07-20),
  `MEMORY.md` (one new entry), and this run record
- `execution/_Coordination/AgentRuns/…/instances/W2/T4/`
  (`EXECUTE_RETURN.md`, `CHANGE_SCOPE_CONTAINMENT.json`)

## Recorded Design Choices

- **Optional extent field, not a new load target class** (per the
  adopted D-54 selection): `Option<LoadExtent>` threading keeps every
  existing constructor, consumer, and match valid; `None` = whole span
  everywhere, so whole-span behavior is structurally identical.
- **Lever rule at the existing statically-equivalent tier**: end shares
  `R_i = W(1-c)`, `R_j = W c` of the exposed-segment resultant
  `W = w L (b-a)` at centroid fraction `c = (a+b)/2`; exact elementary
  statics, reducing exactly (including in floating point) to the
  existing `w L / 2` shares at `(0, 1)`. No fixed-end moment introduced
  anywhere.
- **Deterministic extent id suffix** uses Rust's shortest-round-trip
  `f64` Display (`:extent-0.2-0.7`, `:extent-0.8-1`): stable across
  reruns for identical inputs and distinct for distinct extents.
- **Overlap rule**: extents on one pipe block when their interiors
  intersect (`start_next < end_prev` after sorting); extents sharing
  only an endpoint are disjoint and allowed. Recorded as the
  implementation reading of "multiple disjoint extents per pipe are
  allowed".
- **Missing-marking message** now names both marking forms
  ("exposed_pipe_refs or exposed_spans") in the one case where no span
  is marked at all; code, severity, and diagnostic id are unchanged and
  the pre-existing assertion (`contains("exposed_pipe_refs")`) still
  holds. Recorded truthfully as the single diagnostic-text consequence
  of the additive input surface.
- **Fail-closed curved bends**: partial extents on macro-realized spans
  block with `LOAD_INPUT_INVALID` (message directs whole-span marking or
  separate pipes); extent loads never enter bend intensity maps. The
  whole-span arc-consistent path is untouched.
- **Canonical vocabulary reuse**: schema `exposed_spans` items reference
  `$defs/ElementReference` and carry a `$defs/ElementLoadSpan` extent —
  inline item shape, no new `$defs`. Preview-input field names
  (`pipe_ref`, `start_fraction`, `end_fraction`) mirror the existing
  preview `exposed_pipe_refs`/fraction conventions.

## Implemented Evidence

- Hand-calc witness (written first): partial-extent intensity,
  resultant/centroid, lever-rule derivation with uniqueness, exact
  `(0, 1)` reduction, disjoint-extent superposition, curved-bend
  fail-closed boundary, and the benchmark closed forms (w = 84 N/m;
  extent A [0.2, 0.7] -> 69.3/56.7 N; extent B [0.8, 1.0] ->
  5.04/45.36 N; superposed 74.34/102.06 N; whole-span reduction 126 N).
- primitive_loads unit tests: extent-constructor validation,
  deterministic distinct partial ids with rerun identity, whole-span
  invariance, generator blocking on invalid extents, lever-rule shares
  in `prepare_lumped_nodal_loads` including the exact `(0, 1)` equality
  with the whole-span output.
- product_physics integration tests: partial-extent case matches
  hand-computed lever-rule nodal forces at the occloadgen-precedent
  1e-9 relative comparison; mixed whole-span + partial marking on
  distinct pipes superposes; every §3.5 blocking rule blocks (missing
  span fields, unknown ref, double marking, overlap — with the disjoint
  companion case solving, invalid fractions); partial extent on a
  curved-bend macro-realized span blocks with `LOAD_INPUT_INVALID`. The
  pre-existing whole-span equivalence test
  (`wind_equivalent_static_generation_matches_authored_distributed_load`)
  passes unchanged as the whole-span-only preservation evidence.
- Benchmark fixture `MECH-TP-PMM-P3-SUBSPAN-WIND-EXPOSURE` reproduces
  the witness lever-rule shares, superposition, and whole-span reduction
  at the 1.0e-9 internal epsilon (recorded DEC-026 analytic-class
  relative tier reused; `tolerance_policy: None` on every expected
  value; both README mirrors carry exactly one additive line).

## Checks

- `cargo fmt --check` + `cargo test --offline`
  `core/loads/primitive_loads`: 49 passed (45 + 4 new).
- `cargo fmt --check` + `cargo test --offline` `core/product_physics`:
  94 passed (86 + 8 new).
- `cargo fmt --check` + `cargo test --offline`
  `validation/benchmarks/mechanics`: 38 passed (36 + 2 new).
- Read-only regressions: `core/solver/diagnostics` 24 passed (no new
  finding code; mapping still exhaustive); `core/runner/headless`
  23 + 1 + 15 passed.
- `python3 -m pytest -q tests/test_model_schema.py
  tests/test_physical_to_analytical_transform.py`: 22 passed (19 + 3
  new test functions across the two files).
- `python3 tests/test_headless_runner_contract.py`: PASS.
- del1005 five-case byte-identity at the implementation head: exits
  0/0/0/1/1, all five stdouts byte-identical (`cmp`) to the committed
  `validation/witness/generated/del1005_payload_binding_*.json`.
- `validate_claims_language.py`, `validate_path_anchors.py`,
  `git diff --check`, JSON parse of new/changed `.json`, and
  changed-path containment recorded in the tranche `EXECUTE_RETURN.md`.

## Boundaries And Residuals

- GUI/operation-applier emission of sub-span marking (DEL-07-02
  TP-PMM-GUIEMIT-001 surface) is reported to HELP_HUMAN as a follow-on
  for a separate lawful selection, not performed here.
- No dynamics content (disposition stays with D-12); no code-content
  wind coefficients, exposure categories, or catalogs — all values
  user-entered.
- No fixed-end/work-equivalent tier change and no partial-arc bend
  integration (both remain possible future lawful selections);
  curved-bend partial extents fail closed.
- No threshold/tolerance creation or promotion (DEC-026 tier reused as
  recorded; DEC-046 untouched); no DEL-05-02 (D-45 gated) or DEL-05-04
  work; no witness, solver-crate, runner, or docs edit.
- No lifecycle transition, release-readiness, professional,
  certification, sealing, authentication, or code-compliance claim.
- No commit, push, PR, merge, or network action by this executor; the
  W2 manager commits after independent implementation verification.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
