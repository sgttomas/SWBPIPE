// DEL-12-02 app-side binding (TP-E4-REDACTION-001): TypeScript mirror of the
// core redaction/export-control contract in
// `core/security/redaction/controls.py`, governed by
// `schemas/redaction_export_controls.schema.yaml`. Decisions are based on
// EXPLICIT metadata only; the walk never mutates its input, never reads
// files, never transmits anything, and never infers sensitivity from value
// text. Parity with the python contract is pinned by the shared invented
// corpus `fixtures/redaction_export_controls/cases.json` — change semantics
// on either side and one of the two suites fails.
//
// Boundary (F-PIP-2): every output of this module is an informational
// warning/redaction surface. It never certifies redaction sufficiency,
// release readiness, legal clearance, or professional acceptance.

export const REDACTED_VALUE = "[REDACTED]";

/** The five contexts the redaction panel, the controlled links and every existing route
 * select. The user-released context (`UserReleasedReportContext`) is deliberately not one
 * of them, so no existing surface can select it. */
export type RedactionExportContext =
  | "public_report"
  | "public_example"
  | "shared_model"
  | "downstream_tool"
  | "local_private";

/** W-18b (DEL-08-01 Design `report-user-release.md` §3.1): a report of the user's own
 * session in which the user releases their own project data for sharing, for that one
 * report, with their statement that they hold the redistribution right. It is admitted only on route
 * `DREP-PACKAGE-USER-RELEASE-010` with the wrapper's user-release intent; payload content
 * never selects or grants it. */
export type UserReleasedReportContext = "user_released_report";

/** The full DEL-12-02 export-context vocabulary the controls decide in. */
export type ControlExportContext = RedactionExportContext | UserReleasedReportContext;

export const USER_RELEASED_REPORT_CONTEXT: UserReleasedReportContext = "user_released_report";

/** The user-release route (§3.3). No route uses it before W-18b (r3). */
export const REPORT_PACKAGE_USER_RELEASE_ROUTE_ID = "DREP-PACKAGE-USER-RELEASE-010";

export type RedactionAction =
  | "include"
  | "warning_only"
  | "redact_value"
  | "redact_field"
  | "omit_field"
  | "block_export";

export type RedactionReasonCode =
  | "SAFE_PUBLIC_METADATA"
  | "SAFE_INVENTED_PUBLIC_EXAMPLE"
  | "PRIVATE_DATA_REDACTED"
  | "PRIVATE_LOCAL_ALLOWED_WITH_WARNING"
  | "UNKNOWN_PROVENANCE_WARNING"
  | "REDISTRIBUTION_STATUS_UNKNOWN"
  | "PROTECTED_CONTENT_BLOCKED"
  | "MISSING_METADATA_REDACTED"
  | "LOCAL_PRIVATE_INTENT_REQUIRED"
  | "PROFESSIONAL_BOUNDARY_BLOCKED"
  | "PAYLOAD_METADATA_ONLY_REQUIRED"
  | "SECRET_MATERIAL_BLOCKED"
  | "CLOUD_OR_NETWORK_REFERENCE_BLOCKED"
  | "DIRECT_SQL_ACCESS_BLOCKED"
  | "STORAGE_BYPASS_BLOCKED"
  | "CONCRETE_PATH_REDACTED"
  | "USER_RELEASED_PROJECT_DATA"
  | "USER_RELEASE_INTENT_REQUIRED"
  | "USER_RELEASE_VALUE_UNBOUND";

export type RedactionFindingClass =
  | "IP_BOUNDARY_WARNING"
  | "PRIVATE_DATA_WARNING"
  | "PROVENANCE_WARNING"
  | "PROFESSIONAL_BOUNDARY_WARNING"
  | "STORAGE_BOUNDARY_WARNING"
  | "SAFE_METADATA";

export type RedactionFindingSeverity = "INFO" | "WARNING" | "BLOCKING";

export type RedactionDecision = {
  decision_id: string;
  path: string;
  field_class: string;
  privacy_classification: string;
  redistribution_status: string;
  review_status: string;
  export_context: ControlExportContext;
  action: RedactionAction;
  reason_code: RedactionReasonCode;
  source_metadata_present: boolean;
};

export type RedactionFinding = {
  finding_id: string;
  code: RedactionReasonCode;
  class: RedactionFindingClass;
  severity: RedactionFindingSeverity;
  path: string;
  action: RedactionAction;
  message: string;
  remediation: string;
};

export type RedactionSummary = {
  decision_count: number;
  finding_count: number;
  redacted_count: number;
  omitted_count: number;
  warning_count: number;
  blocking_count: number;
  cloud_transmission_attempted: false;
  professional_claims_made: false;
};

export type RedactionRunResult = {
  payload: unknown;
  decisions: RedactionDecision[];
  findings: RedactionFinding[];
  blocked: boolean;
  summary: RedactionSummary;
};

export type LocalFirstRouteEvidence = {
  route_id: string;
  export_context: string;
  storage_context: string;
  action: "include_metadata_only" | "block_storage";
  reason_code:
    | "SAFE_PUBLIC_METADATA"
    | "PRIVATE_LOCAL_METADATA_ALLOWED"
    | "LOCAL_PRIVATE_INTENT_REQUIRED"
    | "LOCAL_FIRST_ROUTE_UNKNOWN"
    | "LOCAL_FIRST_EXPORT_CONTEXT_INVALID"
    | "USER_RELEASE_METADATA_ALLOWED"
    | "USER_RELEASE_ROUTE_REQUIRED"
    | "USER_RELEASE_INTENT_REQUIRED";
  blocked: boolean;
  metadata_only: true;
  explicit_local_private_intent: boolean;
  /** Present only when the route or the context is the user release's (§3.3), so the
   * evidence of every existing route keeps its exact members. */
  explicit_user_release_intent?: boolean;
};

export type ControlledRouteExport = RedactionRunResult & {
  summary: RedactionSummary & {
    route_id: string;
    materialization_withheld: boolean;
    local_first?: LocalFirstRouteEvidence;
  };
};

/** The selectable contexts (the redaction panel lists exactly these). */
export const EXPORT_CONTEXTS: RedactionExportContext[] = [
  "public_report",
  "public_example",
  "shared_model",
  "downstream_tool",
  "local_private"
];

/** Every context the controls accept: the selectable five and the user-released context. */
export const CONTROL_EXPORT_CONTEXTS: ControlExportContext[] = [...EXPORT_CONTEXTS, USER_RELEASED_REPORT_CONTEXT];

/** The governed local-first routes (exported read-only for the regressions). */
export const GOVERNED_LOCAL_FIRST_ROUTE_IDS: ReadonlySet<string> = new Set([
  "DOTH-CAEPIPE-LOCAL-006",
  "DOTH-FORMAT-003",
  "DOTH-HANDOFF-002",
  "DOTH-HANDOFF-PACKAGE-008",
  "DOTH-JSON-001",
  "DOTH-MODEL-DOC-007",
  "DOTH-PRIVATE-004",
  "DREP-HTML-SAVE-005",
  "DREP-IPC-003",
  "DREP-JSON-002",
  "DREP-LINT-JSON-007",
  "DREP-PACKAGE-SAVE-009",
  "DREP-PACKAGE-USER-RELEASE-010",
  "DREP-UI-001/DREP-JSON-002"
]);

/** The storage context of a context's governed route metadata: a user-released report is
 * written to the user's own local path by the same atomic save, so its route metadata is
 * held under the existing `private_report` storage context (§3.3). */
function storageContextFor(exportContext: string): string {
  return exportContext === USER_RELEASED_REPORT_CONTEXT ? "private_report" : exportContext;
}

/** Whether a caller stated any user-release intent: every value but absent, null or false.
 * The flag assertion and local-first admission refuse such a statement outside
 * `user_released_report`; only `true` admits route 010 (§3.3). */
function userReleaseIntentStated(explicitUserReleaseIntent: unknown): boolean {
  return !(explicitUserReleaseIntent === undefined || explicitUserReleaseIntent === null || explicitUserReleaseIntent === false);
}

