import { act, cleanup, fireEvent, render, screen, waitFor } from "@testing-library/react";
import { useState } from "react";
import { afterEach, describe, expect, it, vi } from "vitest";
import * as hashing from "../../services/hashService";
import type { EditorOperationIntent, PreviewModel } from "../../types";
import { QueueFeedback, useRichQueue } from "./formSupport";

const model = {
  project: { id: "project", name: "Project", description: "", units: { length: "m" } },
  schema_version: "0.2.0",
  analysis_status: { mechanics: "NOT_RUN", rule_check: "NOT_RUN", professional_acceptance: "NOT_REVIEWED" },
  materials: [],
  sections: [],
  nodes: [{ id: "node/A", label: "Node A", position: { x: 0, y: 0, z: 0 }, provenance: "test" }],
  pipe_segments: [],
  supports: [{ id: "support/A", label: "Support A", node: "node/A", restraints: ["UX"], provenance: "source" }],
  components: [],
  load_cases: [],
  combinations: [],
  diagnostics: []
} as unknown as PreviewModel;

function QueueHarness({
  getPreparationEpoch,
  onQueueIntent,
  preparationEpoch
}: {
  getPreparationEpoch?: () => number;
  onQueueIntent: (intent: EditorOperationIntent) => void;
  preparationEpoch?: number;
}) {
  const [provenance, setProvenance] = useState("source");
  const state = useRichQueue({
    getPreparationEpoch,
    model,
    onQueueIntent,
    preparationEpoch,
    selection: { type: "support", id: "support/A" }
  });
  return <section>
    <label>Draft provenance<input
      aria-label="Draft provenance"
      disabled={state.busy}
      onChange={(event) => setProvenance(event.target.value)}
      value={provenance}
    /></label>
    <button
      disabled={state.busy}
      onClick={() => { void state.queue(
        { object_type: "Support", ref: "support/A" },
        "update_support",
        "configuration",
        { provenance: "source" },
        { provenance },
        "Support configuration"
      ); }}
      type="button"
    >Queue draft</button>
    <QueueFeedback {...state} />
  </section>;
}

afterEach(() => {
  cleanup();
  vi.restoreAllMocks();
});

describe("useRichQueue preparation epoch", () => {
  it("rejects synchronous membership ABA without an intervening render, retains the draft, and permits a fresh retry", async () => {
    let epoch = 4;
    let finishCanonicalization!: (value: string) => void;
    const canonical = vi.spyOn(hashing, "canonicalJsonString")
      .mockImplementationOnce(() => new Promise<string>((resolve) => { finishCanonicalization = resolve; }))
      .mockImplementation(async (value) => JSON.stringify(value));
    const queued = vi.fn();
    render(<QueueHarness getPreparationEpoch={() => epoch} onQueueIntent={queued} />);
    fireEvent.change(screen.getByLabelText("Draft provenance"), { target: { value: "retained entry" } });
    fireEvent.click(screen.getByRole("button", { name: "Queue draft" }));
    await waitFor(() => expect(canonical).toHaveBeenCalledTimes(1));

    epoch += 1;
    epoch += 1;
    await act(async () => { finishCanonicalization('{"provenance":"source"}'); });

    expect(queued).not.toHaveBeenCalled();
    expect(screen.getByRole("alert")).toHaveTextContent("selection, model or pending changes changed");
    expect(screen.getByLabelText("Draft provenance")).toHaveValue("retained entry");
    expect(screen.getByRole("button", { name: "Queue draft" })).toBeEnabled();

    fireEvent.click(screen.getByRole("button", { name: "Queue draft" }));
    await waitFor(() => expect(queued).toHaveBeenCalledTimes(1));
    expect(queued).toHaveBeenCalledWith(expect.objectContaining({
      target: { object_type: "Support", ref: "support/A" },
      change: expect.objectContaining({
        after: JSON.stringify({ provenance: "retained entry" }),
        before: JSON.stringify({ provenance: "source" })
      })
    }));
    expect(screen.getByRole("status")).toHaveTextContent("queued for validation and review");
  });

  it("publishes exactly once when the preparation context remains unchanged", async () => {
    vi.spyOn(hashing, "canonicalJsonString").mockImplementation(async (value) => JSON.stringify(value));
    const queued = vi.fn();
    render(<QueueHarness getPreparationEpoch={() => 9} onQueueIntent={queued} />);
    fireEvent.change(screen.getByLabelText("Draft provenance"), { target: { value: "unchanged context" } });
    fireEvent.click(screen.getByRole("button", { name: "Queue draft" }));
    await waitFor(() => expect(queued).toHaveBeenCalledTimes(1));
    expect(queued).toHaveBeenCalledWith(expect.objectContaining({
      change: expect.objectContaining({ after: JSON.stringify({ provenance: "unchanged context" }) })
    }));
  });

  it("rejects a rendered epoch transition and never publishes after unmount", async () => {
    let finishFirst!: (value: string) => void;
    let finishSecond!: (value: string) => void;
    vi.spyOn(hashing, "canonicalJsonString")
      .mockImplementationOnce(() => new Promise<string>((resolve) => { finishFirst = resolve; }))
      .mockImplementationOnce(() => new Promise<string>((resolve) => { finishSecond = resolve; }));
    const queued = vi.fn();
    const view = render(<QueueHarness onQueueIntent={queued} preparationEpoch={1} />);
    fireEvent.click(screen.getByRole("button", { name: "Queue draft" }));
    await waitFor(() => expect(screen.getByRole("button", { name: "Queue draft" })).toBeDisabled());
    view.rerender(<QueueHarness onQueueIntent={queued} preparationEpoch={2} />);
    await act(async () => { finishFirst('{"provenance":"source"}'); });
    expect(queued).not.toHaveBeenCalled();
    expect(screen.getByRole("alert")).toHaveTextContent("selection, model or pending changes changed");

    fireEvent.click(screen.getByRole("button", { name: "Queue draft" }));
    await waitFor(() => expect(screen.getByRole("button", { name: "Queue draft" })).toBeDisabled());
    view.unmount();
    await act(async () => { finishSecond('{"provenance":"source"}'); });
    expect(queued).not.toHaveBeenCalled();
  });
});
