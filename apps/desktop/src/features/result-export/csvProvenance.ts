// RT-R6 (DEL-08-04-R14; Design `csv-export.md` §3): the preamble's provenance values, from the
// sources the desktop already records. Nothing is invented: a result without a recorded value
// states `not_recorded`, and a table without a result states `not_applicable`.
import desktopAppConfig from "../../../src-tauri/tauri.conf.json";
import { canonicalSha256Hex } from "../../services/hashService";
import type { AnalysisRunEnvelope, MechanicsResult, PreviewModel } from "../../types";
import { analysisResultHashScope } from "../results/analysisResultHashScope";
import { CSV_NOT_APPLICABLE, CSV_NOT_RECORDED, isCsvHashValue, type CsvProvenance } from "./csvTable";

/** The application version, as the report states it (`renderableReportInput.ts`). */
export const CSV_SOFTWARE_VERSION: string = desktopAppConfig.version;

/** The unit basis of a table under the report's display rule. */
export const CSV_UNIT_BASIS_DISPLAY_RULE = "stored units; Pa shown as MPa (report display rule), unrounded";
/** The unit basis of a view that converts nothing. */
export const CSV_UNIT_BASIS_STORED = "stored units, unconverted";

/** `<basis>; project units <dimension>=<unit>, ...` in key order. */
export function csvUnitBasis(model: PreviewModel | null, basis: string): string {
  const units = model?.project?.units ?? {};
  const listed = Object.keys(units).sort().map((key) => `${key}=${units[key]}`).join(", ");
  return listed ? `${basis}; project units ${listed}` : basis;
}

/** `sha256:<hex>` of a hex digest, or null when it is not one. */
function sha256(value: unknown): string | null {
  if (typeof value !== "string") return null;
  const hash = value.startsWith("sha256:") ? value : `sha256:${value}`;
  return isCsvHashValue(hash) ? hash : null;
}

/** The model hash as the report computes it: the canonical SHA-256 of the model. */
export async function csvModelHash(model: PreviewModel): Promise<string> {
  return sha256(await canonicalSha256Hex(model)) ?? CSV_NOT_RECORDED;
}

/** A recorded model hash (`sha256:<hex>`, for example a saved state's), or `not_recorded`. */
export function csvRecordedModelHash(value: unknown): string {
  return sha256(value) ?? CSV_NOT_RECORDED;
}

/** The received-result hash of the result's own analysis-run record (`analysisResultHashScope`),
 * `not_recorded` when the result has none, `not_applicable` when there is no result. */
export function csvResultHash(result: MechanicsResult | null, analysisRun: AnalysisRunEnvelope | null | undefined): string {
  if (!result) return CSV_NOT_APPLICABLE;
  const run = analysisRun?.analysis_run;
  if (!run || run.run_id !== result.run_id) return CSV_NOT_RECORDED;
  const scope = analysisResultHashScope(analysisRun!.schema_version);
  const entry = scope === null ? undefined : run.hashes.find((item) => item.payload_scope === scope);
  return (entry && entry.algorithm === "sha256" ? sha256(entry.value) : null) ?? CSV_NOT_RECORDED;
}

/** The analysis route: the result's `producer.semantic_contract_id`, with the same tokens. */
export function csvAnalysisRoute(result: MechanicsResult | null): string {
  if (!result) return CSV_NOT_APPLICABLE;
  const id = result.producer?.semantic_contract_id;
  return typeof id === "string" && id.trim() !== "" && !/\s/.test(id) ? id : CSV_NOT_RECORDED;
}

/** One side's provenance for a results table: the model it renders with, its result and that
 * result's analysis-run record. */
export async function csvResultProvenance(input: {
  model: PreviewModel;
  result: MechanicsResult | null;
  analysisRun?: AnalysisRunEnvelope | null;
}): Promise<CsvProvenance> {
  return {
    modelHash: await csvModelHash(input.model),
    resultHash: csvResultHash(input.result, input.analysisRun),
    analysisRoutes: [csvAnalysisRoute(input.result)]
  };
}

/** A model-only table (weight, centre of gravity): no result, so no result hash or route. */
export function csvModelOnlyProvenance(modelHash: string): CsvProvenance {
  return { modelHash, resultHash: CSV_NOT_APPLICABLE, analysisRoutes: [CSV_NOT_APPLICABLE] };
}