export function enforceLocalFirstRoute(options: {
  routeId: unknown;
  exportContext: unknown;
  explicitLocalPrivateIntent?: boolean;
  explicitUserReleaseIntent?: boolean;
}): LocalFirstRouteEvidence {
  const routeId = typeof options.routeId === "string" ? options.routeId : "";
  const exportContext = typeof options.exportContext === "string" ? options.exportContext : "";
  const validContext = (CONTROL_EXPORT_CONTEXTS as string[]).includes(exportContext);
  const explicitIntent = options.explicitLocalPrivateIntent === true;
  const releaseIntent = options.explicitUserReleaseIntent === true;
  // Any user-release intent other than absent, null or false, the predicate of the flag
  // assertion (`assertUserReleaseIntentContext`): it is refused below in every context but
  // `user_released_report`, so it cannot be admitted in another context here either.
  const releaseIntentStated = userReleaseIntentStated(options.explicitUserReleaseIntent);
  const userReleaseRoute = routeId === REPORT_PACKAGE_USER_RELEASE_ROUTE_ID;
  const userReleaseContext = exportContext === USER_RELEASED_REPORT_CONTEXT;
  // The release intent is recorded only where it is the subject of the decision, so the
  // evidence of every existing route with an absent or false flag keeps its exact members.
  const releaseEvidence =
    userReleaseRoute || userReleaseContext || releaseIntentStated ? { explicit_user_release_intent: releaseIntent } : {};
  if (!GOVERNED_LOCAL_FIRST_ROUTE_IDS.has(routeId)) {
    return {
      route_id: routeId,
      export_context: exportContext,
      storage_context: validContext ? storageContextFor(exportContext) : "invalid",
      action: "block_storage",
      reason_code: "LOCAL_FIRST_ROUTE_UNKNOWN",
      blocked: true,
      metadata_only: true,
      explicit_local_private_intent: explicitIntent,
      ...releaseEvidence
    };
  }
  if (!validContext) {
    return {
      route_id: routeId,
      export_context: exportContext,
      storage_context: "invalid",
      action: "block_storage",
      reason_code: "LOCAL_FIRST_EXPORT_CONTEXT_INVALID",
      blocked: true,
      metadata_only: true,
      explicit_local_private_intent: explicitIntent,
      ...releaseEvidence
    };
  }
  // W-18b (§3.3): the user-released context only on its own route, that route only in that
  // context, and user-release intent only with that context; then the wrapper's release
  // intent.
  if (userReleaseRoute !== userReleaseContext || (releaseIntentStated && !userReleaseContext)) {
    return {
      route_id: routeId,
      export_context: exportContext,
      storage_context: storageContextFor(exportContext),
      action: "block_storage",
      reason_code: "USER_RELEASE_ROUTE_REQUIRED",
      blocked: true,
      metadata_only: true,
      explicit_local_private_intent: explicitIntent,
      ...releaseEvidence
    };
  }
  if (userReleaseRoute) {
    return {
      route_id: routeId,
      export_context: exportContext,
      storage_context: storageContextFor(exportContext),
      action: releaseIntent ? "include_metadata_only" : "block_storage",
      reason_code: releaseIntent ? "USER_RELEASE_METADATA_ALLOWED" : "USER_RELEASE_INTENT_REQUIRED",
      blocked: !releaseIntent,
      metadata_only: true,
      explicit_local_private_intent: explicitIntent,
      explicit_user_release_intent: releaseIntent
    };
  }
  if (exportContext === "local_private" && !explicitIntent) {
    return {
      route_id: routeId,
      export_context: exportContext,
      storage_context: exportContext,
      action: "block_storage",
      reason_code: "LOCAL_PRIVATE_INTENT_REQUIRED",
      blocked: true,
      metadata_only: true,
      explicit_local_private_intent: false
    };
  }
  return {
    route_id: routeId,
    export_context: exportContext,
    storage_context: exportContext,
    action: "include_metadata_only",
    reason_code:
      exportContext === "local_private"
        ? "PRIVATE_LOCAL_METADATA_ALLOWED"
        : "SAFE_PUBLIC_METADATA",
    blocked: false,
    metadata_only: true,
    explicit_local_private_intent: explicitIntent
  };
}

const SAFE_PRIVACY_CLASSES = new Set(["public", "public_metadata", "invented_public_example"]);
/** The private classes other than the user's own project data (§2.7): redacted in the
 * user-released context whatever their redistribution status. */
const NON_PROJECT_PRIVATE_CLASSES = new Set([
  "private_material_data",
  "private_component_data",
  "private_rule_pack_data",
  "owner_standard_data",
  "company_design_basis_data",
  "path_data",
  "secret_like_data"
]);
const PRIVATE_PRIVACY_CLASSES = new Set([
  "private_project_data",
  "private_material_data",
  "private_component_data",
  "private_rule_pack_data",
  "owner_standard_data",
  "company_design_basis_data",
  "path_data",
  "secret_like_data"
]);
const UNKNOWN_PRIVACY_CLASSES = new Set<unknown>(["unknown", "TBD", null, undefined]);
const PUBLIC_REDIS_STATUSES = new Set(["public_permissive", "invented_non_engineering_example"]);
const UNKNOWN_REDIS_STATUSES = new Set<unknown>(["unknown", "TBD", null, undefined]);
const PRIVATE_REDIS_STATUSES = new Set(["private_only"]);

const REDACTION_METADATA_KEYS = new Set([
  "field_class",
  "privacy_classification",
  "redistribution_status",
  "review_status",
  "provenance",
  "export_policy",
  "professional_claim"
]);
const STORAGE_PRIVACY_METADATA_KEYS = new Set([
  "contains_payload",
  "payload_present",
  "secret_material_present",
  "concrete_path_present",
  "cloud_or_network_reference",
  "direct_sql_access",
  "storage_bypass",
  "storage_bypass_requested",
  "direct_storage_bypass",
  "application_service_bypass",
  "local_private_intent",
  "explicit_local_private_intent",
  "user_intent",
  "symbolic_path_class",
  "path_class",
  "storage_locality"
]);
const PAYLOAD_DETAIL_KEYS = new Set([
  "payload",
  "content",
  "contents",
  "raw_value",
  "project_payload",
  "rule_payload",
  "material_values",
  "component_values",
  "report_payload",
  "diagnostic_payload",
  "cache_payload",
  "sqlite_payload"
]);
const SECRET_MATERIAL_KEYS = new Set([
  "secret",
  "secret_value",
  "credential",
  "credential_value",
  "token",
  "password",
  "api_key",
  "access_key",
  "authorization_header",
  "private_key"
]);
const CONCRETE_PATH_DETAIL_KEYS = new Set([
  "path",
  "file_path",
  "absolute_path",
  "local_path",
  "repository_path",
  "db_path",
  "sqlite_path",
  "concrete_path",
  "uri"
]);
const CLOUD_NETWORK_DETAIL_KEYS = new Set([
  "cloud_url",
  "remote_url",
  "network_location",
  "cloud_bucket",
  "cloud_sync_target",
  "hosted_database"
]);
const DIRECT_SQL_DETAIL_KEYS = new Set([
  "sql",
  "query",
  "table",
  "table_name",
  "sqlite_handle",
  "raw_sqlite_handle",
  "connection_string"
]);
const UNSAFE_DETAIL_KEYS = new Set([
  ...PAYLOAD_DETAIL_KEYS,
  ...SECRET_MATERIAL_KEYS,
  ...CONCRETE_PATH_DETAIL_KEYS,
  ...CLOUD_NETWORK_DETAIL_KEYS,
  ...DIRECT_SQL_DETAIL_KEYS
]);

type PlainObject = Record<string, unknown>;

const OMITTED_SENTINEL: unique symbol = Symbol("redaction-omitted");

function isPlainObject(value: unknown): value is PlainObject {
  return typeof value === "object" && value !== null && !Array.isArray(value);
}

