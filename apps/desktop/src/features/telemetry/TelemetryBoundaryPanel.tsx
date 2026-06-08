import { Download, ShieldCheck } from "lucide-react";
import type { LocalStorageCapability, PreviewModel } from "../../types";

type TelemetryAttempt = {
  event_name: string;
  event_version: string;
  source_surface: string;
  field_names: string[];
  field_classes: Record<string, string>;
  decision: {
    action: "drop_disabled" | "reject_forbidden_surface";
    reason_code: string;
    payload_constructed: false;
    network_behavior_initialized: false;
  };
};

export function TelemetryBoundaryPanel({
  model,
  storageCapability
}: {
  model: PreviewModel;
  storageCapability: LocalStorageCapability | null;
}) {
  const packet = buildTelemetryBoundaryPacket({ model, storageCapability });

  return (
    <section
      className="panel telemetry-boundary-panel"
      aria-label="Telemetry boundary review"
      data-testid="telemetry-boundary-panel"
    >
      <div className="panel-title">
        <ShieldCheck size={16} aria-hidden="true" />
        Telemetry Boundary
      </div>
      <div className="report-actions">
        <a
          className="report-export-link"
          data-testid="telemetry-boundary-export-link"
          download={`openpipestress-preview-telemetry-boundary-${safeFileToken(model.project.id)}.json`}
          href={jsonDataHref(packet)}
        >
          <Download size={14} aria-hidden="true" />
          Telemetry JSON
        </a>
        <span data-testid="telemetry-boundary-summary">
          disabled={String(!packet.summary.telemetry_enabled)}; config={packet.summary.config_resolution};
          blocked={packet.summary.blocked_event_count}; payload={String(packet.summary.payload_constructed)};
          network={String(packet.summary.network_transport_initialized)}
        </span>
      </div>
      <div className="report-list" data-testid="telemetry-boundary-body">
        <TelemetryLine
          label="Config"
          value={`requested=${String(packet.config_resolution.requested_enabled)}; opt_in=${String(packet.config_resolution.explicit_opt_in)}; consent=${packet.config_resolution.consent_surface}; allowlist=${String(packet.config_resolution.allowlist_approved)}`}
          testId="telemetry-boundary-config"
        />
        <TelemetryLine
          label="Guard"
          value={`metadata_only=${String(packet.metadata_only_guard.guard_present)}; decisions=${packet.metadata_only_guard.decision_stage}; product_config=${packet.metadata_only_guard.product_config_schema}`}
          testId="telemetry-boundary-guard"
        />
        <TelemetryLine
          label="Attempts"
          value={`attempted=${packet.summary.attempted_event_count}; allowed=${packet.summary.allowed_event_count}; blocked=${packet.summary.blocked_event_count}; reasons=${reasonSummary(packet.event_attempts)}`}
          testId="telemetry-boundary-attempts"
        />
        <TelemetryLine
          label="No-bypass"
          value={packet.no_bypass_surfaces.join(", ")}
          testId="telemetry-boundary-no-bypass"
        />
        <TelemetryLine
          label="Boundary"
          value={`endpoint=${String(packet.runtime_initialization.endpoint_initialized)}; vendor=${String(packet.runtime_initialization.vendor_initialized)}; upload_queue=${String(packet.runtime_initialization.upload_queue_initialized)}; persistence=${String(packet.runtime_initialization.telemetry_persistence_initialized)}; private_payload=${String(packet.private_payload_included)}`}
          testId="telemetry-boundary-runtime"
        />
      </div>
      <small className="report-note">
        Telemetry remains default-off and no-op in this preview. This review records policy and guard state only; it does
        not authorize collection, transport, endpoint, vendor, support-bundle upload, security certification, or
        professional/code-compliance reliance.
      </small>
    </section>
  );
}

function TelemetryLine({ label, value, testId }: { label: string; value: string; testId: string }) {
  return (
    <div className="report-line" data-testid={testId}>
      <span>{label}</span>
      <strong>{value}</strong>
    </div>
  );
}

