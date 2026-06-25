# D-27 - R4 Exit Clearance and R4-to-R5 Target-Stage Advancement

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

Decide whether to accept the refreshed R4 exit-chain evidence after the
`DEC-053` sparse residual repair and advance the Working Desktop Application
Standard's current target stage from **PRD R4** to **PRD R5** in
`execution/_Coordination/_COORDINATION.md`.

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

Human rulings already recorded:

- `D-25` is ruled by `DEC-052` as Option O-B: the current bounded
  seed/product/thirteen-fixture nonlinear evidence and current invented
  component-provenance evidence may proceed to final R4 exit-chain review,
  while the listed residuals remain explicit post-R4/R5 or non-blocking
  hardening residuals unless a later human gate selects one as blocking.
- `D-26` is ruled by `DEC-053` as Option O-B: R4 remained current pending the
  named sparse default-promotion residual evidence/repair.

The `DEC-053` repair tranche `TP-R4-D7-SPARSEDEFAULTPROMOTE-001` has landed and
is included in the refreshed packet. Sparse interactive is now the default
preview/render/live-model-change path, dense scrutiny remains explicitly
selectable, sparse-to-dense fallback is diagnostic/result-basis visible, and
the local observation packet records practical size bands, dense/sparse timing,
RSS/value-storage observations, hardware metadata, parity, residual,
repeatability, pivot-ratio proxy, and true condition number. These are R4 local
evidence observations, not release thresholds, hosted-CI activation, or
hardware-normalized pass/fail gates.

Terminal validation evidence:

- Clean-head `DEC-025` five-surface sweep
  `validation/evidence/sweeps/SWEEP_20260623T051552Z_16cca07f3b64.json`,
  bound to commit `16cca07f3b644a6a4f5291a70bf44bc1773231b7`,
  `working_tree_dirty=false`, `overall_status=pass`.

The clean-head sweep passed all five surfaces:

- `cargo_crate_sweep`: pass.
- `python_pytest`: pass.
- `desktop_vitest`: pass.
- `desktop_playwright_e2e`: pass.
- `desktop_production_build`: pass.

## 3. Bounded R4 Reading And Residuals

The packet's R4 exit reading is bounded to PRD §22.5. It does not claim the full
PRD §16.2 benchmark list or PRD §16.5 validation manual is complete.

The current nonlinear evidence is presentable as active-set self-consistency
and governed convergence evidence for the current assembled seed, invented
product-preview surface, and accepted thirteen-fixture companion set. The PRD
§16.2 named linear-spring, gapped-support, and friction-support benchmark/manual
coverage remains explicit validation-manual residual work unless the human
selects it as an R4 blocker.

These residuals remain visible after any R5 target-stage advancement:

- broader non-seed force/displacement, displacement/reaction-delta, energy, and
  multi-DOF / multi-support nonlinear threshold promotion beyond the current
  accepted envelopes;
- sparse release-performance thresholds, hosted CI activation, and cross-machine
  hardware-normalized pass/fail thresholds beyond the local `DEC-053`
  observation package;
- deeper spring-hanger catalog sizing or constant-effort solve behavior beyond
  the minimal user-entered model ruled by `DEC-049`;
- PRD §16.2 named benchmark/manual coverage outside the bounded R4 exit reading;
- external validation threshold evidence;
- release-quality gates and R5 engineering-beta evidence.

R5 decisions are not resolved by R4 exit: `D-06`, `D-10b`, `D-12`, `D-20`,
`D-05b`, and related release/packaging/legal/protected-content decisions remain
at their Phase E or R5 lead-ups. Held `D-21` remains held. Accepting R4 exit
does not adopt v0.2/R6/R7 scope, live embedded-agent binding, or external
SDK/harness consumption.

## 4. Options