// Input contract (docs/security/redaction_export_controls.md, "Input contract"; HELP_HUMAN ruling
// 2026-10-27). The flag strings all three implementations recognize, after trimming and lowercasing;
// identical in core/security/redaction/controls.py and redaction_binding.rs.
const RECOGNIZED_FLAG_STRINGS = new Set(["true", "yes", "explicit", "present", "local_private", "explicit_local_private"]);
/** Trimmed from an intent flag: ASCII whitespace only, U+0009-U+000D and U+0020. */
const INTENT_FLAG_TRIM = new Set([0x0009, 0x000a, 0x000b, 0x000c, 0x000d, 0x0020]);
/** Trimmed from a blocking flag: the union of what JavaScript's String.prototype.trim, Python's
 * str.strip() (core/security/redaction/controls.py) and Rust's str::trim (the mirror in
 * redaction_binding.rs) removed before the contract, so no implementation reads a blocking flag as
 * unset where it read it as set. The same code points as BLOCKING_FLAG_TRIM there. */
const BLOCKING_FLAG_TRIM = new Set([
  0x0009, 0x000a, 0x000b, 0x000c, 0x000d, 0x001c, 0x001d, 0x001e, 0x001f, 0x0020, 0x0085, 0x00a0, 0x1680,
  0x2000, 0x2001, 0x2002, 0x2003, 0x2004, 0x2005, 0x2006, 0x2007, 0x2008, 0x2009, 0x200a, 0x2028, 0x2029,
  0x202f, 0x205f, 0x3000, 0xfeff
]);

function trimCodePoints(value: string, trimmed: Set<number>): string {
  const codePoints = [...value];
  let start = 0;
  let end = codePoints.length;
  while (start < end && trimmed.has(codePoints[start].codePointAt(0) as number)) start += 1;
  while (end > start && trimmed.has(codePoints[end - 1].codePointAt(0) as number)) end -= 1;
  return codePoints.slice(start, end).join("");
}

/** A blocking flag is set by `true`, a non-zero number, or a recognized string after trimming
 * BLOCKING_FLAG_TRIM (input contract). Any other scalar keeps `Boolean()`, as before the contract. */
function blockingFlag(value: unknown): boolean {
  if (typeof value === "boolean") return value;
  if (typeof value === "string") {
    return RECOGNIZED_FLAG_STRINGS.has(trimCodePoints(value, BLOCKING_FLAG_TRIM).toLowerCase());
  }
  if (typeof value === "number") return value !== 0;
  if (typeof value === "object" && value !== null) return false;
  return Boolean(value);
}

/** An intent flag is set only by `true` or a recognized string after trimming ASCII whitespace
 * (input contract); a number, an array or an object is not intent. */
function intentFlag(value: unknown): boolean {
  if (value === true) return true;
  if (typeof value === "string") {
    return RECOGNIZED_FLAG_STRINGS.has(trimCodePoints(value, INTENT_FLAG_TRIM).toLowerCase());
  }
  return false;
}

/** A professional claim blocks unless it is absent, null or `false` (input contract). */
function professionalClaim(item: PlainObject): boolean {
  const claim = item.professional_claim;
  return !(claim === undefined || claim === null || claim === false);
}

/** A review status that is not a string (nor null) reads as `unknown`, and so gives way to the
 * provenance's (input contract). A privacy or redistribution label that is not a string is a
 * stated label that matches no class: it is not unknown, safe, private or protected, and does not
 * fall back to the provenance; every check below reads it so. */
function reviewLabel(value: unknown): unknown {
  return value === null || value === undefined || typeof value === "string" ? value : "unknown";
}

function isScalarMetadataValue(value: unknown): boolean {
  return !(typeof value === "object" && value !== null);
}

function stringOrUnknown(value: unknown): string {
  if (value === null || value === undefined) return "unknown";
  if (typeof value === "string" && value.length > 0) return value;
  return "unknown";
}

function intersects(keys: Set<string>, item: PlainObject): boolean {
  return Object.keys(item).some((key) => keys.has(key));
}

function hasExplicitMetadata(item: PlainObject): boolean {
  if (intersects(REDACTION_METADATA_KEYS, item)) return true;
  return Object.keys(item).some(
    (key) => STORAGE_PRIVACY_METADATA_KEYS.has(key) && isScalarMetadataValue(item[key])
  );
}

function hasUnsafeItemDetail(item: PlainObject): boolean {
  const keys = Object.keys(item);
  const unsafeKeys = keys.filter((key) => UNSAFE_DETAIL_KEYS.has(key));
  if (unsafeKeys.length === 0) return false;
  if (keys.some((key) => ["field_id", "record_id", "id", "field_class"].includes(key))) return true;
  return keys.length > 0 && keys.every((key) => UNSAFE_DETAIL_KEYS.has(key));
}

function isValueBearing(item: PlainObject): boolean {
  return "value" in item || "text" in item || hasExplicitMetadata(item) || hasUnsafeItemDetail(item);
}

function containsPayload(item: PlainObject): boolean {
  return (
    blockingFlag(item.contains_payload) || blockingFlag(item.payload_present) || intersects(PAYLOAD_DETAIL_KEYS, item)
  );
}

function secretMaterialPresent(item: PlainObject): boolean {
  return blockingFlag(item.secret_material_present) || intersects(SECRET_MATERIAL_KEYS, item);
}

function cloudOrNetworkReference(item: PlainObject): boolean {
  return blockingFlag(item.cloud_or_network_reference) || intersects(CLOUD_NETWORK_DETAIL_KEYS, item);
}

function directSqlAccess(item: PlainObject): boolean {
  return blockingFlag(item.direct_sql_access) || intersects(DIRECT_SQL_DETAIL_KEYS, item);
}

function storageBypassRequested(item: PlainObject): boolean {
  return ["storage_bypass", "storage_bypass_requested", "direct_storage_bypass", "application_service_bypass"].some(
    (key) => blockingFlag(item[key])
  );
}

function concretePathPresent(item: PlainObject): boolean {
  return blockingFlag(item.concrete_path_present) || intersects(CONCRETE_PATH_DETAIL_KEYS, item);
}

function itemIntent(item: PlainObject): boolean | null {
  for (const key of ["local_private_intent", "explicit_local_private_intent", "user_intent"]) {
    if (key in item) return intentFlag(item[key]);
  }
  const exportPolicy = item.export_policy;
  if (isPlainObject(exportPolicy)) {
    for (const key of ["local_private_intent", "explicit_local_private_intent", "user_intent"]) {
      if (key in exportPolicy) return intentFlag(exportPolicy[key]);
    }
  }
  return null;
}

function localPrivateIntent(item: PlainObject, explicitLocalPrivateIntent: boolean): boolean {
  const explicitItemIntent = itemIntent(item);
  if (explicitItemIntent === false) return false;
  return explicitLocalPrivateIntent || explicitItemIntent === true;
}

function unknownAction(exportContext: RedactionExportContext): RedactionAction {
  return exportContext === "local_private" ? "warning_only" : "redact_value";
}

/** Steps 10-13 of the decision order (§3.2), only for `user_released_report`, after the blocks of
 * steps 1-9 and before any unknown-metadata branch; the concrete path (step 7) redacts only what they
 * do not block. Only the wrapper's release intent is read. */
function userReleasedAction(args: {
  privacy: unknown;
  redistribution: unknown;
  explicitUserReleaseIntent: boolean;
}): { action: RedactionAction; reason: RedactionReasonCode } {
  if (typeof args.privacy === "string" && NON_PROJECT_PRIVATE_CLASSES.has(args.privacy)) {
    return { action: "redact_value", reason: "PRIVATE_DATA_REDACTED" };
  }
  if (args.privacy === "private_project_data") {
    return args.explicitUserReleaseIntent
      ? { action: "warning_only", reason: "USER_RELEASED_PROJECT_DATA" }
      : { action: "block_export", reason: "USER_RELEASE_INTENT_REQUIRED" };
  }
  if (
    typeof args.privacy === "string" &&
    SAFE_PRIVACY_CLASSES.has(args.privacy) &&
    typeof args.redistribution === "string" &&
    PUBLIC_REDIS_STATUSES.has(args.redistribution)
  ) {
    return {
      action: "include",
      reason: args.privacy === "invented_public_example" ? "SAFE_INVENTED_PUBLIC_EXAMPLE" : "SAFE_PUBLIC_METADATA"
    };
  }
  return { action: "block_export", reason: "USER_RELEASE_VALUE_UNBOUND" };
}

