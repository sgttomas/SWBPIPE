# UI writer checkpoint 2 foundation evidence

Source commit: `817a20eb28fdbd3427ac5b4be176ab5ccce27645`

Runtime authority: `RUNTIME_LEASE_UI_CHECKPOINT2_V1`, SHA-256 `d7dae8cab2260c1802c96fb28b091a18cbf863a237dd6cb80ebf200626aa53bb`.

All commands ran from `projects/chirality-piping`. Raw stdout/stderr is retained without editing in the numbered files in this directory.

| Attempt | Command | Exit | Result |
| --- | --- | ---: | --- |
| 01 | `npm run build:wasm:desktop` | 0 | Both authorized WASM prerequisites generated; artifacts remain intentionally uncommitted. |
| 02 | `npm exec --workspace apps/desktop tsc -- -b --pretty false` | 1 | Found closure narrowing and a missing test helper name. |
| 03 | same TypeScript command after repair | 0 | Passed. |
| 04 | focused Vitest with mistakenly root-prefixed paths | 1 | No files matched the workspace-relative Vitest root. |
| 05 | corrected six-suite focused Vitest | 1 | 31 passed; one source-integration fixture lacked the scheduler needed by the real replacement path. |
| 06 | TypeScript after fixture repair | 0 | Passed. |
| 07 | corrected six-suite focused Vitest after fixture repair | 0 | 32 passed. |
| 08 | ROOT-maintained App lifecycle/guard selection | 1 | 20 passed; one historical-report assertion had not activated the newly lazy report section. |
| 09 | same App lifecycle/guard selection after activating the report section | 0 | 21 passed; 165 skipped by the exact name filter. |
| 10 | final TypeScript binding | 0 | Passed with empty output. |
| 11 | final corrected six-suite focused Vitest binding | 0 | 6 files and 32 tests passed. |

The final focused command was:

`npm exec --workspace apps/desktop vitest -- run src/features/workspace/dormantSection.test.tsx src/features/workspace/selectionState.test.ts src/features/workspace/modelIndex.test.ts src/features/workspace/uiDiagnostics.test.ts src/features/viewport/viewportSelection.test.ts src/features/viewport/viewportResource.test.ts`

The exact App guard command is preserved at the start of `09-app-guards-retry1-exit-0.txt`.

`SOURCE_SHA256.txt` binds each source path in the source commit. `SOURCE_DIFF_STAT.txt` records its aggregate. `SHA256SUMS` binds this package and excludes itself.

Limitations: these are TypeScript and jsdom unit/integration results. The App guard log includes jsdom's expected `HTMLCanvasElement.getContext` warnings while the guarded single-renderer construction falls back visibly. This package makes no real-WebGL, browser, native, compositor, performance, accessibility, or usability claim. ROOT/NATIVE and VERIFY retain those witnesses.
