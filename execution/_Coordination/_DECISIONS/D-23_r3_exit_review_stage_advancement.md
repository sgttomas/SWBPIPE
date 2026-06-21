# D-23 - R3 Exit Review and R3-to-R4 Target-Stage Advancement

**Date prepared:** 2026-06-20
**Prepared by:** WORKING_ITEMS, per the Application Integration And Issuance
Loop decision-escalation step.
**Epistemic status:** PROPOSAL (non-governing). Only the human project
authority rules. Nothing here changes lifecycle state, issues deliverables,
creates release readiness, professional approval, certification, sealing,
authentication, code-compliance acceptance, or asserts that any PRD milestone is
met until the human records the ruling.

---

## 1. Decision statement and scope

Decide whether to accept the prepared R3 exit-chain evidence and advance the
Working Desktop Application Standard's current target stage from **PRD R3** to
**PRD R4** in `execution/_Coordination/_COORDINATION.md`.

The R3 criterion is from `docs/PRD.md` §22.4:

- User can define a private non-code rule pack and run checks.
- Software blocks pass/fail when required inputs are missing.

This decision governs ordinary tranche selection scope only. If the target
stage advances to R4, Phase D/R4 component and nonlinear-support work becomes
ordinary in-stage work. This decision does not issue deliverables, approve a
release, accept professional reliance, certify code compliance, promote
`DAG-007`, or resolve later human-gated decisions such as `D-15`, `D-17`,
`D-20`, or held `D-21`.

## 2. Evidence basis

Primary verification packet:

- `plans/VERIFICATION_2026-06-20_r3_exit_chain.md`
  (`TP-R3VERIFY-001`, SMOKE TP-MAC-190).

Supporting evidence:

- C1-C4 implementation spine: SMOKE TP-MAC-147..167 and TP-MAC-180.
- C5 readiness/usability/package path: SMOKE TP-MAC-183..188 and
  TP-MAC-272..277.
- C5 closure ruling: `DEC-047` in `execution/_Decomposition/SOFTWARE_DECOMP.md`
  §12. The stale TP-MAC-189 route remains bypassed, not passed.
- Current closeout validation: DEC-025 five-surface sweep
  `validation/evidence/sweeps/SWEEP_20260621T022113Z_f314fc1a67d7.json`,
  bound to commit `f314fc1a67d721aee1fff51498ab09506a1dd6e1`, overall pass.
  Surfaces: cargo crate sweep, repository pytest, desktop Vitest, desktop
  Playwright e2e including dist, and desktop production build.

The R3 verification packet records the evidence matrix and boundary review. Its
verdict is that the R3 product criterion is demonstrated in substance by the
landed C1-C4 implementation evidence and by the C5 replacement usability
closure accepted by the human project authority.

## 3. Known residuals and non-blockers

These remain explicit and do not disappear if the stage advances:

- Formal deliverable lifecycle remains unchanged. The status helper currently
  reports `CHECKING=8`, `IN_PROGRESS=92`, `ISSUED=1`; no lifecycle states are
  changed by this decision packet.
- `DAG-007` remains pending human approval; `_DAG/_LATEST.md` stays on
  `DAG-006`.
- Phase B-tail unit-aware I/O residuals are recorded as not R3-blocking in the
  completion plan.
- Future Phase D decisions remain at their named lead-ups: `D-15` spring-hanger
  scope and `D-17` sparse live-path timing are not prepared; `D-20` is a Phase E
  release-artifact scan decision; `D-21` remains held.
- No live embedded-agent runtime, external SDK/harness consumption, or
  autonomous accepted-model mutation is introduced by the R3 evidence.

## 4. Options

| ID | Option | Consequence |
|---|---|---|
| **O-A** | **Accept R3 exit evidence and advance the current target stage to R4.** | Phase D/R4 becomes ordinary in-stage work under the already-ruled D-16/D-18/D-19 decisions, while later gates stay intact. |
| **O-B** | **Hold at R3 pending additional evidence or repair.** | No Phase D/R4 work begins. The human must name the missing R3 evidence, blocker, or replacement criterion so agents can prepare the next bounded tranche. |
| **O-C** | **Accept the R3 evidence but keep the current target stage at R3 for residual hardening.** | The R3 product criterion is accepted, but ordinary work remains in R3. This avoids stage movement but risks substituting unbounded hardening for the planned R4 dependency spine. |

## 5. Advisory recommended disposition (PROPOSAL)

Recommend **O-A**.

Rationale: the R3 exit criterion is narrow and is demonstrated by the landed
C1-C4 rule-pack/private-library/check-run evidence, while the two R2-carried
R3-exit residuals were closed for C5 by the human `DEC-047` replacement
criterion ruling. Holding at R3 now would require a newly named blocker; absent
one, the completion plan's next dependency-spine work is Phase D/R4. Advancing
the target stage does not relax any boundary and does not create release or
professional claims.

## 6. Proposed coordination edit (applied only on acceptance)

If O-A is accepted, replace the current target-stage paragraph in
`execution/_Coordination/_COORDINATION.md` with text equivalent to:

> **Current target stage - PRD R4 exit criterion (advanced 2026-06-20 by
> `DEC-0XX`).** Phase C/R3 evidence was reviewed per
> `plans/VERIFICATION_2026-06-20_r3_exit_chain.md` and the human accepted the
> R3 exit review through `D-23`. The ordinary in-stage program is now Phase D:
> piping components and nonlinear supports. R4 exit criteria are PRD §22.5:
> nonlinear support validation cases converge, and component provenance appears
> in reports. Boundary prohibitions continue unchanged: no protected standards
> content, private-data defaults, network/telemetry feature, release-readiness
> claim, professional approval, certification, sealing, authentication, or
> code-compliance claim.

The completion plan and `NEXT_INSTANCE_PROMPT.md` should then be updated to
select Phase D/R4 work in order, with `D-15` and `D-17` prepared at their named
lead-ups and with `D-20` held for Phase E.

## 7. Ruling mechanism

Per existing practice, the human project authority selects an option or rules
directly. The ruling is appended to
`execution/_Decomposition/SOFTWARE_DECOMP.md` §12 as the next `DEC` entry
citing this packet. If the ruling advances to R4, the coordination and plan
surfaces named above are updated in a bounded follow-up. This register row then
moves from `AWAITING_RULING` to `RULED` with the decision pointer.
