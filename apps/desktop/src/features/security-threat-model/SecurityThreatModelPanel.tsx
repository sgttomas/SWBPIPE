import { Download, ShieldAlert } from "lucide-react";
import type { LocalStorageCapability, PreviewModel } from "../../types";

type ThreatReviewRow = {
  threat_id: string;
  surface: string;
  initial_risk: "High" | "Medium/High";
  required_posture: string;
};

export function SecurityThreatModelPanel({
  model,
  storageCapability
}: {
  model: PreviewModel;
  storageCapability: LocalStorageCapability | null;
}) {
  const packet = buildSecurityThreatModelPacket({ model, storageCapability });

  return (
    <section
      className="panel security-threat-model-panel"
      aria-label="Security threat model review"
      data-testid="security-threat-model-panel"
    >
      <div className="panel-title">
        <ShieldAlert size={16} aria-hidden="true" />
        Threat Model Review
      </div>
      <div className="report-actions">
        <a
          className="report-export-link"
          data-testid="security-threat-model-export-link"
          download={`openpipestress-preview-security-threat-model-${safeFileToken(model.project.id)}.json`}
          href={jsonDataHref(packet)}
        >
          <Download size={14} aria-hidden="true" />
          Threat JSON
        </a>
        <span data-testid="security-threat-model-summary">
          threats={packet.summary.threat_count}; high={packet.summary.high_risk_count};
          local_first={String(packet.summary.local_first)}; private_payload=
          {String(packet.private_payload_included)}
        </span>
      </div>
      <div className="report-list" data-testid="security-threat-model-body">
        <ThreatLine
          label="Source"
          value={`${packet.source_basis.policy_doc}; tests=${packet.source_basis.guard_tests}`}
          testId="security-threat-model-source"
        />
        <ThreatLine
          label="Coverage"
          value={`boundaries=${packet.summary.trust_boundary_count}; assets=${packet.summary.asset_class_count}; workflows=${packet.summary.export_workflow_count}`}
          testId="security-threat-model-coverage"
        />
        <ThreatLine
          label="No-bypass"
          value={packet.no_bypass_controls.required_controls.join(", ")}
          testId="security-threat-model-controls"
        />
        <ThreatLine
          label="Unit no-bypass"
          value={`unit_checks=${String(packet.unit_policy_evidence.unit_checks_required)}; workflows=${packet.unit_policy_evidence.export_workflow_count}; conversion=${String(packet.unit_policy_evidence.conversion_performed)}; certification=${String(packet.security_certification_claim)}`}
          testId="security-threat-model-unit-policy"
        />
        <ThreatLine
          label="Open decisions"
          value={`tbd=${packet.summary.tbd_decision_count}; plugin=${packet.open_decisions.plugin_permission_model}; telemetry=${packet.open_decisions.telemetry_event_schema}`}
          testId="security-threat-model-open-decisions"
        />
        <ThreatLine
          label="Boundary"
          value={`telemetry=${String(packet.summary.telemetry_default_off)}; security_certification=${String(packet.security_certification_claim)}; professional=${String(packet.release_or_professional_claim)}`}
          testId="security-threat-model-boundary"
        />
      </div>
      <small className="report-note">
        Threat model review is local preview evidence only. It summarizes known risks and required controls; it does not
        prove security sufficiency, authorize telemetry, certify release readiness, or create professional/code-compliance
        reliance.
      </small>
    </section>
  );
}

function ThreatLine({ label, value, testId }: { label: string; value: string; testId: string }) {
  return (
    <div className="report-line" data-testid={testId}>
      <span>{label}</span>
      <strong>{value}</strong>
    </div>
  );
}

