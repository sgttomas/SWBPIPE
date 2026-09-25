import { describe, expect, it } from "vitest";
import sparse from "../../../../../fixtures/results/physics_connected_mechanics_sparse.json";
import dense from "../../../../../fixtures/results/physics_connected_mechanics_dense.json";
import thermalSparse from "../../../../../fixtures/results/physics_thermal_ui_mechanics_sparse.json";
import thermalDense from "../../../../../fixtures/results/physics_thermal_ui_mechanics_dense.json";
import type { MechanicsResult } from "../../types";
import { validatePhysicsEvidence } from "./physicsResultEvidence";

type Probe = Record<string, any>;
const actual = (source: unknown) => structuredClone(source) as MechanicsResult;
const group = (s: Probe) => s.contract_evidence.exact_cases[0].pressure_rhs_assembly.groups[0];
const changes: [string, (source: Probe) => void][] = [
  ["Poisson coefficient", s => { group(s).terms[0].coefficient = 999; }],
  ["terminal cap coefficient", s => { group(s).terms.find((t: Probe) => t.kind === "terminal_cap").coefficient = 999; }],
  ["duplicate source group", s => { s.contract_evidence.exact_cases[0].pressure_rhs_assembly.groups.push(structuredClone(group(s))); }],
  ["source annulus contradiction", s => { s.contract_evidence.exact_cases[0].pipe_sections[0].ro_m = 0.1; s.contract_evidence.pressure[0].geometry[0].ro_m = 0.1; }],
  ["unknown physical kind", s => { const row = structuredClone(s.results.find((r: Probe) => r.kind.endsWith("_v2"))); row.id = "unknown:v2"; row.kind = "pipe_unknown_v2"; s.results.push(row); }],
  ["source-block namespace", s => { s.source_block_recovery = {}; }],
  ["pressure-only ledger thermal claim", s => { s.contract_evidence.pressure[0].applied_loads[0].thermal_included = true; }],
  ["malformed generic operands", s => { s.results[0].source_result_refs = {}; }],
  ["cyclic generic operand", s => { const row = s.results.find((r: Probe) => !r.kind.endsWith("_v2")); row.source_result_refs = [{ ref_type: "result_value", ref_id: row.id }]; }],
  ["unbound maximum and headline", s => {
    const row = structuredClone(s.results.find((r: Probe) => r.kind === "pipe_elastic_normal_stress_maximum_v2" && r.basis_ref.ref_id === s.contract_evidence.exact_cases[1].load_case_id));
    row.id = "unbound:maximum"; row.value = 100000000.25; s.results.push(row);
    s.summary.max_open_formula_stress = { result_ref: row.id, location_ref: row.entity_ref, value: row.value, unit: row.unit };
  }],
];

describe.each([["sparse", sparse], ["dense", dense]] as const)("physics evidence parity: %s", (_mode, fixture) => {
  it("admits actual joined source without mutation", () => {
    const source = actual(fixture), before = structuredClone(source);
    expect(() => validatePhysicsEvidence(source)).not.toThrow();
    expect(source).toEqual(before);
  });
  it.each(changes)("rejects %s", (_name, mutate) => {
    const source = actual(fixture); mutate(source);
    expect(() => validatePhysicsEvidence(source)).toThrow(/PHYSICS_EVIDENCE_/);
  });
  it.each(["assembled_pressure_rhs_global", "rounded_cap_rhs_global", "rounded_poisson_rhs_global"])("rejects pressure moments in %s", field => {
    for (const slot of [3, 4, 5, 9, 10, 11]) {
      const source = actual(fixture);
      (source.contract_evidence as Probe).exact_cases[0].pressure_rhs_assembly[field][slot] = 999;
      expect(() => validatePhysicsEvidence(source)).toThrow(/RHS_PRESSURE_MOMENT/);
    }
  });
});

it.each([["sparse", thermalSparse], ["dense", thermalDense]] as const)("admits actual %s pressure and thermal mechanics with separate ledgers", (_mode, fixture) => {
  const source = actual(fixture), before = structuredClone(source);
  const region = (source.contract_evidence as Probe).pressure[0];
  expect(source.status.mechanics).toBe("MECHANICS_SOLVED");
  expect(region.materials[0].thermal_consumed).toBe(true);
  expect(region.applied_loads[0].thermal_included).toBe(false);
  expect(() => validatePhysicsEvidence(source)).not.toThrow();
  expect(source).toEqual(before);
});
