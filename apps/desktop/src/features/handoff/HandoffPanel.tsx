import { Download, Share2 } from "lucide-react";
import type {
  AgentProposal,
  AnalysisRunEnvelope,
  DesignKnowledge,
  EditorOperationIntent,
  MechanicsResult,
  PreviewComparison,
  PreviewModel,
  SelectedReviewTarget
} from "../../types";
import { buildExportUnitSystemDisclosure, unitDisclosureSummary } from "../exportUnitDisclosure";

type HandoffUnitPreservationWitness = {
  witness_id: string;
  source_ref: {
    object_type: string;
    ref: string;
  };
  source_field_path: string;
  source_quantity: {
    value: number;
    unit: string;
    dimension: string;
  };
  target_field_path: string;
  target_quantity: {
    value: number;
    unit: string;
    dimension: string;
  };
  target_quantity_policy: "referenced_result_value_unit_and_dimension_preserved";
  export_unit_policy: "preserve_source_result_unit_and_dimension";
  conversion_performed: false;
  unit_system_ref: {
    object_type: string;
    ref: string;
  };
  provenance: ReturnType<typeof previewProvenance>;
};

export function HandoffPanel({
  model,
  knowledge,
  result,
  analysisRun,
  comparison,
  editorIntents,
  proposal,
  selectedReviewTarget
}: {
  model: PreviewModel;
  knowledge: DesignKnowledge | null;
  result: MechanicsResult | null;
  analysisRun: AnalysisRunEnvelope | null;
  comparison: PreviewComparison | null;
  editorIntents: EditorOperationIntent[];
  proposal: AgentProposal | null;
  selectedReviewTarget: SelectedReviewTarget | null;
}) {
  const handoffPackage = result && analysisRun ? buildHandoffPackage({
    model,
    knowledge,
    result,
    analysisRun,
    comparison,
    editorIntents,
    proposal,
    selectedReviewTarget
  }) : null;

  return (
    <section className="panel handoff-panel" aria-label="Handoff package" data-testid="handoff-panel">
      <div className="panel-title">
        <Share2 size={16} aria-hidden="true" />
        Handoff Package
      </div>
      {handoffPackage ? (
        <>
          <div className="report-actions">
            <a
              className="report-export-link"
              data-testid="handoff-export-link"
              download={`openpipestress-preview-handoff-${safeFileToken(handoffPackage.analysis_run_ref.ref)}.json`}
              href={jsonDataHref(handoffPackage)}
            >
              <Download size={14} aria-hidden="true" />
              Local package JSON
            </a>
            <span data-testid="handoff-export-summary">
              {handoffPackage.stable_id_map.entity_ref_count} entities; {handoffPackage.diagnostic_refs.length} diagnostics;{" "}
              {handoffPackage.loss_report.unsupported_behavior_refs.length} loss notes; no private payload
            </span>
          </div>
          <div className="report-list" data-testid="handoff-package-body">
            <HandoffLine label="Package" value={handoffPackage.package_id} testId="handoff-package-id" />
            <HandoffLine
              label="State and run"
              value={`${handoffPackage.model_state_ref.ref}; ${handoffPackage.analysis_run_ref.ref}`}
              testId="handoff-state-run"
            />
            <HandoffLine
              label="Target profile"
              value={`${handoffPackage.target_mapping.profile_ref}; ${handoffPackage.target_mapping.mapping_status}`}
              testId="handoff-target-profile"
            />
            <HandoffLine
              label="Stable IDs"
              value={`${handoffPackage.stable_id_map.entity_ref_count} entity refs; ${handoffPackage.stable_id_map.result_ref_count} result refs`}
              testId="handoff-stable-ids"
            />
            <HandoffLine
              label="Units"
              value={unitDisclosureSummary(handoffPackage.unit_system_disclosure)}
              testId="handoff-units"
            />
            <HandoffLine
              label="Unit witnesses"
              value={`count=${handoffPackage.unit_preservation_witnesses.length}; policy=preserve_source_result_units; conversion=${String(
                handoffPackage.unit_preservation_witnesses.some((item) => item.conversion_performed)
              )}`}
              testId="handoff-unit-witnesses"
            />
            <HandoffLine
              label="Loss report"
              value={handoffPackage.loss_report.unsupported_behavior_refs.join("; ")}
              testId="handoff-loss-report"
            />
            <HandoffLine
              label="Review context"
              value={`${formatPendingOperationCount(handoffPackage.editor_intent_refs.length)}; proposal=${handoffPackage.proposal_ref}`}
              testId="handoff-review-context"
            />
            <HandoffLine label="Boundary" value={handoffBoundary(handoffPackage)} testId="handoff-boundary" />
          </div>
        </>
      ) : (
        <p className="muted" data-testid="handoff-empty">
          Run mechanics preview to assemble a local review handoff package with stable IDs, warnings, and unsupported-behavior notes.
        </p>
      )}
      <small className="report-note">
        Local handoff packages are review context only; target-specific mappings and external validation remain user-controlled.
      </small>
    </section>
  );
}

