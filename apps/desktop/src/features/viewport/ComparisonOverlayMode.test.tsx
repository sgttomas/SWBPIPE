// The comparison overlay's schematic (DEL-07-08 REQ-011, AC-003, AC-006; DEL-07-06 REQ-011): the
// banner and the legend's words and counts read from the accessibility tree, each class's signal
// besides colour, marked entities as the only keyboard stops, labels in the model tree's names
// kept inside the drawing, the list of marks moving the view without changing the selection,
// activation of a marked entity, the way out, the up direction from the model, and the authoring
// viewport beneath left untouched. jsdom has no layout, so sizes on screen and the drawn look are
// not observed here. All data are invented.
import { cleanup, fireEvent, render, screen, within } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";
import { compareModelDocuments } from "../comparison/stateComparison";
import { STATE_OVERLAY_CLASSES, RUN_OVERLAY_CLASSES, stateOverlayMarks } from "../comparison/comparisonOverlay";
import type { ShownComparisonOverlay } from "../history/useModelStateHistory";
import { ComparisonOverlayMode, labelPlacement, LABEL_MAX_CHARS, projectOverlayPoint, upright } from "./ComparisonOverlayMode";
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
  model.nodes.push({ id: "node:N-4", label: "Added node", position: { x: 4, y: 0, z: 0 }, provenance: "invented" });
  model.supports[0].label = "Anchor, relabelled";
  model.materials[0].elastic_modulus = { value: 210, unit: "GPa" };
  return model;
})();

function overlayOf(left: Json, right: Json): ShownComparisonOverlay {
  return {
    kind: "states", ids: ["state:1", "state:2"], leftText: "\"Basis\" (state:1)", rightText: "\"Revision\" (state:2)",
    marks: stateOverlayMarks(compareModelDocuments(left, right)), models: { left, right }
  };
}

function mount(overrides: Partial<Parameters<typeof ComparisonOverlayMode>[0]> = {}) {
  const props = { overlay: overlayOf(LEFT, RIGHT), onLeave: vi.fn(), onActivate: vi.fn(), ...overrides };
  return { props, ...render(<ComparisonOverlayMode {...props} />) };
}

const drawn = (ref: string) => screen.getByTestId(`comparison-overlay-entity-${ref}`);
const viewBox = () => screen.getByTestId("comparison-overlay-drawing").getAttribute("viewBox")!.split(" ").map(Number);

