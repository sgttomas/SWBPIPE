// The inspector shows the selected entity's facts and the actions that apply to it, and
// nothing about other kinds of entity. Each case opens a line before it reads inside it, so
// nothing here passes on a control a person cannot reach. Invented model data only.
import { cleanup, fireEvent, render, screen, waitFor, within } from "@testing-library/react";
import type { ComponentProps } from "react";
import { afterEach, describe, expect, it, vi } from "vitest";
import { buildBlankLocalModelDocument } from "../../services/projectService";
import type { EditorOperationIntent, EntityRef, PreviewModel } from "../../types";
import { DisplayUnitsProvider } from "../display-units";
import { entityKey } from "../workspace/selectionState";
import { chooseVirtualTarget } from "../../test-support/workspaceTestControls";
import { PropertyInspector } from "./PropertyInspector";
import { offersBendAtCorner } from "./inspectorActions";

afterEach(cleanup);

/** An L of two pipes meeting at a right angle at node:C, and a straight run through node:M. */
function inventedModel(): PreviewModel {
  const model = buildBlankLocalModelDocument(new Date("2026-01-01T00:00:00Z"));
  model.project.units = { length: "m", force: "N", pressure: "Pa", temperature: "degC", angle: "rad" };
  const node = (id: string, x: number, y: number) => ({ id, label: id, position: { x, y, z: 0 }, provenance: "invented node" });
  model.nodes = [node("node:A", 0, 0), node("node:C", 3, 0), node("node:B", 3, 3), node("node:M", 6, 3), node("node:D", 9, 3)];
  const section = { outside_diameter: { value: 0.168, unit: "m" }, wall_thickness: { value: 0.007, unit: "m" } };
  const pipe = (id: string, from: string, to: string) => ({ id, label: id, from, to, material: "material:X", provenance: "invented pipe", section: structuredClone(section) });
  model.pipe_segments = [pipe("pipe:AC", "node:A", "node:C"), pipe("pipe:CB", "node:C", "node:B"), pipe("pipe:BM", "node:B", "node:M"), pipe("pipe:MD", "node:M", "node:D")];
  model.supports = [{ id: "support:A", label: "Anchor A", node: "node:A", restraints: ["UX", "UY", "UZ", "RX", "RY", "RZ"], provenance: "invented support" }];
  model.materials = [{ id: "material:X", label: "Invented material", elastic_modulus: { value: 2e11, unit: "Pa" }, provenance: "invented material" }];
  return model;
}

type Props = ComponentProps<typeof PropertyInspector>;

function mount(selection: EntityRef, extra: Partial<Props> = {}, model = inventedModel()) {
  const queue = vi.fn<(intent: EditorOperationIntent) => void>();
  render(<DisplayUnitsProvider initialPreference="SI"><PropertyInspector model={model} selection={selection} onQueueIntent={queue} {...extra} /></DisplayUnitsProvider>);
  return { model, queue };
}

const line = (name: string | RegExp) => screen.getByText(name, { selector: "summary, summary > span" }).closest("details")!;
function openLine(name: string | RegExp) {
  const details = line(name);
  fireEvent.click(details.querySelector(":scope > summary")!);
  expect(details).toHaveAttribute("open");
  return details;
}

