# UI foundation observer contract binding V1

Status: **FROZEN VERIFIER BINDING**

The candidate verifier binds the read-only product attachment
`OBSERVABILITY_ATTACHMENT_V1.md`, SHA-256
`ef009d11139fc5dab4639586092a44741ced49ac80d7108f22eecd264a6b5c3a`.
The authoritative attachment is held in the governed UI run evidence. This
file records the verifier's fail-closed use of it; it does not add a product
API.

The exact global is the product-installed, non-writable, non-configurable own
data property `globalThis.__openPipeStressUiDiagnosticsV1`. The value and all
returned objects and arrays are recursively frozen. Its only members are:

```ts
{
  schema: "openpipestress.ui-diagnostics/v1";
  readCurrent(): DiagnosticsSnapshot;
  projectAuthoredPoint(request: ProjectionRequest): ProjectionResult;
}
```

The verifier does not install, enable, mutate, subscribe to, or retain history
through this surface. Before first assignment, `readCurrent()` must return the
specified idle snapshot and never throw. `model.generation` is a dedicated
monotonic numeric App accepted-publication counter. It advances for every
accepted model publication, including edits, normalization, same-ID reopen,
and ABA. `assignment.generation` and projection requests/results bind that
number. `indexGeneration` is an opaque string, `projectSessionGeneration` is a
separate number, and `identityHash` is an opaque string; the verifier never
parses or substitutes any of them for `model.generation`.

An in-progress assignment exposes its reserved next accepted generation only
in assignment state. The current accepted model, viewport, and projection
remain bound to the previous generation until commit. An abandoned pending
assignment does not alter current-model projection.

The projection request is exactly:

```ts
type ProjectionRequest = Readonly<{
  modelGeneration: number;
  cameraSequence: number;
  authoredPoint: Readonly<{x:number; y:number; z:number}>;
}>;
```

The recursively frozen result has no extra fields:

```ts
type ProjectionResult =
  | {
      status: "available";
      modelGeneration: number;
      cameraSequence: number;
      canvasCss: {width:number; height:number};
      canvasDevice: {width:number; height:number};
      localPoint: {x:number; y:number; z:number};
      clip: {x:number; y:number; z:number; w:number};
      ndc: {x:number; y:number; z:number};
      canvasCssPoint: {x:number; y:number};
      insideClosedNdc: boolean;
      insideCanvasCss: boolean;
    }
  | {
      status: "stale";
      requested: {modelGeneration:number; cameraSequence:number};
      current: {modelGeneration:number|null; cameraSequence:number|null};
    }
  | {
      status: "invalid";
      reason:
        | "NON_FINITE_AUTHORED_POINT"
        | "NON_FINITE_PROJECTION"
        | "ZERO_CLIP_W"
        | "NO_CURRENT_MODEL"
        | "NO_CURRENT_CAMERA";
    };
```

Projection is a pure authored-to-local camera projection. It never fits,
mutates, looks up an entity, picks, or reports an expected hit. Candidate
qualification fails on an absent or mutable global/result, schema or union
drift, non-finite available values, stale generation/camera, mismatched canvas,
or missing action- and generation-associated render submission.

`mainRender.submittedAt` and copied `rendererInfo` must be published together
only after the actual `renderer.render(...)` call returns. The optional app
paint-opportunity RAF is bound to the latest generation and submission,
tracked, cancellable, and unable to publish after supersession. It is a paint
opportunity rather than compositor-presentation proof. CDP compositor evidence
and retained browser capture remain separate verifier evidence.

Frozen point-oracle rulings used with this observer:

- Local render origin is the per-axis midpoint of the bounds union of all N+1
  finite authored node positions. Marker radii, arrows, labels, OD envelopes,
  results, and visibility masks are excluded.
- The independent oracle exhaustively evaluates V3 analytic primitives with
  conservative per-primitive AABB rejection, global minimum, and V3 tie rule.
  It deliberately does not emulate product chunk partitioning.
- One separate untimed preflight uses only real Fit/preset controls and freezes
  camera/canvas expectations before five fresh measured sessions. Every
  measured session revalidates its actual camera, frustum, and canvas.
