// RT-R6 CSV adapter of the run comparison table (DEL-08-04-R14; Design `csv-export.md` §2). The
// view's rows (`runComparisonEntries`) and filter (`matchesRunComparisonFilters`) live here, so
// the view and the file list the same rows: every page of the rows the filters match, in the
// view's order. Values and deltas are written as stored, in the rows' own units, as the view
// shows them (nothing is converted). Two-sided: side `a` is the view's Left run and `b` its
// Right run.
import type { PreviewModel } from "../../types";
import type { CsvExport } from "../result-export/CsvExportControl";
import { csvModelHash } from "../result-export/csvProvenance";
import { CSV_NOT_RECORDED, csvExportFileName, type CsvProvenance } from "../result-export/csvTable";
import { CSV_DIMENSIONLESS_UNIT, joinColumnGroups, quantityColumns, storedValueCell, textColumn, type CsvQuantityEntry } from "../result-export/csvValues";
import {
  SUPPORTED_RESULT_FAMILIES,
  type NotComparedReason,
  type RunComparison,
  type RunComparisonPair,
  type RunRowIdentity,
  type UnmatchedReason,
  type UnmatchedRunRow
} from "./runComparison";
import { storedValueFullText } from "./stateComparisonCsv";

export const RUN_COMPARISON_CSV_KIND = "run-comparison";

export const NOT_COMPARED_LABELS: Record<NotComparedReason, string> = {
  semantic_identity_differs: "the runs' results have different semantic identities (route or producer contract)",
  left_run_needs_recompute: "the left run reads needs_recompute",
  right_run_needs_recompute: "the right run reads needs_recompute",
  family_not_compared: `no delta for this result family (deltas are computed for ${SUPPORTED_RESULT_FAMILIES.join(", ")} results, as in the reference comparison)`,
  units_differ: "the units differ (nothing is converted)",
  value_not_numeric: "a value is not a finite number"
};

export const UNMATCHED_LABELS: Record<UnmatchedReason, string> = {
  no_row_with_this_result_id: "the other run has no row with this result id",
  identity_differs: "the other run's row with this result id differs in",
  duplicate_result_id: "this result id repeats within a run, so it is not mapped",
  result_id_missing: "the row has no result id"
};

export type RunComparisonStatus = "compared" | "not_compared" | "left_only" | "right_only";
export type RunComparisonStatusFilter = "all" | "changed" | RunComparisonStatus;
export type RunComparisonFilters = Readonly<{ status: RunComparisonStatusFilter; family: string; idFilter: string }>;

/** One row of the view: a mapped pair or an unmatched row, with its source. */
export type RunComparisonEntry = Readonly<{
  key: string;
  status: RunComparisonStatus;
  nonzero: boolean;
  identity: RunRowIdentity;
  source: Readonly<{ kind: "pair"; pair: RunComparisonPair }> | Readonly<{ kind: "unmatched"; row: UnmatchedRunRow }>;
}>;

export function unmatchedReasonText(row: UnmatchedRunRow): string {
  return row.reason === "identity_differs" ? `${UNMATCHED_LABELS.identity_differs} ${row.differing_fields.join(", ")}` : UNMATCHED_LABELS[row.reason];
}

export function notComparedText(pair: Extract<RunComparisonPair, { status: "not_compared" }>): string {
  return `Not compared: ${pair.reasons.map((reason) => NOT_COMPARED_LABELS[reason]).join("; ")}.`;
}

/** The view's rows: the mapped pairs, then the unmatched rows. */
export function runComparisonEntries(comparison: RunComparison): RunComparisonEntry[] {
  return [
    ...comparison.pairs.map((pair): RunComparisonEntry => ({
      key: `pair:${pair.result_id}`, status: pair.status, nonzero: pair.status === "compared" && pair.delta !== 0, identity: pair.identity, source: { kind: "pair", pair }
    })),
    ...comparison.unmatched.map((row, index): RunComparisonEntry => ({
      key: `${row.side}:${index}`, status: row.side === "left" ? "left_only" : "right_only", nonzero: false, identity: row.identity, source: { kind: "unmatched", row }
    }))
  ];
}

export function matchesRunComparisonFilters(entry: RunComparisonEntry, filters: RunComparisonFilters): boolean {
  const { status, family, idFilter } = filters;
  return (status === "all" || (status === "changed" ? entry.status === "compared" && entry.nonzero : entry.status === status))
    && (family === "all" || (entry.identity.family ?? "other") === family)
    && (!idFilter.trim() || entry.identity.result_id.includes(idFilter.trim()));
}

export function basisText(identity: RunRowIdentity): string {
  return identity.basis_ref ? `${identity.basis_ref.ref_type} ${identity.basis_ref.ref_id}` : "(none)";
}

