import type { PreviewComparisonUnavailable } from "../comparison/withinRunComparison";
import type { AnalysisRunEnvelope, MechanicsResult, PreviewComparison, PreviewModel } from "../../types";
import {
  buildCurrentModelStateSection,
  checkedMembers,
  holdsWarnings,
  pythonSorted,
  sectionTbds,
  setLevelDiagnostics,
  settleDiagnostics,
  unmatchedClassifications,
  type BoundComparisons
} from "./comparisonSections";

const PROFESSIONAL_BOUNDARY = {
  human_review_required: true,
  software_makes_compliance_claim: false,
  software_makes_certification_claim: false,
  software_makes_sealing_claim: false,
  software_makes_approval_claim: false,
  software_makes_authentication_claim: false,
  software_creates_professional_reliance_record: false,
  software_creates_external_validation_record: false
};

const ENGINE_PROVENANCE = {
  source_name: "OpenPipeStress DEL-08-06 report-section assembler",
  source_location: "core/reporting/state_comparison_handoff_sections/engine.py",
  source_license: "project-governed",
  contributor: "OpenPipeStress Type 2 worker",
  contributor_attestation: "implementation-only-no-professional-claim",
  redistribution_status: "public_permissive",
  review_classification: "machine_checked",
  privacy_classification: "public_metadata"
};

type JsonRecord = Record<string, unknown>;

export function buildStateComparisonHandoffParityFixture(inputs: {
  section_set_id: string;
  model_states: unknown[];
  analysis_runs: unknown[];
  model_state_comparisons: unknown[];
  analysis_run_comparisons: unknown[];
  handoff_packages: unknown[];
  export_workflows: unknown[];
  external_prover_metadata: unknown[];
  source_notes: string[];
  provenance: JsonRecord;
}) {
  if (
    inputs.model_states.length > 0 ||
    inputs.analysis_runs.length > 0 ||
    inputs.model_state_comparisons.length > 0 ||
    inputs.analysis_run_comparisons.length > 0 ||
    inputs.handoff_packages.length > 0 ||
    inputs.export_workflows.length > 0 ||
    inputs.external_prover_metadata.length > 0
  ) {
    throw new Error("DESKTOP-PARITY-FIXTURE-SCOPE: this fixture projection is intentionally empty/invented only.");
  }
  return {
    schema_version: "0.1.0",
    deliverable_id: "DEL-08-06",
    package_id: "PKG-08",
    scope_item: "SOW-024",
    objectives: ["OBJ-007", "OBJ-016", "OBJ-017", "OBJ-018"],
    section_set_id: inputs.section_set_id,
    section_contract_status: "backend_report_section_records_only",
    source_notes: inputs.source_notes.slice().sort(),
    sections: { state_run_sections: [], comparison_sections: [], handoff_sections: [] },
    sow_024_coverage: {
      inputs: "TBD",
      sources: "TBD",
      warnings: "TBD",
      assumptions: "TBD",
      results: "TBD",
      rule_pack_checksums: "TBD",
      limitations: "represented_by_limitations_and_unresolved_tbds"
    },
    limitations: [
      {
        limitation_id: "DEL-08-06-LIMIT-BACKEND-SECTIONS-ONLY",
        statement:
          "Report sections are backend records for human review; final layout, transport, solver execution, and external-tool execution are out of scope.",
        human_review_required: true
      }
    ],
    unresolved_tbds: [
      {
        tbd_id: "DEL-08-06-TBD-FINAL-LAYOUT",
        description: "Final report styling, layout, transport, and release thresholds remain outside this deliverable.",
        review_needed: true
      }
    ],
    diagnostics: [],
    professional_boundary: PROFESSIONAL_BOUNDARY,
    provenance: inputs.provenance
  };
}

function stableSort<T>(items: T[]): T[] {
  return items.slice().sort((left, right) => JSON.stringify(left).localeCompare(JSON.stringify(right)));
}

function privateSessionProvenance(model: PreviewModel) {
  return {
    source_name: `OpenPipeStress desktop session (${model.project.name})`,
    source_location: "local desktop session",
    source_license: "user_supplied_or_private",
    contributor: "user_local_session",
    contributor_certification: "not_asserted",
    redistribution_status: "private_only",
    review_status: "pending",
    privacy_classification: "private_project_data"
  };
}

/** The current model state's section (DEL-08-06; the reference's `build_current_model_state_section`):
 * the document's analysis-status labels and diagnostics are its author's, carried as written; the
 * software's statuses beside them; no standing of the state is derived at assembly. */
function modelStateSection(model: PreviewModel, result: MechanicsResult, modelHash: string): JsonRecord {
  return buildCurrentModelStateSection({
    model_ref: result.model_ref,
    model_hash: modelHash,
    authored_analysis_status: model.analysis_status,
    document_diagnostics: (model.diagnostics ?? []) as unknown[],
    provenance: privateSessionProvenance(model)
  });
}