describe("Bend at this corner: offered on what a person can see, refused only by the engine", () => {
  it("is offered on a node that joins exactly two pipes, and nowhere else", () => {
    const model = inventedModel();
    expect(offersBendAtCorner(model, "node:C")).toBe(true);
    expect(offersBendAtCorner(model, "node:M")).toBe(true);
    expect(offersBendAtCorner(model, "node:A")).toBe(false);
    mount({ type: "node", id: "node:A" });
    expect(screen.queryByText("Bend at this corner")).toBeNull();
  });

  it("shows the form closed, and an untouched form shows no error when it is opened", () => {
    mount({ type: "node", id: "node:C" });
    const form = screen.getByTestId("insert-bend-at-corner-form");
    expect(form).not.toBeVisible();
    openLine("Bend at this corner");
    expect(form).toBeVisible();
    expect(within(form).getByRole("group", { name: "Bend radius R" })).toBeVisible();
    expect(within(line("Bend at this corner")).queryByRole("alert")).toBeNull();
    expect(screen.queryByTestId("insert-bend-at-corner-engine-diagnostics")).toBeNull();
  });

  /** Opens the corner's line, enters a complete bend and presses Queue. */
  function queueBend(model: PreviewModel) {
    const { queue } = mount({ type: "node", id: "node:C" }, {}, model);
    const details = openLine("Bend at this corner");
    const field = (label: string, value: string) => {
      const input = within(details).getByLabelText(label);
      expect(input).toBeVisible();
      fireEvent.change(input, { target: { value } });
    };
    field("Bend radius R value", "0.5");
    field("Corner bend flexibility factor k", "1.5");
    field("Corner bend source of k", "invented k source");
    if (within(details).queryByLabelText("Bend component ID")) field("Geometry source reference", "invented geometry source");
    field("Corner bend provenance", "invented provenance");
    fireEvent.click(within(details).getByRole("button", { name: "Queue insert bend at corner" }));
    return { queue, details };
  }
  const refusedBy = async (details: HTMLElement, code: string) =>
    expect(await within(details).findByTestId(`insert-bend-at-corner-engine-diagnostic-${code}`, {}, { timeout: 20_000 })).toBeVisible();

  it("accepts two pipes whose equal sections are written in different member order", async () => {
    const model = inventedModel();
    const pipe = model.pipe_segments[1];
    pipe.section = { wall_thickness: { unit: "m", value: 0.007 }, outside_diameter: { unit: "m", value: 0.168 } } as typeof pipe.section;
    const { queue } = queueBend(model);
    await waitFor(() => expect(queue).toHaveBeenCalledTimes(1), { timeout: 20_000 });
    expect(queue.mock.calls[0][0].change.change_kind).toBe("insert_bend_at_corner");
  });

  it("queues nothing the engine refuses, and shows its code: a primitive load on the corner", async () => {
    const model = inventedModel();
    model.load_cases = [{ id: "load:L", label: "Invented case", kind: "primitive_user_load", status: "preview_only", provenance: "invented case",
      primitive_loads: [{ id: "load:L-C", category: "occasional", target: { type: "node", node: "node:C" }, direction: "global_y", magnitude: { value: 10, unit: "N" }, dimension: "force", provenance: "invented load" }] }] as unknown as PreviewModel["load_cases"];
    const { queue, details } = queueBend(model);
    await refusedBy(details, "OP-BEND-CORNER-ATTACHMENT-UNSUPPORTED");
    expect(queue).not.toHaveBeenCalled();
  });

  it("refuses a reference to either straight", async () => {
    const model = inventedModel();
    model.load_cases = [{ id: "load:L", label: "Invented case", kind: "primitive_user_load", status: "preview_only", provenance: "invented case",
      primitive_loads: [{ id: "load:L-P", category: "weight", target: { type: "element", pipe: "pipe:AC" }, direction: "global_z", magnitude: { value: -10, unit: "N/m" }, dimension: "force_per_length", provenance: "invented load" }] }] as unknown as PreviewModel["load_cases"];
    const { queue, details } = queueBend(model);
    await refusedBy(details, "OP-BEND-CORNER-ATTACHMENT-UNSUPPORTED");
    expect(queue).not.toHaveBeenCalled();
  });

  it("refuses a marker whose angle disagrees with the corner", async () => {
    const model = inventedModel();
    model.components = [{ id: "component:K", label: "Invented marker", kind: "bend", node: "node:C", provenance: "invented marker",
      geometry: { bend_angle: { value: 1.0, unit: "rad" } }, mechanics_interface: { solver_consumption: "mechanics_geometry_only" } }] as unknown as PreviewModel["components"];
    const { queue, details } = queueBend(model);
    await refusedBy(details, "OP-BEND-CORNER-ANGLE-CONFLICT");
    expect(queue).not.toHaveBeenCalled();
  });
});

