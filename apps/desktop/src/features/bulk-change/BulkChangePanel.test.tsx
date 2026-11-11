/** B2 (DEL-07-02 Design `bulk-change.md`): the panel's preview counts and per-entity list, the
 * all-or-nothing queue, and the refusal to queue on a changed basis. */
import { act, fireEvent, render, screen, waitFor, within } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";
import { bulkChangeModel } from "../../test-support/bulkChangeModel";
import type { EntityRef } from "../../types";
import { stableJson } from "../model-data/modelDataIntents";
import { BulkChangePanel } from "./BulkChangePanel";

const canonicalize = async (value: unknown) => `canon:${stableJson(value)}`;
const pipes: EntityRef[] = [{ type: "pipe", id: "pipe:P1" }, { type: "pipe", id: "pipe:P2" }, { type: "pipe", id: "pipe:P3" }];

function choose(testId: string, value: string) {
  fireEvent.change(screen.getByTestId(testId), { target: { value } });
}

describe("BulkChangePanel", () => {
  it("previews the count of affected entities with each one's before and after", () => {
    render(<BulkChangePanel model={bulkChangeModel()} scope={pipes} onQueueBatch={vi.fn()} canonicalize={canonicalize} />);
    expect(screen.getByTestId("bulk-change-scope")).toHaveTextContent("3 selected entities");
    choose("bulk-change-to-family", "line_stop");
    fireEvent.click(screen.getByTestId("bulk-change-dof-UX"));
    // The pipes reach the guides at B and C; the anchor at A and the spring at D are not matched.
    expect(screen.getByTestId("bulk-change-count")).toHaveTextContent("2 entities affected; 0 unchanged; 2 not matched; 0 refused.");
    const rows = within(screen.getByTestId("bulk-change-affected")).getAllByRole("row").slice(1).map((row) => row.textContent);
    expect(rows).toEqual([
      "support:Bguide; restraints UY, UZline stop; restraints UX",
      "support:Cguide; restraints UY, UZline stop; restraints UX"
    ]);
    choose("bulk-change-kind", "material");
    choose("bulk-change-material", "material:alloy");
    expect(screen.getByTestId("bulk-change-count")).toHaveTextContent("3 entities affected; 0 unchanged; 0 not matched; 0 refused.");
  });

  it("starts with no restraint ticked and refuses to queue until one is", () => {
    render(<BulkChangePanel model={bulkChangeModel()} scope={pipes} onQueueBatch={vi.fn()} canonicalize={canonicalize} />);
    choose("bulk-change-to-family", "anchor");
    for (const dof of ["UX", "UY", "UZ", "RX", "RY", "RZ"]) expect(screen.getByTestId(`bulk-change-dof-${dof}`)).not.toBeChecked();
    expect(screen.getByTestId("bulk-change-queue")).toBeDisabled();
    expect(screen.getByTestId("bulk-change-blocked")).toHaveTextContent("Tick the restraints to write; none are inferred from the kind.");
    fireEvent.click(screen.getByTestId("bulk-change-restraints-keep"));
    expect(screen.getByTestId("bulk-change-queue")).toBeEnabled();
  });

  it("queues nothing while any entity in scope is refused", async () => {
    const onQueueBatch = vi.fn();
    render(<BulkChangePanel model={bulkChangeModel()} scope={[...pipes, { type: "support", id: "support:B" }]} onQueueBatch={onQueueBatch} canonicalize={canonicalize} />);
    choose("bulk-change-kind", "material");
    choose("bulk-change-material", "material:alloy");
    expect(screen.getByTestId("bulk-change-count")).toHaveTextContent("3 entities affected; 0 unchanged; 0 not matched; 1 refused.");
    expect(screen.getByTestId("bulk-change-refused")).toHaveTextContent("support:B: A material is assigned to pipes only; support support:B is not one.");
    const queue = screen.getByTestId("bulk-change-queue");
    expect(queue).toBeDisabled();
    expect(queue).toHaveAccessibleDescription(/Nothing is queued while any entity is refused/);
    fireEvent.click(queue);
    await act(async () => {});
    expect(onQueueBatch).not.toHaveBeenCalled();
  });

  it("queues exactly one batch with one operation per affected entity", async () => {
    const onQueueBatch = vi.fn();
    render(<BulkChangePanel model={bulkChangeModel()} scope={pipes} onQueueBatch={onQueueBatch} canonicalize={canonicalize} />);
    choose("bulk-change-kind", "tag");
    fireEvent.change(screen.getByTestId("bulk-change-tag"), { target: { value: "what-if" } });
    fireEvent.click(screen.getByTestId("bulk-change-queue"));
    await waitFor(() => expect(onQueueBatch).toHaveBeenCalledTimes(1));
    const [batch] = onQueueBatch.mock.calls[0];
    expect(batch.batch_id).toBe("bulk-tag-1");
    expect(batch.operations.map((operation: { target: { ref: string }; change: { field_path: string } }) => [operation.target.ref, operation.change.field_path]))
      .toEqual([["pipe:P1", "tags"], ["pipe:P2", "tags"], ["pipe:P3", "tags"]]);
    expect(screen.getByTestId("bulk-change-message")).toHaveTextContent("3 changes queued as one batch for review.");
  });

  it("queues nothing when the model changes while the batch is being built", async () => {
    const onQueueBatch = vi.fn();
    let release: () => void = () => {};
    const gate = new Promise<void>((resolve) => { release = resolve; });
    const slow = async (value: unknown) => { await gate; return canonicalize(value); };
    const view = render(<BulkChangePanel model={bulkChangeModel()} scope={pipes} onQueueBatch={onQueueBatch} canonicalize={slow} />);
    choose("bulk-change-kind", "section");
    choose("bulk-change-section", "section:S1");
    fireEvent.click(screen.getByTestId("bulk-change-queue"));
    view.rerender(<BulkChangePanel model={bulkChangeModel()} scope={pipes} onQueueBatch={onQueueBatch} canonicalize={slow} />);
    await act(async () => { release(); await gate; });
    await waitFor(() => expect(screen.getByTestId("bulk-change-message")).toHaveTextContent("The model or the selection changed. Review the preview and queue again."));
    expect(onQueueBatch).not.toHaveBeenCalled();
  });

  it("takes focus on each focus request", () => {
    const view = render(<BulkChangePanel model={bulkChangeModel()} scope={pipes} onQueueBatch={vi.fn()} focusRequest={0} />);
    expect(screen.getByTestId("bulk-change")).not.toHaveFocus();
    view.rerender(<BulkChangePanel model={bulkChangeModel()} scope={pipes} onQueueBatch={vi.fn()} focusRequest={1} />);
    expect(screen.getByTestId("bulk-change")).toHaveFocus();
  });
});
