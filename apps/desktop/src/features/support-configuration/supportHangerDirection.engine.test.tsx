/** A hanger takes no direction: its load acts along the positive axis of its stiffness DOF (or of its single
 * translational restraint). The support configuration form against the real operation engine (the wasm build of this
 * head, `npm run build:wasm`): a creation that carries a hanger carries no direction, whatever was typed before; an
 * edit that gives a directed support a hanger queues the explicit removal of the stored direction first and the
 * configuration second, never the configuration alone; and the engine's refusals of that pair are shown by the form's
 * own statements. Invented fixtures only. */
import { changeFormControl } from "../../test-support/workspaceTestControls";
import { act, cleanup, fireEvent, render, screen, waitFor, within } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";
import { SupportConfigurationForm } from "./SupportConfigurationForm";
import { makeRichIntent } from "../rich-authoring/formSupport";
import {
  buildDirectionRemovalChange,
  buildHangerSelectionBatch,
  DIRECTION_REMOVAL_FIELD_LABEL,
  DIRECTION_REMOVAL_REASON,
  SUPPORT_DIRECTION_FIELD_PATH,
  type HangerDocument
} from "../hanger-selection/hangerSelection";
import fixture from "../../../../../fixtures/hanger/invented_hanger_library_valid.json";
import { DIRECTION_REMOVAL_BUTTON_TEXT, DIRECTION_REMOVAL_NO_BATCH_TEXT, DIRECTION_REMOVAL_QUEUED_TEXT } from "./directionRemovalQueue";
import { useWorkspaceSession } from "../workspace/workspaceSession";
import { canonicalJsonString, computeModelHash } from "../../services/hashService";
import { applyModelOperation } from "../../services/operationService";
import * as operationBatchService from "../../services/operationBatchService";
import * as previewService from "../../services/previewService";
import type { OperationBatch } from "../../services/operationBatchService";
import type { EditorOperationIntent, PreviewModel } from "../../types";
import {
  DIRECTION_CLEARED_TEXT,
  directionRemovalRefusedText,
  directionRemovalStatement,
  HANGER_NO_DIRECTION_TEXT,
  STORED_DIRECTION_CHANGED_TEXT
} from "./supportDirection";

const { applyOperationBatch, validateOperationBatch } = operationBatchService;

afterEach(() => { cleanup(); vi.restoreAllMocks(); });

const enter = (label: string, value: string) => changeFormControl(screen.getByLabelText(label), { target: { value } });
const CONFIG = ["family", "restraints", "stiffness", "hanger", "nonlinear", "provenance"];
const projection = (support: object) => Object.fromEntries(CONFIG.filter(k => k in support).map(k => [k, (support as Record<string, unknown>)[k]]));
/** Typed text the binary64 parse keeps: 0.1 and 0.2 are inexact, 5e-300 is far from 1. */
const STORED = { x: 0.1, y: -0.2, z: 5e-300 };
const STORED_TEXT = "(0.1, -0.2, 5e-300)";
/** The statement the assignment fixes, word for word. */
const STATEMENT = `Adding the hanger removes this support's stored direction ${STORED_TEXT}. ` +
  "A hanger takes no direction: its load acts along the positive axis of its stiffness degree of freedom (or of its single translational restraint).";
const HANGER_FAMILIES = ["variable_spring_hanger", "spring_hanger", "constant_effort_support"];
/** A complete invented variable spring hanger, as a stored record holds it. */
const STORED_HANGER = {
  hanger_type: "variable_spring_hanger", source_reference: "invented catalogue",
  stiffness: { dof: "UY", value: { value: 35000, unit: "N/m" } },
  installed_load: { value: 1200, unit: "N" }, cold_load: { value: 1200, unit: "N" }, hot_load: { value: 1100, unit: "N" },
  travel_range: { value: 50, unit: "mm" }
};

