import { cleanup, fireEvent, render, screen, waitFor } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";
import { SupportConfigurationForm } from "../support-configuration/SupportConfigurationForm";
import { makeRichIntent } from "./formSupport";
import { canonicalJsonString } from "../../services/hashService";
import { applyModelOperation } from "../../services/operationService";
import { loadPreviewModel } from "../../services/previewService";

// No hash/operation mocks: these checks require the parent's accepted rebuilt Wasm.
// Operation validation/application is covered here; native physics has separate evidence.
afterEach(cleanup);
const enter = (label: string, value: string) => fireEvent.change(screen.getByLabelText(label), { target: { value } });

describe("support family shared-operation engine parity", () => {
  it.each([["line_stop", "UX"], ["vertical_support", "UZ"]])("creates and updates canonical %s with explicit %s", async (family, dof) => {
    const model = await loadPreviewModel(), original = JSON.stringify(model), queue = vi.fn();
    render(<SupportConfigurationForm model={model} selection={{ type: "node", id: model.nodes[0].id }} onQueueIntent={queue} />);
    enter("Support ID", "support:family-engine-test"); enter("Support label", "Invented family test"); enter("Support provenance", "original synthetic test");
    enter("Support family", family);
    fireEvent.click(screen.getByLabelText(`Restrain ${dof}`));
    fireEvent.click(screen.getByText("Queue support creation"));
    await waitFor(() => expect(queue).toHaveBeenCalledOnce());
    const creation = queue.mock.calls[0][0];
    expect(creation.change.before).toBe("not_present");
    const created = await applyModelOperation(model, creation, null);
    expect(created.validation.application_status, JSON.stringify(created.diagnostics)).toBe("applied_to_session_model");
    expect(created.applied_model!.supports.find(s => s.id === creation.target.ref)).toMatchObject({ family, restraints: [dof] });
    expect(JSON.stringify(model)).toBe(original);
    cleanup();

    const updateModel = structuredClone(model);
    const support = updateModel.supports[0];
    support.family = "guide"; support.restraints = [dof];
    const updateOriginal = JSON.stringify(updateModel), updateQueue = vi.fn();
    render(<SupportConfigurationForm model={updateModel} selection={{ type: "support", id: support.id }} onQueueIntent={updateQueue} />);
    enter("Support family", family);
    fireEvent.click(screen.getByText("Queue support configuration"));
    await waitFor(() => expect(updateQueue).toHaveBeenCalledOnce());
    const intent = updateQueue.mock.calls[0][0];
    const before = Object.fromEntries(["family", "restraints", "stiffness", "hanger", "nonlinear", "provenance"].filter(key => key in support).map(key => [key, (support as unknown as Record<string, unknown>)[key]]));
    expect(intent.change.before).toBe(await canonicalJsonString(before));
    const updated = await applyModelOperation(updateModel, intent, null);
    expect(updated.validation.application_status, JSON.stringify(updated.diagnostics)).toBe("applied_to_session_model");
    expect(updated.applied_model!.supports[0]).toMatchObject({ family, restraints: [dof] });
    expect(JSON.stringify(updateModel)).toBe(updateOriginal);
  });

  it.each(["LineStop", "VerticalSupport", "", " line_stop ", "one_way", "gap", "lift_off", "friction"])("rejects noncanonical family %s without mutation", async family => {
    const model = await loadPreviewModel(), support = model.supports[0], original = JSON.stringify(model);
    const before = Object.fromEntries(["family", "restraints", "stiffness", "hanger", "nonlinear", "provenance"].filter(key => key in support).map(key => [key, (support as unknown as Record<string, unknown>)[key]]));
    const intent = makeRichIntent({ object_type: "Support", ref: support.id }, "update_support", "configuration", await canonicalJsonString(before), { ...before, family }, "Unsupported source family");
    const result = await applyModelOperation(model, intent, null);
    expect(result.validation.application_status).not.toBe("applied_to_session_model");
    expect(result.diagnostics.some(diagnostic => /family/i.test(diagnostic.message))).toBe(true);
    expect(JSON.stringify(model)).toBe(original);
  });
});