function buildSecurityThreatModelPacket({
  model,
  storageCapability
}: {
  model: PreviewModel;
  storageCapability: LocalStorageCapability | null;
}) {
  const threats = threatInventory();
  const openDecisionValues = Object.values(openDecisions());
  const unitPolicyEvidence = securityThreatModelUnitPolicyEvidence();

  return {
    schema_version: "0.1.0",
    document_kind: "openpipestress.technical_preview.security_threat_model_review",
    export_scope: "local_browser_download_preview",
    deliverable_id: "DEL-12-05",
    package_id: "PKG-12",
    scope_item: "SOW-040",
    objective: "OBJ-010",
    project_ref: model.project.id,
    source_basis: {
      policy_doc: "docs/security/threat_model.md",
      guard_tests: "tests/test_security_threat_model.py",
      deliverable_context:
        "execution/PKG-12_Security, Privacy, and Private Data Handling/1_Working/DEL-12-05_Security threat model/_CONTEXT.md",
      app_surfaces: [
        "apps/desktop/src/features/security-threat-model/SecurityThreatModelPanel.tsx",
        "apps/desktop/src/features/export-review/ExportReviewPanel.tsx",
        "apps/desktop/src/features/report/ReportPanel.tsx",
        "apps/desktop/src/features/report-lint/ReportLintPanel.tsx"
      ]
    },
    summary: {
      threat_count: threats.length,
      high_risk_count: threats.filter((item) => item.initial_risk === "High").length,
      medium_high_risk_count: threats.filter((item) => item.initial_risk === "Medium/High").length,
      trust_boundary_count: trustBoundaries().length,
      asset_class_count: assetClasses().length,
      export_workflow_count: exportWorkflows().length,
      open_decision_count: openDecisionValues.length,
      tbd_decision_count: openDecisionValues.filter((value) => value === "TBD").length,
      local_first: true,
      telemetry_default_off: true,
      private_payload_included: false,
      protected_content_included: false,
      release_or_professional_claim: false
    },
    asset_classes: assetClasses(),
    trust_boundaries: trustBoundaries(),
    threat_inventory: threats,
    export_workflows: exportWorkflows(),
    no_bypass_controls: {
      required_controls: [
        "schema_validation",
        "unit_checks",
        "provenance",
        "privacy",
        "protected_content",
        "diagnostics",
        "rule_sandbox",
        "report_boundaries",
        "human_review"
      ],
      direct_sql_allowed: false,
      raw_sqlite_handle_allowed: false,
      storage_bypass_allowed: false,
      telemetry_bypass_allowed: false,
      plugin_manifest_grants_runtime_access: false
    },
    runtime_control_evidence: {
      telemetry_guard_seam:
        "apps/desktop/src/services/telemetryPolicyService.ts",
      telemetry_attempt_guarded_before_payload: true,
      telemetry_network_or_persistence_implemented: false,
      plugin_adapter_report_export_interception_implemented: false,
      diagnostic_classes_referenced: [
        "PROVENANCE_WARNING",
        "ASSUMPTION_WARNING",
        "IP_BOUNDARY_WARNING",
        "PRIVACY_WARNING"
      ],
      result_envelope_binding:
        "threat_model_reference_only_not_runtime_security_closure"
    },
    unit_policy_evidence: unitPolicyEvidence,
    open_decisions: openDecisions(),
    storage_capability_ref: storageCapability
      ? {
          engine: storageCapability.engine,
          network_required: storageCapability.network_required,
          daemon_required: storageCapability.daemon_required,
          telemetry_enabled: storageCapability.telemetry_enabled,
          path_policy: storageCapability.path_policy
        }
      : null,
    private_payload_included: false,
    protected_content_included: false,
    release_or_professional_claim: false,
    security_certification_claim: false,
    professional_boundary: professionalBoundary()
  };
}

function securityThreatModelUnitPolicyEvidence() {
  return {
    evidence_id: "unit-policy-evidence:security-threat-model-no-bypass",
    unit_policy: "security_threat_model_requires_unit_checks_no_bypass_for_unit_bearing_workflows",
    unit_checks_required: true,
    unit_bearing_workflow_refs: [
      "common_export_packages_target_profiles_native_open_json",
      "caepipe_mbf_and_external_harness_evidence",
      "stress_neutral_csv_json",
      "conservative_pcf_subset",
      "glb_gltf_review_geometry",
      "export_adapter_sdk_and_additional_targets"
    ],
    export_workflow_count: exportWorkflows().length,
    conversion_performed: false,
    target_writer_invoked: false,
    security_certification_claim: false,
    decision_basis_refs: [
      reference("Decision", "DEC-018"),
      reference("Deliverable", "DEL-02-02"),
      reference("Deliverable", "DEL-12-05")
    ],
    source: "apps/desktop/src/features/security-threat-model/SecurityThreatModelPanel.tsx"
  };
}

function assetClasses() {
  return [
    "project_store_and_model_data",
    "rule_packs",
    "material_and_component_libraries",
    "reports_and_result_exports",
    "diagnostics_logs_and_bug_reports",
    "plugin_and_adapter_manifests",
    "import_export_and_local_fea_handoff_data",
    "sca_004_export_packages",
    "secrets_and_credentials",
    "build_and_release_artifacts"
  ];
}

