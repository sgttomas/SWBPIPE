// Renders the production PipeViewport alone over the bundled invented reference model and its
// preserved result rows, so a real browser draws the result overlays (OD-5, FR-GUI-001): result
// colour on the tubes and the support reaction arrows. In the app the bundled reference is
// inspection-only and never becomes the viewport's result; this harness supplies it directly,
// with the session's own standing path (the reader's numerical standing and the numerical health
// view). `?mode=dense_scrutiny` reads the other bundled solver-mode record; `?theme=dark` paints
// the dark theme. No new model format and no new fixture.

import ReactDOM from "react-dom/client";
import { DisplayUnitsProvider } from "../../src/features/display-units";
import { numericalHealthView, readNumericalHealth } from "../../src/features/results/numericalHealth";
import { numericalResultStanding } from "../../src/features/results/resultStanding";
import { PipeViewport } from "../../src/features/viewport/PipeViewport";
import { buildDeformationOverlay, buildRotationOverlay, rotationGlyphAnchors } from "../../src/features/viewport/deformationOverlay";
import { buildReactionOverlay } from "../../src/features/viewport/reactionOverlay";
import {
  availableResultColourQuantities,
  buildResultColourOverlay,
  resultColourTable,
  resultOverlayCases
} from "../../src/features/viewport/resultColourOverlay";
import { modelIndexFor } from "../../src/features/workspace/modelIndex";
import { entityKey, emptySelection } from "../../src/features/workspace/selectionState";
import { loadBundledMechanicsReference } from "../../src/services/previewService";
import { loadWasmEngine } from "../../src/services/wasmEngine/loadWasmEngine";
import "../../src/styles.css";

async function main() {
  await loadWasmEngine().catch(() => undefined);
  const params = new URLSearchParams(window.location.search);
  const mode = params.get("mode") === "dense_scrutiny" ? "dense_scrutiny" : "sparse_interactive";
  const theme = params.get("theme") === "dark" ? "dark" : "light";
  document.documentElement.dataset.theme = theme;
  const { model, source: result } = await loadBundledMechanicsReference(mode);
  const modelIndex = modelIndexFor(model, 0, 0);
  const assignment = {
    status: "committed" as const,
    generation: 1,
    indexGeneration: modelIndex.generation,
    identityHash: "result-overlay-harness",
    startedAt: 0,
    committedAt: 0
  };
  const standing = numericalResultStanding(result, model);
  const resultStanding = {
    eligible: standing.eligible,
    health: numericalHealthView(readNumericalHealth(result, standing.findings), { eligible: standing.eligible, verdictHold: false })
  };
  // The spec reads what the viewport is expected to draw from the same production functions, for
  // the default quantity and case, so it can project points and check drawn pixels.
  const overlayCase = resultOverlayCases(model, result)[0] ?? null;
  const quantity = availableResultColourQuantities(model, result)[0] ?? null;
  const colour = buildResultColourOverlay({ model, result, standing: resultStanding, quantity, overlayCase });
  const table = resultColourTable(model, colour);
  const reactions = buildReactionOverlay({ model, result, standing: resultStanding, overlayCase });
  const deformation = buildDeformationOverlay(model, result);
  const rotation = buildRotationOverlay(model, result);
  const supportChunk = (supportId: string) => modelIndex.spatialChunks.findIndex((chunk) => chunk.entityKeys.includes(entityKey({ type: "support", id: supportId })));
  const nodes = new Map(model.nodes.map((node) => [node.id, node.position] as const));
  const drawn = (vectors: typeof reactions.forces) => [...vectors.values()].filter((item) => item.displayLength > 0);
  (window as unknown as { __resultOverlayHarness: unknown }).__resultOverlayHarness = {
    mode,
    theme,
    eligible: standing.eligible,
    healthHeadline: resultStanding.health?.headline ?? null,
    quantityLabel: quantity?.label ?? null,
    caseLabel: overlayCase?.label ?? null,
    colourState: colour.state,
    noValueCount: colour.noValueCount,
    memberCount: colour.memberCount,
    pipes: model.pipe_segments.map((pipe) => ({
      id: pipe.id,
      from: nodes.get(pipe.from),
      to: nodes.get(pipe.to),
      colour: table?.get(entityKey({ type: "pipe", id: pipe.id }))?.[theme] ?? null
    })),
    nodes: model.nodes.map((node) => node.position),
    // What else the viewport draws near the tubes, so the spec can sample a tube point clear of it.
    deformedPipes: deformation.state === "available"
      ? model.pipe_segments.flatMap((pipe) => {
        const from = deformation.nodePositions.get(pipe.from);
        const to = deformation.nodePositions.get(pipe.to);
        return from && to ? [{ from, to }] : [];
      })
      : [],
    rotationGlyphs: rotationGlyphAnchors(model, rotation, deformation).map((glyph) => ({
      from: glyph.anchor,
      to: { x: glyph.anchor.x + glyph.axis.x * glyph.displayLength, y: glyph.anchor.y + glyph.axis.y * glyph.displayLength, z: glyph.anchor.z + glyph.axis.z * glyph.displayLength }
    })),
    reactionState: reactions.state,
    withheldCount: reactions.withheld.length,
    forces: drawn(reactions.forces).map((item) => ({ supportId: item.supportId, anchor: nodes.get(item.nodeId), direction: item.direction, displayLength: item.displayLength })),
    moments: drawn(reactions.moments).map((item) => ({ supportId: item.supportId, anchor: nodes.get(item.nodeId), direction: item.direction, displayLength: item.displayLength })),
    forceChunks: new Set(drawn(reactions.forces).map((item) => supportChunk(item.supportId))).size,
    momentChunks: new Set(drawn(reactions.moments).map((item) => supportChunk(item.supportId))).size
  };
  ReactDOM.createRoot(document.getElementById("root")!).render(
    <DisplayUnitsProvider>
      <div style={{ width: 1100, height: 760, display: "flex" }}>
        <PipeViewport
          model={model}
          modelIndex={modelIndex}
          assignment={assignment}
          result={result}
          resultStanding={resultStanding}
          selection={{ type: "project", id: model.project.id }}
          onSelect={() => emptySelection()}
          theme={theme}
        />
      </div>
    </DisplayUnitsProvider>
  );
}

void main();
