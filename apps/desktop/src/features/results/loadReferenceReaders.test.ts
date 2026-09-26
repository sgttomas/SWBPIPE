/** Desktop load/reference-state dispatch, readers and standing (T1 WP2) over the
 * committed producer raws. All inputs are invented; nothing here authenticates a
 * producer or qualifies engineering use. */
import { describe, expect, it } from "vitest";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import type { MechanicsResult, PreviewModel } from "../../types";
import {
  sourceContract, sourceSemanticBinding, numericalResultStanding, currentSemanticContract,
  LOAD_REFERENCE_CONTRACT_ID, LOAD_REFERENCE_CONTRACT_SHA256, LOAD_REFERENCE_SOURCE_CONTRACT_ID, LOAD_REFERENCE_SOURCE_CONTRACT_SHA256,
} from "./numericalResultQuality";
import { semanticContractForSource, resultSemantics } from "./resultSemantics";
import { isFreshSemanticResult, standingReason, knownSemanticNotices } from "./knownSemanticLimitations";
import { validateLoadReferenceEvidence, project, schemaShape } from "./loadReferenceEvidence";
import {
  validateLoadReferenceSourceEvidence, loadReferenceSourceStanding, loadReferenceSourceReceiptShape,
  LOAD_REFERENCE_SOURCE_NOT_NUMERICALLY_ELIGIBLE, LOAD_REFERENCE_SOURCE_VALIDATION_REQUIRED,
} from "./loadReferenceSourceEvidence";

const root = resolve(__dirname, "../../../../../");
const read = (path: string) => readFileSync(resolve(root, path), "utf8");
const json = (path: string) => JSON.parse(read(path));
const LR = ["connected-sparse_interactive", "connected-dense_scrutiny", "pressure-sparse_interactive", "pressure-dense_scrutiny"];
const LRS = ["n05", "n06", "fields", "mixed", "eigen_motion"].flatMap(name => ["sparse_interactive", "dense_scrutiny"].map(mode => `${name}-${mode}`));
const lrRaw = (name: string) => json(`fixtures/product_preview/load_reference/${name}.raw.json`) as MechanicsResult;
const lrModel = (name: string) => json(`fixtures/product_preview/load_reference/${name.split("-")[0]}.request.json`).model as PreviewModel;
const lrsRaw = (name: string) => json(`fixtures/product_preview/load_reference_source/${name}.raw.json`) as MechanicsResult;
const fallback = (mode: string) => json(`core/reporting/result_export/tests/fixtures/load_reference_fallback_uz-${mode}.raw.json`) as MechanicsResult;
const fallbackModel = () => json("core/reporting/result_export/tests/fixtures/load_reference_fallback_uz.request.json").model as PreviewModel;

