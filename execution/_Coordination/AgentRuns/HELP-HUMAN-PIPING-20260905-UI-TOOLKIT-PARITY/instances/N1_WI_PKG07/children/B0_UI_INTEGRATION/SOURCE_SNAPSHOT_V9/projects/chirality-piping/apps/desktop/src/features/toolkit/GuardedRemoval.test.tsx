import { act, fireEvent, render, screen, waitFor } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";
import { SectionAssignment } from "./SectionAssignment";
import { GuardedRemoval } from "./GuardedRemoval";
import { loadPreviewModel } from "../../services/previewService";
const canonical = vi.hoisted(() => vi.fn());
vi.mock("../../services/hashService", () => ({ canonicalJsonString: canonical }));
afterEach(() => canonical.mockReset());
describe("guarded removal", () => {
  it.each(["material", "section", "component"] as const)("queues exact %s identity and whole snapshot without mutating the model", async (type) => {
    const model = await loadPreviewModel();
    model.sections = [{ id: "section:invented", name: "Invented", section_type: "pipe", properties: {}, provenance: "invented test" }];
    const entity = type === "material" ? model.materials![0] : type === "section" ? model.sections![0] : model.components[0];
    const before = JSON.stringify(entity);
    canonical.mockResolvedValue(before);
    const queue = vi.fn(); const unchanged = JSON.stringify(model);
    render(<GuardedRemoval model={model} selection={{ type, id: entity.id }} onQueueIntent={queue} />);
    const button = screen.getByRole("button", { name: "Queue removal for review" });
    await waitFor(() => expect(button).toBeEnabled());
    fireEvent.click(button);
    expect(queue.mock.calls[0][0]).toMatchObject({ operation_kind: "delete", target: { ref: entity.id }, change: { change_kind: `delete_${type}`, before, after: "deleted", unit: "none", dimension: "dimensionless" } });
    expect(canonical).toHaveBeenCalledWith(entity);
    expect(JSON.stringify(model)).toBe(unchanged);
  });
  it("discards stale preparation and fails closed when canonicalization is unavailable", async () => {
    const model = await loadPreviewModel();
    let resolve!: (value: string) => void;
    canonical.mockReturnValueOnce(new Promise<string>((done) => { resolve = done; })).mockRejectedValueOnce(new Error("engine absent"));
    const queue = vi.fn(); const selection = { type: "material" as const, id: model.materials![0].id };
    const view = render(<GuardedRemoval model={model} selection={selection} onQueueIntent={queue} />);
    const replacement = structuredClone(model); replacement.materials![0].label = "Changed basis";
    view.rerender(<GuardedRemoval model={replacement} selection={selection} onQueueIntent={queue} />);
    resolve(JSON.stringify(model.materials![0]));
    await screen.findByRole("alert");
    expect(screen.getByRole("button", { name: "Queue removal for review" })).toBeDisabled();
    fireEvent.click(screen.getByRole("button", { name: "Queue removal for review" }));
    expect(queue).not.toHaveBeenCalled();
  });
});


describe("existing toolkit assignment preparation", () => {
  it("discards an old selection snapshot before queuing assignment", async () => {
    const model = await loadPreviewModel();
    model.sections = [{ id: "section:invented", name: "Invented", section_type: "pipe", properties: {}, provenance: "invented test" }];
    let first!: (value: string) => void;
    let second!: (value: string) => void;
    canonical.mockReturnValueOnce(new Promise<string>((done) => { first = done; }))
      .mockReturnValueOnce(new Promise<string>((done) => { second = done; }));
    const queue = vi.fn();
    const view = render(<SectionAssignment model={model} selection={{ type: "pipe", id: model.pipe_segments[0].id }} onQueueIntent={queue} />);
    view.rerender(<SectionAssignment model={model} selection={{ type: "pipe", id: model.pipe_segments[1].id }} onQueueIntent={queue} />);
    fireEvent.change(screen.getByLabelText("Shared section"), { target: { value: "section:invented" } });
    await act(async () => { first(JSON.stringify(model.pipe_segments[0])); });
    expect(screen.getByRole("button", { name: "Queue section assignment" })).toBeDisabled();
    await act(async () => { second(JSON.stringify(model.pipe_segments[1])); });
    fireEvent.click(screen.getByRole("button", { name: "Queue section assignment" }));
    expect(queue.mock.calls[0][0]).toMatchObject({ target: { ref: model.pipe_segments[1].id }, change: { before: JSON.stringify(model.pipe_segments[1]) } });
  });
});