/** The sample model, with its first support a skewed line stop on UY holding a stored direction. */
async function skewModel(): Promise<PreviewModel> {
  const model = await previewService.loadPreviewModel();
  const support = model.supports[0] as unknown as Record<string, unknown>;
  for (const key of ["stiffness", "hanger", "nonlinear", "properties"]) delete support[key];
  Object.assign(support, { family: "line_stop", restraints: ["UY"], direction: { ...STORED } });
  return model;
}
function expectApplied(outcome: { validation: { application_status: string }; diagnostics: unknown[]; applied_model?: PreviewModel | null }) {
  expect(outcome.validation.application_status, JSON.stringify(outcome.diagnostics)).toBe("applied_to_session_model");
  return outcome.applied_model!;
}
const intentsOf = (queue: ReturnType<typeof vi.fn>) => queue.mock.calls.map(call => call[0] as EditorOperationIntent);

/** Complete invented hanger data, entered through the form's hanger fields: the engine admits a named hanger family
 * only with a hanger object, and a typed hanger only with its required inputs. */
function enterHanger(type: string) {
  if (!screen.queryByRole("group", { name: "Hanger" })) fireEvent.click(screen.getByLabelText("Include hanger data"));
  const group = screen.getByRole("group", { name: "Hanger" });
  expect(group).toBeVisible();
  enter("Hanger type", type);
  const quantity = (label: string, value: string, unit: string) => { enter(`${label} value`, value); enter(`${label} unit`, unit); };
  if (type === "constant_effort_support") quantity("Hanger constant load", "1200", "N");
  else {
    fireEvent.click(within(group).getByText("Add hanger stiffness"));
    enter("Hanger stiffness DOF", "UY");
    quantity("Hanger stiffness quantity", "35000", "N/m");
    quantity("Hanger installed load", "1200", "N"); quantity("Hanger cold load", "1200", "N"); quantity("Hanger hot load", "1100", "N");
  }
  quantity("Hanger travel range", "50", "mm");
  enter("Hanger source reference", "invented catalogue");
}
/** The ways a support comes to carry a hanger: the hanger box alone, or each hanger family with its hanger data. */
const HANGER_PATHS: Array<[string, () => void]> = [
  ["the hanger box", () => fireEvent.click(screen.getByLabelText("Include hanger data"))],
  ...HANGER_FAMILIES.map((family): [string, () => void] => [`the family ${family}`, () => { enter("Support family", family); enterHanger(family); }])
];

describe("the form's words and field names for the removal are hanger selection's", () => {
  it("says the reason by hanger selection's exported constant, and names the removal by its exported field path and label", () => {
    expect(directionRemovalStatement(STORED_TEXT)).toBe(`Adding the hanger removes this support's stored direction ${STORED_TEXT}. ${DIRECTION_REMOVAL_REASON}`);
    expect(directionRemovalStatement(STORED_TEXT)).toBe(STATEMENT);
    expect(buildDirectionRemovalChange("support:s", "{}", "test").change).toMatchObject({
      field_path: SUPPORT_DIRECTION_FIELD_PATH, field_label: DIRECTION_REMOVAL_FIELD_LABEL, source_note: DIRECTION_REMOVAL_REASON
    });
  });
});

