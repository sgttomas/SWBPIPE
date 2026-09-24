# Sweep repair checks and replay

Use repository-relative package root `projects/chirality-piping/apps/desktop`. Raw logs, browser report and exact machine/temporary paths are classified under `_run_records`; `CANDIDATE.json` binds the three repaired source/test files. Node 24.18.0, npm 11.16.0, Vitest 4.1.10, TypeScript 5.9.3, Playwright 1.60.0 and Chrome 153.0.8010.54 were reused. No dependency/native bundle/build or OS setting changed.

Diagnosis commands and outputs are identified in `DIAGNOSIS_RETURN.md` and `_run_records/diagnosis-commands.txt`. Diagnosis preceded repair activation and established separate queue-marker and absent-native-feature causes.

Activated verification:

- `npx vitest run src/App.test.tsx -t 'queues layout-grid cell edits as structured review intents' --reporter=verbose`: one passed, with unchanged feedback and both structured payload assertions.
- `npx vitest run src/App.projectHandlers.test.tsx -t 'records both snapshot observations|keeps both recorded mismatches when a model edit|blocks later (missing|failed) Open until landed B' --reporter=verbose`: all five reproducing cases passed; no unhandled errors.
- `npx vitest run src/features/model-tree/ModelTree.test.tsx src/features/workspace/table/EngineeringTable.test.tsx --reporter=verbose`: 68 passed, no unhandled errors. This followed the final product guard/marker changes.
- `npx tsc -b`: first found an unsupported Testing Library `exact` option in a new assertion. That test-only option was removed; string accessible-name matching remains exact. Final TypeScript passed.
- `npx vitest run src/features/workspace/table/EngineeringTable.test.tsx -t 'keeps complete compact feedback|uses native closed/open|does not swallow a genuine' --reporter=verbose`: the final three new/updated unit cases passed after the type correction. The supported-error case captures only its exact deliberate sentinel after it escapes the product handler; unrelated errors keep the runner's normal handling.
- Browser command selects `Details owns traversed|classic scrollbar compact (comfortable|compact) 5 ` from `e2e/b4-table-editing.spec.ts`, using the unchanged desktop config and existing classic worker fixture. Four passed. It checks real supported Chromium popovers, separate from jsdom's absent API and from native WebKit/Tauri. The process launched immediately before a later reservation-notification hold arrived; manager/ROOT were informed truthfully and acknowledged continuing the already-authorized run. No later browser run was launched.

Port 5174 was verified free before Vite, then reserved for this sole browser run. It was stopped afterward; 5174 and 5175 had no listener. No native/CUA action occurred. Parent owns the next clean sweep and any native witness.
