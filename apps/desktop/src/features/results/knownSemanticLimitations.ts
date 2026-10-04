/** Frozen T0R notices, gate reasons and standing reasons (S1_INTERFACE §10).
 * Text only. N-A: these strings are UI labels and reasons; they are never
 * written into an exported results or stress-neutral document or its manifest. */
import { sourceContract, currentSemanticContract, PRECISION_CONTRACT_ID, PHYSICS_CONTRACT_ID, PHYSICS_SOURCE_CONTRACT_ID, PREVIEW_PHYSICS_CONTRACT_ID, LOAD_REFERENCE_CONTRACT_ID, LOAD_REFERENCE_SOURCE_CONTRACT_ID, PREVIEW_PHYSICS_RETAINED_CONTRACT_ID } from "./numericalResultQuality";
import { SOURCE_BLOCKS_CONTRACT_ID, sourceBlocksOrdinaryCaseLegacy } from "./sourceBlockRecovery";
import { classificationSummary, retainedRowClasses } from "./retainedPrecisionStanding";
import { decodeBinary64, type AccuracyClass } from "./retainedPrecision";
import type { MechanicsResult } from "../../types";

/** Static fresh-identity set: the same constant in each language, no route
 * predicate. T1 added its load/reference-state identities on activation.
 * U6d (D-U6-6; D2 4.7 S-1) adds the F2a preview successor: membership is not
 * standing, which comes only from the accepted reader's registered validation. */