describe("creating a support that carries a hanger", () => {
  function renderCreate(model: PreviewModel) {
    const queue = vi.fn();
    render(<SupportConfigurationForm model={model} selection={{ type: "node", id: model.nodes[0].id }} onQueueIntent={queue} />);
    enter("Support ID", "support:hanger-new"); enter("Support label", "Invented hanger"); enter("Support provenance", "original synthetic test");
    fireEvent.click(screen.getByLabelText("Restrain UY"));
    return queue;
  }
  const typeDirection = () => { enter("Direction x", "0.1"); enter("Direction y", "-0.2"); enter("Direction z", "5e-300"); };

  it("offers the direction fields until the hanger box is ticked, then the line in their place; unticking restores them empty", () => {
    renderCreate({ nodes: [{ id: "node:a" }], supports: [], pipe_segments: [], components: [], load_cases: [] } as unknown as PreviewModel);
    expect(screen.getByTestId("support-direction-fields")).toBeVisible();
    fireEvent.click(screen.getByLabelText("Include hanger data"));
    expect(screen.queryByTestId("support-direction-fields")).toBeNull();
    expect(screen.queryByLabelText("Direction x")).toBeNull();
    expect(screen.getByTestId("support-direction-hanger")).toHaveTextContent(HANGER_NO_DIRECTION_TEXT);
    // Nothing had been typed, so nothing is said to have been cleared.
    expect(screen.queryByTestId("support-direction-cleared")).toBeNull();
    fireEvent.click(screen.getByLabelText("Include hanger data"));
    expect(screen.queryByTestId("support-direction-hanger")).toBeNull();
    for (const axis of ["x", "y", "z"]) expect((screen.getByLabelText(`Direction ${axis}`) as HTMLInputElement).value).toBe("");
  });

  it.each(HANGER_PATHS)("by %s after a direction was typed: the direction is cleared with a word, and the creation the engine applies carries none", async (_path, chooseHanger) => {
    const model = await previewService.loadPreviewModel(), original = JSON.stringify(model);
    const queue = renderCreate(model);
    typeDirection();
    chooseHanger();
    expect(screen.queryByLabelText("Direction x")).toBeNull();
    expect(screen.getByTestId("support-direction-hanger")).toHaveTextContent(HANGER_NO_DIRECTION_TEXT);
    expect(screen.getByTestId("support-direction-cleared")).toHaveTextContent(DIRECTION_CLEARED_TEXT);
    fireEvent.click(screen.getByText("Queue support creation"));
    await waitFor(() => expect(queue).toHaveBeenCalledOnce());
    const [creation] = intentsOf(queue);
    expect(creation.change).toMatchObject({ change_kind: "create_support", field_path: "supports" });
    expect(JSON.parse(creation.change.after)).not.toHaveProperty("direction");
    const created = expectApplied(await applyModelOperation(model, creation, null)).supports.find(s => s.id === "support:hanger-new")!;
    expect(created).not.toHaveProperty("direction");
    expect(JSON.stringify(model)).toBe(original);
  });

  it("unticking the box after the direction was cleared offers empty fields again, and the word goes", () => {
    renderCreate({ nodes: [{ id: "node:a" }], supports: [], pipe_segments: [], components: [], load_cases: [] } as unknown as PreviewModel);
    typeDirection();
    fireEvent.click(screen.getByLabelText("Include hanger data"));
    expect(screen.getByTestId("support-direction-cleared")).toBeVisible();
    fireEvent.click(screen.getByLabelText("Include hanger data"));
    expect(screen.queryByTestId("support-direction-cleared")).toBeNull();
    for (const axis of ["x", "y", "z"]) expect((screen.getByLabelText(`Direction ${axis}`) as HTMLInputElement).value).toBe("");
  });
});

/** The form on `model`'s first support, with a single-change queue and a batch queue, both observed. */
function renderEdit(model: PreviewModel, withBatch = true) {
  const intent = vi.fn(), batch = vi.fn();
  render(<SupportConfigurationForm model={model} selection={{ type: "support", id: model.supports[0].id }} onQueueIntent={intent}
    {...(withBatch ? { onQueueBatch: batch } : {})} />);
  return { intent, batch };
}
const batchesOf = (queue: ReturnType<typeof vi.fn>) => queue.mock.calls.map(call => call[0] as OperationBatch);