function HandoffLine({ label, value, testId }: { label: string; value: string; testId: string }) {
  return (
    <div className="report-line" data-testid={testId}>
      <span>{label}</span>
      <strong>{value}</strong>
    </div>
  );
}

function buildHandoffPackage({
  model,
  knowledge,
  result,
  analysisRun,
  comparison,
  editorIntents,
  proposal,
  selectedReviewTarget
}: {
  model: PreviewModel;
  knowledge: DesignKnowledge | null;
  result: MechanicsResult;
  analysisRun: AnalysisRunEnvelope;
  comparison: PreviewComparison | null;
  editorIntents: EditorOperationIntent[];
  proposal: AgentProposal | null;
  selectedReviewTarget: SelectedReviewTarget | null;
}) {
  const run = analysisRun.analysis_run;
  const diagnosticRefs = [...model.diagnostics, ...(knowledge?.diagnostics ?? []), ...result.diagnostics].map(
    (item) => item.id ?? item.code
  );
  const entityRefs = stableEntityRefs(model);
  const lossReport = {
    omitted_behavior_refs: [] as string[],
    approximated_behavior_refs: [] as string[],
    delegated_behavior_refs: ["target-specific external prover execution remains user-controlled"],
    unsupported_behavior_refs: [
      "protected code/rule criteria are not bundled",
      "target-specific commercial-solver field mapping is not generated",
      "professional validation and acceptance are not software-generated"
    ],
    loss_report_status: "preview_loss_report_not_target_verified"
  };
  const unitSystemDisclosure = buildExportUnitSystemDisclosure({
    model,
    result,
    targetExportUnits: model.project.units,
    conversionPolicy: "handoff_package_preserves_source_result_value_unit_and_dimension_by_reference",
    conversionPerformed: false,
    conversionScope: [],
    sourceLocation: "apps/desktop/src/features/handoff/HandoffPanel.tsx"
  });
  const unitPreservationWitnesses = handoffUnitPreservationWitnesses(result);

  return {
    schema_version: "0.1.0",
    document_kind: "openpipestress.technical_preview.handoff_package",
    export_scope: "local_browser_download_preview",
    package_id: `handoff:${safeRefToken(model.project.id)}:${safeRefToken(result.run_id)}`,
    deliverable_refs: ["DEL-15-01", "DEL-15-02", "DEL-15-03", "DEL-17-03"],
    project_ref: model.project.id,
    model_ref: result.model_ref,
    model_state_ref: run.model_state_ref,
    analysis_run_ref: {
      object_type: "AnalysisRun",
      ref: run.run_id
    },
    units_manifest: model.project.units,
    unit_system_disclosure: unitSystemDisclosure,
    unit_witness_policy: "preserve_source_result_value_unit_and_dimension_per_handoff_result_ref",
    unit_preservation_witnesses: unitPreservationWitnesses,
    stable_id_map: {
      id_basis: "model entity IDs and computed result IDs from the invented preview model",
      entity_ref_count: entityRefs.length,
      entity_refs: entityRefs,
      result_ref_count: result.results.length,
      selected_result_refs: selectedResultRefs(result)
    },
    library_refs: {
      material_refs: model.materials?.map((item) => ({ ref: item.id, provenance: item.provenance })) ?? [],
      component_refs: model.components.map((item) => ({ ref: item.id, provenance: item.provenance })),
      rule_pack_refs: [] as string[],
      private_library_payload_included: false
    },
    target_mapping: {
      profile_ref: "native_open_json_preview",
      mapping_status: "stable_ids_only_not_target_specific",
      target_field_coverage: "TBD",
      manual_mapping_required: false,
      unsupported_behavior_policy: "loss_report_required_before_target_specific_export"
    },
    run_audit_refs: {
      input_manifest_refs: run.reproducibility.input_manifest_refs,
      hash_scopes: run.hashes.map((item) => item.payload_scope),
      result_value_hash_count: run.result_refs.reduce((count, item) => count + item.hash_refs.length, 0)
    },
    comparison_ref: comparison?.comparison_id ?? "not generated",
    comparison_summary: comparison
      ? {
          comparable_result_pairs: comparison.summary.comparable_result_pairs,
          unmatched_left_results: comparison.summary.unmatched_left_results,
          unmatched_right_results: comparison.summary.unmatched_right_results,
          tolerance_status: comparison.summary.tolerance_status,
          tolerance_profile_ref: comparison.summary.tolerance_profile_ref,
          release_or_professional_claim: false
        }
      : null,
    editor_intent_refs: editorIntents.map((intent) => intent.operation_id),
    editor_operation_statuses: [...new Set(editorIntents.map((intent) => intent.validation.application_status))],
    proposal_ref: proposal?.proposal_id ?? "not generated",
    selected_review_target: selectedReviewTarget,
    diagnostic_refs: diagnosticRefs,
    unresolved_assumptions: [
      ...run.reproducibility.unresolved_tbd,
      "target-specific field coverage TBD",
      "external prover acceptance status not generated"
    ],
    loss_report: lossReport,
    data_boundary: model.data_boundary,
    private_payload_included: false,
    protected_content_included: false,
    release_or_professional_claim: false,
    professional_boundary: {
      human_review_required: true,
      software_makes_compliance_claim: false,
      software_makes_certification_claim: false,
      software_makes_sealing_claim: false,
      software_makes_approval_claim: false,
      software_makes_authentication_claim: false
    }
  };
}

