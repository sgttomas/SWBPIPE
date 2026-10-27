/** B5 (DEL-07-01 Design `split-and-slope-entry.md` §2): the Split into equal parts panel. The
 * preview shows the exact coordinates the batch writes; queueing hands over exactly the planner's
 * one batch; a refusal or a model change queues nothing; an open request moves focus. */
import { cleanup, fireEvent, render, screen, waitFor, within } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";
import { canonicalJsonString } from "../../services/hashService";
import { splitPartsModel } from "../../test-support/splitPartsModel";
import { chooseVirtualTarget } from "../../test-support/workspaceTestControls";
import { SplitIntoPartsPanel } from "./SplitIntoPartsPanel";
import { buildSplitIntoPartsBatch, emptySplitPartsRequest } from "./splitIntoParts";

afterEach(() => { cleanup(); vi.restoreAllMocks(); });
const enter = (label: string, value: string) => fireEvent.change(screen.getByLabelText(label), { target: { value } });
function fill(parts: string) {
  enter("Number of equal parts", parts);
  enter("New node ID pattern", "node:S{k}");
  enter("New node label pattern", "S{k}");
  enter("New pipe ID pattern", "pipe:S{k}");
  enter("New pipe label pattern", "S-{k}");
  enter("First number", "10");
  enter("Number step", "10");
  enter("Provenance", "invented split");
}

describe("Split into equal parts panel", () => {
  it("previews the exact coordinates and queues exactly the planner's batch", async () => {
    const model = splitPartsModel();
    const onQueueBatch = vi.fn();
    render(<SplitIntoPartsPanel model={model} onQueueBatch={onQueueBatch} />);
    chooseVirtualTarget("split-parts-pipe", "pipe:S");
    fill("3");
    const nodes = await screen.findByTestId("split-parts-nodes");
    expect(within(nodes).getAllByRole("row").slice(1).map((row) => [...row.children].map((cell) => cell.textContent))).toEqual([
      ["1", "node:S10", "S10", "2.5", "-0.3999999999999999", "8.235666666666667"],
      ["2", "node:S20", "S20", "4.8999999999999995", "-2.05", "4.126333333333333"]
    ]);
    expect(screen.getByTestId("split-parts-summary")).toHaveTextContent("2 splits and 1 exact placement");
    fireEvent.click(screen.getByTestId("split-parts-queue"));
    await waitFor(() => expect(onQueueBatch).toHaveBeenCalledOnce());
    const batch = onQueueBatch.mock.calls[0][0];
    const expected = await buildSplitIntoPartsBatch(model, { ...emptySplitPartsRequest("pipe:S"), parts: "3", nodeIdPattern: "node:S{k}", nodeLabelPattern: "S{k}", pipeIdPattern: "pipe:S{k}", pipeLabelPattern: "S-{k}", firstNumber: "10", step: "10", provenance: "invented split" }, canonicalJsonString, batch.batch_id);
    expect(batch).toEqual(expected);
    // Displayed equals emitted: the table's text is the placement's value.
    const placement = batch.operations.find((op: { change: { change_kind: string } }) => op.change.change_kind === "set_field");
    expect(Object.is(JSON.parse(placement.change.after).value, Number("4.8999999999999995"))).toBe(true);
    expect(await screen.findByTestId("split-parts-message")).toHaveTextContent("3 operations queued as one batch for review.");
  });

  it("states the refusal and queues nothing", async () => {
    const onQueueBatch = vi.fn();
    render(<SplitIntoPartsPanel model={splitPartsModel()} onQueueBatch={onQueueBatch} />);
    chooseVirtualTarget("split-parts-pipe", "pipe:V");
    fill("4");
    await waitFor(() => expect(screen.getByTestId("split-parts-blocked")).toHaveTextContent("components[0].node"));
    expect(screen.getByTestId("split-parts-queue")).toBeDisabled();
    fireEvent.click(screen.getByTestId("split-parts-queue"));
    expect(onQueueBatch).not.toHaveBeenCalled();
  });

  it("a model change while the batch is built queues nothing", async () => {
    const model = splitPartsModel();
    const onQueueBatch = vi.fn();
    let release: (() => void) | null = null;
    const slow = (value: unknown) => new Promise<string>((resolve) => { release = () => resolve(canonicalJsonString(value)); });
    const { rerender } = render(<SplitIntoPartsPanel model={model} onQueueBatch={onQueueBatch} canonicalize={slow} />);
    chooseVirtualTarget("split-parts-pipe", "pipe:H");
    fill("2");
    await waitFor(() => expect(screen.getByTestId("split-parts-queue")).toBeEnabled());
    fireEvent.click(screen.getByTestId("split-parts-queue"));
    await waitFor(() => expect(release).not.toBeNull());
    rerender(<SplitIntoPartsPanel model={splitPartsModel()} onQueueBatch={onQueueBatch} canonicalize={slow} />);
    release!();
    expect(await screen.findByTestId("split-parts-message")).toHaveTextContent("The model or the split changed.");
    expect(onQueueBatch).not.toHaveBeenCalled();
  });

  it("an open request (Split into N…) chooses the pipe and focuses the parts field", async () => {
    const { rerender } = render(<SplitIntoPartsPanel model={splitPartsModel()} onQueueBatch={vi.fn()} />);
    rerender(<SplitIntoPartsPanel model={splitPartsModel()} onQueueBatch={vi.fn()} openRequest={{ pipeId: "pipe:H", serial: 1 }} />);
    await waitFor(() => expect(screen.getByLabelText("Number of equal parts")).toHaveFocus());
    expect(screen.getByTestId("split-parts-pipe").querySelector(".virtual-target-picker-current")).toHaveTextContent("pipe:H");
  });
});
