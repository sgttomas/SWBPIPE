# R4 Exit-Chain Verification Packet Refresh - TP-R4-D9-EXITCHAINREFRESH-001 (2026-06-23)

**Epistemic status:** derivative verification package for human R4 exit review.
This packet refreshes
`plans/VERIFICATION_2026-06-22_r4_exit_chain.md` after the human `D-26`
ruling recorded as `DEC-053`. It assembles already-recorded implementation,
validation, ruling, and sparse default-promotion repair evidence for PRD 22.5.

This packet is not a lifecycle transition, target-stage advancement,
release-readiness claim, professional approval, certification, sealing,
authentication, or code-compliance acceptance. Only the human project authority
can approve R4 exit and authorize target-stage advancement toward R5.

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

This refresh also includes the `DEC-053` selected sparse residual repair:
`TP-R4-D7-SPARSEDEFAULTPROMOTE-001`.

## Human Rulings

### D-25 / DEC-052

Human project authority ruled on 2026-06-22:

> regarding `D-25` my ruling is `O-B` R4 can move to a final exit-chain
> verification packet.

`DEC-052` authorizes assembly of the bounded R4 evidence chain for human exit
review while keeping listed residuals explicit post-R4/R5 or non-blocking
hardening items unless a later human gate selects one as blocking.

### D-26 / DEC-053

Human project authority then ruled on `D-26`:

> `O-B` Hold at R4 pending named additional evidence or repair.

The named blocking residual was:

> Sparse default promotion, sparse timing, allocator/RSS memory, CI,
> hardware-normalized, practical-size-band, true-condition-number, and sparse
> conditioning evidence beyond the bounded generated-grid pivot-ratio proxy.

`DEC-053` records this ruling. The repair tranche
`TP-R4-D7-SPARSEDEFAULTPROMOTE-001` has landed and is included in this packet.
R4 remains current until the human authority accepts this refreshed packet.

## Supersession

`plans/VERIFICATION_2026-06-22_r4_exit_chain.md` remains a historical packet
prepared under `DEC-052`. This 2026-06-23 refresh supersedes it as the current
R4 exit-chain review artifact because it includes the `DEC-053` sparse
default-promotion repair and its local evidence sweep.

## Evidence Matrix

