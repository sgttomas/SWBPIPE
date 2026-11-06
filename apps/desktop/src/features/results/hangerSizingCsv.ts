// RT-R6 CSV adapters of the hanger-sizing tables (DEL-04-11 increment 3b; Design
// `hanger-sizing.md` §8.1; DEL-08-04 Design `csv-export.md` §2 and §3). Two tables, one adapter
// each, over the display model the panel renders (`hangerSizingView`): the candidates and the
// derived solves. They recompute nothing and convert nothing: every number is the published SI
// value 3a passes through (`SiQuantity.value`), written unrounded; every absent value is a status
// cell carrying the panel's own reason text, never a blank. The only value computed here is the
// preamble's `result_hash`, the document's checked canonical hash, the same function 3a uses.
//
// Layout of the candidates table: one row per candidate and operating case, in the document's
// candidate order and then its operating-case order. The case is a row identifier (`Operating
// case`) beside the support, so the column set is fixed and every numeric header states one unit.
// A candidate's case-independent values (W, F_cold, span, counts, ...) repeat on each of its rows.
import type { CsvExport } from "../result-export/CsvExportControl";
import { csvRecordedModelHash } from "../result-export/csvProvenance";
import {
  CSV_NOT_RECORDED,
  csvExportFileName,
  csvNumber,
  csvStatus,
  csvText,
  type CsvCell,
  type CsvColumn,
  type CsvProvenance
} from "../result-export/csvTable";
import { canonicalSha256HexCheckedV1 } from "../../services/hashService";
import type { HangerSizingSizedDocument } from "./hangerSizingResult";
import {
  F_COLD_CONDITION_LABEL,
  F_COLD_LABEL,
  type CandidateRowView,
  type FailureCountsView,
  type HangerSizingSizedView,
  type RecordNotShownReason,
  type SiQuantity,
  type SolveView,
  type VariationView
} from "./hangerSizingView";

export const HANGER_SIZING_CSV_KIND = "hanger-sizing";
/** The solves table's file-name qualifier: `swbpipe-hanger-sizing-solves.csv`. */
export const HANGER_SIZING_SOLVES_QUALIFIER = "solves";
export const HANGER_SIZING_CANDIDATES_FILTER = "all candidates";
export const HANGER_SIZING_SOLVES_FILTER = "all solves";
/** The document's own basis: SI, converted by nothing, rounded by nothing. */
export const HANGER_SIZING_CSV_UNIT_BASIS = "SI as published (forces N, lengths m, rates N/m, ratios and counts 1); unconverted, unrounded";

// The reasons shown in place of an absent value, shared by the panel and the CSV.
/** A `no_record_fits` candidate: nothing is put in place of a selection or its verification. */
export const HANGER_SIZING_NO_FIT = "no fit";
/** A family whose failure counts are null: it was not tested (not the same as zero). */
export const HANGER_SIZING_NOT_TESTED = "not tested";
export const HANGER_SIZING_NOT_APPLICABLE_CONSTANT = "not applicable: constant effort";
export const HANGER_SIZING_NOT_APPLICABLE_VARIABLE = "not applicable: variable spring";
export const HANGER_SIZING_NOT_APPLICABLE_SPRING = "not applicable: spring assumption";
export const HANGER_SIZING_NOT_APPLICABLE_FORCE = "not applicable: force assumption";
/** A per-case or count entry the document does not publish for this row. */
export const HANGER_SIZING_NOT_PUBLISHED = "not published";
export const HANGER_SIZING_NO_NONLINEAR_SUPPORT = "no nonlinear support";
export const HANGER_SIZING_NO_FAILED_TESTS = "none";
/** A record detail the library gate withholds, by the gate's reason. */
export const hangerSizingNotShown = (reason: RecordNotShownReason): string => `not shown: ${reason}`;

export const HANGER_SIZING_VARIABLE_COUNT_KEYS = ["E1", "E2", "V0", "L", "T", "V"] as const;
export const HANGER_SIZING_CONSTANT_COUNT_KEYS = ["E1", "L", "T"] as const;

type Context = Readonly<{ view: HangerSizingSizedView; provenance: CsvProvenance; softwareVersion: string }>;

/** The preamble's provenance: `model_hash` as the caller records it (`not_recorded` when none is
 * supplied); `result_hash`, `sha256:` and the document's checked canonical hash (`not_recorded`
 * when the document cannot be hashed); `analysis_route`, its `semantic_contract_id`. */
