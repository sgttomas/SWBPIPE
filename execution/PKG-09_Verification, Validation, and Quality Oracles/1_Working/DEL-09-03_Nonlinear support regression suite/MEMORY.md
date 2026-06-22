# MEMORY - DEL-09-03 Nonlinear support regression suite

## Scope

Implemented a bounded nonlinear support regression suite for DEV-001 revision
0.5 Tranche A using invented, non-project fixtures only.

## Evidence

- Added `validation/benchmarks/nonlinear/` as an executable Rust benchmark crate.
- Covered active-set one-way support, gap closure, lift-off, friction
  stick/slide behavior, and unresolved non-convergence at the iteration limit.
- Added `tests/test_nonlinear_support_regression.py` as the focused repo-level
  regression entry point for the nonlinear benchmark crate.

## Validation

- `cargo test --quiet` in `validation/benchmarks/nonlinear`: passed, 5 tests.
- `python3 -m pytest -q tests/test_nonlinear_support_regression.py`: passed, 2 tests.
- Existing `cargo test --quiet` in `core/solver/nonlinear_supports`: passed, 8 tests.
- Existing `cargo test --quiet` in `core/solver/diagnostics`: passed, 10 tests.
- `git diff --check`: passed.
- Protected-content, private-data, credential, and professional-claim scans over the
  DEL-09-03 changed files: passed.

## Remaining TBDs

- Non-seed force/displacement thresholds beyond the accepted nine-fixture
  multi-support companion set, general energy thresholds,
  displacement/reaction-delta thresholds beyond the current seed, accepted
  nine-fixture set, and invented product-preview mixed surface, sparse
  default/promotion thresholds, release thresholds, and external validation
  thresholds remain `TBD`.
- The current assembled validation seed has an accepted active-set
  changed-support-count policy under `DEC-046-CV-B-active-set-count-validation-v1`.
- The current assembled validation seed has an accepted free-DOF force/moment
  residual policy under
  `DEC-046-CV-B-free-dof-force-moment-residual-validation-v1`.
- The current assembled validation seed has an accepted final-iteration
  free-DOF work residual policy under
  `DEC-046-CV-B-free-dof-work-residual-validation-v1`.
- The current assembled validation seed has accepted fixture-evidence-envelope
  displacement/reaction delta threshold policies under
  `DEC-046-CV-B-displacement-reaction-delta-threshold-validation-v1`.
- The current invented product-preview nonlinear path has an accepted
  active-set changed-support-count policy under
  `DEC-046-CV-B-product-preview-active-set-count-v1`.
- The current invented product-preview nonlinear path has an accepted free-DOF
  force/moment residual policy under
  `DEC-046-CV-B-product-preview-free-dof-force-moment-residual-v1`.
- The current invented product-preview nonlinear path has an accepted free-DOF
  work residual policy under
  `DEC-046-CV-B-product-preview-free-dof-work-residual-v1`.
- The invented mixed product-preview nonlinear path has an accepted
  displacement/reaction delta threshold policy under
  `DEC-046-CV-B-product-preview-displacement-reaction-delta-threshold-v1`.
- `TP-R4-D9-MULTISUPPORTPOLICY-001`,
  `TP-R4-D9-MULTISUPPORTBREADTH-001`,
  `TP-R4-D9-MULTISUPPORTFRICTION-001`,
  `TP-R4-D9-MULTISUPPORT3DOF-001`,
  `TP-R4-D9-MULTISUPPORTROT-001`,
  `TP-R4-D9-MULTISUPPORTDERIVED-001`,
  `TP-R4-D9-MULTISUPPORTDERIVEDROT-001`, and
  `TP-R4-D9-MULTISUPPORTCASCADE-001`, and
  `TP-R4-D9-MULTISUPPORTNEGAP-001` add nine accepted multi-DOF /
  multi-support companion fixtures under narrow active-set, free-DOF
  force/moment, free-DOF work residual, and displacement/reaction delta
  threshold policies (`TP-R4-D9-WORKPOLICY-001` and
  `TP-R4-D9-DISPREACTIONPOLICY-001`).
- `TP-R4-D9-MULTISUPPORTOBS-001` adds one observation-only multi-DOF /
  multi-support assembled fixture under `TP-R4-D9-MULTISUPPORT-OBS-TBD`; it is
  not part of the accepted current assembled validation seed policy.
- `TP-R4-D9-DISPREACTIONOBS-001` adds observation-only
  displacement/reaction-delta ledgers for the current assembled validation
  seed and accepted multi-support fixture set, plus product-preview metadata
  that was promoted in `TP-R4-D9-PRODDISPREACTIONPOLICY-001`.
- `TP-R4-D9-DISPREACTIONPOLICY-001` promotes fixture-evidence-envelope
  displacement/reaction delta threshold policies for the current assembled
  seed and accepted nine-fixture multi-support set only.
- `TP-R4-D9-PRODDISPREACTIONPOLICY-001` promotes the invented mixed
  product-preview displacement/reaction delta rows to an accepted product-only
  threshold policy; broader non-seed, release, external, sparse-default, and
  general energy thresholds remain `TBD`.
