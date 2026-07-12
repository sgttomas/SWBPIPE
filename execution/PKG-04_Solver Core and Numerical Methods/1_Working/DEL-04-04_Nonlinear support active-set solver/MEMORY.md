# MEMORY - DEL-04-04 Nonlinear Support Active-Set Solver

## 2026-07-12 - D-41 R5 T4 PDU-021 supporting evidence

- The downstream report reader preserves solver context, warnings, assumptions, limitations, and provenance when present in canonical run records. Direct nonlinear-solver producer binding and existing PDU-035/threshold holds remain open.
- Focused evidence is recorded in `_run_records/WORKING_ITEMS_RUN_2026-07-12_D41-R5-T4-PDU012-PDU021-PDU022-PDU040.md`; lifecycle remains `IN_PROGRESS`, with no review, validation, approval, compliance, release, or professional-reliance conclusion.

## 2026-05-01 bounded implementation

Human project authority authorized one deliberately scoped `DEL-04-04` item after
`DEL-04-03` implementation evidence was committed.

Implemented:

- `core/solver/nonlinear_supports`, a Rust mechanics-boundary crate for
  nonlinear support active-set decisions.
- Explicit behavior models for one-way supports, gaps, lift-off/contact, and
  friction-limited supports.
- Trial-state classification into active, inactive, sticking, and sliding
  states.
- Active-set iteration records with changed supports, residual norm,
  convergence flag, and diagnostics sourced through `core/solver/diagnostics`.
- Tests for one-way activation, gap closure, lift-off, friction stick/slip,
  convergence, nonconvergence, and invalid numeric inputs.

Boundaries preserved:

- No global nonlinear matrix assembly or production solve loop integration.
- No sparse-solver selection, production tolerance policy, load-case algebra,
  rule-pack checks, protected standards data, public support/catalog defaults,
  lifecycle transition, dependency-register edit, or professional/code-
  compliance claim.

Open items:

- Canonical calculation unit basis, conversion constants, final support
  coordinate convention, rigid-restraint numerical method, constraint-
  elimination or penalty strategy, sparse-solver integration, production
  residual/tolerance policy, and final result-envelope integration remain
  `TBD`.

## 2026-05-11 TP-RECON-01 Reconciliation

Reconciled `DEL-04-04` history against the TP-RECON-01 dispatch row and the
archived DEV-001 evidence bundle. The committed evidence remains `d3c3533`
(`core: add nonlinear support active-set model`, committed 2026-05-01) for
`BACKEND_FEATURE_SLICE` scope `SOW-012` / objective `OBJ-003`.

Implemented evidence recorded for this deliverable:

- `core/solver/nonlinear_supports` was added as a mechanics-only Rust module
  for nonlinear support active-set decisions.
- The implementation covers one-way, gap, lift-off/contact, and
  friction-limited support state classification with active/inactive,
  sticking, and sliding states.
- Iteration records include changed supports, residual norm, convergence flag,
  and diagnostics through the solver diagnostics boundary.
- Commit `d3c3533` also updated `docs/SPEC.md`, `docs/TYPES.md`, and the
  deliverable memory/coordination dispatch records.

Verification evidence remains bounded to the committed DEV-001 item and
deliverable-local run records: tests were recorded for one-way activation,
gap/lift-off, friction stick/slip, convergence, nonconvergence, invalid numeric
inputs, and deterministic iteration state. The REV05 lifecycle snapshot carries
`CHECKING` with implementation evidence `COMMITTED`.

Deferred scope remains unchanged: global nonlinear solve integration,
production residual/tolerance policy, sparse-solver integration, final
constraint strategy, final coordinate convention, result-envelope integration,
protected/code-specific data, and professional reliance language remain outside
this reconciliation.

## 2026-05-16 - DEV-001 downstream PKG-02 audit memory addendum

Durable context preserved after reconciliation review:
- DEV-001 package-worker audit reviewed this deliverable for downstream compatibility with the accepted PKG-02 foundation contracts.
- Local audit artifacts are `execution/PKG-04_Solver Core and Numerical Methods/1_Working/DEL-04-04_Nonlinear support active-set solver/_REVIEW.md` and `execution/PKG-04_Solver Core and Numerical Methods/1_Working/DEL-04-04_Nonlinear support active-set solver/Review_Findings.csv`.
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

## 2026-06-04 - TP-AUTHORITY-REFRESH-0_7-DAG006