export async function hangerSizingCsvProvenance(input: { document: HangerSizingSizedDocument; modelHash?: string | null }): Promise<CsvProvenance> {
  let resultHash = CSV_NOT_RECORDED;
  try {
    resultHash = `sha256:${await canonicalSha256HexCheckedV1(input.document)}`;
  } catch {
    resultHash = CSV_NOT_RECORDED;
  }
  return {
    modelHash: input.modelHash === undefined || input.modelHash === null ? CSV_NOT_RECORDED : csvRecordedModelHash(input.modelHash),
    resultHash,
    analysisRoutes: [input.document.semantic_contract_id]
  };
}

// ---------------------------------------------------------------------------------------------
// Cells

const quantity = (value: SiQuantity): CsvCell => csvNumber(value.value);
const variationCell = (value: VariationView): CsvCell => (value.state === "ratio" ? csvNumber(value.ratio.value) : csvStatus(value.text));
const countCell = (counts: FailureCountsView, key: string): CsvCell => {
  if (counts.state === "not_tested") return csvStatus(HANGER_SIZING_NOT_TESTED);
  const count = counts.counts[key];
  return typeof count === "number" ? csvNumber(count) : csvStatus(HANGER_SIZING_NOT_PUBLISHED);
};

/** One candidate row for one operating case (`caseId` null only if a candidate lists no case). */
type Line = Readonly<{ row: CandidateRowView; caseId: string | null }>;

type Spec = Readonly<{ column: CsvColumn; cell: (line: Line) => CsvCell }>;
const text = (label: string, cell: Spec["cell"]): Spec => ({ column: { kind: "text", label }, cell });
const number = (label: string, unit: string, cell: Spec["cell"]): Spec => ({ column: { kind: "number", label, unit }, cell });

/** A value of the selection: by family, with the absent reason of the other family and of no fit. */
function bySelection(
  line: Line,
  variable: (selection: Extract<NonNullable<CandidateRowView["selection"]>, { family: "variable_spring" }>) => CsvCell,
  constant: (selection: Extract<NonNullable<CandidateRowView["selection"]>, { family: "constant_effort" }>) => CsvCell
): CsvCell {
  const selection = line.row.selection;
  if (selection === null) return csvStatus(HANGER_SIZING_NO_FIT);
  return selection.family === "variable_spring" ? variable(selection) : constant(selection);
}

function byVerification(
  line: Line,
  variable: (verification: Extract<NonNullable<CandidateRowView["verification"]>, { family: "variable_spring" }>) => CsvCell,
  constant: (verification: Extract<NonNullable<CandidateRowView["verification"]>, { family: "constant_effort" }>) => CsvCell
): CsvCell {
  const verification = line.row.verification;
  if (verification === null) return csvStatus(HANGER_SIZING_NO_FIT);
  return verification.family === "variable_spring" ? variable(verification) : constant(verification);
}

const notPublished = () => csvStatus(HANGER_SIZING_NOT_PUBLISHED);
const naConstant = () => csvStatus(HANGER_SIZING_NOT_APPLICABLE_CONSTANT);
const naVariable = () => csvStatus(HANGER_SIZING_NOT_APPLICABLE_VARIABLE);

/** The entry of a per-case list for this row's case, matched by case id. */
function forCase<T extends { case: string }>(entries: readonly T[], line: Line): T | undefined {
  return line.caseId === null ? undefined : entries.find((entry) => entry.case === line.caseId);
}

function recordCell(line: Line, pick: (record: Extract<NonNullable<CandidateRowView["record"]>, { state: "shown" }>) => string): CsvCell {
  const record = line.row.record;
  if (record === null) return csvStatus(HANGER_SIZING_NO_FIT);
  return record.state === "shown" ? csvText(pick(record)) : csvStatus(hangerSizingNotShown(record.reason));
}

// ---------------------------------------------------------------------------------------------
// The candidates table

