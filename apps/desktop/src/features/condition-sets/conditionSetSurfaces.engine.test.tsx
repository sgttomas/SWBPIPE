// Model document 0.9.0 S3b: the remaining surfaces through the real wasm engine. The
// engineering table's Condition set column, bulk change's "Assign condition set" panel,
// the load manager's kind suggestion and the combination editor's "Operating minus
// sustained difference" template. Base documents are S3a's committed corpus.
import { cleanup, fireEvent, render, screen, within } from "@testing-library/react";
import { afterEach, beforeAll, describe, expect, it, vi } from "vitest";
import type { EditorOperationIntent, OperationOutcome, PreviewModel } from "../../types";
import { applyModelOperation } from "../../services/operationService";
import { computeModelHash } from "../../services/hashService";
import { loadWasmEngine } from "../../services/wasmEngine/loadWasmEngine";
import { sessionCorpusBase } from "../../test-support/conditionSetCorpus";
import { ModelTree } from "../model-tree/ModelTree";
import { PropertyInspector } from "../model-tree/PropertyInspector";
import { BulkChangePanel } from "../bulk-change/BulkChangePanel";
import { LoadCaseManagerPanel } from "../load-cases/LoadCaseManagerPanel";
import { OPERATING_DIFFERENCE_BASIS } from "../load-cases/operatingDifferenceTemplate";

async function applyOutcome(model: PreviewModel, intent: EditorOperationIntent): Promise<OperationOutcome> {
  return applyModelOperation(model, intent, await computeModelHash(model));
}
async function apply(model: PreviewModel, intent: EditorOperationIntent): Promise<PreviewModel> {
  const outcome = await applyOutcome(model, intent);
  expect(outcome.diagnostics.filter((item) => item.severity === "blocking"), JSON.stringify(outcome.diagnostics)).toEqual([]);
  return outcome.applied_model as PreviewModel;
}
function chooseVirtualTarget(panel: HTMLElement, testId: string, value: string) {
  const picker = within(panel).getByTestId(testId);
  fireEvent.change(within(picker).getByRole("combobox"), { target: { value } });
  fireEvent.click(within(picker).getByRole("option", { name: new RegExp(`${value}$`) }));
}
const current = (panel: HTMLElement, testId: string) => within(panel).getByTestId(testId).querySelector(".virtual-target-picker-current")!;

beforeAll(async () => { await loadWasmEngine(); });
afterEach(cleanup);

describe("the engineering table's Condition set column", () => {
  it("assigns and clears a pipe's set from the pipes grid through assign_condition_set", async () => {
    let model = sessionCorpusBase("s3a_02"); // cs:a exists; no pipe carries it
    const queued: EditorOperationIntent[] = [];
    const view = render(<ModelTree model={model} onQueueIntent={(intent) => queued.push(intent)} onSelect={vi.fn()} selection={{ type: "pipe", id: "P2" }} />);
    fireEvent.click(screen.getByTestId("layout-mode-grid"));
    fireEvent.click(screen.getByTestId("entity-grid-type-pipes"));
    const cell = screen.getByTestId("entity-grid-input-P2-condition-set") as HTMLSelectElement;
    expect([...cell.options].map((option) => [option.value, option.textContent])).toEqual([["", "not stated"], ["cs:a", "cs:a"]]);
    expect(cell).toHaveValue("");
    fireEvent.change(cell, { target: { value: "cs:a" } });
    fireEvent.click(screen.getByTestId("queue-entity-grid-intents"));
    expect(queued).toHaveLength(1);
    expect(queued[0].target).toEqual({ object_type: "Element", ref: "P2" });
    expect(queued[0].change).toMatchObject({ change_kind: "assign_condition_set", field_path: "condition_set_ref", before: "not_present", after: JSON.stringify({ condition_set_ref: "cs:a" }), unit: "none", dimension: "dimensionless" });
    model = await apply(model, queued[0]);
    expect(model.pipe_segments.find((pipe) => pipe.id === "P2")?.condition_set_ref).toBe("cs:a");

    view.rerender(<ModelTree model={model} onQueueIntent={(intent) => queued.push(intent)} onSelect={vi.fn()} selection={{ type: "pipe", id: "P2" }} />);
    fireEvent.change(screen.getByTestId("entity-grid-input-P2-condition-set"), { target: { value: "" } });
    fireEvent.click(screen.getByTestId("queue-entity-grid-intents"));
    expect(queued).toHaveLength(2);
    expect(queued[1].change).toMatchObject({ before: JSON.stringify({ condition_set_ref: "cs:a" }), after: "not_present" });
    model = await apply(model, queued[1]);
    expect(model.pipe_segments.find((pipe) => pipe.id === "P2")).not.toHaveProperty("condition_set_ref");
  });

  it("shows the Condition sets section in the model tree", () => {
    render(<ModelTree model={sessionCorpusBase("s3a_04")} onQueueIntent={vi.fn()} onSelect={vi.fn()} selection={{ type: "pipe", id: "P1" }} />);
    expect(screen.getByTestId("condition-sets-section")).toBeInTheDocument();
    expect(screen.getByTestId("condition-set-summary-cs:a")).toHaveTextContent("carried by P1, P3");
  });
});

