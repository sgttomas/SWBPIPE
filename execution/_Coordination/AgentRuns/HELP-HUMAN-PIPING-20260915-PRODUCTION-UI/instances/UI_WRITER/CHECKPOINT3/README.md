# UI writer checkpoint 3 focused compile evidence

Runtime authority: `RUNTIME_LEASE_UI_CHECKPOINT3_V1`, SHA-256 `25dd136d4e815295ea1fa92a15bea0d1757b0c84e185ea7e0cd5da665ce311f3`.

All commands ran from `projects/chirality-piping`. Raw stdout/stderr is retained in the numbered files here.

| Attempt | Command | Exit | Result |
| --- | --- | ---: | --- |
| 01 | `npm run build:wasm:desktop` | 0 | Required operation/self-weight WASM artifacts generated and intentionally left uncommitted. |
| 02 | `npm exec --workspace apps/desktop tsc -- -b --pretty false` | 0 | TypeScript passed. |
| 03 | focused ten-suite Vitest command | 1 | 42 passed; the new virtual-tree test incorrectly expected an off-window collision row to be mounted. |
| 04 | same focused command after filtering to the collision witness | 0 | 10 files and 43 tests passed. |
| 05 | final TypeScript command | 0 | Passed after the test repair. |

The focused command was:

`npm exec --workspace apps/desktop vitest -- run src/features/workspace/uiPreferences.test.ts src/features/workspace/VirtualList.test.tsx src/features/workspace/VirtualTargetPicker.test.tsx src/features/workspace/selectionState.test.ts src/features/workspace/modelIndex.test.ts src/features/workspace/uiDiagnostics.test.ts src/features/viewport/viewportSelection.test.ts src/features/viewport/viewportResource.test.ts src/features/model-workspace/modelView.test.ts src/features/model-tree/ModelTree.test.tsx`

This package supports only TypeScript and focused jsdom/unit source behavior. It makes no browser, native, WebGL, performance, accessibility, or usability claim. Full workspace source remains in progress after this checkpoint.

`VITEST_CONFIG_SNAPSHOT.ts` is the exact registered Vite/Vitest configuration used by the focused command. `SOURCE_SHA256.txt` binds every modified or newly created desktop source file at the final successful TypeScript state. `SOURCE_DIFF_STAT.txt` records the tracked-source aggregate at that state; newly created source paths are individually bound by `SOURCE_SHA256.txt`. `OWNED_PROCESS_CHECK.txt` records the post-run process check. `SHA256SUMS` binds every other file in this evidence package.
