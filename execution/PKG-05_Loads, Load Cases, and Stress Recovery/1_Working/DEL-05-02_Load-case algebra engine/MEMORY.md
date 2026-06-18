# MEMORY - DEL-05-02 Load-case algebra engine

## 2026-05-01 Implementation

Implemented the bounded load-case algebra deliverable within the sealed write
scope.

Changed artifacts:

- `core/loads/load_case_algebra/.gitignore`
- `core/loads/load_case_algebra/Cargo.toml`
- `core/loads/load_case_algebra/README.md`
- `core/loads/load_case_algebra/src/lib.rs`
- `docs/SPEC.md`
- `docs/TYPES.md`
- `execution/PKG-05_Loads, Load Cases, and Stress Recovery/1_Working/DEL-05-02_Load-case algebra engine/MEMORY.md`
- `execution/_Coordination/DEV-001_DISPATCH_DEL-05-02.md`
- `execution/_Coordination/NEXT_INSTANCE_STATE.md`

Implementation notes:

- Added the `open_pipe_stress_load_case_algebra` Rust crate.
- Implemented deterministic user-defined linear combinations, result-state
  subtraction, and min/max range envelopes over compatible mechanics
  quantities.
- Reused the primitive-load dimension vocabulary to preserve unit/dimension
  intent without introducing conversion constants or a unit catalog.
- Preserved analysis-status boundaries and rejected external human-approval
  status as an automatic algebra output.
- Reported missing operands, duplicate operands, non-finite factors,
  incompatible dimensions, missing result states, empty expressions, and
  status-boundary violations as deterministic findings.
- Did not implement code-specific public load combinations, a general
  expression parser, rule-pack evaluator reuse, stress recovery, protected
  standards content, or professional/code-compliance claims.

Verification:

- `cargo fmt --manifest-path core/loads/load_case_algebra/Cargo.toml --check`
- `cargo test --manifest-path core/loads/load_case_algebra/Cargo.toml`
- `cargo test --manifest-path core/loads/primitive_loads/Cargo.toml`
- `git diff --check`

Open items:

- Canonical calculation unit basis and conversion constants remain `TBD`.
- Final result-envelope and persistence integration remain `TBD`.
- General expression grammar/library and rule-pack evaluator reuse remain
  `TBD`.
- Concrete application-service API and production tolerance policy remain
  `TBD`.

## 2026-05-11 TP-RECON-01 Reconciliation

- Historical implementation evidence reconciled from archived DEV-001 rows:
  `DEL-05-02` / `PKG-05` is recorded as committed in commit `0f9189c`
  (`core: add load case algebra engine`) and remains `CHECKING`.
- Load-case algebra evidence: `core/loads/load_case_algebra` implements
  explicit user-defined linear combinations, result-state subtraction, and
  range envelopes over compatible mechanics quantities; it records deterministic
  findings for missing operands, duplicate operands, non-finite factors,
  dimension mismatches, missing result states, empty expressions, and automatic
  status-boundary violations.
- Explicit mechanics combination evidence: TP-MAC-08 uses the algebra crate in
  `core/product_physics` for invented preview `basis: "mechanics"`
  combinations only. Combined scalar displacement, reaction, force, moment, and
  stress component rows carry `basis_ref`, `source_result_refs`, and metadata
  basis `explicit_user_linear_combination`; open-formula stress summary rows
  are skipped with `COMBINATION_STRESS_SUMMARY_SKIPPED`.
- Result schema corroboration: `tests/test_results_schema.py` asserts
  `explicit_user_linear_combination` is an allowed result metadata basis and
  verifies the invented combination row
  `result:combination:combination-C-OPER-ALT:force:pipe-P-120:axial` with
  combination `basis_ref` and source result refs.
