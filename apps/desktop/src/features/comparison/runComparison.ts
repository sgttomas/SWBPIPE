// Comparison of two analysis runs of one project (W-12 S3b; DEL-14-01 Design
// `desktop-states-runs-comparison.md` §4.2; FR-CMP-004, and FR-CMP-006 for
// runs). Deterministic and automatic: result rows are mapped when their whole
// identity is equal, with no manual mapping (FR-CMP-005/007 stay v0.2), and no
// tolerance profile or unit conversion (FR-CMP-008 stays v0.2). The reference
// behaviour is `core/comparison/analysis_run/engine.py` (`compare_analysis_runs`,
// `derive_exact_result_id_mappings`, `_settings_deltas`); the parity golden
// (`fixtures/comparison/analysis_run_comparison_golden.json`, written by
// `runComparisonParity.test.ts`) is checked against it by
// `tests/test_analysis_run_comparison_parity.py`.
//
// A mapped pair gets a `right − left` delta only when the conservative,
// whole-run standing gate holds: both runs read `integrity_checked`
// (`numericalResultStanding`, eligible), both carry the same result semantic
// identity (route and producer contract), and the two rows' units are equal.
// As the reference engine, a delta is computed only for its supported result
// families. Otherwise the pair is `not_compared`, with every reason that
// applies. Rows that do not map are listed per side.
//
// The result is a difference report. It establishes nothing about either run;
// human review is still required for any use of it (F-PIP-2, F-PIP-3).
import type { MechanicsResult } from "../../types";
import type { NumericalStanding } from "../results/resultStanding";
import { readerDispatch, readerRowSemantics } from "../results/resultReader";
import { codePointCompare } from "./stateComparison";

/** The reference engine's supported result families (`SUPPORTED_RESULT_FAMILIES`). */
export const SUPPORTED_RESULT_FAMILIES = ["displacement", "rotation", "force", "moment", "reaction", "stress", "ratio"] as const;

/** A received run whose result or snapshot, as the webview decoded them, does not hash
 * to the store's hashes: it holds values (such as integers beyond 2^53) the desktop
 * cannot represent exactly, so comparing it would hide differences. */
export const COMPARISON_RUN_NOT_EXACT = "COMPARISON-RUN-NOT-EXACT";
/** A received run whose record, result, manifest and state do not agree in the webview
 * as the store found them to agree. */
export const COMPARISON_RUN_RECORD_MISMATCH = "COMPARISON-RUN-RECORD-MISMATCH";

/** A comparison the runs do not admit, refused by name. */
export class RunComparisonRefusal extends Error {
  constructor(readonly code: string, detail: string) {
    super(`${code}: ${detail}`);
    this.name = "RunComparisonRefusal";
  }
}

/** A row's identity: the reference's `(result_id, family, object_ref, basis_ref,
 * dimension)` (`engine.py` `_stable_result_identity`) and the desktop's five row
 * metadata members. Rows map when every member is equal. */
export type RunRowIdentity = {
  result_id: string;
  /** The reader's family of a physical quantity, else null. */
  family: string | null;
  object_ref: string;
  basis_ref: { ref_type: string; ref_id: string } | null;
  /** The reader's source physical dimension (the record's `source_dimension`), else null. */
  dimension: string | null;
  component: string | null;
  location: string | null;
  basis: string | null;
  coordinate_system: string | null;
  sign_convention: string | null;
};

export const IDENTITY_FIELDS = [
  "result_id", "family", "object_ref", "basis_ref", "dimension", "component", "location", "basis", "coordinate_system", "sign_convention"
] as const satisfies ReadonlyArray<keyof RunRowIdentity>;

/** One result row as compared: its identity, its stored value and unit, and its row index. */
export type FlatRunRow = { identity: RunRowIdentity; value: unknown; unit: string; row_index: number };

/** The result semantic identity a run's rows are read under: the reader's route and the
 * route's pinned table binding (producer contract), else the producer's claim. */
export type ResultSemanticIdentity = { route: string; contract_id: string | null; contract_sha256: string | null };

