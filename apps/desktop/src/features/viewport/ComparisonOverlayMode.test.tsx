// The comparison overlay's read-only mode (DEL-07-08 REQ-011, AC-003, AC-006; DEL-07-06 REQ-011):
// the legend's words and counts read from the accessibility tree, each class's signal besides
// colour, activation of a marked entity by keyboard and pointer, the way out, and the authoring
// viewport beneath left untouched. All data are invented.
import { cleanup, fireEvent, render, screen, within } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";
import { compareModelDocuments } from "../comparison/stateComparison";
import { comparisonOverlayScene, STATE_OVERLAY_CLASSES, RUN_OVERLAY_CLASSES, stateOverlayMarks } from "../comparison/comparisonOverlay";
import { ComparisonOverlayMode, projectOverlayPoint } from "./ComparisonOverlayMode";
import { DisplayUnitsProvider } from "../display-units";
import { loadPreviewModel } from "../../services/previewService";
import { PipeViewport } from "./PipeViewport";

type Json = any; // eslint-disable-line @typescript-eslint/no-explicit-any

afterEach(cleanup);

function document(): Json {
  return {
    schema_version: "0.9.0",
    document_kind: "model",
    project: { id: "project:invented", name: "Invented", description: "", units: { length: "m" } },
    data_boundary: {},
    analysis_status: {},
    materials: [{ id: "material:M-1", label: "Steel", elastic_modulus: { value: 200, unit: "GPa" }, provenance: "invented" }],
    nodes: [
      { id: "node:N-1", label: "A", position: { x: 0, y: 0, z: 0 }, provenance: "invented" },
      { id: "node:N-2", label: "B", position: { x: 2, y: 0, z: 0 }, provenance: "invented" },
      { id: "node:N-3", label: "C", position: { x: 2, y: 1, z: 0 }, provenance: "invented" }
    ],
    pipe_segments: [{ id: "pipe:P-1", label: "P", from: "node:N-1", to: "node:N-2", section: {}, material: "material:M-1", provenance: "invented" }],
    supports: [{ id: "support:S-1", label: "Anchor", node: "node:N-1", provenance: "invented" }]
  };
}
const LEFT = document();
const RIGHT = (() => {
  const model = document();
  model.nodes = model.nodes.filter((node: Json) => node.id !== "node:N-3");
  model.nodes.push({ id: "node:N-4", label: "D", position: { x: 4, y: 0, z: 0 }, provenance: "invented" });
  model.supports[0].label = "Anchor, relabelled";
  model.materials[0].elastic_modulus = { value: 210, unit: "GPa" };
  return model;
})();
const SCENE = comparisonOverlayScene("states", stateOverlayMarks(compareModelDocuments(LEFT, RIGHT)), LEFT, RIGHT);

function mount(overrides: Partial<Parameters<typeof ComparisonOverlayMode>[0]> = {}) {
  const props = { scene: SCENE, leftText: "\"Basis\" (state:1)", rightText: "\"Revision\" (state:2)", onLeave: vi.fn(), onActivate: vi.fn(), ...overrides };
  return { props, ...render(<ComparisonOverlayMode {...props} />) };
}

