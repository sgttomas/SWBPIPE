// Delete before-values print numbers with the engine's own `display_number` (the wasm export
// `display_number_text`, #118). Without the loaded engine, no `before` is built and no number
// is formatted another way: the combination and term displays raise `NumberDisplayUnavailable`
// (the editor then builds no delete and says why), and a node's deletion text and intent are
// null. A record or term that needs no number (no terms, or a canonical form) is unaffected.
import { describe, expect, it, vi } from "vitest";

vi.mock("../../services/wasmEngine/loadWasmEngine", () => ({ loadedWasmEngine: () => null, loadWasmEngine: () => Promise.reject(new Error("not loaded")) }));

import type { PreviewModel } from "../../types";
import { NUMBER_DISPLAY_ENGINE_ABSENT, NumberDisplayUnavailable, combinationDeleteBefore, historicalCombinationDisplay, historicalTermDisplay } from "./combinationDelete";
import { applierDisplayNumber, buildDeleteNodeIntent, nodeDeletionSummary } from "../model-tree/removalIntents";

const node = { id: "node:N", label: "N", position: { x: 1.5, y: 0, z: 0 }, provenance: "invented" } as PreviewModel["nodes"][number];
const model = { project: { id: "project:x" } } as PreviewModel;

describe("without the loaded wasm engine", () => {
  it("builds no combination or term before that needs a number, and formats none another way", async () => {
    const flat = { id: "combination:M", label: "m", basis: "mechanics", terms: [{ load_case: "load:L-1", factor: 1.5 }], provenance: "t" };
    expect(() => historicalTermDisplay({ load_case: "load:L-1", factor: 1 })).toThrow(NumberDisplayUnavailable);
    expect(() => historicalCombinationDisplay(flat)).toThrow(NUMBER_DISPLAY_ENGINE_ABSENT);
    await expect(combinationDeleteBefore(flat as never, async () => "unused")).rejects.toThrow(NUMBER_DISPLAY_ENGINE_ABSENT);
    // Needing no number: a record with empty terms keeps its display; a non-flat one is canonical.
    expect(historicalCombinationDisplay({ ...flat, terms: [] })).toBe("combination:M; m; basis=mechanics; terms=none");
    expect(historicalTermDisplay({ combination: "combination:S", factor: 1 })).toBeNull();
  });

  it("builds no node deletion text or intent", () => {
    expect(applierDisplayNumber(1.5)).toBeNull();
    expect(nodeDeletionSummary(node)).toBeNull();
    expect(buildDeleteNodeIntent(node, model)).toBeNull();
    // A blank label is refused by the applier anyway: its text stays empty, as before.
    expect(nodeDeletionSummary({ ...node, label: " " })).toBe("");
  });
});
