# Generic native provenance hook

Implementation: `apps/desktop/src/services/previewService.ts` in the joined checkout. No production registrar or serialized proof flag is exported. This registration establishes actual direct/native-job delivery and unchanged content/model/mode only; existing method/NUM/physics and model/input/run/build/status guards still decide eligibility.

Public queries:

```ts
type NativeMechanicsInvocation = {
  request: { model: PreviewModel; materials: [] };
  solver_mode: PreviewSolverMode;
};
hasNativeMechanicsInvocation(source: MechanicsResult | null | undefined,
  model: PreviewModel | null | undefined, solverMode?: string): boolean;
retainedNativeMechanicsInvocation(source: MechanicsResult,
  model: PreviewModel): NativeMechanicsInvocation | null;
```

The retained helper returns a fresh clone only after matching the still-registered source and supplied model. It cannot register a source. A clone/reparse/history import has no WeakMap entry. Context model follows the exact checked JSON normalization sent to the native command, with no material override guessed from results. Missing explicit input context may retain raw native diagnostics but cannot register model-bound standing.

Private `captureNativeInvocation` captures `{request:{model,materials:[]},solver_mode}` before actual dispatch and sends that captured model. Its private wrapper retains an immutable-content fingerprint plus invalidation/terminal-claim lifecycle state. Private async `validateCapturedSource` verifies actual model identity, checked full raw content, signed-zero positions and stability across checked canonical hashing before installing the WeakMap registration. It is called only following direct native IPC or one terminal completion of a locally known started job. Failed/cancelled/unknown job responses cannot register; accepted cancellation invalidates in-flight capture, including across the async validation window. No result field/header/quality/status is changed.

NUM's method-specific source-block validation should compose inside this same `validateCapturedSource` hook before generic registration, using `capture.invocation`. It remains additional to generic native provenance and does not admit physics1 or another table. There is no source-block receipt/table join here.

`runPreviewMechanics` and `startPreviewMechanicsJob` refuse browser execution, including the unchanged/omitted bundled model. `loadBundledMechanicsReference(mode)` explicitly returns `{standing:'reference_only',source,provenance}` with unchanged raw bytes/values and external provenance; it never registers. The legacy browser receipt vocabulary remains readable, but workspace publication rejects that receipt and has no fixture execution fallback.

Current session state, qualified result export and rule entrypoints use the query. All preceding semantic, model, numeric and integrity checks remain. Analysis-record construction remains a pure reference-capable operation and does not mint native standing. Parent owns final native UI witness; unit replay helpers deliberately simulate transport using exact emitted full-UI model/result pairs and never alter producer values or quality.