/** What a side states for its CSV preamble: its recorded model hash, or the model it was
 * solved on to hash; its result hash; and its analysis route (`producer.semantic_contract_id`). */
export type RunSideCsvProvenance = Readonly<{ modelHash: string | null; model: PreviewModel | null; resultHash: string; analysisRoute: string }>;

export async function runSideProvenance(side: RunSideCsvProvenance | undefined): Promise<CsvProvenance> {
  if (!side) return { modelHash: CSV_NOT_RECORDED, resultHash: CSV_NOT_RECORDED, analysisRoutes: [CSV_NOT_RECORDED] };
  return {
    modelHash: side.modelHash ?? (side.model ? await csvModelHash(side.model) : CSV_NOT_RECORDED),
    resultHash: side.resultHash,
    analysisRoutes: [side.analysisRoute]
  };
}

export const RUN_VALUE_NOT_IN_RUN: Readonly<Record<"left" | "right", string>> = { left: "not in the left run", right: "not in the right run" };
export const RUN_DELTA_UNMATCHED = "unmatched: no delta";
export const RUN_DELTA_NOT_COMPARED = "not compared: no delta";

const storedUnit = (unit: string) => (unit === "" ? CSV_DIMENSIONLESS_UNIT : unit);

function valueEntry(value: unknown, unit: string): CsvQuantityEntry {
  return typeof value === "number" && Number.isFinite(value) ? { value, unit } : { absent: storedValueFullText(value) };
}

function sideValue(entry: RunComparisonEntry, name: "left" | "right"): CsvQuantityEntry {
  const source = entry.source;
  if (source.kind === "pair") {
    const pair = source.pair;
    return pair.status === "compared" ? valueEntry(pair[name], pair.unit) : valueEntry(pair[name], name === "left" ? pair.left_unit : pair.right_unit);
  }
  return source.row.side === name ? valueEntry(source.row.value, source.row.unit) : { absent: RUN_VALUE_NOT_IN_RUN[name] };
}

function outcomeText(entry: RunComparisonEntry): string {
  const source = entry.source;
  if (source.kind === "unmatched") return `Unmatched: ${unmatchedReasonText(source.row)}.`;
  return source.pair.status === "compared" ? "Compared" : notComparedText(source.pair);
}

export function runComparisonCsv(input: {
  leftId: string;
  rightId: string;
  leftDescription: string;
  rightDescription: string;
  sides: Readonly<{ a: CsvProvenance; b: CsvProvenance }>;
  /** The rows the view's filters match, in the view's order. */
  entries: readonly RunComparisonEntry[];
  filters: RunComparisonFilters;
  softwareVersion: string;
  unitBasis: string;
}): CsvExport {
  const entries = input.entries;
  const stored = (value: number) => storedValueCell(value);
  const { columns, rows } = joinColumnGroups(entries.length, [
    textColumn("Row key", entries.map((entry) => entry.key)),
    textColumn("Status", entries.map((entry) => entry.status)),
    textColumn("Result id", entries.map((entry) => entry.identity.result_id)),
    textColumn("Family", entries.map((entry) => entry.identity.family ?? "other")),
    textColumn("Entity", entries.map((entry) => entry.identity.object_ref)),
    textColumn("Basis", entries.map((entry) => basisText(entry.identity))),
    textColumn("Component", entries.map((entry) => entry.identity.component ?? "")),
    textColumn("Location", entries.map((entry) => entry.identity.location ?? "")),
    quantityColumns("Left", entries.map((entry) => sideValue(entry, "left")), stored, storedUnit),
    quantityColumns("Right", entries.map((entry) => sideValue(entry, "right")), stored, storedUnit),
    quantityColumns("Delta (right - left)", entries.map((entry): CsvQuantityEntry => {
      const source = entry.source;
      if (source.kind === "unmatched") return { absent: RUN_DELTA_UNMATCHED };
      return source.pair.status === "compared" ? { value: source.pair.delta, unit: source.pair.unit } : { absent: RUN_DELTA_NOT_COMPARED };
    }), stored, storedUnit),
    textColumn("Outcome", entries.map(outcomeText))
  ]);
  const { filters } = input;
  return {
    table: {
      preamble: {
        table: RUN_COMPARISON_CSV_KIND,
        softwareVersion: input.softwareVersion,
        sides: input.sides,
        caseOrFilter: `a = left ${input.leftDescription}; b = right ${input.rightDescription}; status: ${filters.status}; family: ${filters.family}; result id contains: ${filters.idFilter.trim() || "any"}`,
        unitBasis: input.unitBasis
      },
      columns, rows
    },
    fileName: csvExportFileName(RUN_COMPARISON_CSV_KIND, `${input.leftId} ${input.rightId}`)
  };
}
