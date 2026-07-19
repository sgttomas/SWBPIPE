# Frozen Landing Manifest — D-54 / DEC-087

**Status:** `READY_FOR_DURABLE_LANDING; EFFECT HELD`
**Branch:** `codex/session-20260718`
**Pre-landing HEAD:** `756425eb53814f7a9f154fac5e2c139ef8ed5039`

## Frozen Candidate

- path:
  `projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260718-DISCRETION-REFINEMENT-R5/WORKPLAN_CANDIDATE_2026-07-18_piping_loop.md`
- bytes: `14834`
- Git blob: `b7041c1495bfc8f99b641bdb9db8d266790d68f4`
- SHA-256:
  `756cb3c654be4da851d813843dba443c70dcb8a17e5490769461f7f485ca1b0d`
- future active path:
  `projects/chirality-piping/loop/WORKPLAN_2026-07-18_piping_loop.md`

CHANGE must create the future active path from these exact candidate bytes and
require both `cmp -s` equality and the same Git blob before commit.

The frozen candidate header records the pre-clarification repeat-S5 condition
as historical metadata. D-54 §1A is the later governing source and expressly
supersedes only that condition; the candidate itself states that sources
govern on disagreement. The owner directed no technical-plan rewrite, and the
two v7 returns bind these exact candidate bytes.

Current stable governance bindings:

- D-54 §§2–3: 3,899 bytes; SHA-256
  `c3da25cfa553a4eed02b617cdbdb92f7a74e7a168be2679671e73c273b99cf17`
- DEC-087 description field after owner clarification: 2,091 bytes; SHA-256
  `4912a87c8ac660c7854220dec1f9e97a6785188fbf4fed6ee4cc02b408a4ce81`

## Exact Landing Scope

1. `projects/chirality-piping/execution/_Coordination/_DECISIONS/D-54_reasoned_discretion_standing_approval_refinement.md`
2. `projects/chirality-piping/execution/_Coordination/_DECISIONS/_REGISTER.md`
3. `projects/chirality-piping/execution/_Decomposition/SOFTWARE_DECOMP.md`
4. `projects/chirality-piping/loop/LOOP_INIT.md`
5. `projects/chirality-piping/loop/WORKPLAN_2026-07-18_piping_loop.md`
6. the complete
   `projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260718-DISCRETION-REFINEMENT-R5/`
   directory, including the candidate, all BLOCK/interruption/tool-error
   history, v7 verifier returns, owner-curtailed review record, and this
   manifest.

No other path is in scope. In particular, CHANGE must not modify app-dev,
Shared-Block v1, D-49 through D-53, DEC-082 through DEC-086, old workplans,
receipts, or any DEL-09-04 surface.

## Atomic Landing Gate

Before staging, recheck the candidate byte count, Git blob, and SHA-256 above;
the future active path must still be absent from HEAD, index, and worktree.
After byte-identical materialization, stage only the exact scope and verify
that HEAD-only discovery still selects the committed 2026-07-17 plan. Commit
the complete scope atomically. Post-commit, require a unique mode-`100644`
active blob equal to the candidate blob and require HEAD-only discovery to
select the committed 2026-07-18 plan.

No receipt, push, merge, release, publication, DEL-09-04 action, or external
effect is part of this landing.
