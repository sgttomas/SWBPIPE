import { describe, expect, it } from "vitest";
import inventedSparse from "../../../../../fixtures/results/preview_physics_invented_sparse.json";
import connectedSparse from "../../../../../fixtures/results/preview_physics_connected_sparse.json";
import { retiredPrecisionStatement } from "../../test-support/retiredPrecision";
import pressureSparse from "../../../../../fixtures/results/pressure_connected_ui_mechanics_sparse.json";
import pressureSourceMixed from "../../../../../fixtures/product_preview/pressure_source/physics_source/mixed-sparse_interactive.raw.json";
import {
  COMBINATION_GATE_REASONS, FRESH_SEMANTIC_CONTRACT_IDS, N_HEADLINE, N_HEADLINE_WITHHELD, N_INTENSIFIED,
  isFreshSemanticResult, knownSemanticNotices, resultRowLabel, ruleBindingRefusal, standingReason,
} from "./knownSemanticLimitations";
import { numericalResultStanding } from "./resultStanding";
import { sourceContract } from "./resultReader";
import type { MechanicsResult } from "../../types";

const clone = (value: unknown) => structuredClone(value) as MechanicsResult;
describe("frozen T0R texts (S1 §10), verbatim", () => {
  it("pins the notices and gate reasons", () => {
    expect(N_HEADLINE).toBe("maximum elastic normal stress; nominal; no component intensification; not a code stress");
    expect(N_HEADLINE_WITHHELD).toBe("Stress headline withheld: a stress maximum is not available for every member in every load case (arc members have no maximum on this route).");
    expect(N_INTENSIFIED).toBe("equal-factor intensified bending stress i·hypot(My,Mz)/Z; user SIF; member Z; not a code stress; never combined");
    expect(COMBINATION_GATE_REASONS).toEqual({
      NONLINEAR_COMBINATION_REQUIRES_SOLVE: "Combination withheld: the model has nonlinear supports, so superposing case states is not a solved state.",
      CONSTANT_EFFORT_COMBINATION_REQUIRES_SOLVE: "Combination withheld: a constant-effort support would be counted by the sum of factors instead of once.",
      COMBINATION_MODULUS_BASIS_MIXED: "Combination withheld: its load cases were solved on different modulus bases.",
    });
  });
});

describe("static fresh-identity set and standing reasons", () => {
  it("equals Rust's FRESH_IDENTITIES: preview-physics-2, pressure-2, pressure-source-1 and U6's preview-physics-retained-1 (epoch 8); B7b retired the v2 identities", () => {
    // Epoch 8 (DEL-04-07 Design `desktop-exact-outputs.md` §4.1): the desktop's set equals
    // Rust's `FRESH_IDENTITIES` (`semantic_contract.rs`), one static set in every language.
    expect([...FRESH_SEMANTIC_CONTRACT_IDS].sort()).toEqual([
      "openpipestress.result_semantics/0.3.0/pressure-2", "openpipestress.result_semantics/0.3.0/pressure-source-1",
      "openpipestress.result_semantics/0.3.0/preview-physics-2", "openpipestress.result_semantics/0.3.0/preview-physics-retained-1"]);
    expect(new Set(FRESH_SEMANTIC_CONTRACT_IDS).size).toBe(FRESH_SEMANTIC_CONTRACT_IDS.length);
    expect(isFreshSemanticResult(clone(connectedSparse))).toBe(true);
    // pressure-2 and pressure-source-1 are fresh: membership is not standing, which stays
    // Rust's; each output surface applies its own policy entry and standing.
    expect(sourceContract(clone(pressureSparse))).toBe("pressure");
    expect(isFreshSemanticResult(clone(pressureSparse))).toBe(true);
    expect(sourceContract(clone(pressureSourceMixed))).toBe("pressure_source");
    expect(isFreshSemanticResult(clone(pressureSourceMixed))).toBe(true);
    expect(numericalResultStanding(clone(pressureSourceMixed), { load_cases: [] }).eligible).toBe(false);
    // B7b: a stored statement naming a retired v2 identity is refused by name: unsupported, never fresh.
    for (const [identity, sample] of [["physics-1", pressureSparse], ["physics-source-1", pressureSourceMixed], ["load-reference-1", pressureSparse], ["load-reference-source-1", pressureSourceMixed]] as const) {
      const retired = clone(sample);
      retired.producer = { ...retired.producer!, semantic_contract_id: `openpipestress.result_semantics/0.3.0/${identity}` };
      expect(sourceContract(retired), identity).toBe("unsupported");
      expect(isFreshSemanticResult(retired), identity).toBe(false);
      expect(numericalResultStanding(retired, { load_cases: [] }).findings, identity).toEqual(["SOURCE_NUMERICAL_CONTRACT_UNSUPPORTED"]);
    }
    // B1: a stored precision-1 statement is refused by name: unsupported, never fresh, no standing reason.
    expect(sourceContract(retiredPrecisionStatement())).toBe("unsupported");
    expect(isFreshSemanticResult(retiredPrecisionStatement())).toBe(false);
    expect(standingReason(retiredPrecisionStatement())).toBeNull();
    expect(standingReason(clone(connectedSparse))).toBeNull();
  });
  it("reads a stored source-blocks-1 statement as unsupported (B4/D1 (b)); composites carry ordinary cases", () => {
    const retired = clone(pressureSourceMixed);
    retired.producer = { ...retired.producer!, semantic_contract_id: "openpipestress.result_semantics/0.3.0/source-blocks-1" };
    expect(sourceContract(retired)).toBe("unsupported");
    expect(standingReason(retired)).toBeNull();
    expect(numericalResultStanding(retired, { load_cases: [] }).eligible).toBe(false);
    // pressure-source-1 legitimately carries ordinary cases under pressure-2 semantics.
    const composite = clone(pressureSourceMixed);
    expect(sourceContract(composite)).toBe("pressure_source");
    expect((composite.source_block_recovery as any).body.cases.some((c: any) => c.selected_method !== "retained_source_blocks_exact_v1")).toBe(true);
    expect(standingReason(composite)).toBeNull();
  });
});

