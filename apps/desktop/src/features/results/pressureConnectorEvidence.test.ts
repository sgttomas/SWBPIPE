// T4-U3 (S14): the TypeScript pressure-1 reader admits objective connector
// records and rows over the shared connector corpus (the Rust and Python
// readers read the same bytes), refuses each broken binding by name, and every
// other contract refuses connector evidence.
import { describe, expect, it } from "vitest";
import corpus from "../../../../../fixtures/results/pressure_v3_connector_reader_corpus.json";
import type { MechanicsResult } from "../../types";
import { validatePhysicsEvidence } from "./physicsResultEvidence";
import { validateLoadReferenceEvidence } from "./loadReferenceEvidence";
import { PRESSURE_CONTRACT_ID, validateExactPressureEvidence, validatePressureEvidence } from "./pressureContractEvidence";

type Probe = Record<string, any>;
const cases: [string, Probe][] = (corpus as Probe).cases.map((c: Probe): [string, Probe] => [c.label, c.envelope]);
const copy = (source: Probe) => structuredClone(source) as unknown as MechanicsResult;
const loadState = (source: Probe) => Object.hasOwn(source.contract_evidence, "load_reference_states");
const connectorRow = (e: Probe) => e.results.find((r: Probe) => r.kind.startsWith("connector_"));
const record = (e: Probe) => e.contract_evidence.connector[0];

// [label, mutation, reason]: the same mutations as the Rust and Python tests.
const mutations: [string, (e: Probe) => void, string][] = [
  ["record removed", e => { e.contract_evidence.connector = []; }, "CONNECTOR_ROW_UNBOUND"],
  ["record duplicated", e => { e.contract_evidence.connector.push(structuredClone(record(e))); }, "CONNECTOR_RECORD_DUPLICATE"],
  ["record extra key", e => { record(e).extra = 1; }, "CONNECTOR_RECORD_SHAPE"],
  ["record hardware tied", e => { record(e).hardware = "tied"; }, "CONNECTOR_RECORD_LAW"],
  ["record q_ref short", e => { record(e).q_ref = [0, 0]; }, "CONNECTOR_RECORD_FRAME"],
  ["record rotation scale", e => { record(e).work_matrix.rotation_scale_rad = 2; }, "CONNECTOR_WORK_MATRIX"],
  ["replaced span published", e => { record(e).replaced_pipe_id = "pipe:P-120"; }, "CONNECTOR_REPLACED_SPAN_PUBLISHED"],
  ["row removed", e => { e.results.splice(e.results.indexOf(connectorRow(e)), 1); }, "CONNECTOR_ROW_COVERAGE"],
  ["row unbound", e => { connectorRow(e).entity_ref = "component:C-999"; }, "CONNECTOR_ROW_UNBOUND"],
  ["row unknown kind", e => { connectorRow(e).kind = "connector_energy_v1"; }, "CONNECTOR_ROW_KIND"],
  ["row unit", e => { connectorRow(e).unit = "mm"; }, "CONNECTOR_ROW_SEMANTICS"],
  ["row basis", e => { connectorRow(e).metadata.basis = "objective_connector_v1;replaces_span=pipe:P-120;symmetric_midpoint_small_rotation_v1"; }, "CONNECTOR_ROW_SEMANTICS"],
  ["row sign", e => { connectorRow(e).metadata.sign_convention = "global end action on the connector at its node (node on element), f = B^T g; the connector acts on its node with -f"; }, "CONNECTOR_ROW_SEMANTICS"],
  ["row duplicated", e => { e.results.push({ ...structuredClone(connectorRow(e)), id: "result:connector:duplicate" }); }, "CONNECTOR_ROW_DUPLICATE"],
  ["replaced span row", e => {
    const row = structuredClone(e.results.find((r: Probe) => r.kind === "element_local_axial_force"));
    e.results.push({ ...row, id: "result:replaced-span", entity_ref: "pipe:P-130" });
  }, "CONNECTOR_REPLACED_SPAN_PUBLISHED"],
];

it("covers both document shapes, one connector each", () => {
  expect(cases).toHaveLength(2);
  expect(new Set(cases.map(([, s]) => loadState(s)))).toEqual(new Set([true, false]));
  for (const [, s] of cases) expect(s.contract_evidence.connector).toHaveLength(1);
});

describe.each(cases)("%s", (_label, source) => {
  it("is admitted by the pressure-1 reader and dispatch, without mutation", () => {
    const before = structuredClone(source);
    expect(source.producer.semantic_contract_id).toBe(PRESSURE_CONTRACT_ID);
    expect(() => validatePressureEvidence(source as unknown as MechanicsResult)).not.toThrow();
    expect(() => validateExactPressureEvidence(source as unknown as MechanicsResult)).not.toThrow();
    expect(source).toEqual(before);
  });
  it.each(mutations)("refuses a broken binding: %s", (_what, mutate, reason) => {
    const broken = structuredClone(source);
    mutate(broken);
    expect(() => validatePressureEvidence(copy(broken))).toThrow(new RegExp(`PHYSICS_EVIDENCE_${reason}$`));
  });
  it("is refused by every other contract", () => {
    const asV2 = structuredClone(source);
    for (const c of asV2.contract_evidence.exact_cases) c.profile_mode = "exact_straight_pressure_v2";
    const reader = loadState(source) ? validateLoadReferenceEvidence : validatePhysicsEvidence;
    expect(() => reader(copy(asV2))).toThrow(/CONNECTOR_UNSUPPORTED$|UNSUPPORTED_COMPOSITION$/);
    asV2.contract_evidence.connector = [];
    expect(() => reader(copy(asV2))).toThrow(/PHYSICS_EVIDENCE_CONNECTOR_UNSUPPORTED$/);
  });
});
