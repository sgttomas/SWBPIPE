Commands executed (cwd projects/chirality-piping)

Node-only configurations are preserved here. Read-only source/context inspections used rg, sed, cat, git status/diff, SHA256 reads. Edits used Python within the declared source/evidence fence. No browser, build, server or cohort ran.

```sh
node node_modules/typescript/bin/tsc --noEmit -p P/instances/WRITER/CONTINUATION/tsconfig.json
D70_WRITER_ORIGINAL_RETURN=<integration P>/instances/RUNNER/RETURN.json D70_WRITER_PREPARED_INPUTS=<integration P>/instances/RUNNER/COHORT_INPUT_PREPARATION_V2.json node node_modules/@playwright/test/cli.js test --config P/instances/WRITER/CONTINUATION/node-only.config.ts --grep 'characterization|real driver execution|attempt separates|required metadata|keyboard adapter|reference profile|actual action lifecycle|actual lifecycle retains|product inventory|frozen actual product|performance-targets.spec.ts|uninstrumented smoke canvas guard|continuation metadata|continuation launcher'
node apps/desktop/e2e/ui-foundation/verify-characterization-observations.mjs
D70_WRITER_ORIGINAL_RETURN=<integration P>/instances/RUNNER/RETURN.json node node_modules/@playwright/test/cli.js test --config P/instances/WRITER/CONTINUATION/node-only.config.ts --grep 'continuation launcher'
```

Initial new-test-only run used grep `continuation metadata|continuation launcher` (2 PASS); final broad run 39 PASS; after final budget gate change only launcher regression and TS repeated, both PASS. P is execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260915-PRODUCTION-UI/instances/ROOT/D70_BASELINE. Integration project root is /Users/ryan/.codex/worktrees/8728/chirality-rendering-baseline-20260917/projects/chirality-piping.

Final evidence generation hashes the exact 34 manifest files and scoped git diff. Scoped staging uses SOURCE_MANIFEST paths plus this evidence directory, then git diff --cached --check and local git commit; MANAGER evidence excluded.
