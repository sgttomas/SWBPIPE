# R4 Exit-Chain Verification Packet - TP-R4-D9-EXITCHAIN-001 (2026-06-22)

**Epistemic status:** derivative verification package for human R4 exit review.
This packet assembles already-recorded implementation, validation, and human
ruling evidence for PRD 22.5 under `DEC-052` / `D-25` Option O-B. It is not a
lifecycle transition, target-stage advancement, release-readiness claim,
professional approval, certification, sealing, authentication, or
code-compliance acceptance. Only the human project authority can approve R4
exit and advance the current target stage toward R5.

## Scope

R4 criterion from `docs/PRD.md` 22.5:

- Nonlinear support validation cases converge.
- Component provenance appears in reports.

R4 deliverable families named by the same section:

- bend objects;
- branch objects;
- rigid valves/flanges/reducers;
- expansion joints;
- spring hangers;
- gaps/lift-off/friction.

`DEC-052` rules that the current bounded seed/product/thirteen-fixture
nonlinear evidence and current invented component-provenance evidence may be
assembled into this final exit-chain verification packet. The residuals
formerly listed as D-25 exit-scope candidates remain explicit post-R4/R5 or
non-blocking hardening items unless a later human gate selects one as blocking.

## Human R4 Exit-Scope Ruling

Human project authority ruled on 2026-06-22:

> regarding `D-25` my ruling is `O-B` R4 can move to a final exit-chain
> verification packet.

The ruling is recorded as `DEC-052` in
`execution/_Decomposition/SOFTWARE_DECOMP.md` section 12 and updates
`execution/_Coordination/_DECISIONS/_REGISTER.md` row `D-25` to `RULED`.

This ruling authorizes the packet preparation only. It does not approve R4
exit, advance the target stage to R5, issue deliverables, or create any
release, professional, certification, sealing, authentication, or code
compliance status.

## Evidence Matrix

| R4 item | Evidence basis | Packet disposition under `DEC-052` |
|---|---|---|
| Bend objects | `TP-R4-D1-BENDVIS-001` and `TP-R4-D1-BENDSTRESS-001` landed invented bend visibility, user-entered modifier provenance, stress-recovery multiplier review rows, fixture regeneration, and report-packet evidence under `DEC-045`. | Demonstrated for the invented preview path. |
| Branch objects | `TP-R4-D2-BRANCHSTRESS-001` landed branch-object app/report provenance and side-specific user-entered multiplier evidence; `TP-R4-D9-BRANCHASSEMBLY-001` added the public-original PRD 16.2 branch-assembly mechanics benchmark. | Demonstrated for invented preview path plus one mechanics benchmark. |
| Rigid valves/flanges/reducers | `TP-R4-D3-RIGIDVIS-001` landed rigid/semi-rigid component visibility, provenance, diagnostics, native-package/report evidence, and no frame-stiffness behavior change. | Demonstrated for the invented preview path; deeper rigid macro solve behavior remains non-blocking hardening for this R4 packet. |
| Expansion joints | `TP-R4-D4-EJSTIFF-001`, `TP-R4-D4-EJMACRO-001`, and `TP-R4-D4-EJTHRUST-001` landed expansion-joint mapping, provenance, user-entered stiffness rows, diagnostics, review rows, the dedicated user-stiffness macro-element, and load-side pressure-thrust evidence under `DEC-045`. | Demonstrated for the invented preview path. |
| Spring hangers | `DEC-049` ruled the minimal dedicated user-entered spring-hanger model; `TP-R4-D5-HANGERDATA-001` landed invented variable spring and constant-effort support records, schema slots, validation diagnostics, user-entered review rows, and report/native-package/rendered-report provenance. | Demonstrated for minimal user-data path. Deeper catalog sizing, protected/default values, and deeper constant-effort solve behavior are explicit post-R4/R5 or non-blocking hardening residuals under `DEC-052`. |
| Gaps/lift-off/friction validation | `TP-R4-D9-ASSEMBLEDSEED-001`, `TP-R4-D9-FRICTIONSEED-001`, `TP-R4-D9-FRICTIONSLIDE-001`, `TP-R4-D6-FRICTIONNORMAL-001`, `TP-R4-D9-CONVOBS-001`, `TP-R4-D9-CONVPOLICY-001`, `TP-R4-D9-FORCEDISPRESID-001`, `TP-R4-D9-FREEDOFRESIDPOLICY-001`, `TP-R4-D9-ENERGYOBS-001`, `TP-R4-D9-WORKPOLICY-001`, `TP-R4-D9-GENERALENERGYPOLICY-001`, and the accepted thirteen-fixture multi-support companion set demonstrate bounded active-set, force/moment residual, free-DOF work, general-energy, and displacement/reaction-delta policies for the current seed/product/fixture surfaces. | Bounded convergence evidence is sufficient to present for human R4 exit review under `DEC-052`. Broader non-seed, release, sparse-default, and external threshold axes remain explicit residuals. |
| Product-preview nonlinear path | `TP-R4-D6-PHYSINTEG-001`, `TP-R4-D6-LIVECOVER-001`, `TP-R4-D6-LIVEBUNDLE-001`, `TP-R4-D9-PRODPOLICY-001`, `TP-R4-D9-FREEDOFRESIDPOLICY-001`, `TP-R4-D9-WORKPOLICY-001`, `TP-R4-D9-GENERALENERGYPOLICY-001`, and `TP-R4-D9-PRODDISPREACTIONPOLICY-001` show dense-loop product-preview evidence for invented nonlinear supports. | Demonstrated for the invented mixed product-preview surface; not a release acceptance envelope. |
| Sparse evidence lane | `TP-R4-D7-SPARSELIVE-001`, `TP-R4-D7-SPARSEPROFILE-001`, `TP-R4-D7-NONLINEARPROFILE-001`, `TP-R4-D7-SPARSESUITABILITYOBS-001`, `TP-R4-D7-SPARSETHRESHOLDPOLICY-001`, `TP-R4-D7-SPARSESTORAGEOBS-001`, and `TP-R4-D7-SPARSECONDITIONPOLICY-001` bind sparse direct solving into the nonlinear/product evidence lane, add direct reduced profile-entry evidence, and promote bounded generated-grid sparse suitability, storage-footprint, and pivot-ratio conditioning policy evidence. Dense remains default. | Demonstrated as R4 sparse evidence lane. Default sparse promotion, timing, allocator/RSS memory, CI, hardware-normalized, and true-condition-number thresholds are explicit post-R4/R5 or non-blocking hardening residuals under `DEC-052`. |
| Component provenance in reports | `TP-R4-D8-COMPPROVREPORT-001` carries component provenance and missing-provenance warnings into `ReportSections` and the deterministic hash-bound rendered HTML report. | Demonstrates the second PRD 22.5 exit criterion for current invented component paths. |

