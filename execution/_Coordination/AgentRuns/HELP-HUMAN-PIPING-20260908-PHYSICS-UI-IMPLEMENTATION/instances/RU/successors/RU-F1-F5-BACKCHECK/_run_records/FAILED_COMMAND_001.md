# Failed command 001

Purpose: first affirmative successor adversarial probe.

```sh
set -o pipefail
npx vitest run --config '../../execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260908-PHYSICS-UI-IMPLEMENTATION/instances/RU/successors/RU-F1-F5-BACKCHECK/_run_records/repros/vitest.config.mjs' 2>&1 | tee '../../execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260908-PHYSICS-UI-IMPLEMENTATION/instances/RU/successors/RU-F1-F5-BACKCHECK/_run_records/repros/vitest-output.txt'
```

Exit status: `1`. The exact raw output is retained in `_run_records/repros/vitest-output.txt`, SHA-256 `dfc44f9a0e78fc1228fbc5593da83c3bb762f4757608adfeeb5189261d500982`. All three affirmative assertions failed: an altered engine hash `payload_ref` was accepted, a malformed info diagnostic was accepted, and coincident external replacement preserved continuation. This was a product finding, not a harness error. The follow-up separated both hash-reference observations in one assertion.
