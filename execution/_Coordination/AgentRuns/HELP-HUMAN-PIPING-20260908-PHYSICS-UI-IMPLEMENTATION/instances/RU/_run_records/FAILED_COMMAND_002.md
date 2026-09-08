# Failed command 002

Working directory: `projects/chirality-piping/apps/desktop`

Exact command:

```sh
npx vitest run --config '/Users/ryan/.codex/worktrees/8728/chirality-physics-ui-implementation-20260908/projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260908-PHYSICS-UI-IMPLEMENTATION/instances/RU/_run_records/repros/vitest.config.mjs' > '/Users/ryan/.codex/worktrees/8728/chirality-physics-ui-implementation-20260908/projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260908-PHYSICS-UI-IMPLEMENTATION/instances/RU/_run_records/repros/vitest-output.txt' 2>&1; rc=$?; sed -n '1,260p' '/Users/ryan/.codex/worktrees/8728/chirality-physics-ui-implementation-20260908/projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260908-PHYSICS-UI-IMPLEMENTATION/instances/RU/_run_records/repros/vitest-output.txt'; exit $rc
```

Exit: `1`. Exact output is preserved in `repros/vitest-output-attempt2.txt` (SHA-256 to be included in the final validation inventory). One reproduction passed. The delayed-Apply cancellation reproduction reached App's batch-receipt render and then the private mock's deliberately abbreviated successful receipt lacked `operation_outcomes`; the mock was completed before rerun.
