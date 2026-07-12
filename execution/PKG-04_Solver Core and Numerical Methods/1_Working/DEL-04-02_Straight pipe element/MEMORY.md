# MEMORY - DEL-04-02 Straight Pipe Element

## 2026-07-12 - D-41 R5 T4 PDU-040 supporting evidence

- The downstream persisted-record report reader preserves governed context but does not bind straight-pipe diagnostics/results into the analysis-run producer.
- Focused evidence is recorded in `_run_records/WORKING_ITEMS_RUN_2026-07-12_D41-R5-T4-PDU012-PDU021-PDU022-PDU040.md`; lifecycle remains `IN_PROGRESS`, with no review, validation, approval, compliance, release, or professional-reliance conclusion.

## Decisions And Rulings

- 2026-05-01 - Human project authority authorized proceeding with
  `DEL-04-02` after the `DEL-04-05` evidence refresh.
- 2026-05-01 - This slice consumes committed `DEL-04-01` frame-kernel evidence,
  committed `DEL-03-08` section-property calculator evidence, committed
  `DEL-02-02` unit-contract evidence, and the newly committed `DEL-04-05`
  performance-harness evidence as regression context.

## Implementation Summary

- Added `core/solver/straight_pipe`, a Rust mechanics crate with a path
  dependency on `core/solver/frame_kernel`.
- Added explicit straight-pipe section-property integration into the frame
  element boundary.
- Added weight-hook records for explicit mass-per-length and gravity inputs.
- Added local force recovery from element or global-model displacement vectors.

## Evidence

- `cargo fmt --manifest-path core/solver/straight_pipe/Cargo.toml --check`
  passed.
- `cargo test --manifest-path core/solver/straight_pipe/Cargo.toml` passed:
  6 tests, 0 failures.
- Upstream/regression Rust checks passed:
  `cargo test --manifest-path core/solver/frame_kernel/Cargo.toml` and
  `cargo test --manifest-path core/solver/performance_harness/Cargo.toml`.

## Open TBDs

- Canonical calculation unit basis and conversion constants remain `TBD`.
- Primitive load-case application of weight remains future `PKG-05` work.
- Downstream stress recovery remains future `DEL-05-03` work.
- Result-envelope/application-service integration remains future work.

## Boundaries Preserved

- No protected standards text, protected tables, code-specific allowables,
  SIF/flexibility factors, proprietary data, private project data, or
  professional/code-compliance claims were introduced.
- No lifecycle state, dependency register, candidate edge, or blocker queue was
  changed by the product implementation.

## 2026-05-11 TP-RECON-01 Reconciliation

- Evidence sources checked: `plans/TP-RECON-01_DISPATCH_MATRIX.csv` row for
  `DEL-04-02`; `AGENTS.md`, `docs/CONTRACT.md`,
  `docs/IP_AND_DATA_BOUNDARY.md`; archived DEL-04-02 dispatch/evidence/status
  and lifecycle records; TP-MAC-01 tranche/TP-MAC-01-E brief; related TP-MAC
  plans 02, 04, 06, 07, 09, and 10; commit `b0516e5`; current DEL files;
  `core/solver/straight_pipe`, `core/product_physics`, and
  `tests/product_preview/test_product_preview_service.py`.
- Straight-pipe evidence remains aligned with commit `b0516e5`:
  `core/solver/straight_pipe` adapts explicit section properties to the frame
  kernel, exposes explicit mass-per-length/gravity weight hooks, and recovers
  local forces from element or global-model displacement vectors.
- Product-preview corroboration: `core/product_physics` consumes
  `StraightPipeElement` for invented-data linear-static preview mechanics,
  end-i/end-j local result rows, thermal and pressure axial corrections, fixed
  station-grid interpolation, open-mechanics stress rows, and explicit
  mechanics-basis load combinations; product-preview tests assert those IDs,
  metadata, pressure-longitudinal suppression for thrust-active cases, separate
  mechanics/rule/professional statuses, immutable run/report context, and no
  compliance wording.