- Verification recorded for this reconciliation: `cargo test --manifest-path
  core/loads/load_case_algebra/Cargo.toml`; `cargo test --manifest-path
  core/product_physics/Cargo.toml
  valid_invented_model_exposes_explicit_load_combination_results`; `cargo test
  --manifest-path core/product_physics/Cargo.toml
  combination_stress_summary_rows_are_skipped_with_diagnostics`; `cargo test
  --manifest-path core/product_physics/Cargo.toml
  invalid_combination_records_block_with_diagnostics`; `python3
  tests/test_results_schema.py`.
- Deferred boundaries preserved: no public code-specific load combinations or
  factors, protected standards content, rule-pack checks, allowables,
  SIF/flexibility tables, owner design-basis data, or professional reliance
  claims. TP-MAC-08 product preview integration still
  defers code/rule combinations, expression language, pressure-to-frame
  conversion, final CLI syntax, desktop save/open UX, durable storage workflow,
  external execution, and summary-maxima changes.

## 2026-05-16 - DEV-001 downstream PKG-02 audit memory addendum

Durable context preserved after reconciliation review:
- DEV-001 package-worker audit reviewed this deliverable for downstream compatibility with the accepted PKG-02 foundation contracts.
- Local audit artifacts are `execution/PKG-05_Loads, Load Cases, and Stress Recovery/1_Working/DEL-05-02_Load-case algebra engine/_REVIEW.md` and `execution/PKG-05_Loads, Load Cases, and Stress Recovery/1_Working/DEL-05-02_Load-case algebra engine/Review_Findings.csv`.
- Package audit summary is `execution/PKG-05_Loads, Load Cases, and Stress Recovery/1_Working/_audit/PKG02_DOWNSTREAM_REVIEW_2026-05-16.md`; package run record is `execution/PKG-05_Loads, Load Cases, and Stress Recovery/1_Working/_run_records/TASK_RUN_2026-05-16_PKG02_DOWNSTREAM_AUDIT.md`.
- This was audit evidence only. It did not change lifecycle state, authorize release, or make a professional, certification, sealing, approval, or code-compliance claim.
- The May 16 package-worker TASK run record did not fully preserve canonical per-deliverable TASK documentation context; this addendum preserves the durable deliverable-local pointer without modifying the completed run record.

## 2026-05-16 - DEV-001 PKG-02 grounded finding-resolution memory addendum

Durable context preserved after PKG-02 grounded finding resolution:
- Stage 2 technical resolution used the accepted PKG-02 contract as the governing source for this deliverable's downstream compatibility evidence.
- Original audit finding count for this deliverable: 2 (WARNING=2). Current technical status count in the resolution matrix: TECHNICALLY_ADDRESSED_PENDING_HUMAN=2.
- Resolution evidence is indexed in `execution/_Reconciliation/Reviews/DEV001_FINDING_RESOLUTION_PKG02_GROUNDED_2026-05-16/RESOLUTION_MATRIX.csv`; validation evidence is summarized in `execution/_Reconciliation/Reviews/DEV001_FINDING_RESOLUTION_PKG02_GROUNDED_2026-05-16/VALIDATION_SUMMARY.md`.
- Local `Review_Findings.csv` entries remain subject to the human disposition gate. `HumanDisposition` stays `TBD` until review, and `Status` must not be changed to `RESOLVED` automatically.
- No lifecycle promotion, release claim, or professional/code-compliance claim is implied by the technical closeout.

## 2026-06-04 - TP-AUTHORITY-REFRESH-0_7-DAG006

- WORKING_ITEMS/TASK workforce current-authority refresh applied to active deliverable-local surfaces for `DEL-05-02`.
- Current authority basis is `execution/_Decomposition/SOFTWARE_DECOMP.md` revision `0.7` plus approved `execution/_DAG/DAG-006/` active graph authority.
- Historical run records, historical DAG row IDs, review dispositions, lifecycle `_STATUS.md`, aggregate DAG artifacts, candidate edges, repo-level governance files, schemas, code, and tools were intentionally not changed by this refresh.
- Preserved historical references remain evidence of earlier work, not current authority claims.

