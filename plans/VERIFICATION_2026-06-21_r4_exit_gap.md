# R4 Exit Readiness Gap Packet - TP-R4-D9-EXITGAP-001 (2026-06-21)

**Epistemic status:** derivative readiness/gap package for Phase D/R4 planning.
This packet assembles already-recorded implementation and validation evidence
against the PRD 22.5 R4 criterion, then records why the project is **not yet
ready** for a human R4 exit review. It is not a lifecycle transition,
release-readiness claim, professional approval, certification, sealing,
authentication, or code-compliance acceptance.

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

## Evidence Matrix

| R4 item | Current evidence | Readiness |
|---|---|---|
| Bend objects | `TP-R4-D1-BENDVIS-001` and `TP-R4-D1-BENDSTRESS-001` landed invented bend visibility, user-entered modifier provenance, stress-recovery multiplier review rows, fixture regeneration, and report-packet evidence under `DEC-045`. | Demonstrated for invented preview path |
| Branch objects | `TP-R4-D2-BRANCHSTRESS-001` landed branch-object app/report provenance and side-specific user-entered multiplier evidence; `TP-R4-D9-BRANCHASSEMBLY-001` added the public-original PRD 16.2 branch-assembly mechanics benchmark. | Demonstrated for invented preview path plus one mechanics benchmark |
| Rigid valves/flanges/reducers | `TP-R4-D3-RIGIDVIS-001` landed rigid/semi-rigid component visibility, provenance, diagnostics, native-package/report evidence, and no frame-stiffness behavior change. | Demonstrated for invented preview path; full rigid macro-element solve remains out of scope |
| Expansion joints | `TP-R4-D4-EJSTIFF-001`, `TP-R4-D4-EJMACRO-001`, and `TP-R4-D4-EJTHRUST-001` landed expansion-joint mapping, provenance, user-entered stiffness rows, diagnostics, review rows, the dedicated user-stiffness macro-element, and load-side pressure-thrust evidence under `DEC-045`. | Demonstrated for invented preview path |
| Spring hangers | `D-15_spring_hanger_scope.md` is ruled by `DEC-049` Option B, and `TP-R4-D5-HANGERDATA-001` landed the minimal dedicated user-entered spring-hanger model for invented variable spring and constant-effort supports, including schema slots, validation diagnostics, user-entered review rows, and report/native-package/rendered-report provenance. | Demonstrated for minimal user-data path; catalog sizing and deeper constant-effort solve behavior remain out of scope |
| Gaps/lift-off/friction validation | `TP-R4-D9-ASSEMBLEDSEED-001`, `TP-R4-D9-FRICTIONSEED-001`, `TP-R4-D9-FRICTIONSLIDE-001`, `TP-R4-D6-FRICTIONNORMAL-001`, `TP-R4-D9-CONVOBS-001`, `TP-R4-D9-CONVPOLICY-001`, `TP-R4-D9-FREEDOFRESIDPOLICY-001`, `TP-R4-D9-ENERGYOBS-001`, `TP-R4-D9-MULTISUPPORTPOLICY-001`, `TP-R4-D9-WORKPOLICY-001`, `TP-R4-D9-MULTISUPPORTBREADTH-001`, `TP-R4-D9-MULTISUPPORTFRICTION-001`, `TP-R4-D9-MULTISUPPORT3DOF-001`, `TP-R4-D9-MULTISUPPORTROT-001`, `TP-R4-D9-MULTISUPPORTDERIVED-001`, `TP-R4-D9-MULTISUPPORTDERIVEDROT-001`, and `TP-R4-D9-MULTISUPPORTCASCADE-001` provide current assembled dense-loop validation seeds for one-way, gap, lift-off, and friction classes, accepted final-iteration free-DOF work residual products for the current seed, and eight accepted non-seed multi-support companion fixtures: one-way/gap, lift-off/gap, friction/gap, three-support/three-translation, mixed translation/rotation, derived-normal friction/gap, derived-normal friction/rotational lift-off, and sequential gap/lift-off cascade. All eight companions carry active-set, free-DOF force/moment, and free-DOF work policies. `TP-R4-D9-MULTISUPPORTOBS-001` remains an observation-only Ux/Uy multi-support assembled depth fixture under `TolerancePolicyTbd`. Accepted policies are limited to the current assembled validation seed plus the eight-fixture multi-support companion set; general energy thresholds remain `TBD`. | Partially demonstrated; broader non-seed convergence thresholds remain open |
| Product-preview nonlinear path | `TP-R4-D6-PHYSINTEG-001`, `TP-R4-D6-LIVECOVER-001`, `TP-R4-D9-FRICTIONSLIDE-001`, `TP-R4-D6-FRICTIONNORMAL-001`, `TP-R4-D6-LIVEBUNDLE-001`, `TP-R4-D9-PRODPOLICY-001`, `TP-R4-D9-FREEDOFRESIDPOLICY-001`, `TP-R4-D9-ENERGYOBS-001`, `TP-R4-D9-WORKPOLICY-001`, and `TP-R4-D9-PRODDISPREACTIONPOLICY-001` show dense-loop product-preview evidence for invented nonlinear supports. The active-set-count preview policy is accepted as `DEC-046-CV-B-product-preview-active-set-count-v1`; the free-DOF force/moment residual policy is accepted as `DEC-046-CV-B-product-preview-free-dof-force-moment-residual-v1`; the free-DOF work residual policy is accepted as `DEC-046-CV-B-product-preview-free-dof-work-residual-v1`; the emitted displacement/reaction delta row policy is accepted as `DEC-046-CV-B-product-preview-displacement-reaction-delta-threshold-v1` for the invented mixed preview surface only. General energy, release, external, and broader displacement/reaction-delta threshold axes remain `TBD`. | Evidence exists; not release-policy complete |
| Sparse evidence lane | `TP-R4-D7-SPARSELIVE-001` binds `core/solver/sparse_direct` into the assembled nonlinear integration and product-preview reduced solve paths as `DEC-050` evidence. `TP-R4-D7-SPARSEPROFILE-001` and `TP-R4-D7-NONLINEARPROFILE-001` add direct reduced profile-entry sparse evidence for product-preview and nonlinear/core paths. `TP-R4-D7-SPARSESUITABILITYOBS-001` records generated-grid sparse suitability observations in the performance harness, and `TP-R4-D7-SPARSETHRESHOLDPOLICY-001` promotes bounded generated-grid thresholds for dense-vs-sparse relative delta, sparse residual, repeatability, and nonpositive pivots. Dense remains default; default sparse promotion and timing/memory/conditioning/CI/hardware-normalized thresholds remain follow-on work. | Demonstrated as evidence lane plus bounded generated-grid threshold policy; not default sparse promotion |
| Component provenance in reports | `TP-R4-D8-COMPPROVREPORT-001` carries component provenance and missing-provenance warnings into `ReportSections` and the deterministic hash-bound rendered HTML report. | Demonstrated for current invented component paths |

