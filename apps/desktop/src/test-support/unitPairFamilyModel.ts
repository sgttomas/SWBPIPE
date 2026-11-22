/** Issue 382: an invented model that holds every member of the stored value/unit quantity
 * family once, for the desktop surface regressions (unitPairFamily.*.test.tsx). All values are
 * invented; no library, supplier or project data. A three-node cantilever: pipe A carries an
 * inline section, pipe B (whose own section is inline too) is beside an unreferenced shared
 * section; the material has E, G, an expansion coefficient and one temperature point with its
 * own G; the load case has a distributed force on pipe A and a concentrated force at the tip. */
import { MODEL_SCHEMA_VERSION } from "../services/projectService";
import type { PreviewModel } from "../types";

const PROV = "invented_issue_382_unit_pair_surface_regression_not_library_data";
export const FAMILY = {
  material: "material:uf-steel-like",
  point: "point:uf-hot",
  inlinePipe: "pipe:uf-a",
  section: "section:uf-shared",
  loadCase: "case:uf-primitives",
  tip: "node:uf-3"
} as const;

export function unitPairFamilyModel(): PreviewModel {
  const node = (id: string, x: number) => ({ id, label: id.replace("node:", ""), position: { x, y: 0, z: 0 }, provenance: PROV });
  const inline = () => ({ outside_diameter: { value: 0.1143, unit: "m" }, wall_thickness: { value: 0.006, unit: "m" } });
  return {
    schema_version: MODEL_SCHEMA_VERSION,
    document_kind: "openpipestress.product_preview.model",
    data_boundary: { source: "invented_example_not_library_data" },
    project: {
      id: "project:uf-unit-pair-family",
      name: "Invented unit-pair family cantilever",
      description: "Invented pressure-free model for the issue 382 desktop surface regressions.",
      units: { length: "m", force: "N", angle: "rad", pressure: "Pa", stress: "Pa", temperature: "degC" }
    },
    analysis_status: { mechanics: "ready_for_preview_diagnostics", rule_check: "not_performed_user_rule_inputs_missing", professional_acceptance: "not_provided" },
    materials: [{
      id: FAMILY.material,
      label: "Invented steel-like material",
      elastic_modulus: { value: 200000000000, unit: "Pa" },
      shear_modulus: { value: 77000000000, unit: "Pa" },
      thermal_expansion_coefficient: { value: 0.000012, unit: "1/K" },
      temperature_points: [{
        id: FAMILY.point,
        temperature: { value: 200, unit: "degC" },
        elastic_modulus: { value: 190, unit: "GPa" },
        shear_modulus: { value: 73, unit: "GPa" },
        provenance: PROV
      }],
      provenance: PROV
    }],
    sections: [{
      id: FAMILY.section,
      name: "Invented shared section",
      section_type: "pipe",
      properties: { outside_diameter: { value: 0.1683, unit: "m" }, wall_thickness: { value: 0.00711, unit: "m" } },
      provenance: PROV
    }],
    nodes: [node("node:uf-1", 0), node("node:uf-2", 3), node(FAMILY.tip, 6)],
    pipe_segments: [
      { id: FAMILY.inlinePipe, label: "Span A", from: "node:uf-1", to: "node:uf-2", section: inline(), material: FAMILY.material, y_reference: { x: 0, y: 1, z: 0 }, provenance: PROV },
      { id: "pipe:uf-b", label: "Span B", from: "node:uf-2", to: FAMILY.tip, section: inline(), material: FAMILY.material, y_reference: { x: 0, y: 1, z: 0 }, provenance: PROV }
    ],
    supports: [{ id: "support:uf-anchor", label: "Anchor", node: "node:uf-1", family: "anchor", restraints: ["UX", "UY", "UZ", "RX", "RY", "RZ"], provenance: PROV }],
    components: [],
    load_cases: [{
      id: FAMILY.loadCase,
      label: "Invented primitive loads",
      kind: "primitive_user_load",
      status: "preview_only",
      primitive_loads: [
        { id: "load:uf-line", category: "distributed_force", target: { type: "element", pipe: FAMILY.inlinePipe }, direction: "global_z", magnitude: { value: -190, unit: "N/m" }, dimension: "force_per_length", provenance: PROV },
        { id: "load:uf-tip", category: "concentrated_force", target: { type: "node", node: FAMILY.tip }, direction: "global_y", magnitude: { value: 1000, unit: "N" }, dimension: "force", provenance: PROV }
      ],
      provenance: PROV
    }],
    combinations: [],
    diagnostics: []
  } as unknown as PreviewModel;
}
