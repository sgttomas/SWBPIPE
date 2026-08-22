import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import type { ControlledRouteExport } from "../features/redaction-controls/redactionExportControls";
import { saveReportPackage } from "./reportPackageSaveService";

const invokeMock = vi.hoisted(() => vi.fn());
vi.mock("@tauri-apps/api/core", () => ({ invoke: invokeMock }));

function controlledRequest(): ControlledRouteExport {
  return {
    payload: { package_id: "invented" },
    decisions: [],
    findings: [],
    blocked: false,
    summary: {
      route_id: "DREP-PACKAGE-SAVE-009",
      decision_count: 3,
      finding_count: 1,
      redacted_count: 0,
      omitted_count: 0,
      warning_count: 1,
      blocking_count: 0,
      cloud_transmission_attempted: false as const,
      professional_claims_made: false as const,
      materialization_withheld: false,
      local_first: {
        route_id: "DREP-PACKAGE-SAVE-009",
        export_context: "local_private",
        storage_context: "local_private",
        action: "include_metadata_only" as const,
        reason_code: "PRIVATE_LOCAL_METADATA_ALLOWED" as const,
        blocked: false,
        metadata_only: true as const,
        explicit_local_private_intent: true
      }
    }
  };
}

beforeEach(() => invokeMock.mockReset());
afterEach(() => delete (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__);

describe("reportPackageSaveService", () => {
  it("reports honest desktop-only browser routing without invoke or download", async () => {
    await expect(saveReportPackage(controlledRequest())).resolves.toEqual({
      route: "unavailable_browser",
      diagnostic: "REPORT-PACKAGE-SAVE-DESKTOP-ONLY"
    });
    expect(invokeMock).not.toHaveBeenCalled();
  });

  it("refuses raw and blocked requests before native invoke", async () => {
    expect((await saveReportPackage({ package_id: "raw" })).route).toBe("redaction_blocked");
    const blocked = controlledRequest();
    blocked.blocked = true;
    blocked.payload = null as never;
    expect((await saveReportPackage(blocked)).route).toBe("redaction_blocked");
    expect(invokeMock).not.toHaveBeenCalled();
  });

  it("refuses missing or blocked local-first evidence before native invoke", async () => {
    const missing = controlledRequest();
    delete missing.summary.local_first;
    await expect(saveReportPackage(missing)).resolves.toEqual({
      route: "redaction_blocked",
      diagnostic:
        "REPORT-PACKAGE-LOCAL-FIRST-EVIDENCE-REQUIRED: save requires allowed metadata-only local-first route evidence."
    });

    const blocked = controlledRequest();
    blocked.summary.local_first!.blocked = true;
    blocked.summary.local_first!.action = "block_storage";
    blocked.summary.local_first!.reason_code = "LOCAL_PRIVATE_INTENT_REQUIRED";
    expect((await saveReportPackage(blocked)).route).toBe("redaction_blocked");
    expect(invokeMock).not.toHaveBeenCalled();
  });

  it("invokes the owned native command and preserves a cancellation receipt", async () => {
    (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__ = {};
    const receipt = {
      outcome: "cancelled",
      stage: "picker",
      code: "REPORT-PACKAGE-SAVE-CANCELLED",
      container_file_name: "invented.opsproj",
      byte_count: 0,
      package_identity_sha256_hex: "a".repeat(64),
      container_sha256_hex: "b".repeat(64),
      members: [],
      replaced_existing: false,
      durability: "not_applicable",
      redaction_route_id: "DREP-PACKAGE-SAVE-009",
      redaction_decision_count: 3,
      redaction_finding_count: 1,
      redaction_blocking_count: 0,
      local_first_route_id: "DREP-PACKAGE-SAVE-009",
      local_first_export_context: "local_private",
      local_first_storage_context: "local_private",
      local_first_action: "include_metadata_only",
      local_first_reason_code: "PRIVATE_LOCAL_METADATA_ALLOWED",
      local_first_blocked: false,
      local_first_metadata_only: true,
      local_first_explicit_local_private_intent: true,
      selected_basename: "",
      path_containment: "best_effort_non_adversarial",
      limitation: "Invented TOCTOU limitation"
    };
    invokeMock.mockResolvedValue(receipt);

    const route = await saveReportPackage(controlledRequest());

    expect(invokeMock).toHaveBeenCalledWith("save_report_package", {
      request: { package_id: "invented" },
      redactionEvidence: {
        route_id: "DREP-PACKAGE-SAVE-009",
        decision_count: 3,
        finding_count: 1,
        blocking_count: 0
      },
      localFirstEvidence: controlledRequest().summary.local_first
    });
    expect(route).toEqual({ route: "tauri_report_package_save", receipt });
  });
});