- WORKING_ITEMS/TASK workforce current-authority refresh applied to active deliverable-local surfaces for `DEL-04-04`.
- Current authority basis is `execution/_Decomposition/SOFTWARE_DECOMP.md` revision `0.7` plus approved `execution/_DAG/DAG-006/` active graph authority.
- Historical run records, historical DAG row IDs, review dispositions, lifecycle `_STATUS.md`, aggregate DAG artifacts, candidate edges, repo-level governance files, schemas, code, and tools were intentionally not changed by this refresh.
- Preserved historical references remain evidence of earlier work, not current authority claims.

## 2026-06-04 - TASK nonlinear edge hardening

- Hardened `core/solver/nonlinear_supports` edge behavior for public-struct callers by rejecting missing gap clearance, missing friction coefficient, and non-finite bypassed friction coefficients during classification instead of silently treating them as zero.
- Added blocking solver-diagnostic mapping for missing gap and missing friction-coefficient support definitions.
- Annotated nonlinear nonconvergence diagnostics with changed active-set support IDs and final active-set state summaries.
- Added focused crate tests for missing gap data, negative contact reaction, missing friction coefficient, non-finite public friction coefficient, and active-set context in nonconvergence diagnostics.
- Verification passed: `cargo fmt --manifest-path core/solver/nonlinear_supports/Cargo.toml --check`; `cargo test --manifest-path core/solver/nonlinear_supports/Cargo.toml` (14 tests); `git diff --check`.
- Boundaries preserved: no global nonlinear solve integration, production tolerance policy, sparse-solver selection, lifecycle edit, DAG edit, dependency-register edit, review-disposition edit, protected standards data, rule/code check, or professional/compliance claim.
- Remaining `TBD`: global nonlinear solve integration, final result-envelope integration, accepted production residual/tolerance policy, sparse-solver integration, canonical calculation unit basis/conversions, final support coordinate convention, and human disposition for existing review findings.

## 2026-06-04 - TP-PHYS-024 parent fan-in

- WORKING_ITEMS fan-in reviewed the completed parallel TASK slice for `DEL-04-04` together with sibling slices for `DEL-04-05`, `DEL-05-02`, and `DEL-05-05`.
- Aggregate validation passed after fan-in: `cargo test --manifest-path core/solver/nonlinear_supports/Cargo.toml` (14 tests), `cargo test --manifest-path core/solver/performance_harness/Cargo.toml` (8 tests), `cargo test --manifest-path core/loads/load_case_algebra/Cargo.toml` (17 tests), `cargo test --manifest-path core/loads/user_loads/Cargo.toml` (28 tests), each corresponding `cargo fmt --check`, and `git diff --check`.
- Fan-in found no cross-worker scope drift for this deliverable. Lifecycle, DAG, dependency, review-disposition, release, professional-approval, and code-compliance surfaces remain unchanged.

## 2026-06-05 - Worker A report-facing active-set record

- Added `ActiveSetReportRecord` and `ActiveSetSupportReportState` to `core/solver/nonlinear_supports` so downstream reporting can inspect iteration, max iterations, tolerance, residual norm, convergence state, per-support state, changed supports, structured diagnostics, assumptions, and limitations without parsing diagnostic text.
- Preserved the existing `ActiveSetIteration` API and added `ActiveSetIteration::to_report_record` plus `evaluate_active_set_report` as companion conversion/evaluation surfaces.
- Added focused tests for explicit report-record fields and structured nonconvergence diagnostics; crate README now records the report-facing boundary.
- Verification passed: `cargo fmt --manifest-path core/solver/nonlinear_supports/Cargo.toml --check`; `cargo test --manifest-path core/solver/nonlinear_supports/Cargo.toml --locked` (16 tests).
- Boundaries preserved: no global nonlinear solve integration, production tolerance policy, sparse-solver selection, lifecycle edit, DAG edit, dependency-register edit, review-disposition edit, protected standards data, rule/code check, or professional/compliance claim.
- Remaining `TBD`: global nonlinear solve integration, final result-envelope integration, accepted production residual/tolerance policy, sparse-solver integration, canonical calculation unit basis/conversions, final support coordinate convention, final constraint strategy, and human disposition for existing review findings.

## 2026-06-17 - Lifecycle Housekeeping

- Housekeeping lifecycle reset: `_STATUS.md` current state set to `IN_PROGRESS` to reflect current code development in progress. This does not change review, issuance, release readiness, professional approval, certification, sealing, authentication, or code-compliance status.