- Verification recorded: original DEL evidence records straight-pipe
  `cargo fmt`/`cargo test` plus frame/performance regression checks; related
  TP-MAC plans record 2026-05-10 product-physics cargo tests,
  product-preview pytest, generated fixture, desktop build/test/smoke, and
  `git diff --check` coverage as applicable. TP-RECON-01 did not rerun tests.
- Deferred boundaries remain governed outside this reconciliation: canonical
  calculation unit basis/conversion constants, primitive weight application,
  arbitrary station input, exact internal diagrams, station envelopes, shear
  stress, equivalent/principal stress, broader thermal/pressure behavior,
  code/rule checks, allowables, SIF/flexibility behavior, protected/private
  data, release/professional claims, and lifecycle/dependency/DAG changes.

## 2026-05-12 TP-PHYS-001 Verification Hardening

- Executed one approved `TP-PHYS-001` TASK slice for `DEL-04-02` / `PKG-04`
  with write scope limited to `core/solver/straight_pipe/**`, this
  `MEMORY.md`, and deliverable-local `_run_records/**`.
- Hardened `core/solver/straight_pipe/src/lib.rs` by validating the derived
  `weight_force_per_length` output from explicit mass-per-length and gravity
  inputs before returning a `WeightHook`.
- Expanded straight-pipe tests from 6 to 12 cases, adding coverage for
  non-finite/overflowing weight-hook values, non-finite mass-per-length,
  rotated local/global axial force recovery, noncontiguous global-model DOF
  extraction, non-finite recovery displacements, and required global-model
  displacement length diagnostics.
- Verification passed:
  `cargo fmt --manifest-path core/solver/straight_pipe/Cargo.toml --check`;
  `cargo test --manifest-path core/solver/straight_pipe/Cargo.toml` with 12
  tests passed; `cargo test --manifest-path core/solver/frame_kernel/Cargo.toml`
  with 19 tests passed; `git diff --check` passed.
- Preserved open TBDs: sparse solver library, tolerance policy, canonical unit
  basis/conversion constants, primitive weight application, downstream stress
  recovery, result-envelope integration, release thresholds, and professional
  reliance.
- No protected standards text, protected tables, allowables, SIF/flexibility
  data, private data, code-compliance claim, lifecycle edit, dependency edit,
  coordination edit, DAG edit, or frame-kernel edit was introduced.

## 2026-05-16 - DEV-001 downstream PKG-02 audit memory addendum

Durable context preserved after reconciliation review:
- DEV-001 package-worker audit reviewed this deliverable for downstream compatibility with the accepted PKG-02 foundation contracts.
- Local audit artifacts are `execution/PKG-04_Solver Core and Numerical Methods/1_Working/DEL-04-02_Straight pipe element/_REVIEW.md` and `execution/PKG-04_Solver Core and Numerical Methods/1_Working/DEL-04-02_Straight pipe element/Review_Findings.csv`.
- Package audit summary is `execution/PKG-04_Solver Core and Numerical Methods/1_Working/_audit/PKG02_DOWNSTREAM_REVIEW_2026-05-16.md`; package run record is `execution/PKG-04_Solver Core and Numerical Methods/1_Working/_run_records/TASK_RUN_2026-05-16_PKG04_PKG02_DOWNSTREAM_AUDIT.md`.
- This was audit evidence only. It did not change lifecycle state, authorize release, or make a professional, certification, sealing, approval, or code-compliance claim.
- The May 16 package-worker TASK run record did not fully preserve canonical per-deliverable TASK documentation context; this addendum preserves the durable deliverable-local pointer without modifying the completed run record.

## 2026-05-16 - DEV-001 PKG-02 grounded finding-resolution memory addendum

