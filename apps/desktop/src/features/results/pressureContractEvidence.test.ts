// T4-U2a: TypeScript reader dispatch on the exact pressure contract identity,
// over the shared generated pressure-1 corpus (the Rust and Python readers read
// the same bytes). v2 is unchanged and still refuses p_pa < 0; v3 admits
// straight families with signed p_pa; neither is read as the other; unknown
// identities are refused.
import { describe, expect, it } from "vitest";
import corpus from "../../../../../fixtures/results/pressure_v3_straight_reader_corpus.json";
import type { MechanicsResult } from "../../types";
import { validatePhysicsEvidence } from "./physicsResultEvidence";
import { LOAD_REFERENCE_CONTRACT_ID, validateLoadReferenceEvidence } from "./loadReferenceEvidence";
import { PHYSICS_EVIDENCE_CONTRACT_ID, PRESSURE_CONTRACT_ID, validateExactPressureEvidence, validatePressureEvidence } from "./pressureContractEvidence";

type Probe = Record<string, any>;
const cases: [string, Probe][] = (corpus as Probe).cases.map((c: Probe): [string, Probe] => [c.label, c.envelope]);
const copy = (source: Probe) => structuredClone(source) as unknown as MechanicsResult;
const loadState = (source: Probe) => Object.hasOwn(source.contract_evidence, "load_reference_states");
const v2Reader = (source: Probe) => (loadState(source) ? validateLoadReferenceEvidence : validatePhysicsEvidence)(copy(source));
function relabel(source: Probe, version: string, mode: string): Probe {
  const out = structuredClone(source);
  for (const c of out.contract_evidence.exact_cases) c.profile_mode = mode;
  for (const r of out.contract_evidence.pressure) { r.profile_version = version; r.profile_mode = mode; }
  return out;
}

it("covers both document shapes and signed pressure", () => {
  expect(cases).toHaveLength(4);
  expect(new Set(cases.map(([, s]) => loadState(s)))).toEqual(new Set([true, false]));
  for (const [, s] of cases) if (!loadState(s)) expect(s.contract_evidence.pressure.every((r: Probe) => r.p_pa < 0)).toBe(true);
});

describe.each(cases)("%s", (_label, source) => {
  it("is admitted by the pressure-1 reader and dispatch, without mutation", () => {
    const before = structuredClone(source);
    expect(source.producer.semantic_contract_id).toBe(PRESSURE_CONTRACT_ID);
    expect(() => validatePressureEvidence(source as unknown as MechanicsResult)).not.toThrow();
    expect(() => validateExactPressureEvidence(source as unknown as MechanicsResult)).not.toThrow();
    expect(source).toEqual(before);
  });
  it("is never read as v2, and v2 evidence is never read as v3", () => {
    expect(() => v2Reader(source)).toThrow(/PHYSICS_EVIDENCE_EXACT_PRESSURE_V3_READ_AS_V2$/);
    const asV2 = relabel(source, "2.0.0", "exact_straight_pressure_v2");
    expect(() => validatePressureEvidence(copy(asV2))).toThrow(/PHYSICS_EVIDENCE_EXACT_STRAIGHT_PRESSURE_V2_READ_AS_V3$/);
    if (loadState(source)) expect(() => v2Reader(asV2)).not.toThrow();
    else expect(() => v2Reader(asV2)).toThrow(/PHYSICS_EVIDENCE_PRESSURE_INVALID/);
  });
  it("refuses unknown identities and profiles", () => {
    const unknown = structuredClone(source); unknown.producer.semantic_contract_id = "openpipestress.result_semantics/0.3.0/pressure-2";
    expect(() => validateExactPressureEvidence(copy(unknown))).toThrow(/PRESSURE_CONTRACT_UNKNOWN/);
    const profile = structuredClone(source); profile.formulation_basis.profile_id = "exact_straight_pressure_v2";
    expect(() => validateExactPressureEvidence(copy(profile))).toThrow(/PRESSURE_FORMULATION_PROFILE/);
    expect(() => validatePressureEvidence(copy(relabel(source, "3.0.0", "exact_pressure_v4")))).toThrow(/PHYSICS_EVIDENCE_/);
    const relabelled = structuredClone(source);
    relabelled.producer.semantic_contract_id = loadState(source) ? LOAD_REFERENCE_CONTRACT_ID : PHYSICS_EVIDENCE_CONTRACT_ID;
    expect(() => validateExactPressureEvidence(copy(relabelled))).toThrow(/EXACT_PRESSURE_V3_READ_AS_V2$/);
  });
});
