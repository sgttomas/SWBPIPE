# D-26 - R4 Exit Review and R4-to-R5 Target-Stage Advancement

**Date prepared:** 2026-06-23
**Prepared by:** WORKING_ITEMS, per the Application Integration And Issuance
Loop decision-escalation step.
**Epistemic status:** PROPOSAL (non-governing). Only the human project
authority rules. Nothing here changes lifecycle state, issues deliverables,
creates release readiness, professional approval, certification, sealing,
authentication, code-compliance acceptance, or asserts that any PRD milestone is
met until the human records the ruling.

---

## 1. Decision Statement And Scope

Decide whether to accept the prepared R4 exit-chain evidence and advance the
Working Desktop Application Standard's current target stage from **PRD R4** to
**PRD R5** in `execution/_Coordination/_COORDINATION.md`.

The R4 criterion is from `docs/PRD.md` §22.5:

- nonlinear support validation cases converge;
- component provenance appears in reports.

This decision governs ordinary tranche selection scope only. If the target
stage advances to R5, Phase E/R5 engineering-beta and release-machinery work
becomes ordinary in-stage work. This decision does not issue deliverables,
approve a release, accept professional reliance, certify code compliance,
promote any future DAG successor, resolve Phase E decisions such as `D-06`,
`D-10b`, `D-12`, `D-20`, or `D-05b`, or adopt held v0.2/R6/R7 scope under
`D-21`.

## 2. Evidence Basis

Primary verification packet:

- `plans/VERIFICATION_2026-06-23_r4_exit_chain_refresh.md`
  (`TP-R4-D9-EXITCHAINREFRESH-001`), which supersedes the historical
  `plans/VERIFICATION_2026-06-22_r4_exit_chain.md`
  (`TP-R4-D9-EXITCHAIN-001`) for the current R4 exit review.

Human exit-scope ruling:

- `D-25` is ruled by `DEC-052` as Option O-B: the current bounded
  seed/product/thirteen-fixture nonlinear evidence and current invented
  component-provenance evidence may proceed to final R4 exit-chain review,
  while the listed sparse-default, broader nonlinear-threshold,
  spring-hanger-depth, external-validation, and release-quality axes remain
  explicit post-R4/R5 or non-blocking hardening residuals unless a later human
  gate selects one as blocking.

Supporting evidence:

- D1 bend evidence: `TP-R4-D1-BENDVIS-001`,
  `TP-R4-D1-BENDSTRESS-001`.
- D2 branch evidence: `TP-R4-D2-BRANCHSTRESS-001`,
  plus `TP-R4-D9-BRANCHASSEMBLY-001`.
- D3 rigid/semi-rigid evidence: `TP-R4-D3-RIGIDVIS-001`.
- D4 expansion-joint evidence: `TP-R4-D4-EJSTIFF-001`,
  `TP-R4-D4-EJMACRO-001`, `TP-R4-D4-EJTHRUST-001`.
- D5 spring-hanger user-data evidence under `DEC-049`:
  `TP-R4-D5-HANGERDATA-001`.
- D6/D9 nonlinear seed, product, and accepted thirteen-fixture evidence:
  `TP-R4-D6-LOOPCORE-001`, `TP-R4-D6-PHYSINTEG-001`,
  `TP-R4-D6-LIVECOVER-001`, `TP-R4-D6-LIVEBUNDLE-001`,
  `TP-R4-D6-FRICTIONNORMAL-001`, `TP-R4-D9-ASSEMBLEDSEED-001`,
  `TP-R4-D9-FRICTIONSEED-001`, `TP-R4-D9-FRICTIONSLIDE-001`,
  `TP-R4-D9-CONVOBS-001`, `TP-R4-D9-CONVPOLICY-001`,
  `TP-R4-D9-FORCEDISPRESID-001`, `TP-R4-D9-PRODPOLICY-001`,
  `TP-R4-D9-FREEDOFRESIDPOLICY-001`, `TP-R4-D9-ENERGYOBS-001`,
  `TP-R4-D9-WORKPOLICY-001`, `TP-R4-D9-GENERALENERGYPOLICY-001`,
  `TP-R4-D9-DISPREACTIONOBS-001`, `TP-R4-D9-DISPREACTIONPOLICY-001`,
  `TP-R4-D9-PRODDISPREACTIONPOLICY-001`,
  `TP-R4-D9-MULTISUPPORTPOLICY-001`,
  `TP-R4-D9-MULTISUPPORTBREADTH-001`,
  `TP-R4-D9-MULTISUPPORTFRICTION-001`,
  `TP-R4-D9-MULTISUPPORT3DOF-001`,
  `TP-R4-D9-MULTISUPPORTROT-001`,
  `TP-R4-D9-MULTISUPPORTDERIVED-001`,
  `TP-R4-D9-MULTISUPPORTDERIVEDROT-001`,
  `TP-R4-D9-MULTISUPPORTCASCADE-001`,
  `TP-R4-D9-MULTISUPPORTNEGAP-001`,
  `TP-R4-D9-MULTISUPPORT4CLASS-001`,
  `TP-R4-D9-MULTISUPPORTOPPGAP-001`,
  `TP-R4-D9-MULTISUPPORTSPAN-001`, and
  `TP-R4-D9-MULTISUPPORTTWOSPANGAPS-001`.
