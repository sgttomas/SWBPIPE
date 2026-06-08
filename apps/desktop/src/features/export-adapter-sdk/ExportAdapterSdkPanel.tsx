import { Download, Puzzle } from "lucide-react";
import type { AnalysisRunEnvelope, MechanicsResult, PreviewModel } from "../../types";

type ExportAdapterSdkReference = {
  object_type: string;
  ref: string;
};

type ExportAdapterTarget = {
  target_id: string;
  label: string;
  document_kind: string;
  deliverable_refs: string[];
  target_family: string;
  registry_state: string;
  source_basis_state: string;
  stable_id_policy: string;
  loss_report_policy: string;
  validation_status: string;
  unresolved_tbd_refs: string[];
  preview_panel_available: boolean;
  target_support_claim: false;
  compatibility_claim: false;
  solver_validation_claim: false;
  code_compliance_claim: false;
  professional_reliance_claim: false;
};

const EXPORT_ADAPTER_SDK_VERSION = "0.1.0";
const REGISTRY_ID = "ops.export_adapter_sdk.registry_preview";
const SDK_CONTRACT_ID = "ops.export_adapter_sdk.contract_preview";
const HASH_STATUS_TBD = "TBD_browser_preview_does_not_emit_canonical_package_hash";
const SCHEMA_VALIDATION_STATUS = "desktop_preview_shape_aligned_not_runtime_json_schema_validated";

export function ExportAdapterSdkPanel({
  model,
  result,
  analysisRun
}: {
  model: PreviewModel;
  result: MechanicsResult | null;
  analysisRun: AnalysisRunEnvelope | null;
}) {
  const packet = buildExportAdapterSdkPacket({ model, result, analysisRun });

  return (
    <section
      className="panel export-adapter-sdk-panel"
      aria-label="Export adapter SDK"
      data-testid="export-adapter-sdk-panel"
    >
      <div className="panel-title">
        <Puzzle size={16} aria-hidden="true" />
        Export Adapter SDK
      </div>
      <div className="report-actions">
        <a
          className="report-export-link"
          data-testid="export-adapter-sdk-export-link"
          download={`openpipestress-preview-export-adapter-sdk-${safeFileToken(model.project.id)}.json`}
          href={jsonDataHref(packet)}
        >
          <Download size={14} aria-hidden="true" />
          Registry JSON
        </a>
        <span data-testid="export-adapter-sdk-summary">
          available; targets={packet.target_registry.targets.length}; capabilities=
          {packet.sdk_contract.capabilities.length}; validation={packet.validation_report.validation_status};
          diagnostics={packet.diagnostics.length}
        </span>
      </div>
      <div className="report-list" data-testid="export-adapter-sdk-body">
        <ExportAdapterSdkLine
          label="SDK contract"
          value={`${packet.sdk_contract.sdk_contract_status}; runtime=${packet.sdk_contract.runtime_model.plugin_runtime}; transport=${packet.sdk_contract.runtime_model.public_transport_protocol}`}
          testId="export-adapter-sdk-profile"
        />
        <ExportAdapterSdkLine
          label="Target registry"
          value={`preview_targets=${packet.target_registry.targets.length}; admitted_support_claims=${packet.target_registry.summary.admitted_support_claim_count}; template=${packet.adapter_template.template_status}`}
          testId="export-adapter-sdk-targets"
        />
        <ExportAdapterSdkLine
          label="Validation"
          value={`source_basis=${packet.validation_report.checks.source_basis_required}; stable_ids=${packet.validation_report.checks.stable_ids_required}; loss_report=${packet.validation_report.checks.loss_report_required}; schema=${packet.validation_report.schema_validation_status}`}
          testId="export-adapter-sdk-validation"
        />
        <ExportAdapterSdkLine
          label="Permissions"
          value={`taxonomy=${packet.sdk_contract.runtime_model.permission_taxonomy}; filesystem=${String(packet.sdk_contract.deny_by_default_controls.filesystem_access_granted)}; network=${String(packet.sdk_contract.deny_by_default_controls.network_access_granted)}; process=${String(packet.sdk_contract.deny_by_default_controls.process_access_granted)}; private_data=${String(packet.sdk_contract.deny_by_default_controls.private_data_access_granted)}`}
          testId="export-adapter-sdk-permissions"
        />
        <ExportAdapterSdkLine
          label="Boundary"
          value={`compatibility=${String(packet.professional_boundary.software_makes_target_compatibility_claim)}; release=${String(packet.professional_boundary.software_makes_release_claim)}; code_compliance=${String(packet.professional_boundary.software_makes_compliance_claim)}; professional_reliance=${String(packet.professional_boundary.software_creates_professional_reliance_record)}`}
          testId="export-adapter-sdk-boundary"
        />
      </div>
      <small className="report-note">
        Export adapter SDK output is a local target-registry preview only; plugin runtime, permission taxonomy, source-basis
        admission for additional targets, public API transport, target compatibility, and support claims remain governed TBDs.
      </small>
    </section>
  );
}

