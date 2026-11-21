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
import { DIRECTION_REMOVAL_FIELD_LABEL, DIRECTION_REMOVAL_REASON, SUPPORT_DIRECTION_FIELD_PATH } from "../hanger-selection/hangerSelection";
import { useWorkspaceSession } from "../workspace/workspaceSession";
import { canonicalJsonString, computeModelHash } from "../../services/hashService";
import { applyModelOperation } from "../../services/operationService";
import * as operationBatchService from "../../services/operationBatchService";
import * as previewService from "../../services/previewService";
import type { EditorOperationIntent, PreviewModel } from "../../types";
import {
  buildDirectionRemovalIntent,
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

describe("the form's words and field names for the removal are hanger selection's", () => {
  it("says the reason by hanger selection's exported constant, and names the removal by its exported field path and label", () => {
    expect(directionRemovalStatement(STORED_TEXT)).toBe(`Adding the hanger removes this support's stored direction ${STORED_TEXT}. ${DIRECTION_REMOVAL_REASON}`);
    expect(directionRemovalStatement(STORED_TEXT)).toBe(STATEMENT);
    expect(buildDirectionRemovalIntent("support:s", "{}", "test").change).toMatchObject({
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

  it.each([
    ["the hanger box", () => fireEvent.click(screen.getByLabelText("Include hanger data"))],
    ...HANGER_FAMILIES.map(family => [`the family ${family}`, () => enter("Support family", family)] as const)
  ])("by %s after a direction was typed: the direction is cleared with a word, and the creation the engine applies carries none", async (_path, chooseHanger) => {
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

describe("editing a support that holds a stored direction into one that carries a hanger", () => {
  async function queueHangerEdit(model: PreviewModel, chooseHanger: () => void) {
    const queue = vi.fn();
    render(<SupportConfigurationForm model={model} selection={{ type: "support", id: model.supports[0].id }} onQueueIntent={queue} />);
    expect(screen.getByLabelText("Stored direction")).toHaveTextContent(STORED_TEXT);
    expect(screen.queryByTestId("support-direction-removal")).toBeNull();
    chooseHanger();
    // Stated beside the stored direction line before anything is queued.
    expect(screen.getByTestId("support-direction-removal")).toHaveTextContent(STATEMENT);
    expect(screen.getByTestId("support-direction-removal").previousElementSibling).toBe(screen.getByTestId("support-direction-stored"));
    expect(queue).not.toHaveBeenCalled();
    fireEvent.click(screen.getByText("Queue support configuration"));
    await waitFor(() => expect(queue).toHaveBeenCalledTimes(2));
    return intentsOf(queue);
  }

  it.each([
    ["the hanger box", () => fireEvent.click(screen.getByLabelText("Include hanger data"))],
    ...HANGER_FAMILIES.map(family => [`the family ${family}`, () => enter("Support family", family)] as const)
  ])("by %s: queues the removal then the configuration; the engine applies both and the support holds the hanger and no direction", async (_path, chooseHanger) => {
    const model = await skewModel(), original = JSON.stringify(model), id = model.supports[0].id;
    const [removal, configuration] = await queueHangerEdit(model, chooseHanger);
    expect(removal).toMatchObject({
      operation_kind: "modify", target: { object_type: "Support", ref: id },
      change: { change_kind: "update_support", field_label: DIRECTION_REMOVAL_FIELD_LABEL, field_path: "direction",
        before: await canonicalJsonString(STORED), after: "not_present", unit: "none", dimension: "dimensionless" }
    });
    expect(removal.change.before).toBe('{"x":0.1,"y":-0.2,"z":5e-300}');
    expect(configuration.change).toMatchObject({ change_kind: "update_support", field_path: "configuration", before: await canonicalJsonString(projection(model.supports[0])) });
    expect(configuration.change.after).not.toContain("direction");
    // The engine refuses the configuration alone, so the form never queues it alone.
    expect((await applyModelOperation(model, configuration, null)).diagnostics.map(d => d.code)).toContain("SUPPORT_DIRECTION_RECORD_UNSUPPORTED");
    // Applied in queue order, one change at a time.
    const withoutDirection = expectApplied(await applyModelOperation(model, removal, null));
    expect(withoutDirection.supports[0]).not.toHaveProperty("direction");
    const applied = expectApplied(await applyModelOperation(withoutDirection, configuration, null)).supports[0];
    expect(applied).not.toHaveProperty("direction");
    expect(applied.hanger !== undefined || HANGER_FAMILIES.includes(String(applied.family))).toBe(true);
    // And as one batch, as Review all queued changes as one batch applies them.
    const batch = expectApplied(await applyOperationBatch(model, { batch_id: "batch:test", operations: [removal, configuration] }, await computeModelHash(model)));
    expect(batch.supports[0]).toEqual(applied);
    expect(JSON.stringify(model)).toBe(original);
  });

  it("on a stored record that already holds both a hanger and a direction: offers the removal with the same statement, and never queues the configuration alone", async () => {
    const model = await skewModel(), id = model.supports[0].id;
    Object.assign(model.supports[0], { hanger: { hanger_type: "variable_spring_hanger", source_reference: "invented catalogue" } });
    const queue = vi.fn();
    render(<SupportConfigurationForm model={model} selection={{ type: "support", id }} onQueueIntent={queue} />);
    expect(screen.getByTestId("support-direction-removal")).toHaveTextContent(STATEMENT);
    enter("Support provenance", "revised invented provenance");
    fireEvent.click(screen.getByText("Queue support configuration"));
    await waitFor(() => expect(queue).toHaveBeenCalledTimes(2));
    const [removal, configuration] = intentsOf(queue);
    expect(removal.change).toMatchObject({ field_path: "direction", after: "not_present" });
    expect(configuration.change.field_path).toBe("configuration");
    const applied = expectApplied(await applyOperationBatch(model, { batch_id: "batch:test", operations: [removal, configuration] }, await computeModelHash(model))).supports[0];
    expect(applied).not.toHaveProperty("direction");
    expect(applied.provenance).toBe("revised invented provenance");
  });

  it("leaves a directed support that gains no hanger, and a hanger without a direction, as they were: the configuration alone", async () => {
    const directed = await skewModel();
    const queue = vi.fn();
    render(<SupportConfigurationForm model={directed} selection={{ type: "support", id: directed.supports[0].id }} onQueueIntent={queue} />);
    enter("Support provenance", "revised");
    expect(screen.queryByTestId("support-direction-removal")).toBeNull();
    fireEvent.click(screen.getByText("Queue support configuration"));
    await waitFor(() => expect(queue).toHaveBeenCalledOnce());
    expect(intentsOf(queue)[0].change.field_path).toBe("configuration");
    expect(expectApplied(await applyModelOperation(directed, intentsOf(queue)[0], null)).supports[0].direction).toEqual(STORED);
    cleanup();
    const plain = await skewModel();
    delete (plain.supports[0] as { direction?: unknown }).direction;
    Object.assign(plain.supports[0], { hanger: { hanger_type: "variable_spring_hanger", source_reference: "invented catalogue" } });
    const again = vi.fn();
    render(<SupportConfigurationForm model={plain} selection={{ type: "support", id: plain.supports[0].id }} onQueueIntent={again} />);
    expect(screen.queryByTestId("support-direction-stored")).toBeNull();
    expect(screen.queryByTestId("support-direction-removal")).toBeNull();
    enter("Support provenance", "revised");
    fireEvent.click(screen.getByText("Queue support configuration"));
    await waitFor(() => expect(again).toHaveBeenCalledOnce());
    expect(intentsOf(again)[0].change.field_path).toBe("configuration");
  });

  it("is one Undo step per applied change, each restoring the hash it had", async () => {
    const source = await skewModel();
    vi.spyOn(previewService, "loadPreviewModel").mockResolvedValue(structuredClone(source));
    let session: ReturnType<typeof useWorkspaceSession> | undefined;
    function Workspace() {
      session = useWorkspaceSession();
      const model = session.model.model;
      return model ? <SupportConfigurationForm model={model} selection={{ type: "support", id: source.supports[0].id }}
        queuedIntents={session.operations.editorIntents} onQueueIntent={session.operations.handleQueueEditorIntent}
        operationBusy={session.operations.operationBusy} getPreparationEpoch={session.operations.getPreparationEpoch} /> : null;
    }
    render(<Workspace />);
    await screen.findByLabelText("Stored direction");
    const ops = () => session!.operations;
    const hash = async () => (await computeModelHash(session!.model.model!))!.value;
    const originalHash = await hash();
    fireEvent.click(screen.getByLabelText("Include hanger data"));
    fireEvent.click(screen.getByText("Queue support configuration"));
    await waitFor(() => expect(ops().editorIntents).toHaveLength(2));
    // The session lists the newest first: the configuration above the removal queued ahead of it.
    const [configuration, removal] = ops().editorIntents;
    expect([removal.change.field_path, configuration.change.field_path]).toEqual(["direction", "configuration"]);
    await act(async () => { expect(await ops().handleApplyIntent(removal)).toBe(true); });
    const removedHash = await hash();
    expect(session!.model.model!.supports[0]).not.toHaveProperty("direction");
    await act(async () => { expect(await ops().handleApplyIntent(configuration)).toBe(true); });
    expect(session!.model.model!.supports[0].hanger).toBeDefined();
    expect(ops().undoStack).toHaveLength(2);
    act(() => ops().handleUndoSessionModelEdit());
    expect(await hash()).toBe(removedHash);
    act(() => ops().handleUndoSessionModelEdit());
    expect(await hash()).toBe(originalHash);
    expect(session!.model.model!.supports[0].direction).toEqual(STORED);
  });
});

describe("the engine's refusals of the pair, shown by the form's own statements", () => {
  async function submitHangerEdit(model: PreviewModel) {
    const queue = vi.fn();
    render(<SupportConfigurationForm model={model} selection={{ type: "support", id: model.supports[0].id }} onQueueIntent={queue} />);
    fireEvent.click(screen.getByLabelText("Include hanger data"));
    fireEvent.click(screen.getByText("Queue support configuration"));
    return queue;
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
    const queue = await submitHangerEdit(model);
    const error = await screen.findByTestId("support-direction-removal-engine-error");
    expect(check).toHaveBeenCalledOnce();
    expect(error).toHaveTextContent(directionRemovalRefusedText(STORED_TEXT));
    expect(error).toHaveTextContent(directionRemovalStatement(STORED_TEXT));
    const shown = screen.getByLabelText("Support configuration").textContent!;
    expect(shown).not.toContain(refusal.remediation);
    expect(shown).not.toContain("malformed model-data slot");
    expect(shown).not.toContain("(at supports[0].direction)");
    expect(screen.queryByTestId("support-direction-removal-engine-diagnostic-SUPPORT_DIRECTION_RECORD_UNSUPPORTED")).toBeNull();
    expect(queue).not.toHaveBeenCalled();
  });

  it("OP-BEFORE-VALUE-MISMATCH on the removal: the stored direction changed; reopen the support", async () => {
    const model = await skewModel(), support = model.supports[0];
    // The engine's own answer to a removal whose before is a direction the record no longer holds.
    const stale = buildDirectionRemovalIntent(support.id, await canonicalJsonString({ x: 0, y: 1, z: 0 }), "stale");
    const before = projection(support);
    const configuration = makeRichIntent({ object_type: "Support", ref: support.id }, "update_support", "configuration", await canonicalJsonString(before), { ...before, hanger: {} }, "Support configuration");
    const answer = await validateOperationBatch(model, { batch_id: "batch:stale", operations: [stale, configuration] }, await computeModelHash(model));
    const codes = [...answer.diagnostics, ...answer.operation_outcomes.flatMap(step => step.diagnostics)].map(d => d.code);
    expect(codes, JSON.stringify(answer.diagnostics)).toContain("OP-BEFORE-VALUE-MISMATCH");
    vi.spyOn(operationBatchService, "validateOperationBatch").mockResolvedValueOnce(answer);
    const queue = await submitHangerEdit(model);
    expect(await screen.findByTestId("support-direction-removal-engine-error")).toHaveTextContent(STORED_DIRECTION_CHANGED_TEXT);
    expect(within(screen.getByLabelText("Support configuration")).queryByTestId("support-direction-removal-engine-diagnostic-OP-BEFORE-VALUE-MISMATCH")).toBeNull();
    expect(queue).not.toHaveBeenCalled();
  });

  it("the form's own check, through the engine, admits the pair it builds and queues it", async () => {
    const model = await skewModel();
    const queue = await submitHangerEdit(model);
    await waitFor(() => expect(queue).toHaveBeenCalledTimes(2));
    expect(screen.getByRole("status")).toHaveTextContent("checked together by the operation engine");
    expect(screen.queryByTestId("support-direction-removal-engine-error")).toBeNull();
  });
});
