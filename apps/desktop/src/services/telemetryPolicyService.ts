export type DesktopTelemetryAttempt = {
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

export type DesktopTelemetryAttemptInput = {
  event_name: string;
  event_version?: string;
  source_surface: string;
  field_names: string[];
  field_classes: Record<string, string>;
  [key: string]: unknown;
};

const FORBIDDEN_ATTEMPT_KEYS = new Set([
  "body",
  "data",
  "endpoint",
  "network_transport",
  "payload",
  "persistence",
  "telemetry_persistence",
  "upload_job",
  "upload_queue",
  "vendor"
]);

const FORBIDDEN_FIELD_CLASSES = new Set([
  "generated_report",
  "hash_data",
  "local_path",
  "private_project_data",
  "professional_claim",
  "protected_content",
  "secret_data"
]);

export function resolveDesktopTelemetryPolicy(requestedEnabled: boolean) {
  return {
    telemetry_enabled: false,
    requested_enabled: requestedEnabled,
    explicit_opt_in: false,
    consent_surface: "TBD",
    allowlist_approved: false,
    reason_code: requestedEnabled
      ? "telemetry_requested_without_affirmative_opt_in_or_allowlist"
      : "telemetry_disabled_by_default",
    payload_constructed: false,
    network_transport_initialized: false,
    telemetry_persistence_initialized: false
  } as const;
}

export function guardDesktopTelemetryAttempt(
  attempt: DesktopTelemetryAttemptInput,
  requestedEnabled: boolean
): DesktopTelemetryAttempt {
  const policy = resolveDesktopTelemetryPolicy(requestedEnabled);
  const hasPayloadOrRuntimeSurface = Object.keys(attempt).some((key) =>
    FORBIDDEN_ATTEMPT_KEYS.has(key)
  );
  const hasForbiddenField = Object.values(attempt.field_classes).some((fieldClass) =>
    FORBIDDEN_FIELD_CLASSES.has(fieldClass)
  );
  const forbiddenEvent = /(private|protected|report|rule_pack|secret)/i.test(
    attempt.event_name
  );
  const rejected = hasPayloadOrRuntimeSurface || hasForbiddenField || forbiddenEvent;

  return {
    event_name: attempt.event_name,
    event_version: attempt.event_version ?? "TBD",
    source_surface: attempt.source_surface,
    field_names: [...attempt.field_names],
    field_classes: { ...attempt.field_classes },
    decision: {
      action: rejected ? "reject_forbidden_surface" : "drop_disabled",
      reason_code: rejected
        ? hasPayloadOrRuntimeSurface
          ? "telemetry_payload_or_runtime_surface_rejected"
          : "telemetry_forbidden_field_class"
        : policy.reason_code,
      payload_constructed: false,
      network_behavior_initialized: false
    }
  };
}

export function desktopTelemetryRuntimeEvidence() {
  return {
    network_transport_initialized: false,
    endpoint_initialized: false,
    vendor_initialized: false,
    upload_queue_initialized: false,
    upload_job_initialized: false,
    telemetry_persistence_initialized: false,
    external_service_client_initialized: false,
    background_job_initialized: false
  } as const;
}