- D7 sparse evidence lane under `DEC-050`:
  `TP-R4-D7-SPARSELIVE-001`, `TP-R4-D7-SPARSEPROFILE-001`,
  `TP-R4-D7-NONLINEARPROFILE-001`,
  `TP-R4-D7-SPARSESUITABILITYOBS-001`,
  `TP-R4-D7-SPARSETHRESHOLDPOLICY-001`,
  `TP-R4-D7-SPARSESTORAGEOBS-001`, and
  `TP-R4-D7-SPARSECONDITIONPOLICY-001`.
- D8 report-provenance evidence: `TP-R4-D8-COMPPROVREPORT-001`.
- Clean-head `DEC-025` five-surface sweep
  `validation/evidence/sweeps/SWEEP_20260623T001842Z_bf2e089aa97b.json`,
  bound to commit `bf2e089aa97bf315d4c3cd202680211ee1920abe`,
  `working_tree_dirty=false`, `overall_status=pass`.

The R4 verification packet records the evidence matrix, residual disposition,
validation evidence, and boundary review. Its verdict is that the R4 evidence
chain is ready to present for human R4 exit review under `DEC-052` Option O-B.

## 3. Known Residuals And Non-Blockers

These remain explicit and do not disappear if the stage advances:

- Formal deliverable lifecycle remains unchanged. The status helper currently
  reports `CHECKING=8`, `IN_PROGRESS=92`, `ISSUED=1`; no lifecycle states are
  changed by this decision packet.
- The `DEC-052` residual list remains visible for R5 or later hardening:
  sparse default promotion, sparse timing, allocator/RSS memory, CI,
  hardware-normalized and true-condition-number thresholds; broader nonlinear
  thresholds beyond the accepted current envelopes; deeper spring-hanger
  catalog/constant-effort solve behavior; external validation threshold
  evidence; and release-quality gates.
- R5 decisions are not resolved by R4 exit: `D-06`, `D-10b`, `D-12`, `D-20`,
  `D-05b`, and related release/packaging/legal/protected-content decisions
  remain at their Phase E or R5 lead-ups.
- Held `D-21` remains held. Accepting R4 exit does not adopt v0.2/R6/R7 scope,
  live embedded-agent binding, or external SDK/harness consumption.
- No protected standards content, private project data, hidden support
  defaults, network/telemetry feature, release-readiness claim, professional
  approval, certification, sealing, authentication, or code-compliance claim is
  created by this decision.

## 4. Options

| ID | Option | Consequence |
|---|---|---|
| **O-A** | **Accept R4 exit evidence and advance the current target stage to R5.** | Phase E/R5 becomes ordinary in-stage work under the existing boundary prohibitions. R5 release, packaging, protected-content, external-reproducibility, and final PRD-completeness decisions remain human-gated. |
| **O-B** | **Hold at R4 pending additional evidence or repair.** | No R5 target-stage advancement occurs. The human must name the missing R4 evidence, blocker, or replacement criterion so agents can prepare the next bounded R4 tranche. |
| **O-C** | **Accept the R4 evidence but keep the current target stage at R4 for residual hardening.** | The R4 product criterion is accepted, but ordinary work remains in R4. This avoids stage movement but risks substituting unbounded hardening for the planned R5 dependency spine unless the human names the hardening scope. |