Durable context preserved after PKG-02 grounded finding resolution:
- Stage 2 technical resolution used the accepted PKG-02 contract as the governing source for this deliverable's downstream compatibility evidence.
- Original audit finding count for this deliverable: 2 (WARNING=2). Current technical status count in the resolution matrix: TECHNICALLY_ADDRESSED_PENDING_HUMAN=2.
- Resolution evidence is indexed in `execution/_Reconciliation/Reviews/DEV001_FINDING_RESOLUTION_PKG02_GROUNDED_2026-05-16/RESOLUTION_MATRIX.csv`; validation evidence is summarized in `execution/_Reconciliation/Reviews/DEV001_FINDING_RESOLUTION_PKG02_GROUNDED_2026-05-16/VALIDATION_SUMMARY.md`.
- Local `Review_Findings.csv` entries remain subject to the human disposition gate. `HumanDisposition` stays `TBD` until review, and `Status` must not be changed to `RESOLVED` automatically.
- No lifecycle promotion, release claim, or professional/code-compliance claim is implied by the technical closeout.

## 2026-05-16 TP-PHYS-003 Straight-Pipe End Resultants

- Executed approved `TP-PHYS-003-A` TASK slice for `DEL-04-02` / `PKG-04`
  with write scope limited to `core/solver/straight_pipe/**`, this
  `MEMORY.md`, and deliverable-local `_run_records/**`.
- Added code-neutral straight-pipe end-resultant recovery semantics:
  `PipeEnd::{I,J}`, `PipeEndResultants`,
  `recover_end_resultants(...)`, and
  `recover_end_resultants_from_global_model(...)`.
- Resultant values are direct local force-vector components for the requested
  element end. The recovery path does not reverse signs, classify code stress,
  apply allowables, or interpret engineering acceptance.
- Added focused tests proving direct I/J end component extraction and
  global-model displacement recovery into the J-end axial and torsional
  resultants needed by downstream stress work.
- Verification passed:
  `cargo fmt --manifest-path core/solver/straight_pipe/Cargo.toml --check`;
  `cargo test --manifest-path core/solver/straight_pipe/Cargo.toml` with 15
  tests passed.
- Remaining TBDs: sparse solver library, global tolerance policy, canonical
  unit conversions, primitive-load integration beyond explicit displacement
  recovery, result-envelope integration, release thresholds, and professional
  reliance.
- No `_STATUS.md`, dependency register, DAG, blocker queue, candidate row,
  DEV-001 finding disposition, protected standards content, allowables,
  SIF/flexibility data, code-compliance claim, or professional reliance claim
  was changed or introduced by this TASK slice.

## 2026-05-16 TP-PHYS-006 Partial-Span Distributed Loads

- Executed approved `TP-PHYS-006-A` TASK slice for `DEL-04-02` / `PKG-04`
  with write scope limited to `core/solver/straight_pipe/**`, this
  `MEMORY.md`, and deliverable-local `_run_records/**`.
- Added mechanics-only partial-span uniform distributed load support through
  `UniformLoadSpan`, `SpannedUniformLocalLoad`, and
  `SpannedGlobalUniformLoad`.
- Added spanned local/global equivalent nodal load recovery using exact
  elementary shape-function integration over `[start_fraction, end_fraction]`.
- Added spanned station-resultant recovery that accumulates only the active
  portion of a load between span start and the requested station.
- Existing full-span load APIs remain compatibility surfaces and delegate
  through full-span spanned wrappers.
- Verification passed:
  `cargo fmt --manifest-path core/solver/straight_pipe/Cargo.toml --check`;
  `cargo test --manifest-path core/solver/straight_pipe/Cargo.toml` with 24
  tests passed.
- Remaining TBDs: sparse solver library, global tolerance policy, canonical
  unit conversions, station envelopes beyond this slice, result-envelope
  integration, release thresholds, and professional reliance.
- No `_STATUS.md`, dependency register, DAG, blocker queue, candidate row,
  DEV-001 finding disposition, protected standards content, allowables,
  SIF/flexibility data, code-compliance claim, or professional reliance claim
  was changed or introduced by this TASK slice.

## 2026-05-16 TP-PHYS-005 Oriented Equivalent Load Transformation

- Executed approved `TP-PHYS-005-A` TASK slice for `DEL-04-02` / `PKG-04`
  with write scope limited to `core/solver/straight_pipe/**`, this
  `MEMORY.md`, and deliverable-local `_run_records/**`.
- Added orientation-aware equivalent-load recovery through
  `GlobalUniformLoad`, `GlobalPointForce`, and
  `StraightPipeElement::equivalent_global_nodal_loads`.