/** What one side of a comparison contributes. */
export type RunComparisonInput = {
  rows: FlatRunRow[];
  standing: Pick<NumericalStanding, "eligible" | "status" | "contract" | "findings">;
  semantic_identity: ResultSemanticIdentity;
  /** The run's context as one flat map (`runContext`): solver, settings and load basis. */
  context: Record<string, unknown>;
  /** The run's solve diagnostics (its record's `diagnostics`). */
  diagnostics: RunDiagnostic[];
};

export type RunDiagnostic = { code: string; severity: string | null; affected_refs: string[]; message: string | null };

export const NOT_COMPARED_REASONS = [
  "semantic_identity_differs",
  "left_run_needs_recompute",
  "right_run_needs_recompute",
  "family_not_compared",
  "units_differ",
  "value_not_numeric"
] as const;
export type NotComparedReason = (typeof NOT_COMPARED_REASONS)[number];

export const UNMATCHED_REASONS = ["no_row_with_this_result_id", "identity_differs", "duplicate_result_id", "result_id_missing"] as const;
export type UnmatchedReason = (typeof UNMATCHED_REASONS)[number];

/** One mapped pair of rows: compared, with its delta in the rows' unit, or not, with its reasons. */
export type RunComparisonPair =
  | { status: "compared"; result_id: string; identity: RunRowIdentity; unit: string; left: number; right: number; delta: number }
  | { status: "not_compared"; result_id: string; identity: RunRowIdentity; left_unit: string; right_unit: string; left: unknown; right: unknown; reasons: NotComparedReason[] };

/** A row of one side that maps to no row of the other. */
export type UnmatchedRunRow = { side: "left" | "right"; result_id: string; identity: RunRowIdentity; unit: string; value: unknown; reason: UnmatchedReason; differing_fields: string[] };

/** A context difference, as the reference's `_settings_deltas` gives it. */
export type ContextDelta = { key: string; left: unknown; right: unknown; kind: "added" | "removed" | "changed" };

export type DiagnosticChange = RunDiagnostic & { change: "added" | "removed" };

export type RunComparisonSummary = {
  mapped_pairs: number;
  compared: number;
  compared_nonzero: number;
  not_compared: number;
  unmatched_left: number;
  unmatched_right: number;
  context_deltas: number;
  diagnostics_added: number;
  diagnostics_removed: number;
  diagnostics_common: number;
};

export type RunComparison = {
  summary: RunComparisonSummary;
  /** Mapped pairs in result-id order (code point). */
  pairs: RunComparisonPair[];
  /** The left side's unmatched rows, then the right side's, each in result-id and row order. */
  unmatched: UnmatchedRunRow[];
  context_deltas: ContextDelta[];
  diagnostics: DiagnosticChange[];
  /** Whole-run reasons that apply to every mapped pair (empty when the gate holds). */
  run_gate: NotComparedReason[];
};

function textOrNull(value: unknown): string | null {
  return typeof value === "string" ? value : null;
}

function basisRefOf(value: unknown): RunRowIdentity["basis_ref"] {
  if (!value || typeof value !== "object" || Array.isArray(value)) return null;
  const { ref_type, ref_id } = value as Record<string, unknown>;
  return typeof ref_type === "string" && typeof ref_id === "string" ? { ref_type, ref_id } : null;
}

/** The result's rows as compared: each row's identity read through the Rust reader's
 * row semantics of this result (its family and source dimension), with its metadata. A
 * row the reader cannot read has neither family nor dimension. */