## 5. Human Ruling And Disposition

**Ruling recorded:** 2026-06-23, `DEC-053`.

The human project authority selected **O-B**:

> Hold at R4 pending named additional evidence or repair.

The named residual is:

> Sparse default promotion, sparse timing, allocator/RSS memory, CI,
> hardware-normalized, practical-size-band, true-condition-number, and sparse
> conditioning evidence beyond the bounded generated-grid pivot-ratio proxy.

Disposition:

- R4 remains the current target stage. No R5 advancement is authorized by this
  packet.
- The selected repair tranche is `TP-R4-D7-SPARSEDEFAULTPROMOTE-001`.
- Sparse interactive becomes the default preview/render/live-model-change path;
  dense scrutiny remains explicitly selectable for detailed review.
- Sparse-to-dense fallback must be diagnostic/result-basis visible and may not
  silently present dense results as sparse.
- The R4 residual is closed by a bounded 9-record local observation packet with
  practical size bands, dense/sparse timing observations, value-storage and RSS
  observations, hardware metadata, dense/sparse parity, residual, repeat
  determinism, pivot-ratio proxy, and true condition number computed from the
  reduced dense symmetric matrix.
- Timing/RSS/hardware/CI evidence is observational and local-DEC-025-bound; it
  is not a release threshold, hosted-CI activation, hardware-normalized pass/fail
  gate, professional approval, certification, sealing, authentication, or
  code-compliance acceptance.
- A refreshed final R4 exit-chain verification packet has been prepared as
  `plans/VERIFICATION_2026-06-23_r4_exit_chain_refresh.md` and remains the
  required acceptance surface before any future R4-to-R5 ruling.

## 6. Advisory Recommended Disposition (PROPOSAL)

**Superseded by the 2026-06-23 O-B ruling above.** Historical proposal text
retained for traceability.

Recommend **O-A**, if the human project authority accepts the refreshed
`VERIFICATION_2026-06-23_r4_exit_chain_refresh.md` evidence packet.

Rationale: PRD §22.5 is narrow, and `DEC-052` already ruled the current bounded
evidence envelope sufficient to proceed to final R4 exit-chain review while
making the broader residuals explicit non-R4 blockers unless later selected by
a human gate. Holding at R4 now would require a newly named blocker. Advancing
the target stage to R5 does not relax any boundary and does not create release
or professional claims.

## 7. Proposed Coordination Edit (Applied Only On Acceptance)

If O-A is accepted, replace the current target-stage paragraph in
`execution/_Coordination/_COORDINATION.md` with text equivalent to:

> **Current target stage - PRD R5 exit criterion (advanced 2026-06-23 by
> `DEC-0XX`).** Phase D/R4 evidence was reviewed per
> `plans/VERIFICATION_2026-06-23_r4_exit_chain_refresh.md`, with `D-25` /
> `DEC-052` accepting the bounded R4 evidence envelope for final exit review
> and `D-26` / `DEC-053` selecting and then closing the named sparse residual
> repair for R4 review. The human accepted the R4 exit review through `D-26`.
> The ordinary in-stage program is now
> Phase E: engineering beta and release machinery. R5 exit criteria are
> PRD §22.6: external engineers can reproduce validation examples, and the
> public repository contains no known protected standards data. Boundary
> prohibitions continue unchanged: no protected standards content,
> private-data defaults, unapproved release-readiness claim, professional
> approval, certification, sealing, authentication, or code-compliance claim.

The completion plan and `NEXT_INSTANCE_PROMPT.md` should then select Phase E/R5
work in order, with `D-20` prepared at the E7 lead-up and other R5 decisions
prepared at their named gates.

## 8. Ruling Mechanism

Per existing practice, the human project authority selects an option or rules
directly. The ruling is appended to
`execution/_Decomposition/SOFTWARE_DECOMP.md` §12 as the next `DEC` entry
citing this packet. If the ruling advances to R5, the coordination and plan
surfaces named above are updated in a bounded follow-up. This register row then
moves from `AWAITING_RULING` to `RULED` with the decision pointer.
