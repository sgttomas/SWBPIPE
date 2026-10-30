# Coordination Prompt State-Refresh Plan

**Date:** 2026-06-21

**Epistemic status:** issue-plan under the project-local Issue-Plan Rule. This
plan records an agentic-development-loop problem discovered during R4/Phase D
work. It is not a governing artifact, lifecycle transition, target-stage
advance, release-readiness claim, professional approval, certification,
sealing, authentication, or code-compliance claim.

## Problem

`execution/_Coordination/NEXT_INSTANCE_PROMPT.md` is the stable entry prompt for
new WORKING_ITEMS sessions. It is valuable as an entry protocol, but its
state-oriented "Next Action" section can become stale after rapid tranche
closeout. During the 2026-06-21 R4 D9 work, the prompt still routed agents
through older D6/D9 residual language after later evidence had already landed:

- `TP-R4-D9-FORCEDISPRESID-001`
- `TP-R4-D9-PRODPOLICY-001`
- `TP-R4-D9-GAPREFRESH-001`

At the time this issue was recorded, the current authoritative state was
discoverable and correct elsewhere:

- `execution/_Coordination/_DECISIONS/_REGISTER.md` records both `D-15` and
  `D-17` as `AWAITING_RULING`.
- `docs/PLAN.md` records that with `D-15` and `D-17` both awaiting ruling, no
  current Phase D dependency-spine implementation item should be selected from
  those gates until a ruling lands.
- `plans/VERIFICATION_2026-06-21_r4_exit_gap.md` now records the same blocker
  posture.
- `execution/_Coordination/_COORDINATION.md` says current state must be
  discovered from authoritative artifacts and that `NEXT_INSTANCE_PROMPT.md`
  should remain stable unless the entry protocol itself changes.

The practical failure mode is repeated session re-entry work: agents must
re-discover that the prompt's state snapshot is older than the decision
register, roadmap, and R4 gap packet before they can safely stop or request a
human ruling.

## Proposed Fix

At the next explicit coordination-maintenance opportunity, split the stable
entry prompt into two clearly separated parts:

1. **Stable protocol block.** Keep path anchors, intake order, authority
   hierarchy, and boundary prohibitions in `NEXT_INSTANCE_PROMPT.md`.
2. **State summary block.** Replace detailed landed-tranche lists with a short
   instruction to consume `docs/PLAN.md`,
   `plans/PLAN_2026-06-17_prd_completion.md`,
   `execution/_Coordination/_DECISIONS/_REGISTER.md`, and the current gap or
   verification packet.

The prompt should say explicitly that any state summary inside it is
non-authoritative and may be stale relative to the decision register,
roadmap/completion plan, deliverable-local memory, and run records.

## Acceptance Criteria

- A future coordination-maintenance tranche either updates
  `NEXT_INSTANCE_PROMPT.md` with the split above or records why the current
  single-file prompt remains preferred.
- No future entry prompt text names a completed decision-preparation item as
  "next" when the decision register already shows that packet as
  `AWAITING_RULING`.
- The coordination contract remains the authority: no separate session-state
  file is introduced.

## Current Disposition

Initial disposition: no prompt edit was made in the issue-plan tranche because
`_COORDINATION.md` instructs agents to leave `NEXT_INSTANCE_PROMPT.md` stable
unless the entry protocol itself changes.

Follow-up disposition: during the D-15/D-17 ruling-record tranche on
2026-06-21, the stale state summary in `NEXT_INSTANCE_PROMPT.md` was updated
without changing the entry protocol. The prompt now points to `DEC-049`
and `DEC-050` rather than routing future sessions to already-ruled
decision-packet work.

## Validation

- `git diff --check` passed on 2026-06-21.
- Full DEC-025 evidence sweep passed:
  `validation/evidence/sweeps/SWEEP_20260621T115235Z_0efaab1e6b48-dirty.json`.
- Follow-up D-15/D-17 prompt refresh validation is recorded in the later
  ruling-record tranche evidence.
