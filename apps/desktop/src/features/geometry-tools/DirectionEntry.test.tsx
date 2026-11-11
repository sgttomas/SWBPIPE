/** B5 (DEL-07-01 Design `split-and-slope-entry.md` §3): the end-point entry aid in the Geometry
 * tools' reconnect to a new point. The displayed coordinates are the ones the form takes and the
 * create_node operation emits, bit for bit; nothing is assumed (no up direction, no slope). */
import { cleanup, fireEvent, render, screen, waitFor, within } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";
import { computeModelHash } from "../../services/hashService";
import { applyOperationBatch } from "../../services/operationBatchService";
import { splitPartsModel } from "../../test-support/splitPartsModel";
import { computeDirectionEnd, emptyDirectionDraft, type DirectionDraft } from "./directionArithmetic";
import { chooseVirtualMultiTarget } from "../../test-support/workspaceTestControls";
import { GeometryToolsPanel } from "./GeometryToolsPanel";

afterEach(() => { cleanup(); vi.restoreAllMocks(); });
const enter = (label: string, value: string) => fireEvent.change(screen.getByLabelText(label), { target: { value } });
const P = "Reconnect point";

function openReconnect() {
  const model = splitPartsModel();
  const onQueueBatch = vi.fn();
  render(<GeometryToolsPanel model={model} selection={{ type: "pipe", id: "pipe:H" }} onQueueBatch={onQueueBatch} />);
  enter("Geometry tool", "reconnect");
  chooseVirtualMultiTarget("geometry-source-pipes", "pipe:H");
  enter("Reconnect end", "to");
  fireEvent.click(screen.getByLabelText("To a new point"));
  return { model, onQueueBatch };
}

