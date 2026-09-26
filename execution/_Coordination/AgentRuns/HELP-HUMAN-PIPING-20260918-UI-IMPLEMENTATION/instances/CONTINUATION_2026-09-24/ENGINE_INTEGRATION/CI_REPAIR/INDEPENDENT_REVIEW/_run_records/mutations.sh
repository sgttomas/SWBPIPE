#!/usr/bin/env bash
# Independent-review mutation set. Run from the repository root. Each call restores and verifies.
set -u
export PATH=/opt/node24/bin:$PATH PLAYWRIGHT_CHROMIUM_EXECUTABLE_PATH=/opt/pw-browsers/chromium-1194/chrome-linux/chrome CI=true PLAYWRIGHT_WORKERS=1
M="python3 $(dirname "$0")/mutate.py"
P="projects/chirality-piping"
D="$P/apps/desktop"
PW="--project=chromium-desktop --workers=1 --timeout=180000 --reporter=list"
$M m1-css-base "$D/src/styles.css" "@blob:5b1ccd356" "" $PW e2e/ui-foundation.spec.ts:677 -g "light comfortable 1280x800" || exit 3
$M m2-chip-source-dropped "$D/src/features/workspace/shellLayout.ts" 'SOLVER_NOT_SOLVED, source: "Solve job state" }' 'SOLVER_NOT_SOLVED }' $PW e2e/gui-workflow-validation.spec.ts:108 || exit 3
$M m2b-popover-source-span-dropped "$D/src/features/workspace/shell/ShellStatusBar.tsx" '{chip.source ? <span>{chip.source}</span> : null}' '{null}' $PW e2e/linear-authoring.spec.ts:60 || exit 3
$M m3-design-workspace-reference-leak "$D/src/App.tsx" $'<DesignWorkspacePanel\n                model={model}\n                knowledge={knowledge}\n                result={currentSolvedResult}' $'<DesignWorkspacePanel\n                model={model}\n                knowledge={knowledge}\n                result={currentSolvedResult ?? historicalRun?.mechanicsResult ?? null}' $PW e2e/r2-smoke.spec.ts:351 || exit 3
$M m4-report-reference-leak "$D/src/App.tsx" $'<ReportPanel\n                model={model}\n                knowledge={knowledge}\n                result={currentSolvedResult}' $'<ReportPanel\n                model={model}\n                knowledge={knowledge}\n                result={currentSolvedResult ?? historicalRun?.mechanicsResult ?? null}' $PW e2e/r2-smoke.spec.ts:351 || exit 3
$M m5-issues-reference-leak "$D/src/App.tsx" $'<IssuesHome\n          model={model}\n          knowledge={knowledge}\n          result={result}' $'<IssuesHome\n          model={model}\n          knowledge={knowledge}\n          result={result ?? historicalRun?.mechanicsResult ?? null}' $PW e2e/r2-smoke.spec.ts:1703 || exit 3
$M m6a-results-unit-display-converted "$D/src/features/results/ResultsPanel.tsx" 'const resultUnits = [...new Set(result.results.map((item) => item.unit))].sort(compareResultUnits);' 'const resultUnits = [...new Set(result.results.map((item) => item.unit === "N/m" ? "kN/m" : item.unit))].sort(compareResultUnits);' $PW e2e/r2-smoke.spec.ts:351 || exit 3
$M m6b-fixture-spring-row-converted "$P/fixtures/product_preview/invented_mechanics_result_precision_1_sparse.json" $'"id": "result:spring-hanger:support-SH-140:stiffness",\n      "kind": "spring_hanger_user_input_review",\n      "value": 42000.0,\n      "unit": "N/m",' $'"id": "result:spring-hanger:support-SH-140:stiffness",\n      "kind": "spring_hanger_user_input_review",\n      "value": 42.0,\n      "unit": "kN/m",' $PW e2e/r2-smoke.spec.ts:66 || exit 3
git status --porcelain
git diff --quiet HEAD && echo "git diff --quiet HEAD: clean"
