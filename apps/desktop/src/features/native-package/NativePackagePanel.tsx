import { Download } from "lucide-react";
import { usePackageHash } from "../../services/usePackageHash";
import type {
  AgentProposal,
  AnalysisRunEnvelope,
  Diagnostic,
  EditorOperationIntent,
  LocalProjectSummary,
  LocalStorageCapability,
  MechanicsResult,
  ModelHashEvidence,
  ObjectRef,
  PackageHashEvidence,
  PreviewModel,
  SelectedReviewTarget
} from "../../types";

type Props = {
  analysisRun: AnalysisRunEnvelope | null;
  editorIntents: EditorOperationIntent[];
  model: PreviewModel;
  modelHash: ModelHashEvidence | null;
  projectSummary: LocalProjectSummary | null;
  proposal: AgentProposal | null;
  result: MechanicsResult | null;
  selectedReviewTarget: SelectedReviewTarget | null;
  storageCapability: LocalStorageCapability | null;
};

type QuantityRecord = { value: number; unit: string };

type UnitPreservationWitness = {
  witness_id: string;
  source_ref: {
    ref_type: string;
    ref_id: string;
    field_path: string;
  };
  target_ref: {
    member_path: string;
    field_path: string;
  };
  source_quantity: QuantityRecord & {
    dimension?: string;
  };
  target_quantity: QuantityRecord & {
    dimension?: string;
  };
  conversion_performed: false;
  preservation_status: "unit_and_value_preserved";
  basis_refs: string[];
  provenance: string;
};

export function NativePackagePanel({
  analysisRun,
  editorIntents,
  model,
  modelHash,
  projectSummary,
  proposal,
  result,
  selectedReviewTarget,
  storageCapability
}: Props) {
  const basePacket = result && analysisRun
    ? buildNativePackageReview({
        analysisRun,
        editorIntents,
        model,
        modelHash,
        projectSummary,
        proposal,
        result,
        selectedReviewTarget,
        storageCapability
      })
    : null;
  const packageHash = usePackageHash(basePacket?.package_id ?? null, basePacket);
  const packet = basePacket ? withPackageHash(basePacket, packageHash) : null;
  const jsonDataHref = packet
    ? `data:application/json;charset=utf-8,${encodeURIComponent(JSON.stringify(packet))}`
    : "";

  return (
    <section className="panel native-package-panel" aria-label="Native JSON package" data-testid="native-package-panel">
      <div className="panel-heading">
        <div>
          <p className="eyebrow">Local Package</p>
          <h2>Native JSON Package</h2>
        </div>
      </div>

      {packet ? (
        <>
          <div className="report-actions">
            <a
              className="report-export-link"
              data-testid="native-package-link"
              download={`openpipestress-preview-native-package-${safeFileToken(model.project.id)}.json`}
              href={jsonDataHref}
            >
              <Download size={14} aria-hidden="true" />
              Export JSON
            </a>
            <span data-testid="native-package-summary">
              ready; members={packet.manifest.package_members.length}; entities=
              {packet.stable_id_map.entity_ref_count}; results={packet.stable_id_map.result_ref_count}; operations=
              {packet.stable_id_map.operation_ref_count}
            </span>
          </div>
          <div className="report-list" data-testid="native-package-body">
            <PackageLine
              label="Profile"
              value={`${packet.export_profile.profile_id}; physical_container=${packet.export_profile.physical_project_container}; public_transport=${packet.export_profile.public_transport_protocol}`}
              testId="native-package-profile"
            />
            <PackageLine
              label="Members"
              value={packet.manifest.package_members.map((member) => member.path).join(", ")}
              testId="native-package-members"
            />
            <PackageLine
              label="Unit witnesses"
              value={`project_units=${packet.unit_preservation.project_unit_declarations.length}; model_quantities=${packet.unit_preservation.summary.model_quantity_witness_count}; result_quantities=${packet.unit_preservation.summary.result_quantity_witness_count}; conversion=${String(packet.unit_preservation.conversion_performed)}`}
              testId="native-package-unit-witnesses"
            />
            <PackageLine
              label="Stable IDs"
              value={`entities=${packet.stable_id_map.entity_ref_count}; results=${packet.stable_id_map.result_ref_count}; operations=${packet.stable_id_map.operation_ref_count}`}
              testId="native-package-stable-ids"
            />
            <PackageLine
              label="Validation"
              value={`shape=${packet.validation_report.package_shape_status}; deterministic_paths=${String(packet.validation_report.deterministic_member_paths)}; model_hash=${packet.validation_report.model_hash_status}; package_hash=${packet.validation_report.package_hash_status}`}
              testId="native-package-validation"
            />
            <PackageLine
              label="Loss report"
              value={`${packet.loss_report.summary.tbd_count} TBD; ${packet.loss_report.summary.unsupported_count} unsupported; ${packet.loss_report.entries[0]?.reason ?? "no entries"}`}
              testId="native-package-loss-report"
            />
            <PackageLine
              label="Storage"
              value={`engine=${packet.generation_context.storage_engine}; network=${String(packet.generation_context.network_required)}; telemetry=${String(packet.generation_context.telemetry_enabled)}; repository_default_private_write=false`}
              testId="native-package-storage"
            />
            <PackageLine
              label="Persisted review context"
              value={`editor_intents=${packet.source_project.storage_summary.editor_intent_count}; proposals=${packet.source_project.storage_summary.proposal_count}; selected_targets=${packet.source_project.storage_summary.selected_review_target_count}; selected_ref=${packet.source_project.storage_summary.selected_review_target_ref}; mechanics_results=${packet.source_project.storage_summary.persisted_mechanics_result_count}; analysis_runs=${packet.source_project.storage_summary.persisted_analysis_run_count}; run_ref=${packet.source_project.storage_summary.persisted_analysis_run_ref}`}
              testId="native-package-persisted-review-context"
            />
            <PackageLine
              label="Boundary"
              value={nativePackageBoundary(packet)}
              testId="native-package-boundary"
            />
          </div>
        </>
      ) : (
        <p className="muted" data-testid="native-package-empty">
          Run mechanics preview to assemble a native JSON package review manifest with stable IDs, loss report, validation report, and local-only export boundaries.
        </p>
      )}
    </section>
  );
}