export const FRESH_SEMANTIC_CONTRACT_IDS: readonly string[] = Object.freeze([
  PREVIEW_PHYSICS_CONTRACT_ID,
  SOURCE_BLOCKS_CONTRACT_ID,
  PHYSICS_CONTRACT_ID,
  PHYSICS_SOURCE_CONTRACT_ID,
  LOAD_REFERENCE_CONTRACT_ID,
  LOAD_REFERENCE_SOURCE_CONTRACT_ID,
  PREVIEW_PHYSICS_RETAINED_CONTRACT_ID,
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

/** D2 4.9.9 (revision 4, N-5): binding refusals for classified successor rows
 * before S-I. They never demote the envelope's standing. */
export const RULE_QUANTITY_BELOW_VERIFIED_FLOOR = "RULE_QUANTITY_BELOW_VERIFIED_FLOOR";
export const RULE_QUANTITY_NOT_COVERED = "RULE_QUANTITY_NOT_COVERED";
/** D2 4.9.9 notice texts (before S-I). `{b}` is the receipt's published absolute
 * bound and `{unit}` the SI unit the reader classified the quantity in. */
export const N_RP_ABSOLUTE = "Uncovered quantity: verified only to an absolute bound of ±{b} {unit}, below the relative accuracy floor for this body. It is shown for inspection; rule checks cannot bind to it.";
export const N_RP_NOT_COVERED = "Uncovered quantity: no verified accuracy for this quantity kind. It is shown for inspection; rule checks cannot bind to it.";
export const N_RP_UNVALIDATED = "Retained-precision accuracy classes are unavailable for these exact bytes (not validated in this session, or refused by the reader). No quantity of this result is shown as verified, and rule checks cannot bind to any of them.";
/** The SI unit the reader normalizes a row's unit to, and so the unit the receipt's
 * absolute bound b is published in. The same table as Rust `derivative::si_unit`
 * (RV88 U6a S-2): m and mm to m, rad, N and kN to N, N*m and kN*m to N*m, Pa and MPa
 * to Pa. TS emits only this display label, never the derivative's disclosure message. */
const RETAINED_SI_UNIT: Readonly<Record<string, string>> = Object.freeze({ m: "m", mm: "m", rad: "rad", N: "N", kN: "N", "N*m": "N*m", "kN*m": "N*m", Pa: "Pa", MPa: "Pa" });
/** b with three significant digits, rounded upward: the printed bound is never below b. */
export function upwardBoundText(b: number): string {
  if (b === 0) return "0";
  const [mantissa, exponent] = b.toExponential(2).split("e");
  if (Number(`${mantissa}e${exponent}`) > b) return `${mantissa}e${exponent}`;
  let digits = Math.round(Number(mantissa) * 100) + 1, power = Number(exponent);
  if (digits >= 1000) { digits = 100; power += 1; }
  return `${(digits / 100).toFixed(2)}e${power < 0 ? "-" : "+"}${Math.abs(power)}`;
}
/** The absolute-class label in the bound's SI unit. As in Rust, a unit the reader
 * does not normalize names no bound: the row is labelled uncovered instead. */
export function retainedAbsoluteNotice(boundBits: string, unit: string): string {
  const si = Object.hasOwn(RETAINED_SI_UNIT, unit) ? RETAINED_SI_UNIT[unit] : null;
  return si === null ? N_RP_NOT_COVERED : N_RP_ABSOLUTE.replace("{b}", upwardBoundText(decodeBinary64(boundBits))).replace("{unit}", si);
}
/** The binding refusal of one validated class (D2 4.9.9, before S-I). */
export function classBindingRefusal(cls: AccuracyClass | null | undefined): string | null {
  return cls === "absolute_verified" ? RULE_QUANTITY_BELOW_VERIFIED_FLOOR : cls === "not_covered" ? RULE_QUANTITY_NOT_COVERED : null;
}
/** The class label of a successor row from the registered validation, or null. */
export function retainedRowClassLabel(row: { id?: string; unit?: string }, source: MechanicsResult | null | undefined): string | null {
  const classified = retainedRowClasses(source)?.get(row.id!);
  // G5c gives every absolute_verified row its published bound.
  if (classified?.class === "absolute_verified") return retainedAbsoluteNotice(classified.bound_bits!, row.unit!);
  if (classified?.class === "not_covered") return N_RP_NOT_COVERED;
  return null;
}

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

/** Mirror of `result_export::semantic_contract::rule_binding_refusal`. For the
 * successor (selected by its producer identity, as in Rust) a row binds only when
 * its validated class allows it; a headline binds the row its `result_ref` names
 * and is refused exactly as that row. Without a valid registration no row has a
 * validated class, so every row is refused (fail closed, as Rust F5). */
export function ruleBindingRefusal(source: MechanicsResult, row: Pick<MechanicsResult["results"][number], "id" | "kind">): string | null {
  if (source.producer?.semantic_contract_id === PREVIEW_PHYSICS_RETAINED_CONTRACT_ID) {
    const classes = retainedRowClasses(source);
    return classes ? classBindingRefusal(classes.get(row.id)?.class) : RULE_QUANTITY_NOT_COVERED;
  }
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
  // U6d: the successor reuses the preview notices (its rows and evidence are preview-physics-1's).
  if (route === "preview_physics" || route === "retained_preview_physics") {
    const solved = source.status.mechanics === "MECHANICS_SOLVED";
    if (source.summary.max_open_formula_stress) notices.push({ id: "headline-label", text: `Stress headline: ${N_HEADLINE}.` });
    else if (solved) notices.push({ id: "headline-withheld", text: N_HEADLINE_WITHHELD });
    if (source.results.some(row => row.kind === PREVIEW_INTENSIFIED_KIND)) notices.push({ id: "intensified-label", text: `Intensified measure: ${N_INTENSIFIED}.` });
    for (const gate of previewCombinationGates(source)) {
      if (gate.withheld && gate.reason) notices.push({ id: `combination-gate:${gate.combination_id}`, text: `${gate.combination_id}: ${COMBINATION_GATE_REASONS[gate.reason] ?? gate.reason}` });
    }
  }
  // D2 4.9.9 UI summary: per-case counts over the registered validated classes.
  if (route === "retained_preview_physics") {
    if (!retainedRowClasses(source)) notices.push({ id: "retained-precision-unvalidated", text: N_RP_UNVALIDATED });
    for (const c of classificationSummary(source)) {
      if (c.absolute_verified > 0) notices.push({ id: `retained-precision-absolute:${String(c.case_id)}`, text: `${String(c.case_id)}: ${c.absolute_verified} quantities verified only to an absolute bound, below the relative accuracy floor. Each is labelled and shown for inspection; rule checks cannot bind to them.` });
      if (c.not_covered > 0) notices.push({ id: `retained-precision-not-covered:${String(c.case_id)}`, text: `${String(c.case_id)}: ${c.not_covered} quantities uncovered: no verified accuracy for their quantity kind. Each is labelled and shown for inspection; rule checks cannot bind to them.` });
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

/** Row-level label for a listed row of a new kind; null for other rows. A
 * successor row also carries its validated class label (D2 4.9.9: never unlabelled). */
export function resultRowLabel(row: Pick<MechanicsResult["results"][number], "kind"> & Partial<Pick<MechanicsResult["results"][number], "id" | "unit">>, source: MechanicsResult | null | undefined): string | null {
  if (!source) return null;
  let route: ReturnType<typeof sourceContract>;
  try { route = sourceContract(source); } catch { return null; }
  if (route !== "preview_physics" && route !== "retained_preview_physics") return null;
  const kindLabel = row.kind === PREVIEW_INTENSIFIED_KIND ? N_INTENSIFIED : row.kind === PREVIEW_MAXIMUM_KIND ? N_HEADLINE : null;
  // Only a registered successor has validated classes; any other source has none.
  const classLabel = retainedRowClassLabel(row, source);
  return classLabel && kindLabel ? `${classLabel} ${kindLabel}` : classLabel ?? kindLabel;
}
