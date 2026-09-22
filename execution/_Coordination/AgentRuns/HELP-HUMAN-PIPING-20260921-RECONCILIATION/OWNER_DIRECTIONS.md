# Owner directions — whole-corpus reconciliation of Piping

Recorded 2026-09-21 by HELP_HUMAN Agent 0 (Claude Code). Each quoted block is
extracted from this session's stored transcript
(Claude Code session transcript `efe0b4ff-c1b6-4a97-af70-b04220464651.jsonl`),
the user text of the named entry, UTF-8, no trailing newline added. This is
stored-transcript custody, not original transport bytes. Agent 0's reading
follows each block and is labelled as such; it is not the owner's word.

## Entry brief

The session's opening message, with host-injected `<system-reminder>` blocks
removed, is kept byte-for-byte beside this record at
`instances/ROOT/ENTRY_BRIEF_2026-09-21.md` (transcript entry 2,
2026-09-21T15:29:22.611Z, 8784 bytes, SHA-256 `3767db254c5608611607c8d773826e467add5a7e7d6c49e0b9907f88e7af7823`). It orients the
session, states the owner's intended shape of the undertaking as agreed in a
parallel App session, and authorizes no dispatch.

## Direction 1 — 2026-09-21T15:56:47.413Z (transcript entry 388)

SHA-256 `ac3970390db23ea796109994730118b1edddd06cc45032ed7f48302b7a3e1a62` (1825 bytes).

~~~~
Any merged PR is an extension of my intent, through a standing instruction and my confidence in the existing structure to carry my broad intent sufficiently that I don't need to review the PR once I understand what that tranche of work is intended for.  So merged PRs are great references to see what was done and why.  Does your proposed deployment follow the conventions of the reconciliation workflow?  I'm not sure I understand how the deterministic script will work with the claims extraction, surely you're using LLM judgment at some point?  No other work will proceed in the project (other than a concurrent "reconciliation" effort now underway in a Claude worktree for the `chirality-app-dev` ).

A) Yes piping is suspended at that merger.  The pause will remain in effect until you're done. I don't understand the "4-commit" continuation.  Pull origin/main to get the current state and use that as your baseline. The scope is whole-corpus.  We do not make reference to the July plan, we come up with the best plan under these circumstances.  I waive the `## Remaining` boostrap seeding as a ruled variance, because it's stale.  Yes this could feed `scope-change` and a DAG rebuild depending on the outcomes, but that's outside your scope.  However, that is a typical course of events after a reconciliation.  I've notified other agent sessions and paused work.  

B) Ok, noted.

C) Use `opus-5` models on `high` reasoning (that will be your settings too, so it can just propagate to your subagents).  The concurrency limit is 16 agents, including yourself.  I may revise that later mid-run depending on how the session usage limits are consumed. I don't know what a "no-op nesting proble" is.  Let's discuss that further.  What do you intend by running a fresh sweep for the Gate transcript?  Let's discuss that too.
~~~~

Agent 0's reading:

- A merged PR extends the owner's intent through the standing Git instruction;
  merged PRs are references for what was done and why. Carried to R0 as a
  candidate convention in these words.