function ExportAdapterSdkLine({ label, value, testId }: { label: string; value: string; testId: string }) {
  return (
    <div className="report-line" data-testid={testId}>
      <span>{label}</span>
      <strong>{value}</strong>
    </div>
  );
}

function buildExportAdapterSdkPacket({
  model,
  result,
  analysisRun
}: {
  model: PreviewModel;
  result: MechanicsResult | null;
  analysisRun: AnalysisRunEnvelope | null;
}) {
  const run = analysisRun?.analysis_run;
  const sourceModelRef = reference("Model", model.project.id);
  const modelStateRef = run?.model_state_ref ?? reference("ModelState", "state:TBD");
  const analysisRunRef = run ? reference("AnalysisRun", run.run_id) : reference("AnalysisRun", "not generated");
  const targets = exportAdapterTargets();
  const diagnostics = exportAdapterDiagnostics(targets);
  const validationStatus = diagnostics.some((item) => item.severity === "blocking") ? "blocked" : "boundary_checked";

  return {
    schema_version: EXPORT_ADAPTER_SDK_VERSION,
    document_kind: "openpipestress.technical_preview.export_adapter_sdk_registry",
    deliverable_id: "DEL-17-09",
    package_id: "PKG-17",
    scope_items: ["SOW-030", "SOW-074", "SOW-075"],
    objectives: ["OBJ-009", "OBJ-017", "OBJ-018"],
    export_scope: "local_browser_download_preview",
    registry_id: REGISTRY_ID,
    sdk_contract: {
      sdk_contract_id: SDK_CONTRACT_ID,
      sdk_contract_status: "contract_level_preview",
      adapter_kind: "export_adapter_target_registry",
      source_basis_refs: [
        reference("Deliverable", "DEL-17-01"),
        reference("Deliverable", "DEL-17-02"),
        reference("Deliverable", "DEL-17-09"),
        reference("Deliverable", "DEL-02-04"),
        reference("Deliverable", "DEL-10-01"),
        reference("Deliverable", "DEL-10-02")
      ],
      capabilities: ["export_model", "export_results", "validate_payload", "emit_diagnostics"],
      runtime_model: {
        public_transport_protocol: "TBD",
        endpoint_syntax: "TBD",
        plugin_runtime: "TBD",
        permission_taxonomy: "TBD",
        sandbox_mechanism: "TBD",
        permission_grant_persistence: "TBD",
        adapter_loader: "TBD"
      },
      required_adapter_sections: [
        "target_version_basis",
        "source_basis_refs",
        "unit_policy",
        "coordinate_policy",
        "stable_id_policy",
        "loss_report_policy",
        "diagnostics_policy",
        "boundary_notes",
        "validation_checklist"
      ],
      deny_by_default_controls: denyByDefaultControls(),
      professional_boundary: professionalBoundary()
    },
    adapter_template: {
      template_id: "ops.export_adapter_sdk.additional_target_template",
      template_status: "intake_template_only",
      exact_schema_field_names: "TBD",
      sample_adapter_implementation_included: false,
      source_basis_required_before_support_claim: true,
      target_specific_writer_included: false,
      runtime_loader_included: false,
      public_api_endpoint_included: false,
      required_sections: [
        "target identification",
        "source basis and redistribution posture",
        "unit and coordinate policy",
        "stable ID carrier or sidecar map",
        "loss report taxonomy",
        "diagnostics and validation evidence",
        "privacy and protected-content screening",
        "professional boundary"
      ]
    },
    target_registry: {
      registry_id: REGISTRY_ID,
      registry_status: "desktop_preview_registry",
      targets,
      summary: {
        target_count: targets.length,
        preview_panel_available_count: targets.filter((item) => item.preview_panel_available).length,
        admitted_support_claim_count: targets.filter((item) => item.target_support_claim).length,
        source_basis_admitted_target_count: targets.filter((item) => item.source_basis_state === "source_basis_admitted").length,
        unresolved_tbd_count: targets.reduce((count, item) => count + item.unresolved_tbd_refs.length, 0)
      }
    },
    state_binding: {
      source_model_ref: sourceModelRef,
      model_state_ref: modelStateRef,
      analysis_run_ref: analysisRunRef,
      result_ref: result ? reference("MechanicsResult", result.run_id) : reference("MechanicsResult", "not generated"),
      source_model_hash_status: "TBD_browser_preview_does_not_emit_canonical_model_hash"
    },
    manifest: {
      manifest_id: `export-adapter-sdk:${safeFileToken(model.project.id)}:manifest`,
      package_members: [
        member("registry", "target_registry.json", "json", targets.length),
        member("sdk_contract", "sdk_contract.json", "json", 1),
        member("adapter_template", "adapter_template.json", "json", 1),
        member("validation_report", "validation_report.json", "json", 1),
        member("diagnostics", "diagnostics.json", "json", diagnostics.length)
      ],
      canonical_package_hash_status: HASH_STATUS_TBD
    },
    validation_report: {
      validation_status: validationStatus,
      schema_validation_status: SCHEMA_VALIDATION_STATUS,
      hash_validation_status: HASH_STATUS_TBD,
      checks: {
        common_export_contract_consumed: targets.every((item) => item.deliverable_refs.includes("DEL-17-02")),
        source_basis_required: targets.every((item) => item.source_basis_state !== "source_basis_admitted") ||
          targets.every((item) => item.unresolved_tbd_refs.length === 0),
        stable_ids_required: targets.every((item) => item.stable_id_policy.includes("stable_id") || item.stable_id_policy.includes("canonical")),
        loss_report_required: targets.every((item) => item.loss_report_policy === "mandatory"),
        deny_by_default_controls_active: denyByDefaultControlsActive(denyByDefaultControls()),
        no_support_or_compatibility_claims: targets.every(
          (item) =>
            !item.target_support_claim &&
            !item.compatibility_claim &&
            !item.solver_validation_claim &&
            !item.code_compliance_claim &&
            !item.professional_reliance_claim
        )
      },
      human_review_required: true,
      provenance: previewProvenance()
    },
    unresolved_tbd: [
      "exact adapter SDK schema file layout",
      "public API transport and endpoint syntax",
      "plugin runtime and loader mechanics",
      "permission taxonomy and sandbox implementation",
      "additional target source-basis dossiers",
      "target-specific writers and validation harnesses",
      "adapter signoff format"
    ],
    diagnostics,
    privacy: {
      classification: "invented_public_metadata",
      local_only: true,
      telemetry_allowed: false,
      private_payload_embedded: false,
      protected_payload_embedded: false,
      commercial_tool_payload_embedded: false
    },
    data_boundary: {
      public_examples_policy: model.data_boundary.public_examples_policy,
      private_payload_included: false,
      protected_content_included: false,
      commercial_tool_payload_embedded: false,
      copied_commercial_solver_file_included: false
    },
    provenance: previewProvenance(),
    professional_boundary: professionalBoundary(),
    private_payload_included: false,
    protected_content_included: false,
    target_compatibility_claim: false,
    solver_validation_claim: false,
    code_compliance_claim: false,
    professional_reliance_claim: false,
    release_or_professional_claim: false
  };
}