## 2026-06-04 - TASK documentation alignment to implementation evidence

- WORKING_ITEMS dispatched a bounded deliverable-local TASK to align DEL-05-02 documents with implemented load-case algebra evidence.
- Updated `Datasheet.md`, `Specification.md`, `Guidance.md`, and `Procedure.md` to replace setup-only/future wording with evidence from `core/loads/load_case_algebra/README.md` and `core/loads/load_case_algebra/src/lib.rs`.
- Documented the implemented evidence boundary: explicit unit-aware linear combinations, result-state subtraction, range envelopes, deterministic findings, result-boundary metadata, and no bundled code-specific defaults or rule-pack evaluator.
- Preserved unresolved boundaries: general expression grammar/library, final rule-pack evaluator/interface behavior, final result-envelope/persistence integration, dependency maturity for DEL-05-01 and DEL-05-04, low-confidence DEL-06-02 evaluator interface, and release/CI gate policy remain `TBD` or pending as applicable.
- Validation attempted: `cargo test --manifest-path core/loads/load_case_algebra/Cargo.toml` failed before running tests because `require_result_schema_binding` in the read-only crate source does not cover the current `CanonicalSchemaBinding::ModelLoadCase` enum variant from `core/loads/primitive_loads/src/lib.rs`.
- This TASK did not edit `_STATUS.md`, dependency/review artifacts, schemas, repo-level governance files, DAG/coordination files, or core code.

## 2026-06-04 - TASK load-case algebra boundary hardening

- WORKING_ITEMS dispatched a bounded deliverable-local TASK with edits
  authorized for `core/loads/load_case_algebra/**`, this `MEMORY.md`, and the
  deliverable `_run_records/` surface only.
- Hardened `core/loads/load_case_algebra/src/lib.rs` so algebra evaluation
  records duplicate input operands, preserves missing-result-state findings
  even when subtraction operands are also duplicate, blocks missing analysis
  status metadata, blocks `ModelIncomplete` operands from producing algebra
  output, preserves deterministic analysis-status ordering, validates result
  boundary record/unit/provenance metadata even for blocked results, and sorts
  range-envelope source operands with deterministic tie-breaking by operand ID.
- Added focused crate tests for incompatible/missing status metadata,
  duplicate-and-missing subtraction boundaries, blocked-result metadata
  validation, unit/dimension metadata mismatch, and deterministic range
  envelope tie ordering. The crate now reports 17 passing unit tests.
- Verification passed:
  `cargo fmt --manifest-path core/loads/load_case_algebra/Cargo.toml --check`;
  `cargo test --manifest-path core/loads/load_case_algebra/Cargo.toml`;
  `git diff --check`.
- Preserved boundaries: no public code-specific load combinations or factors,
  protected standards content, rule-pack evaluator behavior, stress recovery,
  allowables, SIF/flexibility tables, lifecycle/status changes, dependency or
  review-disposition edits, release claims, or professional/code-compliance
  claims were introduced.
- Remaining TBDs are unchanged: general expression grammar/library, final
  rule-pack evaluator/interface behavior, final result-envelope/persistence
  integration, DEL-05-01 and DEL-05-04 dependency maturity, low-confidence
  DEL-06-02 interface, and project CI/release-gate policy.

## 2026-06-04 - TP-PHYS-024 parent fan-in

- WORKING_ITEMS fan-in reviewed the completed parallel TASK slice for
  `DEL-05-02` together with sibling slices for `DEL-04-04`, `DEL-04-05`, and
  `DEL-05-05`.
