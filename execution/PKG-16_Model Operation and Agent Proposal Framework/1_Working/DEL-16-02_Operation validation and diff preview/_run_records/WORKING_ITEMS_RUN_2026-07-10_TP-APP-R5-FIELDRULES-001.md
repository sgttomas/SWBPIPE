# WORKING_ITEMS RUN - TP-APP-R5-FIELDRULES-001

Date: 2026-07-10
Agent: WORKING_ITEMS (bounded implementation tranche)
Package: PKG-16 - Model Operation and Agent Proposal Framework
Primary deliverable: DEL-16-02 - Operation validation and diff preview
Supporting deliverables: DEL-03-06 (expansion-joint component model entry
seam), DEL-05-01 (load case engine slots), DEL-02-02 (unit contract reuse)
Tranche: `TP-APP-R5-FIELDRULES-001`
Target stage: R5 (ordinary in-stage work under DEC-054)

Tranche-id basis: the mandate named `TP-OPAPPLIER-FIELDRULES-001`; this crate's
existing applier tranches use the `TP-APP-*` series
(`TP-APP-R2-COMBEXPR-001`, `TP-APP-R2-WASMPKG-001`), so the id was adjusted to
`TP-APP-R5-FIELDRULES-001` to stay in that series at the current target stage.
Deliverable basis: DEL-16-02 owns the `core/model_operations/operation_applier`
seam (crate doc header "DEL-16-02 / DEL-16-03 runtime seam"; the seam-corpus
and combination-expression applier tranches are recorded under this
deliverable's `_run_records/`).

## Scope

The `operation_applier` field-rule registry rejected every component-geometry
`set_field` path with blocking `OP-FIELD-PATH-UNSUPPORTED`, including
long-standing GUI inspector fields (`geometry.bend_radius.value`, the
DEC-045/EJSTIFF expansion-joint fields) and the newly schema'd user-entered
slots from DEC-068 (`geometry.bend_pipe_ref`, `section.mill_tolerance`,
`modulus_basis_ref`, seismic/wind `equivalent_static` generation inputs).
Values entered in the GUI emitted as intents but never applied to the session
model. This tranche extends the registry so accepted intents actually apply.

Gap verified live before building: a probe test applying
`geometry.bend_radius.value` on a bend component failed with
`OP-FIELD-PATH-UNSUPPORTED` ("Field path `geometry.bend_radius.value` on
`Component` is not an editable field in the apply-operation seam.") at the
tranche base head `f5a4c3946645ff3446b99fd243419233156eea08`.

## Implemented

New field-rule kinds in `core/model_operations/operation_applier/src/lib.rs`,
all for optional user-entered schema slots (authorable when absent; absence is
staleness-checked as the inspector's explicit `TBD` sentinel; missing
intermediate objects are created on apply, and an existing non-object
intermediate is a blocking finding, never silently overwritten):

- `OptionalText` - free-text slot; non-empty replacement required.
- `OptionalId` - schema `Id` pattern `^[A-Za-z][A-Za-z0-9_.:-]*$`
  (new blocking code `OP-ID-PATTERN-INVALID`).
- `OptionalEntityRef` - reference into another model collection
  (`OP-REFERENCE-NOT-FOUND`; referenced entities never created implicitly).
- `OptionalEnum` - closed vocabulary (new blocking code
  `OP-ENUM-TOKEN-INVALID`).
- `OptionalEntityRefList` - comma-separated reference list stored as a string
  array (schema `minItems: 1`; empty list is `OP-VALUE-EMPTY`).
- `OptionalQuantity` - `{value, unit}` dimensioned slot pinned to one schema
  dimension. Dimension-kind check against the intent dimension
  (`OP-UNIT-DIMENSION-MISMATCH`); DEC-018 unit-symbol/dimension check
  (`OP-UNIT-MISMATCH-CONVERSION-UNAVAILABLE`); dimensionless slots also accept
  the preview model's stored `none` marker. Present slots edit in place with
  the existing B2 sibling-unit semantics (entered unit preserved; a
  plain-number edit may not silently change the stored unit); absent slots are
  authored from the explicit user entry as a whole `{value, unit}` record - no
  default and no hidden fallback unit. Value constraints mirror the solve-side
  validation for the same slot (`core/product_physics`,
  `core/loads/primitive_loads`): `Positive`, `NonNegative` (new blocking code
  `OP-VALUE-NEGATIVE`), or `AnyFinite`.

Registry additions (paths now accepted):

- `Component` (all four GUI component families):
  - bend/elbow: `geometry.bend_pipe_ref` (pipe ref), `geometry.bend_radius.value`
    (length, positive), `geometry.bend_angle.value` (angle, positive),
    `geometry.bend_plane_orientation` (text).
  - branch: `geometry.branch_header_pipe_ref`, `geometry.branch_branch_pipe_ref`
    (pipe refs), `geometry.branch_run_size.value`,
    `geometry.branch_header_size.value` (length, positive),
    `geometry.branch_connection_angle.value` (angle, positive),
    `geometry.branch_connection_type` (text).
  - rigid/semi-rigid: `geometry.rigid_pipe_ref` (pipe ref),
    `geometry.rigid_body_length.value`, `geometry.end_a_size.value`,
    `geometry.end_b_size.value` (length, positive), `geometry.weight.value`
    (force, finite), `geometry.stiffness_behavior_reference` (text).
  - expansion joint (EJSTIFF family): `geometry.expansion_joint_pipe_ref`
    (pipe ref), `geometry.effective_area.value` (area, positive),
    `geometry.movement_limit.value` (length, positive),
    `geometry.hardware_reference`, `geometry.manufacturer_reference`,
    `geometry.pressure_thrust_reference` (text).
  - user modifier values (user-entered only; no code table, catalog value, or
    default): `modifiers.sif_user_value.value`,
    `modifiers.branch_header_sif_user_value.value`,
    `modifiers.branch_branch_sif_user_value.value`,
    `modifiers.flexibility_factor_user_value.value`,
    `modifiers.stiffness_scaling_user_value.value` (dimensionless, positive);
    `modifiers.linear_stiffness_user_value.value`,
    `modifiers.axial_stiffness_user_value.value`,
    `modifiers.lateral_stiffness_user_value.value` (linear_stiffness,
    positive); `modifiers.rotational_stiffness_user_value.value`,
    `modifiers.angular_stiffness_user_value.value`,
    `modifiers.torsional_stiffness_user_value.value` (rotational_stiffness,
    positive).
- `Element`: `section.mill_tolerance.value` (length, non-negative - explicit
  zero accepted, negative reduction blocked, mirroring the effective-wall
  consumer; DEC-068 item 3).
- `Load`: `modulus_basis_ref` (schema Id shape; exact selection against stored
  temperature points remains the solve-side contract - DEC-068 item 1; nothing
  interpolation-related, D-38 stays AWAITING_RULING);
  `equivalent_static.seismic.gravity_acceleration.value` (acceleration,
  positive), `equivalent_static.seismic.g_factor_x|y|z.value` (dimensionless,
  finite - sign meaningful), `equivalent_static.wind.pressure.value`
  (pressure, finite), `equivalent_static.wind.shape_factor.value`
  (dimensionless, finite), `equivalent_static.wind.direction`
  (closed set `global_x|global_y|global_z`),
  `equivalent_static.wind.exposed_pipe_refs` (pipe reference list)
  (DEC-068 item 2).

Paths deliberately left unsupported (existing `OP-FIELD-PATH-UNSUPPORTED`
diagnostic; recorded as residual, not guessed):

- `Component.geometry.center_of_gravity` - the inspector displays a free-text
  vector rendering ("x=..., y=..., z=... unit"); applying it needs a governed
  vector payload format (design ruling), so the intent remains an explicit
  blocked finding. A new unit test pins this behavior.
- `Component.kind` and `Combination.terms` remain deferred per the existing
  `DEFERRED_FIELDS` scope limits (unchanged).

## Files Touched

- `core/model_operations/operation_applier/src/lib.rs` (registry, resolution,
  path-creating write seam, 11 new unit tests incl. the probe)
- `core/model_operations/operation_applier/tests/contract_corpus.rs`
  (coverage floor extended with the four new blocking codes)
- `fixtures/model_operations/contract_corpus/case_66..case_75` (10 new
  invented cases: 6 accepted applies, 4 blocks; blessed from the Rust contract
  reference; README review_status row marks them pending their own human
  review entry per the corpus convention)
- `fixtures/model_operations/contract_corpus/README.md`

## Validation

- Probe test failed at base head with `OP-FIELD-PATH-UNSUPPORTED` (gap
  verified live), passes after the registry extension.
- `cargo test` in `core/model_operations/operation_applier` - 72/72 lib tests
  (61 pre-existing + 11 new), canonical-hash parity 1/1, contract corpus 2/2
  (75 cases incl. the 10 new; coverage floor now pins
  `OP-REFERENCE-NOT-FOUND`, `OP-VALUE-NEGATIVE`, `OP-ID-PATTERN-INVALID`,
  `OP-ENUM-TOKEN-INVALID`).
- Corpus bless produced zero churn on the 65 pre-existing case files.
- `cargo test` in `core/product_physics` - 71/71.
- `npm ci` in `apps/desktop`, `npm run build:wasm:desktop`, then
  `npm run test:desktop` - 19/19 files, 427/427 tests (includes the
  cross-engine corpus mirror executing all 75 cases through the wasm engine
  and the routing adapter).
- Repo-root `PYTHONDONTWRITEBYTECODE=1 python3
  tools/practitioner_harness/harness.py self-check` - exit 0.
- Repo-root `python3 -m pytest tools/practitioner_harness -q` - 263 passed,
  1 skipped.
- DEC-025 five-surface evidence sweep at the committed clean head: see the
  sweep addendum at the end of this record (artifact committed separately so
  the sweep binds to the code commit).

## Boundary

- User-entered values only: no default, catalog value, or code-table value is
  supplied anywhere; value constraints only mirror the named solve-side
  validation for the same slot; no unit conversion beyond what the crate
  already performs (entered units are preserved; no new conversion path); no
  threshold semantics introduced.
- Nothing interpolation-related (D-38 remains AWAITING_RULING); exact-selection
  checking of `modulus_basis_ref` stays a solve-side contract.
- No lifecycle state, review disposition, DAG pointer, release-readiness,
  professional approval, certification, sealing, authentication, or
  code-compliance claim changed. All new corpus data is invented
  (PUBLIC_DOMAIN_OR_ORIGINAL).

## Residual

- `Component.geometry.center_of_gravity` application semantics (vector payload
  format) need a design ruling before the path can be supported.
- Corpus cases 66-75 are blessed from the Rust contract reference and await
  their own human review entry per the corpus README convention (prior
  acceptances are DEC-030 / DEC-032; new cases do not ride them).
- The GUI does not yet emit intents for the four DEC-068 slot families
  (`geometry.bend_pipe_ref`, `section.mill_tolerance.value`,
  `modulus_basis_ref`, `equivalent_static.*`); the apply seam now accepts
  them, so the inspector/manager emitters are follow-up GUI work.

## Sweep addendum

DEC-025 five-surface evidence sweep executed from the project root at the
committed clean head `9fc322fcad8d0ce13c93f82319d72bab0c08647d`
(`python3 tools/release/run_evidence_sweep.py --execute`): overall **pass**,
all five surfaces pass (cargo_crate_sweep, python_pytest, desktop_vitest,
desktop_playwright_e2e, desktop_production_build). Artifact:
`validation/evidence/sweeps/SWEEP_20260711T024706Z_9fc322fcad8d.json`
(committed in the follow-up sweep-summary commit so the artifact binds to the
code commit). No Playwright port collision occurred; no retry chain.
