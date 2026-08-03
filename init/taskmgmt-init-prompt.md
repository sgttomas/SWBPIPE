# Task Management session init prompt — chirality-piping register

Launches one `TASK_MANAGEMENT` session bound to the Piping loop's Action
Item register. Paste-ready as written; replace `<none>` with a per-run steer
to narrow the run (e.g. `deferral review only` or `staleness pass only, no
harvest`). In a full generational Task Management phase, run this child-loop
session and land its closeout before the root session
(`init/taskmgmt-init-prompt.md` at the repo root).

<init-prompt>
Resolve `REPO_ROOT` with `git rev-parse --show-toplevel`.

Read `{REPO_ROOT}/AGENTS.md`.
Read `{REPO_ROOT}/agents/AGENT_TASK_MANAGEMENT.md` and follow it in full.

Act as `TASK_MANAGEMENT` for the chirality-piping loop. Register home:
`{REPO_ROOT}/projects/chirality-piping/execution/_Coordination/_TaskManagement/`.
Write scope is that register home only, plus routed drafts shipped solely
through closeout.

Run the generational pass, each step gated on my rulings before the next:

1. Mandatory federation preflight; report per-register status counts.
2. Mode: candidate harvest — full PRD §5.1 sweep. Present the Candidate
   Harvest Report; await my promotion rulings; apply exactly what I rule.
3. Mode: deferral review — the full DEFERRED population, including rows
   minted in step 2. Present the classification report; await my rulings;
   apply them.
4. Run `taskmgmt archive`; validate the live register and
   REGISTER_CLOSED.csv; run a final federation pass and quote its status
   counts as closeout evidence.
5. Closeout: assemble this session's durable products on a branch — the
   register and archive, reports, and any ruled routed notices with
   reciprocal citations (inbound SHA, row ID, evidence refs). The PR body
   carries the closeout record: every row changed and the ruling it cites,
   archive counts, status totals, federation evidence. Open the PR; do not
   merge — my merge is the gate. Report the PR number and final head SHA,
   then stop.

Never write a foreign register. Where a ruling requires another loop's
action, the response is a draft notice shipped only in the closeout tranche
under my gate.

Steer (this run): <none>
</init-prompt>
