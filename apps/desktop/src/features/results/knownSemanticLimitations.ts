/** Frozen T0R notices, gate reasons and standing reasons (S1_INTERFACE §10).
 * Text only. N-A: these strings are UI labels and reasons; they are never
 * written into an exported results or stress-neutral document or its manifest. */
import { sourceContract, currentSemanticContract, PRECISION_CONTRACT_ID, PHYSICS_CONTRACT_ID, PHYSICS_SOURCE_CONTRACT_ID, PREVIEW_PHYSICS_CONTRACT_ID } from "./numericalResultQuality";
import { SOURCE_BLOCKS_CONTRACT_ID, sourceBlocksOrdinaryCaseLegacy } from "./sourceBlockRecovery";
import type { MechanicsResult } from "../../types";

/** Static fresh-identity set: the same constant in each language, no route
 * predicate. T1 adds its identities when it activates them. */
export const FRESH_SEMANTIC_CONTRACT_IDS: readonly string[] = Object.freeze([
  PREVIEW_PHYSICS_CONTRACT_ID,
  SOURCE_BLOCKS_CONTRACT_ID,
  PHYSICS_CONTRACT_ID,
  PHYSICS_SOURCE_CONTRACT_ID,
]);

export const N_HEADLINE = "maximum elastic normal stress; nominal; no component intensification; not a code stress";
export const N_HEADLINE_WITHHELD = "Stress headline withheld: a stress maximum is not available for every member in every load case (arc members have no maximum on this route).";
export const N_P1 = "Historical precision-1 result. Its support reactions are force norms only (no moments; constant-effort supports read 0 N), its member stress is the sum of absolute axial and bending components (up to about 1.414 times the circular maximum), its headline covers only the first load case, and its component rows multiply stress by SIF×k and enter combinations. It stays readable but is not Current, rule-, report- or export-eligible. Solve again to obtain preview-physics-1 results.";
export const N_SB = "Summary stress in this retained-source result is the sum of absolute axial and bending components, not the circular-section maximum. The admitted loads are nodal only, so it is conservative and at most √2 (about 1.414) times the maximum. Rule checks cannot bind to it until T3.";
export const N_SB_MIXED = "This retained-source result contains an ordinary load case whose rows keep the retired precision-1 semantics (norm-only reactions and an absolute-sum stress summary). It is not Current, rule- or export-eligible until T3.";
export const N_REPORT = "The report package is unavailable for fresh results until T6 (owner-accepted outage).";
export const N_RULE_RETIRED = "This rule pack binds a result retired by preview-physics-1 (reaction_resultant, open_formula_stress_summary or component_user_stress_multiplier_review). Rebind it to support_reaction_component_v2, pipe_elastic_normal_stress_maximum_v2 or component_equal_factor_intensified_bending_stress_v1 rows; until then it reports RULE_INPUTS_INCOMPLETE.";
export const N_INTENSIFIED = "equal-factor intensified bending stress i·hypot(My,Mz)/Z; user SIF; member Z; not a code stress; never combined";

export const COMBINATION_GATE_REASONS: Readonly<Record<string, string>> = Object.freeze({
  NONLINEAR_COMBINATION_REQUIRES_SOLVE: "Combination withheld: the model has nonlinear supports, so superposing case states is not a solved state.",
  CONSTANT_EFFORT_COMBINATION_REQUIRES_SOLVE: "Combination withheld: a constant-effort support would be counted by the sum of factors instead of once.",
  COMBINATION_MODULUS_BASIS_MIXED: "Combination withheld: its load cases were solved on different modulus bases.",
});

export const PRECISION_1_HISTORICAL_SEMANTICS = "PRECISION_1_HISTORICAL_SEMANTICS";
export const SOURCE_BLOCKS_ORDINARY_CASE_LEGACY_SEMANTICS = "SOURCE_BLOCKS_ORDINARY_CASE_LEGACY_SEMANTICS";
export const RULE_SOURCE_BLOCKS_SUMMARY_NOT_RELIABLE = "RULE_SOURCE_BLOCKS_SUMMARY_NOT_RELIABLE";
export const RULE_BINDS_RETIRED_RESULT = "RULE_BINDS_RETIRED_RESULT";
export const REPORT_PACKAGE_FRESH_RESULT_UNAVAILABLE = "REPORT-PACKAGE-FRESH-RESULT-UNAVAILABLE";
export const REPORT_PACKAGE_PRECISION_1_HISTORICAL = "REPORT-PACKAGE-PRECISION-1-HISTORICAL";

export const PREVIEW_INTENSIFIED_KIND = "component_equal_factor_intensified_bending_stress_v1";
export const PREVIEW_MAXIMUM_KIND = "pipe_elastic_normal_stress_maximum_v2";

/** Readable identity, by dispatch, that belongs to the static fresh set. */
export function isFreshSemanticResult(source: MechanicsResult | null | undefined): boolean {
  const binding = currentSemanticContract(source);
  return !!binding && FRESH_SEMANTIC_CONTRACT_IDS.includes(binding.id);
}

