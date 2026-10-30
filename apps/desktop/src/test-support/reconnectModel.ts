/** W-10 invented reconnect model shared by desktop tests: A -P- B -Q- D -R- C.
 * P (A -> B) has no y_reference; moving P's `to` end to C is a clean reconnect. */
import { MODEL_SCHEMA_VERSION } from "../services/projectService";
import type { PreviewModel } from "../types";

const PROV = "invented W-10 session test input";
const node = (id: string, x: number, y: number) => ({ id, label: id.replace("node:", ""), position: { x, y, z: 0 }, provenance: PROV });
const section = { outside_diameter: { value: 0.1, unit: "m" }, wall_thickness: { value: 0.005, unit: "m" } };
/** P (A -> B) has no y_reference; reconnecting its `to` end to C with an entered one adds it. */
export function reconnectModel(): PreviewModel {
  return {
    schema_version: MODEL_SCHEMA_VERSION, document_kind: "openpipestress.product_preview.model",
    project: { id: "project:w10-session", name: "W-10 session", units: { length: "m", force: "N", angle: "rad", pressure: "Pa", stress: "Pa", temperature: "degC" } },
    analysis_status: { mechanics: "ready_for_preview_diagnostics", rule_check: "not_performed_user_rule_inputs_missing", professional_acceptance: "not_provided" },
    nodes: [node("node:A", 0, 0), node("node:B", 2, 0), node("node:C", 2, 2), node("node:D", 4, 0)],
    pipe_segments: [
      { id: "pipe:P", label: "P", from: "node:A", to: "node:B", section, material: "material:steel", provenance: PROV },
      { id: "pipe:Q", label: "Q", from: "node:B", to: "node:D", section, material: "material:steel", y_reference: { x: 0, y: 0, z: 1 }, provenance: PROV },
      { id: "pipe:R", label: "R", from: "node:D", to: "node:C", section, material: "material:steel", y_reference: { x: 0, y: 0, z: 1 }, provenance: PROV }],
    supports: [
      { id: "support:A", node: "node:A", family: "anchor", restraints: ["UX", "UY", "UZ", "RX", "RY", "RZ"], provenance: PROV },
      { id: "support:D", node: "node:D", family: "anchor", restraints: ["UX", "UY", "UZ", "RX", "RY", "RZ"], provenance: PROV }],
    components: [],
    materials: [{ id: "material:steel", elastic_modulus: { value: 200e9, unit: "Pa" }, shear_modulus: { value: 80e9, unit: "Pa" }, provenance: PROV }],
    load_cases: [{ id: "case:w", label: "Weight", kind: "sustained", status: "draft", primitive_loads: [], provenance: PROV }],
    combinations: [],
    diagnostics: []
  } as unknown as PreviewModel;
}
