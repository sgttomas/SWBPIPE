/** B2 invented bulk-change model shared by desktop tests: A -P1- B -P2- C -P3- D.
 * Guides at B and C, an anchor at A, a spring at D; an unrealized bend at B, a realized bend
 * on P3 and a valve at C; two materials and a pipe and a rigid section. */
import { MODEL_SCHEMA_VERSION } from "../services/projectService";
import type { PreviewModel } from "../types";

const PROV = "invented B2 bulk-change test input";
const node = (id: string, x: number) => ({ id, label: id.replace("node:", ""), position: { x, y: 0, z: 0 }, provenance: PROV });
const section = { outside_diameter: { value: 0.1, unit: "m" }, wall_thickness: { value: 0.005, unit: "m" } };
const pipe = (id: string, from: string, to: string, extra: Record<string, unknown> = {}) => ({
  id, label: id.replace("pipe:", ""), from, to, section: structuredClone(section), material: "material:steel", y_reference: { x: 0, y: 0, z: 1 }, provenance: PROV, ...extra
});

export function bulkChangeModel(): PreviewModel {
  return {
    schema_version: MODEL_SCHEMA_VERSION, document_kind: "openpipestress.product_preview.model",
    project: { id: "project:b2-bulk", name: "B2 bulk change", units: { length: "m", force: "N", angle: "rad", pressure: "Pa", stress: "Pa", temperature: "degC" } },
    analysis_status: { mechanics: "ready_for_preview_diagnostics", rule_check: "not_performed_user_rule_inputs_missing", professional_acceptance: "not_provided" },
    nodes: [node("node:A", 0), node("node:B", 2), node("node:C", 4), node("node:D", 6)],
    pipe_segments: [
      pipe("pipe:P1", "node:A", "node:B", { tags: ["rack-1"] }),
      pipe("pipe:P2", "node:B", "node:C"),
      pipe("pipe:P3", "node:C", "node:D")
    ],
    supports: [
      { id: "support:A", label: "Anchor A", node: "node:A", family: "anchor", restraints: ["UX", "UY", "UZ", "RX", "RY", "RZ"], provenance: PROV },
      { id: "support:B", label: "Guide B", node: "node:B", family: "guide", restraints: ["UY", "UZ"], provenance: PROV },
      { id: "support:C", label: "Guide C", node: "node:C", family: "guide", restraints: ["UY", "UZ"], provenance: PROV, tags: ["what-if"] },
      { id: "support:D", label: "Spring D", node: "node:D", family: "spring", restraints: [], stiffness: { dof: "UY", value: { value: 1000, unit: "N/m" } }, provenance: PROV }
    ],
    components: [
      { id: "component:BEND-B", label: "Bend B", kind: "bend", node: "node:B", geometry: { bend_radius: { value: 0.15, unit: "m" } }, provenance: PROV },
      { id: "component:BEND-R", label: "Realized bend", kind: "elbow", node: "node:D", geometry: { bend_pipe_ref: "pipe:P3", bend_radius: { value: 2.5, unit: "m" } }, mechanics_interface: { solver_consumption: "curved_bend_macro_element" }, provenance: PROV },
      { id: "component:VALVE-C", label: "Valve C", kind: "valve", node: "node:C", provenance: PROV }
    ],
    materials: [
      { id: "material:steel", elastic_modulus: { value: 200e9, unit: "Pa" }, shear_modulus: { value: 80e9, unit: "Pa" }, provenance: PROV },
      { id: "material:alloy", elastic_modulus: { value: 190e9, unit: "Pa" }, shear_modulus: { value: 75e9, unit: "Pa" }, provenance: PROV }
    ],
    sections: [
      { id: "section:S1", name: "S1", section_type: "pipe", properties: { outside_diameter: { value: 0.1683, unit: "m" }, wall_thickness: { value: 0.00711, unit: "m" } }, provenance: PROV },
      { id: "section:RIGID", name: "Rigid", section_type: "rigid", properties: {}, provenance: PROV }
    ],
    load_cases: [{ id: "case:w", label: "Weight", kind: "sustained", status: "draft", primitive_loads: [], provenance: PROV }],
    combinations: [],
    diagnostics: []
  } as unknown as PreviewModel;
}
