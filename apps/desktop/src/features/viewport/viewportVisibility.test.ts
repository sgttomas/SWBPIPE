import { describe, expect, it } from "vitest";
import type { EntityRef, PreviewModel } from "../../types";
import { buildModelIndex } from "../workspace/modelIndex";
import { entityKey, type EntityKey } from "../workspace/selectionState";
import { deriveViewportVisibility } from "./viewportVisibility";

const key = (type: EntityRef["type"], id: string) => entityKey({ type, id });
const n = key("node", "n");
const end = key("node", "end");
const pipe = key("pipe", "p");
const support = key("support", "s");
const component = key("component", "c");

function fixture(): PreviewModel {
  return {
    schema_version: "test", document_kind: "test", data_boundary: {},
    project: { id: "project", name: "Visibility", description: "", units: { length: "m" } },
    analysis_status: { mechanics: "TBD", rule_check: "TBD", professional_acceptance: "TBD" },
    nodes: [
      { id: "n", label: "N", position: { x: 0, y: 0, z: 0 }, provenance: "test" },
      { id: "end", label: "End", position: { x: 4, y: 0, z: 0 }, provenance: "test" }
    ],
    pipe_segments: [{ id: "p", label: "Pipe", from: "n", to: "end", section: {}, material: "m", provenance: "test" }],
    supports: [{ id: "s", label: "Support", node: "n", restraints: [], provenance: "test" }],
    components: [{ id: "c", label: "Component", node: "n", kind: "generic", provenance: "test" }],
    load_cases: [], diagnostics: []
  };
}

const indexFor = (model = fixture()) => buildModelIndex(model, 1, 1);

describe("viewport visibility projection", () => {
  it("distinguishes inactive isolation from an active empty snapshot", () => {
    const index = indexFor();
    expect(deriveViewportVisibility(index, new Set(), null)).toEqual({
      hiddenKeys: new Set(), dimmedKeys: new Set(), hiddenCount: 0, isolationActive: false
    });
    expect(deriveViewportVisibility(index, new Set(), new Set())).toEqual({
      hiddenKeys: new Set(), dimmedKeys: new Set([n, end, pipe, support, component]), hiddenCount: 0, isolationActive: true
    });
  });

  it("keeps snapshot membership literal for distinct attached entities", () => {
    const index = indexFor();
    expect(deriveViewportVisibility(index, new Set(), new Set([n])).dimmedKeys)
      .toEqual(new Set([end, pipe, support, component]));
    expect(deriveViewportVisibility(index, new Set(), new Set([pipe, support])).dimmedKeys)
      .toEqual(new Set([n, end, component]));
  });

  it("dims new geometry and keeps isolation active after the captured entity is deleted", () => {
    const model = fixture();
    const snapshot = new Set([pipe]);
    model.nodes.push({ id: "new", label: "New", position: { x: 10, y: 0, z: 0 }, provenance: "test" });
    expect(deriveViewportVisibility(indexFor(model), new Set(), snapshot).dimmedKeys.has(key("node", "new"))).toBe(true);
    model.pipe_segments = [];
    const stale = deriveViewportVisibility(indexFor(model), new Set(), snapshot);
    const pruned = deriveViewportVisibility(indexFor(model), new Set(), new Set());
    expect(stale).toEqual(pruned);
    expect(pruned.isolationActive).toBe(true);
    expect(pruned.dimmedKeys).toEqual(indexFor(model).visibilityEligibleKeys);
  });

  it("expands node Hide to attachments once and gives Hide precedence over isolation", () => {
    const index = indexFor();
    const hidden = new Set([n, support]);
    for (const snapshot of [null, new Set([n, support, component]), new Set([end]), new Set<EntityKey>()]) {
      const result = deriveViewportVisibility(index, hidden, snapshot);
      expect(result.hiddenKeys).toEqual(new Set([n, support, component]));
      expect(result.hiddenCount).toBe(3);
      expect([...result.dimmedKeys].some((item) => result.hiddenKeys.has(item))).toBe(false);
    }
    expect(deriveViewportVisibility(index, hidden, new Set()).dimmedKeys).toEqual(new Set([end, pipe]));
  });

  it("excludes invalid, nongeometry and stale keys from count and dimming", () => {
    const model = fixture();
    model.nodes.push({ id: "bad", label: "Bad", position: { x: NaN, y: 0, z: 0 }, provenance: "test" });
    const excluded = [key("node", "bad"), key("project", "project"), key("pipe", "deleted")];
    const result = deriveViewportVisibility(indexFor(model), new Set(excluded), new Set(excluded));
    expect(result.hiddenCount).toBe(0);
    expect(result.hiddenKeys).toEqual(new Set(excluded));
    expect(result.dimmedKeys).toEqual(new Set([n, end, pipe, support, component]));
  });

  it("does not mutate or alias the input sets or index eligibility", () => {
    const index = indexFor();
    const hidden = new Set([n]);
    const snapshot = new Set([pipe]);
    const eligibleBefore = [...index.visibilityEligibleKeys];
    const result = deriveViewportVisibility(index, hidden, snapshot);
    expect(hidden).toEqual(new Set([n]));
    expect(snapshot).toEqual(new Set([pipe]));
    expect([...index.visibilityEligibleKeys]).toEqual(eligibleBefore);
    hidden.clear();
    snapshot.clear();
    expect(result.hiddenKeys).toEqual(new Set([n, support, component]));
    expect(result.dimmedKeys).toEqual(new Set([end]));
  });
});