export function flattenRunResult(result: MechanicsResult): FlatRunRow[] {
  const rows = Array.isArray(result.results) ? result.results : [];
  let semantics: ReturnType<typeof readerRowSemantics> | null = null;
  try { semantics = readerRowSemantics(result); } catch { semantics = null; }
  return rows.map((row, row_index) => {
    let family: string | null = null;
    let dimension: string | null = null;
    try {
      const signature = semantics?.(row).signature ?? null;
      family = signature?.category === "physical_quantity" ? signature.family ?? null : null;
      dimension = signature?.source_physical_semantic_dimension ?? null;
    } catch { /* A row the reader refuses: no family, no dimension. */ }
    const metadata = (row?.metadata && typeof row.metadata === "object" ? row.metadata : {}) as Record<string, unknown>;
    return {
      identity: {
        result_id: String(row?.id ?? ""),
        family,
        object_ref: String(row?.entity_ref ?? ""),
        basis_ref: basisRefOf(row?.basis_ref),
        dimension,
        component: textOrNull(metadata.component),
        location: textOrNull(metadata.location),
        basis: textOrNull(metadata.basis),
        coordinate_system: textOrNull(metadata.coordinate_system),
        sign_convention: textOrNull(metadata.sign_convention)
      },
      value: row?.value,
      unit: typeof row?.unit === "string" ? row.unit : "",
      row_index
    };
  });
}

/** The result's semantic identity: the reader's route and table binding. */
export function resultSemanticIdentity(result: MechanicsResult): ResultSemanticIdentity {
  const dispatch = readerDispatch(result);
  return {
    route: dispatch.route,
    contract_id: dispatch.binding?.id ?? result.producer?.semantic_contract_id ?? null,
    contract_sha256: dispatch.binding?.sha256 ?? null
  };
}

type ManifestLike = {
  solver_basis?: { solver_name?: unknown; solver_version?: unknown; solver_build_ref?: unknown; solver_mode?: unknown; settings?: unknown };
  load_basis?: { load_cases?: unknown; combinations?: unknown };
};

/** A run's context as one flat map, from its input manifest: the solver
 * (`solver_name`, `solver_version`, `solver_build_ref`, `solver_mode`), each solver
 * setting (`settings.<key>`), and each load case and combination by id
 * (`load_case:<id>`, `combination:<id>`, present as `true`), so the reference's
 * `_settings_deltas` over two such maps gives the solver, settings and load-basis
 * set differences. */
export function runContext(manifest: ManifestLike): Record<string, unknown> {
  const context: Record<string, unknown> = {};
  const solver = manifest.solver_basis ?? {};
  for (const key of ["solver_name", "solver_version", "solver_build_ref", "solver_mode"] as const) {
    if (solver[key] !== undefined) context[key] = solver[key];
  }
  const settings = solver.settings && typeof solver.settings === "object" && !Array.isArray(solver.settings) ? solver.settings as Record<string, unknown> : {};
  for (const [key, value] of Object.entries(settings)) context[`settings.${key}`] = value;
  const ids = (list: unknown) => (Array.isArray(list) ? list : []).map((item) => (item && typeof item === "object" ? (item as { id?: unknown }).id : undefined)).filter((id): id is string => typeof id === "string");
  for (const id of ids(manifest.load_basis?.load_cases)) context[`load_case:${id}`] = true;
  for (const id of ids(manifest.load_basis?.combinations)) context[`combination:${id}`] = true;
  return context;
}

/** A run's solve diagnostics: its record's diagnostics (the producer's, then the
 * rule-pack reference diagnostics), each as code, severity, affected refs and message. */
export function runDiagnostics(record: { analysis_run?: { diagnostics?: unknown } } | null | undefined): RunDiagnostic[] {
  const listed = Array.isArray(record?.analysis_run?.diagnostics) ? record!.analysis_run!.diagnostics as unknown[] : [];
  return listed.flatMap((item) => {
    const annotation = item && typeof item === "object" ? (item as { source_annotation?: unknown }).source_annotation : null;
    if (!annotation || typeof annotation !== "object") return [];
    const { code, severity, affected_refs, message } = annotation as Record<string, unknown>;
    if (typeof code !== "string") return [];
    return [{
      code,
      severity: textOrNull(severity),
      affected_refs: Array.isArray(affected_refs) ? affected_refs.filter((ref): ref is string => typeof ref === "string") : [],
      message: textOrNull(message)
    }];
  });
}

