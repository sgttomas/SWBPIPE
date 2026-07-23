import { invoke } from "@tauri-apps/api/core";
import type { ControlledRouteExport } from "../features/redaction-controls/redactionExportControls";
import { REPORT_PACKAGE_SAVE_ROUTE_ID } from "../features/report/reportRedactionProjector";

export type ReportPackageMemberSummary = {
  role: string;
  file_name: string;
  sha256_hex: string;
  byte_length: number;
};

export type ReportPackageSaveReceipt = {
  outcome: "saved" | "cancelled";
  stage: string;
  code: string;
  container_file_name: string;
  byte_count: number;
  package_identity_sha256_hex: string;
  container_sha256_hex: string;
  members: ReportPackageMemberSummary[];
  replaced_existing: boolean;
  durability: "durable" | "saved_durability_uncertain" | "not_applicable";
  redaction_route_id: string;
  redaction_decision_count: number;
  redaction_finding_count: number;
  redaction_blocking_count: number;
  selected_basename: string;
  path_containment: "best_effort_non_adversarial";
  limitation: string;
};

export type ReportPackageSaveRoute =
  | { route: "tauri_report_package_save"; receipt: ReportPackageSaveReceipt }
  | { route: "unavailable_browser"; diagnostic: "REPORT-PACKAGE-SAVE-DESKTOP-ONLY" }
  | { route: "redaction_blocked"; diagnostic: string };

function isTauriRuntime(): boolean {
  return typeof window !== "undefined" && "__TAURI_INTERNALS__" in window;
}

function isControlledPackageRequest(value: unknown): value is ControlledRouteExport {
  if (typeof value !== "object" || value === null) return false;
  const candidate = value as ControlledRouteExport;
  return (
    candidate.summary?.route_id === REPORT_PACKAGE_SAVE_ROUTE_ID &&
    typeof candidate.blocked === "boolean" &&
    Array.isArray(candidate.findings) &&
    Array.isArray(candidate.decisions)
  );
}

export async function saveReportPackage(input: unknown): Promise<ReportPackageSaveRoute> {
  if (!isControlledPackageRequest(input)) {
    return {
      route: "redaction_blocked",
      diagnostic:
        "REPORT-PACKAGE-REDACTION-CONTROL-REQUIRED: save requires a DREP-PACKAGE-SAVE-009 controlled request."
    };
  }
  if (input.blocked || input.payload === null) {
    return {
      route: "redaction_blocked",
      diagnostic: `REPORT-PACKAGE-REDACTION-BLOCKED: ${input.findings.length} finding(s) prevent package assembly or persistence.`
    };
  }
  if (!isTauriRuntime()) {
    return { route: "unavailable_browser", diagnostic: "REPORT-PACKAGE-SAVE-DESKTOP-ONLY" };
  }
  const receipt = await invoke<ReportPackageSaveReceipt>("save_report_package", {
    request: input.payload,
    redactionEvidence: {
      route_id: input.summary.route_id,
      decision_count: input.summary.decision_count,
      finding_count: input.summary.finding_count,
      blocking_count: input.summary.blocking_count
    }
  });
  return { route: "tauri_report_package_save", receipt };
}