- External validation claims remain `TBD`; this suite is software verification
  evidence only.

## 2026-06-22 - TP-R4-D9-MULTISUPPORTNEGAP-001

- Added `NL-ASSEMBLED-MULTI-DOF-NEGATIVE-GAP-ACCEPTED-ORIGINAL`, an invented
  assembled frame fixture that pairs negative-direction Uy gap closure with Ux
  one-way release.
- Broadened the accepted multi-support validation fixture set from eight
  companions to nine companions: one-way/gap, lift-off/gap, friction/gap,
  three-support/three-translation, mixed translation/rotation,
  derived-normal friction/gap, derived-normal friction/rotational lift-off,
  sequential gap/lift-off cascade, and negative-direction gap/one-way.
- Updated the multi-support active-set-count, free-DOF force/moment,
  free-DOF work, and displacement/reaction-delta policy records so their
  evidence fixture IDs name the nine-fixture public-original set.
- Boundary preserved: no non-seed force/displacement threshold beyond the
  accepted nine-fixture set, broader displacement-delta or reaction-delta
  threshold, general energy threshold, sparse-default, external validation,
  release-readiness, lifecycle, professional, or code-compliance claim was
  added.
- Validation evidence is in
  `_run_records/WORKING_ITEMS_RUN_2026-06-22_TP-R4-D9-MULTISUPPORTNEGAP-001.md`.

## 2026-06-22 - TP-R4-D9-PRODDISPREACTIONPOLICY-001

- Added `DEC-046-CV-B-product-preview-displacement-reaction-delta-threshold-v1`
  to `core/product_physics` as the product-preview-only policy for emitted
  displacement/reaction delta rows.
- Bound the mixed one-way/gap/friction product preview regression to explicit
  limits: `50.0 mm`, `0.05 rad`, `110000.0 N`, and `110000.0 N*m`.
- Regenerated `fixtures/product_preview/invented_mechanics_result.json` so the
  canned preview diagnostics surface the policy ref for emitted delta rows,
  while one-iteration fixture cases still emit no synthetic delta rows.
- Boundary preserved: no broader non-seed validation threshold, release
  threshold, external validation threshold, sparse-default threshold, general
  energy threshold, lifecycle, professional, or code-compliance claim was
  added.
- Validation evidence is in
  `_run_records/WORKING_ITEMS_RUN_2026-06-22_TP-R4-D9-PRODDISPREACTIONPOLICY-001.md`.

## 2026-06-22 - TP-R4-D9-DISPREACTIONPOLICY-001

- Added governed `DEC-046` displacement/reaction delta threshold policy records:
  `validation/benchmarks/nonlinear/displacement_reaction_delta_policy.dec046.json`
  and
  `validation/benchmarks/nonlinear/multisupport_displacement_reaction_delta_policy.dec046.json`.
- Promoted the current assembled validation seed to accepted class-tiered
  fixture-evidence-envelope limits: one-way/lift-off/friction `100.0 mm`,
  `0.0 rad`, `10.0 N`, `0.0 N-m`; gap `50.0 mm`, `0.0 rad`, `5.0 N`,
  `0.0 N-m`.
- Promoted the accepted eight-fixture multi-support set to fixture-evidence
  envelope limits: `100.0 mm`, `0.005 rad`, `10.0 N`, and `3.0 N-m`.
- Updated Rust policy predicates, focused Python guards, benchmark README, and
  hand-calc notes so accepted seed and accepted multi-support residual
  observations cite the policy while the separate multi-support depth fixture
  and product-preview metadata remain observation-only / `TBD`.
- Boundary preserved: no product-preview delta threshold, broader non-seed
  force/displacement threshold, general energy threshold, sparse-default,
  external validation, release-readiness, lifecycle, professional, or
  code-compliance claim was added.
- Validation evidence is in
  `_run_records/WORKING_ITEMS_RUN_2026-06-22_TP-R4-D9-DISPREACTIONPOLICY-001.md`.

## 2026-06-22 - TP-R4-D9-MULTISUPPORTCASCADE-001

- Added `NL-ASSEMBLED-MULTI-DOF-CASCADE-GAP-LIFT-OFF-ACCEPTED-ORIGINAL`, an
  invented assembled frame fixture with a measured sequential active-set
  cascade: first Rz lift-off release, then Uy gap closure, then final
  convergence.
- Broadened the accepted multi-support validation fixture set from seven
  companions to eight companions: one-way/gap, lift-off/gap, friction/gap,
  three-support/three-translation, mixed translation/rotation,
  derived-normal friction/gap, derived-normal friction/rotational lift-off, and
  sequential gap/lift-off cascade.
- Updated the multi-support active-set-count, free-DOF force/moment,
  free-DOF work, and displacement/reaction-delta observation records so their
  evidence fixture IDs name the eight-fixture public-original set.
- Boundary preserved: no non-seed force/displacement threshold beyond the
  accepted eight-fixture set, displacement-delta, reaction-delta, general
  energy, sparse-default, external validation, release-readiness, lifecycle,
  professional, or code-compliance claim was added.