/** JSON text with keys in code-point order: equal for equal JSON data. */
function sortedText(value: unknown): string {
  if (value === undefined) return "undefined";
  if (value === null || typeof value !== "object") return JSON.stringify(value);
  if (Array.isArray(value)) return `[${value.map(sortedText).join(",")}]`;
  const keys = Object.keys(value as Record<string, unknown>).sort(codePointCompare);
  return `{${keys.map((key) => `${JSON.stringify(key)}:${sortedText((value as Record<string, unknown>)[key])}`).join(",")}}`;
}

/** The reference's `_settings_deltas`: every key of either map in code-point order,
 * `added` (right only), `removed` (left only) or `changed` (unequal values). */
export function contextDeltas(left: Record<string, unknown>, right: Record<string, unknown>): ContextDelta[] {
  const keys = [...new Set([...Object.keys(left), ...Object.keys(right)])].sort(codePointCompare);
  return keys.flatMap((key): ContextDelta[] => {
    const inLeft = Object.hasOwn(left, key);
    const inRight = Object.hasOwn(right, key);
    if (!inLeft) return [{ key, left: null, right: right[key], kind: "added" }];
    if (!inRight) return [{ key, left: left[key], right: null, kind: "removed" }];
    return sortedText(left[key]) === sortedText(right[key]) ? [] : [{ key, left: left[key], right: right[key], kind: "changed" }];
  });
}

/** Solve diagnostics added and removed, keyed by code and affected refs (as a multiset). */
export function diagnosticChanges(left: RunDiagnostic[], right: RunDiagnostic[]): { changes: DiagnosticChange[]; common: number } {
  const key = (diagnostic: RunDiagnostic) => JSON.stringify([diagnostic.code, diagnostic.affected_refs]);
  const remaining = new Map<string, RunDiagnostic[]>();
  for (const diagnostic of left) remaining.set(key(diagnostic), [...(remaining.get(key(diagnostic)) ?? []), diagnostic]);
  const added: DiagnosticChange[] = [];
  let common = 0;
  for (const diagnostic of right) {
    const matches = remaining.get(key(diagnostic));
    if (matches?.length) { matches.shift(); common += 1; }
    else added.push({ ...diagnostic, change: "added" });
  }
  const removed = [...remaining.values()].flat().map((diagnostic): DiagnosticChange => ({ ...diagnostic, change: "removed" }));
  const order = (a: DiagnosticChange, b: DiagnosticChange) => codePointCompare(a.code, b.code) || codePointCompare(JSON.stringify(a.affected_refs), JSON.stringify(b.affected_refs));
  return { changes: [...removed.sort(order), ...added.sort(order)], common };
}

function identityKey(identity: RunRowIdentity): string {
  return JSON.stringify(IDENTITY_FIELDS.map((field) => identity[field]));
}

function differingFields(left: RunRowIdentity, right: RunRowIdentity): string[] {
  return IDENTITY_FIELDS.filter((field) => JSON.stringify(left[field]) !== JSON.stringify(right[field]));
}

function sameSemanticIdentity(left: ResultSemanticIdentity, right: ResultSemanticIdentity): boolean {
  return left.route === right.route && left.contract_id === right.contract_id && left.contract_sha256 === right.contract_sha256;
}

function isFiniteNumber(value: unknown): value is number {
  return typeof value === "number" && Number.isFinite(value);
}

/** The whole-run reasons, which hold for every mapped pair: a different semantic
 * identity, and each run whose standing is not eligible. */
export function runGate(left: RunComparisonInput, right: RunComparisonInput): NotComparedReason[] {
  const reasons: NotComparedReason[] = [];
  if (!sameSemanticIdentity(left.semantic_identity, right.semantic_identity)) reasons.push("semantic_identity_differs");
  if (left.standing.eligible !== true) reasons.push("left_run_needs_recompute");
  if (right.standing.eligible !== true) reasons.push("right_run_needs_recompute");
  return reasons;
}

/** Compares two runs' rows (automatic mapping by whole identity; deltas under the
 * standing gate), their contexts and their solve diagnostics. Deterministic. */