describe("the comparison schematic", () => {
  it("says in one sentence what it is and offers the way out by button and by Escape", () => {
    const { props } = mount();
    const banner = screen.getByTestId("comparison-overlay-banner");
    expect(banner.querySelector("p")).toHaveTextContent(
      "Comparison overlay: a schematic in fixed views, not the three-dimensional viewport, of the later saved state, \"Revision\" (state:2), against the earlier, \"Basis\" (state:1); it is not the working model and nothing in it can be edited."
    );
    // One sentence: no full stop before its end.
    expect(banner.querySelector("p")!.textContent!.trim().slice(0, -1)).not.toMatch(/\.\s/u);
    fireEvent.click(screen.getByRole("button", { name: "Leave the comparison overlay" }));
    expect(props.onLeave).toHaveBeenCalledTimes(1);
    fireEvent.keyDown(screen.getByTestId("comparison-overlay-drawing"), { key: "Escape" });
    expect(props.onLeave).toHaveBeenCalledTimes(2);
  });

  it("states each class with its count, the ghosts and what is not drawn, in short words in the accessibility tree", () => {
    mount();
    const legend = screen.getByRole("region", { name: "Comparison overlay legend" });
    const classes = within(legend).getAllByRole("listitem").slice(0, 5).map((item) => item.querySelector(":scope > span")!.textContent);
    expect(classes).toEqual([
      "Added 1",
      "Removed 1",
      // The support, the material and the model entity (its ordered id lists) are changed.
      "Changed 3",
      "Unchanged 3 (context)",
      "Faint and hollow: only in the earlier saved state"
    ]);
    // The samples are hidden from the accessibility tree; the words carry the class.
    for (const sample of legend.querySelectorAll("svg")) expect(sample).toHaveAttribute("aria-hidden", "true");
    expect(within(legend).getByTestId("comparison-overlay-not-drawn")).toHaveTextContent("Not drawn 2 (Changed 2): no drawing; they are in the table.");
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
    expect(drawn("node:N-4")).toHaveAttribute("data-line", "solid");
    expect(drawn("node:N-4")).toHaveAttribute("data-glyph", "+");
    expect(drawn("node:N-3")).toHaveAttribute("data-line", "dashed");
    expect(drawn("node:N-3")).toHaveAttribute("data-glyph", "−");
    expect(drawn("node:N-3")).toHaveAttribute("data-ghost", "true");
    expect(drawn("support:S-1")).toHaveAttribute("data-line", "dash-dot");
    expect(drawn("pipe:P-1")).toHaveAttribute("data-line", "thin");
  });

  it("makes only marked entities keyboard stops, named in words; unchanged ones are context, drawn beneath every mark", () => {
    mount();
    for (const ref of ["node:N-1", "node:N-2", "pipe:P-1"]) {
      expect(drawn(ref)).toHaveAttribute("data-marked", "false");
      expect(drawn(ref)).not.toHaveAttribute("tabindex");
      expect(drawn(ref)).not.toHaveAttribute("role");
    }
    expect(screen.getByRole("button", { name: "Node node:N-3 (C), only in the earlier saved state, drawn as a ghost: Removed" })).toBe(drawn("node:N-3"));
    expect(screen.getByRole("button", { name: "Support support:S-1 (Anchor, relabelled): Changed" })).toBe(drawn("support:S-1"));
    for (const ref of ["node:N-3", "node:N-4", "support:S-1"]) expect(drawn(ref)).toHaveAttribute("tabindex", "0");
    // Painting order is document order: every mark after every context entity, ghosts before live marks.
    const order = Array.from(screen.getByTestId("comparison-overlay-drawing").querySelectorAll<SVGGElement>(".comparison-overlay-entity")).map((element) => element.dataset.marked);
    expect(order.lastIndexOf("false")).toBeLessThan(order.indexOf("true"));
    expect(drawn("support:S-1").querySelector(".comparison-overlay-halo")).not.toBeNull();
    expect(drawn("pipe:P-1").querySelector(".comparison-overlay-halo")).toBeNull();
  });

  it("asks for a marked entity's row when it is activated by Enter, Space or pointer, and not for a context entity", () => {
    const { props } = mount();
    fireEvent.keyDown(drawn("node:N-3"), { key: "Enter" });
    expect(props.onActivate).toHaveBeenLastCalledWith("node:N-3", "node");
    fireEvent.keyDown(drawn("support:S-1"), { key: " " });
    expect(props.onActivate).toHaveBeenLastCalledWith("support:S-1", "support");
    fireEvent.click(drawn("node:N-4"));
    expect(props.onActivate).toHaveBeenLastCalledWith("node:N-4", "node");
    fireEvent.click(drawn("pipe:P-1"));
    expect(props.onActivate).toHaveBeenCalledTimes(3);
  });

  it("labels each mark with the model tree's name, a ghost as earlier only, and gives every entity a title", () => {
    mount();
    const label = (ref: string) => drawn(ref).querySelector(".comparison-overlay-label")?.textContent;
    expect(label("node:N-4")).toBe("Added node");
    expect(label("support:S-1")).toBe("Anchor, relabelled");
    expect(label("node:N-3")).toBe("C (earlier only)");
    expect(label("pipe:P-1")).toBeUndefined();
    expect(drawn("pipe:P-1").querySelector("title")).toHaveTextContent("Pipe segment pipe:P-1 (P): Unchanged");
    // The rightmost mark's label is set to the left of its glyph, inside the drawing.
    expect(drawn("node:N-4").querySelector(".comparison-overlay-label")).toHaveAttribute("data-side", "end");
    expect(drawn("node:N-3").querySelector(".comparison-overlay-label")).toHaveAttribute("data-side", "start");
    fireEvent.click(screen.getByRole("button", { name: "Labels" }));
    expect(label("node:N-4")).toBeUndefined();
  });

  it("keeps a long name inside the room it has, on the side with more room", () => {
    const long = "Pipe support on the north rack, bay 12, level 3, at the expansion loop";
    const box = { x: 0, width: 100 };
    // Plenty of room on the right: cut to the most a label shows.
    const roomy = labelPlacement(long, 5, 1, box, 1);
    expect(roomy.side).toBe("start");
    expect([...roomy.text]).toHaveLength(LABEL_MAX_CHARS);
    expect(roomy.text.endsWith("…")).toBe(true);
    // Near the right edge: set to the left.
    expect(labelPlacement(long, 95, 1, box, 1).side).toBe("end");
    // Little room either side: cut to what fits (each character 0.62 of the font size).
    const tight = labelPlacement(long, 50, 1, box, 4);
    expect([...tight.text].length * 4 * 0.62).toBeLessThanOrEqual(49);
    // A short name is shown whole.
    expect(labelPlacement("Anchor", 95, 1, box, 1).text).toBe("Anchor");
  });

  it("lists the marks in words, and choosing one moves the view to it without activating or selecting it", () => {
    const { props } = mount();
    const list = screen.getByRole("list", { name: "Marks (3)" });
    expect(within(list).getAllByRole("button").map((button) => button.textContent)).toEqual([
      "+ Added node Added",
      "− C Removed, earlier saved state only",
      "Δ Anchor, relabelled Changed"
    ]);
    const before = viewBox();
    const choice = within(list).getByRole("button", { name: /Added node/u });
    expect(choice.tagName).toBe("BUTTON");
    fireEvent.click(choice);
    const after = viewBox();
    // Zoomed in to at least four times, centred on the mark (node N-4 at x = 4 in the Isometric view).
    expect(after[2]).toBeCloseTo(before[2] / 4);
    const centre = projectOverlayPoint({ x: 4, y: 0, z: 0 }, "iso");
    expect(after[0] + after[2] / 2).toBeCloseTo(centre.x);
    expect(after[1] + after[3] / 2).toBeCloseTo(centre.y);
    expect(choice).toHaveAttribute("aria-current", "location");
    expect(drawn("node:N-4")).toHaveAttribute("data-located", "true");
    expect(props.onActivate).not.toHaveBeenCalled();
  });

  it("draws a member seen end on as a point, and stacks the glyphs of marks that coincide in a view", () => {
    const left = document();
    const right = document();
    // A riser along Y, added; two changed supports on one node.
    right.nodes.push({ id: "node:N-UP", label: "Up", position: { x: 2, y: 3, z: 0 }, provenance: "invented" });
    right.pipe_segments.push({ id: "pipe:P-UP", label: "Riser", from: "node:N-2", to: "node:N-UP", section: {}, material: "material:M-1", provenance: "invented" });
    left.supports.push({ id: "support:S-2", label: "Guide", node: "node:N-1", provenance: "invented" });
    right.supports = [{ ...right.supports[0], label: "Anchor, moved" }, { id: "support:S-2", label: "Guide, changed", node: "node:N-1", provenance: "invented" }];
    mount({ overlay: overlayOf(left, right) });
    const riser = () => drawn("pipe:P-UP");
    expect(riser()).not.toHaveAttribute("data-collapsed");
    fireEvent.change(screen.getByTestId("comparison-overlay-view"), { target: { value: "top" } });
    expect(riser()).toHaveAttribute("data-collapsed", "true");
    expect(riser().querySelector("circle.comparison-overlay-point")).not.toBeNull();
    const glyphY = (ref: string) => Number(drawn(ref).querySelector(".comparison-overlay-glyph")!.getAttribute("y"));
    expect(glyphY("support:S-1")).not.toBe(glyphY("support:S-2"));
  });

  it("zooms and pans by its buttons and returns to the whole schematic", () => {
    mount();
    const whole = viewBox();
    expect(screen.getByRole("button", { name: "Zoom out" })).toBeDisabled();
    fireEvent.click(screen.getByRole("button", { name: "Zoom in" }));
    expect(viewBox()[2]).toBeCloseTo(whole[2] / 1.5);
    const zoomed = viewBox();
    fireEvent.click(screen.getByRole("button", { name: "Pan right" }));
    expect(viewBox()[0]).toBeGreaterThan(zoomed[0]);
    fireEvent.click(screen.getByRole("button", { name: "Fit" }));
    expect(viewBox()).toEqual(whole);
  });

  it("takes up from the model's weight loads, says which case holds, and lets the person's choice override it", () => {
    const withWeight = (model: Json) => ({ ...model, load_cases: [{ id: "load:W", label: "Weight", kind: "primitive_user_load", primitive_loads: [
      { id: "load:W-1", category: "weight", target: { type: "element", pipe: "pipe:P-1" }, direction: "global_z", magnitude: { value: -100, unit: "N/m" }, dimension: "force_per_length" }
    ] }] });
    mount({ overlay: overlayOf(withWeight(LEFT), withWeight(RIGHT)) });
    expect(screen.getByTestId("comparison-overlay-drawing")).toHaveAttribute("data-up", "+z");
    expect(screen.getByTestId("comparison-overlay-up-readout")).toHaveTextContent("Up +Z, opposite the model's 1 weight load.");
    fireEvent.change(screen.getByTestId("comparison-overlay-up"), { target: { value: "+y" } });
    expect(screen.getByTestId("comparison-overlay-drawing")).toHaveAttribute("data-up", "+y");
    expect(screen.getByTestId("comparison-overlay-up-readout")).toHaveTextContent("Up +Y, chosen. Up +Z, opposite the model's 1 weight load.");
    cleanup();
    mount();
    expect(screen.getByTestId("comparison-overlay-drawing")).toHaveAttribute("data-up", "+y");
    expect(screen.getByTestId("comparison-overlay-up-readout")).toHaveTextContent("Up +Y: the model has no weight load to take it from.");
  });

  it("presents no comparison as validation, approval or compliance", () => {
    mount();
    expect(screen.getByTestId("comparison-overlay").textContent).not.toMatch(/valid|approv|complian|certif|accept|verif|pass|fail|correct/iu);
    cleanup();
    mount({ overlay: { ...overlayOf(LEFT, RIGHT), kind: "runs", marks: [] } });
    expect(screen.getByTestId("comparison-overlay").textContent).not.toMatch(/valid|approv|complian|certif|accept|verif|pass|fail|correct/iu);
  });

  it("projects with the authoring viewport's presets, Y up, and turns each signed up axis upward", () => {
    expect(projectOverlayPoint({ x: 1, y: 2, z: 3 }, "front")).toEqual({ x: 1, y: -2 });
    expect(projectOverlayPoint({ x: 1, y: 2, z: 3 }, "top")).toEqual({ x: 1, y: 3 });
    for (const view of ["iso", "iso-opposite"] as const) {
      const up = projectOverlayPoint({ x: 0, y: 1, z: 0 }, view);
      expect(up.x).toBeCloseTo(0);
      expect(up.y).toBeLessThan(0);
    }
    // The opposite corner sees the model from behind: X runs the other way.
    expect(Math.sign(projectOverlayPoint({ x: 1, y: 0, z: 0 }, "iso").x)).toBe(1);
    expect(Math.sign(projectOverlayPoint({ x: 1, y: 0, z: 0 }, "iso-opposite").x)).toBe(-1);
    // Each signed axis is turned to +Y, by a rotation (lengths and handedness kept).
    const unit = { "+x": { x: 1, y: 0, z: 0 }, "-x": { x: -1, y: 0, z: 0 }, "+y": { x: 0, y: 1, z: 0 }, "-y": { x: 0, y: -1, z: 0 }, "+z": { x: 0, y: 0, z: 1 }, "-z": { x: 0, y: 0, z: -1 } } as const;
    for (const [axis, vector] of Object.entries(unit)) {
      const turned = upright(vector, axis as keyof typeof unit);
      expect([turned.x, turned.y, turned.z].map((value) => value + 0)).toEqual([0, 1, 0]);
      const ex = upright({ x: 1, y: 0, z: 0 }, axis as keyof typeof unit);
      const ey = upright({ x: 0, y: 1, z: 0 }, axis as keyof typeof unit);
      const ez = upright({ x: 0, y: 0, z: 1 }, axis as keyof typeof unit);
      // ex × ey = ez: a proper rotation.
      expect([ex.y * ey.z - ex.z * ey.y, ex.z * ey.x - ex.x * ey.z, ex.x * ey.y - ex.y * ey.x].map((value) => value + 0)).toEqual([ez.x + 0, ez.y + 0, ez.z + 0]);
    }
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
        {active ? <ComparisonOverlayMode overlay={overlayOf(LEFT, RIGHT)} onLeave={vi.fn()} onActivate={vi.fn()} /> : null}
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
    fireEvent.keyDown(drawn("node:N-3"), { key: "Enter" });
    fireEvent.click(screen.getByTestId("comparison-overlay-mark-node:N-4"));
    view.rerender(viewport(false));
    expect(view.container.querySelector(".viewport-shell")).toBe(shell);
    expect(shell).not.toHaveAttribute("inert");
    expect(onSelect).not.toHaveBeenCalled();
    expect(JSON.stringify(model)).toBe(before);
  });
});
