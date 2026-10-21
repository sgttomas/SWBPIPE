// Renders the production PipeViewport alone over the bundled invented
// reference model and its preserved result rows, so a real browser draws the
// rotation glyphs (DEL-07-05:1, CLM-024 browser evidence). In the app the
// bundled reference is inspection-only and never becomes the viewport's
// result; this harness supplies it directly. `?variant=translational` drops
// the emitted rotation rows and nothing else, for comparison. No new model
// format and no new fixture: both variants read the bundled reference.

import ReactDOM from "react-dom/client";
import { DisplayUnitsProvider } from "../../src/features/display-units";
import { PipeViewport } from "../../src/features/viewport/PipeViewport";
import { buildDeformationOverlay, buildRotationOverlay, rotationGlyphAnchors } from "../../src/features/viewport/deformationOverlay";
import { modelIndexFor } from "../../src/features/workspace/modelIndex";
import { emptySelection } from "../../src/features/workspace/selectionState";
import { loadBundledMechanicsReference } from "../../src/services/previewService";
import { loadWasmEngine } from "../../src/services/wasmEngine/loadWasmEngine";
import "../../src/styles.css";

const ROTATION_KINDS = new Set(["global_nodal_rotation_x", "global_nodal_rotation_y", "global_nodal_rotation_z"]);

async function main() {
  await loadWasmEngine().catch(() => undefined);
  const variant = new URLSearchParams(window.location.search).get("variant") ?? "full";
  const { model, source } = await loadBundledMechanicsReference();
  const result = variant === "translational"
    ? { ...source, results: source.results.filter((row) => !ROTATION_KINDS.has(row.kind)) }
    : source;
  const modelIndex = modelIndexFor(model, 0, 0);
  const assignment = {
    status: "committed" as const,
    generation: 1,
    indexGeneration: modelIndex.generation,
    identityHash: "rotation-glyph-harness",
    startedAt: 0,
    committedAt: 0
  };
  // The spec reads the glyphs the viewport is expected to draw from the same
  // production functions, so it can project a glyph point into the canvas.
  const rotation = buildRotationOverlay(model, result);
  const deformation = buildDeformationOverlay(model, result);
  (window as unknown as { __rotationGlyphHarness: unknown }).__rotationGlyphHarness = {
    variant,
    rotationState: rotation.state,
    glyphs: rotationGlyphAnchors(model, rotation, deformation)
  };
  ReactDOM.createRoot(document.getElementById("root")!).render(
    <DisplayUnitsProvider>
      <div style={{ width: 1100, height: 760, display: "flex" }}>
        <PipeViewport
          model={model}
          modelIndex={modelIndex}
          assignment={assignment}
          result={result}
          selection={{ type: "project", id: model.project.id }}
          onSelect={() => emptySelection()}
        />
      </div>
    </DisplayUnitsProvider>
  );
}

void main();
