// Invented fixtures for the result overlay tests: a three-member model with three supports and the
// rows its solve would emit, written by hand. Not a product fixture; no engineering meaning.

import type { MechanicsResult, PreviewModel } from "../types";
import { numericalHealthView, parseNumericalHealth, type NumericalHealthState, type ResultOverlayStanding } from "../features/results/numericalHealth";

type Row = MechanicsResult["results"][number];

export function overlayModel(): PreviewModel {
  return {
    nodes: [
      { id: "node:A", position: { x: 0, y: 0, z: 0 } },
      { id: "node:B", position: { x: 1, y: 0, z: 0 } },
      { id: "node:C", position: { x: 2, y: 0, z: 0 } },
      { id: "node:D", position: { x: 2, y: 1, z: 0 } }
    ],
    pipe_segments: [
      { id: "pipe:P-1", from: "node:A", to: "node:B" },
      { id: "pipe:P-2", from: "node:B", to: "node:C" },
      { id: "pipe:P-3", from: "node:C", to: "node:D" }
    ],
    supports: [
      { id: "support:S-A", node: "node:A" },
      { id: "support:S-C", node: "node:C" },
      { id: "support:S-D", node: "node:D" }
    ],
    components: [],
    load_cases: [
      { id: "load:L-1", label: "Invented one", kind: "invented", status: "draft", provenance: "invented" },
      { id: "load:L-2", label: "Invented two", kind: "invented", status: "draft", provenance: "invented" }
    ]
  } as unknown as PreviewModel;
}

const L1 = { ref_type: "load_case", ref_id: "load:L-1" } as const;
const L2 = { ref_type: "load_case", ref_id: "load:L-2" } as const;

export function row(id: string, kind: string, entity: string, value: number, unit: string, extra: Partial<Row> = {}): Row {
  return { id, kind, entity_ref: entity, value, unit, basis_ref: L1, ...extra };
}

const GLOBAL = { coordinate_system: "global", location: "node", basis: "recovered_from_assembled_support_law", sign_convention: "support-on-pipe; positive global force and right-hand couple about attached node" };

export function reaction(support: string, component: string, value: number, unit: string, extra: Partial<Row> = {}): Row {
  return row(`r:${support}:${component}`, "support_reaction_component_v2", support, value, unit, { metadata: { component, ...GLOBAL }, ...extra });
}

export function overlayResult(rows: Row[], evidence?: Record<string, unknown>): MechanicsResult {
  return {
    schema_version: "0.2.0",
    document_kind: "mechanics_result",
    run_id: "run:invented",
    model_ref: "model:invented",
    status: { mechanics: "MECHANICS_SOLVED", rule_check: "not_run", professional_acceptance: "not_claimed" },
    summary: {},
    results: rows,
    diagnostics: [],
    ...(evidence ? { contract_evidence: evidence } : {})
  };
}

/** The standard rows: stresses at stations, nodal displacements, an intensified row on a
 * component, a second case and a range envelope that must not colour. */
export function standardRows(): Row[] {
  return [
    row("s:P-1:i", "element_local_normal_stress_extreme", "pipe:P-1", -4, "MPa"),
    row("s:P-1:j", "element_local_normal_stress_extreme", "pipe:P-1", 3, "MPa"),
    row("s:P-2:i", "element_local_normal_stress_extreme", "pipe:P-2", 8, "MPa"),
    row("s:P-2:env", "element_local_normal_stress_extreme", "pipe:P-2", 99, "MPa", { metadata: { component: "sigma_plus", coordinate_system: "pipe_section", location: "end_i", basis: "explicit_user_range_envelope", sign_convention: "invented" } }),
    row("s:P-2:L2", "element_local_normal_stress_extreme", "pipe:P-2", 50, "MPa", { basis_ref: L2 }),
    row("s:P-3:nan", "element_local_normal_stress_extreme", "pipe:P-3", Number.NaN, "MPa"),
    row("d:A", "displacement_magnitude", "node:A", 0, "mm"),
    row("d:B", "displacement_magnitude", "node:B", 2, "mm"),
    row("d:C", "displacement_magnitude", "node:C", 1, "mm"),
    row("b:P-3:y", "element_local_bending_normal_stress_y", "pipe:P-3", 5, "MPa"),
    row("ib:C-1", "component_equal_factor_intensified_bending_stress_v1", "component:C-1", 7e6, "Pa", { source_result_refs: ["b:P-3:y"] })
  ];
}

export function healthStanding(options: {
  eligible: boolean;
  route?: "generic" | "pressure_source";
  states?: Record<string, NumericalHealthState>;
}): ResultOverlayStanding {
  const states = options.states ?? { "load:L-1": "checks_passed", "load:L-2": "checks_passed" };
  const parsed = parseNumericalHealth({
    route: options.route ?? "generic",
    verdict_only: false,
    cases: Object.entries(states).map(([caseId, state]) => ({
      case_id: caseId, state, code: null, integrity_diagnostic_id: null, rcond: null, digits: null, conditioning_sensitive: false, causes: []
    })),
    combinations: []
  });
  if (!parsed) throw new Error("invented health did not parse");
  return { eligible: options.eligible, health: numericalHealthView(parsed, { eligible: options.eligible, verdictHold: !options.eligible }) };
}
