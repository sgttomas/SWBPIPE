import { cleanup, fireEvent, render, screen, waitFor, within } from "@testing-library/react";
import type { ComponentProps } from "react";
import { afterEach, describe, expect, it, vi } from "vitest";
import { buildBlankLocalModelDocument } from "../../services/projectService";
import { changeFormControl } from "../../test-support/workspaceTestControls";
import type { EntityRef, PreviewModel } from "../../types";
import { DisplayUnitsProvider } from "../display-units";
import { PropertyInspector } from "./PropertyInspector";

afterEach(cleanup);

/** Invented model: every entity and value is synthetic test input. A node and a material share the id "X". */
function inventedModel(): PreviewModel {
  const model = buildBlankLocalModelDocument(new Date("2026-01-01T00:00:00Z"));
  model.project.units = { length: "mm", force: "N", pressure: "MPa", temperature: "C", angle: "rad" };
  model.nodes = [
    { id: "X", label: "Node named X", position: { x: 0, y: 0, z: 0 }, provenance: "invented node" },
    { id: "node:B", label: "Node B", position: { x: 1000, y: 0, z: 0 }, provenance: "invented node" }
  ];
  model.materials = [{ id: "X", label: "Material named X", elastic_modulus: { value: 200000, unit: "MPa" }, provenance: "invented material" }];
  model.pipe_segments = [
    { id: "pipe:1", label: "Pipe 1", from: "X", to: "node:B", material: "X", provenance: "invented pipe",
      section: { outside_diameter: { value: 100, unit: "mm" }, wall_thickness: { value: 5, unit: "mm" } } },
    { id: "pipe:2", label: "Pipe 2", from: "node:B", to: "node:missing", material: "TBD", provenance: "invented pipe",
      section: { outside_diameter: { value: 100, unit: "mm" }, wall_thickness: { value: 5, unit: "mm" } } }
  ];
  model.supports = [{ id: "support:H", label: "Hanger", node: "node:B", family: "variable_spring_hanger", restraints: [], provenance: "invented support" }];
  return model;
}

type Props = ComponentProps<typeof PropertyInspector>;

function mount(selection: EntityRef, model = inventedModel(), onSelectReference?: Props["onSelectReference"]) {
  const props: Props = { model, selection, onQueueIntent: vi.fn(), onSelectReference };
  const view = render(<DisplayUnitsProvider initialPreference="SI"><PropertyInspector {...props} /></DisplayUnitsProvider>);
  return {
    select: (next: EntityRef) => view.rerender(<DisplayUnitsProvider initialPreference="SI"><PropertyInspector {...props} selection={next} /></DisplayUnitsProvider>)
  };
}

const overview = (label: string) => screen.getByTestId(`inspector-overview-${label.toLowerCase().replaceAll(" ", "-")}`);
const allProperties = () => screen.getByText("All properties").closest("details")!;