## Blocking Gaps

1. **The accepted convergence policies are intentionally narrow.**
   `DEC-046-CV-B-active-set-count-validation-v1` and
   `DEC-046-CV-B-free-dof-force-moment-residual-validation-v1` apply only to
   the current public-original assembled validation seed; the multi-support
   `DEC-046` policies apply only to the accepted public-original companion
   fixture set.
   Product-preview policies apply only to the current invented preview
   dense-loop surface. The accepted free-DOF work residual policies are bounded
   to the current validation seed, the accepted fixture set, and the invented
   product-preview surface. They do not govern broader non-seed
   force/displacement thresholds, general energy thresholds,
   displacement-delta thresholds,
   reaction-delta thresholds, sparse default behavior, release thresholds, or
   external validation thresholds.
2. **The current assembled nonlinear fixtures are validation seeds plus eight
   accepted multi-support companions, not PRD-depth acceptance coverage.** The
   accepted-policy seed cases exercise small invented systems and active-set
   state transitions. `TP-R4-D9-MULTISUPPORTPOLICY-001` adds one accepted
   multi-DOF / multi-support companion fixture, and
   `TP-R4-D9-MULTISUPPORTBREADTH-001` adds a second accepted lift-off/gap
   companion, `TP-R4-D9-MULTISUPPORTFRICTION-001` adds a third accepted
   friction/gap companion, and `TP-R4-D9-MULTISUPPORT3DOF-001` adds a fourth
   accepted three-support / three-translation companion,
   `TP-R4-D9-MULTISUPPORTROT-001` adds a fifth accepted mixed
   translation/rotation companion, `TP-R4-D9-MULTISUPPORTDERIVED-001`
   adds a sixth accepted derived-normal friction/gap companion, and
   `TP-R4-D9-MULTISUPPORTDERIVEDROT-001` adds a seventh accepted
   derived-normal friction/rotational lift-off companion, and
   `TP-R4-D9-MULTISUPPORTCASCADE-001` adds an eighth accepted sequential
   gap/lift-off cascade companion;
   `TP-R4-D9-MULTISUPPORTOBS-001` remains
   observation-only. Together they still do not demonstrate broad
   non-seed force, displacement, or general energy residual acceptance
   criteria.