describe("explicit dispatch of the two load/reference-state identities", () => {
  it.each(LR)("%s dispatches as load-reference-1 with its own table", name => {
    const raw = lrRaw(name);
    expect(sourceContract(raw)).toBe("load_reference");
    expect(sourceSemanticBinding(raw)).toEqual({ id: LOAD_REFERENCE_CONTRACT_ID, sha256: LOAD_REFERENCE_CONTRACT_SHA256 });
    expect(semanticContractForSource(raw)).toMatchObject({ semantic_contract_id: LOAD_REFERENCE_CONTRACT_ID, formulation_profile_id: "resolved_straight_load_state_v1" });
    expect(semanticContractForSource(raw).rows).toEqual(json("fixtures/results/semantic_contract_v0_3_load_reference_1.json").rows);
    expect(isFreshSemanticResult(raw)).toBe(true);
    expect(standingReason(raw)).toBeNull();
    expect(knownSemanticNotices(raw)).toEqual([]);
    const row = raw.results.find(r => r.kind === "pipe_axial_membrane_stress_v2" || r.kind.endsWith("_v2"))!;
    expect(resultSemantics(row, raw)?.category).toBe("physical_quantity");
  });
  it.each(LRS)("%s dispatches as load-reference-source-1 with its own table", name => {
    const raw = lrsRaw(name);
    expect(sourceContract(raw)).toBe("load_reference_source");
    expect(currentSemanticContract(raw)).toEqual({ id: LOAD_REFERENCE_SOURCE_CONTRACT_ID, sha256: LOAD_REFERENCE_SOURCE_CONTRACT_SHA256 });
    expect(semanticContractForSource(raw)).toMatchObject({ semantic_contract_id: LOAD_REFERENCE_SOURCE_CONTRACT_ID, formulation_profile_id: "resolved_straight_load_state_source_v1" });
    expect(semanticContractForSource(raw).rows).toEqual(json("fixtures/results/semantic_contract_v0_3_load_reference_source_1.json").rows);
    expect(isFreshSemanticResult(raw)).toBe(true);
    expect(standingReason(raw)).toBeNull();
  });
  it("binds each identity to exactly one profile, its evidence and its receipt", () => {
    const lr = lrRaw(LR[0]), lrs = lrsRaw(LRS[0]);
    const cases: [string, MechanicsResult][] = [];
    const push = (name: string, source: MechanicsResult, edit: (s: any) => void) => { const copy = structuredClone(source) as any; edit(copy); cases.push([name, copy]); };
    push("LR with the joined profile", lr, s => { s.formulation_basis.profile_id = "resolved_straight_load_state_source_v1"; });
    push("LR with the physics profile", lr, s => { s.formulation_basis.profile_id = "exact_straight_pressure_v2"; });
    push("LR without contract evidence", lr, s => { delete s.contract_evidence; });
    push("LR with null contract evidence", lr, s => { s.contract_evidence = null; });
    push("LR with array contract evidence", lr, s => { s.contract_evidence = []; });
    push("LR with a source-block receipt", lr, s => { s.source_block_recovery = structuredClone(lrs.source_block_recovery); });
    push("LR with a carrier namespace", lr, s => { s.carrier_evidence = {}; });
    push("joined with the LR profile", lrs, s => { s.formulation_basis.profile_id = "resolved_straight_load_state_v1"; });
    push("joined without its receipt", lrs, s => { delete s.source_block_recovery; });
    push("joined with the physics-source policy", lrs, s => { s.source_block_recovery.body.policy = "PHYSICS-SOURCE-1"; });
    push("joined with an extra receipt key", lrs, s => { s.source_block_recovery.body.invented = 1; });
    push("joined without contract evidence", lrs, s => { delete s.contract_evidence; });
    push("joined relabelled as load-reference-1", lrs, s => { s.producer.semantic_contract_id = LOAD_REFERENCE_CONTRACT_ID; s.formulation_basis.profile_id = "resolved_straight_load_state_v1"; });
    push("LR relabelled as joined", lr, s => { s.producer.semantic_contract_id = LOAD_REFERENCE_SOURCE_CONTRACT_ID; s.formulation_basis.profile_id = "resolved_straight_load_state_source_v1"; });
    for (const [name, source] of cases) expect(sourceContract(source), name).toBe("unsupported");
  });
  it("receipt shape: the joined policy plus the closed physics-source-1 shape", () => {
    const receipt = lrsRaw(LRS[0]).source_block_recovery as any;
    expect(loadReferenceSourceReceiptShape(receipt)).toBe(true);
    expect(loadReferenceSourceReceiptShape({ ...receipt, body: { ...receipt.body, policy: "PHYSICS-SOURCE-1" } })).toBe(false);
    expect(loadReferenceSourceReceiptShape({ ...receipt, extra: 1 })).toBe(false);
    expect(loadReferenceSourceReceiptShape(null)).toBe(false);
  });
});