describe("editing a support that holds a stored direction into one that carries a hanger", () => {
  async function queueHangerEdit(model: PreviewModel, chooseHanger: () => void) {
    const queues = renderEdit(model);
    expect(screen.getByLabelText("Stored direction")).toHaveTextContent(STORED_TEXT);
    expect(screen.queryByTestId("support-direction-removal")).toBeNull();
    chooseHanger();
    // Stated beside the stored direction line before anything is queued.
    expect(screen.getByTestId("support-direction-removal")).toHaveTextContent(STATEMENT);
    expect(screen.getByTestId("support-direction-removal").previousElementSibling).toBe(screen.getByTestId("support-direction-stored"));
    expect(queues.batch).not.toHaveBeenCalled();
    fireEvent.click(screen.getByText(DIRECTION_REMOVAL_BUTTON_TEXT));
    await waitFor(() => expect(queues.batch).toHaveBeenCalledOnce());
    expect(screen.getByTestId("support-direction-removal-engine-notice")).toHaveTextContent(DIRECTION_REMOVAL_QUEUED_TEXT);
    // No single change, ever.
    expect(queues.intent).not.toHaveBeenCalled();
    return batchesOf(queues.batch)[0];
  }

  it.each(HANGER_PATHS)("by %s: queues exactly one batch, the removal then the configuration, and no single change; the engine applies it whole", async (_path, chooseHanger) => {
    const model = await skewModel(), original = JSON.stringify(model), id = model.supports[0].id;
    const batch = await queueHangerEdit(model, chooseHanger);
    expect(batch.operations).toHaveLength(2);
    const [removal, configuration] = batch.operations;
    expect(removal).toMatchObject({
      operation_kind: "modify", target: { object_type: "Support", ref: id },
      change: { change_kind: "update_support", field_label: DIRECTION_REMOVAL_FIELD_LABEL, field_path: "direction",
        before: await canonicalJsonString(STORED), after: "not_present", unit: "none", dimension: "dimensionless" }
    });
    expect(removal.change.before).toBe('{"x":0.1,"y":-0.2,"z":5e-300}');
    expect(removal.rationale).toContain(STATEMENT);
    expect(configuration.change).toMatchObject({ change_kind: "update_support", field_path: "configuration", before: await canonicalJsonString(projection(model.supports[0])) });
    expect(configuration.change.after).not.toContain("direction");
    // The engine refuses the configuration alone, which is why it is never queued alone.
    expect((await applyModelOperation(model, configuration, null)).diagnostics.map(d => d.code)).toContain("SUPPORT_DIRECTION_RECORD_UNSUPPORTED");
    const applied = expectApplied(await applyOperationBatch(model, batch, await computeModelHash(model))).supports[0];
    expect(applied).not.toHaveProperty("direction");
    expect(applied.hanger !== undefined || HANGER_FAMILIES.includes(String(applied.family))).toBe(true);
    expect(JSON.stringify(model)).toBe(original);
  });

  it("on a stored record that already holds both a hanger and a direction: the same statement and the same one batch, never the configuration alone", async () => {
    const model = await skewModel();
    Object.assign(model.supports[0], { family: "variable_spring_hanger", hanger: STORED_HANGER });
    const queues = renderEdit(model);
    expect(screen.getByTestId("support-direction-removal")).toHaveTextContent(STATEMENT);
    enter("Support provenance", "revised invented provenance");
    fireEvent.click(screen.getByText(DIRECTION_REMOVAL_BUTTON_TEXT));
    await waitFor(() => expect(queues.batch).toHaveBeenCalledOnce());
    expect(queues.intent).not.toHaveBeenCalled();
    const [batch] = batchesOf(queues.batch);
    expect(batch.operations.map(operation => operation.change.field_path)).toEqual(["direction", "configuration"]);
    const applied = expectApplied(await applyOperationBatch(model, batch, await computeModelHash(model))).supports[0];
    expect(applied).not.toHaveProperty("direction");
    expect(applied.provenance).toBe("revised invented provenance");
  });

  it("builds the removal with hanger selection's builder: member for member the selection's, but for its identifiers and its own rationale", async () => {
    const model = await skewModel(), id = model.supports[0].id;
    const doc = structuredClone(fixture) as HangerDocument;
    const selection = (await buildHangerSelectionBatch(model, doc, doc.hanger_records[0].hanger_id, { supportId: id, restraints: ["UY"] })).operations[0];
    const form = (await queueHangerEdit(model, () => fireEvent.click(screen.getByLabelText("Include hanger data")))).operations[0];
    const comparable = (operation: EditorOperationIntent) => ({ ...operation, operation_id: "id", rationale: "own", change: { ...operation.change, change_id: "id" } });
    expect(comparable(form)).toEqual(comparable(selection));
    expect(Object.keys(form).sort()).toEqual(Object.keys(selection).sort());
    expect(Object.keys(form.change).sort()).toEqual(Object.keys(selection.change).sort());
  });

  it("without a batch queue refuses in a plain sentence and queues nothing, never two single changes", async () => {
    const model = await skewModel();
    const queues = renderEdit(model, false);
    fireEvent.click(screen.getByLabelText("Include hanger data"));
    fireEvent.click(screen.getByText(DIRECTION_REMOVAL_BUTTON_TEXT));
    expect(await screen.findByTestId("support-direction-removal-engine-error")).toHaveTextContent(DIRECTION_REMOVAL_NO_BATCH_TEXT);
    expect(queues.intent).not.toHaveBeenCalled();
  });

  it("leaves a directed support that gains no hanger, and a hanger without a direction, as they were: the configuration alone", async () => {
    const directed = await skewModel();
    const first = renderEdit(directed);
    enter("Support provenance", "revised");
    expect(screen.queryByTestId("support-direction-removal")).toBeNull();
    fireEvent.click(screen.getByText("Queue support configuration"));
    await waitFor(() => expect(first.intent).toHaveBeenCalledOnce());
    expect(first.batch).not.toHaveBeenCalled();
    expect(intentsOf(first.intent)[0].change.field_path).toBe("configuration");
    expect(expectApplied(await applyModelOperation(directed, intentsOf(first.intent)[0], null)).supports[0].direction).toEqual(STORED);
    cleanup();
    const plain = await skewModel();
    delete (plain.supports[0] as { direction?: unknown }).direction;
    Object.assign(plain.supports[0], { family: "variable_spring_hanger", hanger: STORED_HANGER });
    const second = renderEdit(plain);
    expect(screen.queryByTestId("support-direction-stored")).toBeNull();
    expect(screen.queryByTestId("support-direction-removal")).toBeNull();
    enter("Support provenance", "revised");
    fireEvent.click(screen.getByText("Queue support configuration"));
    await waitFor(() => expect(second.intent).toHaveBeenCalledOnce());
    expect(second.batch).not.toHaveBeenCalled();
    expect(intentsOf(second.intent)[0].change.field_path).toBe("configuration");
  });
});

