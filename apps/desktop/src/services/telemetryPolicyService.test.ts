import { describe, expect, it } from "vitest";
import {
  desktopTelemetryRuntimeEvidence,
  guardDesktopTelemetryAttempt,
  resolveDesktopTelemetryPolicy,
  type DesktopTelemetryAttemptInput
} from "./telemetryPolicyService";

function metadataAttempt(extra: Record<string, unknown> = {}): DesktopTelemetryAttemptInput {
  return {
    event_name: "app_started",
    event_version: "TBD",
    source_surface: "test",
    field_names: ["app_version"],
    field_classes: { app_version: "operational_metadata" },
    ...extra
  } as DesktopTelemetryAttemptInput;
}

describe("desktop private-by-default telemetry policy seam", () => {
  it("resolves absent/default and capability-requested states to disabled", () => {
    expect(resolveDesktopTelemetryPolicy(false)).toMatchObject({
      telemetry_enabled: false,
      requested_enabled: false,
      explicit_opt_in: false,
      allowlist_approved: false,
      network_transport_initialized: false,
      telemetry_persistence_initialized: false
    });
    expect(resolveDesktopTelemetryPolicy(true)).toMatchObject({
      telemetry_enabled: false,
      requested_enabled: true,
      explicit_opt_in: false,
      allowlist_approved: false,
      reason_code: "telemetry_requested_without_affirmative_opt_in_or_allowlist"
    });
  });

  it("drops metadata locally and rejects payload, runtime, and forbidden-field bypass attempts", () => {
    const disabled = guardDesktopTelemetryAttempt(metadataAttempt(), false);
    const requestedWithoutConsent = guardDesktopTelemetryAttempt(metadataAttempt(), true);
    const payloadBypass = guardDesktopTelemetryAttempt(
      metadataAttempt({ payload: { app_version: "invented" } }),
      false
    );
    const persistenceBypass = guardDesktopTelemetryAttempt(
      metadataAttempt({ telemetry_persistence: "local.sqlite" }),
      false
    );
    const reportBypass = guardDesktopTelemetryAttempt(
      {
        ...metadataAttempt(),
        event_name: "report_export_attempted",
        field_classes: { report_packet: "generated_report" },
        field_names: ["report_packet"]
      },
      false
    );

    expect(disabled.decision.action).toBe("drop_disabled");
    expect(requestedWithoutConsent.decision.action).toBe("drop_disabled");
    for (const rejected of [payloadBypass, persistenceBypass, reportBypass]) {
      expect(rejected.decision.action).toBe("reject_forbidden_surface");
      expect(rejected.decision.payload_constructed).toBe(false);
      expect(rejected.decision.network_behavior_initialized).toBe(false);
    }
  });

  it("exposes only negative runtime initialization evidence", () => {
    expect(Object.values(desktopTelemetryRuntimeEvidence())).toEqual([
      false,
      false,
      false,
      false,
      false,
      false,
      false,
      false
    ]);
  });
});
