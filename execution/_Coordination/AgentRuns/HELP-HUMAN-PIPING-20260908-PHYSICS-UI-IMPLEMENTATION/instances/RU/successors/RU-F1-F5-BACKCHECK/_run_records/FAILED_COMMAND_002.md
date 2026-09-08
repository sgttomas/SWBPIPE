# Failed command 002

Purpose: confirmation probe showing the two distinct hash-reference failures independently in one received object.

```sh
set -o pipefail
npx vitest run --config '../../execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260908-PHYSICS-UI-IMPLEMENTATION/instances/RU/successors/RU-F1-F5-BACKCHECK/_run_records/repros/vitest.config.mjs' 2>&1 | tee '../../execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260908-PHYSICS-UI-IMPLEMENTATION/instances/RU/successors/RU-F1-F5-BACKCHECK/_run_records/repros/vitest-output-confirmation.txt'
```

Exit status: `1`. The exact raw output is retained in `_run_records/repros/vitest-output-confirmation.txt`, SHA-256 `fda1aec191c2f2a426500066e445ccbd471ece4ae75961fd1af15a5aa2ea84df`. It reports both `engineGeneratedRefAccepted: true` and `echoedFrozenRefAccepted: true`, plus the independently repeated malformed-diagnostic and external-replacement failures.
