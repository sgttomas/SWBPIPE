---
doc_id: OPS-NEXT-SESSION-PROMPT
doc_kind: init.next_session_prompt
status: evergreen_bootstrap_protocol
updated: 2026-05-03
assignment: fresh_session_bootstrap_to_coordination_loop
approved_decomposition: execution/_Decomposition/SOFTWARE_DECOMP.md
approved_revision: "0.5"
scope: bootstrap fresh sessions into the live coordination protocol and mutable handoff state
next_workflow: defer to execution/_Coordination/NEXT_INSTANCE_PROMPT.md and execution/_Coordination/NEXT_INSTANCE_STATE.md
exclude_scope: any execution not authorized by the live coordination protocol and latest human gate
---

# NEXT SESSION PROMPT - Fresh-Session Bootstrap

Use this file when starting a new development session from scratch.

This file is only the bootstrap entrypoint. It is not the active coordination
protocol, does not name the next objective, and does not authorize work. Its job
is to route a fresh instance into the live control loop.

## Read First

Read these files before planning or editing:

1. `INIT.md`
2. `AGENTS.md`
3. `execution/_Coordination/NEXT_INSTANCE_PROMPT.md`
4. `execution/_Coordination/NEXT_INSTANCE_STATE.md`
5. `execution/_Coordination/_COORDINATION.md`

Then follow the fuller required-reading list in
`execution/_Coordination/NEXT_INSTANCE_PROMPT.md`.

## Bootstrap Behavior

- Start in read-only `ORCHESTRATOR` posture until the required reading is
  complete.
- Do not derive the current objective from this file. Derive it from
  `NEXT_INSTANCE_PROMPT.md`, `NEXT_INSTANCE_STATE.md`, `_COORDINATION.md`, and
  the latest explicit human gate.
- After reading, state that reading is complete, summarize the active control
  state and governing constraints, and check `git status --short`.

## Authority Order

For project coordination facts, use this order:

1. `INIT.md` and `AGENTS.md` for repository bootstrap and agent posture.
2. `execution/_Coordination/NEXT_INSTANCE_PROMPT.md` for the live coordination
   protocol.
3. `execution/_Coordination/NEXT_INSTANCE_STATE.md` for mutable handoff truth.
4. `execution/_Coordination/_COORDINATION.md` for durable coordination rulings.
5. The latest explicit human instruction, within the active governance
   constraints.

If this file conflicts with the live coordination prompt or mutable handoff
state, treat this file as stale bootstrap guidance and follow the live
coordination surfaces.

## Non-Authority

This file does not authorize:

- product implementation;
- `PREPARATION`;
- DAG refresh materialization;
- dependency refresh;
- blocker regeneration;
- lifecycle transitions;
- Type 2 dispatch;
- Chirality app/harness integration;
- commits or pushes.

## Session Conclusion Protocol

Before ending a session, follow the closeout protocol in
`execution/_Coordination/NEXT_INSTANCE_PROMPT.md`. Update this file only when
the fresh-session bootstrap protocol itself changes.
