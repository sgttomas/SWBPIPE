// The analysis-basis override entered through the element-state editor, against
// the product's own rule (core/product_physics case_state material selection):
// a member whose selected property temperature differs from its known operating
// temperature is refused without a reason and provenance and resolved with them.
// The edits are queued and applied through the wasm operation engine; the
// product's resolution runs through the wasm model checks, the solve's own
// checks before stiffness formation. Invented values on the committed request
// model.
import { cleanup, fireEvent, render, screen, waitFor } from "@testing-library/react";
import { afterEach, beforeAll, expect, it, vi } from "vitest";
import type { EditorOperationIntent, PreviewModel } from "../../types";
import { computeModelHash } from "../../services/hashService";
import { applyModelOperation, warmupOperationEngine } from "../../services/operationService";
import { loadWasmEngine } from "../../services/wasmEngine/loadWasmEngine";
import { LoadReferenceStateInputs } from "./LoadReferenceStateInputs";
import connected from "../../../../../fixtures/product_preview/pressure/load_reference/connected.request.json";

const REQUIRED = "LOAD_STATE_MATERIAL_TEMPERATURE_OVERRIDE_REQUIRED";
const INVALID = "LOAD_STATE_MATERIAL_OVERRIDE_INVALID";
const change = (label: string, value: string) => fireEvent.change(screen.getByLabelText(label), { target: { value } });
afterEach(cleanup);
beforeAll(async () => {
  expect((await warmupOperationEngine()).state).toBe("ready");
});

async function enterAndApply(model: PreviewModel, edit: () => void): Promise<PreviewModel> {
  const queue = vi.fn();
  render(<LoadReferenceStateInputs model={model} selection={{ type: "load", id: "case:hot" }} onQueueIntent={queue} loadCase={model.load_cases.find(c => c.id === "case:hot")!} />);
  edit();
  fireEvent.click(screen.getByRole("button", { name: "Queue analysis state" }));
  await waitFor(() => expect(queue).toHaveBeenCalledOnce(), { timeout: 20_000 });
  const outcome = await applyModelOperation(model, queue.mock.calls[0][0] as EditorOperationIntent, await computeModelHash(model));
  expect(outcome.diagnostics.filter(d => d.severity === "blocking")).toEqual([]);
  cleanup();
  return outcome.applied_model as PreviewModel;
}

async function blockingCodes(model: PreviewModel): Promise<string[]> {
  const report = JSON.parse((await loadWasmEngine()).modelChecksJson(JSON.stringify(model))) as { checked: boolean; diagnostics?: Array<{ code: string; severity: string }> };
  expect(report.checked).toBe(true);
  return (report.diagnostics ?? []).filter(d => d.severity === "blocking" || d.severity === "error").map(d => d.code).sort();
}

it("a selection temperature other than the operating one is refused without the override, refused with a blank reason, and resolved with the pair entered in the editor", async () => {
  const fixture = structuredClone(connected.model) as unknown as PreviewModel;
  const baseline = await blockingCodes(fixture);
  expect(baseline).not.toContain(REQUIRED);

  // case:hot, pipe:first operates at 150 degC; its property is selected at the 20 degC point.
  let model = await enterAndApply(fixture, () => change("Element state 1 material point", "point:cold"));
  expect(await blockingCodes(model)).toEqual([...baseline, REQUIRED].sort());

  // A blank reason is accepted by the document's validation and refused by the solve's own check.
  model = await enterAndApply(model, () => {
    change("Element state 1 analysis-basis reason", " ");
    change("Element state 1 analysis-basis provenance", "invented test input");
  });
  expect(await blockingCodes(model)).toEqual([...baseline, INVALID].sort());

  model = await enterAndApply(model, () => change("Element state 1 analysis-basis reason", "invented: properties held at the 20 degC point for this comparison"));
  const member = model.load_cases.find(c => c.id === "case:hot")!.analysis_state!.element_states[0];
  expect(member.analysis_basis_override).toEqual({ reason: "invented: properties held at the 20 degC point for this comparison", provenance: "invented test input" });
  expect(await blockingCodes(model)).toEqual(baseline);
}, 120_000);
