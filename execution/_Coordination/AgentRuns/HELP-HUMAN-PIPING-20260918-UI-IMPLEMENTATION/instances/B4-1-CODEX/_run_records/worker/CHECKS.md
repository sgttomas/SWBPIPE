# B4.1 focused verification, source freeze 01

All commands ran in the assigned `projects/chirality-piping/apps/desktop`, serially, with one Vitest worker. Actual Node version is node-version.txt; Vitest version, timestamps/durations and raw output are in each log. No worker build/browser/native/full-suite/Git mutation occurred.

Final candidate checks:

- `npx vitest run src/features/workspace/table src/App.tableOperations.test.tsx src/features/model-tree/ModelTree.test.tsx src/features/toolkit/DisplayIntegration.test.tsx src/App.projectHandlers.test.tsx --maxWorkers=1`: 8 files / 130 tests passed; focused-final-01.txt.
- `npx tsc --noEmit`: exit 0; typescript-final-01.txt (empty output).
- `npx vitest run src/App.test.tsx --maxWorkers=1 -t <exact inventory regex>`: 17 passed, 201 unrelated skipped; exact argv is app-consumer-final-command.json, raw app-consumer-final-01.txt. Includes six parameterized stale/busy ownership cases after audit found they were not in the initial literal-it extraction.

Earlier retained development checks: core-test-01/02; adapter-test-01 (actual WASM user versus explicitly reviewed offline agent semantic equality); controller-test-01; focused-test-03/05; transitions-test-01; focus-repair-test-01; ownership-repair-01. Expected jsdom canvas-unavailable diagnostics appear in App/display cases; no WebGL/native capability claim is inferred from unit passes.

Material failed checks remain: focused-test-06 exposed first-click pointer marker regression, diagnosed in FOCUS_DEFECT_DIAGNOSIS.md, fixed and proven by unchanged regression; ownership-repro-01 exposed invalid editor replacement and stale navigation selection, diagnosed in OWNERSHIP_DEFECT_DIAGNOSIS.md, fixed and proven by unchanged regressions. focused-test-04 failed a new App test's unscoped Apply selector (another existing Apply is accessible); scoped to the primary EngineeringTable. typescript-02 exposed a missing OperationOutcome import; typescript-04 exposed unsupported testing-library `exact` option. Both mechanical errors corrected; final type check passes. No protected existing oracle changed.

Browser plan: manager-owned granted 5174 slot, one worker/shared lock, two configured projects. New e2e/b4-table-editing.spec.ts has one connected primary edit/history/save journey plus one approved 1000-node fixture scroll/filter/invalid-draft/hash-invariance journey per project (4 executions total). Existing shared helper/fixture/instrument bytes unchanged. No repeated explicit-size loop or timed qualification.

Native plan: bounded real WebKit pointer first/second click; Apply/Cancel/Escape; Enter/Tab/caret/up/down; boundary exits and external-focus ownership; actual typed-text Cmd-Z/Shift-Cmd-Z while editing compared with toolbar/model Undo/Redo outside the input. A native-menu fix, if measured, requires separate scope. Source browser evidence cannot establish native text undo.