/** The status-bearing members the desktop's run and within-run sections carry from the
 * software (DEL-08-06 AC-005), with their vocabularies (`comparisonSections.ts`). */
const RUN_SECTION_STATUS_MEMBERS: ReadonlyArray<readonly [string, string]> = [
  ["analysis_status[]", "software_analysis_statuses"],
  ["diagnostics[].severity", "diagnostic_severities"]
];
const WITHIN_RUN_STATUS_MEMBERS: ReadonlyArray<readonly [string, string]> = [
  ["result_deltas[].classification", "delta_classifications"],
  ["result_deltas[].classification_basis", "delta_classification_bases"],
  ["diagnostics[].severity", "diagnostic_severities"]
];

function analysisRunSection(model: PreviewModel, result: MechanicsResult, analysisRun: AnalysisRunEnvelope, diagnosticsOut: JsonRecord[]): JsonRecord {
  const run = analysisRun.analysis_run;
  const provenance = privateSessionProvenance(model);
  const checked = checkedMembers({ analysis_status: run.analysis_status, diagnostics: result.diagnostics }, RUN_SECTION_STATUS_MEMBERS, { object_type: "AnalysisRun", ref: run.run_id }, diagnosticsOut);
  const diagnostics = checked.diagnostics as MechanicsResult["diagnostics"];
  return {
    section_id: `state-run:${run.run_id}`,
    section_kind: "analysis_run_record",
    state_ref: run.model_state_ref,
    run_ref: { object_type: "AnalysisRun", ref: run.run_id },
    hash_refs: run.hashes,
    diagnostics,
    warnings: diagnostics.filter((item) => item.severity !== "blocking" && item.severity !== "error"),
    assumptions: [],
    analysis_status: stableSort(checked.analysis_status as string[]),
    unit_context: { ref_type: "unit_system", ref_id: "unit-system:dec-018-si-dual-display" },
    solver_context: null,
    settings_ref: null,
    load_basis_refs: run.load_basis_refs,
    result_refs: run.result_refs.map((item) => ({
      ...structuredClone(item),
      privacy_classification: "private_project_data",
      provenance
    })),
    rule_pack_refs: [],
    library_refs: [],
    source_provenance: provenance,
    privacy_classification: "private_project_data",
    review_state: "pending",
    reproducibility: run.reproducibility,
    limitations: [],
    professional_boundary: PROFESSIONAL_BOUNDARY,
    text_origin: { diagnostics: "solver", warnings: "solver", source_provenance: "software", privacy_classification: "software", review_state: "software" }
  };
}

function comparisonSection(model: PreviewModel, comparisonSource: PreviewComparison, diagnosticsOut: JsonRecord[]): JsonRecord {
  const checked = checkedMembers(
    { result_deltas: comparisonSource.result_deltas, diagnostics: comparisonSource.diagnostics },
    WITHIN_RUN_STATUS_MEMBERS,
    { object_type: "AnalysisRunComparison", ref: comparisonSource.comparison_id },
    diagnosticsOut
  );
  const comparison = { ...comparisonSource, result_deltas: checked.result_deltas as PreviewComparison["result_deltas"], diagnostics: checked.diagnostics as PreviewComparison["diagnostics"] };
  return {
    section_id: `comparison:${comparison.comparison_id}`,
    section_kind: "analysis_run_comparison",
    comparison_ref: { object_type: "AnalysisRunComparison", ref: comparison.comparison_id },
    run_context: { left: comparison.left, right: comparison.right },
    manual_mappings: comparison.result_deltas.map((item) => ({
      mapping_id: item.mapping_id,
      left_ref: item.left_result_id,
      right_ref: item.right_result_id,
      classification: item.classification
    })),
    // Tokens and references only; the messages stay in `diagnostics`.
    unmatched_classifications: unmatchedClassifications(comparison.diagnostics as unknown[]),
    tolerance_profile_refs: comparison.summary.tolerance_profile_ref === "TBD" ? [] : [comparison.summary.tolerance_profile_ref],
    unit_normalized_deltas: comparison.result_deltas,
    settings_deltas: [],
    diagnostics: comparison.diagnostics,
    source_provenance: privateSessionProvenance(model),
    privacy_classification: "private_project_data",
    review_state: "pending",
    professional_boundary: PROFESSIONAL_BOUNDARY,
    text_origin: { unit_normalized_deltas: "solver_results", diagnostics: "comparison", run_context: "comparison", source_provenance: "software" }
  };
}

/** A within-run refusal as the section set carries it: its reason as its code only, never the
 * refusal's text (`reason_code` is the code `buildWithinRunComparison` derived, or its fixed
 * COMPARISON-BUILD-REFUSED when none could be). */