- The helper projects global force vectors through the element frame into
  local straight-pipe load components, recovers the existing local equivalent
  nodal load vector, and transforms the nodal load vector back to global
  solver DOFs.
- Added focused tests for uniform and point-force loads on a pipe aligned with
  global `Y`, proving that the recovered global nodal loads preserve force
  direction while using local beam mechanics internally.
- Verification passed:
  `cargo fmt --manifest-path core/solver/straight_pipe/Cargo.toml --check`;
  `cargo test --manifest-path core/solver/straight_pipe/Cargo.toml` with 21
  tests passed.
- Fan-in regression also passed for stress recovery:
  `cargo test --manifest-path core/loads/stress_recovery/Cargo.toml` with 20
  tests passed.
- Remaining TBDs: sparse solver library, global tolerance policy, canonical
  unit conversions, partial-span load recovery, station envelopes,
  result-envelope integration, release thresholds, and professional reliance.
- No `_STATUS.md`, dependency register, DAG, blocker queue, candidate row,
  DEV-001 finding disposition, protected standards content, allowables,
  SIF/flexibility data, code-compliance claim, or professional reliance claim
  was changed or introduced by this TASK slice.

## 2026-05-17 TP-PHYS-004 Load-To-Resultant Integration

- Executed approved `TP-PHYS-004-A` TASK slice for `DEL-04-02` / `PKG-04`
  with write scope limited to `core/solver/straight_pipe/**`, this
  `MEMORY.md`, and deliverable-local `_run_records/**`.
- Added mechanics-only straight-pipe load recovery primitives:
  `LocalLoadDirection`, `UniformLocalLoad`, `PointLocalForce`, and
  `StationResultants`.
- Added equivalent nodal load recovery for full-element local uniform loads
  and interior point forces, including axial force distribution and Hermite
  bending-force interpolation for local `Y`/`Z` loads.
- Added station-resultant recovery from loaded I-end resultants and solved
  displacement vectors. The displacement path subtracts equivalent nodal loads
  from recovered local forces before station accumulation so fixed-end load
  effects participate in shear and bending recovery.
- Corrected the station bending recurrence during implementation so a
  cantilever under explicit loads decays to the free-end shear/moment
  boundary instead of accumulating moment in the wrong direction.
- Verification passed:
  `cargo fmt --manifest-path core/solver/straight_pipe/Cargo.toml`;
  `cargo test --manifest-path core/solver/straight_pipe/Cargo.toml` with 19
  tests passed.
- Remaining TBDs: sparse solver library, global tolerance policy, canonical
  unit conversions, partial-span load recovery, station envelopes, release
  thresholds, and professional reliance.
- No `_STATUS.md`, dependency register, DAG, blocker queue, candidate row,
  DEV-001 finding disposition, protected standards content, allowables,
  SIF/flexibility data, code-compliance claim, or professional reliance claim
  was changed or introduced by this TASK slice.

## 2026-05-17 TP-PHYS-007 Station Resultant Sweeps

- Executed approved `TP-PHYS-007-A` TASK slice for `DEL-04-02` / `PKG-04`
  with write scope limited to `core/solver/straight_pipe/**`, this
  `MEMORY.md`, and deliverable-local `_run_records/**`.
- Added ordered station-resultant sweep helpers over the existing
  single-station straight-pipe recovery paths, covering full-span
  compatibility loads, spanned uniform loads, point forces, element
  displacement recovery, and global-model displacement recovery.
- Sweep helpers preserve caller station order and reuse existing deterministic
  validation for non-finite or out-of-range station fractions; they do not
  sort, deduplicate, clip, or infer result-envelope semantics.
- Added focused tests for ordered spanned-load station sweeps and invalid
  station rejection.
- Verification passed:
  `cargo fmt --manifest-path core/solver/straight_pipe/Cargo.toml`;
  `cargo test --manifest-path core/solver/straight_pipe/Cargo.toml` with 26
  tests passed.
- Remaining TBDs: sparse solver library, global tolerance policy, canonical
  unit conversions, final result-envelope/export/API integration, release
  thresholds, and professional reliance.