describe("end point from a slope in the reconnect form", () => {
  it("states that the slope is not saved, requires the up direction, and emits exactly the displayed coordinates", async () => {
    const { onQueueBatch } = openReconnect();
    const aid = screen.getByRole("group", { name: `${P}: end point from a length and direction` });
    expect(aid).toHaveTextContent("Entry aid only. The direction, angle or slope is not saved: the model stores the node coordinates computed below, from node:D.");
    enter(`${P}: Entry method`, "slope");
    expect(screen.getByLabelText(`${P}: Up direction`)).toHaveValue("");
    expect(screen.getByLabelText(`${P}: Along the horizontal direction the line`)).toHaveValue("");
    expect(screen.getByLabelText(`${P}: Slope 1 in n: rise or fall over the horizontal run, n`)).toHaveValue("");
    enter(`${P}: Horizontal run`, "137.3");
    enter(`${P}: Length unit`, "in");
    enter(`${P}: Horizontal direction X`, "3");
    enter(`${P}: Horizontal direction Y`, "-1");
    enter(`${P}: Horizontal direction Z`, "0");
    enter(`${P}: Slope 1 in n: rise or fall over the horizontal run, n`, "96");
    enter(`${P}: Along the horizontal direction the line`, "falls");
    expect(within(aid).getByText("State which global direction is up. The model records no vertical axis, so none is assumed.")).toBeInTheDocument();
    const use = within(aid).getByRole("button", { name: "Use these coordinates for the reconnect point" });
    expect(use).toBeDisabled();
    enter(`${P}: Up direction`, "+z");
    await waitFor(() => expect(use).toBeEnabled());
    const readout = screen.getByLabelText(`${P}: Computed end point`).textContent!;
    const [, x, y, z] = /X (\S+), Y (\S+), Z (\S+)$/.exec(readout)!;
    fireEvent.click(use);
    expect(screen.getByLabelText("New point X")).toHaveValue(x);
    expect(screen.getByLabelText("New point Y")).toHaveValue(y);
    expect(screen.getByLabelText("New point Z")).toHaveValue(z);
    for (const [kind, value] of [["ID", "node:R1"], ["label", "R1"], ["provenance", "invented"]]) enter(`New node ${kind}`, value);
    fireEvent.click(screen.getByRole("button", { name: "Queue geometry batch" }));
    await waitFor(() => expect(onQueueBatch).toHaveBeenCalledOnce());
    const createNode = onQueueBatch.mock.calls[0][0].operations[0];
    expect(createNode.change.change_kind).toBe("create_node");
    expect(createNode.change.unit).toBe("ft");
    const emitted = JSON.parse(createNode.change.after).position;
    expect([Object.is(emitted.x, Number(x)), Object.is(emitted.y, Number(y)), Object.is(emitted.z, Number(z))]).toEqual([true, true, true]);
    // Independent hand value: the fall is the run in ft over 96 (137.3 in = 11.441666… ft).
    expect(Math.abs(emitted.z - (9 - 137.3 / 12 / 96))).toBeLessThan(1e-14);
  });

  it("has no start until the moving end is chosen", () => {
    const model = splitPartsModel();
    render(<GeometryToolsPanel model={model} selection={{ type: "pipe", id: "pipe:H" }} onQueueBatch={vi.fn()} />);
    enter("Geometry tool", "reconnect");
    chooseVirtualMultiTarget("geometry-source-pipes", "pipe:H");
    fireEvent.click(screen.getByLabelText("To a new point"));
    expect(screen.getByText("Choose the source pipe and which end moves; the end that stays is the start.")).toBeInTheDocument();
  });

  // Displayed = emitted = stored = computed, per component with Object.is. "Displayed" is the
  // rendered readout text and the New point fields it fills; "emitted" is the queued create_node;
  // "stored" is the model the wasm engine applies; "computed" is the arithmetic, independently.
  const angle: DirectionDraft = { ...emptyDirectionDraft("m"), mode: "angle", length: "2.71", direction: { x: "0", y: "1", z: "0" }, up: "+z", angle: "7.5", angleUnit: "deg" };
  const slope: DirectionDraft = { ...emptyDirectionDraft("in"), mode: "slope", length: "137.3", direction: { x: "3", y: "-1", z: "0" }, up: "+z", slopeN: "96", slopeSense: "falls" };
  it.each([["angle", angle], ["slope", slope]] as const)("%s: displayed, emitted and stored coordinates are bit-identical (wasm)", async (_name, entry) => {
    const { model, onQueueBatch } = openReconnect();
    enter(`${P}: Entry method`, entry.mode);
    enter(entry.mode === "slope" ? `${P}: Horizontal run` : `${P}: Length along the element`, entry.length);
    enter(`${P}: Length unit`, entry.lengthUnit);
    for (const axis of ["X", "Y", "Z"] as const) enter(`${P}: Horizontal direction ${axis}`, entry.direction[axis.toLowerCase() as "x" | "y" | "z"]);
    enter(`${P}: Up direction`, entry.up);
    if (entry.mode === "angle") { enter(`${P}: Angle from the horizontal (positive rises)`, entry.angle); enter(`${P}: Angle unit`, entry.angleUnit); }
    else { enter(`${P}: Slope 1 in n: rise or fall over the horizontal run, n`, entry.slopeN); enter(`${P}: Along the horizontal direction the line`, entry.slopeSense); }
    const use = screen.getByRole("button", { name: "Use these coordinates for the reconnect point" });
    await waitFor(() => expect(use).toBeEnabled());
    const [, ...readout] = /X (\S+), Y (\S+), Z (\S+)$/.exec(screen.getByLabelText(`${P}: Computed end point`).textContent!)!;
    fireEvent.click(use);
    const fields = (["X", "Y", "Z"] as const).map((axis) => (screen.getByLabelText(`New point ${axis}`) as HTMLInputElement).value);
    for (const [kind, value] of [["ID", "node:R1"], ["label", "R1"], ["provenance", "invented"]]) enter(`New node ${kind}`, value);
    fireEvent.click(screen.getByRole("button", { name: "Queue geometry batch" }));
    await waitFor(() => expect(onQueueBatch).toHaveBeenCalledOnce());
    const batch = onQueueBatch.mock.calls[0][0];
    const emitted = JSON.parse(batch.operations[0].change.after).position;
    const outcome = await applyOperationBatch(model, batch, await computeModelHash(model));
    expect(outcome.validation.application_status, JSON.stringify(outcome.diagnostics)).toBe("applied_to_session_model");
    const stored = outcome.applied_model!.nodes.find((n) => n.id === "node:R1")!.position;
    const computed = (await computeDirectionEnd(entry, model.nodes.find((n) => n.id === "node:D")!.position, "ft")).position;
    (["x", "y", "z"] as const).forEach((axis, i) => {
      const values = [Number(readout[i]), Number(fields[i]), emitted[axis], stored[axis]];
      expect(values.map((value) => Object.is(value, computed[axis])), `${axis}: ${values.join(" / ")} vs ${computed[axis]}`).toEqual([true, true, true, true]);
    });
  });
});
