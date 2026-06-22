# WORKING_ITEMS Run Record - D-25 R4 Exit Scope Packet

- Date: 2026-06-22
- Agent: WORKING_ITEMS (Type 1 persona)
- Tranche: TP-DECIDE-D25-R4EXITSCOPE-001
- Repo HEAD at preparation: `df017ba1d`

## Trigger

After the accepted twelve-fixture multi-support validation set landed, the
human asked whether any pending decisions would help close R4. The existing
pending packets are mostly R5, lifecycle, or v0.2 decisions, but the current R4
gap packet still leaves ambiguity about whether several residuals are R4
blockers or explicit post-R4/R5 hardening.

## Scope

Prepared a PROPOSAL-only decision packet for the R4 exit-scope bar:

- whether every residual named in the R4 gap packet must be closed before R4
  exit review;
- whether the bounded current seed/product/twelve-fixture nonlinear evidence
  plus current invented component-provenance evidence can be sufficient for an
  R4 exit-chain packet;
- or whether the human should name a smaller must-close subset.

No implementation, schema, solver, UI, fixture, report, release, lifecycle, or
target-stage behavior changed in this tranche.

## Evidence Sources Reviewed

- `docs/PRD.md` §22.5 for the literal R4 exit criteria.
- `execution/_Coordination/_COORDINATION.md` current target-stage state.
- `docs/PLAN.md` current roadmap posture.
- `plans/PLAN_2026-06-17_prd_completion.md` Phase D rows and FR-021 status.
- `plans/VERIFICATION_2026-06-21_r4_exit_gap.md` current not-ready gap packet.
- `execution/_Coordination/_DECISIONS/_REGISTER.md` current human decision
  register.
- `execution/_DAG/_LATEST.md` current approved DAG pointer.

## Outputs

- Added `execution/_Coordination/_DECISIONS/D-25_r4_exit_scope.md`.
- Added decision register row `D-25` as `AWAITING_RULING`.
- Updated active coordination and planning surfaces so later sessions know that
  the R4 exit-scope ruling is pending.

## Packet Recommendation

The packet recommends Option O-B: a bounded R4 exit envelope. Under O-B, agents
would prepare a final R4 exit-chain verification packet from current bounded
evidence and explicitly defer the listed sparse-default, external-validation,
deeper spring-hanger, broader-threshold, and release-quality residuals, subject
to a separate human R4 exit ruling.

This recommendation is not a ruling. R4 remains not ready until the human rules
D-25 and a final R4 exit-chain packet is prepared, validated, and separately
accepted.

## Validation

- `git diff --check` - passed.
- `test -f execution/_Coordination/_DECISIONS/D-25_r4_exit_scope.md` -
  passed.
- `rg -n "D-25|R4 exit-scope|AWAITING_RULING|twelve-fixture|R4 exit-chain"
  ...` across the decision packet, register, coordination, prompt, and planning
  surfaces - passed.
- `python3 tools/coordination/list_deliverable_status.py --dag DAG-007 --format
  table --summary` - passed; status counts remain `CHECKING=8`,
  `IN_PROGRESS=92`, `ISSUED=1`.

## Boundary Review

Packet is `PROPOSAL` / `AWAITING_RULING` only. It creates no lifecycle
transition, target-stage advancement, release-readiness claim, professional
approval, certification, sealing, authentication, code-compliance acceptance,
protected-data use, or deferral by itself.
