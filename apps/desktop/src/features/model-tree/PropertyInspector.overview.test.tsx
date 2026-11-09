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

function mount(selection: EntityRef, model = inventedModel()) {
  const props: Props = { model, selection, onQueueIntent: vi.fn() };
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
});