- Validation evidence is in
  `_run_records/WORKING_ITEMS_RUN_2026-06-22_TP-R4-D9-MULTISUPPORTCASCADE-001.md`.

## 2026-06-22 - TP-R4-D9-MULTISUPPORTDERIVEDROT-001

- Added
  `NL-ASSEMBLED-MULTI-DOF-DERIVED-NORMAL-ROTATIONAL-ACCEPTED-ORIGINAL`, an
  invented assembled frame fixture with derived-normal friction sliding on Ux
  and rotational lift-off release on Rz in the first iteration.
- Broadened the accepted multi-support validation fixture set from six
  companions to seven companions: one-way/gap, lift-off/gap, friction/gap,
  three-support/three-translation, mixed translation/rotation,
  derived-normal friction/gap, and derived-normal friction/rotational lift-off.
- Updated the multi-support active-set-count, free-DOF force/moment,
  free-DOF work, and displacement/reaction-delta observation records so their
  evidence fixture IDs name the seven-fixture public-original set.
- Boundary preserved: no non-seed force/displacement threshold beyond the
  accepted seven-fixture set, displacement-delta, reaction-delta, general
  energy, sparse-default, external validation, release-readiness, lifecycle,
  professional, or code-compliance claim was added.
- Validation evidence is in
  `_run_records/WORKING_ITEMS_RUN_2026-06-22_TP-R4-D9-MULTISUPPORTDERIVEDROT-001.md`.

## 2026-06-22 - TP-R4-D9-MULTISUPPORTDERIVED-001

- Added `NL-ASSEMBLED-MULTI-DOF-DERIVED-NORMAL-GAP-ACCEPTED-ORIGINAL`, an
  invented assembled frame fixture with derived-normal friction sliding on Ux
  and gap closure on Uz in the first iteration.
- Broadened the accepted multi-support validation fixture set from five
  companions to six companions: one-way/gap, lift-off/gap, friction/gap,
  three-support/three-translation, mixed translation/rotation, and
  derived-normal friction/gap.
- Updated the multi-support active-set-count, free-DOF force/moment,
  free-DOF work, and displacement/reaction-delta observation records so their
  evidence fixture IDs name the six-fixture public-original set.
- Boundary preserved: no non-seed force/displacement threshold beyond the
  accepted six-fixture set, displacement-delta, reaction-delta, general
  energy, sparse-default, external validation, release-readiness, lifecycle,
  professional, or code-compliance claim was added.
- Validation evidence is in
  `_run_records/WORKING_ITEMS_RUN_2026-06-22_TP-R4-D9-MULTISUPPORTDERIVED-001.md`.

## 2026-06-22 - TP-R4-D9-MULTISUPPORTROT-001

- Added `NL-ASSEMBLED-MULTI-DOF-ROTATIONAL-ACCEPTED-ORIGINAL`, an invented
  assembled frame fixture with simultaneous Ux one-way release and Rz lift-off
  release in the first iteration.
- Broadened the accepted multi-support validation fixture set from four
  companions to five companions: one-way/gap, lift-off/gap, friction/gap,
  three-support/three-translation, and mixed translation/rotation.
- Updated the multi-support active-set-count, free-DOF force/moment,
  free-DOF work, and displacement/reaction-delta observation records so their
  evidence fixture IDs name the five-fixture public-original set.
- Boundary preserved: no non-seed force/displacement threshold beyond the
  accepted five-fixture set, displacement-delta, reaction-delta, general
  energy, sparse-default, external validation, release-readiness, lifecycle,
  professional, or code-compliance claim was added.
- Validation evidence is in
  `_run_records/WORKING_ITEMS_RUN_2026-06-22_TP-R4-D9-MULTISUPPORTROT-001.md`.

## 2026-06-22 - TP-R4-D9-MULTISUPPORT3DOF-001

- Added `NL-ASSEMBLED-MULTI-DOF-THREE-SUPPORT-ACCEPTED-ORIGINAL`, an invented
  assembled frame fixture with simultaneous Ux one-way release, Uy gap closure,
  and Uz friction sliding in the first iteration.
- Broadened the accepted multi-support validation fixture set from three
  companions to four companions: one-way/gap, lift-off/gap, friction/gap, and
  three-support/three-translation.
- Updated the multi-support active-set-count, free-DOF force/moment,
  free-DOF work, and displacement/reaction-delta observation records so their
  evidence fixture IDs name the four-fixture public-original set.
- Boundary preserved: no non-seed force/displacement threshold beyond the
  accepted four-fixture set, displacement-delta, reaction-delta, general
  energy, sparse-default, external validation, release-readiness, lifecycle,
  professional, or code-compliance claim was added.
- Validation evidence is in
  `_run_records/WORKING_ITEMS_RUN_2026-06-22_TP-R4-D9-MULTISUPPORT3DOF-001.md`.

## 2026-06-22 - TP-R4-D9-DISPREACTIONOBS-001

- Added machine-readable observation-only displacement/reaction-delta records:
  `validation/benchmarks/nonlinear/displacement_reaction_delta_observation.dec046.json`
  and
  `validation/benchmarks/nonlinear/multisupport_displacement_reaction_delta_observation.dec046.json`.
