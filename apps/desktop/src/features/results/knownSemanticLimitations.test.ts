import { describe, expect, it } from "vitest";
import inventedSparse from "../../../../../fixtures/results/preview_physics_invented_sparse.json";
import connectedSparse from "../../../../../fixtures/results/preview_physics_connected_sparse.json";
import precisionSparse from "../../../../../fixtures/results/precision_connected_ui_mechanics_sparse.json";
import physicsSparse from "../../../../../fixtures/results/physics_connected_ui_mechanics_sparse.json";
import multicase from "../../../../../fixtures/product_preview/source_blocks/multicase-sparse_interactive.raw.json";
import physicsSourceMixed from "../../../../../fixtures/product_preview/physics_source/mixed-sparse_interactive.raw.json";
import {
  COMBINATION_GATE_REASONS, FRESH_SEMANTIC_CONTRACT_IDS, N_HEADLINE, N_HEADLINE_WITHHELD, N_INTENSIFIED, N_P1, N_REPORT, N_RULE_RETIRED, N_SB, N_SB_MIXED,
  PRECISION_1_HISTORICAL_SEMANTICS, RULE_SOURCE_BLOCKS_SUMMARY_NOT_RELIABLE, SOURCE_BLOCKS_ORDINARY_CASE_LEGACY_SEMANTICS,
  isFreshSemanticResult, isRetiredResultId, knownSemanticNotices, resultRowLabel, ruleBindingRefusal, standingReason,
} from "./knownSemanticLimitations";
import { sourceBlockStanding, sourceBlocksOrdinaryCaseLegacy } from "./sourceBlockRecovery";
import { sourceContract } from "./numericalResultQuality";
import type { MechanicsResult } from "../../types";

const clone = (value: unknown) => structuredClone(value) as MechanicsResult;
/** Receipt-only derivation probe: one case relabelled as an ordinary selection.
 * The hashes are not recomputed; this never claims a validated publication. */
function mixed(): MechanicsResult {
  const source = clone(multicase);
  const receipt = source.source_block_recovery as { body: { cases: { selected_method: string; outcome: string }[] } };
  receipt.body.cases[1].selected_method = "ordinary_sparse_structural_v1";
  return source;
}

describe("frozen T0R texts (S1 §10), verbatim", () => {
  it("pins the notices and gate reasons", () => {
    expect(N_HEADLINE).toBe("maximum elastic normal stress; nominal; no component intensification; not a code stress");
    expect(N_HEADLINE_WITHHELD).toBe("Stress headline withheld: a stress maximum is not available for every member in every load case (arc members have no maximum on this route).");
    expect(N_P1).toBe("Historical precision-1 result. Its support reactions are force norms only (no moments; constant-effort supports read 0 N), its member stress is the sum of absolute axial and bending components (up to about 1.414 times the circular maximum), its headline covers only the first load case, and its component rows multiply stress by SIF×k and enter combinations. It stays readable but is not Current, rule-, report- or export-eligible. Solve again to obtain preview-physics-1 results.");
    expect(N_SB).toBe("Summary stress in this retained-source result is the sum of absolute axial and bending components, not the circular-section maximum. The admitted loads are nodal only, so it is conservative and at most √2 (about 1.414) times the maximum. Rule checks cannot bind to it until T3.");
    expect(N_SB_MIXED).toBe("This retained-source result contains an ordinary load case whose rows keep the retired precision-1 semantics (norm-only reactions and an absolute-sum stress summary). It is not Current, rule- or export-eligible until T3.");
    expect(N_REPORT).toBe("The report package is unavailable for fresh results until T6 (owner-accepted outage).");
    expect(N_RULE_RETIRED).toBe("This rule pack binds a result retired by preview-physics-1 (reaction_resultant, open_formula_stress_summary or component_user_stress_multiplier_review). Rebind it to support_reaction_component_v2, pipe_elastic_normal_stress_maximum_v2 or component_equal_factor_intensified_bending_stress_v1 rows; until then it reports RULE_INPUTS_INCOMPLETE.");
    expect(N_INTENSIFIED).toBe("equal-factor intensified bending stress i·hypot(My,Mz)/Z; user SIF; member Z; not a code stress; never combined");
    expect(COMBINATION_GATE_REASONS).toEqual({
      NONLINEAR_COMBINATION_REQUIRES_SOLVE: "Combination withheld: the model has nonlinear supports, so superposing case states is not a solved state.",
      CONSTANT_EFFORT_COMBINATION_REQUIRES_SOLVE: "Combination withheld: a constant-effort support would be counted by the sum of factors instead of once.",
      COMBINATION_MODULUS_BASIS_MIXED: "Combination withheld: its load cases were solved on different modulus bases.",
    });
  });
});

