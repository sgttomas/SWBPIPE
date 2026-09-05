import { cleanup, fireEvent, render, screen, waitFor } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";
import { loadPreviewModel } from "../../services/previewService";
import { computeModelHash } from "../../services/hashService";
import { applyOperationBatch, validateOperationBatch } from "../../services/operationBatchService";
import { BoundaryAuthoringPanel } from "./BoundaryAuthoringPanel";
import { boundaryMembers, buildBoundaryBatch, DOFS, emptyBoundaryDraft, type BoundaryDraft } from "./boundaryDraft";
afterEach(cleanup);
const enter = (label: string, value: string) => fireEvent.change(screen.getByLabelText(label), { target: { value } });
function draft(node: string): BoundaryDraft {
  const value = emptyBoundaryDraft();
  Object.assign(value, { boundaryId: "boundary:test", label: "Invented boundary", kind: "equipment_nozzle", equipmentReference: "invented equipment", nozzleReference: "invented nozzle", node, provenance: "invented UI test", coordinateSystem: "global" });
  for (const dof of DOFS) value.dofs[dof].mode = "free";
  value.dofs.UX.mode = "rigid"; value.dofs.RZ.mode = "rigid";
  value.dofs.UY = { mode: "spring", value: "10", unit: "N/m" };
  return value;
}
describe("boundary authoring canonical members", () => {
  it("makes pristine clear observable without queueing", async () => {
    const model = await loadPreviewModel(); const onQueueBatch = vi.fn();
    render(<BoundaryAuthoringPanel model={model} selection={{ type: "project", id: model.project.id }} onQueueBatch={onQueueBatch} />);
    fireEvent.click(screen.getByRole("button", { name: "Clear / cancel boundary draft" }));
    expect(screen.getByText("Boundary draft cleared. No model changes applied.")).toBeInTheDocument();
    expect(onQueueBatch).not.toHaveBeenCalled();
  });
  it("human choices queue one rigid-first batch with preserved equipment/nozzle metadata in the actual engine", async () => {
    const model = await loadPreviewModel(); const original = JSON.stringify(model); const onQueueBatch = vi.fn();
    render(<BoundaryAuthoringPanel model={model} selection={{ type: "node", id: model.nodes[0].id }} onQueueBatch={onQueueBatch} />);
    expect(screen.getByRole("button", { name: "Queue boundary batch" })).toBeDisabled();
    for (const [label, value] of [["Boundary ID", "boundary:test"], ["Boundary label", "Invented boundary"], ["Boundary kind", "equipment_nozzle"], ["Equipment reference", "invented equipment"], ["Nozzle reference", "invented nozzle"], ["Boundary node", model.nodes[0].id], ["Boundary provenance", "invented UI test"], ["Boundary coordinate system", "global"]]) enter(label, value);
    for (const dof of DOFS) enter(`${dof} mode`, "free");
    expect(screen.getByText(/All DOFs are free/)).toBeInTheDocument();
    enter("UX mode", "rigid"); enter("RZ mode", "rigid"); enter("UY mode", "spring");
    enter("UY stiffness value", "10"); enter("UY stiffness unit", "N/m");
    fireEvent.click(screen.getByRole("button", { name: "Queue boundary batch" }));
    await waitFor(() => expect(onQueueBatch).toHaveBeenCalledOnce());
    const batch = onQueueBatch.mock.calls[0][0];
    expect(batch.operations.map((op: { change: { after: string } }) => JSON.parse(op.change.after))).toEqual(boundaryMembers(model, draft(model.nodes[0].id)));
    const hash = await computeModelHash(model);
    const preview = await validateOperationBatch(model, batch, hash);
    expect(preview.validation.batch_validation_status, JSON.stringify(preview)).toBe("passed"); expect(preview.applied_model).toBeNull();
    const result = await applyOperationBatch(model, batch, hash);
    expect(result.validation.application_status, JSON.stringify(result)).toBe("applied_to_session_model");
    expect(result.applied_model!.supports.slice(-2).map(s => s.id)).toEqual(["boundary:test:rigid", "boundary:test:spring:UY"]);
    expect(result.applied_model!.supports.at(-1)!.boundary_association!.nozzle_reference).toBe("invented nozzle");
    expect(JSON.stringify(model)).toBe(original);
  });
  it("orders mixed translational and rotational springs canonically and trims form references once", async () => {
    const model = await loadPreviewModel(); const value = draft(model.nodes[0].id);
    value.dofs.RX = { mode: "spring", value: "125", unit: "N*m/rad" };
    value.equipmentReference = "  invented equipment  "; value.nozzleReference = "  invented nozzle  ";
    const batch = buildBoundaryBatch(model, value, "batch:mixed");
    expect(batch.operations.map(op => op.target.ref)).toEqual(["boundary:test:rigid", "boundary:test:spring:UY", "boundary:test:spring:RX"]);
    const result = await applyOperationBatch(model, batch, await computeModelHash(model));
    expect(result.validation.application_status, JSON.stringify(result)).toBe("applied_to_session_model");
    expect(result.applied_model!.supports.at(-1)!.stiffness!.value).toEqual({ value: 125, unit: "N*m/rad" });
    expect(result.applied_model!.supports.at(-1)!.boundary_association!.equipment_reference).toBe("invented equipment");
    const blocked = await applyOperationBatch(model, batch, null);
    expect(blocked.applied_model).toBeNull(); expect(blocked.validation.application_status).toBe("blocked");
  });
  it("blocks incompatible spring units atomically and existing IDs before queue", async () => {
    const model = await loadPreviewModel(); const value = draft(model.nodes[0].id); value.dofs.UY.unit = "mm";
    const result = await applyOperationBatch(model, buildBoundaryBatch(model, value, "batch:invalid-units"), await computeModelHash(model));
    expect(result.validation.application_status).toBe("blocked"); expect(result.applied_model).toBeNull();
    value.boundaryId = "boundary:collision";
    model.supports[0].id = "boundary:collision:rigid";
    expect(() => boundaryMembers(model, value)).toThrow(/already exists/);
  });
  it("clears explicit modes and shows actual read-only remaining members after deletion", async () => {
    const model = await loadPreviewModel(); const value = draft(model.nodes[0].id);
    model.supports.push(boundaryMembers(model, value)[1]);
    const onQueueBatch = vi.fn();
    render(<BoundaryAuthoringPanel model={model} selection={{ type: "project", id: model.project.id }} onQueueBatch={onQueueBatch} />);
    expect(screen.getByText("Existing boundary members (1)")).toBeInTheDocument();
    expect(screen.getByText(/boundary:test:spring:UY: boundary:test/)).toBeInTheDocument();
    enter("UX mode", "rigid"); fireEvent.click(screen.getByRole("button", { name: "Clear / cancel boundary draft" }));
    expect(screen.getByLabelText("UX mode")).toHaveValue("");
    expect(screen.getByText("Boundary draft cleared. No model changes applied.")).toBeInTheDocument();
    expect(onQueueBatch).not.toHaveBeenCalled();
  });
});