- Added typed nonlinear benchmark ledger entries and tests proving those axes
  are dimensioned observations with `ConvergencePolicyStatus::Tbd`, not
  accepted threshold policies.
- Updated product-preview nonlinear residual metadata so translation/reaction
  delta rows name the product observation reference and carry
  `threshold_policy_status=tbd`.
- Boundary preserved: no accepted displacement-delta, reaction-delta, general
  energy, sparse-default, external validation, release-readiness, lifecycle,
  professional, or code-compliance claim was added.
- Validation evidence is in
  `_run_records/WORKING_ITEMS_RUN_2026-06-22_TP-R4-D9-DISPREACTIONOBS-001.md`.

## 2026-06-22 - TP-R4-D9-MULTISUPPORTFRICTION-001

- Added `NL-ASSEMBLED-MULTI-DOF-FRICTION-GAP-ACCEPTED-ORIGINAL`, an invented
  assembled frame fixture with simultaneous Ux friction sliding and Uy gap
  closure in the first iteration.
- Broadened the accepted multi-support validation fixture set from two
  companions to three companions: one-way/gap, lift-off/gap, and friction/gap.
- Updated the multi-support active-set-count, free-DOF force/moment, and
  free-DOF work policy records so their evidence fixture IDs name the
  three-fixture public-original set.
- Boundary preserved: no non-seed force/displacement threshold beyond the
  accepted fixture set, displacement-delta, reaction-delta, general energy,
  sparse-default, external validation, release-readiness, lifecycle,
  professional, or code-compliance claim was added.
- Validation evidence is in
  `_run_records/WORKING_ITEMS_RUN_2026-06-22_TP-R4-D9-MULTISUPPORTFRICTION-001.md`.

## 2026-06-22 - TP-R4-D9-MULTISUPPORTBREADTH-001

- Added `NL-ASSEMBLED-MULTI-DOF-GAP-LIFT-OFF-ACCEPTED-ORIGINAL`, an invented
  assembled frame fixture with simultaneous Ux lift-off release and Uy gap
  closure in the first iteration.
- Broadened the accepted multi-support validation fixture set from one
  one-way/gap companion to two companions: one-way/gap plus lift-off/gap.
- Updated the multi-support active-set-count, free-DOF force/moment, and
  free-DOF work policy records so their evidence fixture IDs name the
  two-fixture public-original set.
- Boundary preserved: no non-seed force/displacement threshold beyond the
  accepted fixture set, displacement-delta, reaction-delta, general energy,
  sparse-default, external validation, release-readiness, lifecycle,
  professional, or code-compliance claim was added.
- Validation evidence is in
  `_run_records/WORKING_ITEMS_RUN_2026-06-22_TP-R4-D9-MULTISUPPORTBREADTH-001.md`.

## 2026-06-22 - TP-R4-D9-WORKPOLICY-001

- Promoted bounded `DEC-046` final-iteration free-DOF work residual policies
  for the current public-original assembled validation seed, the current
  invented product-preview nonlinear surface, and the accepted Ux/Uy
  multi-support companion fixture.
- Added machine-readable policy evidence at
  `validation/benchmarks/nonlinear/free_dof_work_policy.dec046.json` and
  `validation/benchmarks/nonlinear/multisupport_free_dof_work_policy.dec046.json`.
- Updated nonlinear benchmark observations, hand-calculation notes,
  product-preview metadata, and regression guards so accepted work residual
  products carry `0.0 N-m` limits while general energy,
  displacement/reaction-delta, sparse-default, external validation, release,
  and broader non-seed threshold axes remain open.
- Boundary preserved: no general energy, displacement-delta, reaction-delta,
  sparse-default, external validation, release-readiness, lifecycle,
  professional, or code-compliance claim was added.
- Validation evidence is in
  `_run_records/WORKING_ITEMS_RUN_2026-06-22_TP-R4-D9-WORKPOLICY-001.md`.

## 2026-06-22 - TP-R4-D9-MULTISUPPORTOBS-001

- Added `NL-ASSEMBLED-MULTI-DOF-MULTI-SUPPORT-OBS-ORIGINAL`, an invented
  assembled frame fixture with simultaneous Ux one-way release and Uy gap
  closure in the first iteration.
- Exposed separate multi-support depth inventories and observations so the
  fixture remains outside `assembled_fixture_inventory()` and outside the
  accepted `DEC-046` current-seed threshold policies.
- Added the hand-calc/provenance note
  `validation/hand_calcs/nonlinear/assembled_multi_support_multi_dof.md` and
  focused Python guardrails for the observation-only boundary.
- Boundary preserved: no non-seed threshold promotion, R4 exit readiness,
  lifecycle, professional, or code-compliance claim was added.
- Validation evidence is in
  `_run_records/WORKING_ITEMS_RUN_2026-06-22_TP-R4-D9-MULTISUPPORTOBS-001.md`.

## 2026-06-22 - TP-R4-D9-FREEDOFRESIDPOLICY-001

