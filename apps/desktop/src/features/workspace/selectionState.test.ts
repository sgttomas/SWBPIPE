import { describe, expect, it } from "vitest";
import {
  applyDisplayedRange,
  applySelection,
  emptySelection,
  entityKey,
  entityRefFromKey,
  projectSelection,
  setSelectionFocus
} from "./selectionState";

describe("ordered typed selection", () => {
  it("keeps typed tuple identities collision safe", () => {
    const node = entityKey({ type: "node", id: "a:b" });
    const pipe = entityKey({ type: "pipe", id: "a:b" });
    expect(node).not.toBe(pipe);
    expect(entityRefFromKey(node)).toEqual({ type: "node", id: "a:b" });
    expect(entityRefFromKey(entityKey({ type: "node", id: "[\"pipe\",\"x\"]" }))).toEqual({
      type: "node",
      id: "[\"pipe\",\"x\"]"
    });
  });

  it("orders additions, toggles, and primary promotion while advancing ABA epochs", () => {
    const project = { type: "project" as const, id: "same" };
    const a = { type: "node" as const, id: "same" };
    const b = { type: "pipe" as const, id: "same" };
    const initial = projectSelection(project);
    const selectedA = applySelection(initial, a);
    const selectedAB = applySelection(selectedA, b, { additive: true });
    const backToA = applySelection(selectedAB, b, { toggle: true });
    expect(backToA.orderedKeys).toEqual([entityKey(a)]);
    expect(backToA.primaryKey).toBe(entityKey(a));
    expect(backToA.preparationEpoch).toBeGreaterThan(selectedA.preparationEpoch);
    expect(applySelection(backToA, a)).toBe(backToA);
  });

  it("keeps project exclusive and focus independent from selection", () => {
    const project = { type: "project" as const, id: "p" };
    const node = { type: "node" as const, id: "n" };
    const selected = applySelection(projectSelection(project), node, { additive: true });
    const focused = setSelectionFocus(selected, entityKey({ type: "pipe", id: "f" }));
    expect(focused.preparationEpoch).toBe(selected.preparationEpoch);
    expect(projectSelection(project, focused).orderedKeys).toEqual([entityKey(project)]);
  });

  it("selects tree ranges in displayed order without replacing earlier members", () => {
    const refs = ["a", "b", "c", "d"].map((id) => ({ type: "node" as const, id }));
    const order = refs.map(entityKey);
    const anchored = applySelection(emptySelection(), refs[1]);
    const ranged = applyDisplayedRange(anchored, refs[3], order);
    expect(ranged.orderedKeys).toEqual(order.slice(1));
    expect(ranged.primaryKey).toBe(entityKey(refs[3]));
  });
});