## 2026-06-21 - TP-R4-D6-LIVEBUNDLE-001

- Added product-preview live dense-loop regression coverage for a mixed
  invented support bundle: one-way deactivation, gap closure, and
  explicit-normal friction sliding in one `core/product_physics` solve.
- The active-set classifier remains the per-iteration state oracle and now has
  product-level evidence that multiple support classes can be consumed together
  by the integration loop without adding hidden normal-force defaults or
  generated friction loads.
- Validation passed in the tranche run record:
  `../_run_records/WORKING_ITEMS_RUN_2026-06-21_TP-R4-D6-LIVEBUNDLE-001.md`.
  Boundaries preserved: no sparse live-path binding, governed threshold
  promotion, lifecycle transition, release-readiness claim, professional
  approval, certification, sealing, authentication, or code-compliance claim.

## 2026-06-21 - TP-R4-D6-FRICTIONNORMAL-001

- D6 integration tranche added derived friction normal-reaction support in
  `core/solver/nonlinear_integration`: callers may name a restrained linear
  support DOF and the dense active-set loop uses the absolute reaction at that
  DOF as friction normal evidence.
- The active-set classifier itself remains the per-iteration state oracle; no
  protected normal-force value, catalog/default support value, sparse live path,
  release convergence value, lifecycle transition, or professional/code-
  compliance claim was introduced.
- Validation passed in the tranche run record:
  `../_run_records/WORKING_ITEMS_RUN_2026-06-21_TP-R4-D6-FRICTIONNORMAL-001.md`,
  including the full five-surface evidence sweep summary
  `validation/evidence/sweeps/SWEEP_20260621T092312Z_53b592aee006-dirty.json`.

## 2026-06-21 - TP-R4-D6-LOOPCORE-001 ownership re-point

- `DEC-044` re-pointed assembled nonlinear loop ownership to the PKG-04
  integration tranche `core/solver/nonlinear_integration`, bridging DEL-04-04
  and DEL-04-01.
- DEL-04-04 remains the per-iteration active-set classifier/state oracle:
  one-way, gap, lift-off/contact, and friction classification plus
  report-facing active-set records and diagnostics.
- Updated this deliverable's context, datasheet, and specification to remove
  stale ownership language that made DEL-04-04 the owner of the assembled loop.
- Boundaries preserved: no lifecycle transition, release-readiness claim,
  professional approval, certification, sealing, authentication,
  code-compliance claim, protected standards content, public defaults, or
  private data was introduced.

## 2026-06-21 - TP-R4-D9-FRICTIONSLIDE-001 friction sliding anti-chatter

- Added a deterministic active-set iteration rule for friction supports:
  when a support was already `Sliding`, and the released DOF still has nonzero
  trial displacement, the iteration keeps the support in `Sliding` rather than
  chattering back to `Sticking` solely because the released reaction is zero.
- Direct per-state classification remains unchanged; the persistence rule is
  scoped to iteration evaluation and documented in report assumptions.
- Added focused crate coverage in `core/solver/nonlinear_supports` and
  assembled-loop coverage in `core/solver/nonlinear_integration`.
- Evidence is recorded in the PKG-04 run record
  `../_run_records/WORKING_ITEMS_RUN_2026-06-21_TP-R4-D9-FRICTIONSLIDE-001.md`.
- Boundaries preserved: explicit invented normal input only; no derived
  normal-force model, sparse live-path binding, measured convergence values,
  lifecycle transition, release-readiness claim, professional approval,
  certification, sealing, authentication, or code-compliance claim.

## 2026-06-22 - TP-R4-D7-NONLINEARPROFILE-001 nonlinear direct-profile sparse evidence

- `core/solver/nonlinear_integration` sparse evidence now records
  `assembly_basis=direct_reduced_profile_entries` and uses
  `solve_symmetric_system_from_entries` against entries assembled from the
  active free-DOF map.
- Dense remains the active nonlinear solve path and parity oracle; sparse
  evidence stays non-blocking observation.
- Validation passed, including nonlinear integration tests 11/11, product
  physics tests 44/44, nonlinear benchmark tests 16/16, crate fmt check, and
  full DEC-025 sweep
  `validation/evidence/sweeps/SWEEP_20260622T102208Z_479c6fd7d97a-dirty.json`.
- Residuals remain explicit: default sparse promotion / large-model sparse
  suitability, non-seed convergence thresholds, accepted
  displacement/reaction-delta thresholds, deeper spring-hanger behavior,
  external validation thresholds, and final R4 exit evidence remain open.