| R4 item | Evidence basis | Packet disposition |
|---|---|---|
| Bend objects | `TP-R4-D1-BENDVIS-001` and `TP-R4-D1-BENDSTRESS-001` landed invented bend visibility, user-entered modifier provenance, stress-recovery multiplier review rows, fixture regeneration, and report-packet evidence under `DEC-045`. | Demonstrated for the invented preview path. |
| Branch objects | `TP-R4-D2-BRANCHSTRESS-001` landed branch-object app/report provenance and side-specific user-entered multiplier evidence; `TP-R4-D9-BRANCHASSEMBLY-001` added the public-original PRD 16.2 branch-assembly mechanics benchmark. | Demonstrated for invented preview path plus one mechanics benchmark. |
| Rigid valves/flanges/reducers | `TP-R4-D3-RIGIDVIS-001` landed rigid/semi-rigid component visibility, provenance, diagnostics, native-package/report evidence, and no frame-stiffness behavior change. | Demonstrated for the invented preview path; deeper rigid macro solve behavior remains non-blocking hardening for this R4 packet. |
| Expansion joints | `TP-R4-D4-EJSTIFF-001`, `TP-R4-D4-EJMACRO-001`, and `TP-R4-D4-EJTHRUST-001` landed expansion-joint mapping, provenance, user-entered stiffness rows, diagnostics, review rows, the dedicated user-stiffness macro-element, and load-side pressure-thrust evidence under `DEC-045`. | Demonstrated for the invented preview path. |
| Spring hangers | `DEC-049` ruled the minimal dedicated user-entered spring-hanger model; `TP-R4-D5-HANGERDATA-001` landed invented variable spring and constant-effort support records, schema slots, validation diagnostics, user-entered review rows, and report/native-package/rendered-report provenance. | Demonstrated for minimal user-data path. Deeper catalog sizing, protected/default values, and deeper constant-effort solve behavior remain explicit post-R4/R5 or non-blocking hardening residuals. |
| Gaps/lift-off/friction validation | `TP-R4-D9-ASSEMBLEDSEED-001`, `TP-R4-D9-FRICTIONSEED-001`, `TP-R4-D9-FRICTIONSLIDE-001`, `TP-R4-D6-FRICTIONNORMAL-001`, `TP-R4-D9-CONVOBS-001`, `TP-R4-D9-CONVPOLICY-001`, `TP-R4-D9-FORCEDISPRESID-001`, `TP-R4-D9-FREEDOFRESIDPOLICY-001`, `TP-R4-D9-ENERGYOBS-001`, `TP-R4-D9-WORKPOLICY-001`, `TP-R4-D9-GENERALENERGYPOLICY-001`, and the accepted thirteen-fixture multi-support companion set demonstrate bounded active-set, force/moment residual, free-DOF work, general-energy, and displacement/reaction-delta policies for the current seed/product/fixture surfaces. | Bounded convergence evidence is sufficient to present for human R4 exit review. Broader non-seed, release, external, and threshold-generalization axes remain explicit residuals. |
| Product-preview nonlinear path | `TP-R4-D6-PHYSINTEG-001`, `TP-R4-D6-LIVECOVER-001`, `TP-R4-D6-LIVEBUNDLE-001`, `TP-R4-D9-PRODPOLICY-001`, `TP-R4-D9-FREEDOFRESIDPOLICY-001`, `TP-R4-D9-WORKPOLICY-001`, `TP-R4-D9-GENERALENERGYPOLICY-001`, and `TP-R4-D9-PRODDISPREACTIONPOLICY-001` show product-preview evidence for invented nonlinear supports. | Demonstrated for the invented mixed product-preview surface; not a release acceptance envelope. |
| Sparse evidence lane and sparse default promotion | `TP-R4-D7-SPARSELIVE-001`, `TP-R4-D7-SPARSEPROFILE-001`, `TP-R4-D7-NONLINEARPROFILE-001`, `TP-R4-D7-SPARSESUITABILITYOBS-001`, `TP-R4-D7-SPARSETHRESHOLDPOLICY-001`, `TP-R4-D7-SPARSESTORAGEOBS-001`, and `TP-R4-D7-SPARSECONDITIONPOLICY-001` bind sparse direct solving into the nonlinear/product evidence lane. `TP-R4-D7-SPARSEDEFAULTPROMOTE-001` then promotes sparse interactive as the default preview/render/live-model-change path, keeps dense scrutiny explicitly selectable, makes sparse-to-dense fallback diagnostic/result-basis visible, and records a 9-record observation packet with timing/RSS/hardware/practical-size-band/parity/residual/repeatability/pivot-proxy/true-condition-number fields. | The `DEC-053` named sparse R4 blocker is closed for R4 exit review by local bounded evidence. Timing/RSS/hardware/CI evidence is observational, not release-threshold or hosted-CI activation evidence. Dense remains the explicit scrutiny mode. |
| Component provenance in reports | `TP-R4-D8-COMPPROVREPORT-001` carries component provenance and missing-provenance warnings into `ReportSections` and the deterministic hash-bound rendered HTML report. | Demonstrates the second PRD 22.5 exit criterion for current invented component paths. |

## DEC-053 Sparse Residual Closure Matrix