describe("rule binding refusal mirror", () => {
  it("refuses no preview or composite row (the source-blocks-1 summary refusal left with that identity)", () => {
    const preview = clone(inventedSparse);
    expect(ruleBindingRefusal(preview, preview.results.find(r => r.kind === "pipe_elastic_normal_stress_maximum_v2")!)).toBeNull();
    const composite = clone(pressureSourceMixed);
    expect(composite.results.filter(r => ruleBindingRefusal(composite, r))).toEqual([]);
  });
});

describe("text-only notices", () => {
  it("labels the preview headline, the intensified measure and each gated combination", () => {
    const source = clone(inventedSparse);
    const notices = knownSemanticNotices(source);
    expect(notices.map(n => n.id)).toEqual(["headline-label", "intensified-label", "combination-gate:combination:C-OPER-ALT"]);
    expect(notices[0].text).toContain(N_HEADLINE);
    expect(notices[1].text).toContain(N_INTENSIFIED);
    expect(notices[2].text).toBe(`combination:C-OPER-ALT: ${COMBINATION_GATE_REASONS.NONLINEAR_COMBINATION_REQUIRES_SOLVE}`);
    const intensified = source.results.find(r => r.kind === "component_equal_factor_intensified_bending_stress_v1")!;
    expect(resultRowLabel(intensified, source)).toBe(N_INTENSIFIED);
    expect(resultRowLabel(source.results.find(r => r.kind === "pipe_elastic_normal_stress_maximum_v2")!, source)).toBe(N_HEADLINE);
    expect(resultRowLabel(intensified, clone(pressureSparse))).toBeNull();
  });
  it("shows the withheld reason when the stress headline is null on a solved preview result", () => {
    const source = clone(connectedSparse);
    source.summary.max_open_formula_stress = null;
    expect(knownSemanticNotices(source)).toEqual([{ id: "headline-withheld", text: N_HEADLINE_WITHHELD }]);
  });
  it("shows no notice on a retired precision-1 statement, pressure-2 or a composite", () => {
    expect(knownSemanticNotices(retiredPrecisionStatement())).toEqual([]);
    expect(knownSemanticNotices(clone(pressureSparse))).toEqual([]);
    expect(knownSemanticNotices(clone(pressureSourceMixed))).toEqual([]);
  });
});
