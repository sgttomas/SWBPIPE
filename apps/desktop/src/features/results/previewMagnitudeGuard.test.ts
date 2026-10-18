/** E17: the actual wasm reader consumes fresh invented support-only statements.
 * These in-memory representation controls have no maximum witness, numerical
 * solve, native capture or producer-origin claim. Current shape, coverage,
 * attribution and header laws apply unchanged. */
import { describe, expect, it } from "vitest";
import table from "../../../../../fixtures/results/semantic_contract_v0_3_preview_physics_2.json";
import controls from "../../../../../fixtures/results/preview_magnitude_controls.json";
import type { MechanicsResult } from "../../types";
import { readerDispatch, readerValidate } from "./resultReader";

type Json = any; // eslint-disable-line @typescript-eslint/no-explicit-any
const FORCE = "support_reaction_force_magnitude_v2", MOMENT = "support_reaction_moment_magnitude_v2";
function binary64(bits: string) {
  const buffer = new ArrayBuffer(8), view = new DataView(buffer);
  view.setBigUint64(0, BigInt(`0x${bits}`)); return view.getFloat64(0);
}
function inventedSupportStatement(): MechanicsResult {
  const cid = "invented:e17:case", support = "invented:e17:support";
  const basis = { ref_type: "load_case", ref_id: cid };
  const identity = [cid, support].map(part => `${new TextEncoder().encode(part).length}:${part}`).join(":");
  const components = ["Fx", "Fy", "Fz", "Mx", "My", "Mz", "force_magnitude", "moment_magnitude"];
  return {
    schema_version: "0.2.0", document_kind: "openpipestress.product_preview.mechanics_result", run_id: "invented:e17:run", model_ref: "invented:e17:model",
    producer: { component_name: "open_pipe_stress_product_physics", component_version: "0.2.0", semantic_contract_id: table.semantic_contract_id },
    formulation_basis: { profile_id: table.formulation_profile_id, limitations: structuredClone(table.supported_profile_limitations) },
    numerical_quality: { value_representation: "finite_binary64", publication_quantization: "none", integrity_policy: "M03-INTEGRITY-v1", status: "checks_passed",
      cases: [{ basis_ref: basis, structural_status: "passive_model_basis", solve_quality: "checks_passed", model_matrix_fidelity: "represented_equations_retained", accuracy_evidence: "not_claimed", evidence_refs: [] }] },
    status: { mechanics: "MECHANICS_SOLVED", rule_check: "RULE_INPUTS_INCOMPLETE", professional_acceptance: "NOT_PROVIDED" },
    results: components.map((component, index) => ({ id: `result:support-action:${identity}:${component}`, entity_ref: support, basis_ref: basis, value: 0,
      kind: index < 6 ? "support_reaction_component_v2" : index === 6 ? FORCE : MOMENT, unit: index < 3 || index === 6 ? "N" : "N*m",
      metadata: { component, coordinate_system: "global", location: "node", basis: "recovered_from_assembled_support_law",
        sign_convention: "support-on-pipe; positive global force and right-hand couple about attached node; force and moment norms remain separate" } })),
    summary: { node_count: 1, segment_count: 0, support_count: 1, load_case_count: 1, component_stress_modifier_count: 0, max_displacement: null, max_open_formula_stress: null },
    diagnostics: [], contract_evidence: { preview_cases: [{ load_case_id: cid, pipe_stress_extrema: [],
      stress_maximum_coverage: { complete: true, unavailable_pipe_ids: [], outside_domain_pipe_ids: [] },
      support_attribution: { attributed_support_ids: [support], withheld: [] }, intensified_measures: [] }], combination_gates: [] },
    professional_boundary: { human_review_required: true }, accepted_model_state_mutated: false,
  } as unknown as MechanicsResult;
}
function sourceControl(control: typeof controls.controls[number], target: string, combined: boolean) {
  const source = inventedSupportStatement() as Json;
  const values = control.component_bits.map(binary64), magnitude = binary64(control.magnitude_bits);
  const components = target === FORCE ? ["Fx", "Fy", "Fz"] : target === MOMENT ? ["Mx", "My", "Mz"] : ["nodal_displacement_x", "nodal_displacement_y", "nodal_displacement_z"];
  if (!combined) {
    const evidence = source.contract_evidence.preview_cases[0], support = evidence.support_attribution.attributed_support_ids[0];
    const rows = source.results.filter((row: Json) => row.entity_ref === support && row.basis_ref?.ref_id === evidence.load_case_id);
    components.forEach((component, axis) => { rows.find((row: Json) => row.metadata?.component === component).value = values[axis]; });
    rows.find((row: Json) => row.kind === target).value = magnitude;
    return source as MechanicsResult;
  }
  const combination = "invented:e17:combination";
  source.contract_evidence.combination_gates.push({ combination_id: combination, withheld: false, reason: null });
  const entity = "invented:e17:combined", basis = { ref_type: "combination", ref_id: combination };
  const metadata = (component: string) => ({ component, coordinate_system: "global", location: "node", basis: "explicit_user_linear_combination",
    sign_convention: "positive value follows explicit user linear combination of matching source result sign conventions" });
  components.forEach((component, axis) => source.results.push({ id: `result:e17:${target}:${component}`, entity_ref: entity, basis_ref: basis, value: values[axis],
    kind: target === "displacement_magnitude" ? ["global_nodal_displacement_x", "global_nodal_displacement_y", "global_nodal_displacement_z"][axis] : "support_reaction_component_v2",
    unit: target === FORCE ? "N" : target === MOMENT ? "N*m" : "mm", metadata: metadata(component), source_result_refs: [] }));
  const row: Json = { id: `result:e17:${target}:magnitude`, kind: target, entity_ref: entity, basis_ref: basis, value: magnitude,
    unit: target === FORCE ? "N" : target === MOMENT ? "N*m" : "mm", source_result_refs: [] };
  if (target !== "displacement_magnitude") row.metadata = metadata(target === FORCE ? "force_magnitude" : "moment_magnitude");
  source.results.push(row);
  return source as MechanicsResult;
}

describe("E17 wasm G7 magnitude boundary", () => {
  it("the untouched invented statement passes current header/coverage/attribution and reaches G7", () => {
    const source = inventedSupportStatement();
    expect(readerDispatch(source).route).toBe("preview_physics");
    expect(() => readerValidate("preview-physics", source)).not.toThrow();
    const inconsistent = structuredClone(source);
    inconsistent.results.find(row => row.kind === FORCE)!.value = 1;
    expect(() => readerValidate("preview-physics", inconsistent)).toThrow("SOURCE_PREVIEW_PHYSICS_SUPPORT_MAGNITUDE");
  });
  for (const combined of [false, true]) {
    const targets = combined ? [FORCE, MOMENT, "displacement_magnitude"] : [FORCE, MOMENT];
    for (const target of targets) it.each(controls.controls)(`${combined ? "combined" : "direct"} ${target}: $id`, control => {
      const source = sourceControl(control, target, combined);
      const read = () => readerValidate("preview-physics", source);
      if (control.admitted) expect(read).not.toThrow();
      else expect(read).toThrow(combined ? "SOURCE_PREVIEW_PHYSICS_COMBINATION_MAGNITUDE" : "SOURCE_PREVIEW_PHYSICS_SUPPORT_MAGNITUDE");
    });
  }
});
