import { describe, expect, it, vi } from "vitest";
import { SelectionPresentationBinding } from "./selectionPresentationBinding";
import { entityKey } from "../workspace/selectionState";

const node = entityKey({ type: "node", id: "same" });
const pipe = entityKey({ type: "pipe", id: "same" });
const resource = () => ({ contextSnapshot: { generation: 2 }, submissionSequence: 7, setSelectionPresentation: vi.fn() });

describe("actual selection presentation binding", () => {
  it("records only a successful setter, then requires a strictly later actual callback", () => {
    const binding = new SelectionPresentationBinding(); const source = resource();
    binding.afterFrame(2, 3, 8); expect(binding.current(2, 3, 8)).toBeNull();
    source.setSelectionPresentation.mockImplementationOnce(() => { throw new Error("setter failed"); });
    expect(() => binding.apply(source, 3, [node])).toThrow("setter failed");
    binding.afterFrame(2, 3, 8); expect(binding.current(2, 3, 8)).toBeNull();
    source.setSelectionPresentation.mockImplementationOnce(() => expect(binding.current(2, 3, 8)).toBeNull());
    binding.apply(source, 3, [node]);
    expect(source.setSelectionPresentation).toHaveBeenLastCalledWith([node]);
    expect(binding.current(2, 3, 8)).toBeNull();
    binding.afterFrame(2, 3, 7); expect(binding.current(2, 3, 7)).toBeNull();
    binding.afterFrame(2, 3, 8);
    expect(binding.current(2, 3, 8)).toEqual({ resourceGeneration: 2, modelGeneration: 3, revision: 1,
      appliedAfterSubmissionSequence: 7, renderedSubmissionSequence: 8, orderedKeys: [node] });
    expect(binding.qualifyingSubmission(2, 3, 8, 7, [node])).toBe(8);
    expect(binding.qualifyingSubmission(2, 3, 8, 8, [node])).toBe(0);
  });
  it("rejects intended-new/applied-old, reordered and typed-collision membership", () => {
    const binding = new SelectionPresentationBinding(); const source = resource();
    binding.apply(source, 3, [node, pipe]); binding.afterFrame(2, 3, 8);
    expect(binding.qualifyingSubmission(2, 3, 8, 7, [node, pipe])).toBe(8);
    for (const keys of [[], [pipe], [pipe, node], [node, node]]) expect(binding.qualifyingSubmission(2, 3, 8, 7, keys)).toBe(0);
    source.submissionSequence = 8; binding.apply(source, 3, []);
    expect(binding.current(2, 3, 8)).toBeNull();
    binding.afterFrame(2, 3, 8); expect(binding.current(2, 3, 8)).toBeNull();
    binding.afterFrame(2, 3, 9); expect(binding.qualifyingSubmission(2, 3, 9, 8, [])).toBe(9);
  });
  it("reuses unchanged empty application only after the no-op action's new frame", () => {
    const binding = new SelectionPresentationBinding(); binding.apply(resource(), 3, []); binding.afterFrame(2, 3, 8);
    const revision = binding.current(2, 3, 8)!.revision;
    expect(binding.qualifyingSubmission(2, 3, 8, 8, [])).toBe(0);
    binding.afterFrame(2, 3, 9);
    expect(binding.qualifyingSubmission(2, 3, 9, 8, [])).toBe(9);
    expect(binding.current(2, 3, 9)!.revision).toBe(revision);
    expect(binding.current(2, 3, 8)).toBeNull();
  });
  it("rejects generation mismatch and clears replacement state while keeping resource revisions monotonic", () => {
    const binding = new SelectionPresentationBinding(); const source = resource(); binding.apply(source, 3, [node]);
    binding.afterFrame(4, 3, 8); expect(binding.current(4, 3, 8)).toBeNull();
    binding.afterFrame(2, 4, 8); expect(binding.current(2, 4, 8)).toBeNull();
    binding.afterFrame(2, 3, 8); expect(binding.current(2, 4, 8)).toBeNull();
    binding.clear(); expect(binding.current(2, 3, 8)).toBeNull();
    binding.apply(source, 4, [pipe]); binding.afterFrame(2, 4, 8); expect(binding.current(2, 4, 8)!.revision).toBe(2);
    source.contextSnapshot.generation = 5; binding.apply(source, 4, [pipe]); binding.afterFrame(5, 4, 8);
    expect(binding.current(5, 4, 8)!.revision).toBe(1);
  });
  it("invalidates old evidence after a throwing setter and snapshots the actual supplied keys", () => {
    const binding = new SelectionPresentationBinding(); const source = resource(); const keys = [node];
    binding.apply(source, 3, keys); keys.push(pipe); binding.afterFrame(2, 3, 8);
    expect(binding.current(2, 3, 8)!.orderedKeys).toEqual([node]);
    source.setSelectionPresentation.mockImplementation(() => { throw new Error("partial application"); });
    expect(() => binding.apply(source, 3, [])).toThrow(); expect(binding.current(2, 3, 8)).toBeNull();
  });
  it("rejects invalid application and action barriers without altering the setter call count", () => {
    const binding = new SelectionPresentationBinding(); const source = resource();
    binding.apply(source, null, [node]); binding.afterFrame(2, 3, 8); expect(binding.current(2, 3, 8)).toBeNull();
    source.submissionSequence = -1; binding.apply(source, 3, [node]); binding.afterFrame(2, 3, 8); expect(binding.current(2, 3, 8)).toBeNull();
    source.submissionSequence = 7; binding.apply(source, 3, [node]); binding.afterFrame(2, 3, 8);
    for (const barrier of [-1, NaN, 1.5, Number.MAX_SAFE_INTEGER + 1]) expect(binding.qualifyingSubmission(2, 3, 8, barrier, [node])).toBe(0);
    expect(source.setSelectionPresentation).toHaveBeenCalledTimes(3);
  });
  it.each([0, -1, Number.NaN, Number.POSITIVE_INFINITY, 1.5, Number.MAX_SAFE_INTEGER + 1])("rejects unsafe frame/generation %s", (bad) => {
    const binding = new SelectionPresentationBinding(); binding.apply(resource(), 3, [node]);
    binding.afterFrame(2, 3, bad); expect(binding.current(2, 3, bad)).toBeNull();
    binding.afterFrame(bad, 3, 8); expect(binding.current(bad, 3, 8)).toBeNull();
    binding.afterFrame(2, bad, 8); expect(binding.current(2, bad, 8)).toBeNull();
  });
});