describe("a selection shows itself", () => {
  it("holds no creation form, no operation card and no other kind's actions for a node", () => {
    mount({ type: "node", id: "node:M" });
    for (const kind of ["section", "material", "support", "component"]) {
      expect(screen.getByTestId(`create-${kind}-intent-panel`)).not.toBeVisible();
    }
    for (const name of ["New section", "New material", "New support", "New component"]) {
      expect(screen.queryByText(name, { selector: "summary" })).toBeNull();
    }
    // Its delete's operation stays behind a closed line until asked for.
    for (const preview of screen.queryAllByTestId("editor-operation-preview")) expect(preview).not.toBeVisible();
    expect(screen.getByText("Add a support here", { selector: "summary" })).toBeVisible();
  });

  it("puts a creation form away when another entity is inspected, keeping its draft for its command", () => {
    const model = inventedModel();
    const props = { model, onQueueIntent: vi.fn() };
    const nodeM: EntityRef = { type: "node", id: "node:M" };
    const nodeA: EntityRef = { type: "node", id: "node:A" };
    const view = render(<DisplayUnitsProvider initialPreference="SI"><PropertyInspector {...props} selection={nodeM} taskRequest={{ sequence: 1, target: nodeM, view: "properties", focusTestId: "create-support-id" }} /></DisplayUnitsProvider>);
    const area = screen.getByTestId("inspector-creation-support");
    expect(area).toBeVisible();
    fireEvent.change(screen.getByTestId("create-support-id"), { target: { value: "support:kept" } });
    view.rerender(<DisplayUnitsProvider initialPreference="SI"><PropertyInspector {...props} selection={nodeA} taskRequest={{ sequence: 1, target: nodeM, view: "properties", focusTestId: "create-support-id" }} /></DisplayUnitsProvider>);
    expect(screen.getByTestId("inspector-creation-support")).not.toBeVisible();
    view.rerender(<DisplayUnitsProvider initialPreference="SI"><PropertyInspector {...props} selection={nodeM} taskRequest={{ sequence: 1, target: nodeM, view: "properties", focusTestId: "create-support-id" }} /></DisplayUnitsProvider>);
    expect(screen.getByTestId("inspector-creation-support")).not.toBeVisible();
    view.rerender(<DisplayUnitsProvider initialPreference="SI"><PropertyInspector {...props} selection={nodeM} taskRequest={{ sequence: 2, target: nodeM, view: "properties", focusTestId: "create-support-id" }} /></DisplayUnitsProvider>);
    expect(screen.getByTestId("inspector-creation-support")).toBeVisible();
    expect(screen.getByTestId("create-support-id")).toHaveValue("support:kept");
  });

  it("puts one Delete control at the foot of a support's actions, with what it refuses", () => {
    const { queue } = mount({ type: "support", id: "support:A" });
    const remove = screen.getByRole("button", { name: "Delete support" });
    expect(remove).toBeVisible();
    expect(remove).toHaveAttribute("data-testid", "queue-delete-support-intent");
    expect(screen.getByTestId("delete-support-intent-panel")).toHaveTextContent(
      "Queued in Review changes; nothing is deleted until you apply it there. Nothing else is removed. Refused while another record depends on it; the refusal names which."
    );
    // The operation itself stays behind its closed line until asked for.
    expect(within(screen.getByTestId("delete-support-intent-panel")).getByTestId("editor-operation-preview")).not.toBeVisible();
    fireEvent.click(remove);
    expect(queue.mock.calls[0][0].change.change_kind).toBe("delete_support");
    expect(screen.getByText("Edit this support", { selector: "summary" })).toBeVisible();
  });

  it("counts several things by kind and offers what can be done with several", () => {
    const change = vi.fn();
    const refs: EntityRef[] = [{ type: "node", id: "node:A" }, { type: "node", id: "node:C" }, { type: "support", id: "support:A" }];
    mount(refs[0], { onChangeSelected: change, selectionState: { orderedKeys: refs.map(entityKey), primaryKey: entityKey(refs[0]), rangeAnchorKey: null, focusKey: null, preparationEpoch: 0 } });
    const aggregate = screen.getByTestId("aggregate-property-inspector");
    expect(aggregate).toBeVisible();
    expect(aggregate).toHaveTextContent("Nodes2");
    expect(aggregate).toHaveTextContent("Support1");
    expect(aggregate).not.toHaveTextContent(/draft|typed target/i);
    // The first entity's own actions (its support line, its Delete) are not shown for several.
    const actions = screen.getByTestId("property-inspector").querySelector('[role="group"][aria-label="Actions"]');
    expect(actions).not.toBeNull();
    expect(actions).not.toBeVisible();
    fireEvent.click(within(aggregate).getByRole("button", { name: "Change selected…" }));
    expect(change).toHaveBeenCalledTimes(1);
  });
});

