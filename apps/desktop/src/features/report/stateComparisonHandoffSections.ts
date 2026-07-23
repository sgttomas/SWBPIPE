import type { AnalysisRunEnvelope, MechanicsResult, PreviewComparison, PreviewModel } from "../../types";

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

function modelStateSection(model: PreviewModel, result: MechanicsResult, modelHash: string): JsonRecord {
  const stateRef = `state:${result.model_ref}:preview`;
  return {
    section_id: `state-run:${stateRef}`,
    section_kind: "model_state_record",
    state_ref: { object_type: "ModelState", ref: stateRef },
    run_ref: null,
    hash_refs: [
      {
        algorithm: "sha256",
        canonicalization: "rfc8785_jcs",
        payload_ref: { object_type: "ModelState", ref: stateRef },
        payload_scope: "model_state",
        value: modelHash
      }
    ],
    diagnostics: model.diagnostics ?? [],
    warnings: model.diagnostics ?? [],
    assumptions: [],
    analysis_status: stableSort(
      [model.analysis_status.mechanics, model.analysis_status.rule_check, "HUMAN_REVIEW_REQUIRED"].filter(Boolean)
    ),
    unit_context: { ref_type: "unit_system", ref_id: "unit-system:dec-018-si-dual-display" },
    solver_context: null,
    settings_ref: null,
    source_provenance: privateSessionProvenance(model),
    privacy_classification: "private_project_data",
    review_state: "pending",
    limitations: [],
    professional_boundary: PROFESSIONAL_BOUNDARY
  };
}

function analysisRunSection(model: PreviewModel, result: MechanicsResult, analysisRun: AnalysisRunEnvelope): JsonRecord {
  const run = analysisRun.analysis_run;
  const provenance = privateSessionProvenance(model);
  return {
    section_id: `state-run:${run.run_id}`,
    section_kind: "analysis_run_record",
    state_ref: run.model_state_ref,
    run_ref: { object_type: "AnalysisRun", ref: run.run_id },
    hash_refs: run.hashes,
    diagnostics: result.diagnostics,
    warnings: result.diagnostics.filter((item) => item.severity !== "blocking" && item.severity !== "error"),
    assumptions: [],
    analysis_status: stableSort(run.analysis_status),
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
    professional_boundary: PROFESSIONAL_BOUNDARY
  };
}

function comparisonSection(model: PreviewModel, comparison: PreviewComparison): JsonRecord {
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
    unmatched_classifications: comparison.diagnostics,
    tolerance_profile_refs: comparison.summary.tolerance_profile_ref === "TBD" ? [] : [comparison.summary.tolerance_profile_ref],
    unit_normalized_deltas: comparison.result_deltas,
    settings_deltas: [],
    diagnostics: comparison.diagnostics,
    source_provenance: privateSessionProvenance(model),
    privacy_classification: "private_project_data",
    review_state: "pending",
    professional_boundary: PROFESSIONAL_BOUNDARY
  };
}

export function buildStateComparisonHandoffSections({
  model,
  result,
  analysisRun,
  comparison,
  modelHash
}: {
  model: PreviewModel;
  result: MechanicsResult;
  analysisRun: AnalysisRunEnvelope;
  comparison: PreviewComparison | null;
  modelHash: string;
}) {
  const stateRunSections = stableSort([
    modelStateSection(model, result, modelHash),
    analysisRunSection(model, result, analysisRun)
  ]);
  const comparisonSections = comparison ? [comparisonSection(model, comparison)] : [];
  const handoffSections: JsonRecord[] = [];
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
    sow_024_coverage: {
      inputs: "represented_by_state_run_and_handoff_references",
      sources: "represented_by_source_provenance_and_source_notes",
      warnings: stateRunSections.some((item) => (item.diagnostics as unknown[]).length > 0)
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
    unresolved_tbds: [
      {
        tbd_id: "DEL-08-06-TBD-FINAL-LAYOUT",
        description: "Final report styling, layout, transport, and release thresholds remain outside this deliverable.",
        review_needed: true
      }
    ],
    diagnostics: [],
    professional_boundary: PROFESSIONAL_BOUNDARY,
    provenance: privateSessionProvenance(model)
  };
}