function exportAdapterTargets(): ExportAdapterTarget[] {
  return [
    target({
      target_id: "native_json_package",
      label: "Native JSON package",
      document_kind: "openpipestress.technical_preview.native_json_package_review",
      deliverable_refs: ["DEL-17-03", "DEL-17-02"],
      target_family: "openpipestress_native_json",
      registry_state: "project_owned_preview_surface",
      source_basis_state: "project_owned_package_contract",
      stable_id_policy: "canonical_stable_ids_directly_carried",
      validation_status: "available_after_mechanics_run_or_manifest_preview",
      unresolved_tbd_refs: ["physical_project_container"]
    }),
    target({
      target_id: "stress_neutral_csv_json_package",
      label: "Stress-neutral CSV/JSON package",
      document_kind: "openpipestress.technical_preview.stress_neutral_csv_json_package",
      deliverable_refs: ["DEL-17-06", "DEL-17-02", "DEL-08-04", "DEL-14-02"],
      target_family: "openpipestress_stress_neutral",
      registry_state: "project_owned_result_package_preview",
      source_basis_state: "project_owned_package_contract",
      stable_id_policy: "canonical_result_ids_and_sidecar_map",
      validation_status: "available_after_mechanics_run",
      unresolved_tbd_refs: ["canonical_package_hash_status"]
    }),
    target({
      target_id: "review_geometry_export",
      label: "Review geometry export",
      document_kind: "openpipestress.technical_preview.review_geometry_export",
      deliverable_refs: ["DEL-17-08", "DEL-17-02"],
      target_family: "gltf_review_geometry",
      registry_state: "visual_review_preview_surface",
      source_basis_state: "review_geometry_only_not_solver_geometry",
      stable_id_policy: "stable_id_sidecar_metadata",
      validation_status: "visual_review_only",
      unresolved_tbd_refs: ["glb_binary_writer", "viewer_compatibility"]
    }),
    target({
      target_id: "conservative_pcf_export",
      label: "Conservative PCF export",
      document_kind: "openpipestress.technical_preview.conservative_pcf_export_package",
      deliverable_refs: ["DEL-17-07", "DEL-17-02", "DEL-17-01"],
      target_family: "pcf",
      registry_state: "candidate_target_preview_surface",
      source_basis_state: "source_basis_tbd_or_partial",
      stable_id_policy: "sidecar_stable_id_map_required",
      validation_status: "blocked_missing_explicit_pcf_target_fields",
      unresolved_tbd_refs: ["target_profile_version_basis", "nominal_size_source_basis"]
    }),
    target({
      target_id: "caepipe_mbf_export",
      label: "CAEPIPE MBF export",
      document_kind: "openpipestress.technical_preview.caepipe_mbf_export_package",
      deliverable_refs: ["DEL-17-04", "DEL-17-02", "DEL-17-01"],
      target_family: "caepipe_mbf",
      registry_state: "candidate_target_smoke_subset_preview_surface",
      source_basis_state: "source_basis_tbd_or_partial",
      stable_id_policy: "sidecar_stable_id_map_until_direct_carrier_confirmed",
      validation_status: "boundary_checked",
      unresolved_tbd_refs: ["TBD-17-01-001", "TBD-17-01-002", "TBD-17-01-003"]
    })
  ];
}

