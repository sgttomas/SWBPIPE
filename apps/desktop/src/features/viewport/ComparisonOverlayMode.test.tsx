// The comparison overlay's read-only mode (DEL-07-08 REQ-011, AC-003, AC-006; DEL-07-06 REQ-011):
// the legend's words and counts read from the accessibility tree, each class's signal besides
// colour, activation of a marked entity by keyboard and pointer, the way out, and the authoring
// viewport beneath left untouched. All data are invented.
import { cleanup, fireEvent, render, screen, within } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";
import { compareModelDocuments } from "../comparison/stateComparison";
import { comparisonOverlayScene, STATE_OVERLAY_CLASSES, RUN_OVERLAY_CLASSES, stateOverlayMarks } from "../comparison/comparisonOverlay";
import { ComparisonOverlayMode, projectOverlayPoint, upright } from "./ComparisonOverlayMode";
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
      "Unchanged (glyph =, thin line): 3 entities; drawn as context, not marked",
      "Ghost (faint, labelled “earlier saved state only”): an entity only in the earlier saved state, in its own position there."
    ]);
    expect(within(legend).getByTestId("comparison-overlay-drawn")).toHaveTextContent("3 marked entities are drawn above 3 context entities.");
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

  it("makes only marked entities keyboard stops; unchanged ones are context, drawn beneath every mark", () => {
    mount();
    const drawn = (ref: string) => screen.getByTestId(`comparison-overlay-entity-${ref}`);
    for (const ref of ["node:N-1", "node:N-2", "pipe:P-1"]) {
      expect(drawn(ref)).toHaveAttribute("data-marked", "false");
      expect(drawn(ref)).not.toHaveAttribute("tabindex");
      expect(drawn(ref)).not.toHaveAttribute("role");
    }
    expect(screen.getAllByRole("button", { name: /^(Node|Support|Pipe segment) /u }).map((element) => element.dataset.testid)).toEqual([
      "comparison-overlay-entity-node:N-3",
      "comparison-overlay-entity-node:N-4",
      "comparison-overlay-entity-support:S-1"
    ]);
    for (const ref of ["node:N-3", "node:N-4", "support:S-1"]) expect(drawn(ref)).toHaveAttribute("tabindex", "0");
    // Painting order is document order: every mark after every context entity, ghosts before live marks.
    const order = Array.from(screen.getByTestId("comparison-overlay-drawing").querySelectorAll<SVGGElement>(".comparison-overlay-entity")).map((element) => element.dataset.marked);
    expect(order.lastIndexOf("false")).toBeLessThan(order.indexOf("true"));
    // Each mark sits on a halo, so no line drawn earlier shows through it.
    expect(drawn("support:S-1").querySelector(".comparison-overlay-halo")).not.toBeNull();
    expect(drawn("pipe:P-1").querySelector(".comparison-overlay-halo")).toBeNull();
  });

  it("labels each marked entity with its identifier, a ghost as earlier only, and gives every entity a title", () => {
    mount();
    const label = (ref: string) => screen.getByTestId(`comparison-overlay-entity-${ref}`).querySelector(".comparison-overlay-label")?.textContent;
    expect(label("node:N-4")).toBe("node:N-4");
    expect(label("node:N-3")).toBe("node:N-3 (earlier saved state only)");
    expect(label("pipe:P-1")).toBeUndefined();
    expect(screen.getByTestId("comparison-overlay-entity-pipe:P-1").querySelector("title")).toHaveTextContent("Pipe segment pipe:P-1: Unchanged");
    fireEvent.click(screen.getByTestId("comparison-overlay-labels"));
    expect(label("node:N-4")).toBeUndefined();
  });

  it("draws a member seen end on as a point, and stacks the glyphs of marks that coincide in a view", () => {
    const left = document();
    const right = document();
    // A riser along Y, added; two changed supports on one node.
    right.nodes.push({ id: "node:N-UP", label: "Up", position: { x: 2, y: 3, z: 0 }, provenance: "invented" });
    right.pipe_segments.push({ id: "pipe:P-UP", label: "Riser", from: "node:N-2", to: "node:N-UP", section: {}, material: "material:M-1", provenance: "invented" });
    left.supports.push({ id: "support:S-2", label: "Guide", node: "node:N-1", provenance: "invented" });
    right.supports = [{ ...right.supports[0], label: "Anchor, moved" }, { id: "support:S-2", label: "Guide, changed", node: "node:N-1", provenance: "invented" }];
    mount({ scene: comparisonOverlayScene("states", stateOverlayMarks(compareModelDocuments(left, right)), left, right) });
    const riser = () => screen.getByTestId("comparison-overlay-entity-pipe:P-UP");
    expect(riser()).not.toHaveAttribute("data-collapsed");
    expect(riser().querySelector("line.comparison-overlay-line")).not.toBeNull();
    fireEvent.change(screen.getByTestId("comparison-overlay-view"), { target: { value: "top" } });
    expect(riser()).toHaveAttribute("data-collapsed", "true");
    expect(riser().querySelector("circle.comparison-overlay-point")).not.toBeNull();
    const glyphY = (ref: string) => Number(screen.getByTestId(`comparison-overlay-entity-${ref}`).querySelector(".comparison-overlay-glyph")!.getAttribute("y"));
    expect(glyphY("support:S-1")).not.toBe(glyphY("support:S-2"));
  });

  it("zooms and pans by its buttons and returns to the whole schematic", () => {
    mount();
    const drawing = screen.getByTestId("comparison-overlay-drawing");
    const whole = drawing.getAttribute("viewBox")!;
    const width = (box: string) => Number(box.split(" ")[2]);
    expect(screen.getByRole("button", { name: "Zoom out" })).toBeDisabled();
    fireEvent.click(screen.getByRole("button", { name: "Zoom in" }));
    expect(width(drawing.getAttribute("viewBox")!)).toBeCloseTo(width(whole) / 1.5);
    const zoomed = drawing.getAttribute("viewBox")!;
    fireEvent.click(screen.getByRole("button", { name: "Pan right" }));
    expect(Number(drawing.getAttribute("viewBox")!.split(" ")[0])).toBeGreaterThan(Number(zoomed.split(" ")[0]));
    fireEvent.click(screen.getByRole("button", { name: "Fit" }));
    expect(drawing.getAttribute("viewBox")).toBe(whole);
  });

  it("asks for a marked entity's row when it is activated by Enter, Space or pointer", () => {
    const { props } = mount();
    const removed = screen.getByRole("button", { name: /Node node:N-3/u });
    fireEvent.keyDown(removed, { key: "Enter" });
    expect(props.onActivate).toHaveBeenLastCalledWith("node:N-3", "node");
    fireEvent.keyDown(screen.getByRole("button", { name: /Support support:S-1/u }), { key: " " });
    expect(props.onActivate).toHaveBeenLastCalledWith("support:S-1", "support");
    fireEvent.click(screen.getByRole("button", { name: /Node node:N-4/u }));
    expect(props.onActivate).toHaveBeenLastCalledWith("node:N-4", "node");
    // A context entity is not activated.
    fireEvent.click(screen.getByTestId("comparison-overlay-entity-pipe:P-1"));
    expect(props.onActivate).toHaveBeenCalledTimes(3);
  });

  it("names both sides, says it is not the working model, and offers the way out by button and by Escape", () => {
    const { props } = mount();
    const banner = screen.getByTestId("comparison-overlay-banner");
    expect(banner).toHaveTextContent("Comparison overlay. This is not the working model.");
    expect(banner).toHaveTextContent("It is a schematic of the compared models in fixed views, not the three-dimensional viewport.");
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
    for (const view of ["iso", "iso-opposite"] as const) {
      const up = projectOverlayPoint({ x: 0, y: 1, z: 0 }, view);
      expect(up.x).toBeCloseTo(0);
      expect(up.y).toBeLessThan(0);
    }
    // With Z up, a point above the origin along Z is drawn upward in Front view.
    expect(projectOverlayPoint(upright({ x: 0, y: 0, z: 1 }, "z"), "front")).toEqual({ x: 0, y: -1 });
    expect(upright({ x: 1, y: 2, z: 3 }, "y")).toEqual({ x: 1, y: 2, z: 3 });
    // The opposite corner sees the model from behind: X runs the other way.
    expect(Math.sign(projectOverlayPoint({ x: 1, y: 0, z: 0 }, "iso").x)).toBe(1);
    expect(Math.sign(projectOverlayPoint({ x: 1, y: 0, z: 0 }, "iso-opposite").x)).toBe(-1);
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
