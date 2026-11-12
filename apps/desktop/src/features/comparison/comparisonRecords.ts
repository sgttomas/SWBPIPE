// A computed comparison of two saved states or two runs restated as a record a report
// section set reads (DEL-08-06 REQ-011, REQ-015; the record forms the DEL-08-06 assembler
// `core/reporting/state_comparison_handoff_sections/engine.py` reads, after DEL-14-03 and
// DEL-14-04). The record is the complete comparison, as the desktop computed it, adding
// nothing:
// - each side by identifier and hash (a saved state's model hash; a recorded run's record
//   and result hashes; the session's current run stated as the current run, not recorded);
// - for states, every added, removed and changed entity, with each changed field's path,
//   change kind, both stored values and their units, and beside a changed entity its
//   origin reference (the entity's `provenance` string) where it has one. A field of the
//   entity's provenance record, which holds a library record's own content, is stated by
//   its path and the SHA-256 of each side's canonical value, never by value (DEL-08-06
//   REQ-009);
// - for runs, every compared pair as a delta in its common unit with its dimension, every
//   pair not compared with its reasons and no number, every unmatched row with its reason
//   and no value, the run gate, the context differences as settings deltas, and the
//   solve diagnostics added and removed;
// - the comparison table's CSV: its kind and the row count of the unfiltered export.
// The desktop has no tolerance profile: a delta is stated `not_tolerance_checked`, never a
// classification that implies one. Identifiers, hashes and stored values only: no state or
// run name, note or tag, and no project name. The record's `hashes` member names the
// SHA-256 of the record's RFC 8785 form without that member; the CSV's preamble states the
// same value. The record is formed when it is needed (an inclusion, a CSV export) and the
// session keeps only the bounded section the assembler forms from it.
//
// The record is a difference report. It establishes nothing about either side; human
// review is still required for any use of it.
import { canonicalSha256Hex } from "../../services/hashService";
import type { CurrentRunIdentity, RunComparisonResult, StateComparisonResult } from "../history/useModelStateHistory";
import { comparisonSections, type BoundComparisons } from "../report/comparisonSections";
import type { RunComparisonSide } from "./runComparisonSides";
import type { RunRowIdentity } from "./runComparison";
import { RUN_COMPARISON_CSV_KIND, runComparisonEntries } from "./runComparisonCsv";
import { flattenModelDocument, pathText, type StateComparison } from "./stateComparison";
import { STATE_COMPARISON_CSV_KIND } from "./stateComparisonCsv";

type JsonRecord = Record<string, unknown>;

/** The two comparison kinds a session includes in a report. */
export type ReportComparisonKind = "state" | "run";

export const COMPARISON_RECORD_PROFESSIONAL_BOUNDARY = {
  human_review_required: true,
  software_makes_compliance_claim: false,
  software_makes_certification_claim: false,
  software_makes_sealing_claim: false,
  software_makes_approval_claim: false,
  software_makes_authentication_claim: false,
  software_creates_professional_reliance_record: false,
  software_creates_external_validation_record: false
};

/** The record's provenance: the desktop session, private, review pending. No project name. */
export const COMPARISON_RECORD_PROVENANCE = {
  source_name: "OpenPipeStress desktop session comparison",
  source_location: "local desktop session",
  source_license: "user_supplied_or_private",
  contributor: "user_local_session",
  contributor_attestation: "not_asserted",
  redistribution_status: "private_only",
  review_status: "pending",
  privacy_classification: "private_project_data"
};

/** What the desktop comparison applies, stated as the comparison's settings. */
const STATE_SETTINGS = {
  mapping_basis: "stable_id_within_family",
  manual_mappings: "none",
  unit_conversion: "none",
  tolerance: "not_tolerance_checked"
};
const RUN_TOLERANCE = "not_tolerance_checked";

/** The first field-path segment of a member that holds a library record's own content. */
export const RECORD_CONTENT_MEMBER = "provenance_record";

/** A deterministic comparison identifier from the two side identifiers, left first. */
export function comparisonId(kind: ReportComparisonKind, leftId: string, rightId: string): string {
  return `${kind === "state" ? "model-state-comparison" : "analysis-run-comparison"}:${leftId}:${rightId}`;
}

/** The origin reference of each entity that differs: its `provenance` string on each side,
 * by `JSON.stringify([family, ...key])`, read from the two compared documents. */