function target({
  target_id,
  label,
  document_kind,
  deliverable_refs,
  target_family,
  registry_state,
  source_basis_state,
  stable_id_policy,
  validation_status,
  unresolved_tbd_refs
}: Omit<ExportAdapterTarget, "loss_report_policy" | "preview_panel_available" | "target_support_claim" | "compatibility_claim" | "solver_validation_claim" | "code_compliance_claim" | "professional_reliance_claim">): ExportAdapterTarget {
  return {
    target_id,
    label,
    document_kind,
    deliverable_refs,
    target_family,
    registry_state,
    source_basis_state,
    stable_id_policy,
    loss_report_policy: "mandatory",
    validation_status,
    unresolved_tbd_refs,
    preview_panel_available: true,
    target_support_claim: false,
    compatibility_claim: false,
    solver_validation_claim: false,
    code_compliance_claim: false,
    professional_reliance_claim: false
  };
}

function exportAdapterDiagnostics(targets: ExportAdapterTarget[]) {
  const diagnostics = [];
  const missingCommonContract = targets.filter((item) => !item.deliverable_refs.includes("DEL-17-02"));
  if (missingCommonContract.length > 0) {
    diagnostics.push({
      code: "EXPORT-ADAPTER-SDK-MISSING-COMMON-CONTRACT",
      severity: "blocking",
      message: "A target registry entry is missing the DEL-17-02 common export contract reference.",
      affected_refs: missingCommonContract.map((item) => item.target_id)
    });
  }
  const supportClaims = targets.filter((item) => item.target_support_claim || item.compatibility_claim);
  if (supportClaims.length > 0) {
    diagnostics.push({
      code: "EXPORT-ADAPTER-SDK-SUPPORT-CLAIM",
      severity: "blocking",
      message: "A target registry entry attempts to claim support or compatibility from preview metadata.",
      affected_refs: supportClaims.map((item) => item.target_id)
    });
  }
  return diagnostics;
}