describe("the pipe inspector's Condition set field", () => {
  it("shows the pipe's set in its properties and offers assignment from the model's sets", () => {
    const model = sessionCorpusBase("s3a_04"); // P1 carries cs:a; P2 carries none
    // The inspector's existing label editor reads a pipe label, which the corpus omits.
    for (const pipe of model.pipe_segments) pipe.label ??= pipe.id;
    const view = render(<PropertyInspector model={model} onQueueIntent={vi.fn()} selection={{ type: "pipe", id: "P1" }} />);
    expect(screen.getByTestId("pipe-condition-set-current")).toHaveTextContent("Carries condition set cs:a.");
    expect(screen.getByRole("region", { name: "Pipe condition set" })).toBeInTheDocument();
    expect(screen.getByText("Condition set", { selector: "dt" }).nextElementSibling).toHaveTextContent("cs:a");
    view.rerender(<PropertyInspector model={model} onQueueIntent={vi.fn()} selection={{ type: "pipe", id: "P2" }} />);
    expect(screen.getByText("Condition set", { selector: "dt" }).nextElementSibling).toHaveTextContent("not stated");
    expect(screen.getByLabelText("Condition set", { selector: "select" })).toHaveValue("");
  });
});

describe("bulk change: Assign condition set", () => {
  it("previews each pipe's before and after, refuses non-pipes, and queues one batch", async () => {
    const model = sessionCorpusBase("s3a_04"); // P1 and P3 carry cs:a
    const batches: Array<{ operations: EditorOperationIntent[] }> = [];
    render(<BulkChangePanel model={model} scope={[{ type: "pipe", id: "P1" }, { type: "pipe", id: "P2" }]} onQueueBatch={(batch) => { batches.push(batch); }} />);
    fireEvent.change(screen.getByTestId("bulk-change-kind"), { target: { value: "condition-set" } });
    expect(screen.getByTestId("bulk-change-blocked")).toHaveTextContent("Choose a condition set.");
    fireEvent.change(screen.getByTestId("bulk-change-condition-set"), { target: { value: "cs:a" } });
    expect(screen.getByTestId("bulk-change-count")).toHaveTextContent("1 entity affected; 1 unchanged; 0 not matched; 0 refused.");
    expect(within(screen.getByTestId("bulk-change-affected")).getAllByRole("row")[1]).toHaveTextContent("P2not statedcs:a");
    fireEvent.click(screen.getByTestId("bulk-change-queue"));
    await screen.findByTestId("bulk-change-message");
    expect(batches).toHaveLength(1);
    expect(batches[0].operations.map((operation) => [operation.target.ref, operation.change.change_kind, operation.change.after])).toEqual([["P2", "assign_condition_set", JSON.stringify({ condition_set_ref: "cs:a" })]]);

    fireEvent.change(screen.getByTestId("bulk-change-condition-set"), { target: { value: "\u0000clear" } });
    expect(screen.getByTestId("bulk-change-count")).toHaveTextContent("1 entity affected; 1 unchanged");
    expect(within(screen.getByTestId("bulk-change-affected")).getAllByRole("row")[1]).toHaveTextContent("P1cs:anot stated (cleared)");
  });
});