export type ComparisonOrigins = Record<string, { left?: string; right?: string }>;

export function entityKey(family: string, key: readonly string[]): string {
  return JSON.stringify([family, ...key]);
}

export function comparisonOrigins(leftModel: unknown, rightModel: unknown, comparison: StateComparison): ComparisonOrigins {
  const differing = new Set(comparison.rows.filter((row) => row.classification !== "unchanged").map((row) => entityKey(row.family, row.key)));
  const origins: ComparisonOrigins = {};
  for (const [side, model] of [["left", leftModel], ["right", rightModel]] as const) {
    for (const entity of flattenModelDocument(model, side)) {
      const key = entityKey(entity.family, entity.key);
      const provenance = entity.fields.provenance;
      if (!differing.has(key) || typeof provenance !== "string") continue;
      (origins[key] ??= {})[side] = provenance;
    }
  }
  return origins;
}

/** The unfiltered saved-state CSV's row count: a line per field change, or one for an entity without one. */
export function stateCsvRowCount(comparison: StateComparison): number {
  return comparison.rows.reduce((count, row) => count + Math.max(1, row.changes.length), 0);
}

/** The saved-state comparison as a record (without its `hashes` member). */
export async function stateComparisonRecord(result: StateComparisonResult): Promise<JsonRecord> {
  const { left, right, comparison } = result;
  const origins = result.origins ?? {};
  const stateRef = (state: StateComparisonResult["left"]) => ({ object_type: "ModelState", ref: state.state_id, model_hash: state.model_hash });
  const side = async (value: unknown, name: "left" | "right", contentMember: boolean): Promise<JsonRecord> => {
    if (value === undefined) return {};
    return contentMember ? { [`${name}_sha256`]: await canonicalSha256Hex(value) } : { [name]: value };
  };
  const entities = await Promise.all(comparison.rows
    .filter((row) => row.classification !== "unchanged")
    .map(async (row) => {
      const ref = { object_type: row.family, ref: row.id };
      const origin = origins[entityKey(row.family, row.key)] ?? {};
      return {
        row_id: `row:${row.family}:${row.key.join("/")}`,
        classification: row.classification,
        match_basis: row.classification === "changed" ? "stable_id" : "unmatched",
        mapping_id: null,
        left_ref: row.classification === "added" ? null : ref,
        right_ref: row.classification === "removed" ? null : ref,
        ...(origin.left !== undefined ? { left_origin_ref: origin.left } : {}),
        ...(origin.right !== undefined ? { right_origin_ref: origin.right } : {}),
        changes: await Promise.all(row.changes.map(async (change) => {
          const contentMember = change.path[0] === RECORD_CONTENT_MEMBER;
          return {
            path: pathText(change.path),
            change_kind: change.kind,
            ...(await side(change.left, "left", contentMember)),
            ...(await side(change.right, "right", contentMember)),
            ...(change.left_unit !== undefined ? { left_unit: change.left_unit } : {}),
            ...(change.right_unit !== undefined ? { right_unit: change.right_unit } : {})
          };
        }))
      };
    }));
  return {
    comparison_id: comparisonId("state", left.state_id, right.state_id),
    comparison_kind: "model_state_to_model_state",
    left_state_ref: stateRef(left),
    right_state_ref: stateRef(right),
    summary: { ...comparison.summary },
    entity_counts: { ...comparison.entity_counts },
    entities,
    settings: { ...STATE_SETTINGS },
    csv: { table: STATE_COMPARISON_CSV_KIND, row_count: stateCsvRowCount(comparison) },
    diagnostics: [],
    privacy_classification: "private_project_data",
    review_status: "pending",
    professional_boundary: { ...COMPARISON_RECORD_PROFESSIONAL_BOUNDARY },
    provenance: { ...COMPARISON_RECORD_PROVENANCE }
  };
}

/** A side's standing as the comparison holds it: for its whole run, its status and findings. */
function standingRef(side: RunComparisonSide): JsonRecord {
  return { status: side.standing.status, findings: [...side.standing.findings] };
}