function denyByDefaultControls() {
  return {
    filesystem_access_granted: false,
    network_access_granted: false,
    process_access_granted: false,
    private_data_access_granted: false,
    report_control_bypass_granted: false,
    solver_bypass_granted: false,
    rule_pack_bypass_granted: false,
    direct_storage_mutation_granted: false,
    public_api_boundary_required: true,
    schema_validation_required: true,
    unit_validation_required: true,
    provenance_required: true,
    protected_content_screening_required: true,
    export_review_required: true,
    loss_report_required: true,
    stable_id_map_required: true
  };
}

function denyByDefaultControlsActive(controls: ReturnType<typeof denyByDefaultControls>): boolean {
  return (
    !controls.filesystem_access_granted &&
    !controls.network_access_granted &&
    !controls.process_access_granted &&
    !controls.private_data_access_granted &&
    !controls.report_control_bypass_granted &&
    !controls.solver_bypass_granted &&
    !controls.rule_pack_bypass_granted &&
    !controls.direct_storage_mutation_granted &&
    controls.public_api_boundary_required &&
    controls.schema_validation_required &&
    controls.unit_validation_required &&
    controls.provenance_required &&
    controls.protected_content_screening_required &&
    controls.export_review_required &&
    controls.loss_report_required &&
    controls.stable_id_map_required
  );
}

function member(role: string, path: string, mediaType: string, recordCount: number) {
  return {
    role,
    path,
    media_type: mediaType,
    record_count: recordCount,
    hash_status: HASH_STATUS_TBD
  };
}

function reference(objectType: string, ref: string): ExportAdapterSdkReference {
  return {
    object_type: objectType,
    ref
  };
}

function professionalBoundary() {
  return {
    human_review_required: true,
    software_makes_target_support_claim: false,
    software_makes_target_compatibility_claim: false,
    software_makes_solver_validation_claim: false,
    software_makes_release_claim: false,
    software_makes_compliance_claim: false,
    software_makes_certification_claim: false,
    software_makes_sealing_claim: false,
    software_makes_approval_claim: false,
    software_makes_authentication_claim: false,
    software_creates_professional_reliance_record: false
  };
}

function previewProvenance() {
  return {
    source_name: "OpenPipeStress desktop technical preview",
    source_location: "apps/desktop/src/features/export-adapter-sdk/ExportAdapterSdkPanel.tsx",
    source_license: "project-invented metadata only",
    contributor: "OpenPipeStress app integration tranche",
    contributor_certification: "Invented public metadata; no protected standards, proprietary target files, or private project payloads.",
    redistribution_status: "invented_non_engineering_example",
    review_status: "pending"
  };
}

function jsonDataHref(payload: unknown): string {
  return `data:application/json;charset=utf-8,${encodeURIComponent(JSON.stringify(payload, null, 2))}`;
}

function safeFileToken(value: string): string {
  return value.replace(/[^a-z0-9-]+/gi, "-").replace(/^-+|-+$/g, "").toLowerCase() || "preview";
}
