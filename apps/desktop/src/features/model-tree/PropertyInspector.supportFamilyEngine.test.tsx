// RT-S1/RT-U1: the `create_support` intent the New support form emits, with its family, passes the
// operation applier's `SUPPORT_FAMILIES` validation (`rich_authoring.rs`) in the shared Wasm engine,
// and a value outside that list is refused. No hash or operation mocks: these checks require the
// accepted rebuilt Wasm. All entities are invented test input.
import { cleanup, fireEvent, render, screen } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";
import { changeFormControl } from "../../test-support/workspaceTestControls";
import type { EditorOperationIntent } from "../../types";
import { DisplayUnitsProvider } from "../display-units";
import { applyModelOperation } from "../../services/operationService";
import { loadPreviewModel } from "../../services/previewService";
import { PropertyInspector } from "./PropertyInspector";

afterEach(cleanup);

async function emitted(choose: (() => void) | null) {
  const model = await loadPreviewModel();
  const node = model.nodes[0].id;
  const queue = vi.fn<(intent: EditorOperationIntent) => void>();
  render(<DisplayUnitsProvider initialPreference="SI"><PropertyInspector model={model} selection={{ type: "node", id: node }} onQueueIntent={queue} /></DisplayUnitsProvider>);
  for (const dof of ["RX", "RY", "RZ"]) fireEvent.click(screen.getByTestId(`create-support-restraint-${dof}`));
  choose?.();
  fireEvent.click(screen.getByTestId("queue-create-support-intent"));
  expect(queue).toHaveBeenCalledOnce();
  return { model, node, intent: queue.mock.calls[0][0] };
}

describe("the New support form's family in the operation applier", () => {
  it.each([["anchor", null], ["guide", "guide"], ["line_stop", "line_stop"], ["vertical_support", "vertical_support"]] as const)(
    "applies the emitted %s support", async (family, pick) => {
      const { model, node, intent } = await emitted(pick ? () => changeFormControl(screen.getByTestId("create-support-family"), { target: { value: pick } }) : null);
      const original = JSON.stringify(model);
      const applied = await applyModelOperation(model, intent, null);
      expect(applied.validation.application_status, JSON.stringify(applied.diagnostics)).toBe("applied_to_session_model");
      expect(applied.applied_model!.supports.find((support) => support.id === intent.target.ref))
        .toMatchObject({ node, family, restraints: ["UX", "UY", "UZ", "RX", "RY", "RZ"] });
      expect(JSON.stringify(model)).toBe(original);
    });

  it("refuses the same intent with a family outside SUPPORT_FAMILIES", async () => {
    const { model, intent } = await emitted(null);
    const after = { ...JSON.parse(intent.change.after), family: "strut" };
    const refused = await applyModelOperation(model, { ...intent, change: { ...intent.change, after: JSON.stringify(after) } }, null);
    expect(refused.validation.application_status).not.toBe("applied_to_session_model");
    expect(JSON.stringify(refused.diagnostics)).toContain("family");
  });
});
