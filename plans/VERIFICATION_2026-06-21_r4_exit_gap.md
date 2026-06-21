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
| Expansion joints | `TP-R4-D4-EJSTIFF-001` landed expansion-joint mapping, provenance, user-entered stiffness rows, diagnostics, and review rows under `DEC-045`. | Demonstrated for invented preview path; pressure-thrust load generation and full assembled macro-element solve remain out of scope |
| Spring hangers | `D-15_spring_hanger_scope.md` is prepared and awaiting human ruling. The current public implementation still has only the generic `spring` support path, not dedicated constant-effort / variable-rate hanger behavior. | **Not ready** |
| Gaps/lift-off/friction validation | `TP-R4-D9-ASSEMBLEDSEED-001`, `TP-R4-D9-FRICTIONSEED-001`, `TP-R4-D9-FRICTIONSLIDE-001`, `TP-R4-D6-FRICTIONNORMAL-001`, `TP-R4-D9-CONVOBS-001`, and `TP-R4-D9-CONVPOLICY-001` provide current assembled dense-loop validation seeds for one-way, gap, lift-off, and friction classes. The accepted policy is `DEC-046-CV-B-active-set-count-validation-v1`, limited to the current assembled validation seed and active-set changed-support-count residual. | Partially demonstrated; non-seed convergence thresholds remain open |
| Product-preview nonlinear path | `TP-R4-D6-PHYSINTEG-001`, `TP-R4-D6-LIVECOVER-001`, `TP-R4-D9-FRICTIONSLIDE-001`, `TP-R4-D6-FRICTIONNORMAL-001`, and `TP-R4-D6-LIVEBUNDLE-001` show dense-loop product-preview evidence for invented nonlinear supports. Product-preview diagnostics intentionally preserve `TOLERANCE_POLICY_TBD`. | Evidence exists; not release-policy complete |
| Component provenance in reports | `TP-R4-D8-COMPPROVREPORT-001` carries component provenance and missing-provenance warnings into `ReportSections` and the deterministic hash-bound rendered HTML report. | Demonstrated for current invented component paths |

## Blocking Gaps

1. **D5 spring-hanger scope remains human-gated.** `D-15` is the only current
   decision packet awaiting ruling. Until the human rules the spring-hanger
   scope and the selected behavior is implemented or explicitly deferred, R4
   cannot be treated as complete.
2. **The accepted convergence policy is intentionally narrow.**
   `DEC-046-CV-B-active-set-count-validation-v1` applies only to the current
   public-original assembled validation seed and its active-set
   changed-support-count residual. It does not govern force residuals,
   displacement residuals, energy residuals, sparse live-path behavior,
   product-preview thresholds, or external validation thresholds.
3. **Sparse live-path adoption is still gated by `D-17`.** The sparse solver
   crate exists from earlier work, but `frame_kernel` / `product_physics` still
   use the dense live path. `D-17` is not prepared yet and must be handled at
   the D7 lead-up before sparse adoption can proceed.
4. **No human R4 exit review packet is ready.** This file is a gap packet, not
   the final `VERIFICATION_<date>_r4_exit_chain.md` successor for human R4 exit
   review.

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
  invented bend, branch, rigid/semi-rigid, and expansion-joint paths;
- current assembled nonlinear validation seeds converge under the accepted
  active-set-count policy for one-way, gap, lift-off, and friction classes.

The decisive blockers are `D-15` / D5 spring-hanger scope and the intentionally
unpromoted convergence-policy surfaces outside the current assembled validation
seed. The next unblocked plan item after this gap packet is the `D-17`
decision-preparation packet for sparse live-path adoption timing, unless the
human first rules `D-15`.