- Promoted a bounded `DEC-046` free-DOF force/moment residual policy for the
  current public-original assembled validation seed:
  `DEC-046-CV-B-free-dof-force-moment-residual-validation-v1`, with limits of
  `0.0 N` and `0.0 N-m` for final-iteration residuals across the current
  one-way, gap, lift-off, and friction seed classes.
- Added machine-readable policy evidence at
  `validation/benchmarks/nonlinear/free_dof_force_moment_policy.dec046.json`
  and updated `convergence_observations.md` to cite the policy while preserving
  displacement/reaction deltas as observation-only.
- Updated product-preview residual metadata to cite
  `DEC-046-CV-B-product-preview-free-dof-force-moment-residual-v1` for the
  invented dense-loop preview surface; displacement/reaction delta rows still
  carry `threshold=TBD`.
- Boundary preserved: no non-seed, displacement-delta, reaction-delta, energy,
  sparse-default, external validation, release-readiness, lifecycle,
  professional, or code-compliance claim was added.
- Validation evidence is in
  `_run_records/WORKING_ITEMS_RUN_2026-06-22_TP-R4-D9-FREEDOFRESIDPOLICY-001.md`.

## 2026-06-21 - TP-R4-D9-PRODPOLICY-001

- Promoted the current invented product-preview nonlinear dense-loop
  active-set-count convergence surface to
  `DEC-046-CV-B-product-preview-active-set-count-v1`.
- `core/product_physics` now uses `ConvergencePolicyStatus::Accepted`,
  tolerance `0.0`, absolute floor `0.0`, and max iteration cap `4` for this
  preview active-set-count surface, and records deterministic support-class
  coverage in iteration-count metadata.
- Updated `fixtures/product_preview/invented_mechanics_result.json` and the
  product-preview service regression so the checked-in preview fixture no
  longer emits `TOLERANCE_POLICY_TBD` for the active-set-count surface.
- Boundary preserved: force/displacement threshold promotion, sparse live-path
  behavior, external validation thresholds, release thresholds, R4 exit
  readiness, lifecycle transitions, and professional/code-compliance claims
  remain out of scope.
- Validation evidence is in
  `_run_records/WORKING_ITEMS_RUN_2026-06-21_TP-R4-D9-PRODPOLICY-001.md`.

## 2026-06-21 - TP-R4-D9-GAPREFRESH-001

- Refreshed the derivative R4 readiness/gap packet after D-17 packet
  preparation and product-preview active-set policy promotion.
- The packet now records both `D-15` and `D-17` as prepared and awaiting human
  ruling, and no longer points to D-17 decision preparation as the next
  unblocked plan item.
- Boundary preserved: planning/evidence text only; no D-15/D-17 ruling, sparse
  live-path adoption, threshold promotion, R4 closure, lifecycle transition, or
  professional/code-compliance claim.
- Validation evidence is in
  `_run_records/WORKING_ITEMS_RUN_2026-06-21_TP-R4-D9-GAPREFRESH-001.md`.

## 2026-06-21 - TP-R4-D9-FORCEDISPRESID-001

- Added final-iteration force/displacement residual observations to the dense
  nonlinear integration loop: max translation/rotation deltas, max
  force/moment reaction deltas, free-DOF force/moment residuals, and the
  existing active-set changed-support count.
- Carried the observation rows into product-preview nonlinear support results
  as non-combined evidence rows with `observed_residual_only` basis text.
- Exposed `ForceDisplacementResidualObservation` and
  `assembled_force_displacement_residual_observations()` from the nonlinear
  benchmark crate, and recorded current assembled-fixture residual observations
  in the hand-calc convergence note.
- Boundary preserved: no force/displacement threshold is promoted; the governed
  convergence policy remains
  `DEC-046-CV-B-active-set-count-validation-v1` for active-set
  changed-support-count validation-seed evidence only. Product-preview,
  sparse live-path, and external validation thresholds remain `TBD`; `D-15`
  and `D-17` remain pending unless separately ruled.
- Validation evidence is in
  `_run_records/WORKING_ITEMS_RUN_2026-06-21_TP-R4-D9-FORCEDISPRESID-001.md`.

## 2026-06-21 - TP-R4-D9-EXITGAP-001

- Added derivative R4 readiness/gap packet
  `plans/VERIFICATION_2026-06-21_r4_exit_gap.md` for PRD 22.5.
- Verdict recorded: R4 is **not ready** for human exit review. Component
  provenance appears in the current rendered report path and current assembled
  nonlinear validation seeds converge under
  `DEC-046-CV-B-active-set-count-validation-v1`, but D5 spring-hanger scope
  remains gated by `D-15` and non-seed convergence-policy surfaces remain
  outside the accepted policy.
- The packet is derivative planning/evidence only; it does not close D9, close
  R4, change lifecycle state, advance the target stage, or make release,
  professional, certification, sealing, authentication, or code-compliance
  claims.
- Validation evidence is in
  `_run_records/WORKING_ITEMS_RUN_2026-06-21_TP-R4-D9-EXITGAP-001.md`.

## 2026-06-21 - TP-R4-D9-CONVPOLICY-001