describe("the batch in a real workspace session", () => {
  type Session = ReturnType<typeof useWorkspaceSession>;
  /** The desktop's own session on the directed model, with the form wired as the inspector wires it. */
  async function workspace(source: PreviewModel) {
    vi.spyOn(previewService, "loadPreviewModel").mockResolvedValue(structuredClone(source));
    const holder: { session?: Session } = {};
    function Workspace() {
      const session = useWorkspaceSession();
      holder.session = session;
      const model = session.model.model;
      return model ? <SupportConfigurationForm model={model} selection={{ type: "support", id: source.supports[0].id }}
        queuedIntents={session.operations.editorIntents} onQueueIntent={session.operations.handleQueueEditorIntent}
        onQueueBatch={session.operations.handleQueueOperationBatch}
        operationBusy={session.operations.operationBusy} getPreparationEpoch={session.operations.getPreparationEpoch} /> : null;
    }
    render(<Workspace />);
    await screen.findByLabelText("Stored direction");
    const ops = () => holder.session!.operations;
    const model = () => holder.session!.model.model!;
    const hash = async () => (await computeModelHash(model()))!.value;
    return { ops, model, hash };
  }
  async function queueFromForm(ops: () => Session["operations"], count: number) {
    if (!screen.queryByRole("group", { name: "Hanger" })) fireEvent.click(screen.getByLabelText("Include hanger data"));
    fireEvent.click(screen.getByText(DIRECTION_REMOVAL_BUTTON_TEXT));
    await waitFor(() => expect(ops().queuedBatches).toHaveLength(count));
    await waitFor(() => expect(screen.getByTestId("support-direction-removal-engine-notice")).toBeVisible());
  }

  it("queues the batch the engine checked, unchanged; one Apply applies both, one Undo restores the hash, Redo reapplies", async () => {
    const source = await skewModel();
    const checked = vi.spyOn(operationBatchService, "validateOperationBatch");
    const { ops, model, hash } = await workspace(source);
    const originalHash = await hash();
    await queueFromForm(ops, 1);
    expect(ops().editorIntents).toHaveLength(0);
    const [entry] = ops().queuedBatches;
    expect(checked).toHaveBeenCalledOnce();
    expect(entry.batch).toEqual(checked.mock.calls[0][1]);
    expect(entry.batch.operations.map(operation => operation.change.field_path)).toEqual(["direction", "configuration"]);
    const undoBefore = ops().undoStack.length;
    await act(async () => { await ops().handleRunOperationBatch(entry, true); });
    await waitFor(() => expect(ops().batchReceipts).toHaveLength(1));
    expect(model().supports[0]).not.toHaveProperty("direction");
    expect(model().supports[0].hanger).toBeDefined();
    expect(ops().undoStack).toHaveLength(undoBefore + 1);
    const appliedHash = await hash();
    act(() => ops().handleUndoSessionModelEdit());
    expect(await hash()).toBe(originalHash);
    expect(model().supports[0].direction).toEqual(STORED);
    act(() => ops().handleRedoSessionModelEdit());
    expect(await hash()).toBe(appliedHash);
    expect(model().supports[0]).not.toHaveProperty("direction");
  });

  it("is applied whole or not at all: after the stored direction changes, Apply refuses and the model, history and receipts stay", async () => {
    const source = await skewModel();
    const { ops, model, hash } = await workspace(source);
    await queueFromForm(ops, 1);
    const [entry] = ops().queuedBatches;
    // The engine refuses the batch whole on a model whose stored direction is another.
    const changed = structuredClone(source);
    Object.assign(changed.supports[0], { direction: { x: 0, y: 1, z: 0 } });
    const refused = await applyOperationBatch(changed, entry.batch, await computeModelHash(changed));
    expect(refused.validation.application_status).toBe("blocked");
    expect(refused.applied_model ?? null).toBeNull();
    expect([...refused.diagnostics, ...refused.operation_outcomes.flatMap(step => step.diagnostics)].map(d => d.code)).toContain("OP-BEFORE-VALUE-MISMATCH");
    // In the session the stored direction changes by an applied change (here its removal on its own); the queued batch
    // was prepared on the earlier model and is not applied.
    await act(async () => { expect(await ops().handleApplyIntent(buildDirectionRemovalChange(source.supports[0].id, await canonicalJsonString(STORED), "test"))).toBe(true); });
    const afterChange = { model: JSON.stringify(model()), hash: await hash(), undo: ops().undoStack.length, receipts: ops().batchReceipts.length };
    await act(async () => { await ops().handleRunOperationBatch(entry, true); });
    expect(JSON.stringify(model())).toBe(afterChange.model);
    expect(await hash()).toBe(afterChange.hash);
    expect(ops().undoStack).toHaveLength(afterChange.undo);
    expect(ops().batchReceipts).toHaveLength(afterChange.receipts);
    expect(ops().queuedBatches[0].retired).toBeTruthy();
    expect(model().supports[0].hanger).toBeUndefined();
  });

  it("a second batch for the same support is not refused by the form, which cannot see queued batches; once the first is applied the second is retired and never applies", async () => {
    const source = await skewModel();
    const { ops, model } = await workspace(source);
    await queueFromForm(ops, 1);
    await queueFromForm(ops, 2);
    const [first, second] = ops().queuedBatches;
    await act(async () => { await ops().handleRunOperationBatch(first, true); });
    await waitFor(() => expect(ops().batchReceipts).toHaveLength(1));
    const applied = JSON.stringify(model());
    expect(ops().queuedBatches.find(entry => entry.key === second.key)!.retired).toBeTruthy();
    await act(async () => { await ops().handleRunOperationBatch(second, true); });
    expect(JSON.stringify(model())).toBe(applied);
    expect(ops().batchReceipts).toHaveLength(1);
  });
});