const CANDIDATE_SPECS: readonly Spec[] = [
  text("Support", (line) => csvText(line.row.support)),
  text("Node", (line) => csvText(line.row.node)),
  text("DOF", (line) => csvText(line.row.dof)),
  text("Family policy", (line) => csvText(line.row.familyPolicy)),
  text("Outcome", (line) => csvText(line.row.outcome)),
  text("Operating case", (line) => (line.caseId === null ? notPublished() : csvText(line.caseId))),
  text("Selected family", (line) => bySelection(line, (s) => csvText(s.family), (s) => csvText(s.family))),
  text("Hanger id", (line) => bySelection(line, (s) => csvText(s.hangerId), (s) => csvText(s.hangerId))),
  text("Record sha256", (line) => bySelection(line, (s) => csvText(s.recordSha256), (s) => csvText(s.recordSha256))),
  text("Record name", (line) => recordCell(line, (record) => record.name)),
  text("Record type", (line) => recordCell(line, (record) => record.hangerType)),
  number("W", "N", (line) => quantity(line.row.w)),
  number(`F_cold: ${F_COLD_LABEL}`, "N", (line) => bySelection(line, (s) => quantity(s.fCold.quantity), naConstant)),
  number(`F_C: ${F_COLD_CONDITION_LABEL}`, "N", (line) => bySelection(line, (s) => quantity(s.fColdCondition.quantity), naConstant)),
  number("k", "N/m", (line) => bySelection(line, (s) => quantity(s.k), naConstant)),
  number("Envelope low", "N", (line) => bySelection(line, (s) => quantity(s.envelope[0]), naConstant)),
  number("Envelope high", "N", (line) => bySelection(line, (s) => quantity(s.envelope[1]), naConstant)),
  number("Constant load", "N", (line) => bySelection(line, naVariable, (s) => quantity(s.load))),
  number("u_C", "m", (line) => quantity(line.row.uCold)),
  number("u_i", "m", (line) => {
    const entry = forCase(line.row.operating, line);
    return entry ? quantity(entry.position) : notPublished();
  }),
  number("F_i", "N", (line) => bySelection(line, (s) => {
    const entry = forCase(s.operating, line);
    return entry ? quantity(entry.force) : notPublished();
  }, naConstant)),
  number("v_i", "1", (line) => bySelection(line, (s) => {
    const entry = forCase(s.operating, line);
    return entry ? variationCell(entry.variation) : notPublished();
  }, naConstant)),
  number("Span min", "m", (line) => quantity(line.row.span[0])),
  number("Span max", "m", (line) => quantity(line.row.span[1])),
  number("Required travel", "m", (line) => quantity(line.row.requiredTravel)),
  text("Governing travel case", (line) => (line.row.governing === null ? csvStatus(HANGER_SIZING_NO_FIT) : csvText(line.row.governing.travelCase))),
  text("Governing variation case", (line) => {
    const governing = line.row.governing;
    if (governing === null) return csvStatus(HANGER_SIZING_NO_FIT);
    return governing.variationCase === null ? naConstant() : csvText(governing.variationCase);
  }),
  text("Verification assumption", (line) => csvText(line.row.verificationAssumption.role)),
  number("Assumed k", "N/m", (line) => {
    const assumption = line.row.verificationAssumption;
    return assumption.role === "spring" ? quantity(assumption.k) : csvStatus(HANGER_SIZING_NOT_APPLICABLE_FORCE);
  }),
  number(`Assumed F_cold: ${F_COLD_LABEL}`, "N", (line) => {
    const assumption = line.row.verificationAssumption;
    return assumption.role === "spring" ? quantity(assumption.fCold.quantity) : csvStatus(HANGER_SIZING_NOT_APPLICABLE_FORCE);
  }),
  number("Assumed force", "N", (line) => {
    const assumption = line.row.verificationAssumption;
    return assumption.role === "force" ? quantity(assumption.force) : csvStatus(HANGER_SIZING_NOT_APPLICABLE_SPRING);
  }),
  number("Verified u_C", "m", (line) => byVerification(line, (v) => quantity(v.uCold), (v) => quantity(v.uCold))),
  number("Verified u_i", "m", (line) => byVerification(line, (v) => {
    const entry = forCase(v.operating, line);
    return entry ? quantity(entry.position) : notPublished();
  }, (v) => {
    const entry = forCase(v.operating, line);
    return entry ? quantity(entry.position) : notPublished();
  })),
  number("Verified F_i", "N", (line) => byVerification(line, (v) => {
    const entry = forCase(v.operating, line);
    return entry ? quantity(entry.force) : notPublished();
  }, naConstant)),
  number("Verified v_i", "1", (line) => byVerification(line, (v) => {
    const entry = forCase(v.operating, line);
    return entry ? variationCell(entry.variation) : notPublished();
  }, naConstant)),
  number("Verified span min", "m", (line) => byVerification(line, (v) => quantity(v.span[0]), (v) => quantity(v.span[0]))),
  number("Verified span max", "m", (line) => byVerification(line, (v) => quantity(v.span[1]), (v) => quantity(v.span[1]))),
  number("Verified required travel", "m", (line) => byVerification(line, (v) => quantity(v.requiredTravel), (v) => quantity(v.requiredTravel))),
  text("Verified T pass", (line) => byVerification(line, (v) => csvText(String(v.tPass)), (v) => csvText(String(v.tPass)))),
  number(`Verified F_C: ${F_COLD_CONDITION_LABEL}`, "N", (line) => byVerification(line, (v) => quantity(v.fColdCondition.quantity), naConstant)),
  number("Departure", "N", (line) => byVerification(line, (v) => quantity(v.departure), naConstant)),
  text("Failed tests", (line) => {
    const failed = (tests: readonly string[]) => csvText(tests.length > 0 ? tests.join(" ") : HANGER_SIZING_NO_FAILED_TESTS);
    return byVerification(line, (v) => failed(v.failedTests), (v) => failed(v.failedTests));
  }),
  ...HANGER_SIZING_VARIABLE_COUNT_KEYS.map((key) => number(`Variable records failing ${key}`, "1", (line) => countCell(line.row.failureCounts.variable, key))),
  ...HANGER_SIZING_CONSTANT_COUNT_KEYS.map((key) => number(`Constant records failing ${key}`, "1", (line) => countCell(line.row.failureCounts.constant, key)))
];