function isNonCompositeSourceBlocks(source: MechanicsResult): boolean {
  try { return sourceContract(source) === "source_blocks"; } catch { return false; }
}

/** Reader-derived standing reason (not a producer field); null when none applies. */
export function standingReason(source: MechanicsResult | null | undefined): string | null {
  if (!source) return null;
  let route: ReturnType<typeof sourceContract>;
  try { route = sourceContract(source); } catch { return null; }
  if (route === "precision" && source.producer?.semantic_contract_id === PRECISION_CONTRACT_ID) return PRECISION_1_HISTORICAL_SEMANTICS;
  if (route === "source_blocks" && sourceBlocksOrdinaryCaseLegacy(source)) return SOURCE_BLOCKS_ORDINARY_CASE_LEGACY_SEMANTICS;
  return null;
}

/** Mirror of `result_export::semantic_contract::rule_binding_refusal`. */
export function ruleBindingRefusal(source: MechanicsResult, row: Pick<MechanicsResult["results"][number], "id" | "kind">): string | null {
  if (!isNonCompositeSourceBlocks(source) || source.producer?.semantic_contract_id !== SOURCE_BLOCKS_CONTRACT_ID) return null;
  if (row.kind === "open_formula_stress_summary") return RULE_SOURCE_BLOCKS_SUMMARY_NOT_RELIABLE;
  if (row.id === source.summary.max_open_formula_stress?.result_ref) return RULE_SOURCE_BLOCKS_SUMMARY_NOT_RELIABLE;
  return null;
}

// Base-id shapes of the three retired kinds, optionally qualified by load case or
// combination (`result:loadcase:{stable(case)}:…`, `result:combination:{stable(c)}:…`).
const RETIRED_ID_TAILS = [
  /^reaction:[^:]+$/,               // reaction_resultant
  /^stress:[^:]+$/,                 // open_formula_stress_summary
  /^stress:.+:user-multiplier$/,    // component_user_stress_multiplier_review
];
/** True when an unresolved id has the shape of a row that preview-physics-1 retired. */
export function isRetiredResultId(resultId: string): boolean {
  const match = /^result:(?:(?:loadcase|combination):[^:]+:)?(.+)$/.exec(resultId);
  return !!match && RETIRED_ID_TAILS.some(tail => tail.test(match[1]));
}

export type KnownSemanticNotice = { id: string; text: string };

/** Text-only notices a surface shows for a received result. The caller decides
 * placement; no notice changes standing, rows or exported bytes. */
export function knownSemanticNotices(source: MechanicsResult | null | undefined): KnownSemanticNotice[] {
  if (!source) return [];
  let route: ReturnType<typeof sourceContract>;
  try { route = sourceContract(source); } catch { return []; }
  const notices: KnownSemanticNotice[] = [];
  if (route === "precision") notices.push({ id: "precision-1", text: N_P1 });
  if (route === "source_blocks") {
    notices.push(sourceBlocksOrdinaryCaseLegacy(source)
      ? { id: "source-blocks-mixed", text: N_SB_MIXED }
      : { id: "source-blocks-summary", text: N_SB });
  }
  if (route === "preview_physics") {
    const solved = source.status.mechanics === "MECHANICS_SOLVED";
    if (source.summary.max_open_formula_stress) notices.push({ id: "headline-label", text: `Stress headline: ${N_HEADLINE}.` });
    else if (solved) notices.push({ id: "headline-withheld", text: N_HEADLINE_WITHHELD });
    if (source.results.some(row => row.kind === PREVIEW_INTENSIFIED_KIND)) notices.push({ id: "intensified-label", text: `Intensified measure: ${N_INTENSIFIED}.` });
    for (const gate of previewCombinationGates(source)) {
      if (gate.withheld && gate.reason) notices.push({ id: `combination-gate:${gate.combination_id}`, text: `${gate.combination_id}: ${COMBINATION_GATE_REASONS[gate.reason] ?? gate.reason}` });
    }
  }
  return notices;
}

export function previewCombinationGates(source: MechanicsResult): { combination_id: string; withheld: boolean; reason: string | null }[] {
  const gates = (source.contract_evidence as { combination_gates?: unknown } | undefined)?.combination_gates;
  if (!Array.isArray(gates)) return [];
  return gates.filter((g): g is { combination_id: string; withheld: boolean; reason: string | null } =>
    !!g && typeof g === "object" && typeof (g as { combination_id?: unknown }).combination_id === "string"
    && typeof (g as { withheld?: unknown }).withheld === "boolean");
}

/** Row-level label for a listed row of a new kind; null for other rows. */
export function resultRowLabel(row: Pick<MechanicsResult["results"][number], "kind">, source: MechanicsResult | null | undefined): string | null {
  if (!source) return null;
  try { if (sourceContract(source) !== "preview_physics") return null; } catch { return null; }
  if (row.kind === PREVIEW_INTENSIFIED_KIND) return N_INTENSIFIED;
  if (row.kind === PREVIEW_MAXIMUM_KIND) return N_HEADLINE;
  return null;
}