| ID | Option | Consequence |
|---|---|---|
| **O-A** | **Accept the refreshed R4 exit evidence and advance the current target stage to R5.** | Phase E/R5 becomes ordinary in-stage work under the existing boundary prohibitions. R5 release, packaging, protected-content, external-reproducibility, and final PRD-completeness decisions remain human-gated. |
| **O-B** | **Hold at R4 pending one hand-calc-referenced nonlinear support benchmark.** | No R5 target-stage advancement occurs. Agents prepare the named benchmark/manual tranche before returning to exit review. |
| **O-C** | **Accept the R4 evidence but keep the current target stage at R4 for named hardening.** | The R4 product criterion is accepted, but ordinary work remains in R4 for the hardening scope the human names. |

## 5. Recommended Disposition (PROPOSAL)

Recommend **O-A**, if the human project authority accepts the bounded PRD §22.5
reading and the explicit PRD §16.2/§16.5 residual disposition in the refreshed
verification packet.

Rationale: `DEC-052` already ruled the bounded seed/product/thirteen-fixture
nonlinear evidence and current invented component-provenance evidence sufficient
to proceed to final R4 exit review; `DEC-053` selected the sparse residual as
the named R4 hold; the selected repair is now included in the refreshed packet
and the packet has a clean-head `DEC-025` sweep bound to the current committed
head. Advancing the target stage to R5 does not relax any boundary and does not
create release-readiness, professional, certification, sealing, authentication,
or code-compliance claims.

## 6. Human Ruling And Disposition

**Ruling recorded:** 2026-06-23, `DEC-054`.

The human project authority accepted `D-27` as a conditional R4 gate:

> For D-27 I accept this as a conditional R4 gate; §16.2/§16.5 require a
> complete benchmark/manual evidence system. Sufficient to proceed and the
> remaining evidence will be gathered once the agent harness is active and can
> participate in that activity too.

Disposition:

- The refreshed R4 exit-chain packet is accepted as sufficient to proceed from
  PRD R4 to PRD R5.
- PRD §16.2 / §16.5 are not closed by this ruling. A complete benchmark/manual
  evidence system remains required and is carried forward as explicit residual
  work.
- Future agent-harness participation may help gather that remaining evidence
  after the harness is active, but this ruling does not itself adopt `D-21`,
  v0.2/R6/R7 scope, live embedded-agent binding, or app-dev package consumption.
- No lifecycle issuance, release-readiness, professional approval,
  certification, sealing, authentication, or code-compliance acceptance is
  created.

## 7. Coordination Edit Applied After DEC-054

After `DEC-054`, the current target-stage paragraph in
`execution/_Coordination/_COORDINATION.md` was updated with text equivalent to:

> **Current target stage - PRD R5 exit criterion (advanced 2026-06-23 by
> `DEC-054`).** Phase D/R4 evidence was reviewed per
> `plans/VERIFICATION_2026-06-23_r4_exit_chain_refresh.md`, with `D-25` /
> `DEC-052` accepting the bounded R4 evidence envelope for final exit review,
> `D-26` / `DEC-053` selecting the named sparse residual repair, and `D-27` /
> `DEC-054` accepting the refreshed packet after clean-head validation. The
> ordinary in-stage program is now Phase E: engineering beta and release
> machinery. R5 exit criteria are PRD §22.6: external engineers can reproduce
> validation examples, and the public repository contains no known protected
> standards data. Boundary prohibitions continue unchanged: no protected
> standards content, private-data defaults, unapproved release-readiness claim,
> professional approval, certification, sealing, authentication, or
> code-compliance claim.

The completion plan and `NEXT_INSTANCE_PROMPT.md` now select Phase E/R5 work in
order, with `D-20` prepared at the E7 lead-up and other R5 decisions prepared at
their named gates. `D-21` remains held until the governed `SCOPE_CHANGE` packet
is separately prepared and ruled.

## 8. Ruling Mechanism

Per existing practice, the human project authority selected an option or ruled
directly. The ruling is appended to
`execution/_Decomposition/SOFTWARE_DECOMP.md` §12 as `DEC-054` citing this
packet. Because the ruling advanced the current target stage to R5, the
coordination and plan surfaces named above were updated in a bounded follow-up.
This register row moved from `AWAITING_RULING` to `RULED` with the decision
pointer.