function candidateLines(view: HangerSizingSizedView): Line[] {
  return view.rows.flatMap<Line>((row) => (row.operating.length > 0
    ? row.operating.map((entry) => ({ row, caseId: entry.case }))
    : [{ row, caseId: null }]));
}

/** The candidates table: one row per candidate and operating case. */
export function hangerSizingCandidatesCsv(input: Context): CsvExport {
  const lines = candidateLines(input.view);
  return {
    table: {
      preamble: {
        table: HANGER_SIZING_CSV_KIND,
        softwareVersion: input.softwareVersion,
        ...input.provenance,
        caseOrFilter: HANGER_SIZING_CANDIDATES_FILTER,
        unitBasis: HANGER_SIZING_CSV_UNIT_BASIS
      },
      columns: CANDIDATE_SPECS.map((spec) => spec.column),
      rows: lines.map((line) => CANDIDATE_SPECS.map((spec) => spec.cell(line)))
    },
    fileName: csvExportFileName(HANGER_SIZING_CSV_KIND)
  };
}

// ---------------------------------------------------------------------------------------------
// The solves table

type SolveLine = Readonly<{ solve: SolveView; state: SolveView["states"][number] | null }>;

const SOLVE_COLUMNS: readonly CsvColumn[] = [
  { kind: "text", label: "Solve" },
  { kind: "text", label: "Case" },
  { kind: "text", label: "Derived document sha256" },
  { kind: "text", label: "Nonlinear support" },
  { kind: "text", label: "State" }
];

/** The solves table: one row per solve and nonlinear support, in the document's solve order and
 * key order; a solve of a model with no nonlinear support is one row stating so. */
export function hangerSizingSolvesCsv(input: Context): CsvExport {
  const lines: SolveLine[] = input.view.solves.flatMap<SolveLine>((solve) => (solve.states.length > 0
    ? solve.states.map((state) => ({ solve, state }))
    : [{ solve, state: null }]));
  return {
    table: {
      preamble: {
        table: HANGER_SIZING_CSV_KIND,
        softwareVersion: input.softwareVersion,
        ...input.provenance,
        caseOrFilter: HANGER_SIZING_SOLVES_FILTER,
        unitBasis: HANGER_SIZING_CSV_UNIT_BASIS
      },
      columns: SOLVE_COLUMNS,
      rows: lines.map(({ solve, state }) => [
        csvText(solve.label),
        csvText(solve.case),
        csvText(solve.derivedDocumentSha256),
        state === null ? csvStatus(HANGER_SIZING_NO_NONLINEAR_SUPPORT) : csvText(state.support),
        state === null ? csvStatus(HANGER_SIZING_NO_NONLINEAR_SUPPORT) : csvText(state.state)
      ])
    },
    fileName: csvExportFileName(HANGER_SIZING_CSV_KIND, HANGER_SIZING_SOLVES_QUALIFIER)
  };
}
