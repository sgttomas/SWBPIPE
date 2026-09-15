import { describe, expect, it } from "vitest";
import type { PreviewModel } from "../../types";
import { buildModelIndex, modelIndexFor } from "./modelIndex";
import { entityKey } from "./selectionState";

function fixture(): PreviewModel {
  return {
    schema_version: "test",
    document_kind: "test",
    data_boundary: {},
    project: { id: "same", name: "Index", description: "", units: { length: "m" } },
    analysis_status: { mechanics: "TBD", rule_check: "TBD", professional_acceptance: "TBD" },
    materials: [],
    sections: [{ id: "sec", name: "Section", section_type: "pipe", properties: {}, provenance: "test" }],
    nodes: [
      { id: "same", label: "Node", position: { x: 1_000_000_000, y: 2, z: 3 }, provenance: "test" },
      { id: "end", label: "End", position: { x: 1_000_000_010, y: 2, z: 3 }, provenance: "test" }
    ],
    pipe_segments: [{
      id: "same",
      label: "Pipe",
      from: "same",
      to: "end",
      section: {},
      section_ref: "sec",
      material: "m",
      provenance: "test"
    }],
    supports: [],
    components: [],
    load_cases: [],
    diagnostics: []
  };
}

describe("shared model index", () => {
  it("memoizes one immutable index per model/session/revision generation", () => {
    const model = fixture();
    expect(modelIndexFor(model, 2, 7)).toBe(modelIndexFor(model, 2, 7));
    expect(modelIndexFor(model, 2, 8)).not.toBe(modelIndexFor(model, 2, 7));
  });

  it("retains only the latest generation for a revisited model identity", () => {
    const model = fixture();
    const first = modelIndexFor(model, 4, 1);
    const second = modelIndexFor(model, 4, 2);
    const revisited = modelIndexFor(model, 4, 1);
    expect(second.generation).toBe("4:2");
    expect(revisited.generation).toBe("4:1");
    expect(revisited).not.toBe(first);
    expect(modelIndexFor(model, 4, 1)).toBe(revisited);
  });

  it("keeps duplicate raw ids distinct by typed tuple key", () => {
    const index = buildModelIndex(fixture(), 1, 1);
    expect(index.entities.get(entityKey({ type: "node", id: "same" }))?.label).toBe("Node");
    expect(index.entities.get(entityKey({ type: "pipe", id: "same" }))?.label).toBe("Pipe");
    expect(index.entities.get(entityKey({ type: "project", id: "same" }))?.label).toBe("Index");
  });

  it("indexes adjacency, shared section binding, bounds, and invalid geometry", () => {
    const model = fixture();
    model.nodes.push({ id: "bad", label: "Bad", position: { x: Number.NaN, y: 0, z: 0 }, provenance: "test" });
    const index = buildModelIndex(model, 1, 1);
    const nodeKey = entityKey({ type: "node", id: "same" });
    const pipeKey = entityKey({ type: "pipe", id: "same" });
    expect(index.pipeKeysByNode.get(nodeKey)).toContain(pipeKey);
    expect(index.sectionBindings.get(pipeKey)?.source).toBe("shared");
    expect(index.geometryBounds?.min.x).toBe(1_000_000_000);
    expect(index.invalidGeometry.get(entityKey({ type: "node", id: "bad" }))).toContain("non-finite");
  });

  it("retains duplicate same-type node records but excludes ambiguous geometry and dependants", () => {
    const model = fixture();
    model.nodes.push({
      id: "same",
      label: "Duplicate node",
      position: { x: 5, y: 6, z: 7 },
      provenance: "test"
    });
    model.supports.push({ id: "support:duplicate", label: "Ambiguous", node: "same", restraints: [], provenance: "test" });
    const index = buildModelIndex(model, 1, 2);
    expect(index.entities.get(entityKey({ type: "node", id: "same" }))?.record).toBe(model.nodes[0]);
    expect(index.invalidGeometry.get(entityKey({ type: "node", id: "same" }))).toContain("duplicate same-type");
    expect(index.invalidGeometry.get(entityKey({ type: "pipe", id: "same" }))).toContain("ambiguous duplicate");
    expect(index.invalidGeometry.get(entityKey({ type: "support", id: "support:duplicate" }))).toContain("ambiguous duplicate");
    expect(index.visibilityEligibleKeys.has(entityKey({ type: "node", id: "same" }))).toBe(false);
  });
});
