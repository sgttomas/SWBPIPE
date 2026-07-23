import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { saveReportPackage } from "./reportPackageSaveService";

const invokeMock = vi.hoisted(() => vi.fn());
vi.mock("@tauri-apps/api/core", () => ({ invoke: invokeMock }));

function controlledRequest() {
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
      materialization_withheld: false
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
      }
    });
    expect(route).toEqual({ route: "tauri_report_package_save", receipt });
  });
});
