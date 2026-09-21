# Sealed launch message — W2 PKG-03 verifier-triggered rerun manager (cycle 1)

You are a fresh WORKING_ITEMS manager for a verifier-triggered rerun in
gate wave W2 of run HELP-HUMAN-PIPING-20260921-RECONCILIATION (WAVE_PLAN.md,
"Verifier reruns"). Your parent is HELP_HUMAN Agent 0.

Read your brief first and follow it exactly:
/Users/ryan/dev/chirality/.claude/worktrees/task-management-gen-pass-518da2/projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260921-RECONCILIATION/briefs/R2-MANAGER_brief.md (SHA-256 dc8a6da2c52326026282fa3f75585f06a78dd331b0f05977ca74d629f98a595e)

The worker brief it names is
/Users/ryan/dev/chirality/.claude/worktrees/task-management-gen-pass-518da2/projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260921-RECONCILIATION/briefs/R2-WORKER_brief.md (SHA-256 2d793d0a7669618d6eca6f1c8cb66c180ea0edb038a9605d0c0e29556b0db141).
If either hash does not match, stop and report.

Values:
- {PKG} = PKG-03
- {WAVE} = W2 (worker ledgers stay under RUN/WAVES/W2/PKG-03/)
- {ASSIGNMENTS} =
G1: DEL-03-07
- {WORKER_BUDGET} = 1
- {FREEZE} = /private/tmp/claude-501/-Users-ryan-dev-chirality--claude-worktrees-task-management-gen-pass-518da2/efe0b4ff-c1b6-4a97-af70-b04220464651/scratchpad/freeze (read-only checkout of 00115c71931bcae79909602d653740d3bb72dfa1; never write there)
- {REPO} = /Users/ryan/dev/chirality/.claude/worktrees/task-management-gen-pass-518da2

Differences from a first run:
- Your record directory is
  /Users/ryan/dev/chirality/.claude/worktrees/task-management-gen-pass-518da2/projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260921-RECONCILIATION/_run_records/W2-PKG-03-RERUN1-MANAGER/
  (not the W2-PKG-03-MANAGER directory, which belongs to the first run).
- This is a rerun. The worker's launch message must say so and invoke the
  worker brief's rerun clause: before writing anything, move the existing
  DEL-03-07 files unchanged into RUN/WAVES/W2/PKG-03/DEL-03-07/superseded_1/,
  then encode the deliverable afresh (both passes, sealed, under Part F).
- The defect to name in the worker's launch message is the verifier's
  finding, not a validator failure: the package verifier
  (RUN/WAVES/W2/PKG-03/PKG-03_VERIFICATION.md, SHA-256 f08a7f0d2092db06ca6ebba0d951f2c22264ea2a0c3e81e7ace9ce85ef817bbd) returned
  RERUN DEL-03-07 because DEL-03-07:SOW#CLM-021 was ALIGNED while its own
  Notes record an unmet element of the claim (F1). The worker may read that
  verification report's DEL-03-07 findings after its forward ledger is
  sealed, not before (the forward pass must be its own judgment).
- The batch step (brief step 5) runs over all eight PKG-03 forward ledgers,
  using the fresh DEL-03-07 one.

Model for the worker: opus; reasoning inherited ("high (inherited)").
The run cap is 16 live agents including Agent 0. Do not exceed 1 live worker.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