describe("static fresh-identity set and standing reasons", () => {
  it("admits only preview-physics-1, source-blocks-1, physics-1 and physics-source-1", () => {
    expect([...FRESH_SEMANTIC_CONTRACT_IDS].sort()).toEqual([
      "openpipestress.result_semantics/0.3.0/physics-1", "openpipestress.result_semantics/0.3.0/physics-source-1",
      "openpipestress.result_semantics/0.3.0/preview-physics-1", "openpipestress.result_semantics/0.3.0/source-blocks-1"]);
    expect(isFreshSemanticResult(clone(connectedSparse))).toBe(true);
    expect(isFreshSemanticResult(clone(physicsSparse))).toBe(true);
    expect(isFreshSemanticResult(clone(multicase))).toBe(true);
    expect(isFreshSemanticResult(clone(precisionSparse))).toBe(false); // precision-1 offered as fresh
    expect(standingReason(clone(precisionSparse))).toBe(PRECISION_1_HISTORICAL_SEMANTICS);
    expect(standingReason(clone(connectedSparse))).toBeNull();
  });
  it("derives the mixed source-blocks reason for non-composite source-blocks-1 only", () => {
    expect(sourceBlocksOrdinaryCaseLegacy(clone(multicase))).toBe(false); // all-selected keeps its standing
    expect(standingReason(clone(multicase))).toBeNull();
    const probe = mixed();
    expect(sourceContract(probe)).toBe("source_blocks");
    expect(sourceBlocksOrdinaryCaseLegacy(probe)).toBe(true);
    expect(standingReason(probe)).toBe(SOURCE_BLOCKS_ORDINARY_CASE_LEGACY_SEMANTICS);
    // A2 10: standing validates first. This relabelled receipt has no validated
    // registration, so it is refused on validation alone; the standing reason
    // is not reached. (No actual producer fixture carries a qualified ordinary
    // case in a non-composite source-blocks-1 envelope, so the validated-mixed
    // path is covered by the receipt derivation above.)
    const standing = sourceBlockStanding(probe, { load_cases: [] });
    expect(standing).toEqual({ eligible: false, findings: ["SOURCE_BLOCKS_VALIDATED_INVOCATION_REQUIRED"] });
    // physics-source-1 legitimately carries ordinary cases under physics-1 semantics.
    const composite = clone(physicsSourceMixed);
    expect(sourceContract(composite)).toBe("physics_source");
    expect((composite.source_block_recovery as any).body.cases.some((c: any) => c.selected_method !== "retained_source_blocks_exact_v1")).toBe(true);
    expect(sourceBlocksOrdinaryCaseLegacy(composite)).toBe(false);
    expect(standingReason(composite)).toBeNull();
  });
});

describe("selected + failed source-blocks-1 (receipt partial)", () => {
  it("stays not eligible, is not reported as mixed, and keeps the summary notice", () => {
    // Receipt-derivation probe only; the validator's own partial path adds
    // SOURCE_BLOCKS_ENVELOPE_UNQUALIFIED for a validated partial receipt.
    const source = clone(multicase);
    const body = (source.source_block_recovery as any).body;
    body.status = "partial";
    Object.assign(body.cases[1], { outcome: "failed", selected_method: null });
    expect(sourceBlocksOrdinaryCaseLegacy(source)).toBe(false);
    expect(standingReason(source)).toBeNull();
    expect(sourceBlockStanding(source, { load_cases: [] }).eligible).toBe(false);
    expect(knownSemanticNotices(source)).toEqual([{ id: "source-blocks-summary", text: N_SB }]);
  });
});

describe("rule binding refusal mirror", () => {
  it("refuses the all-selected source-blocks-1 summary row and headline ref; a force row still binds", () => {
    const source = clone(multicase);
    const summary = source.results.find(r => r.kind === "open_formula_stress_summary")!;
    expect(ruleBindingRefusal(source, summary)).toBe(RULE_SOURCE_BLOCKS_SUMMARY_NOT_RELIABLE);
    const headline = source.summary.max_open_formula_stress!;
    expect(ruleBindingRefusal(source, { id: headline.result_ref, kind: "any" })).toBe(RULE_SOURCE_BLOCKS_SUMMARY_NOT_RELIABLE);
    const force = source.results.find(r => r.kind === "element_local_axial_force")!;
    expect(ruleBindingRefusal(source, force)).toBeNull();
    const preview = clone(inventedSparse);
    expect(ruleBindingRefusal(preview, preview.results.find(r => r.kind === "pipe_elastic_normal_stress_maximum_v2")!)).toBeNull();
    const composite = clone(physicsSourceMixed);
    expect(composite.results.filter(r => ruleBindingRefusal(composite, r))).toEqual([]);
  });
  it("recognises retired id shapes, including case and combination qualification", () => {
    for (const id of ["result:reaction:support-S-100", "result:stress:pipe-P-100", "result:loadcase:load-L-200:stress:pipe-P-100",
      "result:combination:combination-C-OPER-ALT:reaction:support-S-100", "result:stress:component-C-110:pipe-P-100:end-j:user-multiplier"]) expect(isRetiredResultId(id)).toBe(true);
    for (const id of ["result:stress:pipe-P-100:end-j:torsional-shear", "result:elastic-maximum:10:load:L-100:10:pipe:P-120", "result:disp:node-N-140"]) expect(isRetiredResultId(id)).toBe(false);
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
    expect(resultRowLabel(intensified, clone(physicsSparse))).toBeNull();
  });
  it("shows the withheld reason when the stress headline is null on a solved preview result", () => {
    const source = clone(connectedSparse);
    source.summary.max_open_formula_stress = null;
    expect(knownSemanticNotices(source)).toEqual([{ id: "headline-withheld", text: N_HEADLINE_WITHHELD }]);
  });
  it("shows N-P1 on precision-1, N-SB on all-selected and N-SB-MIXED on mixed source-blocks-1", () => {
    expect(knownSemanticNotices(clone(precisionSparse))).toEqual([{ id: "precision-1", text: N_P1 }]);
    expect(knownSemanticNotices(clone(multicase))).toEqual([{ id: "source-blocks-summary", text: N_SB }]);
    expect(knownSemanticNotices(mixed())).toEqual([{ id: "source-blocks-mixed", text: N_SB_MIXED }]);
    expect(knownSemanticNotices(clone(physicsSparse))).toEqual([]);
    expect(knownSemanticNotices(clone(physicsSourceMixed))).toEqual([]);
  });
});
