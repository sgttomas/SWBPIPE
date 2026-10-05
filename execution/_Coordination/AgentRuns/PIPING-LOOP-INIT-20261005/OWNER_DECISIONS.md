# Owner decisions: piping LOOP_INIT in the binding form (2026-10-05)

Run `PIPING-LOOP-INIT-20261005`. HELP_HUMAN (Claude Code session, ROOT for piping T3) transcribed the owner's words exactly from the chat session. This file records owner direction; the agents' readings are labelled as such.

## The question (owner, exact, 2026-10-05)

> Before I proceed, review the LOOP_INIT.md as it currently is in the `chirality-app-v4` project and compare that to the one you're using and the legacy one in the `chirality-app-dev` project.  Do you see the kinds of changes I'm looking for?

**HELP_HUMAN's reading, given in reply.** The changes are the ones the owner decided for App v4 on 2026-10-04, in `projects/chirality-app-v4/execution/_Coordination/AgentRuns/APP-V4-GRAPH-CLOSURE-20261004/OWNER_DECISIONS.md`: "How LOOP_INIT steers agents through the guidance" and "What LOOP_INIT is, and what an agent reads at entry". In short:
- LOOP_INIT only binds the project to the general layers. Each sentence is specific to the project, instructs, and is stated nowhere else.
- The entry reading is a map: the Agent User Manual's headings and the Field Book, then the work graph. The agent decides when to read further. There are no routers.
- No new record type keeps current state; the handoff is ephemeral. Editions are not pinned.

Piping's LOOP_INIT is the legacy App (v3) text with project substitutions, and HELP_HUMAN's T3 handoff prompt runs against these changes.

## The direction (owner, exact, 2026-10-05)

> The steering instructions can be more extensive that one or two sentences. In fact, they can be quite elaborate if justified.  I haven't started anything else yet.  Make appropriate changes to the LOOP_INIT and your own handoff steering instruction (that will just be copied and pasted into the next init-prompt to begin the session).  So don't start the next session, I'll do that, but otherwise carry out those actions to get things set up properly.

**Effect.**
- **Authorized:** changing `projects/chirality-piping/loop/LOOP_INIT.md` into the binding form, with the consequential edits needed elsewhere to keep instructions consistent, and replacing HELP_HUMAN's T3 handoff prompt with a steer for piping's init prompt.
- **The steer may be elaborate** where that is justified.
- **No other piping loop has been started.**
- **HELP_HUMAN does not start the next session.**

## Earlier in the same session (owner, exact, 2026-10-05)

> Well yes I only want an appropriate level of CI done.

This applies to this tranche's own gates.
