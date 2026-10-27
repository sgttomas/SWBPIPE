// B9: the entity commands. A material, section or component removal is the inspector's Remove
// panel intent, prepared from the same canonical snapshot and disabled while it is prepared or
// when it cannot be; load cases, combinations and the project state where they are removed, or
// that they are not. Invented fixtures only.
import { cleanup, fireEvent, render, screen, waitFor } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";
import { buildBlankLocalModelDocument } from "../../../services/projectService";
import type { EditorOperationIntent, EntityRef, PreviewModel } from "../../../types";
import { GuardedRemoval } from "../../toolkit/GuardedRemoval";
import { modelIndexFor } from "../modelIndex";
import { entityKey } from "../selectionState";
import { EntityContextMenu, entityCommandEntries, type EntityCommandEnvironment } from "./EntityContextMenu";

const canonical = vi.hoisted(() => vi.fn());
vi.mock("../../../services/hashService", () => ({ canonicalJsonString: canonical }));
afterEach(() => { cleanup(); canonical.mockReset(); });

function inventedModel(): PreviewModel {
  const model = buildBlankLocalModelDocument(new Date("2026-01-01T00:00:00Z"));
  model.project.units = { length: "mm", force: "N", pressure: "MPa", temperature: "C", angle: "rad" };
  model.nodes = [{ id: "node:A", label: "Node A", position: { x: 0, y: 0, z: 0 }, provenance: "invented node" }];
  model.materials = [{ id: "material:X", label: "Invented material", elastic_modulus: { value: 200000, unit: "MPa" }, provenance: "invented material" }];
  model.sections = [{ id: "section:S", name: "Invented section", section_type: "pipe", properties: {}, provenance: "invented section" }];
  return model;
}

function environment(model: PreviewModel, selected: readonly EntityRef[], extra: Partial<EntityCommandEnvironment> = {}): EntityCommandEnvironment {
  return {
    model, modelIndex: modelIndexFor(model, 0, 0), selectedKeys: selected.map(entityKey), hiddenKeys: new Set(),
    showAllAvailable: true, viewCommand: vi.fn(), operationBusy: false, onQueueIntent: vi.fn(), ...extra
  };
}

const request = () => ({ anchor: { x: 0, y: 0 }, returnFocus: null });
const reasonOf = (entries: ReturnType<typeof entityCommandEntries>, id: string) => {
  const entry = entries.find((candidate) => candidate.id === id);
  return entry?.kind === "command" ? entry.reason ?? null : undefined;
};

describe("the entity commands", () => {
  it.each(["material", "section"] as const)("queues the Remove panel's %s intent once its snapshot is prepared", async (type) => {
    const model = inventedModel();
    const target: EntityRef = { type, id: type === "material" ? "material:X" : "section:S" };
    const entity = type === "material" ? model.materials![0] : model.sections![0];
    canonical.mockResolvedValue(JSON.stringify(entity));
    // The inspector's Remove panel.
    const panelQueue = vi.fn<(intent: EditorOperationIntent) => void>();
    const panel = render(<GuardedRemoval model={model} selection={target} onQueueIntent={panelQueue} />);
    const panelButton = screen.getByRole("button", { name: "Queue removal for review" });
    await waitFor(() => expect(panelButton).toBeEnabled());
    fireEvent.click(panelButton);
    panel.unmount();

    const env = environment(model, [target]);
    let resolve!: (value: string) => void;
    canonical.mockReturnValueOnce(new Promise<string>((done) => { resolve = done; }));
    render(<EntityContextMenu env={env} target={target} request={request()} onClose={() => {}} />);
    const item = screen.getByTestId("entity-context-menu-item-queue-removal");
    expect(item).toHaveAccessibleDescription("Preparing the current entity snapshot…");
    resolve(JSON.stringify(entity));
    await waitFor(() => expect(item).not.toHaveAttribute("aria-disabled"));
    fireEvent.click(item);
    expect(env.onQueueIntent).toHaveBeenCalledExactlyOnceWith(panelQueue.mock.calls[0][0]);
  });

  it("fails closed when the snapshot cannot be prepared, and waits for a running operation", async () => {
    const model = inventedModel();
    const target: EntityRef = { type: "material", id: "material:X" };
    canonical.mockRejectedValue(new Error("engine absent"));
    const env = environment(model, [target]);
    render(<EntityContextMenu env={env} target={target} request={request()} onClose={() => {}} />);
    const item = screen.getByTestId("entity-context-menu-item-queue-removal");
    await waitFor(() => expect(item).toHaveAccessibleDescription("The removal preview could not be prepared. Check that the operation engine is available."));
    fireEvent.click(item);
    expect(env.onQueueIntent).not.toHaveBeenCalled();
    expect(reasonOf(entityCommandEntries({ ...env, operationBusy: true }, target), "queue-removal")).toBe("Another operation is running.");
  });

  it("states where load cases and combinations are removed, and that the project has no removal", () => {
    const model = inventedModel();
    for (const [target, reason] of [
      [{ type: "load", id: "load:invented" }, "Load cases and combinations are removed in the Load Cases panel."],
      [{ type: "combination", id: "combination:invented" }, "Load cases and combinations are removed in the Load Cases panel."],
      [{ type: "project", id: model.project.id }, "This entity has no removal command."],
      [{ type: "component", id: "component:absent" }, "component:absent is not in the current model."]
    ] as const) {
      expect(reasonOf(entityCommandEntries(environment(model, [target]), target), "queue-removal")).toBe(reason);
    }
  });

  it("disables Hide and Isolate for an entity that is not drawn, with the viewport's reason", () => {
    const model = inventedModel();
    const target: EntityRef = { type: "material", id: "material:X" };
    const entries = entityCommandEntries(environment(model, [target]), target);
    expect(reasonOf(entries, "hide-selection")).toBe("Select at least one valid drawable node, pipe, support, or component.");
    expect(reasonOf(entries, "isolate-selection")).toBe("Select at least one valid drawable node, pipe, support, or component.");
    expect(reasonOf(entries, "fit-selection")).toBe("No visible valid selected geometry can be fitted.");
    const node: EntityRef = { type: "node", id: "node:A" };
    const drawable = entityCommandEntries(environment(model, [node]), node);
    expect(reasonOf(drawable, "hide-selection")).toBeNull();
    expect(reasonOf(drawable, "fit-selection")).toBeNull();
  });
});