## Accepted Nonlinear Fixture Envelope

The accepted current envelope is intentionally bounded:

- current assembled validation seed policies for one-way, gap, lift-off, and
  friction classes;
- current invented product-preview nonlinear policies for active-set count,
  free-DOF force/moment residuals, free-DOF work residuals, general-energy
  residuals, and displacement/reaction-delta rows;
- thirteen accepted public-original non-seed multi-support companion fixtures:
  one-way/gap, lift-off/gap, friction/gap, three-support/three-translation,
  mixed translation/rotation, derived-normal friction/gap, derived-normal
  friction/rotational lift-off, sequential gap/lift-off cascade,
  negative-direction gap/one-way, four-class one-way/gap/friction/lift-off,
  gap-only opposing-direction, multi-node/two-span, and two-span opposing-gap;
- `TP-R4-D9-MULTISUPPORTOBS-001` remains observation-only and outside the
  accepted policy envelope.

Within that envelope, the R4 nonlinear support validation evidence is
presentable for human exit review under `DEC-052`. Outside that envelope,
threshold promotion remains residual work.

## Residual Disposition

Under `DEC-052` Option O-B, these items are not blockers to preparing this
R4 exit-chain packet:

1. Non-seed force/displacement threshold promotion beyond the accepted
   thirteen-fixture multi-support set.
2. Broader displacement/reaction-delta thresholds beyond the accepted
   current-seed, thirteen-fixture, and product-preview surfaces.
3. Broader energy thresholds outside the accepted current-seed,
   thirteen-fixture, and product-preview surfaces.
4. Broader multi-DOF / multi-support nonlinear acceptance thresholds beyond
   the accepted thirteen-fixture set.
5. Sparse default promotion, sparse timing, allocator/RSS memory, CI,
   hardware-normalized, practical-size-band, true-condition-number, and sparse
   conditioning evidence beyond the bounded generated-grid pivot-ratio proxy.
6. Deeper spring-hanger catalog sizing or constant-effort solve behavior
   beyond the minimal user-entered model ruled by `DEC-049`.
7. External validation threshold evidence.
8. Release-quality gates and R5 engineering-beta evidence.

The residuals remain visible for R5 or later hardening selection. They are not
silently closed by this packet.

## Validation Evidence

Packet-preparation validation is recorded in the companion run record
`execution/PKG-09_Verification, Validation, and Quality Oracles/1_Working/DEL-09-03_Nonlinear support regression suite/_run_records/WORKING_ITEMS_RUN_2026-06-22_TP-R4-D9-EXITCHAIN-001.md`:

- `git diff --check`: passed.
- `python3 -m pytest -q tests/test_nonlinear_support_regression.py`: passed,
  8 tests.

Clean-head `DEC-025` evidence sweep status: `TO_BE_RECORDED_AFTER_PACKET_COMMIT`.

## Boundary Review

- No protected standards text, tables, figures, copied code formulas, material
  allowables, SIF/flexibility tables, protected dimensional tables, proprietary
  vendor data, private project data, network path, telemetry path, or hidden
  support default is introduced by this packet.
- All cited examples and fixtures are invented, public-original, or
  repository-local validation evidence already recorded by prior tranches.
- This packet does not change lifecycle status, issue deliverables, advance the
  current target stage, make a release-readiness claim, or assert professional
  approval, certification, sealing, authentication, or code compliance.

## Verdict

The R4 evidence chain is ready to present for human R4 exit review under
`DEC-052` Option O-B after closeout validation is recorded.

If the human project authority accepts this packet in a separate R4 exit review,
the next governed action is to record that R4 exit decision and update
coordination for R5 target-stage work. If the packet is not accepted, the next
ordinary work is a human-selected residual from the list above.