type ActionInputs = {
  privacy: unknown;
  redistribution: unknown;
  reviewStatus: unknown;
  metadataPresent: boolean;
  exportContext: ControlExportContext;
  explicitLocalPrivateIntent: boolean;
  explicitUserReleaseIntent: boolean;
  professionalClaim: boolean;
  containsPayload: boolean;
  secretMaterialPresent: boolean;
  cloudOrNetworkReference: boolean;
  directSqlAccess: boolean;
  storageBypass: boolean;
  concretePathPresent: boolean;
};

/** The blocks of the decision order (§3.2) that come first in every context: the claim, network,
 * SQL, storage-bypass, secret and payload checks, then the protected and the quarantined or
 * rejected blocks. `null` when none of them decides. The concrete-path redaction comes after
 * the context's own decision (`actionFor`). */
function boundaryActionFor(args: ActionInputs): { action: RedactionAction; reason: RedactionReasonCode } | null {
  if (args.professionalClaim) return { action: "block_export", reason: "PROFESSIONAL_BOUNDARY_BLOCKED" };
  if (args.cloudOrNetworkReference) return { action: "block_export", reason: "CLOUD_OR_NETWORK_REFERENCE_BLOCKED" };
  if (args.directSqlAccess) return { action: "block_export", reason: "DIRECT_SQL_ACCESS_BLOCKED" };
  if (args.storageBypass) return { action: "block_export", reason: "STORAGE_BYPASS_BLOCKED" };
  if (args.secretMaterialPresent) return { action: "block_export", reason: "SECRET_MATERIAL_BLOCKED" };
  if (args.containsPayload) return { action: "block_export", reason: "PAYLOAD_METADATA_ONLY_REQUIRED" };
  if (args.privacy === "protected_suspected" || args.redistribution === "protected_suspected") {
    return { action: "block_export", reason: "PROTECTED_CONTENT_BLOCKED" };
  }
  if (args.reviewStatus === "quarantined" || args.reviewStatus === "rejected") {
    return { action: "block_export", reason: "PROTECTED_CONTENT_BLOCKED" };
  }
  return null;
}

/** The context's own decision (§3.2), after `boundaryActionFor` decided nothing: steps 10-13 in
 * `user_released_report`, today's sequence in the five existing contexts. */
function contextActionFor(args: ActionInputs): { action: RedactionAction; reason: RedactionReasonCode } {
  if (args.exportContext === USER_RELEASED_REPORT_CONTEXT) return userReleasedAction(args);
  if (!args.metadataPresent) {
    return { action: unknownAction(args.exportContext), reason: "MISSING_METADATA_REDACTED" };
  }
  if (UNKNOWN_REDIS_STATUSES.has(args.redistribution)) {
    return { action: unknownAction(args.exportContext), reason: "REDISTRIBUTION_STATUS_UNKNOWN" };
  }
  if (UNKNOWN_PRIVACY_CLASSES.has(args.privacy)) {
    return { action: unknownAction(args.exportContext), reason: "UNKNOWN_PROVENANCE_WARNING" };
  }
  if (
    typeof args.privacy === "string" &&
    SAFE_PRIVACY_CLASSES.has(args.privacy) &&
    typeof args.redistribution === "string" &&
    PUBLIC_REDIS_STATUSES.has(args.redistribution)
  ) {
    return {
      action: "include",
      reason: args.privacy === "invented_public_example" ? "SAFE_INVENTED_PUBLIC_EXAMPLE" : "SAFE_PUBLIC_METADATA"
    };
  }
  if (
    (typeof args.privacy === "string" && PRIVATE_PRIVACY_CLASSES.has(args.privacy)) ||
    (typeof args.redistribution === "string" && PRIVATE_REDIS_STATUSES.has(args.redistribution))
  ) {
    if (args.exportContext === "local_private") {
      if (args.explicitLocalPrivateIntent) {
        return { action: "warning_only", reason: "PRIVATE_LOCAL_ALLOWED_WITH_WARNING" };
      }
      return { action: "block_export", reason: "LOCAL_PRIVATE_INTENT_REQUIRED" };
    }
    return { action: "redact_value", reason: "PRIVATE_DATA_REDACTED" };
  }
  return { action: unknownAction(args.exportContext), reason: "UNKNOWN_PROVENANCE_WARNING" };
}

/** `boundary` is true when a marker or disposition decided: a `boundaryActionFor` block, or a
 * concrete path. A concrete path redacts only what would not block (input contract, 2026-10-27):
 * it never turns a protected, quarantined/rejected, intent or user-release block into a redaction. */
function actionFor(args: ActionInputs): {
  action: RedactionAction;
  reason: RedactionReasonCode;
  boundary: boolean;
} {
  const boundary = boundaryActionFor(args);
  if (boundary !== null) return { ...boundary, boundary: true };
  const decided = contextActionFor(args);
  if (args.concretePathPresent && decided.action !== "block_export") {
    return { action: "redact_value", reason: "CONCRETE_PATH_REDACTED", boundary: true };
  }
  return { ...decided, boundary: args.concretePathPresent };
}

const FINDING_CLASS_BY_REASON: Record<string, RedactionFindingClass> = {
  PRIVATE_DATA_REDACTED: "PRIVATE_DATA_WARNING",
  PRIVATE_LOCAL_ALLOWED_WITH_WARNING: "PRIVATE_DATA_WARNING",
  UNKNOWN_PROVENANCE_WARNING: "PROVENANCE_WARNING",
  REDISTRIBUTION_STATUS_UNKNOWN: "PROVENANCE_WARNING",
  MISSING_METADATA_REDACTED: "PROVENANCE_WARNING",
  PROTECTED_CONTENT_BLOCKED: "IP_BOUNDARY_WARNING",
  LOCAL_PRIVATE_INTENT_REQUIRED: "PRIVATE_DATA_WARNING",
  PROFESSIONAL_BOUNDARY_BLOCKED: "PROFESSIONAL_BOUNDARY_WARNING",
  PAYLOAD_METADATA_ONLY_REQUIRED: "STORAGE_BOUNDARY_WARNING",
  SECRET_MATERIAL_BLOCKED: "PRIVATE_DATA_WARNING",
  CLOUD_OR_NETWORK_REFERENCE_BLOCKED: "STORAGE_BOUNDARY_WARNING",
  DIRECT_SQL_ACCESS_BLOCKED: "STORAGE_BOUNDARY_WARNING",
  STORAGE_BYPASS_BLOCKED: "STORAGE_BOUNDARY_WARNING",
  CONCRETE_PATH_REDACTED: "STORAGE_BOUNDARY_WARNING",
  USER_RELEASED_PROJECT_DATA: "PRIVATE_DATA_WARNING",
  USER_RELEASE_INTENT_REQUIRED: "PRIVATE_DATA_WARNING",
  USER_RELEASE_VALUE_UNBOUND: "PROVENANCE_WARNING"
};

const FINDING_MESSAGE_BY_ACTION: Record<string, string> = {
  warning_only: "Export retained the value only in a local/private context with an explicit warning.",
  redact_value: "Export value was replaced because metadata does not support public/shared release.",
  redact_field: "Export field was redacted because metadata does not support release.",
  omit_field: "Export field was omitted because metadata does not support release.",
  block_export: "Export is blocked until the metadata or boundary issue is resolved."
};

/** The user-released context's own messages (§3.1); every other reason keeps its action's message. */
const FINDING_MESSAGE_BY_REASON: Record<string, string> = {
  USER_RELEASED_PROJECT_DATA:
    "Export retained the user's own project data in this report under the user's release of their project data, with a warning.",
  USER_RELEASE_INTENT_REQUIRED:
    "Export is blocked: this report carries the user's project data and no user release of that data was selected for it.",
  USER_RELEASE_VALUE_UNBOUND:
    "Export is blocked: a user-released report includes only values whose privacy and redistribution metadata it recognizes."
};