function stableEntityRefs(model: PreviewModel): string[] {
  return [
    model.project.id,
    ...(model.materials?.map((item) => item.id) ?? []),
    ...model.nodes.map((item) => item.id),
    ...model.pipe_segments.map((item) => item.id),
    ...model.supports.map((item) => item.id),
    ...model.components.map((item) => item.id),
    ...model.load_cases.map((item) => item.id),
    ...(model.combinations?.map((item) => item.id) ?? [])
  ].sort();
}

function handoffUnitPreservationWitnesses(result: MechanicsResult): HandoffUnitPreservationWitness[] {
  return result.results
    .filter((item) => typeof item.value === "number" && Number.isFinite(item.value) && Boolean(item.unit))
    .map((item) => {
      const dimension = resultDimension(item.kind);
      return {
        witness_id: `handoff-unit:${safeRefToken(item.id)}`,
        source_ref: reference("ResultRow", item.id),
        source_field_path: `mechanics_result.results[${item.id}].value`,
        source_quantity: {
          value: item.value,
          unit: item.unit,
          dimension
        },
        target_field_path: "handoff_package.unit_preservation_witnesses[]",
        target_quantity: {
          value: item.value,
          unit: item.unit,
          dimension
        },
        target_quantity_policy: "referenced_result_value_unit_and_dimension_preserved",
        export_unit_policy: "preserve_source_result_unit_and_dimension",
        conversion_performed: false,
        unit_system_ref: reference("UnitSystem", "unit-system:dec-018-si-dual-display"),
        provenance: previewProvenance()
      };
    });
}

function resultDimension(kind: string): string {
  if (kind.includes("displacement")) return "displacement";
  if (kind.includes("rotation")) return "rotation";
  if (kind.includes("force")) return "force";
  if (kind.includes("moment")) return "moment";
  if (kind.includes("reaction")) return "force";
  if (kind.includes("stress")) return "stress";
  if (kind.includes("ratio")) return "dimensionless";
  return "TBD";
}

function selectedResultRefs(result: MechanicsResult): string[] {
  return [
    result.summary.max_displacement?.result_ref,
    result.summary.max_open_formula_stress?.result_ref,
    result.results.find((item) => item.id === "result:force:pipe-P-120:axial")?.id,
    result.results.find((item) => item.id === "result:stress:pipe-P-120:end-j:torsional-shear")?.id
  ].filter((value): value is string => Boolean(value));
}

function reference(objectType: string, ref: string) {
  return { object_type: objectType, ref };
}

function previewProvenance() {
  return {
    source_name: "OpenPipeStress technical preview handoff package",
    source_location: "apps/desktop/src/features/handoff/HandoffPanel.tsx",
    source_license: "project-governed",
    contributor: "OpenPipeStress app integration tranche",
    contributor_certification:
      "Invented preview result unit metadata only; no protected standards or private payloads.",
    redistribution_status: "public_permissive",
    review_status: "desktop_preview",
    privacy_classification: "public_metadata"
  };
}

function handoffBoundary(handoffPackage: ReturnType<typeof buildHandoffPackage>): string {
  if (
    handoffPackage.professional_boundary.human_review_required &&
    !handoffPackage.professional_boundary.software_makes_compliance_claim &&
    !handoffPackage.professional_boundary.software_makes_approval_claim &&
    !handoffPackage.private_payload_included &&
    !handoffPackage.protected_content_included &&
    !handoffPackage.release_or_professional_claim
  ) {
    return "local review handoff only; no private payload; no protected content; validation occurs in the user's accepted professional tools";
  }
  return "handoff boundary requires attention";
}

function formatPendingOperationCount(count: number): string {
  return `${count} pending ${count === 1 ? "operation" : "operations"}`;
}

function jsonDataHref(value: unknown): string {
  return `data:application/json;charset=utf-8,${encodeURIComponent(JSON.stringify(value, null, 2))}`;
}

function safeRefToken(value: string): string {
  return value.replace(/[^a-zA-Z0-9:_-]+/g, "-").replace(/^-+|-+$/g, "") || "ref";
}

function safeFileToken(value: string): string {
  return value.replace(/[^a-zA-Z0-9_-]+/g, "-").replace(/^-+|-+$/g, "") || "preview";
}