## 2026-07-10 - TP-PMM-P2-COMPLEMENTARITY-001 / TP-PMM-P2-FRICTION-001 DEC-067 clauses

- `classify_support_state` now takes the prior state and applies the DEC-067
  state-switched complementarity test: engaged one-way/lift-off/gap supports
  (including closed gaps) classify on trial reaction sign; released supports
  classify on trial displacement penetration toward the bearing side (or the
  explicit gap clearance). Re-engagement of a lifted support and lift-off of a
  closed gap are now observable, and converged states no longer depend on
  seeded `initial_states`. The friction stick/slip test and the deterministic
  sliding-persistence anti-chatter rule are unchanged.
- `core/solver/nonlinear_integration` applies a bounded `+/- mu*N` Coulomb
  tangential force at supports classified sliding (opposing the prior
  iterate's motion, normal reaction from current-iterate evidence) instead of
  a full DOF release; support reactions are reported against the caller's
  base force so the bounded force appears as the support's tangential
  reaction. A sliding state seeded before any solved iterate defers
  convergence one iteration so the bound is applied before the loop can
  converge (seed-independence witnessed by test). No path/load-step friction
  history model was added.
- New transition witnesses + assembled fixtures:
  `validation/hand_calcs/nonlinear/assembled_one_way_reengagement.md`,
  `assembled_gap_lift_off.md`, `assembled_friction_bounded_sliding.md` with
  matching `NL-ASSEMBLED-ONE-WAY-REENGAGE-ORIGINAL`,
  `NL-ASSEMBLED-GAP-LIFT-OFF-ORIGINAL`,
  `NL-ASSEMBLED-FRICTION-BOUNDED-SLIDE-ORIGINAL` cases under the existing
  DEC-046 class-tiered tighten-only policy refs (unmeasured axes stay TBD; the
  DEC-046 policy mechanism itself is unchanged).
- Reconciled existing invented fixtures to the new mechanics (classifier-level
  one-way/gap/nonconvergence trials; sliding fixtures' invented normal
  evidence retuned so bounded net drives stay exactly representable under the
  0.0 residual limits); product-preview canned envelope regenerated for the
  bounded-sliding rows.
- Residuals: sliding-direction evidence comes from the prior iterate only, so
  a sliding support whose bound exceeds the drive can oscillate to the visible
  nonconvergence cap instead of re-sticking (anti-chatter interaction,
  recorded, not silent); sliding-force magnitude is not itself a convergence
  residual axis; arc interior stations remain open under DEC-070.
- Boundaries preserved: user-entered values only, no defaults, no protected
  content, no lifecycle transition, release-readiness, professional,
  certification, sealing, authentication, or code-compliance claim.

## 2026-07-10 - TP-PMM-P2-NONCONVDIAG-001 nonconvergence-diagnostic corner closed

- Closed the TP-PMM-P2-FRICTION-001 recorded corner: a sliding-seeded friction
  support at `max_iterations == 1` returned `converged == false` with a zero
  active-set residual and no diagnostic. The assembled loop in
  `core/solver/nonlinear_integration` now guards every non-converged exit: if
  the final diagnostics carry no residual-based `NonConvergence` code, it
  emits a `NonConvergence` failure diagnostic naming the deferred
  sliding-force cause (or a generic non-converged exit), the iteration count,
  and the final active-set state summary, with remediation to raise
  `max_iterations`.
- Diagnostics only: no mechanics change, no friction history model (DEC-067
  fence), no DEC-046 threshold change (TBD axes stay TBD), and the
  residual-based classifier diagnostic path is unchanged (the guard never
  double-emits when the classifier already reported nonconvergence).
- New crate tests: sliding seed at the single-iteration cap emits exactly one
  `NonConvergence` failure diagnostic; a sticking-friction solve that
  genuinely converges at `max_iterations == 1` gains no false positive; the
  existing state-switching capped-gap test now pins exactly one failure
  diagnostic.
- Residuals unchanged otherwise: anti-chatter oscillation still reaches the
  visible cap (now always loud), sliding-force magnitude is not a residual
  axis, arc interior stations remain open under DEC-070.
## 2026-07-12 - D-41 R5 T2 PDU-035 hold

- Unit metadata binding remains technically addressed at metadata grain.
- The PKG-02 review finding still requires its owning human disposition, and dimensional/conversion suitability is not independently validated; PDU-035 remains held.