3. **Sparse default promotion remains follow-on under `DEC-050`.**
   `TP-R4-D7-SPARSELIVE-001` closes the R4 sparse evidence-lane adoption item:
   the sparse skyline solver observes dense-reduced live systems while dense
   remains default. Direct product/nonlinear profile-entry assembly and
   generated-grid sparse suitability observations now exist. Bounded
   generated-grid sparse suitability thresholds are accepted, but default
   promotion, timing/memory/conditioning/CI thresholds, and hardware-normalized
   scale-benefit methodology are not closed by this packet.
4. **Deeper spring-hanger behavior is not closed by D5.**
   `TP-R4-D5-HANGERDATA-001` intentionally lands user-entered variable spring
   and constant-effort support data/review evidence only. It does not authorize
   catalog sizing, protected/default values, or full constant-effort solve
   behavior.
5. **No human R4 exit review packet is ready.** This file is a gap packet, not
   the final `VERIFICATION_<date>_r4_exit_chain.md` successor for human R4 exit
   review.

## External Review Caveats

A 2026-06-21 adversarial review of the Phase D evidence upheld this packet's
`not ready` verdict and added these planning caveats:

- do not read Phase D green tests as whole-plan green;
- D6/D9 convergence acceptance evidence is still narrow even after
  active-set-count, free-DOF force/moment residual, free-DOF work residual, and
  accepted multi-support fixture-set policy promotion; the multi-support depth
  fixture remains observation-only and general energy thresholds remain open;
- the old D4 caveat about a missing solver-consumed EJ stiffness macro-element
  is superseded by `TP-R4-D4-EJMACRO-001`; pressure-thrust load generation is
  superseded by `TP-R4-D4-EJTHRUST-001`;
- D7 computes sparse evidence after dense reduction/assembly and is not a
  default or scale path;
- D8 provenance reaches the hash-bound report path, but a cross-layer
  TypeScript-to-Rust provenance test remains useful;
- Phase E/R5, B-tail breadth, and R3 exit usability substance were outside that
  audit's verified scope.

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

R4 is **not ready** for human exit review. The strongest current evidence is:

- component provenance appears in the rendered report path for the current
  invented bend, branch, rigid/semi-rigid, expansion-joint, and spring-hanger
  user-data paths;
- current assembled nonlinear validation seeds converge under the accepted
  active-set-count policy for one-way, gap, lift-off, and friction classes;
- current assembled nonlinear validation seeds and invented product-preview
  rows carry accepted free-DOF force/moment and free-DOF work residual policies
  for the measured zero residual surface.

The decisive blockers are the intentionally unpromoted convergence-policy
surfaces outside the current assembled validation seed, accepted
multi-support eight-fixture set, and invented product-preview surface, broader
multi-DOF / multi-support nonlinear acceptance thresholds beyond that fixture
set, default sparse promotion plus sparse timing/memory/conditioning/CI and
hardware-normalized thresholds if required by the R4 exit review, and deeper
spring-hanger behavior if the R4 exit review requires more than the minimal
user-data path. The next ordinary Phase D dependency-spine item is remaining D9
threshold/acceptance evidence after the landed multi-support and free-DOF work
policy slices.