- Aggregate validation passed after fan-in:
  `cargo test --manifest-path core/solver/nonlinear_supports/Cargo.toml` (14
  tests), `cargo test --manifest-path core/solver/performance_harness/Cargo.toml`
  (8 tests), `cargo test --manifest-path core/loads/load_case_algebra/Cargo.toml`
  (17 tests), `cargo test --manifest-path core/loads/user_loads/Cargo.toml` (28
  tests), each corresponding `cargo fmt --check`, and `git diff --check`.
- Fan-in found no cross-worker scope drift for this deliverable. Lifecycle, DAG,
  dependency, review-disposition, release, professional-approval, and
  code-compliance surfaces remain unchanged.

## 2026-06-05 - Review readiness preparation

- TASK prepared lifecycle-review readiness evidence in `_run_records/TASK_RUN_2026-06-05_2052_REVIEW_READINESS_PREP.md`.
- Readiness classification: `REVIEW_PREPARED_WITH_BLOCKERS`.
- Targeted validation passed in this run: `cargo test --manifest-path core/loads/load_case_algebra/Cargo.toml` with 17 unit tests and 0 doctests.
- Remaining gates: DEL-05-01 dependency row still pending while upstream is `CHECKING`; DEL-05-04 remains `IN_PROGRESS`; low-confidence DEL-06-02 evaluator interface remains `TBD`/`IN_PROGRESS`; two PKG-02 compatibility WARNING findings remain technically addressed pending human disposition.
- This was readiness preparation only. It did not edit `_STATUS.md`, `Dependencies.csv`, `Review_Findings.csv`, lifecycle state, release surfaces, or professional/code-compliance claims.

## 2026-06-05 - Blocker closure and lifecycle-readiness review

- Human ruling packet: `execution/_Reconciliation/Reviews/PKG05_BLOCKER_CLOSURE_RULING_PACKET_2026-06-05_2120.md`.
- Review snapshot: `execution/_Reconciliation/Reviews/REV_DEL-05-02_2026-06-05_2120/`.
- Recommendation: `RECOMMEND_ADVANCE_TO_CHECKING` for `IN_PROGRESS -> CHECKING`.
- Lifecycle action: none; `_STATUS.md` remains `IN_PROGRESS` pending later Gate 5 approval.
- Finding disposition: Findings `DEL-05-02-PKG02-W001` and `DEL-05-02-PKG02-W002` were accepted as technically resolved by human ruling and set to `HumanDisposition=ACCEPT_AS_IS`, `Status=RESOLVED`.
- Dependency update: Rows `DAG-002-E0451` and `DAG-002-E0453` were updated to `SATISFIED`; low-confidence future evaluator-interface row `DAG-002-E0616` was set to `NOT_APPLICABLE` for this review cycle by approved human ruling while preserving the future interface TBD in notes.
- Validation: Locked load-case algebra crate tests passed with 17 unit tests and 0 doctests.
- Residual boundaries: General expression grammar/library, final evaluator reuse, final result-envelope/persistence integration, and release/CI policy remain explicit TBDs outside this lifecycle recommendation.
- No release, professional approval, certification, sealing, authentication, code-compliance claim, protected standards data, or private data was introduced.

## 2026-06-05 - Gate 5 CHECKING approval applied

- Explicit Gate 5 approval changed `_STATUS.md` from `IN_PROGRESS` to
  `CHECKING` after blocker closure and review snapshot
  `execution/_Reconciliation/Reviews/REV_DEL-05-02_2026-06-05_2120/`.
- No additional dependency register, review finding, source code, schema,
  aggregate DAG artifact, release record, professional approval,
  certification, sealing, authentication, or code-compliance claim was changed.

## 2026-06-11 - TP-APP-R2-LOADMGR-001 load-case primitive magnitude manager

- WORKING_ITEMS app-integration tranche surfaced existing combination terms in
  the desktop Load Cases manager. No `core/loads/load_case_algebra` source
  behavior changed.
- The manager displays `combination:C-OPER-ALT` with mechanics basis and terms
  `load:L-100 x 1` and `load:L-200 x 0.5`, preserving the existing explicit
  user-defined mechanics-combination/no-code-default boundary.