- No `_STATUS.md`, dependency register, DAG, blocker queue, candidate row,
  DEV-001 finding disposition, protected standards content, allowables,
  SIF/flexibility data, code-compliance claim, release claim, or professional
  reliance claim was changed or introduced by this TASK slice.

## 2026-05-17 TP-PHYS-008 Axial Effects For Thermal And Pressure

- Executed approved `TP-PHYS-008-A` TASK slice for `DEL-04-02` / `PKG-04`
  with write scope limited to `core/solver/straight_pipe/**`, this
  `MEMORY.md`, and deliverable-local `_run_records/**`.
- Added `StraightPipeAxialEffect` plus local/global equivalent nodal load
  recovery for explicit axial effects. Positive axial force is represented as
  `-F` at node I and `+F` at node J along local X before global
  transformation.
- Added corrected local-force, end-resultant, station-resultant, and
  station-sweep recovery helpers for axial-effect-only recovery from element
  and global-model displacement vectors.
- Verification passed:
  `cargo fmt --manifest-path core/solver/straight_pipe/Cargo.toml --check`;
  `cargo test --manifest-path core/solver/straight_pipe/Cargo.toml` with 30
  tests passed.
- Remaining TBDs: sparse solver library, global tolerance policy, canonical
  unit conversions, final result-envelope/export/API integration, release
  thresholds, and professional reliance.
- No `_STATUS.md`, dependency register, DAG, blocker queue, candidate row,
  DEV-001 finding disposition, protected standards content, allowables,
  SIF/flexibility data, code-compliance claim, release claim, or professional
  reliance claim was changed or introduced by this TASK slice.

## 2026-05-17 TP-PHYS-009-B Combined Axial/Load Recovery

- Executed approved `TP-PHYS-009-B` TASK slice for `DEL-04-02` / `PKG-04`
  with write scope limited to `core/solver/straight_pipe/**`, this
  `MEMORY.md`, and deliverable-local `_run_records/**`.
- Determined that existing axial-only and load-only APIs exposed the needed
  mechanics internals, but combined end/station recovery still required a
  caller to duplicate equivalent-load subtraction; production helper code was
  therefore warranted.
- Added combined mechanics-only recovery helpers for axial effects plus
  spanned local uniform loads and point local forces, covering element and
  global-model displacement inputs for end resultants, station resultants, and
  station sweeps.
- The helpers reuse existing equivalent-load, axial-effect, local-force, and
  station-resultant internals; they do not introduce new load mechanics,
  stress checks, rule checks, allowables, or compliance/professional claims.
- Added focused tests for combined fixed-end end-resultant recovery and
  ordered global-model station sweep recovery with axial effect, spanned
  distributed load, and point force present together.
- Verification passed:
  `cargo fmt --manifest-path core/solver/straight_pipe/Cargo.toml --check`;
  `cargo test --manifest-path core/solver/straight_pipe/Cargo.toml` with 32
  tests passed.
- Remaining TBDs: sparse solver library, global tolerance policy, canonical
  unit conversions, final result-envelope/export/API integration, release
  thresholds, and professional reliance.
- No `_STATUS.md`, dependency register, DAG, blocker queue, candidate row,
  DEV-001 finding disposition, protected standards content, allowables,
  SIF/flexibility data, code-compliance claim, release claim, or professional
  reliance claim was changed or introduced by this TASK slice.

## 2026-06-04 - TP-AUTHORITY-REFRESH-0_7-DAG006

- WORKING_ITEMS/TASK workforce current-authority refresh applied to active deliverable-local surfaces for `DEL-04-02`.
- Current authority basis is `execution/_Decomposition/SOFTWARE_DECOMP.md` revision `0.7` plus approved `execution/_DAG/DAG-006/` active graph authority.
- Historical run records, historical DAG row IDs, review dispositions, lifecycle `_STATUS.md`, aggregate DAG artifacts, candidate edges, repo-level governance files, schemas, code, and tools were intentionally not changed by this refresh.
- Preserved historical references remain evidence of earlier work, not current authority claims.