- Piping is suspended at the PR #834 merge; the pause holds until this run
  finishes. The baseline is current `origin/main` after fetch:
  `00115c71931bcae79909602d653740d3bb72dfa1` (PR #835, which touches no path
  under `projects/chirality-piping/`; the Piping tree is identical to the
  PR #834 merge `620ff6387b211c774c0de7dabec0a170acdf6017`).
- Scope is whole-corpus.
- The July plan is not referenced; the method is chosen for current
  circumstances.
- `## Remaining` bootstrap seeding is waived as a ruled variance
  (clarified in Direction 2).
- Scope-change and a DAG rebuild are typical follow-ons and outside this run.
- No other Piping work proceeds; the concurrent App reconciliation is
  separate. The owner has notified other sessions.
- Models: `opus-5`, high reasoning, for Agent 0 and every subagent.
- Concurrency: 16 agents including Agent 0; the owner may revise mid-run.

## Direction 2 — 2026-09-21T16:09:59.750Z (transcript entry 405)

SHA-256 `ab4cc637fed085f4497cbcd58df0de634e2451b18f25d7856443497b0ddfe297` (765 bytes).

~~~~
Wait, maybe I didn't understand this correctly: "No per-deliverable seeding. The kernel's activation pattern seeds gated Remaining items in each deliverable. You have now ruled that out as a variance, and the packet will say so." 

I meant that the current `## Remaining Items` are stale and can't be referred to as an authority, but seeding new items was never in my thoughts.  What's that about?  I thought that any changes to the deliverables folders or files was a separate act outside of this workflow?

Okay I understand the deterministic script and how you intend to use it.  Very good.

PR #834 has been merged.  I agree you should use a fresh run at R0. I have already confirmed that this harness will permit you to nest subagents.  I'm sure you can do it.
~~~~

## Direction 3 — 2026-09-21T16:11:40.674Z (transcript entry 412)

SHA-256 `78cff4835a1c59d6b96da3d0c7c800919cba87636cee04ccf4488aa3fc78f4dd` (110 bytes).

~~~~
1 and 2 match what I meant.  Proceed on that basis to plan out the work ahead from the perspective of Agent 0.
~~~~

Agent 0's reading of Directions 2 and 3 (the owner confirmed items 1 and 2 of
Agent 0's reply "match what I meant"):

1. No new items are seeded into any deliverable's `## Remaining`. The kernel's
   activation pattern (`docs/DELIVERABLE_CONCORDANCE_METHOD.md` §6) would seed
   them; not doing so is a variance for the owner's ruling in D-73.
2. Existing `## Remaining` entries are stale and are never cited as authority
   for what is open or done. They are audited as declared-state claims.
   Carried to R0 as a candidate convention.
3. Deliverable files change only through edits the owner authorizes after
   seeing the findings; discovery never edits them.
4. The gate transcript is a fresh run at R0. Nested delegation is confirmed
   by the owner.

## Direction 4 — 2026-09-21T16:14:50.722Z (transcript entry 430)

Returned with the owner's rejection of the first plan-approval request.
SHA-256 `94ebe9ac617496273102d7910c7e86192d4eb0c7d756b5ca18c949e26468ac1b` (82 bytes).

~~~~
Did you declare the max concurrency for agents and subagents (including yourself)?
~~~~

Agent 0 added the concurrency declaration to the plan: a hard cap of 16 live
agents at any instant, counting Agent 0 and every agent at any nesting depth,
with per-phase allocations. The owner then approved the plan
(2026-09-21T16:15:18.321Z, transcript entry 446). The approved plan is kept
beside this record as `PLAN.md`.

## Direction 5 — 2026-09-21T17:24:42.046Z and 17:25:45.749Z (entries 904, 925)

SHA-256 `1baa99b73b9c267c9ff176607b387c2ca7fc9fd967d38b0bedc1645c5b88e245` (88 bytes):

~~~~
why are you doing a full 60 minutes CI run for what was essentially a plan being merged?
~~~~

SHA-256 `41221e64eb4870ff9e3b47a6fcefc426daf471745b9dcf69fad03dee8233d8ab` (140 bytes):

~~~~
I think you can just site the existing sweep and CI records you've just obtained.  What value is there in running even a trimmed test suite?
~~~~

Agent 0's reading: no test suite is run; the run cites the PR #834 hosted CI
and the 9d55 local sweep, both bound to the frozen Piping code. Recorded as
`../../_DECISIONS/D-73_RULING_ADDENDUM_2026-09-21.md`.

## Direction 6 — 2026-09-21T20:27:41.240Z (transcript entry 2384)

SHA-256 `4e85c7551c0e285e8bfc2417c7a5bab486c9830c96e0e86e81d87afcf7dcb24a` (673 bytes):

~~~~
13 deliverables is fine, I don't remember ruling 12 at any point (what's your reference for that?  I may need to overturn).  Your instructions are 16 max concurrent agents including yourself and all subagents.  And you should try to maintain the max, if a certain set of packages and deliverables has more than what can be done at once and you do need to break up, keep at the max concurrency until all is done.  You shouldn't purposely limit yourself to less, without a valid reason (such as maintaining a slot for reviewing, or other impending task that you need to orchestrate in the right sequence).  But why only 7 right now?  That should be more as far as I can tell.
~~~~

Agent 0's reading: (1) the 13-deliverable first wave is confirmed (owner item
1 at the wave 1 checkpoint is closed). The "12" came from the calibration
review's recommendation (`R0_REVIEW.md` §7), bundled into the R0 amendment-set
question the owner adopted; the owner did not choose that number separately.
(2) Standing concurrency rule: keep 16 live agents (Agent 0 and all
subagents at any depth) whenever work is available; run fewer only for a
stated reason such as a reserved review slot or a sequencing dependency.

## Direction 8 — 2026-09-22T00:09:59.138Z (transcript entry 3208)

SHA-256 `4bb2eaab60c5b38d9fd17b83c0aa89c34be4fa31bf0a6321f955c38356a51992` (153 bytes):

~~~~
I confirm the SEMANTIC_READY reading, but also that it is outdated as a status and should be advanced upon my subsequent approval in the proper workflow.
~~~~

Agent 0's reading:
1. The Agent 0 reading of the F3 exception is confirmed. The SEMANTIC_READY
   architecture-basis sub-claims (9 in W1, 23 in W2, and any later ones) are
   `STALE_REVIEW_OR_EVIDENCE`. The resolutions class changes from
   `AGENT_READING` to `OWNER_CONFIRMED`.
2. The SEMANTIC_READY status is outdated. Advancing it is a lifecycle change
   that needs the owner's later approval through the proper workflow. It is
   outside this run (D-73: no deliverable, lifecycle or DAG writes through
   R4), so it goes to R4 as a routing item: the status advance, on owner
   approval, through the proper lifecycle workflow. This run does not change
   any status.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
