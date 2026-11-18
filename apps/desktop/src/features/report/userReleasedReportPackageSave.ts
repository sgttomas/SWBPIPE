// File > Save Shareable Report Package… (W-18b desktop slice; DEL-08-01 Design
// `report-user-release.md` §2.2, §2.3 "Session", §2.5). Route `DREP-PACKAGE-USER-RELEASE-010`
// of the one native command, `save_report_package`:
// - the run record sent is the solve's own transient record, built with no rule-check
//   aggregate and no rule-pack exclusion (`buildAnalysisRunPreview(result, { inputManifest })`);
// - the request is `buildUserReleasedReportPackageRequest`'s, with the record bound to the
//   session and the user's confirmation time;
// - the user-release projector screens the request and the model with the wrapper's
//   user-release intent (`controlUserReleasedReportPackageRequest`), lossless;
// - the native arguments carry route 010's screening and local-first evidence, the four
//   canonical texts (request, model, input manifest, run record) and, for an exact source,
//   W-11's resubmitted texts.
// Native admission derives the whole request from its bound basis and decides. Nothing here
// persists the selection, records an acceptance or issuance, or sends anything anywhere.
// Standard claim fence applies (claims taxonomy per DEC-081).
import { invokeWithIngress } from "../../services/nativeIngress";
import { canonicalJsonString, canonicalSha256Hex } from "../../services/hashService";
import { buildAnalysisRunPreview } from "../../services/previewService";
import type { CurrentSessionInputManifestEvidence } from "../../services/inputManifestService";
import type { ReportPackageSaveReceipt, ReportPackageSaveRoute } from "../../services/reportPackageSaveService";
import type { MechanicsResult, PreviewModel } from "../../types";
import {
  controlUserReleasedReportPackageRequest,
  REPORT_PACKAGE_USER_RELEASE_ROUTE_ID,
  USER_RELEASED_REPORT_CONTEXT,
  type ControlledRouteExport,
  type LocalFirstRouteEvidence
} from "../redaction-controls/redactionExportControls";
import type { ReportComposition } from "./reportComposition";
import { buildUserReleasedReportPackageRequest, reportPackageExactSubmission } from "./reportPackageRequest";

/** The dialog's one answer: the application clock's reading when the user
 * confirmed, `YYYY-MM-DDTHH:MM:SSZ`. It lives only in the argument of the one save. */
export type UserReleaseConfirmation = { userReleasedAt: string };

/** Route 010's native receipt: the package receipt plus the record's two receipt members. */
export type UserReleasedReportPackageSaveReceipt = ReportPackageSaveReceipt & {
  user_release_recorded: boolean;
  user_release_model_document_sha256: string;
};

/** The status the session keeps of a route-010 save: the native receipt without the record's
 * bound model document SHA-256, which stays in the report and its package (DEL-08-01 REQ-019,
 * AC-010). */
export type UserReleasedReportPackageSaveStatus = Omit<UserReleasedReportPackageSaveReceipt, "user_release_model_document_sha256">;

/** Route 010's screening evidence (`ReportPackageSaveScreening`, `deny_unknown_fields`). */
type UserReleasedSaveScreening = {
  route_id: typeof REPORT_PACKAGE_USER_RELEASE_ROUTE_ID;
  export_context: typeof USER_RELEASED_REPORT_CONTEXT;
  explicit_local_private_intent: false;
  explicit_user_release_intent: true;
  blocked: false;
  materialization_withheld: false;
  lossless_required: true;
  exact_payload_match: true;
  blocking_count: number;
  decision_count: number;
  finding_count: number;
  payload_sha256: string;
};

/** The native command's arguments on route 010 (`save_report_package_args`). */
export type UserReleasedReportPackageSaveArgs = {
  requestCanonicalText: string;
  modelCanonicalText: string;
  inputManifestCanonicalText: string;
  /** The solve's own transient run record; a validation input only, never saved. */
  analysisRunCanonicalText: string;
  screening: UserReleasedSaveScreening;
  localFirstEvidence: LocalFirstRouteEvidence;
  exactResultText?: string;
  exactIntakeText?: string;
};

export type PreparedUserReleasedReportPackageSave =
  | { route: "ready"; args: UserReleasedReportPackageSaveArgs; requestSha256: string }
  | Exclude<ReportPackageSaveRoute, { route: "tauri_report_package_save" }>;

/** What `prepareShareableReportPackageSave` yields: the screened request's evidence, the
 * prepared save (or the route that ends it), and the request (for the exact re-check). */
export type ShareableReportPackagePreparation = {
  controlled: ControlledRouteExport;
  prepared: PreparedUserReleasedReportPackageSave;
  request: object;
};

function isTauriRuntime(): boolean {
  return typeof window !== "undefined" && "__TAURI_INTERNALS__" in window;
}