describe("settings are not properties", () => {
  it("shows the project's settings as closed lines that state their values, with the controls inside", () => {
    mount({ type: "project", id: "project" });
    const settings = line("Model settings");
    expect(settings).not.toHaveAttribute("open");
    expect(settings.querySelector("summary")).toHaveTextContent("Shear deformation: Euler–Bernoulli (default) · Node proximity: 5 mm (default)");
    expect(screen.getByTestId("model-settings-shear-model-euler_bernoulli")).not.toBeVisible();
    openLine("Model settings");
    const euler = screen.getByTestId("model-settings-shear-model-euler_bernoulli");
    expect(euler).toBeVisible();
    expect(euler).toBeChecked();
    expect(settings).not.toHaveTextContent("NODE_NEAR_DUPLICATE");
    expect(settings).not.toHaveTextContent(/pressure contract|Pressure panel/);
    const units = line("Project units");
    expect(units).not.toHaveAttribute("open");
    expect(units.querySelector("summary")).toHaveTextContent("m, N");
    expect(screen.getByTestId("inspector-overview")).not.toHaveTextContent("Schema");
  });
});

describe("the Support tool and New support agree", () => {
  type Armed = "support" | null;
  function harness() {
    const model = inventedModel();
    const arm = vi.fn<(tool: Armed) => void>();
    const nodeM: EntityRef = { type: "node", id: "node:M" };
    const request = { sequence: 1, target: nodeM, view: "properties" as const, focusTestId: "create-support-id" };
    const ui = (selection: EntityRef, armed: Armed) => <DisplayUnitsProvider initialPreference="SI"><PropertyInspector model={model} onQueueIntent={vi.fn()} selection={selection} taskRequest={request} armedCreationTool={armed} onArmCreationTool={arm} /></DisplayUnitsProvider>;
    const view = render(ui(nodeM, "support"));
    const current = () => screen.getByTestId("create-support-node").querySelector(".virtual-target-picker-current");
    return { arm, view, ui, current };
  }

  it("Insert > Support, then a node picked in the view: the form stays open on that node", () => {
    const { view, ui, current } = harness();
    expect(screen.getByTestId("inspector-creation-support")).toBeVisible();
    expect(current()).toHaveTextContent("node:M");
    view.rerender(ui({ type: "node", id: "node:A" }, "support"));
    expect(screen.getByTestId("inspector-creation-support")).toBeVisible();
    expect(current()).toHaveTextContent("node:A");
  });

  it("keeps a node the person entered through a later pick", () => {
    const { view, ui, current } = harness();
    chooseVirtualTarget("create-support-node", "node:B");
    view.rerender(ui({ type: "node", id: "node:A" }, "support"));
    expect(screen.getByTestId("inspector-creation-support")).toBeVisible();
    expect(current()).toHaveTextContent("node:B");
  });

  it("closing New support disarms the tool; disarming the tool leaves the form as it is", () => {
    const { arm, view, ui } = harness();
    view.rerender(ui({ type: "node", id: "node:A" }, "support"));
    view.rerender(ui({ type: "node", id: "node:A" }, null));
    expect(arm).not.toHaveBeenCalled();
    expect(screen.getByTestId("inspector-creation-support")).toBeVisible();
    view.rerender(ui({ type: "node", id: "node:A" }, "support"));
    fireEvent.click(screen.getByTestId("inspector-creation-close-support"));
    expect(arm).toHaveBeenCalledWith(null);
    expect(screen.getByTestId("inspector-creation-support")).not.toBeVisible();
  });
});