function carriedUnavailable(unavailable: PreviewComparisonUnavailable): JsonRecord {
  const { reason: _text, ...carried } = unavailable as PreviewComparisonUnavailable & { reason?: unknown };
  return carried;
}

export function buildStateComparisonHandoffSections({
  model,
  result,
  analysisRun,
  comparison,
  comparisonUnavailable = null,
  modelHash,
  sessionComparisons = null
}: {
  model: PreviewModel;
  result: MechanicsResult;
  analysisRun: AnalysisRunEnvelope;
  comparison: PreviewComparison | null;
  /** W-12 S3c: a chosen within-run comparison the builder refused. It is carried
   * beside the sections (not as a section, which the DEL-08-06 assembler's contract
   * does not hold), and only when present, so other records are unchanged. */
  comparisonUnavailable?: PreviewComparisonUnavailable | null;
  modelHash: string;
  /** The saved-state and run comparisons the session includes in the report: the sections the
   * reference assembler's rules (`comparisonSections.ts`) formed from their complete records
   * (`comparisonRecords.ts`), with the source diagnostics that forming raised. With them the
   * set carries the set-level rules those sections touch: their set-level diagnostics, the
   * coverage of their warnings, their unresolved values and the canonical order. Without
   * any, the set is as it was. */
  sessionComparisons?: BoundComparisons | null;
}) {
  // Values outside a status-bearing member's vocabulary in the desktop's own sections
  // (DEL-08-06 AC-005): reported here, and not carried.
  const ownDiagnostics: JsonRecord[] = [];
  const stateRunSections = stableSort([
    modelStateSection(model, result, modelHash),
    analysisRunSection(model, result, analysisRun, ownDiagnostics)
  ]);
  const withinRunSections = comparison ? [comparisonSection(model, comparison, ownDiagnostics)] : [];
  const included = sessionComparisons && sessionComparisons.sections.length > 0 ? sessionComparisons : null;
  const sourceDiagnostics: JsonRecord[] = included ? included.diagnostics : [];
  const comparisonSections = included ? pythonSorted([...withinRunSections, ...included.sections]) : withinRunSections;
  const handoffSections: JsonRecord[] = [];
  const layoutTbd = {
    tbd_id: "DEL-08-06-TBD-FINAL-LAYOUT",
    description: "Final report styling, layout, transport, and release thresholds remain outside this deliverable.",
    review_needed: true
  };
  return {
    schema_version: "0.1.0",
    deliverable_id: "DEL-08-06",
    package_id: "PKG-08",
    scope_item: "SOW-024",
    objectives: ["OBJ-007", "OBJ-016", "OBJ-017", "OBJ-018"],
    section_set_id: `desktop-current-session:${result.run_id}`,
    section_contract_status: "backend_report_section_records_only",
    source_notes: [
      "Current-session desktop projection parity-bound to the DEL-08-06 Python assembler; no source state mutated."
    ],
    sections: {
      state_run_sections: stateRunSections,
      comparison_sections: comparisonSections,
      handoff_sections: handoffSections
    },
    ...(comparisonUnavailable ? { comparison_unavailable: carriedUnavailable(comparisonUnavailable) } : {}),
    sow_024_coverage: {
      inputs: "represented_by_state_run_and_handoff_references",
      sources: "represented_by_source_provenance_and_source_notes",
      warnings: stateRunSections.some((item) => (item.diagnostics as unknown[]).length > 0) || (included !== null && comparisonSections.some(holdsWarnings))
        ? "represented_by_warning_and_diagnostic_lists"
        : "TBD",
      assumptions: "TBD",
      results: "represented_by_result_refs_and_unit_normalized_deltas",
      rule_pack_checksums: "TBD",
      limitations: "represented_by_limitations_and_unresolved_tbds"
    },
    limitations: [
      {
        limitation_id: "DEL-08-06-LIMIT-BACKEND-SECTIONS-ONLY",
        statement:
          "Report sections are backend records for human review; final layout, transport, solver execution, and external-tool execution are out of scope.",
        human_review_required: true
      }
    ],
    unresolved_tbds: included ? pythonSorted([layoutTbd, ...sectionTbds(comparisonSections)]) : [layoutTbd],
    diagnostics: included || ownDiagnostics.length > 0
      ? settleDiagnostics([
        ...ownDiagnostics,
        ...sourceDiagnostics,
        ...(included ? comparisonSections.flatMap((section, index) => setLevelDiagnostics(section, `$.sections.comparison_sections[${index}]`)) : [])
      ])
      : [],
    professional_boundary: PROFESSIONAL_BOUNDARY,
    provenance: privateSessionProvenance(model)
  };
}
