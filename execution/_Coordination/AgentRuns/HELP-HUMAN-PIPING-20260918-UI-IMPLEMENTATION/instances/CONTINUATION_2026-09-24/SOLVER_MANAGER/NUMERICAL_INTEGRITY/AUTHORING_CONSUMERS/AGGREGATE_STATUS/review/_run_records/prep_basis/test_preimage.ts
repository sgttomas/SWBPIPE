import { describe, expect, it } from "vitest";
import type { MechanicsResult, PreviewModel } from "../../types";
import { numericalResultStanding, sourceContract, PRECISION_CONTRACT_ID } from "./numericalResultQuality";
const model = { load_cases: [{ id: "case:a" }, { id: "case:b" }] } as PreviewModel;
function source(): MechanicsResult {
  return { schema_version: "0.2.0", document_kind: "MechanicsResult", run_id: "test", model_ref: "test", summary: {},
    status: { mechanics: "MECHANICS_SOLVED", rule_check: "RULE_INPUTS_INCOMPLETE", professional_acceptance: "NOT_PROVIDED" },
    results: [{ id: "r", kind: "global_nodal_rotation_x", value: -1e-12, unit: "rad", entity_ref: "n" }], diagnostics: [],
    producer: { component_name: "open_pipe_stress_product_physics", component_version: "0.2.0", semantic_contract_id: PRECISION_CONTRACT_ID },
    formulation_basis: { profile_id: "product_preview_mechanics_v1", limitations: ["Pressure/component preview limitations retained"] },
    numerical_quality: { value_representation: "finite_binary64", publication_quantization: "none", integrity_policy: "M03-INTEGRITY-v1", status: "checks_passed", cases: model.load_cases.map(c => ({ basis_ref: { ref_type: "load_case", ref_id: c.id }, structural_status: "passive_model_basis", solve_quality: "checks_passed", model_matrix_fidelity: "represented_equations_retained", accuracy_evidence: "not_claimed", evidence_refs: ["r"] })) } };
}
describe("numerical standing independent of authenticity and engineering acceptance", () => {
  it("requires complete requested cases and actual emitted evidence IDs", () => {
    const raw = source(), before = JSON.stringify(raw);
    expect(numericalResultStanding(raw, model).eligible).toBe(true);
    expect(JSON.stringify(raw)).toBe(before);
    expect(numericalResultStanding(raw).eligible).toBe(false);
    raw.numerical_quality!.cases.pop(); expect(numericalResultStanding(raw, model).eligible).toBe(false);
    raw.numerical_quality!.cases = source().numerical_quality!.cases;
    raw.numerical_quality!.cases[0].evidence_refs = ["invented"];
    expect(numericalResultStanding(raw, model).eligible).toBe(false);
  });
  it("does not promote rounded legacy, unassessed, unknown or contradictory headers", () => {
    const raw = source(); raw.schema_version = "0.1.0"; delete raw.producer; delete raw.numerical_quality; delete raw.formulation_basis;
    expect(numericalResultStanding(raw, model).status).toBe("needs_recompute");
    for (const status of ["not_assessed", "unresolved", "failed"] as const) { const candidate = source(); candidate.numerical_quality!.status = status; expect(numericalResultStanding(candidate, model).eligible).toBe(false); }
    const future = source(); future.producer!.semantic_contract_id = PRECISION_CONTRACT_ID.replace("precision-1", "pressure-1"); expect(sourceContract(future)).toBe("unsupported");
    delete future.producer; expect(sourceContract(future)).toBe("unsupported");
  });
  it("qualifies sensitive only with retained equations, consistent aggregate and resolved evidence", () => {
    const raw = source(); raw.numerical_quality!.cases[0].solve_quality = "sensitive";
    expect(numericalResultStanding(raw, model).eligible).toBe(false);
    raw.numerical_quality!.status = "sensitive"; expect(numericalResultStanding(raw, model).eligible).toBe(true);
    raw.numerical_quality!.cases[0].model_matrix_fidelity = "assembly_loss_detected";
    expect(numericalResultStanding(raw, model).eligible).toBe(false);
  });
});

it("rejects ambiguous or missing emitted IDs instead of granting evidence by a collapsed set", () => {
 const raw=source();raw.diagnostics=[{id:"r",code:"test",severity:"info",message:"duplicate"}];
 expect(numericalResultStanding(raw,model).eligible).toBe(false);
 delete raw.diagnostics[0].id;expect(numericalResultStanding(raw,model).eligible).toBe(false);
 raw.diagnostics[0].id="";expect(numericalResultStanding(raw,model).eligible).toBe(false);
});

it("rejects present falsy precision fields on legacy sources instead of treating them as absent", () => {
  const clean = source(); clean.schema_version = "0.1.0";
  delete clean.producer; delete clean.numerical_quality; delete clean.formulation_basis;
  expect(sourceContract(clean)).toBe("legacy");
  for (const key of ["producer", "numerical_quality", "formulation_basis"]) {
    for (const value of [null, false, 0, ""]) {
      const contradictory = Object.assign(structuredClone(clean), { [key]: value });
      const before = JSON.stringify(contradictory);
      expect(sourceContract(contradictory)).toBe("unsupported");
      expect(numericalResultStanding(contradictory, model).eligible).toBe(false);
      expect(JSON.stringify(contradictory)).toBe(before);
    }
  }
});

it("accepts general nonempty source basis shapes without granting Current load-case coverage", () => {
  const raw = source();
  raw.numerical_quality!.cases[0].basis_ref.ref_type = "custom-basis";
  expect(sourceContract(raw)).toBe("precision");
  expect(numericalResultStanding(raw, model).eligible).toBe(false);
  expect(numericalResultStanding(raw, model).findings).toContain("NUMERICAL_CASE_COVERAGE_INCOMPLETE");
  for (const key of ["ref_type", "ref_id"]) for (const value of ["", null, false, 0, {}, []]) {
    const bad = source(); (bad.numerical_quality!.cases[0].basis_ref as any)[key] = value;
    expect(sourceContract(bad)).toBe("unsupported");
  }
});