const FINDING_REMEDIATION =
  "Record explicit privacy, provenance, redistribution, review, and user-intent metadata before export.";

function findingFor(decision: RedactionDecision, findingIndex: number): RedactionFinding | null {
  if (decision.action === "include") return null;
  const severity: RedactionFindingSeverity = decision.action === "block_export" ? "BLOCKING" : "WARNING";
  return {
    finding_id: `REDC-FND-${String(findingIndex).padStart(4, "0")}`,
    code: decision.reason_code,
    class: FINDING_CLASS_BY_REASON[decision.reason_code],
    severity,
    path: decision.path,
    action: decision.action,
    message: FINDING_MESSAGE_BY_REASON[decision.reason_code] ?? FINDING_MESSAGE_BY_ACTION[decision.action],
    remediation: FINDING_REMEDIATION
  };
}

function withoutUnsafeDetail(item: PlainObject): PlainObject {
  const output: PlainObject = {};
  for (const [key, value] of Object.entries(item)) {
    if (!UNSAFE_DETAIL_KEYS.has(key)) output[key] = value;
  }
  return output;
}

function applyAction(item: PlainObject, action: RedactionAction): unknown | typeof OMITTED_SENTINEL {
  if (action === "include" || action === "warning_only") return item;
  if (action === "omit_field") return OMITTED_SENTINEL;
  const redacted = withoutUnsafeDetail(item);
  if (action === "redact_field") {
    const output: PlainObject = {};
    for (const key of ["field_id", "record_id", "field_class", "privacy_classification"]) {
      if (key in redacted) output[key] = redacted[key];
    }
    return output;
  }
  if ("value" in redacted) redacted.value = REDACTED_VALUE;
  if ("text" in redacted) redacted.text = REDACTED_VALUE;
  redacted.privacy_classification = "redacted";
  return redacted;
}

function classify(
  item: PlainObject,
  args: {
    path: string;
    exportContext: ControlExportContext;
    explicitLocalPrivateIntent: boolean;
    explicitUserReleaseIntent: boolean;
    decisionIndex: number;
    findingIndex: number;
  }
): { decision: RedactionDecision; finding: RedactionFinding | null; boundary: boolean } {
  const metadataPresent = hasExplicitMetadata(item) || intersects(UNSAFE_DETAIL_KEYS, item);
  const fieldClass = stringOrUnknown(item.field_class ?? "unknown");
  let privacy: unknown = item.privacy_classification;
  let redistribution: unknown = item.redistribution_status;
  let reviewStatus: unknown = reviewLabel(item.review_status) ?? "unknown";
  // Payload-carried local-private intent is not read in the user-released context (§3.2):
  // only the wrapper's release intent decides there.
  const intent =
    args.exportContext === USER_RELEASED_REPORT_CONTEXT ? false : localPrivateIntent(item, args.explicitLocalPrivateIntent);

  const provenance = item.provenance;
  if (isPlainObject(provenance)) {
    privacy = privacy !== null && privacy !== undefined ? privacy : provenance.privacy_classification;
    redistribution =
      redistribution !== null && redistribution !== undefined ? redistribution : provenance.redistribution_status;
    reviewStatus =
      reviewStatus !== null && reviewStatus !== undefined && reviewStatus !== "unknown"
        ? reviewStatus
        : (reviewLabel(provenance.review_status) ?? "unknown");
  }

  const { action, reason, boundary } = actionFor({
    privacy,
    redistribution,
    reviewStatus,
    metadataPresent,
    exportContext: args.exportContext,
    explicitLocalPrivateIntent: intent,
    explicitUserReleaseIntent: args.explicitUserReleaseIntent,
    professionalClaim: professionalClaim(item),
    containsPayload: containsPayload(item),
    secretMaterialPresent: secretMaterialPresent(item),
    cloudOrNetworkReference: cloudOrNetworkReference(item),
    directSqlAccess: directSqlAccess(item),
    storageBypass: storageBypassRequested(item),
    concretePathPresent: concretePathPresent(item)
  });
  const decision: RedactionDecision = {
    decision_id: `REDC-DEC-${String(args.decisionIndex).padStart(4, "0")}`,
    path: args.path,
    field_class: fieldClass,
    privacy_classification: stringOrUnknown(privacy),
    redistribution_status: stringOrUnknown(redistribution),
    review_status: stringOrUnknown(reviewStatus),
    export_context: args.exportContext,
    action,
    reason_code: reason,
    source_metadata_present: metadataPresent
  };
  return { decision, finding: findingFor(decision, args.findingIndex), boundary };
}

export function classifyExportItem(
  item: PlainObject,
  options: {
    exportContext: ControlExportContext;
    explicitLocalPrivateIntent?: boolean;
    explicitUserReleaseIntent?: boolean;
    path?: string;
  }
): RedactionDecision {
  assertContext(options.exportContext);
  assertUserReleaseIntentContext(options.exportContext, options.explicitUserReleaseIntent);
  return classify(item, {
    path: options.path ?? "$",
    exportContext: options.exportContext,
    explicitLocalPrivateIntent: options.explicitLocalPrivateIntent ?? false,
    explicitUserReleaseIntent: options.explicitUserReleaseIntent === true,
    decisionIndex: 1,
    findingIndex: 1
  }).decision;
}

/** The redaction walk serves the five existing contexts only; `redactExportPayload` refuses
 * `user_released_report` before it is reached (§3.2). */
function walk(
  value: unknown,
  args: {
    path: string;
    exportContext: RedactionExportContext;
    explicitLocalPrivateIntent: boolean;
    decisions: RedactionDecision[];
    findings: RedactionFinding[];
  }
): unknown | typeof OMITTED_SENTINEL {
  if (isPlainObject(value)) {
    if (isValueBearing(value)) {
      const { decision, finding } = classify(value, {
        path: args.path,
        exportContext: args.exportContext,
        explicitLocalPrivateIntent: args.explicitLocalPrivateIntent,
        explicitUserReleaseIntent: false,
        decisionIndex: args.decisions.length + 1,
        findingIndex: args.findings.length + 1
      });
      args.decisions.push(decision);
      if (finding !== null) args.findings.push(finding);
      return applyAction(value, decision.action);
    }
    const output: PlainObject = {};
    for (const [key, item] of Object.entries(value)) {
      const child = walk(item, { ...args, path: `${args.path}.${key}` });
      if (child !== OMITTED_SENTINEL) output[key] = child;
    }
    return output;
  }
  if (Array.isArray(value)) {
    const output: unknown[] = [];
    value.forEach((item, index) => {
      const child = walk(item, { ...args, path: `${args.path}[${index}]` });
      if (child !== OMITTED_SENTINEL) output.push(child);
    });
    return output;
  }
  return value;
}

function assertContext(exportContext: string): void {
  if (!CONTROL_EXPORT_CONTEXTS.includes(exportContext as ControlExportContext)) {
    throw new Error(`unsupported export_context: ${exportContext}`);
  }
}

/** The flag assertion (§3.3): user-release intent with any context other than
 * `user_released_report` is refused loudly, so the flag cannot enable export in another
 * context. */
function assertUserReleaseIntentContext(exportContext: string, explicitUserReleaseIntent: unknown): void {
  if (!userReleaseIntentStated(explicitUserReleaseIntent)) return;
  if (exportContext !== USER_RELEASED_REPORT_CONTEXT) {
    throw new Error(
      `explicitUserReleaseIntent is admitted only in the ${USER_RELEASED_REPORT_CONTEXT} context, not in ${exportContext}`
    );
  }
}