describe("the engine's refusals of the batch, shown by the form's own statements", () => {
  async function submitHangerEdit(model: PreviewModel) {
    const queues = renderEdit(model);
    fireEvent.click(screen.getByLabelText("Include hanger data"));
    fireEvent.click(screen.getByText(DIRECTION_REMOVAL_BUTTON_TEXT));
    return queues;
  }

  it("SUPPORT_DIRECTION_RECORD_UNSUPPORTED: the form's statement and what to do, never the applier's message tail or its remediation", async () => {
    const model = await skewModel(), support = model.supports[0];
    // The engine's own answer to the configuration alone (a path this form does not take), handed to the form as the
    // answer to its check.
    const before = projection(support);
    const alone = makeRichIntent({ object_type: "Support", ref: support.id }, "update_support", "configuration", await canonicalJsonString(before), { ...before, hanger: {} }, "Support configuration");
    const answer = await validateOperationBatch(model, { batch_id: "batch:alone", operations: [alone] }, await computeModelHash(model));
    const refusal = answer.diagnostics.find(d => d.code === "SUPPORT_DIRECTION_RECORD_UNSUPPORTED")!;
    expect(refusal, JSON.stringify(answer.diagnostics)).toBeDefined();
    expect(refusal.severity).toBe("blocking");
    const check = vi.spyOn(operationBatchService, "validateOperationBatch").mockResolvedValueOnce(answer);
    const queues = await submitHangerEdit(model);
    const error = await screen.findByTestId("support-direction-removal-engine-error");
    expect(check).toHaveBeenCalledOnce();
    expect(error).toHaveTextContent(directionRemovalRefusedText(STORED_TEXT));
    const shown = screen.getByLabelText("Support configuration").textContent!;
    expect(shown).not.toContain(refusal.remediation);
    expect(shown).not.toContain("malformed model-data slot");
    expect(shown).not.toContain("(at supports[0].direction)");
    expect(screen.queryByTestId("support-direction-removal-engine-diagnostic-SUPPORT_DIRECTION_RECORD_UNSUPPORTED")).toBeNull();
    expect(queues.batch).not.toHaveBeenCalled();
    expect(queues.intent).not.toHaveBeenCalled();
  });

  it("OP-BEFORE-VALUE-MISMATCH on the removal: the stored direction changed; reopen the support", async () => {
    const model = await skewModel(), support = model.supports[0];
    // The engine's own answer to a removal whose before is a direction the record no longer holds.
    const stale = buildDirectionRemovalChange(support.id, await canonicalJsonString({ x: 0, y: 1, z: 0 }), "stale");
    const before = projection(support);
    const configuration = makeRichIntent({ object_type: "Support", ref: support.id }, "update_support", "configuration", await canonicalJsonString(before), { ...before, hanger: {} }, "Support configuration");
    const answer = await validateOperationBatch(model, { batch_id: "batch:stale", operations: [stale, configuration] }, await computeModelHash(model));
    const codes = [...answer.diagnostics, ...answer.operation_outcomes.flatMap(step => step.diagnostics)].map(d => d.code);
    expect(codes, JSON.stringify(answer.diagnostics)).toContain("OP-BEFORE-VALUE-MISMATCH");
    vi.spyOn(operationBatchService, "validateOperationBatch").mockResolvedValueOnce(answer);
    const queues = await submitHangerEdit(model);
    expect(await screen.findByTestId("support-direction-removal-engine-error")).toHaveTextContent(STORED_DIRECTION_CHANGED_TEXT);
    expect(within(screen.getByLabelText("Support configuration")).queryByTestId("support-direction-removal-engine-diagnostic-OP-BEFORE-VALUE-MISMATCH")).toBeNull();
    expect(queues.batch).not.toHaveBeenCalled();
    expect(queues.intent).not.toHaveBeenCalled();
  });
});