- The editable part of this tranche is limited to primitive-load magnitude
  updates through structured operations; combination basis/terms editing
  remains residual A4 scope.
- Evidence is recorded in
  `_run_records/WORKING_ITEMS_RUN_2026-06-11_load_case_manager_primitive_magnitude.md`
  and `apps/desktop/SMOKE.md` TP-MAC-91. Validation passed:
  `npm test --workspace apps/desktop` (32/32), `npm run build --workspace
  apps/desktop`, `npm run test:e2e:desktop` (1/1), and `git diff --check -- .
  ':!init/init-prompt.md'`.
- No lifecycle state, dependency/review disposition, code-specific
  combination/default, rule-pack evaluator behavior, release claim,
  professional approval, certification, sealing, authentication,
  code-compliance claim, protected standards data, or private data changed.

## 2026-06-11 - TP-APP-R2-COMBCREATE-001 combination creation editor

- WORKING_ITEMS app-integration tranche added explicit mechanics-basis
  combination creation in the desktop Load Cases manager. No
  `core/loads/load_case_algebra` source behavior changed.
- The manager queues structured `create_combination` intents for a new
  `Combination` with `field_path=combinations`, `before=not_present`, unit
  `none`, dimension `dimensionless`, and one explicit existing load-case term.
- The tested preview path creates `combination:C-300` with `basis=mechanics`
  and initial term `load:L-100 x 1`. Subtraction/range expression authoring,
  code/rule combinations, and broader algebra schema changes remain outside
  this tranche.
- Evidence is recorded in
  `_run_records/WORKING_ITEMS_RUN_2026-06-11_combination_creation_editor.md`
  and `apps/desktop/SMOKE.md` TP-MAC-106. Validation passed:
  operation-applier format check, Rust operation-applier tests 34/34,
  src-tauri Rust tests 26/26, desktop Vitest 58/58, desktop build, desktop
  Playwright smoke 1/1, and targeted browser smoke.
- No lifecycle state, dependency/review disposition, code-specific
  combination/default, rule-pack evaluator behavior, release claim,
  professional approval, certification, sealing, authentication,
  code-compliance claim, protected standards data, or private data changed.

## 2026-06-11 - TP-APP-R2-COMBTERMDELETE-001 combination term deletion editor

- WORKING_ITEMS app-integration tranche added explicit indexed deletion for
  existing combination terms in the desktop Load Cases manager. No
  `core/loads/load_case_algebra` source behavior changed.
- The manager queues structured `delete_combination_term` intents for
  `Combination.terms.N` with a current before-value, `after=not_present`, unit
  `none`, dimension `dimensionless`, local-session audit metadata, and no
  professional approval claim.
- The tested preview edit removes `combination:C-OPER-ALT` term 1
  (`load:L-200 x 0.5`) while preserving `load:L-100 x 1`. Whole-term
  replacement, code/rule combinations, and broader algebra authoring remain
  outside this tranche.
- Evidence is recorded in
  `_run_records/WORKING_ITEMS_RUN_2026-06-11_combination_term_deletion_editor.md`
  and `apps/desktop/SMOKE.md` TP-MAC-105. Validation passed:
  operation-applier format check, Rust operation-applier tests 33/33,
  src-tauri Rust tests 26/26, desktop Vitest 56/56, desktop build, desktop
  Playwright smoke 1/1, and in-app browser smoke over
  `combination:C-OPER-ALT`.
- No lifecycle state, dependency/review disposition, code-specific
  combination/default, rule-pack evaluator behavior, release claim,
  professional approval, certification, sealing, authentication,
  code-compliance claim, protected standards data, or private data changed.

## 2026-06-11 - TP-APP-R2-COMBTERMCREATE-001 combination term creation editor