function summarize(decisions: RedactionDecision[], findings: RedactionFinding[]): RedactionSummary {
  return {
    decision_count: decisions.length,
    finding_count: findings.length,
    redacted_count: decisions.filter((decision) => decision.action === "redact_value" || decision.action === "redact_field")
      .length,
    omitted_count: decisions.filter((decision) => decision.action === "omit_field").length,
    warning_count: findings.filter((finding) => finding.severity === "WARNING").length,
    blocking_count: findings.filter((finding) => finding.severity === "BLOCKING").length,
    cloud_transmission_attempted: false,
    professional_claims_made: false
  };
}

/** The redaction walk refuses the `user_released_report` context outright, in every call and
 * before any walk (§3.2), as the generic route exporter does. The walk decides a value-bearing
 * object as a whole and keeps an unlabelled value, so in that context it would admit what the
 * user-release screen refuses (a marker or a non-project private class on a descendant of a
 * labelled object). `controlUserReleasedReportPackageRequest` is the only payload entry for
 * the context; `classifyExportItem` decides one item by the rule. */
function assertRedactionWalkContext(exportContext: unknown): void {
  if (exportContext === USER_RELEASED_REPORT_CONTEXT) {
    throw new Error(
      `USER_RELEASE_ROUTE_REQUIRED: the redaction walk does not admit the ${USER_RELEASED_REPORT_CONTEXT} context; only the user-release projector does`
    );
  }
}

export function redactExportPayload(
  payload: unknown,
  options: {
    exportContext: RedactionExportContext;
    explicitLocalPrivateIntent?: boolean;
    explicitUserReleaseIntent?: boolean;
  }
): RedactionRunResult {
  assertContext(options.exportContext);
  assertUserReleaseIntentContext(options.exportContext, options.explicitUserReleaseIntent);
  assertRedactionWalkContext(options.exportContext);
  const decisions: RedactionDecision[] = [];
  const findings: RedactionFinding[] = [];
  const redacted = walk(structuredClone(payload), {
    path: "$",
    exportContext: options.exportContext,
    explicitLocalPrivateIntent: options.explicitLocalPrivateIntent ?? false,
    decisions,
    findings
  });
  return {
    payload: redacted === OMITTED_SENTINEL ? null : redacted,
    decisions,
    findings,
    blocked: decisions.some((decision) => decision.action === "block_export"),
    summary: summarize(decisions, findings)
  };
}

const ROUTE_INTENT_KEYS = new Set([
  "local_private_intent",
  "explicit_local_private_intent",
  "user_intent"
]);
const ROUTE_KEY_PREFIX = "__route_key__";

const PRIVATE_ROUTE_TOKENS = [
  "component",
  "content",
  "coordinate",
  "design_basis",
  "displacement",
  "force",
  "free_metadata",
  "geometry",
  "diameter",
  "material",
  "moment",
  "owner",
  "path",
  "pipe_segment",
  "project_name",
  "project_ref",
  "project_id",
  "result_value",
  "rotation",
  "rule_detail",
  "stress",
  "text",
  "thickness",
  "value"
];
type StructuralProjection = "pcf_export" | "caepipe_mbf_export";

const PCF_STRUCTURAL_PUBLIC_PATHS = new Set([
  "$.scope_items[]",
  "$.export_profile.profile_id",
  "$.export_profile.profile_version",
  "$.export_profile.target_family",
  "$.export_profile.target_profile_version_basis",
  "$.export_profile.artifact_format",
  "$.export_profile.subset_scope",
  "$.export_profile.unit_policy",
  "$.export_profile.coordinate_policy",
  "$.export_profile.identity_policy",
  "$.export_profile.loss_report_policy",
  "$.export_profile.translator_default_policy",
  "$.export_profile.support_restraint_policy",
  "$.export_profile.source_basis_refs[].object_type",
  "$.export_profile.source_basis_refs[].ref",
  "$.export_profile.boundary_notes[]",
  "$.conversion_witnesses[].witness_id",
  "$.conversion_witnesses[].target_quantity.target_field",
  "$.conversion_witnesses[].conversion_factor_to_target"
]);

const CAEPIPE_MBF_STRUCTURAL_PUBLIC_PATHS = new Set([
  "$.scope_items[]",
  "$.export_profile.profile_id",
  "$.export_profile.profile_version",
  "$.export_profile.target_family",
  "$.export_profile.target_version_basis",
  "$.export_profile.record_subset_basis",
  "$.export_profile.stable_id_policy",
  "$.export_profile.unit_policy",
  "$.export_profile.loss_report_policy",
  "$.export_profile.external_execution_policy",
  "$.export_profile.source_basis_refs[].object_type",
  "$.export_profile.source_basis_refs[].ref",
  "$.export_profile.carried_tbd_refs[]",
  "$.export_profile.boundary_notes[]",
  "$.conversion_witnesses[].witness_id",
  "$.conversion_witnesses[].target_quantity.target_field",
  "$.conversion_witnesses[].conversion_factor_to_target"
]);

function isExactStructuralPublicPath(
  structuralProjection: StructuralProjection | null,
  path: string
): boolean {
  if (structuralProjection === null) return false;
  const normalizedPath = path.replace(/\[\d+\]/g, "[]");
  const allowlist =
    structuralProjection === "pcf_export"
      ? PCF_STRUCTURAL_PUBLIC_PATHS
      : CAEPIPE_MBF_STRUCTURAL_PUBLIC_PATHS;
  return allowlist.has(normalizedPath);
}

function structuralProjectionFor(payload: unknown, routeId: string): StructuralProjection | null {
  if (
    routeId !== "DOTH-FORMAT-003" ||
    !isPlainObject(payload) ||
    !Array.isArray(payload.scope_items) ||
    !isPlainObject(payload.export_profile) ||
    !Array.isArray(payload.conversion_witnesses)
  ) {
    return null;
  }
  if (
    payload.document_kind ===
      "openpipestress.technical_preview.conservative_pcf_export_package" &&
    payload.deliverable_id === "DEL-17-07"
  ) {
    return "pcf_export";
  }
  if (
    payload.document_kind ===
      "openpipestress.technical_preview.caepipe_mbf_export_package" &&
    payload.deliverable_id === "DEL-17-04"
  ) {
    return "caepipe_mbf_export";
  }
  return null;
}

function stripRouteIntent(value: unknown): unknown {
  if (Array.isArray(value)) return value.map(stripRouteIntent);
  if (!isPlainObject(value)) return structuredClone(value);
  const output: PlainObject = {};
  for (const [key, item] of Object.entries(value)) {
    if (ROUTE_INTENT_KEYS.has(key)) continue;
    if (key === "export_policy" && isPlainObject(item)) {
      output[key] = Object.fromEntries(
        Object.entries(item)
          .filter(([policyKey]) => !ROUTE_INTENT_KEYS.has(policyKey))
          .map(([policyKey, policyValue]) => [policyKey, stripRouteIntent(policyValue)])
      );
    } else {
      output[key] = stripRouteIntent(item);
    }
  }
  return output;
}