describe("the comparison overlay mode", () => {
  it("states in words, in the accessibility tree, each class with its count and what is not drawn", () => {
    mount();
    const legend = screen.getByRole("region", { name: "Comparison overlay legend" });
    // The class's sample line and glyph are hidden from the accessibility tree; the words carry it.
    const items = within(legend).getAllByRole("listitem").map((item) => item.querySelector(":scope > span")!.textContent);
    for (const sample of legend.querySelectorAll("svg")) expect(sample).toHaveAttribute("aria-hidden", "true");
    expect(items).toEqual([
      "Added (glyph +, solid line): 1 entity",
      "Removed (glyph −, dashed line): 1 entity",
      // The support, the material and the model entity (its ordered id lists) are changed.
      "Changed (glyph Δ, dash-dot line): 3 entities, 2 not drawn",
      "Unchanged (glyph =, thin line): 3 entities",
      "Ghost (faint, its class's line): an entity only in the earlier saved state, in its own position there."
    ]);
    expect(within(legend).getByTestId("comparison-overlay-not-drawn")).toHaveTextContent(
      "2 classified entities are not drawn (Changed: 2): an entity the model drawing does not show (the model, a material, a section, a load case, a setting or a record). They are listed in the comparison table."
    );
  });

  it("gives each class a glyph and a line style of its own besides its colour, on the legend and on the drawing", () => {
    for (const classes of [STATE_OVERLAY_CLASSES, RUN_OVERLAY_CLASSES]) {
      expect(new Set(classes.map((style) => style.glyph)).size).toBe(classes.length);
      expect(new Set(classes.map((style) => style.line)).size).toBe(classes.length);
    }
    mount();
    for (const style of STATE_OVERLAY_CLASSES) {
      const item = screen.getByTestId(`comparison-overlay-legend-${style.markClass}`);
      expect(item).toHaveAttribute("data-line", style.line);
      expect(item).toHaveAttribute("data-glyph", style.glyph);
    }
    const drawn = (ref: string) => screen.getByTestId(`comparison-overlay-entity-${ref}`);
    expect(drawn("node:N-4")).toHaveAttribute("data-line", "solid");
    expect(drawn("node:N-4")).toHaveAttribute("data-glyph", "+");
    expect(drawn("node:N-3")).toHaveAttribute("data-line", "dashed");
    expect(drawn("node:N-3")).toHaveAttribute("data-glyph", "−");
    expect(drawn("node:N-3")).toHaveAttribute("data-ghost", "true");
    expect(drawn("support:S-1")).toHaveAttribute("data-line", "dash-dot");
    expect(drawn("pipe:P-1")).toHaveAttribute("data-line", "thin");
    // Each marked entity is named in words, its class included.
    expect(screen.getByRole("button", { name: "Node node:N-3, only in the earlier saved state, drawn as a ghost: Removed" })).toBe(drawn("node:N-3"));
    expect(screen.getByRole("button", { name: "Support support:S-1: Changed" })).toBe(drawn("support:S-1"));
  });

  it("asks for a marked entity's row when it is activated by Enter, Space or pointer", () => {
    const { props } = mount();
    const removed = screen.getByRole("button", { name: /Node node:N-3/u });
    fireEvent.keyDown(removed, { key: "Enter" });
    expect(props.onActivate).toHaveBeenLastCalledWith("node:N-3", "node");
    fireEvent.keyDown(screen.getByRole("button", { name: /Support support:S-1/u }), { key: " " });
    expect(props.onActivate).toHaveBeenLastCalledWith("support:S-1", "support");
    fireEvent.click(screen.getByRole("button", { name: /Pipe segment pipe:P-1/u }));
    expect(props.onActivate).toHaveBeenLastCalledWith("pipe:P-1", "pipe_segment");
    expect(props.onActivate).toHaveBeenCalledTimes(3);
  });

  it("names both sides, says it is not the working model, and offers the way out by button and by Escape", () => {
    const { props } = mount();
    const banner = screen.getByTestId("comparison-overlay-banner");
    expect(banner).toHaveTextContent("Comparison overlay. This is not the working model.");
    expect(banner).toHaveTextContent("It draws the later saved state, \"Revision\" (state:2), compared with the earlier saved state, \"Basis\" (state:1).");
    fireEvent.click(screen.getByRole("button", { name: "Leave the comparison overlay" }));
    expect(props.onLeave).toHaveBeenCalledTimes(1);
    fireEvent.keyDown(screen.getByTestId("comparison-overlay-drawing"), { key: "Escape" });
    expect(props.onLeave).toHaveBeenCalledTimes(2);
  });

  it("presents no comparison as validation, approval or compliance", () => {
    mount();
    expect(screen.getByTestId("comparison-overlay").textContent).not.toMatch(/valid|approv|complian|certif|accept|verif|pass|fail|correct/iu);
    cleanup();
    mount({ scene: comparisonOverlayScene("runs", [], LEFT, RIGHT) });
    expect(screen.getByTestId("comparison-overlay").textContent).not.toMatch(/valid|approv|complian|certif|accept|verif|pass|fail|correct/iu);
  });

  it("projects with the authoring viewport's presets, Y up", () => {
    expect(projectOverlayPoint({ x: 1, y: 2, z: 3 }, "front")).toEqual({ x: 1, y: -2 });
    expect(projectOverlayPoint({ x: 1, y: 2, z: 3 }, "top")).toEqual({ x: 1, y: 3 });
    const iso = projectOverlayPoint({ x: 0, y: 1, z: 0 }, "iso");
    expect(iso.x).toBeCloseTo(0);
    expect(iso.y).toBeLessThan(0);
  });
});

describe("the authoring viewport beneath the overlay", () => {
  it("is hidden and inert while the overlay shows, and its model and selection are unchanged across showing and leaving it", async () => {
    const model = await loadPreviewModel();
    const before = JSON.stringify(model);
    const onSelect = vi.fn();
    const selection = { type: "project" as const, id: model.project.id };
    const viewport = (active: boolean) => (
      <DisplayUnitsProvider>
        {active ? <ComparisonOverlayMode scene={SCENE} leftText="a" rightText="b" onLeave={vi.fn()} onActivate={vi.fn()} /> : null}
        <PipeViewport model={model} selection={selection} onSelect={onSelect} comparisonOverlayActive={active} />
      </DisplayUnitsProvider>
    );
    const view = render(viewport(false));
    const shell = view.container.querySelector(".viewport-shell")!;
    expect(shell).not.toHaveAttribute("inert");
    view.rerender(viewport(true));
    // The same element: the viewport was not remounted.
    expect(view.container.querySelector(".viewport-shell")).toBe(shell);
    expect(shell).toHaveAttribute("inert");
    expect(shell).toHaveAttribute("aria-hidden", "true");
    fireEvent.keyDown(screen.getByRole("button", { name: /Node node:N-3/u }), { key: "Enter" });
    view.rerender(viewport(false));
    expect(view.container.querySelector(".viewport-shell")).toBe(shell);
    expect(shell).not.toHaveAttribute("inert");
    expect(onSelect).not.toHaveBeenCalled();
    expect(JSON.stringify(model)).toBe(before);
  });
});