export function compareRunInputs(left: RunComparisonInput, right: RunComparisonInput): RunComparison {
  const gate = runGate(left, right);
  const byId = (rows: FlatRunRow[]) => {
    const grouped = new Map<string, FlatRunRow[]>();
    for (const row of rows) grouped.set(row.identity.result_id, [...(grouped.get(row.identity.result_id) ?? []), row]);
    return grouped;
  };
  const leftById = byId(left.rows);
  const rightById = byId(right.rows);
  const pairs: RunComparisonPair[] = [];
  const unmatchedRow = (side: "left" | "right", row: FlatRunRow, reason: UnmatchedReason, differing: string[] = []): UnmatchedRunRow => ({
    side, result_id: row.identity.result_id, identity: row.identity, unit: row.unit, value: row.value, reason, differing_fields: differing
  });
  const ids = [...new Set([...leftById.keys(), ...rightById.keys()])].sort(codePointCompare);
  const leftOnly: UnmatchedRunRow[] = [];
  const rightOnly: UnmatchedRunRow[] = [];
  for (const id of ids) {
    const leftRows = leftById.get(id) ?? [];
    const rightRows = rightById.get(id) ?? [];
    // As the reference (`_unique_quantity_records`): a row without a result id, or
    // with an id repeated within either run, maps to nothing.
    if (!id || leftRows.length > 1 || rightRows.length > 1) {
      const reason: UnmatchedReason = id ? "duplicate_result_id" : "result_id_missing";
      for (const row of leftRows) leftOnly.push(unmatchedRow("left", row, reason));
      for (const row of rightRows) rightOnly.push(unmatchedRow("right", row, reason));
      continue;
    }
    const [l] = leftRows;
    const [r] = rightRows;
    if (!l) { rightOnly.push(unmatchedRow("right", r, "no_row_with_this_result_id")); continue; }
    if (!r) { leftOnly.push(unmatchedRow("left", l, "no_row_with_this_result_id")); continue; }
    if (identityKey(l.identity) !== identityKey(r.identity)) {
      const differing = differingFields(l.identity, r.identity);
      leftOnly.push(unmatchedRow("left", l, "identity_differs", differing));
      rightOnly.push(unmatchedRow("right", r, "identity_differs", differing));
      continue;
    }
    const reasons: NotComparedReason[] = [...gate];
    if (!l.identity.family || !(SUPPORTED_RESULT_FAMILIES as readonly string[]).includes(l.identity.family)) reasons.push("family_not_compared");
    if (!l.unit || l.unit !== r.unit) reasons.push("units_differ");
    if (!isFiniteNumber(l.value) || !isFiniteNumber(r.value)) reasons.push("value_not_numeric");
    if (reasons.length === 0) {
      const delta = (r.value as number) - (l.value as number);
      pairs.push({ status: "compared", result_id: id, identity: l.identity, unit: l.unit, left: l.value as number, right: r.value as number, delta });
    } else {
      pairs.push({ status: "not_compared", result_id: id, identity: l.identity, left_unit: l.unit, right_unit: r.unit, left: l.value, right: r.value, reasons });
    }
  }
  const unmatched = [...leftOnly, ...rightOnly];
  const context_deltas = contextDeltas(left.context, right.context);
  const { changes, common } = diagnosticChanges(left.diagnostics, right.diagnostics);
  const compared = pairs.filter((pair) => pair.status === "compared");
  return {
    summary: {
      mapped_pairs: pairs.length,
      compared: compared.length,
      compared_nonzero: compared.filter((pair) => pair.status === "compared" && pair.delta !== 0).length,
      not_compared: pairs.length - compared.length,
      unmatched_left: leftOnly.length,
      unmatched_right: rightOnly.length,
      context_deltas: context_deltas.length,
      diagnostics_added: changes.filter((change) => change.change === "added").length,
      diagnostics_removed: changes.filter((change) => change.change === "removed").length,
      diagnostics_common: common
    },
    pairs,
    unmatched,
    context_deltas,
    diagnostics: changes,
    run_gate: gate
  };
}