## 2026-06-05 - Force-per-length straight-pipe metadata boundary

- Executed Worker B for `TP-DEL-04-01-04-02-05-03-FORCE-PER-LENGTH-BOUNDARY-001`
  with write scope limited to `core/solver/straight_pipe/**`, this
  `MEMORY.md`, and deliverable-local `_run_records/**`.
- Changed `StraightPipeBoundaryMetadata` so `weight_force_per_length_unit`
  requires `CanonicalDimension::ForcePerLength` instead of the prior `TBD`
  placeholder.
- Added focused tests proving the accepted `force_per_length` metadata is
  carried and the old `TBD` dimension is rejected.
- Updated `core/solver/straight_pipe/README.md` to describe explicit
  `force_per_length` unit metadata while preserving the no-conversion and
  no-default boundary.
- Verification passed:
  `cargo fmt --manifest-path core/solver/straight_pipe/Cargo.toml --check`;
  `cargo test --manifest-path core/solver/straight_pipe/Cargo.toml --locked`
  with 33 tests passed.
- No `_STATUS.md`, review disposition, dependency register, DAG artifact,
  schema file, repo governance file, lifecycle state, release claim,
  professional approval, code-compliance claim, protected standards content,
  private data, unit conversion, load default, or tolerance policy was changed
  or introduced.

## 2026-06-05 - Force-per-length boundary parent fan-in

- WORKING_ITEMS completed parent fan-in for
  `TP-DEL-04-01-04-02-05-03-FORCE-PER-LENGTH-BOUNDARY-001` spanning
  `DEL-04-01`, `DEL-04-02`, and `DEL-05-03`.
- Validation evidence passed: frame-kernel format check and 34 locked tests;
  straight-pipe format check and 33 locked tests; stress-recovery format check
  and 24 locked tests; primitive-load locked tests with 40 tests;
  product-physics locked tests with 23 tests; `git diff --check`.
- Fan-in record:
  `_run_records/WORKING_ITEMS_RUN_2026-06-05_2000_FORCE-PER-LENGTH-BOUNDARY_FANIN.md`.
- No lifecycle state, review disposition, dependency register, DAG artifact,
  coordination prompt, schema file, repo governance file, release claim,
  professional approval, code-compliance claim, protected standards content,
  private data, conversion constant, load default, or tolerance policy was
  changed by parent fan-in.

## 2026-06-05 - Blocker closure and lifecycle-readiness review

- Human ruling packet: `execution/_Reconciliation/Reviews/PKG05_BLOCKER_CLOSURE_RULING_PACKET_2026-06-05_2120.md`.
- Review snapshot: `execution/_Reconciliation/Reviews/REV_DEL-04-02_2026-06-05_2120/`.
- Recommendation: `RECOMMEND_ADVANCE_TO_CHECKING` for `IN_PROGRESS -> CHECKING`.
- Lifecycle action: CHECKING applied by approved blocker-closure ruling.
- Finding disposition: Findings `PKG04-DEL0402-PKG02-001` and `PKG04-DEL0402-PKG02-002` were accepted as technically resolved by human ruling and set to `HumanDisposition=ACCEPT_AS_IS`, `Status=RESOLVED`.
- Dependency update: Rows `DAG-002-E0432`, `DAG-002-E0433`, and `DAG-002-E0434` were updated from `UNKNOWN` to `SATISFIED` using current DEL-04-01, DEL-03-08, and DEL-02-02 evidence plus straight-pipe boundary metadata.
- Validation: Straight-pipe format check passed; locked crate tests passed with 33 unit tests and 0 doctests.
- Residual boundaries: Sparse solver library, production tolerance policy, release thresholds, final result-envelope integration, and professional reliance remain explicit downstream or human-governed TBDs.
- No release, professional approval, certification, sealing, authentication, code-compliance claim, protected standards data, or private data was introduced.

## 2026-06-17 - Lifecycle Housekeeping

- Housekeeping lifecycle reset: `_STATUS.md` current state set to `IN_PROGRESS` to reflect current code development in progress. This does not change review, issuance, release readiness, professional approval, certification, sealing, authentication, or code-compliance status.
