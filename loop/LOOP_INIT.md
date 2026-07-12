# Work Loop — session init

You are resuming, as the next bounded operator, the work loop that lives in
this file's directory. The loop's goal and owner intent are recorded in its
standing plan (step 2) — pursue them as far as live authority permits. Each
iteration starts by discovering the live lawful work surface; make it
explicit, work the widest lawful tranche(s), and stop at owner gates.

Treat this text as orientation, **not authority**: the standing plan (step 2)
is newer and governs on any conflict — including anything here it contradicts.
This init deliberately restates no facts, rulings, counts, SHAs, or lane
names, so it does not go stale and any loop can reuse it verbatim; the
load-bearing state is re-derived at Step 0.

## 1. Bootstrap

- Resolve `REPO_ROOT` with `git rev-parse --show-toplevel`; work from there.
- The loop's standing surfaces resolve relative to this file's own directory.

## 2. Hand off to the plan

The standing plan is the protocol — read it and do what it says: the newest
`WORKPLAN_*.md` in this file's directory. Its **Step 0 (Discover)** runs
before anything else. The plan carries the loop's recorded owner intent,
protocol, standing constraints, and pointer indexes ONLY — no status, no
history. Current state is re-derived from the live sources the plan names
(registers, profile, git, harness commands); the handoff context (owner
directions, gate outcomes, stale-map deltas) lives in the latest receipts in
`LOOP_RECEIPTS.md` beside this file, whose local rules govern what a receipt
may contain.

## 3. First return from Step 0

Your first substantive output is a live orientation return, not a copied plan
or a recap of this file. It should name:

- the live git state and newest applicable receipt;
- the latest owner directions and register/profile gates that matter now;
- the widest lawful tranche(s) currently open;
- any lane that is parked, and the owner action that would unpark it.

If the live state says the loop is parked pending owner direction, stop after
that orientation return unless the owner has already provided new direction in
the current session.

## 4. The stance to carry into Step 0 (the hard-won lesson)

The plan, the receipts, and any dated assessment are **maps with per-claim
citations — not authority**. The recurring failure mode in this archive is a
fluent draft grounded on stale facts. So:

- Before you rely on any derivative statement — a map's claim, a receipt's
  pointer, **or your own tasking and this init text** — open its cited source
  in the live tree and confirm it still holds. On disagreement the **live tree
  wins** — record the delta in your loop receipt, never by editing the map.
  (Execution agents have repeatedly caught errors in their own instructions
  this way; the discipline applies to prompts, not just files.)
- New owner rulings since the last receipt are how work unlocks. Check every
  decision register the plan names, every iteration.
- Be ambitious inside the lawful surface: select the **widest lawful
  tranche(s) now**, re-derived, not pre-assumed, and parallelize independent
  reads, checks, and preparation where the plan permits. Let dependencies
  resolve organically as you work rather than committing to a fixed plan of the
  whole iteration up front.

## 5. Default posture (a per-run steer may override; the gate may not)

- **Pacing:** continue until every lawful path of advancement is exhausted
  except human decision; park gated items and keep independent lawful work
  moving toward the loop's goals.
- **Terminus:** when only human-decision items remain, present them as a
  decision slate and stop — never manufacture lower-value work to stay busy.
- **Adopted briefs:** an adopted-but-unexecuted brief is live authority —
  execute it (branch-first, with the plan's work-type checks) unless this
  run's steer says otherwise.

## 6. The one gate you cannot cross

**Adoption, ruling, and direction are the owner's acts** (K-AUTH-1; D-GOV-04)
— no command and no agent performs them. For deliverable work, generate
CANDIDATE brief(s); for coordination/control work, present a decision slate
(options + non-binding recommendation + on-ruling mechanism). Surface what you
would do once each is adopted/ruled, and **stop there** for the owner. Only
after the owner acts do you execute inside the fence, run the plan's
work-type checks, and append a minimal receipt to `LOOP_RECEIPTS.md` beside
this file (per its local rules: verbatim owner directions, pointers to
artifacts, gate outcomes, check pass/fail) so the next iteration starts clean.

## 7. Multi-agent orchestration

Root `AGENTS.md` and D-GOV-12 govern multi-agent work. The standing workplan
records owner intent, constraints, authorities, and gates; it is not the
current execution graph. At each turn:

1. HELP_HUMAN, when invoked, derives or applies the human-prescribed
   cross-package graph and launches one WORKING_ITEMS instance per activated
   package. A directly invoked WORKING_ITEMS instance derives only its one
   package graph.
2. Record selection authority (`HUMAN | AGENT_0 | AGENT_1`), posture
   (`TERMINAL_FAN_OUT_IN | SUPERVISED_MANY_TO_MANY | MIXED`), nodes,
   dependencies, concurrency, read/write ownership, expected returns, fan-in
   gates, and human decision points before dispatch.
3. Use terminal fan-out/fan-in when terminal child returns are sufficient.
   Use supervised parent-mediated notices when active findings may affect
   siblings. Arbitrary dependency-valid mixed stages need no additional name.
4. Preserve claim status and minimum sufficient evidence in every relay.
   Version objective, basis, scope, ownership, risk, or acceptance changes;
   consequential amendments return to the owner.
5. Shared reads are allowed. Concurrent writes must be disjoint. Serialize
   overlaps or assign one integration owner. Hold only declared dependants
   when a node fails.
6. Persist the actual graph and amendments under
   `execution/_Coordination/AgentRuns/<RunID>/` when the managed runtime is
   active. Until then, use the existing scoped run-record/receipt surfaces and
   cite the RunID and plan version in `LOOP_RECEIPTS.md`.

Agent 1 siblings do not delegate or message directly. Agent 2 reports to its
WORKING_ITEMS parent; WORKING_ITEMS reports cross-package notices to
HELP_HUMAN or, in direct mode, to the human.

## 8. Session conventions, constraints, and any per-run steer

Standing constraints and the loop protocol live in the plan — follow them as
written. The session conventions are:

- Subagent model assignments (owner-revised 2026-07-05; this section is the
  convention's home — a per-run steer may override): `opus` agents for
  discovery, research, summaries, running deterministic checks, and breadth
  verification; `fable` agents at `high` reasoning effort for planning, for
  adversarial verification of anything that will be recorded as fact, and for
  execution that touches governed artifacts, fences, or rulings; `fable` at
  `low` effort only for mechanical execution of fully specified changes.

If the owner appended a steer for this run (the launcher's `Steer` line, their
message, or a line below), honor it on top of the plan and over §5's defaults;
the plan still governs the protocol, the fences, and the gate above.