function buildTelemetryBoundaryPacket({
  model,
  storageCapability
}: {
  model: PreviewModel;
  storageCapability: LocalStorageCapability | null;
}) {
  const eventAttempts = telemetryEventAttempts();
  return {
    schema_version: "0.1.0",
    document_kind: "openpipestress.technical_preview.telemetry_boundary_review",
    export_scope: "local_browser_download_preview",
    deliverable_id: "DEL-12-03",
    package_id: "PKG-12",
    scope_item: "SOW-037",
    objective: "OBJ-010",
    project_ref: model.project.id,
    source_basis: {
      policy_doc: "docs/security/telemetry_policy.md",
      guard_module: "core/security/telemetry_policy/",
      guard_tests: "tests/security/test_telemetry_policy.py",
      app_surfaces: [
        "apps/desktop/src/services/projectService.ts",
        "apps/desktop/src/features/telemetry/TelemetryBoundaryPanel.tsx"
      ]
    },
    summary: {
      telemetry_enabled: Boolean(storageCapability?.telemetry_enabled),
      config_resolution: "absent_or_preview_config_resolves_disabled",
      attempted_event_count: eventAttempts.length,
      allowed_event_count: 0,
      blocked_event_count: eventAttempts.length,
      payload_constructed: false,
      network_transport_initialized: false,
      private_payload_included: false,
      protected_content_included: false,
      release_or_professional_claim: false
    },
    config_resolution: {
      requested_enabled: false,
      explicit_opt_in: false,
      consent_surface: "TBD",
      approved_consent_surfaces: [],
      allowlist_approved: false,
      allowlist_approval_record: "TBD",
      product_config_schema: "TBD",
      product_config_storage: "TBD",
      reason_code: "telemetry_disabled_by_default",
      config_source: storageCapability ? "desktop_local_storage_capability" : "storage_capability_pending"
    },
    metadata_only_guard: {
      guard_present: true,
      guard_module_ref: "core/security/telemetry_policy",
      decision_stage: "before_payload_construction",
      metadata_only: true,
      product_config_schema: "TBD",
      runtime_integration_status: "desktop_preview_policy_surface_only",
      helper_authorizes_runtime_telemetry: false
    },
    event_attempts: eventAttempts,
    runtime_initialization: {
      network_transport_initialized: false,
      endpoint_initialized: false,
      vendor_initialized: false,
      upload_queue_initialized: false,
      upload_job_initialized: false,
      telemetry_persistence_initialized: false,
      external_service_client_initialized: false,
      background_job_initialized: false
    },
    no_bypass_surfaces: [
      "plugins",
      "adapters",
      "import_export",
      "reports",
      "private_libraries",
      "cli_runner",
      "diagnostics"
    ],
    forbidden_payload_classes: [
      "private_project_models",
      "code_specific_rule_data",
      "private_rule_packs",
      "private_material_or_component_libraries",
      "generated_reports_and_exports",
      "model_hashes",
      "local_file_paths",
      "secrets_and_credentials",
      "protected_standards_content",
      "professional_or_code_compliance_claims"
    ],
    open_decisions: [
      "product configuration schema",
      "configuration storage location",
      "consent UI or CLI surface",
      "endpoint vendor transport and retention policy",
      "concrete event schema and human-approved allowlist",
      "support bundle workflow"
    ],
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

function telemetryEventAttempts(): TelemetryAttempt[] {
  return [
    disabledAttempt("app_started", "apps/desktop/src/App.tsx", ["app_version", "os_family"]),
    disabledAttempt("project_opened", "apps/desktop/src/services/projectService.ts", ["storage_mode", "operation"]),
    disabledAttempt("solve_requested", "apps/desktop/src/features/solve/SolvePanel.tsx", ["command", "job_state"]),
    {
      event_name: "report_export_attempted",
      event_version: "TBD",
      source_surface: "apps/desktop/src/features/report/ReportPanel.tsx",
      field_names: ["report_packet", "result_hash"],
      field_classes: {
        report_packet: "generated_report",
        result_hash: "hash_data"
      },
      decision: {
        action: "reject_forbidden_surface",
        reason_code: "telemetry_forbidden_field_class",
        payload_constructed: false,
        network_behavior_initialized: false
      }
    }
  ];
}

function disabledAttempt(eventName: string, sourceSurface: string, fieldNames: string[]): TelemetryAttempt {
  return {
    event_name: eventName,
    event_version: "TBD",
    source_surface: sourceSurface,
    field_names: fieldNames,
    field_classes: Object.fromEntries(fieldNames.map((fieldName) => [fieldName, "operational_metadata"])),
    decision: {
      action: "drop_disabled",
      reason_code: "telemetry_disabled_no_opt_in_or_allowlist",
      payload_constructed: false,
      network_behavior_initialized: false
    }
  };
}

function reasonSummary(eventAttempts: TelemetryAttempt[]): string {
  return Array.from(new Set(eventAttempts.map((attempt) => attempt.decision.reason_code))).join(",");
}

function professionalBoundary() {
  return {
    human_review_required: true,
    software_makes_compliance_claim: false,
    software_makes_certification_claim: false,
    software_makes_security_certification_claim: false,
    software_makes_sealing_claim: false,
    software_makes_approval_claim: false,
    software_makes_authentication_claim: false
  };
}

function jsonDataHref(payload: unknown): string {
  return `data:application/json;charset=utf-8,${encodeURIComponent(JSON.stringify(payload, null, 2))}`;
}

function safeFileToken(value: string): string {
  return value.replace(/[^a-z0-9]+/gi, "-").replace(/^-+|-+$/g, "").toLowerCase();
}