function trustBoundaries() {
  return [
    "local_filesystem",
    "public_repository",
    "report_export_share",
    "target_export_packages",
    "external_caepipe_or_harness_execution",
    "bug_reports_and_diagnostics",
    "telemetry",
    "plugin_import_export_fea_handoff",
    "rule_evaluator",
    "build_and_release_supply_chain"
  ];
}

function exportWorkflows() {
  return [
    "common_export_packages_target_profiles_native_open_json",
    "caepipe_mbf_and_external_harness_evidence",
    "stress_neutral_csv_json",
    "conservative_pcf_subset",
    "glb_gltf_review_geometry",
    "export_adapter_sdk_and_additional_targets"
  ];
}

function threatInventory(): ThreatReviewRow[] {
  return [
    threat("STM-001", "local_storage", "High", "private_paths_and_public_example_exclusion"),
    threat("STM-002", "report_export_share", "High", "explicit_user_action_and_redaction_tbd"),
    threat("STM-003", "bug_reports", "High", "redacted_default_bundle_and_attachment_review"),
    threat("STM-004", "telemetry", "High", "disabled_by_default_and_payload_selection"),
    threat("STM-005", "plugins_and_adapters", "High", "deny_by_default_manifest_and_no_bypass"),
    threat("STM-006", "imports_and_contributions", "High", "provenance_and_review_disposition"),
    threat("STM-007", "rule_evaluator", "High", "sandboxed_deterministic_unit_aware_evaluation"),
    threat("STM-008", "public_examples", "High", "invented_or_public_permissive_examples"),
    threat("STM-009", "secrets", "High", "secret_values_excluded_and_storage_policy_tbd"),
    threat("STM-010", "hash_provenance_spoofing", "Medium/High", "canonical_hash_basis_and_rereview"),
    threat("STM-011", "build_release_supply_chain", "Medium/High", "dependency_license_and_integrity_review"),
    threat("STM-012", "authority_overclaim", "Medium/High", "professional_boundary_and_prohibited_claim_scans"),
    threat("STM-013", "export_target_profiles", "High", "source_basis_and_no_target_compatibility_claim"),
    threat("STM-014", "stable_id_maps_loss_reports", "High", "authoritative_sidecars_and_blocking_diagnostics"),
    threat("STM-015", "caepipe_mbf_external_harness", "High", "no_bundled_solver_or_license_bypass"),
    threat("STM-016", "stress_neutral_csv_json", "High", "unit_dimension_and_diagnostic_export_semantics"),
    threat("STM-017", "conservative_pcf", "Medium/High", "explicit_fields_and_loss_reporting"),
    threat("STM-018", "glb_gltf_review_geometry", "Medium/High", "visual_review_only_and_metadata_minimization"),
    threat("STM-019", "export_adapter_sdk", "High", "denied_runtime_grants_until_admission_review")
  ];
}

function threat(
  threatId: string,
  surface: string,
  initialRisk: ThreatReviewRow["initial_risk"],
  requiredPosture: string
): ThreatReviewRow {
  return {
    threat_id: threatId,
    surface,
    initial_risk: initialRisk,
    required_posture: requiredPosture
  };
}

function openDecisions() {
  return {
    storage_substrate: "SCA-003_local_sqlite_backed_store_selected",
    storage_roots: "TBD",
    schema_and_db_migration: "TBD",
    private_library_encryption: "TBD",
    secret_storage: "TBD",
    plugin_permission_model: "TBD",
    public_api_transport_and_formats: "TBD",
    sca004_target_profile_basis: "TBD",
    external_harness_invocation: "TBD",
    export_adapter_sdk_runtime_grants: "TBD",
    redaction_and_bug_report_bundle: "TBD",
    telemetry_event_schema: "TBD",
    cloud_exception_workflow: "TBD",
    dependency_signing_and_reproducible_build: "TBD"
  };
}

function professionalBoundary() {
  return {
    human_review_required: true,
    software_makes_compliance_claim: false,
    software_makes_security_certification_claim: false,
    software_makes_certification_claim: false,
    software_makes_sealing_claim: false,
    software_makes_approval_claim: false,
    software_makes_authentication_claim: false
  };
}

function reference(objectType: string, ref: string) {
  return {
    object_type: objectType,
    ref
  };
}

function jsonDataHref(payload: unknown): string {
  return `data:application/json;charset=utf-8,${encodeURIComponent(JSON.stringify(payload, null, 2))}`;
}

function safeFileToken(value: string): string {
  return value.replace(/[^a-z0-9]+/gi, "-").replace(/^-+|-+$/g, "").toLowerCase();
}
