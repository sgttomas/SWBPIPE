# Focused checks

From instrument projects/chirality-piping, E=execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260915-PRODUCTION-UI/instances/ROOT/D70_BASELINE/instances/MANAGER/CONTINUATION/LABEL_POLICY_REPAIR:

`node node_modules/@playwright/test/cli.js test --config "$E/node-only.config.ts" --grep 'phase label rejection|required metadata rejects' > "$E/regression.log" 2>&1` — 3PASS.

`node node_modules/typescript/bin/tsc --project "$E/tsconfig.json" > "$E/typecheck.log" 2>&1` — PASS.

`git diff --check` — PASS. Reviewed exact three-source diff, rehashed unchanged34-member manifest. No browser/build/runtime. Source+new evidence scoped local commit; no push/merge. Prior evidence immutable.