| Named residual axis | Closure evidence in this packet | R4 disposition |
|---|---|---|
| Sparse default promotion | Solver/product/desktop paths now default to sparse interactive for preview/render/live model changes; dense scrutiny is explicit. | Closed for R4 interactive default. |
| Sparse timing | `validation/benchmarks/sparse_default_promotion_observation.dec053.json` records dense and sparse timing observations for 9 fixtures. | Closed for R4 as observation, not threshold. |
| Allocator/RSS memory | The same packet records value-storage observations per fixture and RSS/platform metadata at packet emission. | Closed for R4 as observation, not threshold. |
| CI | The local `DEC-025` evidence sweep passed cargo, pytest, desktop Vitest, desktop Playwright, and desktop production build surfaces. | Closed for R4 by local evidence coverage; no hosted CI activation claim. |
| Hardware-normalized | Observation packet records hardware/platform metadata and explicitly avoids cross-machine pass/fail thresholds. | Closed for R4 by hardware metadata; no hardware-normalized gate. |
| Practical size band | The 9-record observation set covers chain, generated-grid, and product/nonlinear proxy fixtures with named practical size bands. | Closed for R4 by bounded observation set. |
| True condition number | Each observation records a true condition number computed from the reduced dense symmetric matrix with a deterministic eigenvalue routine. | Closed for R4 bounded observation evidence. |
| Sparse conditioning beyond pivot proxy | The pivot-ratio proxy remains labeled as proxy evidence and is accompanied by true condition-number fields. | Closed for R4 by side-by-side proxy and true-condition fields. |

## Accepted Nonlinear Fixture Envelope

The accepted current envelope remains intentionally bounded:

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
presentable for human exit review. Outside that envelope, threshold promotion
remains residual work.

## Residual Disposition After Refresh

After `DEC-053` repair, sparse default promotion is no longer an open R4
blocker for this packet. The following remain visible residuals for R5 or later
hardening unless a later human gate selects one as blocking:

1. Non-seed force/displacement threshold promotion beyond the accepted
   thirteen-fixture multi-support set.
2. Broader displacement/reaction-delta thresholds beyond the accepted
   current-seed, thirteen-fixture, and product-preview surfaces.
3. Broader energy thresholds outside the accepted current-seed,
   thirteen-fixture, and product-preview surfaces.
4. Broader multi-DOF / multi-support nonlinear acceptance thresholds beyond
   the accepted thirteen-fixture set.
5. Sparse release-performance thresholds, hosted CI activation, and
   cross-machine hardware-normalized pass/fail thresholds beyond the local
   `DEC-053` observation package.
6. Deeper spring-hanger catalog sizing or constant-effort solve behavior
   beyond the minimal user-entered model ruled by `DEC-049`.
7. External validation threshold evidence.
8. Release-quality gates and R5 engineering-beta evidence.

The residuals remain visible. They are not silently closed by this packet.

## Validation Evidence

The `DEC-053` repair tranche validation is recorded in:

- `execution/PKG-04_Solver Core and Numerical Methods/1_Working/DEL-04-05_Sparse solver performance harness/_run_records/WORKING_ITEMS_RUN_2026-06-23_TP-R4-D7-SPARSEDEFAULTPROMOTE-001.md`
- `validation/evidence/sweeps/SWEEP_20260623T020002Z_3194bd29f417-dirty.json`

The local `DEC-025` sweep passed all five surfaces:

- `cargo_crate_sweep`: pass.
- `python_pytest`: pass.
- `desktop_vitest`: pass.
- `desktop_playwright_e2e`: pass.
- `desktop_production_build`: pass.

The sweep is bound to commit `3194bd29f417b8ca5489a5a524b16a460fb63260` with
the `TP-R4-D7-SPARSEDEFAULTPROMOTE-001` changed paths dirty at sweep time. The
changed paths and sweep artifact were committed afterward as
`26d2cff2f Close R4 sparse default promotion residual`.

Packet-refresh validation is recorded in the companion run record:

`execution/PKG-09_Verification, Validation, and Quality Oracles/1_Working/DEL-09-03_Nonlinear support regression suite/_run_records/WORKING_ITEMS_RUN_2026-06-23_TP-R4-D9-EXITCHAINREFRESH-001.md`.

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

The refreshed R4 evidence chain is ready to present for human R4 exit review
under `DEC-052` and `DEC-053`, with the named `D-26` sparse residual repaired
and the remaining residuals explicitly scoped to R5 or later hardening unless a
future human gate selects one as blocking.

If the human project authority accepts this refreshed packet in a separate R4
exit review ruling, the next governed action is to record that R4 exit decision
and update coordination for R5 target-stage work. If the packet is not
accepted, the next ordinary work is the human-selected residual named in that
ruling.