function runSideRef(side: RunComparisonSide): JsonRecord {
  if (side.kind === "current") {
    return {
      object_type: "AnalysisRun",
      ref: side.id,
      side_kind: "current_session_run",
      run_record_hash: "not_recorded",
      result_hash: side.csv?.resultHash ?? "not_recorded",
      standing: standingRef(side)
    };
  }
  return {
    object_type: "AnalysisRun",
    ref: side.id,
    side_kind: "recorded_run",
    run_record_hash: side.recorded?.run_record_hash ?? "not_recorded",
    result_hash: side.recorded?.result_hash ?? "not_recorded",
    standing: standingRef(side)
  };
}

function identityRef(identity: RunRowIdentity): JsonRecord {
  return {
    result_id: identity.result_id,
    family: identity.family,
    object_ref: identity.object_ref,
    basis_ref: identity.basis_ref,
    dimension: identity.dimension,
    component: identity.component,
    location: identity.location,
    basis: identity.basis,
    coordinate_system: identity.coordinate_system,
    sign_convention: identity.sign_convention
  };
}

/** The run comparison as a record (without its `hashes` member). */
export function runComparisonRecord(result: RunComparisonResult): JsonRecord {
  const { left, right, comparison } = result;
  const deltas: JsonRecord[] = [];
  const notCompared: JsonRecord[] = [];
  for (const pair of comparison.pairs) {
    // A pair is given a number only with its unit and its dimension (DEL-14-04 AC-002);
    // the comparison refuses a pair without a dimension, and so does the record.
    if (pair.status === "compared" && pair.identity.dimension) {
      deltas.push({
        mapping_id: `auto:${pair.result_id}`,
        match_basis: "stable_result_identity",
        left_result_id: pair.result_id,
        right_result_id: pair.result_id,
        result_family: pair.identity.family,
        identity: identityRef(pair.identity),
        dimension: pair.identity.dimension,
        normalized_unit: pair.unit,
        left_magnitude: pair.left,
        right_magnitude: pair.right,
        normalized_delta: pair.delta,
        tolerance_profile_ref: null,
        classification: RUN_TOLERANCE,
        classification_basis: "no_tolerance_profile"
      });
    } else {
      notCompared.push({
        result_id: pair.result_id,
        identity: identityRef(pair.identity),
        left_unit: pair.status === "compared" ? pair.unit : pair.left_unit,
        right_unit: pair.status === "compared" ? pair.unit : pair.right_unit,
        reasons: pair.status === "compared" ? ["dimension_missing"] : [...pair.reasons]
      });
    }
  }
  return {
    comparison_id: comparisonId("run", left.id, right.id),
    comparison_kind: "analysis_run_to_analysis_run",
    run_context: {
      left: runSideRef(left),
      right: runSideRef(right),
      run_gate: [...comparison.run_gate],
      summary: { ...comparison.summary },
      not_compared: notCompared,
      unmatched: comparison.unmatched.map((row) => ({
        side: row.side,
        result_id: row.result_id,
        identity: identityRef(row.identity),
        unit: row.unit,
        reason: row.reason,
        differing_fields: [...row.differing_fields]
      })),
      tolerance: RUN_TOLERANCE
    },
    result_deltas: deltas,
    settings_deltas: comparison.context_deltas.map((delta) => ({
      setting_key: delta.key,
      left_value: delta.left,
      right_value: delta.right,
      delta_kind: delta.kind
    })),
    csv: { table: RUN_COMPARISON_CSV_KIND, row_count: runComparisonEntries(comparison).length },
    diagnostics: comparison.diagnostics.map((diagnostic) => ({
      code: diagnostic.code,
      severity: diagnostic.severity,
      affected_refs: [...diagnostic.affected_refs],
      message: diagnostic.message,
      change: diagnostic.change
    })),
    privacy_classification: "private_project_data",
    review_status: "pending",
    professional_boundary: { ...COMPARISON_RECORD_PROFESSIONAL_BOUNDARY },
    provenance: { ...COMPARISON_RECORD_PROVENANCE }
  };
}

/** The SHA-256 (hex) of the record's RFC 8785 form without its `hashes` member. */
export async function comparisonRecordHash(record: JsonRecord): Promise<string> {
  const { hashes: _ignored, ...body } = record;
  return canonicalSha256Hex(body);
}