- WORKING_ITEMS app-integration tranche added explicit child-term creation
  for existing combinations in the desktop Load Cases manager. No
  `core/loads/load_case_algebra` source behavior changed.
- The manager queues structured `create_combination_term` intents for
  `Combination.terms` with `before=not_present`, unit `none`, dimension
  `dimensionless`, and JSON payload `{ load_case, factor }`.
- The tested preview path creates `load:L-300` and appends it to
  `combination:C-OPER-ALT` as a new term, preserving existing
  `load:L-100 x 1` and `load:L-200 x 0.5` terms. Whole-term replacement,
  deletion, code/rule combinations, and broader algebra authoring remain
  outside this tranche.
- Evidence is recorded in
  `_run_records/WORKING_ITEMS_RUN_2026-06-11_combination_term_creation_editor.md`
  and `apps/desktop/SMOKE.md` TP-MAC-104. Validation passed:
  operation-applier format check, Rust operation-applier tests 32/32,
  src-tauri Rust tests 26/26, desktop Vitest 54/54, desktop build, desktop
  Playwright smoke 1/1, and in-app browser smoke over
  `combination:C-OPER-ALT`.
- No lifecycle state, dependency/review disposition, code-specific
  combination/default, rule-pack evaluator behavior, release claim,
  professional approval, certification, sealing, authentication,
  code-compliance claim, protected standards data, or private data changed.

## 2026-06-11 - TP-APP-R2-COMBBASIS-001 combination basis editor

- WORKING_ITEMS app-integration tranche added editing for existing
  `Combination.basis` text values in the desktop Load Cases manager. No
  `core/loads/load_case_algebra` source behavior changed.
- The manager queues explicit structured `update_load` intents for
  `Combination.basis` with unit `none`, dimension `dimensionless`,
  local-session audit metadata, and no professional approval claim.
- The tested preview edit updates `combination:C-OPER-ALT` from
  `basis=mechanics` to `basis=mechanics_user_review`, preserving existing
  `terms` and `provenance`. Whole `terms` replacement, term
  creation/deletion, code/rule combinations, and broader algebra authoring
  remain outside this tranche.
- Evidence is recorded in
  `_run_records/WORKING_ITEMS_RUN_2026-06-11_combination_basis_editor.md` and
  `apps/desktop/SMOKE.md` TP-MAC-103. Validation passed:
  operation-applier format check, Rust operation-applier tests 31/31,
  src-tauri Rust tests 26/26, desktop Vitest 52/52, desktop build, desktop
  Playwright smoke 1/1, and in-app browser smoke over
  `combination:C-OPER-ALT`.
- No lifecycle state, dependency/review disposition, code-specific
  combination/default, rule-pack evaluator behavior, release claim,
  professional approval, certification, sealing, authentication,
  code-compliance claim, protected standards data, or private data changed.

## 2026-06-11 - TP-APP-R2-COMBFACTOR-001 combination term-factor editor

- WORKING_ITEMS app-integration tranche added editing for existing
  combination term factors in the desktop Load Cases manager. No
  `core/loads/load_case_algebra` source behavior changed.
- The manager queues explicit structured `update_load` intents for
  `Combination.terms.N.factor` with unit `none`, dimension `dimensionless`,
  local-session audit metadata, and no professional approval claim.
- The tested preview edit updates `combination:C-OPER-ALT` term 1 from
  `load:L-200 x 0.5` to `load:L-200 x 0.75`. Whole `terms` replacement,
  `basis` editing, term creation/deletion, code/rule combinations, and broader
  algebra authoring remain outside this tranche.
- Evidence is recorded in
  `_run_records/WORKING_ITEMS_RUN_2026-06-11_combination_term_factor_editor.md`
  and `apps/desktop/SMOKE.md` TP-MAC-96. Validation passed:
  operation-applier format check, Rust operation-applier tests 24/24,
  src-tauri Rust tests 26/26, desktop Vitest 37/37, desktop build, desktop
  Playwright smoke 1/1, and in-app browser smoke over
  `combination:C-OPER-ALT`.