describe("PropertyInspector overview", () => {
  it("names typed references by their own type's label and keeps the exact id", () => {
    mount({ type: "pipe", id: "pipe:1" });
    // "X" is both a node and a material: each row takes its own type's name.
    expect(overview("From")).toHaveTextContent("Node named X");
    expect(overview("From")).not.toHaveTextContent("Material named X");
    expect(overview("Material")).toHaveTextContent("Material named X");
    expect(within(overview("From")).getByText("X")).toHaveClass("property-reference-id");
    expect(overview("To")).toHaveTextContent("Node B");
    expect(overview("To")).toHaveTextContent("node:B");
    expect(overview("OD")).toBeInTheDocument();
  });

  it("states a missing typed reference and leaves other text as stated", () => {
    mount({ type: "pipe", id: "pipe:2" });
    expect(overview("To")).toHaveTextContent("node:missing (no node with this id in the model)");
    expect(within(overview("Material")).getByRole("definition")).toHaveTextContent(/^TBD$/u);
    expect(within(overview("Material")).queryByTestId("property-reference-missing")).toBeNull();
  });

  it("shows a support family's product label above its stored value and keeps empty restraints empty", () => {
    mount({ type: "support", id: "support:H" });
    expect(overview("Family")).toHaveTextContent("Variable spring hanger");
    expect(within(overview("Family")).getByText("variable_spring_hanger")).toHaveClass("property-enum-value");
    expect(within(overview("Restraints")).getByRole("definition")).toHaveTextContent(/^$/u);
  });

  it("keeps the user's All properties choice across selection changes, closed by default", async () => {
    const { select } = mount({ type: "pipe", id: "pipe:1" });
    expect(allProperties()).not.toHaveAttribute("open");
    select({ type: "pipe", id: "pipe:2" });
    expect(allProperties()).not.toHaveAttribute("open");
    fireEvent.click(screen.getByText("All properties"));
    await waitFor(() => expect(within(allProperties()).getByText("Provenance")).toBeInTheDocument());
    select({ type: "node", id: "node:B" });
    expect(allProperties()).toHaveAttribute("open");
    expect(within(allProperties()).getByText("Position")).toBeInTheDocument();
    fireEvent.click(screen.getByText("All properties"));
    await waitFor(() => expect(allProperties()).not.toHaveAttribute("open"));
    select({ type: "pipe", id: "pipe:1" });
    expect(allProperties()).not.toHaveAttribute("open");
  });

  it("leaves a task draft and its pinned target untouched while the disclosure and selection change", async () => {
    const { select } = mount({ type: "pipe", id: "pipe:1" });
    fireEvent.click(screen.getByRole("tab", { name: "Task" }));
    fireEvent.click(screen.getByTestId("inspector-start-task"));
    changeFormControl(screen.getByTestId("editor-intent-field"), { target: { value: "label" } });
    fireEvent.change(screen.getByTestId("editor-intent-value"), { target: { value: "Draft name" } });
    fireEvent.click(screen.getByRole("tab", { name: "Properties" }));
    fireEvent.click(screen.getByText("All properties"));
    await waitFor(() => expect(allProperties()).toHaveAttribute("open"));
    select({ type: "pipe", id: "pipe:2" });
    fireEvent.click(screen.getByRole("tab", { name: "Task" }));
    expect(screen.getByTestId("inspector-frozen-task-target")).toHaveTextContent("Draft target: pipe: pipe:1");
    expect(screen.getByTestId("editor-intent-value")).toHaveValue("Draft name");
  });

  it("selects a resolved reference as its own type, from a native button", () => {
    const onSelect = vi.fn();
    mount({ type: "pipe", id: "pipe:1" }, inventedModel(), onSelect);
    const from = within(overview("From")).getByRole("button", { name: "Select node Node named X (X)" });
    expect(from).toHaveAttribute("type", "button");
    expect(from).toHaveAttribute("data-testid", "property-reference");
    fireEvent.click(from);
    expect(onSelect).toHaveBeenLastCalledWith({ type: "node", id: "X" });
    // The material shares the id "X" and is still selected as a material.
    fireEvent.click(within(overview("Material")).getByRole("button", { name: "Select material Material named X (X)" }));
    expect(onSelect).toHaveBeenLastCalledWith({ type: "material", id: "X" });
    expect(onSelect).toHaveBeenCalledTimes(2);
    // Focus leaves the pressed reference for the Properties panel, which stays mounted.
    expect(document.activeElement).toBe(screen.getByRole("tabpanel"));
  });

  it("leaves an unresolved reference and TBD as text that cannot select", () => {
    const onSelect = vi.fn();
    mount({ type: "pipe", id: "pipe:2" }, inventedModel(), onSelect);
    expect(within(overview("To")).queryByRole("button")).toBeNull();
    expect(overview("To")).toHaveTextContent("node:missing (no node with this id in the model)");
    expect(within(overview("Material")).queryByRole("button")).toBeNull();
    expect(within(overview("Material")).getByRole("definition")).toHaveTextContent(/^TBD$/u);
    // The resolved From reference is the only selectable one in the overview.
    expect(within(screen.getByTestId("inspector-overview")).getAllByRole("button")).toHaveLength(1);
  });

  it("keeps references as text when the host offers no selection route", () => {
    mount({ type: "pipe", id: "pipe:1" });
    expect(within(screen.getByTestId("inspector-overview")).queryAllByRole("button")).toHaveLength(0);
    expect(within(overview("From")).getByTestId("property-reference")).toHaveTextContent("Node named X X");
  });

  it("states the heading's kind once, keeping the exact id and the kind of a prefixless or other-kind id", () => {
    const model = inventedModel();
    model.nodes.push({ id: "pipe:odd", label: "Node with a pipe-like id", position: { x: 0, y: 1, z: 0 }, provenance: "invented node" });
    const { select } = mount({ type: "pipe", id: "pipe:1" }, model);
    const heading = () => screen.getByRole("heading", { level: 2 });
    expect(heading()).toHaveTextContent(/^Pipe 1 — pipe:1$/u);
    expect(heading().querySelector(".typed-identity")).toHaveTextContent(/^— pipe:1$/u);
    select({ type: "node", id: "X" });
    expect(heading()).toHaveTextContent(/^Node named X — node: X$/u);
    select({ type: "node", id: "pipe:odd" });
    expect(heading()).toHaveTextContent(/^Node with a pipe-like id — node: pipe:odd$/u);
  });
});

