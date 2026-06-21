# MEMORY - DEL-04-04 Nonlinear Support Active-Set Solver

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
