// A computed comparison of two saved states or two runs restated as a record a report
// section set reads (DEL-08-06 REQ-011, REQ-015; the record forms the DEL-08-06 assembler
// `core/reporting/state_comparison_handoff_sections/engine.py` reads, after DEL-14-03 and
// DEL-14-04). The record restates the comparison as the desktop computed it and adds
// nothing:
// - each side by identifier and hash (a saved state's model hash; a recorded run's record
//   and result hashes; the session's current run stated as the current run, not recorded);
// - for states, every added, removed and changed entity, with each changed field's path,
//   change kind and units (no values, and the unchanged count only in the summary);
// - for runs, every compared pair as a delta in its common unit with its dimension, every
//   pair not compared with its reasons and no number, every unmatched row with its reason
//   and no value, the run gate, the context differences as settings deltas, and the
//   solve diagnostics added and removed.
// The desktop has no tolerance profile: a delta is stated `not_tolerance_checked`, never a
// classification that implies one. Identifiers and hashes only: no state or run name,
// note or tag, and no project name. The record's `hashes` member names the SHA-256 of the
// record's RFC 8785 form without that member.
//
// The record is a difference report. It establishes nothing about either side; human
// review is still required for any use of it.
import { canonicalSha256Hex } from "../../services/hashService";
import type { CurrentRunIdentity, RunComparisonResult, StateComparisonResult } from "../history/useModelStateHistory";
import type { RunComparisonSide } from "./runComparisonSides";
import type { RunRowIdentity } from "./runComparison";
import { pathText } from "./stateComparison";

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

/** A deterministic comparison identifier from the two side identifiers, left first. */
export function comparisonId(kind: ReportComparisonKind, leftId: string, rightId: string): string {
  return `${kind === "state" ? "model-state-comparison" : "analysis-run-comparison"}:${leftId}:${rightId}`;
}

/** The saved-state comparison as a record (without its `hashes` member). */
export function stateComparisonRecord(result: StateComparisonResult): JsonRecord {
  const { left, right, comparison } = result;
  const stateRef = (state: StateComparisonResult["left"]) => ({ object_type: "ModelState", ref: state.state_id, model_hash: state.model_hash });
  const entities = comparison.rows
    .filter((row) => row.classification !== "unchanged")
    .map((row) => {
      const ref = { object_type: row.family, ref: row.id };
      return {
        row_id: `row:${row.family}:${row.key.join("/")}`,
        classification: row.classification,
        match_basis: row.classification === "changed" ? "stable_id" : "unmatched",
        mapping_id: null,
        left_ref: row.classification === "added" ? null : ref,
        right_ref: row.classification === "removed" ? null : ref,
        changes: row.changes.map((change) => ({
          path: pathText(change.path),
          change_kind: change.kind,
          ...(change.left_unit !== undefined ? { left_unit: change.left_unit } : {}),
          ...(change.right_unit !== undefined ? { right_unit: change.right_unit } : {})
        }))
      };
    });
  return {
    comparison_id: comparisonId("state", left.state_id, right.state_id),
    comparison_kind: "model_state_to_model_state",
    left_state_ref: stateRef(left),
    right_state_ref: stateRef(right),
    summary: { ...comparison.summary },
    entity_counts: { ...comparison.entity_counts },
    entities,
    settings: { ...STATE_SETTINGS },
    diagnostics: [],
    privacy_classification: "private_project_data",
    review_status: "pending",
    professional_boundary: { ...COMPARISON_RECORD_PROFESSIONAL_BOUNDARY },
    provenance: { ...COMPARISON_RECORD_PROVENANCE }
  };
}

function runSideRef(side: RunComparisonSide): JsonRecord {
  if (side.kind === "current") {
    return {
      object_type: "AnalysisRun",
      ref: side.id,
      side_kind: "current_session_run",
      run_record_hash: "not_recorded",
      result_hash: side.csv?.resultHash ?? "not_recorded"
    };
  }
  return {
    object_type: "AnalysisRun",
    ref: side.id,
    side_kind: "recorded_run",
    run_record_hash: side.recorded?.run_record_hash ?? "not_recorded",
    result_hash: side.recorded?.result_hash ?? "not_recorded"
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

/** The record with its `hashes` member: the SHA-256 of the record's RFC 8785 form without it. */
export async function withRecordHash(record: JsonRecord): Promise<JsonRecord> {
  const { hashes: _ignored, ...body } = record;
  const value = await canonicalSha256Hex(body);
  return {
    ...body,
    hashes: [{
      algorithm: "sha256",
      canonicalization: "rfc8785_jcs",
      payload_ref: { object_type: record.comparison_kind === "model_state_to_model_state" ? "ModelStateComparison" : "AnalysisRunComparison", ref: String(record.comparison_id) },
      payload_scope: "comparison_record_without_hashes",
      value
    }]
  };
}

/** A comparison a session includes in a report: its two side identifiers and its record,
 * restated when it was included. It holds no side's rows: the comparison's own output is
 * what the record restates, and nothing else of either side is kept. */
export type IncludedComparison = {
  kind: ReportComparisonKind;
  projectId: string;
  leftId: string;
  rightId: string;
  /** For a run comparison with a current side, that run's identity: the comparison leaves
   * the report when the session's current run is another. */
  currentRun: CurrentRunIdentity | null;
  record: JsonRecord;
};

/** The comparisons a session includes in a report, at most one of each kind. */
export type ReportComparisons = { state: IncludedComparison | null; run: IncludedComparison | null };

/** A shown comparison as included: identifiers and record only. */
export function includedStateComparison(result: StateComparisonResult): IncludedComparison {
  return { kind: "state", projectId: result.projectId, leftId: result.left.state_id, rightId: result.right.state_id, currentRun: null, record: stateComparisonRecord(result) };
}

export function includedRunComparison(result: RunComparisonResult): IncludedComparison {
  return { kind: "run", projectId: result.projectId, leftId: result.left.id, rightId: result.right.id, currentRun: result.currentRun ?? null, record: runComparisonRecord(result) };
}

/** Whether an included comparison is of these two identifiers, in either order. */
export function includesComparisonOf(included: IncludedComparison | null, leftId: string, rightId: string): boolean {
  return included !== null && ((included.leftId === leftId && included.rightId === rightId) || (included.leftId === rightId && included.rightId === leftId));
}

/** The records of the comparisons a session includes, each with its hash, in kind order. */
export async function reportComparisonRecords(included: ReportComparisons): Promise<{ model_state_comparisons: JsonRecord[]; analysis_run_comparisons: JsonRecord[] }> {
  return {
    model_state_comparisons: included.state ? [await withRecordHash(included.state.record)] : [],
    analysis_run_comparisons: included.run ? [await withRecordHash(included.run.record)] : []
  };
}