- No lifecycle state, dependency/review disposition, code-specific
  combination/default, rule-pack evaluator behavior, release claim,
  professional approval, certification, sealing, authentication,
  code-compliance claim, protected standards data, or private data changed.

## 2026-06-12 - TP-APP-R2-COMBEXPR-001 subtraction/range expression authoring

- TASK app-integration tranche (completion-plan A4 residual) landed
  authoring, seam validation, and solve evaluation for
  `result_state_subtraction` and `range_envelope` load combinations. The
  combination basis is now a closed set mirroring the
  `core/loads/load_case_algebra` vocabulary (`mechanics`,
  `result_state_subtraction` with `minuend_id`/`subtrahend_id`,
  `range_envelope` with `operand_ids` + `mode` in
  `min`/`max`/`min_abs`/`max_abs`). New record fields are strictly additive
  and optional; the model-document schema version stays 0.1.0 and the
  regenerated mechanics preview fixture is byte-identical.
- `core/loads/load_case_algebra` gained only additive `RangeMode`
  token helpers (`token`/`parse_token`); no existing operation semantics
  changed. `core/product_physics` evaluates the two new bases over solved
  result rows by reusing `evaluate_result_state_subtraction` and
  `evaluate_range_envelope`, with per-basis pre-solve structural validation
  (new named blocking codes `LOAD_COMBINATION_SHAPE_INVALID`,
  `LOAD_COMBINATION_RANGE_MODE_UNKNOWN`,
  `LOAD_COMBINATION_OPERANDS_EMPTY`) and per-basis result-metadata basis
  values added additively to `schemas/results.schema.yaml`.
- `core/model_operations/operation_applier` (sole engine, native + wasm per
  DEC-020) validates per-basis create payload shapes with named blocking
  diagnostics and now validates `Combination.basis` updates against the
  same closed set. Behavior change recorded: basis edits were free text
  since TP-APP-R2-COMBBASIS-001; unsupported tokens now block
  (`OP-COMBINATION-BASIS-UNSUPPORTED`) and cross-shape changes block
  (`OP-COMBINATION-BASIS-SHAPE-MISMATCH`); the first Playwright test's
  `mechanics_user_review` fill was replaced by a closed-set selection.
  Load-case deletion counts subtraction/range operand references.
- Desktop Load Cases manager create form gained the closed-set basis
  selector with conditional per-basis fields; the basis editor became a
  closed-set selector; combination rows display per-basis expressions.
  Existing intent/preview/queue/apply pattern and testids unchanged.
- Contract corpus cases 58–65 added and blessed via the documented
  workflow; README inventory extended with an explicit note that cases 58+
  were added by TP-APP-R2-COMBEXPR-001 and are pending human review — they
  do not ride the DEC-030 acceptance of the original 57 cases.
  `schemas/model.schema.yaml` Combination gained the two basis tokens,
  optional `minuend_ref`/`subtrahend_ref`/`operand_refs`/`mode`, and
  per-basis conditional requires per the existing if/then convention.
- Evidence is recorded in `_run_records/TASK_RUN_2026-06-12_1138.md` and
  `apps/desktop/SMOKE.md` TP-MAC-143. Validation passed: operation-applier
  cargo tests 61/61, product_physics cargo tests 31/31, load_case_algebra
  cargo tests 18/18, desktop Vitest 239/239, desktop build, Playwright e2e
  2/2 (authors a subtraction combination through visible controls with the
  wasm engine applying in real Chromium), src-tauri cargo tests 32/32
  (untouched), repo Python suite 358/358, `cargo fmt --check` clean on the
  three touched crates.
