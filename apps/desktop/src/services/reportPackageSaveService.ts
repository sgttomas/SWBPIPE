// File > Save Report Package… native save (DEL-08-01 Design
// `report-package-current-results.md` §2.5, §2.6). The save is split so that the
// session's last stale-work guard follows every preparation await:
// - `prepareReportPackageSave` checks the controlled route evidence and computes the
//   RFC 8785 (JCS) canonical texts of the request, the model and the run's input
//   manifest, and the request text's SHA-256 (the screening evidence names exactly
//   those bytes);
// - `invokeReportPackageSave` then sends them through `invokeWithIngress`, whose
//   `stillCurrent` guard runs synchronously immediately before every send, the
//   first and each headroom re-send. A retired basis is never sent and rejects
//   with `IngressRequestRetiredError`.
// The native command re-checks the evidence, binds the three texts (the input
// manifest to the audit input-manifest hash, whose `active_rule_packs` must be the
// package's rule packs) and screens the request and the model before the save
// dialog (`src-tauri/src/report_package_save.rs`).
import { invokeWithIngress } from "./nativeIngress";
import { canonicalJsonString, canonicalSha256Hex } from "./hashService";
import type { ControlledRouteExport, LocalFirstRouteEvidence } from "../features/redaction-controls/redactionExportControls";
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
  local_first_route_id: string;
  local_first_export_context: string;
  local_first_storage_context: string;
  local_first_action: string;
  local_first_reason_code: string;
  local_first_blocked: boolean;
  local_first_metadata_only: boolean;
  local_first_explicit_local_private_intent: boolean;
  selected_basename: string;
  path_containment: "best_effort_non_adversarial";
  limitation: string;
  /** Non-blocking findings of the native screen (for example `MODEL-LIBRARY-REF-UNRESOLVED`). */
  warnings?: Array<{ code: string; path: string; message: string }>;
};

export type ReportPackageSaveRoute =
  | { route: "tauri_report_package_save"; receipt: ReportPackageSaveReceipt }
  | { route: "unavailable_browser"; diagnostic: "REPORT-PACKAGE-SAVE-DESKTOP-ONLY" }
  | { route: "redaction_blocked"; diagnostic: string };

/** The native screening evidence (`ReportPackageSaveScreening`, `deny_unknown_fields`). */
export type ReportPackageSaveScreening = {
  route_id: string;
  export_context: "local_private";
  explicit_local_private_intent: boolean;
  blocked: boolean;
  materialization_withheld: boolean;
  lossless_required: boolean;
  exact_payload_match: boolean;
  blocking_count: number;
  decision_count: number;
  finding_count: number;
  /** `sha256:<hex>` of `requestCanonicalText`. */
  payload_sha256: string;
};

/** The native command's arguments (`save_report_package`, §2.6 item 1). */
export type ReportPackageSaveArgs = {
  requestCanonicalText: string;
  modelCanonicalText: string;
  /** The run's input manifest (`CurrentSessionInputManifest`), whose SHA-256 is the
   * request's audit input-manifest hash. A validation input only; never saved. */
  inputManifestCanonicalText: string;
  screening: ReportPackageSaveScreening;
  localFirstEvidence: LocalFirstRouteEvidence;
};

/** A prepared save: the native arguments, or the route that ends the save here. */
export type PreparedReportPackageSave =
  | { route: "ready"; args: ReportPackageSaveArgs; requestSha256: string }
  | Exclude<ReportPackageSaveRoute, { route: "tauri_report_package_save" }>;

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

function hasAllowedLocalFirstEvidence(input: ControlledRouteExport): boolean {
  const evidence = input.summary.local_first;
  return (
    evidence?.route_id === REPORT_PACKAGE_SAVE_ROUTE_ID &&
    evidence.export_context === "local_private" &&
    evidence.storage_context === "local_private" &&
    evidence.action === "include_metadata_only" &&
    evidence.reason_code === "PRIVATE_LOCAL_METADATA_ALLOWED" &&
    evidence.blocked === false &&
    evidence.metadata_only === true &&
    evidence.explicit_local_private_intent === true
  );
}

/** The preparation (§2.5, §2.6 "The webview side"): the route evidence, then the
 * canonical texts. `request` is the request the route control screened; the
 * controlled payload must be canonically equal to it (the route is lossless).
 * `inputManifest` is the run's input manifest the request was built from. */
export async function prepareReportPackageSave(
  controlled: unknown,
  model: unknown,
  request: unknown,
  inputManifest: unknown
): Promise<PreparedReportPackageSave> {
  if (!isControlledPackageRequest(controlled)) {
    return {
      route: "redaction_blocked",
      diagnostic: "REPORT-PACKAGE-REDACTION-CONTROL-REQUIRED: save requires a DREP-PACKAGE-SAVE-009 controlled request."
    };
  }
  if (!hasAllowedLocalFirstEvidence(controlled)) {
    return {
      route: "redaction_blocked",
      diagnostic: "REPORT-PACKAGE-LOCAL-FIRST-EVIDENCE-REQUIRED: save requires allowed metadata-only local-first route evidence."
    };
  }
  if (controlled.blocked || controlled.payload === null || controlled.summary.materialization_withheld) {
    return {
      route: "redaction_blocked",
      diagnostic: `REPORT-PACKAGE-REDACTION-BLOCKED: ${controlled.findings.length} finding(s) prevent package assembly or persistence.`
    };
  }
  if (!isTauriRuntime()) {
    return { route: "unavailable_browser", diagnostic: "REPORT-PACKAGE-SAVE-DESKTOP-ONLY" };
  }
  const [requestCanonicalText, screenedRequestText, hex, modelCanonicalText, inputManifestCanonicalText] = await Promise.all([
    canonicalJsonString(controlled.payload),
    canonicalJsonString(request),
    canonicalSha256Hex(controlled.payload),
    canonicalJsonString(model),
    canonicalJsonString(inputManifest)
  ]);
  if (requestCanonicalText !== screenedRequestText) {
    return {
      route: "redaction_blocked",
      diagnostic: "REPORT-PACKAGE-REDACTION-BLOCKED: the route control changed the request (CANONICAL_PAYLOAD_MATERIALIZATION_CHANGED)."
    };
  }
  const requestSha256 = `sha256:${hex}`;
  return {
    route: "ready",
    requestSha256,
    args: {
      requestCanonicalText,
      modelCanonicalText,
      inputManifestCanonicalText,
      screening: {
        route_id: controlled.summary.route_id,
        export_context: "local_private",
        explicit_local_private_intent: true,
        blocked: false,
        materialization_withheld: false,
        lossless_required: true,
        exact_payload_match: true,
        blocking_count: controlled.summary.blocking_count,
        decision_count: controlled.summary.decision_count,
        finding_count: controlled.summary.finding_count,
        payload_sha256: requestSha256
      },
      localFirstEvidence: controlled.summary.local_first!
    }
  };
}

/** The native save of a prepared request. `isCurrent` is the session's basis guard,
 * evaluated by `invokeWithIngress` immediately before each send. */
export async function invokeReportPackageSave(
  prepared: Extract<PreparedReportPackageSave, { route: "ready" }>,
  isCurrent: () => boolean
): Promise<ReportPackageSaveRoute> {
  const receipt = await invokeWithIngress<ReportPackageSaveReceipt>("save_report_package", prepared.args, { stillCurrent: isCurrent });
  return { route: "tauri_report_package_save", receipt };
}