function projectRouteValue(
  value: unknown,
  args: {
    routeId: string;
    path: string;
    publicBasis: boolean;
    structuralProjection: StructuralProjection | null;
  }
): unknown {
  if (Array.isArray(value)) {
    return value.map((item, index) =>
      projectRouteValue(item, { ...args, path: `${args.path}[${index}]`, publicBasis: false })
    );
  }
  if (isPlainObject(value)) {
    if (
      "value" in value &&
      !isPlainObject(value.value) &&
      !Array.isArray(value.value) &&
      hasExplicitMetadata(value)
    ) {
      return structuredClone(value);
    }
    // The `classification` alias is read only when `privacy_classification` is absent, as
    // Python reads it: a present primary label, null included, is the label, so a null cannot
    // let the alias lift a sibling into a public basis (HELP_HUMAN decision, 2026-10-27).
    const directPrivacy = Object.prototype.hasOwnProperty.call(value, "privacy_classification")
      ? value.privacy_classification
      : value.classification;
    // A non-string label is a stated label that matches no class (input contract): `["public"]`
    // is not a public basis.
    const directRedistribution = value.redistribution_status;
    const boundedPublicBasis =
      typeof directPrivacy === "string" &&
      SAFE_PRIVACY_CLASSES.has(directPrivacy) &&
      typeof directRedistribution === "string" &&
      PUBLIC_REDIS_STATUSES.has(directRedistribution);
    return Object.fromEntries(
      Object.entries(value).map(([key, item]) => [
        `${ROUTE_KEY_PREFIX}${key}`,
        projectRouteValue(item, {
          ...args,
          path: `${args.path}.${key}`,
          publicBasis: boundedPublicBasis
        })
      ])
    );
  }

  const lowered = args.path.toLowerCase().replaceAll("-", "_");
  const unknown = value === null || value === undefined || value === "TBD" || lowered.includes("tbd");
  const pathSegments = lowered.replaceAll("[", ".").replaceAll("]", "").split(".");
  const leafSegment = pathSegments.at(-1) ?? "unknown";
  const checksumValue =
    leafSegment === "value" && (lowered.includes("checksum") || lowered.includes("hash"));
  const unitMapLeaf =
    lowered.includes(".model_units.") || lowered.includes(".target_export_units.");
  const privateValueLeaf =
    !checksumValue &&
    !unitMapLeaf &&
    PRIVATE_ROUTE_TOKENS.some(
      (token) => leafSegment === token || leafSegment.endsWith(`_${token}`)
    );
  const safePublicValue =
    !privateValueLeaf && isExactStructuralPublicPath(args.structuralProjection, args.path);
  const privateValue =
    privateValueLeaf ||
    (!safePublicValue && PRIVATE_ROUTE_TOKENS.some((token) => lowered.includes(token)));
  const privacy = unknown
    ? "unknown"
    : privateValue
      ? "private_project_data"
      : args.publicBasis || safePublicValue
        ? "public_metadata"
        : "unknown";
  const redistribution = unknown
    ? "unknown"
    : privateValue
      ? "private_only"
      : args.publicBasis || safePublicValue
        ? "public_permissive"
        : "unknown";
  return {
    field_id: `${args.routeId}:${args.path}`,
    field_class: args.path.split(".").at(-1)?.split("[")[0] || "unknown",
    privacy_classification: privacy,
    redistribution_status: redistribution,
    review_status:
      unknown || (!privateValue && !args.publicBasis && !safePublicValue) ? "pending" : "accepted",
    value: structuredClone(value),
    _route_projected_leaf: true
  };
}

function materializeRouteValue(value: unknown): unknown {
  if (Array.isArray(value)) return value.map(materializeRouteValue);
  if (!isPlainObject(value)) return structuredClone(value);
  if (value._route_projected_leaf === true) return structuredClone(value.value);
  return Object.fromEntries(
    Object.entries(value)
      .filter(([key]) => key !== "_route_projected_leaf")
      .map(([key, item]) => [
        key.startsWith(ROUTE_KEY_PREFIX) ? key.slice(ROUTE_KEY_PREFIX.length) : key,
        materializeRouteValue(item)
      ])
  );
}

/** The generic route exporter serves the five existing contexts only (§3.3). The
 * `user_released_report` context and route `DREP-PACKAGE-USER-RELEASE-010` are refused
 * outright, in every pairing and before any projection, as `assertContext` refuses a context
 * the exporter does not support: its path-token projector labels a value by its key, so it
 * would admit what the user-release screen refuses (markers on enclosing objects, non-project
 * private classes). `controlUserReleasedReportPackageRequest` is the only entry for the
 * user-released context. */
function assertGenericRouteExport(routeId: unknown, exportContext: unknown): void {
  if (exportContext === USER_RELEASED_REPORT_CONTEXT || routeId === REPORT_PACKAGE_USER_RELEASE_ROUTE_ID) {
    throw new Error(
      `USER_RELEASE_ROUTE_REQUIRED: the generic route exporter does not admit the ${USER_RELEASED_REPORT_CONTEXT} context or route ${REPORT_PACKAGE_USER_RELEASE_ROUTE_ID}; only the user-release projector does`
    );
  }
}

export function controlRouteExport(
  payload: unknown,
  options: {
    routeId: string;
    exportContext: RedactionExportContext;
    explicitLocalPrivateIntent?: boolean;
    explicitUserReleaseIntent?: boolean;
    requireLosslessMaterialization?: boolean;
  }
): ControlledRouteExport {
  assertContext(options.exportContext);
  assertUserReleaseIntentContext(options.exportContext, options.explicitUserReleaseIntent);
  assertGenericRouteExport(options.routeId, options.exportContext);
  const localFirst = enforceLocalFirstRoute(options);
  const structuralProjection = structuralProjectionFor(payload, options.routeId);
  const projected = projectRouteValue(stripRouteIntent(payload), {
    routeId: options.routeId,
    path: "$",
    publicBasis: false,
    structuralProjection
  });
  const controlled = redactExportPayload(projected, {
    exportContext: options.exportContext,
    explicitLocalPrivateIntent: options.explicitLocalPrivateIntent ?? false,
    ...(options.explicitUserReleaseIntent === undefined ? {} : { explicitUserReleaseIntent: options.explicitUserReleaseIntent })
  });
  const destructive = controlled.decisions.some((decision) =>
    ["redact_value", "redact_field", "omit_field", "block_export"].includes(decision.action)
  );
  const materializationWithheld = Boolean(options.requireLosslessMaterialization && destructive);
  const blocked = controlled.blocked || materializationWithheld || localFirst.blocked;
  return {
    ...controlled,
    payload: blocked ? null : materializeRouteValue(controlled.payload),
    blocked,
    summary: {
      ...controlled.summary,
      route_id: options.routeId,
      materialization_withheld: materializationWithheld,
      local_first: localFirst
    }
  };
}

/** The label a user-release leaf takes from its enclosing objects (§3.2). */
type UserReleaseLabel = { privacy: unknown; redistribution: unknown; review: unknown };

const USER_RELEASE_DEFAULT_LABEL: UserReleaseLabel = {
  privacy: "private_project_data",
  redistribution: "private_only",
  review: undefined
};

const ABSENT: unique symbol = Symbol("user-release-absent");

function ownMember(object: unknown, key: string): unknown | typeof ABSENT {
  return isPlainObject(object) && Object.prototype.hasOwnProperty.call(object, key) ? object[key] : ABSENT;
}

function isStatedValue(value: unknown): boolean {
  return value !== ABSENT && value !== null && value !== undefined;
}

/** What an object states for one member, read as `classify` reads it: its own non-null
 * member, else its `provenance` object's member, else its own null member; ABSENT when it
 * states nothing. */
function statedMember(object: PlainObject, key: string): unknown | typeof ABSENT {
  const own = ownMember(object, key);
  if (isStatedValue(own)) return own;
  const fromProvenance = ownMember(object.provenance, key);
  if (fromProvenance !== ABSENT) return fromProvenance;
  return own;
}

/** The privacy and redistribution an object states, as the projector reads them (ABSENT
 * when it states none): its own `privacy_classification`, else its provenance's, else its
 * own `privacy_class`; its own `redistribution_status`, else its provenance's. */
function statedUserReleaseLabel(object: PlainObject): { privacy: unknown; redistribution: unknown } {
  const statedPrivacy = statedMember(object, "privacy_classification");
  return {
    privacy: statedPrivacy !== ABSENT ? statedPrivacy : ownMember(object, "privacy_class"),
    redistribution: statedMember(object, "redistribution_status")
  };
}

/** The values the classification gives a meaning in `user_released_report` (§3.2): the
 * privacy classes steps 8 and 10-12 decide (any other privacy, null included, falls to
 * step 13), and the redistribution statuses it recognizes (public, private, the
 * unknown forms it treats as unknown, and protected). Built from the classification's
 * own sets. */
const USER_RELEASE_RECOGNIZED_PRIVACY = new Set<unknown>([
  ...SAFE_PRIVACY_CLASSES,
  ...PRIVATE_PRIVACY_CLASSES,
  "protected_suspected"
]);
const USER_RELEASE_RECOGNIZED_REDISTRIBUTION = new Set<unknown>([
  ...PUBLIC_REDIS_STATUSES,
  ...PRIVATE_REDIS_STATUSES,
  ...UNKNOWN_REDIS_STATUSES,
  "protected_suspected"
]);