- Residuals: corpus cases 58–65 pending human review; browser-mode solve of
  edited models still requires the backend
  (`BROWSER_SOLVE_BACKEND_REQUIRED_FOR_EDITED_MODEL`), so new-basis e2e
  evidence stops at authoring/apply; cross-shape basis conversion is
  intentionally delete-and-recreate. One write-boundary note for human
  ruling: the per-basis solve validation required editing
  `core/product_physics/src/validation.rs`, which the tranche whitelist did
  not list explicitly (it listed `core/product_physics/src/lib.rs`); the
  deviation is disclosed in the run record.
- No lifecycle state, dependency/review disposition, code-specific
  combination/default, rule-pack evaluator behavior, release claim,
  professional approval, certification, sealing, authentication,
  code-compliance claim, protected standards data, or private data changed.

## 2026-06-17 - Lifecycle Housekeeping

- Housekeeping lifecycle reset: `_STATUS.md` current state set to `IN_PROGRESS` to reflect current code development in progress. This does not change review, issuance, release readiness, professional approval, certification, sealing, authentication, or code-compliance status.

## 2026-06-17 - TP-UNITS-BTAIL-LOADMANAGERDIMLESSUNITVALID-001 dimensionless load-manager operation evidence

- Primary role for a bounded Phase B-tail Load Cases manager unit-validation
  slice while C5.7 remains human-execution gated.
- Empty load-case shell creation, load-case metadata edits, whole load-case
  deletion, combination creation, combination basis/factor edits, combination
  term create/delete, and whole-combination deletion now record
  `unit_validation=not_required_dimensionless` in the existing operation
  intent validation field.
- These operations already declare `unit=none` and `dimension=dimensionless`;
  the tranche makes that non-unit-bearing status explicit in previews instead
  of leaving `unit_validation=not_run`.
- Evidence:
  `_run_records/WORKING_ITEMS_RUN_2026-06-17_TP-UNITS-BTAIL-LOADMANAGERDIMLESSUNITVALID-001.md`;
  `apps/desktop/SMOKE.md` TP-MAC-217; completion log entry.
- Validation passed: focused manager App Vitest 18/18, full desktop Vitest
  399/399, focused R2/R3 Playwright 14/14, and desktop production build with
  the existing Vite large-chunk warning.
- Boundary preserved: no load-case algebra solver behavior, operation
  application behavior, accepted model-state mutation, durable persistence,
  unit conversion API, protected standards content, private data, lifecycle
  transition, release-readiness claim, professional approval, certification,
  sealing, authentication, or code-compliance claim changed.
## 2026-06-18 - TP-UNITS-BTAIL-LOADMANAGERLINTUNITS-001 supporting report-lint inventory evidence

- Supporting role for DEL-05-02: Report Content Lint now inventories the
  existing Load Case Manager load-case and combination unit-validation surface
  through `load-manager-unit-validation-surface`.
- The lint packet now includes
  `apps/desktop/src/features/load-cases/LoadCaseManagerPanel.tsx`; public
  unit-policy targets increase from 38 to 39 while conversion-witness targets
  remain two.
- Evidence:
  `_run_records/WORKING_ITEMS_RUN_2026-06-18_TP-UNITS-BTAIL-LOADMANAGERLINTUNITS-001.md`;
  `apps/desktop/SMOKE.md` TP-MAC-256; completion log entry; primary
  DEL-08-05 run record and supporting DEL-07-02/DEL-05-01/DEL-16-02/DEL-02-02
  run records.
- Validation passed: focused App Vitest workspace-render; focused load/unit
  App tests 26/26; focused R2 Playwright smoke 2/2; full desktop Vitest
  399/399; single-worker R2/R3 Playwright smoke 18/18; `git diff --check`;
  and desktop production build with the existing Vite large-chunk warning.
- Boundary preserved: no load-case algebra behavior, primitive-load behavior,
  operation validation/application, unit conversion behavior, private data,
  lifecycle transition, release-readiness claim, professional approval,
  certification, sealing, authentication, or code-compliance claim changed.
