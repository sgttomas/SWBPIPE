import { describe, expect, it } from "vitest";
import type { PreviewModel } from "../../types";
import { buildModelIndex, convertedSectionEnvelopeIsConsistent, modelIndexFor } from "./modelIndex";
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

  it("defers mixed-unit shared/inline physical consistency until converted values are available", () => {
    const model = fixture();
    model.sections![0].properties = {
      outside_diameter: { value: 114.3, unit: "mm" },
      wall_thickness: { value: 5.08, unit: "mm" }
    };
    model.pipe_segments[0].section = {
      outside_diameter: { value: 4.5, unit: "in" },
      wall_thickness: { value: 0.2, unit: "in" }
    };
    const index = buildModelIndex(model, 1, 3);
    const pipeKey = entityKey({ type: "pipe", id: "same" });

    expect(index.sectionBindings.get(pipeKey)?.issue).toBeNull();
    expect(convertedSectionEnvelopeIsConsistent(0.1143, 0.00508, 0.1143, 0.00508)).toBe(true);
    expect(convertedSectionEnvelopeIsConsistent(0.1143, 0.00508, 0.11684, 0.00508)).toBe(false);
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

  it("excludes every same-type duplicate pipe, support, and component from geometry while retaining typed inspection", () => {
    const model = fixture();
    model.supports = [
      { id: "same-support", label: "Support A", node: "same", restraints: [], provenance: "test" },
      { id: "same-support", label: "Support B", node: "end", restraints: [], provenance: "test" }
    ];
    model.components = [
      { id: "same-component", label: "Component A", node: "same", kind: "generic", provenance: "test" },
      { id: "same-component", label: "Component B", node: "end", kind: "generic", provenance: "test" }
    ];
    model.pipe_segments.push({ ...model.pipe_segments[0], label: "Duplicate pipe" });
    const index = buildModelIndex(model, 1, 4);
    for (const [type, id] of [
      ["pipe", "same"], ["support", "same-support"], ["component", "same-component"]
    ] as const) {
      const key = entityKey({ type, id });
      expect(index.entities.has(key)).toBe(true);
      expect(index.invalidGeometry.get(key)).toContain("duplicate same-type");
      expect(index.visibilityEligibleKeys.has(key)).toBe(false);
    }
  });
});
