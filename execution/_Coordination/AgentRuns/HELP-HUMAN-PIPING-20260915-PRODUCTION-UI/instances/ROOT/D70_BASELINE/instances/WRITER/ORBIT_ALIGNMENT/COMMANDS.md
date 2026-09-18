# Commands

Read sealed brief/INPUTS, ROOT orbit repair brief and canonical controls-smoke-02 failure; rg existing constant, smoke and timed orbit setup; git rev-parse --show-toplevel. Python verified all INPUTS hashes and changed exactly the two import/use lines; source diff is preserved losslessly in SOURCE_DIFF.patch.gz. New evidence/configs only under this directory.

From instrument projects/chirality-piping, E=execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260915-PRODUCTION-UI/instances/ROOT/D70_BASELINE/instances/WRITER/ORBIT_ALIGNMENT:

`node node_modules/@playwright/test/cli.js test --config "$E/node-only.config.ts" --grep 'uninstrumented smoke canvas guard' > "$E/regression.log" 2>&1` — 1 PASS.

`node node_modules/typescript/bin/tsc --project "$E/tsconfig.json" > "$E/typecheck.log" 2>&1` — PASS.

`git diff --check` and `git diff --stat`; scoped git add, cached diff check, local commit. No browser/build/cohort/network/push/merge. No prior evidence rewritten.
