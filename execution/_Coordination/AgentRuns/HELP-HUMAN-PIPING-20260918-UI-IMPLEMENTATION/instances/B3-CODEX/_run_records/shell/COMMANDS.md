# Commands retained for reproduction

Working directory: `projects/chirality-piping/apps/desktop` within assigned repository checkout. Let `R=../../execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION` and `E=$R/instances/B3-CODEX/_run_records/shell`.

All browser calls used `PLAYWRIGHT_WORKERS=1 sh "$R/tools/with_e2e_lock.sh" npx playwright test ...`. JSON-reporting calls also set `PLAYWRIGHT_JSON_OUTPUT_NAME="$E/<stem>.json"`, `--reporter=list,json`, `--output "$E/<stem>-artifacts"`, and redirected both streams to `$E/<stem>.log`. No test timeout was widened. `--max-failures=1` stops early; it creates no skipped test annotation.

| Stem | Arguments after `playwright test` (reporter/output/redirection above) |
|---|---|
| focus-clearance | `e2e/workspace-layout.spec.ts --grep 'workspace preserves|scale, triad|measurement'` (list reporter only; matched navigation only) |
| workspace | `e2e/workspace-layout.spec.ts` (list reporter only) |
| clearance-diagnostic; clearance-repaired | `e2e/workspace-layout.spec.ts --grep 'table strip clears'` |
| source-final | `e2e/workspace-layout.spec.ts e2e/ui-foundation.spec.ts --grep 'workspace preserves|D-72 canvases|routing uses|table strip clears|switch to Table|structural review|split and the drawer|below 1280|task and analysis dock preserve usable canvas'` |
| source-portal-repair | `e2e/ui-foundation.spec.ts e2e/workspace-layout.spec.ts --grep 'task and analysis dock preserve usable canvas|workspace preserves|routing uses|structural review'` |
| source-budget-narrow | `e2e/ui-foundation.spec.ts --grep 'task and analysis dock preserve usable canvas light comfortable 1024x768' --project=chromium-desktop --max-failures=1` |
| source-budget-wide | same as narrow, replacing `1024x768` with `1440x920` |
| dist-budget | `--config playwright.dist.config.ts e2e/ui-foundation-dist.spec.ts --grep 'content-aware narrow canvas budget'` |
| dist-budget-repair | same as dist-budget plus `--max-failures=1` |
| authoring-repair; authoring-repair2 | `e2e/linear-authoring.spec.ts --project=chromium-desktop --max-failures=1` |
| source-verified; source-complete | `e2e/workspace-layout.spec.ts e2e/ui-foundation.spec.ts e2e/linear-authoring.spec.ts --grep 'workspace preserves|D-72 canvases|routing uses|table strip clears|switch to Table|structural review|split and the drawer|below 1280|task and analysis dock preserve usable canvas|compact blank-to-straight' --max-failures=1` |

Other exact commands: `npm run build` twice (`dist-build.log`, `dist-build-portal-repair.log`); `npx vitest run src/features/viewport` twice (`viewport-final.log`, `viewport-final-closeout.log`); `git diff --check` (passed). WASM was already rebuilt by manager before delegation, and no new WASM build is claimed here.

`source-portal-repair` was stopped with SIGINT after the repeated hidden-inspector timeout. `source-complete` was stopped with SIGINT on manager instruction to release the serialized slot for native and the final full lane. Neither is a complete run. The uncompressed report/log SHA-256 and gzip bindings are in `compressed-raw-evidence.json`.
