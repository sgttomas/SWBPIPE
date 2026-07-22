import { Download, ShieldCheck } from "lucide-react";
import { useState } from "react";
import type { LocalStorageCapability, PreviewModel } from "../../types";
import {
  desktopTelemetryRuntimeEvidence,
  guardDesktopTelemetryAttempt,
  resolveDesktopTelemetryPolicy,
  type DesktopTelemetryAttempt
} from "../../services/telemetryPolicyService";

export function TelemetryBoundaryPanel({
  model,
  storageCapability
}: {
  model: PreviewModel;
  storageCapability: LocalStorageCapability | null;
}) {
  const [affirmativeRequestRecorded, setAffirmativeRequestRecorded] = useState(false);
  const packet = buildTelemetryBoundaryPacket({
    model,
    storageCapability,
    affirmativeRequestRecorded
  });

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
        <button
          type="button"
          className="secondary-action"
          data-testid="telemetry-affirmative-request"
          disabled={affirmativeRequestRecorded}
          onClick={() => setAffirmativeRequestRecorded(true)}
        >
          {affirmativeRequestRecorded
            ? "Telemetry review requested — remains off"
            : "Request telemetry enablement review"}
        </button>
        <ControlledExportLink
          className="report-export-link"
          data-testid="telemetry-boundary-export-link"
          download={`openpipestress-preview-telemetry-boundary-${safeFileToken(model.project.id)}.json`}
          href={jsonDataHref(packet)}
        >
          <Download size={14} aria-hidden="true" />
          Telemetry JSON
        </ControlledExportLink>
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
          label="Affirmative request"
          value={`recorded=${String(packet.affirmative_request.request_recorded)}; action=${packet.affirmative_request.action_id}; effect=${packet.affirmative_request.effect}; enabled=${String(packet.affirmative_request.telemetry_enabled)}`}
          testId="telemetry-boundary-affirmative-request"
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
        Telemetry remains default-off and no-op. This review records policy and guard state only; it does not authorize
        collection, transport, endpoint, vendor, or support-bundle upload. Acceptance and professional judgment remain
        with the responsible engineer.
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
  storageCapability,
  affirmativeRequestRecorded
}: {
  model: PreviewModel;
  storageCapability: LocalStorageCapability | null;
  affirmativeRequestRecorded: boolean;
}) {
  const requestedEnabled = affirmativeRequestRecorded;
  const telemetryPolicy = resolveDesktopTelemetryPolicy(requestedEnabled);
  const eventAttempts = telemetryEventAttempts(requestedEnabled);
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
        "apps/desktop/src/services/telemetryPolicyService.ts",
        "apps/desktop/src/features/telemetry/TelemetryBoundaryPanel.tsx"
      ]
    },
    summary: {
      telemetry_enabled: telemetryPolicy.telemetry_enabled,
      config_resolution: telemetryPolicy.reason_code,
      affirmative_request_recorded: affirmativeRequestRecorded,
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
      requested_enabled: telemetryPolicy.requested_enabled,
      explicit_opt_in: telemetryPolicy.explicit_opt_in,
      consent_surface: telemetryPolicy.consent_surface,
      approved_consent_surfaces: [],
      allowlist_approved: telemetryPolicy.allowlist_approved,
      allowlist_approval_record: "TBD",
      product_config_schema: "TBD",
      product_config_storage: "TBD",
      reason_code: telemetryPolicy.reason_code,
      config_source: affirmativeRequestRecorded
        ? "desktop_telemetry_boundary_affirmative_request"
        : "no_affirmative_telemetry_request"
    },
    affirmative_request: {
      request_recorded: affirmativeRequestRecorded,
      action_id: "request_telemetry_enablement_review",
      action_label: "Request telemetry enablement review",
      distinct_from_actions: [
        "terms_acceptance",
        "installation",
        "application_open",
        "project_open",
        "solve_request"
      ],
      effect: affirmativeRequestRecorded
        ? "request_recorded_fail_closed_pending_consent_and_allowlist"
        : "no_request_recorded",
      product_config_mutated: false,
      explicit_opt_in_granted: false,
      consent_granted: false,
      allowlist_approved: false,
      telemetry_enabled: false,
      payload_constructed: false,
      network_behavior_initialized: false
    },
    metadata_only_guard: {
      guard_present: true,
      guard_module_ref: "apps/desktop/src/services/telemetryPolicyService.ts",
      reference_guard_module_ref: "core/security/telemetry_policy",
      decision_stage: "before_payload_construction",
      metadata_only: true,
      product_config_schema: "TBD",
      runtime_integration_status: "desktop_preview_fail_closed_attempt_guard",
      helper_authorizes_runtime_telemetry: false
    },
    event_attempts: eventAttempts,
    runtime_initialization: desktopTelemetryRuntimeEvidence(),
    no_bypass_surfaces: [
      "plugins",
      "adapters",
      "import_export",
      "reports",
      "private_libraries",
      "cli_runner",
      "diagnostics"
    ],
    no_bypass_enforcement_status:
      "telemetry_panel_attempts_guarded_other_runtime_surfaces_not_wired",
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

function telemetryEventAttempts(requestedEnabled: boolean): DesktopTelemetryAttempt[] {
  return [
    metadataAttempt("app_started", "apps/desktop/src/App.tsx", ["app_version", "os_family"], requestedEnabled),
    metadataAttempt("project_opened", "apps/desktop/src/services/projectService.ts", ["storage_mode", "operation"], requestedEnabled),
    metadataAttempt("solve_requested", "apps/desktop/src/features/solve/SolvePanel.tsx", ["command", "job_state"], requestedEnabled),
    guardDesktopTelemetryAttempt({
      event_name: "report_export_attempted",
      event_version: "TBD",
      source_surface: "apps/desktop/src/features/report/ReportPanel.tsx",
      field_names: ["report_packet", "result_hash"],
      field_classes: {
        report_packet: "generated_report",
        result_hash: "hash_data"
      }
    }, requestedEnabled)
  ];
}

function metadataAttempt(eventName: string, sourceSurface: string, fieldNames: string[], requestedEnabled: boolean): DesktopTelemetryAttempt {
  return guardDesktopTelemetryAttempt({
    event_name: eventName,
    event_version: "TBD",
    source_surface: sourceSurface,
    field_names: fieldNames,
    field_classes: Object.fromEntries(fieldNames.map((fieldName) => [fieldName, "operational_metadata"]))
  }, requestedEnabled);
}

function reasonSummary(eventAttempts: DesktopTelemetryAttempt[]): string {
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
import { ControlledExportLink } from "../redaction-controls/ControlledExportLink";
