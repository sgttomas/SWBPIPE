// W-17 slice (b), DEL-02-02 Design `project-unit-selection.md` §3 "Editing":
// a choice is validated against the model it was made on. When the model
// changes while that validation is in flight, the stale response queues
// nothing and the section asks for the unit to be chosen again. The engine's
// answer is held back here so the model can change meanwhile. Invented data only.
import { cleanup, fireEvent, render, screen, waitFor } from "@testing-library/react";
import { afterEach, beforeAll, describe, expect, it, vi } from "vitest";
import type { EditorOperationIntent, OperationOutcome, PreviewModel } from "../../types";
import { buildBlankLocalModelDocument } from "../../services/projectService";
import { loadWasmEngine } from "../../services/wasmEngine/loadWasmEngine";
import { ProjectUnitsSection } from "./ProjectUnitsSection";

const gate = vi.hoisted(() => ({ release: null as null | (() => void), calls: 0 }));

vi.mock("../../services/operationService", async (importOriginal) => {
  const actual = await importOriginal<typeof import("../../services/operationService")>();
  return {
    ...actual,
    validateModelOperation: async (...args: Parameters<typeof actual.validateModelOperation>): Promise<OperationOutcome> => {
      gate.calls += 1;
      const outcome = await actual.validateModelOperation(...args);
      await new Promise<void>((resolve) => {
        gate.release = resolve;
      });
      return outcome;
    }
  };
});

beforeAll(async () => {
  await loadWasmEngine();
});
afterEach(() => {
  cleanup();
  gate.release = null;
  gate.calls = 0;
});

const blank = (): PreviewModel => buildBlankLocalModelDocument(new Date("2026-01-01T00:00:00Z"));

describe("project units section, delayed validation", () => {
  it("queues nothing from a stale response and asks for the unit again", async () => {
    const model = blank();
    const list: EditorOperationIntent[] = [];
    const none: EditorOperationIntent[] = [];
    const queue = (intent: EditorOperationIntent) => list.push(intent);
    const view = render(<ProjectUnitsSection model={model} queuedIntents={none} onQueueIntent={queue} />);
    fireEvent.change(screen.getByTestId("project-units-section-force"), { target: { value: "lbf" } });
    await waitFor(() => expect(gate.release).not.toBeNull());
    expect(screen.getByTestId("project-units-section-force")).toBeDisabled();
    // The model changes (another edit applied) while the engine's answer is pending.
    const changed: PreviewModel = { ...model, project: { ...model.project, units: { ...model.project.units, pressure: "psi" } } };
    view.rerender(<ProjectUnitsSection model={changed} queuedIntents={none} onQueueIntent={queue} />);
    gate.release!();
    expect(await screen.findByTestId("project-units-section-error")).toHaveTextContent(
      "The model or pending changes changed. Choose the unit again."
    );
    expect(list).toEqual([]);
    expect(screen.queryByTestId("project-units-section-notice")).toBeNull();
    // Chosen again on the current model, it is checked and queued.
    await waitFor(() => expect(screen.getByTestId("project-units-section-force")).toBeEnabled());
    gate.release = null;
    fireEvent.change(screen.getByTestId("project-units-section-force"), { target: { value: "lbf" } });
    await waitFor(() => expect(gate.release).not.toBeNull());
    gate.release!();
    await waitFor(() => expect(list).toHaveLength(1));
    expect(list[0].change).toMatchObject({ field_path: "project.units.force", before: "N", after: "lbf" });
    expect(screen.getByTestId("project-units-section-notice")).toHaveTextContent("queued for Review/Apply");
    expect(gate.calls).toBe(2);
  });

  it("queues nothing when the pending changes change while validation is in flight", async () => {
    const model = blank();
    const list: EditorOperationIntent[] = [];
    const none: EditorOperationIntent[] = [];
    const queue = (intent: EditorOperationIntent) => list.push(intent);
    const view = render(<ProjectUnitsSection model={model} queuedIntents={none} onQueueIntent={queue} />);
    fireEvent.change(screen.getByTestId("project-units-section-angle"), { target: { value: "deg" } });
    await waitFor(() => expect(gate.release).not.toBeNull());
    // Another change is queued elsewhere meanwhile: the pending set is no longer the one checked against.
    const other = { change: { change_kind: "set_field", field_path: "label" } } as unknown as EditorOperationIntent;
    view.rerender(<ProjectUnitsSection model={model} queuedIntents={[other]} onQueueIntent={queue} />);
    gate.release!();
    expect(await screen.findByTestId("project-units-section-error")).toHaveTextContent("Choose the unit again.");
    expect(list).toEqual([]);
  });
});