- Promoted the governed DEC-046 convergence policy for the current assembled
  validation seed only: `DEC-046-CV-B-active-set-count-validation-v1`, residual
  basis active-set changed-support count, relative tolerance `0.0`, absolute
  residual floor `0.0`, and max iteration cap `4` across one-way, gap,
  lift-off, and friction classes.
- Added machine-readable policy evidence at
  `validation/benchmarks/nonlinear/convergence_policy.dec046.json` and exposed
  `governed_convergence_policy_entries()` from the nonlinear benchmark crate.
- Updated assembled fixture notes and convergence observations to cite the
  accepted policy; the six assembled validation fixtures now run with
  `ConvergencePolicyStatus::Accepted` and no longer emit `TolerancePolicyTbd`
  on the validation-seed surface.
- Boundaries preserved: product-preview convergence remains `TBD`; sparse live
  path remains gated by `D-17`; force/displacement residual thresholds,
  external validation thresholds, lifecycle transitions, release-readiness
  claims, and professional/code-compliance claims remain out of scope.
- Validation evidence is in
  `_run_records/WORKING_ITEMS_RUN_2026-06-21_TP-R4-D9-CONVPOLICY-001.md`.

## 2026-05-11 TP-RECON-01 Reconciliation

- Source bundle reviewed: TP-RECON-01 dispatch row for `DEL-09-03`, archived
  DEV-001 evidence rows, lifecycle snapshot, sealed Tranche A brief, Tranche A
  proposal and review closeout, commit `abdecbd`, and current deliverable
  records.
- Reconciled history: DEV-001 revision 0.5 Tranche A produced the nonlinear
  benchmark crate under `validation/benchmarks/nonlinear/`, the focused
  regression entry point `tests/test_nonlinear_support_regression.py`, and this
  deliverable `MEMORY.md`; archived evidence records promote that work to
  `COMMITTED` at `abdecbd` on 2026-05-04, with lifecycle preserved as
  `CHECKING`.
- Verification evidence preserved: nonlinear benchmark crate `cargo test`, the
  focused Python regression test, existing nonlinear support and diagnostics
  cargo tests, `git diff --check`, and protected-content/private-data/prohibited
  claim scans over the DEL-09-03 changed files.
- Deferred scope remains `TBD`: production tolerance policy, release thresholds,
  CI/publication policy, and external validation claims. This reconciliation
  records software verification and workflow evidence only.

## 2026-05-16 - DEV-001 downstream PKG-02 audit memory addendum

Durable context preserved after reconciliation review:
- DEV-001 package-worker audit reviewed this deliverable for downstream compatibility with the accepted PKG-02 foundation contracts.
- Local audit artifacts are `execution/PKG-09_Verification, Validation, and Quality Oracles/1_Working/DEL-09-03_Nonlinear support regression suite/_REVIEW.md` and `execution/PKG-09_Verification, Validation, and Quality Oracles/1_Working/DEL-09-03_Nonlinear support regression suite/Review_Findings.csv`.
- Package audit summary is `execution/PKG-09_Verification, Validation, and Quality Oracles/1_Working/_audit/PKG02_DOWNSTREAM_REVIEW_2026-05-16.md`; package run record is `execution/PKG-09_Verification, Validation, and Quality Oracles/1_Working/_run_records/TASK_RUN_2026-05-16_PKG09_PKG02_DOWNSTREAM_AUDIT.md`.
- This was audit evidence only. It did not change lifecycle state, authorize release, or make a professional, certification, sealing, approval, or code-compliance claim.
- The May 16 package-worker TASK run record did not fully preserve canonical per-deliverable TASK documentation context; this addendum preserves the durable deliverable-local pointer without modifying the completed run record.

## 2026-05-16 - DEV-001 PKG-02 grounded finding-resolution memory addendum

Durable context preserved after PKG-02 grounded finding resolution:
- Stage 2 technical resolution used the accepted PKG-02 contract as the governing source for this deliverable's downstream compatibility evidence.
- Original audit finding count for this deliverable: 2 (BLOCKER=1, WARNING=1). Current technical status count in the resolution matrix: TECHNICALLY_ADDRESSED_PENDING_HUMAN=2.
- Resolution evidence is indexed in `execution/_Reconciliation/Reviews/DEV001_FINDING_RESOLUTION_PKG02_GROUNDED_2026-05-16/RESOLUTION_MATRIX.csv`; validation evidence is summarized in `execution/_Reconciliation/Reviews/DEV001_FINDING_RESOLUTION_PKG02_GROUNDED_2026-05-16/VALIDATION_SUMMARY.md`.
- Local `Review_Findings.csv` entries remain subject to the human disposition gate. `HumanDisposition` stays `TBD` until review, and `Status` must not be changed to `RESOLVED` automatically.
- No lifecycle promotion, release claim, or professional/code-compliance claim is implied by the technical closeout.

## 2026-06-04 - TP-AUTHORITY-REFRESH-0_7-DAG006

