# Sealed launch message — W3 PKG-12 manager

You are the WORKING_ITEMS manager for PKG-12 in wave W3 (rolling queue) of run
HELP-HUMAN-PIPING-20260921-RECONCILIATION. Your parent is HELP_HUMAN Agent 0.

Read your brief first and follow it exactly:
/Users/ryan/dev/chirality/.claude/worktrees/task-management-gen-pass-518da2/projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260921-RECONCILIATION/briefs/R2-MANAGER_brief.md (SHA-256 dc8a6da2c52326026282fa3f75585f06a78dd331b0f05977ca74d629f98a595e)

The worker brief it names is
/Users/ryan/dev/chirality/.claude/worktrees/task-management-gen-pass-518da2/projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260921-RECONCILIATION/briefs/R2-WORKER_brief.md (SHA-256 2d793d0a7669618d6eca6f1c8cb66c180ea0edb038a9605d0c0e29556b0db141).
If either hash does not match, stop and report.

Values:
- {PKG} = PKG-12
- {WAVE} = W3
- {ASSIGNMENTS} =
G1: DEL-12-01, DEL-12-02, DEL-12-03
G2: DEL-12-04, DEL-12-05
- {WORKER_BUDGET} = 1
- {FREEZE} = /private/tmp/claude-501/-Users-ryan-dev-chirality--claude-worktrees-task-management-gen-pass-518da2/efe0b4ff-c1b6-4a97-af70-b04220464651/scratchpad/freeze (read-only checkout of 00115c71931bcae79909602d653740d3bb72dfa1; never write there)
- {REPO} = /Users/ryan/dev/chirality/.claude/worktrees/task-management-gen-pass-518da2

This wave runs under CONVENTIONS Part F: every worker validates with
`--notes-gap` before sealing, and you validate every deliverable with
`--notes-gap` too (manager brief step 4). Deliverable-local evidence paths
may be cited in evidence columns (paths with spaces resolve at the freeze).

With a budget of 1, launch the groups one at a time (G1, then G2 when G1
returns), each in the foreground; this overrides brief step 3's "all groups
in one message" so that the budget holds.

Model for workers: opus; reasoning inherited ("high (inherited)").
The run cap is 16 live agents including Agent 0; your budget is counted
against it. Do not exceed 1 live workers.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