describe("load-reference-1 goes through T0R's generic standing unchanged", () => {
  it.each(LR)("%s: reader validation passes and the published checks_passed cases are eligible with the actual model", name => {
    const raw = lrRaw(name), model = lrModel(name), before = JSON.stringify(raw);
    expect(() => validateLoadReferenceEvidence(raw, model)).not.toThrow();
    const standing = numericalResultStanding(raw, model);
    expect(standing).toMatchObject({ contract: "load_reference", status: "integrity_checked", eligible: true, findings: [] });
    expect(JSON.stringify(raw)).toBe(before);
  });
  it("a reader refusal is the standing finding, and never eligible", () => {
    const raw = lrRaw(LR[0]) as any, model = lrModel(LR[0]);
    raw.contract_evidence.load_reference_states[0].members[0].derived_G_pa = 1;
    const standing = numericalResultStanding(raw, model);
    expect(standing.eligible).toBe(false);
    expect(standing.findings).toContain("SOURCE_LOAD_REFERENCE_MEMBER_G_BINDING");
  });
  it("the actual model binding of the physics-1 reader applies to the projected copy", () => {
    const raw = lrRaw(LR[0]), model = structuredClone(lrModel(LR[0]));
    model.load_cases = model.load_cases.slice(1);
    const standing = numericalResultStanding(raw, model);
    expect(standing.eligible).toBe(false);
    expect(standing.findings.some(f => f.startsWith("SOURCE_LOAD_REFERENCE_PHYSICS_EVIDENCE: PHYSICS_EVIDENCE_MODEL_CASE_COVERAGE"))).toBe(true);
  });
  it.each(["sparse_interactive", "dense_scrutiny"])("a sensitive ordinary %s result stays readable but is never eligible", mode => {
    const raw = fallback(mode);
    expect(sourceContract(raw)).toBe("load_reference");
    expect(() => validateLoadReferenceEvidence(raw, fallbackModel())).not.toThrow();
    const standing = numericalResultStanding(raw, fallbackModel());
    expect(standing.eligible).toBe(false);
    expect(standing.findings).toContain("NUMERICAL_INTEGRITY_NOT_QUALIFIED");
  });
  it("the projection removes only what the pre-pass bound and never mutates its input", () => {
    const raw = lrRaw(LR[2]), before = JSON.stringify(raw), projected = project(raw) as any;
    expect(JSON.stringify(raw)).toBe(before);
    expect(Object.keys(projected.contract_evidence).sort()).toEqual(["connector", "exact_cases", "pressure"]);
    expect(projected.contract_evidence.exact_cases.every((c: any) => c.material_basis === "base_material_common_E_nu")).toBe(true);
  });
  it("the transport schema interpreter keeps Python's closed-object and anchored-pattern semantics", () => {
    const schema = { $defs: {}, type: "object", additionalProperties: false, required: ["a"], properties: { a: { type: "string", pattern: "^[0-9a-f]{4}$" }, n: { type: "integer", minimum: 0 } } };
    expect(schemaShape({ a: "00ff" }, schema, schema)).toBe(true);
    expect(schemaShape({ a: "00ffx" }, schema, schema)).toBe(false);
    expect(schemaShape({ a: "00ff", extra: 1 }, schema, schema)).toBe(false);
    expect(schemaShape({ a: "00ff", n: 1.5 }, schema, schema)).toBe(false);
    expect(schemaShape({ a: "00ff", n: -1 }, schema, schema)).toBe(false);
    expect(schemaShape({ n: 1 }, schema, schema)).toBe(false);
    expect(schemaShape([], { items: { type: "number" } }, schema)).toBe(true);
    expect(schemaShape({ x: true }, { properties: { x: { const: 1 } } }, schema)).toBe(false);
  });
});

describe("load-reference-source-1 standing: validation, then T0R's reason, then T1's early needs_recompute", () => {
  it.each(LRS)("%s is admitted by the joined reader and still needs recompute", async name => {
    const raw = lrsRaw(name), before = JSON.stringify(raw);
    expect(numericalResultStanding(raw).findings).toEqual([LOAD_REFERENCE_SOURCE_VALIDATION_REQUIRED]);
    await expect(validateLoadReferenceSourceEvidence(raw)).resolves.toBe(false);
    const standing = numericalResultStanding(raw, json(`fixtures/product_preview/load_reference_source/${name.split("-")[0]}.request.json`).model);
    expect(standing).toEqual({ contract: "load_reference_source", status: "needs_recompute", eligible: false, findings: [LOAD_REFERENCE_SOURCE_NOT_NUMERICALLY_ELIGIBLE] });
    expect(JSON.stringify(raw)).toBe(before);
  });
  it("a refused joined envelope keeps its validation finding (never the T1 reason)", async () => {
    const raw = lrsRaw(LRS[0]) as any;
    raw.source_block_recovery.receipt_sha256 = "0".repeat(64);
    await expect(validateLoadReferenceSourceEvidence(raw)).rejects.toThrow("SOURCE_LOAD_REFERENCE_JOIN_RECEIPT_HASH");
    expect(numericalResultStanding(raw).findings).toEqual(["SOURCE_LOAD_REFERENCE_JOIN_RECEIPT_HASH"]);
  });
  it("a registration never transfers to a copy or survives an edit", async () => {
    const raw = lrsRaw(LRS[6]);
    await validateLoadReferenceSourceEvidence(raw);
    expect(loadReferenceSourceStanding(raw).findings).toEqual([LOAD_REFERENCE_SOURCE_NOT_NUMERICALLY_ELIGIBLE]);
    expect(loadReferenceSourceStanding(structuredClone(raw)).findings).toEqual([LOAD_REFERENCE_SOURCE_VALIDATION_REQUIRED]);
    (raw.results[0] as any).value = -0;
    expect(loadReferenceSourceStanding(raw).findings).toEqual([LOAD_REFERENCE_SOURCE_VALIDATION_REQUIRED]);
    (raw.results[0] as any).value = 1;
    expect(loadReferenceSourceStanding(raw).findings).toEqual([LOAD_REFERENCE_SOURCE_VALIDATION_REQUIRED]);
  });
});