function hasAllowedUserReleaseEvidence(controlled: ControlledRouteExport): boolean {
  const evidence = controlled.summary.local_first;
  return (
    controlled.summary.route_id === REPORT_PACKAGE_USER_RELEASE_ROUTE_ID &&
    evidence?.route_id === REPORT_PACKAGE_USER_RELEASE_ROUTE_ID &&
    evidence.export_context === USER_RELEASED_REPORT_CONTEXT &&
    evidence.storage_context === "private_report" &&
    evidence.action === "include_metadata_only" &&
    evidence.reason_code === "USER_RELEASE_METADATA_ALLOWED" &&
    evidence.blocked === false &&
    evidence.metadata_only === true &&
    evidence.explicit_user_release_intent === true &&
    evidence.explicit_local_private_intent !== true
  );
}

/** The preparation on route 010: the projector's evidence, then the canonical
 * texts. The controlled payload must be canonically equal to the request it screened. */
export async function prepareUserReleasedReportPackageSave(
  controlled: ControlledRouteExport,
  model: unknown,
  request: unknown,
  inputManifest: unknown,
  analysisRun: unknown,
  exactSubmission: { exactResultText: string; exactIntakeText: string } | null = null
): Promise<PreparedUserReleasedReportPackageSave> {
  if (!hasAllowedUserReleaseEvidence(controlled)) {
    return {
      route: "redaction_blocked",
      diagnostic: "REPORT-PACKAGE-LOCAL-FIRST-EVIDENCE-REQUIRED: a shareable save requires allowed metadata-only user-release route evidence."
    };
  }
  if (controlled.blocked || controlled.payload === null || controlled.summary.materialization_withheld) {
    return {
      route: "redaction_blocked",
      diagnostic: `REPORT-PACKAGE-REDACTION-BLOCKED: ${controlled.findings.length} finding(s) prevent a shareable package.`
    };
  }
  if (!isTauriRuntime()) {
    return { route: "unavailable_browser", diagnostic: "REPORT-PACKAGE-SAVE-DESKTOP-ONLY" };
  }
  const [requestCanonicalText, screenedRequestText, hex, modelCanonicalText, inputManifestCanonicalText, analysisRunCanonicalText] = await Promise.all([
    canonicalJsonString(controlled.payload),
    canonicalJsonString(request),
    canonicalSha256Hex(controlled.payload),
    canonicalJsonString(model),
    canonicalJsonString(inputManifest),
    canonicalJsonString(analysisRun)
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
      analysisRunCanonicalText,
      screening: {
        route_id: REPORT_PACKAGE_USER_RELEASE_ROUTE_ID,
        export_context: USER_RELEASED_REPORT_CONTEXT,
        explicit_local_private_intent: false,
        explicit_user_release_intent: true,
        blocked: false,
        materialization_withheld: false,
        lossless_required: true,
        exact_payload_match: true,
        blocking_count: controlled.summary.blocking_count,
        decision_count: controlled.summary.decision_count,
        finding_count: controlled.summary.finding_count,
        payload_sha256: requestSha256
      },
      localFirstEvidence: controlled.summary.local_first!,
      ...(exactSubmission ? { exactResultText: exactSubmission.exactResultText, exactIntakeText: exactSubmission.exactIntakeText } : {})
    }
  };
}

/** The whole webview side of one shareable save, before IPC: the solve's
 * own run record, the request with the bound record, the user-release projector and the
 * preparation. `result` is the full result (a native result's bounded full copy). */
export async function prepareShareableReportPackageSave(input: {
  model: PreviewModel;
  result: MechanicsResult;
  inputManifest: CurrentSessionInputManifestEvidence;
  reportComposition: ReportComposition | null;
  confirmation: UserReleaseConfirmation;
}): Promise<ShareableReportPackagePreparation> {
  const { model, result, inputManifest, reportComposition, confirmation } = input;
  const analysisRun = await buildAnalysisRunPreview(result, { inputManifest });
  const request = await buildUserReleasedReportPackageRequest({
    model,
    result,
    analysisRun,
    inputManifest,
    reportComposition,
    userReleasedAt: confirmation.userReleasedAt
  });
  const controlled = controlUserReleasedReportPackageRequest(request, model, true);
  const prepared = await prepareUserReleasedReportPackageSave(
    controlled,
    model,
    request,
    inputManifest.manifest,
    analysisRun,
    reportPackageExactSubmission(request)
  );
  return { controlled, prepared, request };
}

/** The native save of a prepared route-010 request. `isCurrent` is the session's basis guard
 * (with the exact re-check), evaluated immediately before each send. The status returned keeps
 * whether the package records the user release, not the binding (the native receipt's
 * contract is unchanged). */
export async function invokeUserReleasedReportPackageSave(
  prepared: Extract<PreparedUserReleasedReportPackageSave, { route: "ready" }>,
  isCurrent: () => boolean
): Promise<ReportPackageSaveRoute> {
  const { user_release_model_document_sha256: _binding, ...receipt } =
    await invokeWithIngress<UserReleasedReportPackageSaveReceipt>("save_report_package", prepared.args, { stillCurrent: isCurrent });
  const status: UserReleasedReportPackageSaveStatus = receipt;
  return { route: "tauri_report_package_save", receipt: status };
}