/** The record with its `hashes` member. */
export async function withRecordHash(record: JsonRecord): Promise<JsonRecord> {
  const { hashes: _ignored, ...body } = record;
  return {
    ...body,
    hashes: [{
      algorithm: "sha256",
      canonicalization: "rfc8785_jcs",
      payload_ref: { object_type: record.comparison_kind === "model_state_to_model_state" ? "ModelStateComparison" : "AnalysisRunComparison", ref: String(record.comparison_id) },
      payload_scope: "comparison_record_without_hashes",
      value: await comparisonRecordHash(body)
    }]
  };
}

/** The complete record of a computed comparison, with its hash. */
export async function completeComparisonRecord(kind: "state", result: StateComparisonResult): Promise<JsonRecord>;
export async function completeComparisonRecord(kind: "run", result: RunComparisonResult): Promise<JsonRecord>;
export async function completeComparisonRecord(kind: ReportComparisonKind, result: StateComparisonResult | RunComparisonResult): Promise<JsonRecord> {
  return withRecordHash(kind === "state" ? await stateComparisonRecord(result as StateComparisonResult) : runComparisonRecord(result as RunComparisonResult));
}

/** The record hash the CSV's preamble states for a computed comparison: `sha256:<hex>`, the
 * value of the section's hash reference, formed from the complete record whatever the
 * export's filters. */
export async function comparisonCsvRecordHash(kind: "state", result: StateComparisonResult): Promise<string>;
export async function comparisonCsvRecordHash(kind: "run", result: RunComparisonResult): Promise<string>;
export async function comparisonCsvRecordHash(kind: ReportComparisonKind, result: StateComparisonResult | RunComparisonResult): Promise<string> {
  const record = kind === "state" ? await stateComparisonRecord(result as StateComparisonResult) : runComparisonRecord(result as RunComparisonResult);
  return `sha256:${await comparisonRecordHash(record)}`;
}

/** A comparison a session includes in a report: its two side identifiers, the hash of its
 * complete record, and the bounded section and source diagnostics the assembler formed from
 * that record. Nothing else of the comparison, its record or its sides is kept. */
export type IncludedComparison = {
  kind: ReportComparisonKind;
  projectId: string;
  leftId: string;
  rightId: string;
  /** For a run comparison with a current side, that run's identity: the comparison leaves
   * the report when the session's current run is another. */
  currentRun: CurrentRunIdentity | null;
  recordHash: string;
  section: JsonRecord;
  diagnostics: JsonRecord[];
};

/** The comparisons a session includes in a report, at most one of each kind. */
export type ReportComparisons = { state: IncludedComparison | null; run: IncludedComparison | null };

async function included(kind: ReportComparisonKind, record: JsonRecord, base: Omit<IncludedComparison, "recordHash" | "section" | "diagnostics">): Promise<IncludedComparison> {
  const diagnostics: JsonRecord[] = [];
  const [section] = comparisonSections(kind === "state"
    ? { model_state_comparisons: [record], analysis_run_comparisons: [] }
    : { model_state_comparisons: [], analysis_run_comparisons: [record] }, diagnostics);
  const recordHash = String(((record.hashes as JsonRecord[])[0] as JsonRecord).value);
  return { ...base, recordHash, section, diagnostics };
}

/** A shown comparison as included. */
export async function includedStateComparison(result: StateComparisonResult): Promise<IncludedComparison> {
  return included("state", await completeComparisonRecord("state", result), {
    kind: "state", projectId: result.projectId, leftId: result.left.state_id, rightId: result.right.state_id, currentRun: null
  });
}

export async function includedRunComparison(result: RunComparisonResult): Promise<IncludedComparison> {
  return included("run", await completeComparisonRecord("run", result), {
    kind: "run", projectId: result.projectId, leftId: result.left.id, rightId: result.right.id, currentRun: result.currentRun ?? null
  });
}

/** Whether an included comparison is of these two identifiers, in either order. */
export function includesComparisonOf(included: IncludedComparison | null, leftId: string, rightId: string): boolean {
  return included !== null && ((included.leftId === leftId && included.rightId === rightId) || (included.leftId === rightId && included.rightId === leftId));
}

/** The sections and source diagnostics of the comparisons a session includes, for the section set. */
export function reportComparisonSections(includedComparisons: ReportComparisons): BoundComparisons {
  const held = [includedComparisons.state, includedComparisons.run].filter((item): item is IncludedComparison => item !== null);
  return { sections: held.map((item) => item.section), diagnostics: held.flatMap((item) => item.diagnostics) };
}
