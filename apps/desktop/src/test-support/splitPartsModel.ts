/** B5 invented split-and-slope model shared by desktop tests (DEL-07-01 Design
 * `split-and-slope-entry.md`). Project length unit: ft, so the frame check converts.
 *   A -S- B          a sloped run in 3D (pipe:S, tagged, with a y_reference, on line:L1)
 *   B -V- C          a valve at C, so a split of V is refused at its endpoint
 *   C -W- D          a wind-exposed span (load case names pipe:W), so a split of W is refused
 *   D -H- E          a horizontal run along +X with a support at E (node supports do not block)
 * All values are invented. */
import { MODEL_SCHEMA_VERSION } from "../services/projectService";
import type { PreviewModel } from "../types";

const PROV = "invented B5 split test input";
const node = (id: string, x: number, y: number, z: number) => ({ id, label: id.replace("node:", ""), position: { x, y, z }, provenance: PROV });
const section = () => ({ outside_diameter: { value: 4.5, unit: "in" }, wall_thickness: { value: 0.237, unit: "in" } });
const pipe = (id: string, from: string, to: string, extra: Record<string, unknown> = {}) => ({
  id, label: id.replace("pipe:", ""), from, to, section: section(), material: "material:steel", y_reference: { x: 0, y: 0, z: 1 }, provenance: PROV, ...extra
});

export function splitPartsModel(): PreviewModel {
  return {
    schema_version: MODEL_SCHEMA_VERSION, document_kind: "openpipestress.product_preview.model",
    data_boundary: {
      public_examples_policy: "invented_or_cleared_data_only", protected_source_policy: "no_bundled_protected_owner_or_standards_data",
      private_data_policy: "no_private_project_data", professional_boundary: "technical_preview_requires_human_engineering_review"
    },
    project: { id: "project:b5-split", name: "B5 split and slope", units: { length: "ft", force: "lbf", angle: "deg", pressure: "psi", stress: "psi", temperature: "degF" } },
    analysis_status: { mechanics: "ready_for_preview_diagnostics", rule_check: "not_performed_user_rule_inputs_missing", professional_acceptance: "not_provided" },
    nodes: [
      node("node:A", 0.1, 1.25, 12.345),
      node("node:B", 7.3, -3.7, 0.017),
      node("node:C", 7.3, -3.7, 9),
      node("node:D", 20, -3.7, 9),
      node("node:E", 32.5, -3.7, 9)
    ],
    pipe_segments: [
      pipe("pipe:S", "node:A", "node:B", { tags: ["drain"], y_reference: { x: 0, y: 1, z: 0 } }),
      pipe("pipe:V", "node:B", "node:C", { y_reference: { x: 1, y: 0, z: 0 } }),
      pipe("pipe:W", "node:C", "node:D"),
      pipe("pipe:H", "node:D", "node:E")
    ],
    supports: [
      { id: "support:E", label: "Guide E", node: "node:E", family: "guide", restraints: ["UY", "UZ"], provenance: PROV }
    ],
    components: [
      { id: "component:VALVE-C", label: "Valve C", kind: "valve", node: "node:C", provenance: PROV }
    ],
    materials: [{ id: "material:steel", elastic_modulus: { value: 29e6, unit: "psi" }, shear_modulus: { value: 11e6, unit: "psi" }, provenance: PROV }],
    load_cases: [{
      id: "case:wind", label: "Wind", kind: "occasional", status: "draft", primitive_loads: [], provenance: PROV,
      equivalent_static: { wind: { exposed_pipe_refs: ["pipe:W"] } }
    }],
    combinations: [],
    lines: [{ id: "line:L1", label: "L1", pipe_refs: ["pipe:S"], provenance: PROV }],
    diagnostics: []
  } as unknown as PreviewModel;
}
