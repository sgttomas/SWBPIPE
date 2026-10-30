import { act, cleanup, fireEvent, render, screen, waitFor } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";
import { buildBlankLocalModelDocument } from "../../services/projectService";
import type { DisplayQuantityResult } from "../../services/displayQuantityService";
import { LoadCaseManagerPanel } from "./LoadCaseManagerPanel";

const convert = vi.hoisted(() => vi.fn());
vi.mock("../../services/displayQuantityService", () => ({ convertDisplayQuantities: convert }));
vi.mock("../../services/unitCatalogService", async (original) => ({
  ...await original<typeof import("../../services/unitCatalogService")>(),
  loadUnitCatalog: async () => ({ route: "unavailable_browser_preview", diagnostic: "Preview catalog unavailable" })
}));
afterEach(() => { cleanup(); convert.mockReset(); });

function mount(withExistingPrimitive = false) {
  const model = buildBlankLocalModelDocument();
  model.project.units.temperature = "C";
  model.nodes = [{ id: "node:1", label: "Node", position: { x: 0, y: 0, z: 0 }, provenance: "test" }];
  model.load_cases = [{ id: "load:1", label: "Load", kind: "thermal", status: "draft", provenance: "test" }];
  if (withExistingPrimitive) model.load_cases[0].primitive_loads = [{
    id: "load:existing", category: "concentrated_force", target: { type: "node", node: "node:1" },
    direction: "global_y", magnitude: { value: 20, unit: "kN" }, dimension: "force", provenance: "existing load"
  }];
  const queue = vi.fn();
  render(<LoadCaseManagerPanel model={model} onQueueIntent={queue} onSelect={vi.fn()} selection={{ type: "project", id: model.project.id }} />);
  return queue;
}

describe("primitive unit readiness", () => {
  it("renders an existing primitive and retains its entered magnitude unit", async () => {
    convert.mockResolvedValue([{ id: "primitive-unit", status: "converted", value: 1, unit: "N" }]);
    mount(true);
    expect(screen.getByTestId("load-manager-magnitude-unit")).toHaveValue("kN");
    expect(screen.getByTestId("load-manager-magnitude-value")).toHaveValue("20");
    await waitFor(() => expect(screen.getByTestId("queue-create-primitive-intent")).toBeEnabled());
  });

  it("blocks pending and unavailable core validation even with populated model metadata", async () => {
    let resolve!: (items: DisplayQuantityResult[]) => void;
    convert.mockImplementation(() => new Promise((done) => { resolve = done; }));
    mount();
    expect(screen.getByTestId("queue-create-primitive-intent")).toBeDisabled();
    await act(async () => resolve([{ id: "primitive-unit", status: "unavailable", message: "Unknown unit" }]));
    expect(screen.getByTestId("queue-create-primitive-intent")).toBeDisabled();
    expect(screen.getByTestId("load-manager-create-primitive-preview")).toHaveTextContent("Unknown unit");
  });

  it("invalidates a previous success when returning to the same unit during pending validation", async () => {
    const pending: Array<(items: DisplayQuantityResult[]) => void> = [];
    convert.mockImplementation(() => new Promise((done) => pending.push(done)));
    mount();
    await act(async () => pending[0]([{ id: "primitive-unit", status: "converted", value: 1, unit: "N" }]));
    expect(screen.getByTestId("queue-create-primitive-intent")).toBeEnabled();
    fireEvent.change(screen.getByTestId("load-manager-create-primitive-category"), { target: { value: "thermal" } });
    fireEvent.change(screen.getByTestId("load-manager-create-primitive-category"), { target: { value: "concentrated_force" } });
    expect(pending).toHaveLength(3);
    expect(screen.getByTestId("queue-create-primitive-intent")).toBeDisabled();
    await act(async () => pending[2]([{ id: "primitive-unit", status: "unavailable", message: "Current validation unavailable" }]));
    expect(screen.getByTestId("queue-create-primitive-intent")).toBeDisabled();
    expect(screen.getByTestId("load-manager-create-primitive-preview")).toHaveTextContent("Current validation unavailable");
  });

  it("queues only after core dimension validation and ignores stale replies", async () => {
    const pending: Array<(items: DisplayQuantityResult[]) => void> = [];
    convert.mockImplementation(() => new Promise((done) => pending.push(done)));
    const queue = mount();
    fireEvent.change(screen.getByTestId("load-manager-create-primitive-category"), { target: { value: "thermal" } });
    await waitFor(() => expect(pending).toHaveLength(2));
    expect(convert.mock.calls[1][0][0]).toMatchObject({ from_unit: "degC", to_unit: "degC", dimension_id: "temperature_interval" });
    await act(async () => pending[1]([{ id: "primitive-unit", status: "unavailable", message: "Rejected interval" }]));
    await act(async () => pending[0]([{ id: "primitive-unit", status: "converted", value: 1, unit: "N" }]));
    expect(screen.getByTestId("load-manager-create-primitive-preview")).toHaveTextContent("Rejected interval");
    expect(screen.getByTestId("queue-create-primitive-intent")).toBeDisabled();
    fireEvent.change(screen.getByTestId("load-manager-create-primitive-category"), { target: { value: "concentrated_force" } });
    await act(async () => pending[2]([{ id: "primitive-unit", status: "converted", value: 1, unit: "N" }]));
    expect(screen.getByTestId("queue-create-primitive-intent")).toBeEnabled();
    fireEvent.click(screen.getByTestId("queue-create-primitive-intent"));
    expect(queue).toHaveBeenCalledOnce();
  });
});