describe("the load manager's kind suggestion and difference template", () => {
  it("offers operating among the kind suggestions, as vocabulary only", () => {
    const model = sessionCorpusBase("s3a_01");
    render(<LoadCaseManagerPanel model={model} onQueueIntent={vi.fn()} onSelect={vi.fn()} selection={{ type: "load", id: "SUS" }} />);
    const suggestions = [...screen.getByTestId("load-manager-kind-suggestions").querySelectorAll("option")].map((option) => option.getAttribute("value"));
    expect(suggestions).toContain("operating");
    expect(screen.getByTestId("load-manager-create-load-kind")).not.toHaveValue("operating");
  });

  it("writes exactly the chosen operating and sustained cases plus difference_basis, choosing neither, and the engine stores it", async () => {
    let model = sessionCorpusBase("s3a_01");
    const queued: EditorOperationIntent[] = [];
    render(<LoadCaseManagerPanel model={model} onQueueIntent={(intent) => queued.push(intent)} onSelect={vi.fn()} selection={{ type: "load", id: "SUS" }} />);
    const manager = screen.getByTestId("load-case-manager");
    expect(within(manager).queryByTestId("load-manager-operating-difference")).toBeNull(); // only on a subtraction
    fireEvent.change(within(manager).getByTestId("load-manager-create-combination-basis"), { target: { value: "result_state_subtraction" } });
    const template = within(manager).getByTestId("load-manager-operating-difference");
    expect(template).toHaveTextContent("Operating minus sustained difference");
    // The template is never called "expansion"; that naming belongs to the user's rule pack.
    expect(within(manager).getByRole("region", { name: "Create combination" }).textContent?.toLowerCase()).not.toContain("expansion");
    expect(within(manager).queryByText(/subtrahend_support_state/)).toBeNull();
    expect(within(manager).queryByText(/minuend_final/)).toBeNull();
    fireEvent.click(within(manager).getByTestId("load-manager-operating-difference-toggle"));
    expect(current(manager, "load-manager-create-combination-minuend")).toHaveTextContent("No target selected");
    expect(current(manager, "load-manager-create-combination-subtrahend")).toHaveTextContent("No target selected");
    expect(within(manager).getByTestId("load-manager-operating-difference-note")).toHaveTextContent("withholds the combination with a warning");
    expect(within(manager).getByRole("region", { name: "Create combination" }).textContent?.toLowerCase()).not.toContain("expansion");
    expect(within(manager).getByTestId("queue-create-combination-intent")).toBeDisabled();
    chooseVirtualTarget(manager, "load-manager-create-combination-minuend", "WIND");
    chooseVirtualTarget(manager, "load-manager-create-combination-subtrahend", "SUS");
    fireEvent.click(within(manager).getByTestId("queue-create-combination-intent"));
    expect(queued).toHaveLength(1);
    const payload = JSON.parse(queued[0].change.after);
    expect(payload).toMatchObject({ basis: "result_state_subtraction", minuend_id: "WIND", subtrahend_id: "SUS", difference_basis: OPERATING_DIFFERENCE_BASIS });
    expect(payload).not.toHaveProperty("subtrahend_support_state");
    model = await apply(model, queued[0]);
    const stored = model.combinations!.find((item) => item.id === payload.id)!;
    expect(stored).toMatchObject({ minuend_id: "WIND", subtrahend_id: "SUS", difference_basis: OPERATING_DIFFERENCE_BASIS });
    expect(stored).not.toHaveProperty("subtrahend_support_state");
  });

  it("shows the template refused by name on a document with a pressure contract, and a stored subtrahend_support_state as retained", () => {
    const model = sessionCorpusBase("s3a_12");
    model.combinations = [{ id: "C-DIFF", label: "Difference", basis: "result_state_subtraction", minuend_id: "WIND", subtrahend_id: "SUS", difference_basis: OPERATING_DIFFERENCE_BASIS, subtrahend_support_state: "own", provenance: "invented" }];
    render(<LoadCaseManagerPanel model={model} onQueueIntent={vi.fn()} onSelect={vi.fn()} selection={{ type: "load", id: "SUS" }} />);
    const manager = screen.getByTestId("load-case-manager");
    fireEvent.change(within(manager).getByTestId("load-manager-create-combination-basis"), { target: { value: "result_state_subtraction" } });
    expect(within(manager).getByTestId("load-manager-operating-difference-toggle")).toBeDisabled();
    expect(within(manager).getByTestId("load-manager-operating-difference-refused")).toHaveTextContent("LOAD_COMBINATION_DIFFERENCE_BASIS_EXACT_UNSUPPORTED");
    expect(within(manager).getByTestId("load-manager-combination-C-DIFF")).toHaveTextContent(`difference_basis=${OPERATING_DIFFERENCE_BASIS}; subtrahend_support_state=own (retained as authored; not written by this editor)`);
    expect(within(manager).getByTestId("load-condition-use-refused-exact")).toHaveTextContent("LOAD_CASE_CONDITION_USE_EXACT_UNSUPPORTED");
  });
});