- WORKING_ITEMS/TASK workforce current-authority refresh applied to active deliverable-local surfaces for `DEL-09-03`.
- Current authority basis is `execution/_Decomposition/SOFTWARE_DECOMP.md` revision `0.7` plus approved `execution/_DAG/DAG-006/` active graph authority.
- Historical run records, historical DAG row IDs, review dispositions, lifecycle `_STATUS.md`, aggregate DAG artifacts, candidate edges, repo-level governance files, schemas, code, and tools were intentionally not changed by this refresh.
- Preserved historical references remain evidence of earlier work, not current authority claims.

## 2026-06-06 - Nonlinear dimension allowlist regression

- Changed files: `validation/benchmarks/nonlinear/src/lib.rs`, `tests/test_nonlinear_support_regression.py`, this `MEMORY.md`, and `_run_records/TASK_RUN_2026-06-06_1538.md`.
- Added validation-local `force_per_length` support to the nonlinear fixture dimension allowlist and a source-level regression check that confirms the token is present only in the current `CANONICAL_DIMENSIONS` list.
- Preserved unresolved tolerance policy posture: fixture `tolerance_policy` assignments remain `None`/TBD.
- Validation in this TASK run: `cargo test --manifest-path validation/benchmarks/nonlinear/Cargo.toml` passed; `python3 -m pytest -q tests/test_nonlinear_support_regression.py` passed; `git diff --check` passed.

## 2026-06-06 - TP-FORCE-PER-LENGTH Parent Fan-In

- Parent WORKING_ITEMS fan-in for the cross-deliverable force-per-length
  alignment tranche is recorded in
  `../DEL-09-01_Mechanics benchmark suite/_run_records/PARENT_FANIN_2026-06-06_1549_TP-FORCE-PER-LENGTH.md`.
- Parent validation passed: mechanics, stress, and nonlinear benchmark crates;
  schema/adapter/nonlinear/witness/results pytest suite; focused distributed
  `N/m` scan; and `git diff --check`.
- No lifecycle, review-disposition, release, acceptance, protected-content,
  private-data, professional-approval, or code-compliance surface was changed.

## 2026-06-06 - TP-PKG09-READINESS-DEL-09-03

- Hardened the focused nonlinear readiness pytest surface for fixture-family
  coverage, one hand-calculation note per public-original fixture, provenance
  wording, explicit fixture-local unit basis, unresolved tolerance posture, and
  prohibited protected-content/professional-claim wording.
- Preserved solver behavior and validation fixture data; this tranche added
  source-level readiness checks only.
- Production tolerance policy, release thresholds, CI/publication policy,
  external validation claims, unit catalog, conversion constants, and
  professional reliance remain `TBD`.
- Validation in this TASK run: nonlinear benchmark `cargo fmt --check` passed;
  nonlinear benchmark `cargo test` passed; focused nonlinear pytest passed.

## 2026-06-06 - TP-PKG09-READINESS Parent Fan-In

- Parent WORKING_ITEMS fan-in for the PKG-09 benchmark readiness tranche is
  recorded in
  `../DEL-09-01_Mechanics benchmark suite/_run_records/PARENT_FANIN_2026-06-06_TP-PKG09-READINESS.md`.
- Parent validation passed: mechanics, stress, and nonlinear benchmark crates;
  focused nonlinear/witness pytest; DAG-006 dependency schema validation; and
  `git diff --check`.
- No lifecycle, review-disposition, release, acceptance, protected-content,
  private-data, professional-approval, or code-compliance surface was changed.

## 2026-06-21 - TP-R4-D6-FRICTIONNORMAL-001

- Added `NL-ASSEMBLED-FRICTION-DERIVED-NORMAL-ORIGINAL` to the nonlinear
  benchmark inventory with hand-calc evidence in
  `validation/hand_calcs/nonlinear/assembled_friction_derived_normal.md`.
- The fixture uses public invented data only: a friction support derives normal
  evidence from a named restrained `UY` support DOF; it does not use protected
  standards data, catalog/default normal-force values, private data, or release
  thresholds.
- Focused validation passed for the nonlinear benchmark crate and focused
  pytest regression, and the full five-surface evidence sweep passed; details
  are recorded in
  `_run_records/WORKING_ITEMS_RUN_2026-06-21_TP-R4-D6-FRICTIONNORMAL-001.md`.

## 2026-06-21 - TP-R4-D9-FRICTIONSLIDE-001 assembled sliding friction

- Added `NL-ASSEMBLED-FRICTION-SLIDE-ORIGINAL` to the nonlinear benchmark
  inventory with explicit invented normal-reaction input, final `Sliding`
  state, zero released reaction, and visible `TOLERANCE_POLICY_TBD`.
- Added `validation/hand_calcs/nonlinear/assembled_friction_sliding.md` and
  wired the focused nonlinear pytest requirement to that note.
- Product-preview evidence now includes invented support `support:NL-130-FRIC`
  and result-envelope rows for sliding state code, displacement, released
  reaction, and explicit normal reaction evidence.
- Evidence is recorded in
  `_run_records/WORKING_ITEMS_RUN_2026-06-21_TP-R4-D9-FRICTIONSLIDE-001.md`.
- Residuals remain: derived friction normal-force modeling, measured governed
  convergence values, sparse live-path adoption, broader live-solver coverage,
  the PRD section 16.2 branch-assembly benchmark, and the R4 exit evidence
  package.

