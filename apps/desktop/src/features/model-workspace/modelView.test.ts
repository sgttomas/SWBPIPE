import { describe, expect, it } from "vitest";
import type { EntityRef, PreviewModel } from "../../types";
import { entityLabel, selectedProperties } from "./modelView";

const sharedId = "same/id";
const model = {
  project: { id: "p", name: "Project", units: { length: "m" } },
  schema_version: "0.2.0",
  analysis_status: { mechanics: "NOT_RUN", rule_check: "NOT_RUN", professional_acceptance: "NOT_REVIEWED" },
  materials: [],
  sections: [],
  nodes: [{ id: sharedId, label: "Node label", position: { x: 1, y: 2, z: 3 }, provenance: "node source" }],
  pipe_segments: [{
    id: sharedId,
    label: "Pipe label",
    from: "a",
    to: "b",
    section: { outside_diameter: { value: 0.2, unit: "m" }, wall_thickness: { value: 0.01, unit: "m" } },
    material: "m",
    provenance: "pipe source"
  }],
  supports: [{ id: sharedId, label: "Support label", node: "a", restraints: ["UX"], provenance: "support source" }],
  components: [{ id: sharedId, label: "Component label", kind: "valve", node: "a", provenance: "component source" }],
  load_cases: [],
  diagnostics: []
} as unknown as PreviewModel;

describe("typed model view identity", () => {
  it.each([
    [{ type: "node", id: sharedId } as EntityRef, "Node label", ["Position", "1, 2, 3 m"]],
    [{ type: "pipe", id: sharedId } as EntityRef, "Pipe label", ["From", "a"]],
    [{ type: "support", id: sharedId } as EntityRef, "Support label", ["Restraints", "UX"]],
    [{ type: "component", id: sharedId } as EntityRef, "Component label", ["Kind", "valve"]]
  ])("resolves %s without falling through another same-ID type", (selection, label, expectedRow) => {
    expect(entityLabel(model, selection)).toBe(label);
    expect(selectedProperties(model, selection)).toContainEqual(expectedRow);
  });
});