/** Whether an object's own stated label blocks by step 13 (§3.2), so that no descendant
 * can replace it: a stated privacy or redistribution the classification does not
 * recognize (`{}`, `[]`, a number, a boolean, an empty or unknown string; a null,
 * `unknown` or `TBD` privacy), or a stated pair that the classification itself decides by
 * step 13 (a safe class with a non-public redistribution). `decided` is the
 * classification of the object's own stated members alone; an absent member is not stated
 * and inherits. */
function ownUserReleaseLabelUnbound(
  stated: { privacy: unknown; redistribution: unknown },
  decided: RedactionDecision
): boolean {
  const privacyStated = stated.privacy !== ABSENT;
  const redistributionStated = stated.redistribution !== ABSENT;
  if (privacyStated && !USER_RELEASE_RECOGNIZED_PRIVACY.has(stated.privacy)) return true;
  if (redistributionStated && !USER_RELEASE_RECOGNIZED_REDISTRIBUTION.has(stated.redistribution)) return true;
  return privacyStated && redistributionStated && decided.reason_code === "USER_RELEASE_VALUE_UNBOUND";
}

/** The nearest enclosing object that states a member supplies it: an object's own
 * `privacy_classification` (else its provenance's, as `classify` reads them, else its own
 * `privacy_class`), `redistribution_status` and `review_status` replace what it inherits.
 * As in `classify`, a review status of `unknown` gives way to the provenance's. A blocking
 * disposition is never replaced by a nested statement: `protected_suspected` and a
 * `quarantined` or `rejected` review stay on every leaf beneath the object that states
 * them. A label can add a refusal; no label grants inclusion, since the wrapper's intent
 * alone admits project data. */
function nextUserReleaseLabel(object: PlainObject, inherited: UserReleaseLabel): UserReleaseLabel {
  const { privacy, redistribution } = statedUserReleaseLabel(object);
  const ownReview = ownMember(object, "review_status");
  const provenanceReview = ownMember(object.provenance, "review_status");
  const review =
    isStatedValue(ownReview) && ownReview !== "unknown"
      ? ownReview
      : provenanceReview !== ABSENT
        ? provenanceReview
        : ownReview;
  return {
    privacy:
      inherited.privacy === "protected_suspected" || privacy === ABSENT ? inherited.privacy : privacy,
    redistribution:
      inherited.redistribution === "protected_suspected" || redistribution === ABSENT
        ? inherited.redistribution
        : redistribution,
    review:
      inherited.review === "quarantined" || inherited.review === "rejected" || review === ABSENT
        ? inherited.review
        : review
  };
}

/** The release projector's screen of one value (§3.2). Every enclosing object is first
 * decided by steps 1-9 exactly as `classifyExportItem` decides it, from its own members
 * (claim, network, SQL, storage bypass, secret, payload and path markers; protected,
 * quarantined and rejected dispositions, its provenance's included); a decision there is
 * recorded for the object. Otherwise an object's explicitly stated own label that the
 * classification does not recognize, or decides by step 13, is recorded for the object as
 * `block_export` / `USER_RELEASE_VALUE_UNBOUND`, so no descendant's label can replace it;
 * only an absent label inherits. Only then is each leaf labelled with its nearest
 * enclosing metadata and decided by steps 8-13. */
function screenUserReleaseValue(
  value: unknown,
  args: {
    routeId: string;
    path: string;
    label: UserReleaseLabel;
    explicitUserReleaseIntent: boolean;
    decisions: RedactionDecision[];
    findings: RedactionFinding[];
  }
): void {
  const decide = (item: PlainObject) =>
    classify(item, {
      path: args.path,
      exportContext: USER_RELEASED_REPORT_CONTEXT,
      explicitLocalPrivateIntent: false,
      explicitUserReleaseIntent: args.explicitUserReleaseIntent,
      decisionIndex: args.decisions.length + 1,
      findingIndex: args.findings.length + 1
    });
  const push = (decision: RedactionDecision): void => {
    args.decisions.push(decision);
    const finding = findingFor(decision, args.findings.length + 1);
    if (finding !== null) args.findings.push(finding);
  };
  const fieldId = `${args.routeId}:${args.path}`;
  const fieldClass = args.path.split(".").at(-1)?.split("[")[0] || "unknown";
  if (Array.isArray(value)) {
    value.forEach((item, index) => screenUserReleaseValue(item, { ...args, path: `${args.path}[${index}]` }));
    return;
  }
  if (isPlainObject(value)) {
    const screened = decide(value);
    if (screened.boundary) {
      push(screened.decision);
    } else {
      // The object's own stated label, decided alone (§3.2): an unrecognized or step-13 own
      // label blocks here, before any descendant could replace it for its leaves.
      const stated = statedUserReleaseLabel(value);
      if (stated.privacy !== ABSENT || stated.redistribution !== ABSENT) {
        const { decision } = decide({
          field_id: fieldId,
          field_class: fieldClass,
          ...(stated.privacy === ABSENT ? {} : { privacy_classification: stated.privacy }),
          ...(stated.redistribution === ABSENT ? {} : { redistribution_status: stated.redistribution })
        });
        if (ownUserReleaseLabelUnbound(stated, decision)) {
          push({ ...decision, action: "block_export", reason_code: "USER_RELEASE_VALUE_UNBOUND" });
        }
      }
    }
    const label = nextUserReleaseLabel(value, args.label);
    for (const [key, item] of Object.entries(value)) {
      screenUserReleaseValue(item, { ...args, path: `${args.path}.${key}`, label });
    }
    return;
  }
  push(
    decide({
      field_id: fieldId,
      field_class: fieldClass,
      privacy_classification: args.label.privacy,
      redistribution_status: args.label.redistribution,
      ...(args.label.review === undefined ? {} : { review_status: args.label.review })
    }).decision
  );
}

/** The user-release projector for route `DREP-PACKAGE-USER-RELEASE-010` (§3.2), lossless.
 * The request and the model are screened alike: every enclosing object by steps 1-9 as
 * `classifyExportItem` decides it, then every leaf labelled with the privacy and
 * redistribution of its nearest enclosing object that states them, else
 * `private_project_data` / `private_only`; there is no path-token labelling and no field
 * table. The release intent is read only from the wrapper's argument; payload members of
 * any name grant nothing and are not stripped, so an admitted payload is the request
 * itself. The model is never part of the payload. This is the webview's early check;
 * native admission's derivation decides (§2.6). No route uses it before W-18b (r3). */
export function controlUserReleasedReportPackageRequest(
  payload: unknown,
  model: unknown,
  explicitUserReleaseIntent: boolean
): ControlledRouteExport {
  const routeId = REPORT_PACKAGE_USER_RELEASE_ROUTE_ID;
  const releaseIntent = explicitUserReleaseIntent === true;
  const localFirst = enforceLocalFirstRoute({
    routeId,
    exportContext: USER_RELEASED_REPORT_CONTEXT,
    explicitUserReleaseIntent: releaseIntent
  });
  const decisions: RedactionDecision[] = [];
  const findings: RedactionFinding[] = [];
  const screen = { routeId, label: USER_RELEASE_DEFAULT_LABEL, explicitUserReleaseIntent: releaseIntent, decisions, findings };
  screenUserReleaseValue(payload, { ...screen, path: "$.request" });
  screenUserReleaseValue(model, { ...screen, path: "$.model" });
  const decisionsBlock = decisions.some((decision) => decision.action === "block_export");
  const materializationWithheld = decisions.some((decision) =>
    ["redact_value", "redact_field", "omit_field", "block_export"].includes(decision.action)
  );
  const blocked = decisionsBlock || materializationWithheld || localFirst.blocked;
  return {
    payload: blocked ? null : structuredClone(payload),
    decisions,
    findings,
    blocked,
    summary: {
      ...summarize(decisions, findings),
      route_id: routeId,
      materialization_withheld: materializationWithheld,
      local_first: localFirst
    }
  };
}