## 2026-06-06 - TP-PKG09-READINESS-GATE Review

- Appended a `SELF_CHECK` readiness-gate section to `_REVIEW.md` and recorded
  local review evidence in
  `_run_records/REVIEW_RUN_2026-06-06_TP-PKG09-READINESS-GATE_DEL-09-03.md`.
- Parent review fan-in is recorded in
  `../DEL-09-01_Mechanics benchmark suite/_run_records/PARENT_REVIEW_FANIN_2026-06-06_TP-PKG09-READINESS-GATE.md`.
- Recommendation: mechanically ready for human-approved `IN_PROGRESS ->
  CHECKING`; `_STATUS.md` was not changed.
- Existing findings `PKG09-0903-PKG02-001` and `PKG09-0903-PKG02-002` remain
  technically addressed pending human disposition.

## 2026-06-17 - Lifecycle Housekeeping

- Housekeeping lifecycle reset: `_STATUS.md` current state set to `IN_PROGRESS` to reflect current code development in progress. This does not change review, issuance, release readiness, professional approval, certification, sealing, authentication, or code-compliance status.

## 2026-06-21 - TP-R4-D9-ASSEMBLEDSEED-001

- Added the first D9 assembled global-loop validation seed for nonlinear supports under `DEC-044` and `DEC-046`.
- Extended `validation/benchmarks/nonlinear/` beyond classifier-only fixtures with three two-node dense frame cases that call `core/solver/nonlinear_integration::solve_active_set_frame`: one-way deactivation, positive gap closure, and lift-off release.
- Added public-original hand-calculation notes for the assembled cases under `validation/hand_calcs/nonlinear/` and extended the focused pytest readiness checks to require those notes, source references, invented-data boundary wording, visible `TolerancePolicyTbd`, and the `DEC-046-CV-B-assembled-validation-seed-TBD` convergence-policy reference.
- D9 remains partial: friction normal-force model integration, measured class-tiered convergence values, sparse live-path adoption, broader live-solver coverage, the PRD section 16.2 branch-assembly benchmark, component provenance in the rendered report path, and the R4 exit evidence package remain open.
- Validation in this WORKING_ITEMS run: nonlinear benchmark `cargo fmt --check` passed; nonlinear benchmark `cargo test` passed; focused nonlinear pytest passed; `git diff --check` passed; the five-surface `DEC-025` evidence sweep passed and wrote `validation/evidence/sweeps/SWEEP_20260621T073226Z_4cb593a09376-dirty.json`.

## 2026-06-21 - TP-R4-D9-FRICTIONSEED-001

- Added an assembled friction sticking seed for the dense active-set loop:
  `NL-ASSEMBLED-FRICTION-STICK-ORIGINAL` uses explicit invented normal-reaction
  evidence and remains `Sticking`.
- Added `validation/hand_calcs/nonlinear/assembled_friction_sticking.md` and
  extended the focused nonlinear pytest note requirements to include it.
- Updated `core/product_physics` to surface explicit friction normal-reaction
  evidence in mechanics result envelopes as
  `nonlinear_support_friction_normal_reaction_input` while excluding that input
  evidence from load-case combination algebra.
- D6/D9 remain partial: the derived friction normal-force model, sliding
  friction assembled/product validation, measured class-tiered convergence
  values, sparse live-path adoption, broader live-solver coverage, the PRD
  section 16.2 branch-assembly benchmark, component provenance in the rendered
  report path, and the R4 exit evidence package remain open.
- Validation in this WORKING_ITEMS run: nonlinear benchmark `cargo fmt --check`
  passed; product physics `cargo fmt --check` passed; nonlinear benchmark
  `cargo test` passed; focused nonlinear pytest passed; product physics
  `cargo test` passed; `git diff --check` passed; the five-surface `DEC-025`
  evidence sweep passed and wrote
  `validation/evidence/sweeps/SWEEP_20260621T074615Z_a83ced203fac-dirty.json`.

## 2026-06-21 - TP-R4-D9-CONVOBS-001

- Added a structured convergence-observation inventory for the current
  assembled nonlinear validation fixture set:
  `assembled_convergence_observations()` in
  `validation/benchmarks/nonlinear/src/lib.rs`.
- Added `validation/hand_calcs/nonlinear/convergence_observations.md`; the
  note records observed iteration counts and final residuals for one-way
  deactivation, gap closure, lift-off release, explicit-normal friction
  sticking/sliding, and derived-normal friction sticking.
- Every observation preserves
  `DEC-046-CV-B-assembled-validation-seed-TBD` and visible
  `TolerancePolicyTbd`; governed class-tier threshold promotion remains
  unresolved.
- Focused validation passed for nonlinear benchmark rustfmt, nonlinear
  benchmark tests, focused nonlinear pytest, and `git diff --check`; the full
  five-surface evidence sweep passed and wrote
  `validation/evidence/sweeps/SWEEP_20260621T095800Z_4d3bae24de12-dirty.json`.
- Residuals remain: sparse live-path adoption (`D-17`), broader live-solver
  coverage, governed threshold promotion under `DEC-046`, and the R4 exit
  evidence package.
