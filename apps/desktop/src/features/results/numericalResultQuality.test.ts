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
  it("keeps sensitive inspectable but ineligible even with consistent metadata and a reference claim", () => {
    const raw = source(); raw.numerical_quality!.cases[0].solve_quality = "sensitive";
    expect(numericalResultStanding(raw, model).eligible).toBe(false);
    raw.numerical_quality!.status = "sensitive";
    raw.numerical_quality!.cases[0].accuracy_evidence = "reference_verified";
    const before = JSON.stringify(raw);
    expect(sourceContract(raw)).toBe("precision");
    expect(numericalResultStanding(raw, model).eligible).toBe(false);
    expect(JSON.stringify(raw)).toBe(before);
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

// Consumer contract controls: these hand-authored records are not producer qualification evidence.
type QualityStatus = NonNullable<MechanicsResult["numerical_quality"]>["status"];
function aggregateFixture(caseStatuses: QualityStatus[], claimed: QualityStatus) {
  const raw = source();
  const requested = { load_cases: caseStatuses.map((_, i) => ({ id: `case:${i}` })) } as PreviewModel;
  const template = raw.numerical_quality!.cases[0];
  raw.numerical_quality!.status = claimed;
  raw.numerical_quality!.cases = caseStatuses.map((solve_quality, i) => ({
    ...structuredClone(template), basis_ref: { ref_type: "load_case", ref_id: `case:${i}` }, solve_quality,
  }));
  for (const c of raw.numerical_quality!.cases) {
    if (c.solve_quality === "not_assessed") {
      Object.assign(c, { structural_status: "numerically_unresolved", model_matrix_fidelity: "not_assessed", accuracy_evidence: "not_claimed", evidence_refs: [] });
    } else if (c.solve_quality === "unresolved" || c.solve_quality === "failed") {
      const id = `diagnostic:${c.basis_ref.ref_id}`;
      Object.assign(c, { structural_status: "numerically_unresolved", model_matrix_fidelity: "assembly_uncertainty", accuracy_evidence: "unresolved", evidence_refs: [id] });
      raw.diagnostics.push({ id, code: "consumer-control", severity: "info", message: "Nonpassing case evidence control" });
    }
  }
  return { raw, requested };
}

describe("aggregate consistency is distinct from numerical qualification", () => {
  it.each<{ cases: QualityStatus[]; aggregate: QualityStatus }>([
    { cases: ["sensitive", "checks_passed"], aggregate: "sensitive" },
    { cases: ["not_assessed"], aggregate: "not_assessed" },
    { cases: ["unresolved"], aggregate: "unresolved" },
    { cases: ["failed"], aggregate: "failed" },
    { cases: ["checks_passed", "sensitive", "not_assessed"], aggregate: "not_assessed" },
    { cases: ["unresolved", "sensitive", "not_assessed"], aggregate: "unresolved" },
    { cases: ["unresolved", "failed", "sensitive"], aggregate: "failed" },
  ])("keeps honest $aggregate for $cases ineligible without a contradiction", ({ cases, aggregate }) => {
    const { raw, requested } = aggregateFixture(cases, aggregate);
    const before = JSON.stringify({ raw, requested });
    const standing = numericalResultStanding(raw, requested);
    expect(standing).toEqual({ contract: "precision", status: "needs_recompute", eligible: false,
      findings: ["NUMERICAL_INTEGRITY_NOT_QUALIFIED", "NUMERICAL_CASE_EVIDENCE_INCOMPLETE"] });
    expect(JSON.stringify({ raw, requested })).toBe(before);
  });

  it("recognizes unresolved aggregate with unassessed matrix fidelity", () => {
    const { raw, requested } = aggregateFixture(["unresolved"], "unresolved");
    raw.numerical_quality!.cases[0].model_matrix_fidelity = "not_assessed";
    expect(numericalResultStanding(raw, requested)).toEqual({ contract: "precision", status: "needs_recompute", eligible: false,
      findings: ["NUMERICAL_INTEGRITY_NOT_QUALIFIED", "NUMERICAL_CASE_EVIDENCE_INCOMPLETE"] });
  });

  it.each<{ cases: QualityStatus[]; claim: QualityStatus }>([
    { cases: ["unresolved", "sensitive"], claim: "sensitive" },
    { cases: ["failed", "unresolved"], claim: "unresolved" },
    { cases: ["not_assessed"], claim: "checks_passed" },
    { cases: ["checks_passed"], claim: "failed" },
    { cases: ["unresolved"], claim: "failed" },
    { cases: ["checks_passed"], claim: "sensitive" },
  ])("flags a mismatched $claim claim for $cases, including conservative claims", ({ cases, claim }) => {
    const { raw, requested } = aggregateFixture(cases, claim);
    const standing = numericalResultStanding(raw, requested);
    expect(standing.contract).toBe("precision");
    expect(standing.status).toBe("needs_recompute");
    expect(standing.eligible).toBe(false);
    expect(standing.findings).toContain("NUMERICAL_AGGREGATE_CONTRADICTION");
  });

  it.each<{ cases: QualityStatus[]; aggregate: QualityStatus }>([
    { cases: ["checks_passed", "checks_passed"], aggregate: "checks_passed" },
  ])("retains qualified $aggregate controls with complete evidence", ({ cases, aggregate }) => {
    const { raw, requested } = aggregateFixture(cases, aggregate);
    expect(numericalResultStanding(raw, requested)).toEqual({ contract: "precision", status: "integrity_checked", eligible: true, findings: [] });
  });

  it("defaults empty cases to not_assessed but still requires requested-case coverage", () => {
    const { raw } = aggregateFixture([], "not_assessed");
    expect(numericalResultStanding(raw, model)).toEqual({ contract: "precision", status: "needs_recompute", eligible: false,
      findings: ["NUMERICAL_INTEGRITY_NOT_QUALIFIED", "NUMERICAL_CASE_COVERAGE_INCOMPLETE"] });
    raw.numerical_quality!.status = "checks_passed";
    expect(numericalResultStanding(raw, model)).toEqual({ contract: "precision", status: "needs_recompute", eligible: false,
      findings: ["NUMERICAL_CASE_COVERAGE_INCOMPLETE", "NUMERICAL_AGGREGATE_CONTRADICTION"] });
  });
});