function PackageLine({ label, testId, value }: { label: string; testId: string; value: string }) {
  return (
    <div className="comparison-fact" data-testid={testId}>
      <span>{label}</span>
      <strong>{value}</strong>
    </div>
  );
}

export function buildNativePackageReview({
  analysisRun,
  editorIntents,
  model,
  modelHash,
  projectSummary,
  proposal,
  result,
  selectedReviewTarget,
  storageCapability
}: {
  analysisRun: AnalysisRunEnvelope;
  editorIntents: EditorOperationIntent[];
  model: PreviewModel;
  modelHash: ModelHashEvidence | null;
  projectSummary: LocalProjectSummary | null;
  proposal: AgentProposal | null;
  result: MechanicsResult;
  selectedReviewTarget: SelectedReviewTarget | null;
  storageCapability: LocalStorageCapability | null;
}) {
  const entityRefs = modelEntityRefs(model);
  const resultRefs = result.results.map((item) => item.id).sort();
  const operationRefs = operationReviewRefs({ editorIntents, proposal });
  const proposalRefs = proposal ? [proposal.proposal_id] : [];
  const diagnostics = normalizeDiagnostics([...model.diagnostics, ...result.diagnostics]);
  const unitPreservation = buildUnitPreservationEvidence({ model, result });
  const runHashRefs = analysisRun.analysis_run.hashes.map((hash) => ({
    algorithm: hash.algorithm,
    canonicalization: hash.canonicalization,
    payload_ref: hash.payload_ref,
    payload_scope: hash.payload_scope,
    value: hash.value
  }));
  const memberRecords = [
    member("manifest.json", "manifest", "package member inventory and boundary summary", "TBD_browser_preview_manifest_not_canonicalized"),
    member("model/project.json", "model_payload", "invented preview project/model payload", modelHash?.value ?? "TBD_model_hash_not_available"),
    member("maps/stable_id_map.json", "stable_id_map", "canonical OpenPipeStress entity/result/operation refs", "TBD_id_map_hash_not_available"),
    member("reports/loss_report.json", "loss_report", "required exported/omitted/approximated/delegated/unsupported/tbd review", "TBD_loss_report_hash_not_available"),
    member("reports/validation_report.json", "validation_report", "package-shape and source-boundary evidence only", "TBD_validation_report_hash_not_available"),
    member("diagnostics/diagnostics.json", "diagnostics", "model and mechanics diagnostics by stable ID", "TBD_diagnostics_hash_not_available"),
    member("runs/analysis_run_ref.json", "analysis_run_ref", "analysis run references and accepted run hashes", runHashStatus(runHashRefs)),
    member("results/result_envelope_ref.json", "result_envelope_ref", "result-envelope reference, status, and hash scope", runHashStatus(runHashRefs, "result_envelope")),
    member("maps/unit_preservation_witnesses.json", "unit_preservation_witnesses", "native package unit/value preservation witnesses", "TBD_unit_preservation_witness_hash_not_available"),
    member("operations/pending_operations.json", "operation_queue", "review-only editor/proposal operations", "TBD_operation_queue_hash_not_available")
  ];

  return {
    schema_version: "0.1.0",
    document_kind: "openpipestress.technical_preview.native_json_package_review",
    deliverable_refs: ["DEL-17-02", "DEL-17-03", "DEL-02-05", "DEL-12-01", "DEL-08-04", "DEL-14-02"],
    package_refs: ["PKG-17", "PKG-02", "PKG-12", "PKG-08", "PKG-14"],
    scope_items: ["SOW-030", "SOW-074", "SOW-050", "SOW-029", "SOW-046", "SOW-072"],
    objectives: ["OBJ-009", "OBJ-017", "OBJ-012", "OBJ-010", "OBJ-016"],
    package_id: `native-json-preview:${model.project.id}`,
    export_scope: "local_browser_download_preview",
    export_profile: {
      profile_id: "native_open_json_preview",
      profile_status: "technical_preview_review_manifest",
      target_family: "project_owned_native_json",
      profile_version: "0.1.0",
      source_basis_refs: ["DEL-17-02", "DEL-17-03"],
      unit_policy: "preserve explicit units from model and result envelopes",
      unit_witness_policy: "required_sidecar_for_native_json_quantity_fields",
      coordinate_policy: "preserve source model coordinates",
      stable_id_policy: "package sidecar stable ID map",
      loss_report_policy: "mandatory",
      physical_project_container: "TBD",
      public_transport_protocol: "TBD",
      target_specific_adapter_behavior: "TBD"
    },
    source_project: {
      project_ref: reference("Project", model.project.id),
      project_name: model.project.name,
      model_schema_version: model.schema_version,
      data_boundary: model.data_boundary,
      storage_summary: projectSummary
        ? {
            storage_mode: projectSummary.storage_mode,
            migration_status: projectSummary.migration_status,
            copied_external_files: projectSummary.copied_external_files,
            editor_intent_count: projectSummary.editor_intent_count,
            proposal_count: projectSummary.proposal_count,
            selected_review_target_count: projectSummary.selected_review_target_count,
            selected_review_target_ref: projectSummary.selected_review_target_ref,
            persisted_mechanics_result_count: projectSummary.persisted_mechanics_result_count,
            persisted_analysis_run_count: projectSummary.persisted_analysis_run_count,
            persisted_analysis_run_ref: projectSummary.persisted_analysis_run_ref
          }
        : {
            storage_mode: "not_persisted_this_session",
            migration_status: "TBD",
            copied_external_files: false,
            editor_intent_count: 0,
            proposal_count: 0,
            selected_review_target_count: 0,
            selected_review_target_ref: "not_selected",
            persisted_mechanics_result_count: 0,
            persisted_analysis_run_count: 0,
            persisted_analysis_run_ref: "not_persisted"
          }
    },
    manifest: {
      source_model_ref: reference("Project", model.project.id),
      source_model_version_or_hash_basis: modelHash?.value ?? "TBD_model_hash_not_available",
      export_profile_id: "native_open_json_preview",
      package_members: memberRecords,
      deterministic_hash_basis: "JCS-compatible JSON where member canonical payloads are available",
      runtime_timestamp_fields_in_hash_inputs: false,
      boundary_notes: [
        "technical-preview browser-local JSON review package only",
        "physical project container remains TBD",
        "public transport and target-specific adapter behavior remain TBD",
        "no protected standards content or private rule payload is bundled",
        "no release, compatibility, code-compliance, or professional acceptance claim"
      ]
    },
    stable_id_map: {
      carrier: "sidecar_mapping",
      entity_ref_count: entityRefs.length,
      result_ref_count: resultRefs.length,
      operation_ref_count: operationRefs.length,
      entity_refs: entityRefs,
      result_refs: resultRefs,
      operation_refs: operationRefs,
      proposal_refs: proposalRefs,
      omitted_canonical_ids: [],
      direct_target_carried_ids: [],
      target_generated_ids: []
    },
    unit_preservation: unitPreservation,
    operation_review: {
      record_count: operationRefs.length,
      editor_intent_count: editorIntents.length,
      proposal_count: proposal ? 1 : 0,
      persisted_review_context: {
        editor_intent_count: projectSummary?.editor_intent_count ?? 0,
        proposal_count: projectSummary?.proposal_count ?? 0,
        selected_review_target_count: projectSummary?.selected_review_target_count ?? 0,
        selected_review_target_ref: projectSummary?.selected_review_target_ref ?? "not_selected"
      },
      held_for_user_acceptance_count: operationRefs.length,
      accepted_count: 0,
      rejected_count: 0,
      operation_refs: operationRefs,
      proposal_refs: proposalRefs,
      selected_review_target: selectedReviewTarget,
      accepted_model_state_mutated: false,
      operation_application_status: "not_applied",
      audit_boundary: {
        requires_user_acceptance: true,
        preview_records_do_not_apply_operations: true,
        direct_model_mutation_allowed: false
      }
    },
    loss_report: {
      categories: ["exported", "omitted", "approximated", "delegated", "unsupported", "tbd"],
      entries: [
        {
          category: "exported",
          severity: "info",
          affected_refs: [model.project.id, result.run_id],
          reason: "invented model, analysis-run refs, diagnostics, and stable IDs are represented in the review package",
          downstream_implication: "usable for local review and regression discussion only"
        },
        {
          category: "unsupported",
          severity: "warning",
          affected_refs: ["target_specific_adapter_behavior"],
          reason: "target-specific external files and commercial-solver mappings are not generated",
          downstream_implication: "use native JSON review package only; no target compatibility claim"
        },
        {
          category: "tbd",
          severity: "warning",
          affected_refs: ["physical_project_container", "public_transport_protocol"],
          reason: "physical project package/container and public transport protocol remain TBD",
          downstream_implication: "do not treat this browser-local packet as a release package format"
        }
      ],
      summary: {
        exported_count: 1,
        omitted_count: 0,
        approximated_count: 0,
        delegated_count: 0,
        unsupported_count: 1,
        tbd_count: 1
      }
    },
    validation_report: {
      package_shape_status: "review_manifest_complete",
      implementation_status: "browser_panel_review_packet_not_native_writer",
      deterministic_member_paths: true,
      deterministic_member_ordering: true,
      model_hash_status: modelHash ? "computed_local_preview_sha256" : "TBD_model_hash_not_available",
      model_hash: modelHash,
      validation_scope: "package shape, stable IDs, boundaries, and current app state only",
      protected_content_screening: "invented preview fixture policy; no protected standards payload expected",
      private_payload_screening: "private project/rule/component/material payloads are not bundled",
      release_or_professional_claim: false,
      checks: [
        "package member paths declared",
        "stable entity/result/operation refs declared",
        "native JSON unit preservation witnesses declared",
        "review-only operation refs declared when present",
        "persisted local project review-context summary carried when available",
        "loss report uses exported/omitted/approximated/delegated/unsupported/tbd categories",
        "analysis run hash refs carried when available",
        "private/protected/release/professional claim flags are false"
      ]
    },
    diagnostics,
    generation_context: {
      context_kind: "current_desktop_preview_state",
      storage_engine: storageCapability?.engine ?? "unknown_local_preview_store",
      storage_mode: projectSummary?.storage_mode ?? "not_persisted_this_session",
      network_required: storageCapability?.network_required ?? false,
      daemon_required: storageCapability?.daemon_required ?? false,
      telemetry_enabled: storageCapability?.telemetry_enabled ?? false,
      fts5_available: storageCapability?.fts5_available ?? false,
      external_file_copy: projectSummary?.copied_external_files ?? false,
      persisted_editor_intent_count: projectSummary?.editor_intent_count ?? 0,
      persisted_proposal_count: projectSummary?.proposal_count ?? 0,
      persisted_selected_review_target_count: projectSummary?.selected_review_target_count ?? 0,
      persisted_selected_review_target_ref: projectSummary?.selected_review_target_ref ?? "not_selected",
      persisted_mechanics_result_count: projectSummary?.persisted_mechanics_result_count ?? 0,
      persisted_analysis_run_count: projectSummary?.persisted_analysis_run_count ?? 0,
      persisted_analysis_run_ref: projectSummary?.persisted_analysis_run_ref ?? "not_persisted",
      repository_default_private_write: false,
      runtime_timestamp_omitted: true
    },
    run_refs: {
      model_state_ref: analysisRun.analysis_run.model_state_ref,
      analysis_run_ref: reference("AnalysisRun", analysisRun.analysis_run.run_id),
      result_envelope_ref: reference("ResultEnvelope", `result-envelope:${analysisRun.analysis_run.run_id}`),
      result_count: result.results.length,
      diagnostic_count: diagnostics.length,
      analysis_status: analysisRun.analysis_run.analysis_status,
      hash_refs: runHashRefs
    },
    private_payload_included: false,
    protected_content_included: false,
    release_or_professional_claim: false,
    compatibility_claim_made: false,
    code_compliance_claim_made: false,
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

function buildUnitPreservationEvidence({
  model,
  result
}: {
  model: PreviewModel;
  result: MechanicsResult;
}) {
  const modelQuantityWitnesses: UnitPreservationWitness[] = [];
  const resultQuantityWitnesses: UnitPreservationWitness[] = [];

  const pushModelWitness = ({
    dimension,
    fieldPath,
    quantity,
    refId,
    refType
  }: {
    dimension?: string;
    fieldPath: string;
    quantity: QuantityRecord;
    refId: string;
    refType: string;
  }) => {
    modelQuantityWitnesses.push(
      unitPreservationWitness({
        dimension,
        fieldPath,
        memberPath: "model/project.json",
        quantity,
        refId,
        refType,
        witnessPrefix: "native-unit:model"
      })
    );
  };

  for (const material of model.materials ?? []) {
    pushModelWitness({
      dimension: "stress",
      fieldPath: "elastic_modulus",
      quantity: material.elastic_modulus,
      refId: material.id,
      refType: "material"
    });
    pushModelWitness({
      dimension: "stress",
      fieldPath: "shear_modulus",
      quantity: material.shear_modulus,
      refId: material.id,
      refType: "material"
    });
    if (material.thermal_expansion_coefficient) {
      pushModelWitness({
        dimension: "thermal_expansion_coefficient",
        fieldPath: "thermal_expansion_coefficient",
        quantity: material.thermal_expansion_coefficient,
        refId: material.id,
        refType: "material"
      });
    }
  }

  for (const segment of model.pipe_segments) {
    for (const [field, quantity] of Object.entries(segment.section)) {
      pushModelWitness({
        dimension: "length",
        fieldPath: `section.${field}`,
        quantity,
        refId: segment.id,
        refType: "pipe_segment"
      });
    }
  }

  for (const component of model.components) {
    const componentQuantities = [
      { dimension: "length", fieldPath: "geometry.bend_radius", quantity: component.geometry?.bend_radius },
      { dimension: "angle", fieldPath: "geometry.bend_angle", quantity: component.geometry?.bend_angle },
      { dimension: "dimensionless", fieldPath: "modifiers.sif_user_value", quantity: component.modifiers?.sif_user_value },
      {
        dimension: "dimensionless",
        fieldPath: "modifiers.flexibility_factor_user_value",
        quantity: component.modifiers?.flexibility_factor_user_value
      }
    ];
    for (const { dimension, fieldPath, quantity } of componentQuantities) {
      if (!quantity) continue;
      pushModelWitness({
        dimension,
        fieldPath,
        quantity,
        refId: component.id,
        refType: "component"
      });
    }
  }

  for (const loadCase of model.load_cases) {
    for (const primitiveLoad of loadCase.primitive_loads ?? []) {
      const quantity = quantityRecord(primitiveLoad.magnitude);
      const primitiveId = typeof primitiveLoad.id === "string" ? primitiveLoad.id : `${loadCase.id}:primitive`;
      const dimension = typeof primitiveLoad.dimension === "string" ? primitiveLoad.dimension : undefined;
      if (!quantity) continue;
      pushModelWitness({
        dimension,
        fieldPath: "magnitude",
        quantity,
        refId: primitiveId,
        refType: "primitive_load"
      });
    }
  }

  const summaryQuantities = [
    { fieldPath: "summary.max_displacement", quantity: result.summary.max_displacement, refId: "summary:max_displacement" },
    {
      fieldPath: "summary.max_open_formula_stress",
      quantity: result.summary.max_open_formula_stress,
      refId: "summary:max_open_formula_stress"
    }
  ];
  for (const summaryQuantity of summaryQuantities) {
    const quantity = quantityRecord(summaryQuantity.quantity);
    if (!quantity) continue;
    resultQuantityWitnesses.push(
      unitPreservationWitness({
        fieldPath: summaryQuantity.fieldPath,
        memberPath: "results/result_envelope_ref.json",
        quantity,
        refId: summaryQuantity.refId,
        refType: "result_summary",
        witnessPrefix: "native-unit:result"
      })
    );
  }

  for (const resultRow of result.results) {
    resultQuantityWitnesses.push(
      unitPreservationWitness({
        fieldPath: "value",
        memberPath: "results/result_envelope_ref.json",
        quantity: { value: resultRow.value, unit: resultRow.unit },
        refId: resultRow.id,
        refType: "result_row",
        witnessPrefix: "native-unit:result"
      })
    );
  }

  return {
    schema_version: "0.1.0",
    unit_system_ref: reference("UnitSystem", "unit-system:dec-018-si-dual-display"),
    preservation_policy: "native_open_json_preserves_source_quantity_value_and_unit_fields",
    conversion_performed: false,
    target_package_units: {
      policy: "same_as_source_json_fields",
      target_family: "project_owned_native_json"
    },
    project_unit_declarations: Object.entries(model.project.units)
      .sort(([left], [right]) => left.localeCompare(right))
      .map(([dimension, unit]) => ({
        dimension,
        unit,
        source_ref: {
          ref_type: "project_units",
          ref_id: model.project.id,
          field_path: `project.units.${dimension}`
        },
        target_ref: {
          member_path: "model/project.json",
          field_path: `project.units.${dimension}`
        },
        preservation_status: "unit_declaration_preserved",
        conversion_performed: false
      })),
    model_quantity_witnesses: modelQuantityWitnesses,
    result_quantity_witnesses: resultQuantityWitnesses,
    summary: {
      project_unit_declaration_count: Object.keys(model.project.units).length,
      model_quantity_witness_count: modelQuantityWitnesses.length,
      result_quantity_witness_count: resultQuantityWitnesses.length,
      total_witness_count:
        Object.keys(model.project.units).length + modelQuantityWitnesses.length + resultQuantityWitnesses.length
    }
  };
}

function unitPreservationWitness({
  dimension,
  fieldPath,
  memberPath,
  quantity,
  refId,
  refType,
  witnessPrefix
}: {
  dimension?: string;
  fieldPath: string;
  memberPath: string;
  quantity: QuantityRecord;
  refId: string;
  refType: string;
  witnessPrefix: string;
}): UnitPreservationWitness {
  return {
    witness_id: `${witnessPrefix}:${refId}:${fieldPath}`,
    source_ref: {
      ref_type: refType,
      ref_id: refId,
      field_path: fieldPath
    },
    target_ref: {
      member_path: memberPath,
      field_path: fieldPath
    },
    source_quantity: {
      value: round(quantity.value),
      unit: quantity.unit,
      ...(dimension ? { dimension } : {})
    },
    target_quantity: {
      value: round(quantity.value),
      unit: quantity.unit,
      ...(dimension ? { dimension } : {})
    },
    conversion_performed: false,
    preservation_status: "unit_and_value_preserved",
    basis_refs: ["DEC-018", "DEL-02-02", "DEL-17-03"],
    provenance: "apps/desktop native JSON browser-preview packet generated from invented fixture state"
  };
}

export function withPackageHash(
  packet: ReturnType<typeof buildNativePackageReview>,
  packageHash: PackageHashEvidence | null
) {
  return {
    ...packet,
    validation_report: {
      ...packet.validation_report,
      package_hash_status: packageHash
        ? "computed_local_preview_sha256"
        : "TBD_canonical_package_hash_service_not_available",
      package_hash: packageHash
    }
  };
}

function member(path: string, memberKind: string, description: string, hashStatus: string) {
  return {
    path,
    member_kind: memberKind,
    deterministic_path: true,
    hash_basis: "JCS-compatible JSON when canonical payload bytes are available",
    hash_status: hashStatus,
    description
  };
}

function modelEntityRefs(model: PreviewModel): string[] {
  return [
    model.project.id,
    ...(model.materials ?? []).map((item) => item.id),
    ...model.nodes.map((item) => item.id),
    ...model.pipe_segments.map((item) => item.id),
    ...model.supports.map((item) => item.id),
    ...model.components.map((item) => item.id),
    ...model.load_cases.map((item) => item.id),
    ...(model.combinations ?? []).map((item) => item.id)
  ].sort();
}

function operationReviewRefs({
  editorIntents,
  proposal
}: {
  editorIntents: EditorOperationIntent[];
  proposal: AgentProposal | null;
}): string[] {
  return [
    ...editorIntents.map((item) => item.queue_id ?? item.operation_id),
    ...(proposal ? [proposal.operation.operation_id] : [])
  ].sort();
}

function normalizeDiagnostics(diagnostics: Diagnostic[]) {
  return diagnostics.map((item, index) => ({
    diagnostic_ref: item.id ?? `diagnostic:native-package:${index + 1}`,
    code: item.code,
    severity: item.severity,
    source: item.source ?? "desktop_preview_native_package",
    affected_refs: item.affected_refs ?? [],
    message: item.message,
    provenance: "invented preview fixture"
  }));
}

function reference(objectType: string, ref: string): ObjectRef {
  return { object_type: objectType, ref };
}

function quantityRecord(value: unknown): QuantityRecord | null {
  if (!value || typeof value !== "object") return null;
  const candidate = value as { value?: unknown; unit?: unknown };
  if (typeof candidate.value !== "number" || typeof candidate.unit !== "string") return null;
  return { value: candidate.value, unit: candidate.unit };
}

function runHashStatus(
  runHashRefs: Array<{ payload_scope: string; value: string }>,
  scope?: string
): string {
  if (!scope) return `${runHashRefs.length}_analysis_run_hash_refs_available`;
  const hash = runHashRefs.find((item) => item.payload_scope === scope);
  return hash ? `sha256:${hash.value}` : "TBD_hash_ref_not_available";
}

function nativePackageBoundary(packet: ReturnType<typeof buildNativePackageReview>): string {
  if (
    !packet.private_payload_included &&
    !packet.protected_content_included &&
    !packet.release_or_professional_claim &&
    !packet.compatibility_claim_made &&
    !packet.code_compliance_claim_made &&
    !packet.professional_boundary.software_makes_compliance_claim &&
    !packet.professional_boundary.software_makes_approval_claim
  ) {
    return "local review package only; no private payload, protected content, release claim, compatibility claim, compliance claim, or professional approval claim";
  }
  return "boundary requires review before export";
}

function safeFileToken(value: string): string {
  return value.toLowerCase().replace(/[^a-z0-9]+/g, "-").replace(/(^-|-$)/g, "") || "project";
}

function round(value: number): number {
  return Number(value.toFixed(6));
}
