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

export type RedactionExportContext =
  | "public_report"
  | "public_example"
  | "shared_model"
  | "downstream_tool"
  | "local_private";

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
  | "CONCRETE_PATH_REDACTED";

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
  export_context: RedactionExportContext;
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

export type ControlledRouteExport = RedactionRunResult & {
  summary: RedactionSummary & {
    route_id: string;
    materialization_withheld: boolean;
  };
};

export const EXPORT_CONTEXTS: RedactionExportContext[] = [
  "public_report",
  "public_example",
  "shared_model",
  "downstream_tool",
  "local_private"
];

const SAFE_PRIVACY_CLASSES = new Set(["public", "public_metadata", "invented_public_example"]);
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

function truthy(value: unknown): boolean {
  if (typeof value === "boolean") return value;
  if (typeof value === "string") {
    return ["true", "yes", "explicit", "present", "local_private", "explicit_local_private"].includes(
      value.trim().toLowerCase()
    );
  }
  if (typeof value === "object" && value !== null) return false;
  return Boolean(value);
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
  return truthy(item.contains_payload) || truthy(item.payload_present) || intersects(PAYLOAD_DETAIL_KEYS, item);
}

function secretMaterialPresent(item: PlainObject): boolean {
  return truthy(item.secret_material_present) || intersects(SECRET_MATERIAL_KEYS, item);
}

function cloudOrNetworkReference(item: PlainObject): boolean {
  return truthy(item.cloud_or_network_reference) || intersects(CLOUD_NETWORK_DETAIL_KEYS, item);
}

function directSqlAccess(item: PlainObject): boolean {
  return truthy(item.direct_sql_access) || intersects(DIRECT_SQL_DETAIL_KEYS, item);
}

function storageBypassRequested(item: PlainObject): boolean {
  return ["storage_bypass", "storage_bypass_requested", "direct_storage_bypass", "application_service_bypass"].some(
    (key) => truthy(item[key])
  );
}

function concretePathPresent(item: PlainObject): boolean {
  return truthy(item.concrete_path_present) || intersects(CONCRETE_PATH_DETAIL_KEYS, item);
}

function itemIntent(item: PlainObject): boolean | null {
  for (const key of ["local_private_intent", "explicit_local_private_intent", "user_intent"]) {
    if (key in item) return truthy(item[key]);
  }
  const exportPolicy = item.export_policy;
  if (isPlainObject(exportPolicy)) {
    for (const key of ["local_private_intent", "explicit_local_private_intent", "user_intent"]) {
      if (key in exportPolicy) return truthy(exportPolicy[key]);
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

function actionFor(args: {
  privacy: unknown;
  redistribution: unknown;
  reviewStatus: unknown;
  metadataPresent: boolean;
  exportContext: RedactionExportContext;
  explicitLocalPrivateIntent: boolean;
  professionalClaim: boolean;
  containsPayload: boolean;
  secretMaterialPresent: boolean;
  cloudOrNetworkReference: boolean;
  directSqlAccess: boolean;
  storageBypass: boolean;
  concretePathPresent: boolean;
}): { action: RedactionAction; reason: RedactionReasonCode } {
  if (args.professionalClaim) return { action: "block_export", reason: "PROFESSIONAL_BOUNDARY_BLOCKED" };
  if (args.cloudOrNetworkReference) return { action: "block_export", reason: "CLOUD_OR_NETWORK_REFERENCE_BLOCKED" };
  if (args.directSqlAccess) return { action: "block_export", reason: "DIRECT_SQL_ACCESS_BLOCKED" };
  if (args.storageBypass) return { action: "block_export", reason: "STORAGE_BYPASS_BLOCKED" };
  if (args.secretMaterialPresent) return { action: "block_export", reason: "SECRET_MATERIAL_BLOCKED" };
  if (args.containsPayload) return { action: "block_export", reason: "PAYLOAD_METADATA_ONLY_REQUIRED" };
  if (args.concretePathPresent) return { action: "redact_value", reason: "CONCRETE_PATH_REDACTED" };
  if (args.privacy === "protected_suspected" || args.redistribution === "protected_suspected") {
    return { action: "block_export", reason: "PROTECTED_CONTENT_BLOCKED" };
  }
  if (args.reviewStatus === "quarantined" || args.reviewStatus === "rejected") {
    return { action: "block_export", reason: "PROTECTED_CONTENT_BLOCKED" };
  }
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
  CONCRETE_PATH_REDACTED: "STORAGE_BOUNDARY_WARNING"
};

const FINDING_MESSAGE_BY_ACTION: Record<string, string> = {
  warning_only: "Export retained the value only in a local/private context with an explicit warning.",
  redact_value: "Export value was replaced because metadata does not support public/shared release.",
  redact_field: "Export field was redacted because metadata does not support release.",
  omit_field: "Export field was omitted because metadata does not support release.",
  block_export: "Export is blocked until the metadata or boundary issue is resolved."
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
    message: FINDING_MESSAGE_BY_ACTION[decision.action],
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
    exportContext: RedactionExportContext;
    explicitLocalPrivateIntent: boolean;
    decisionIndex: number;
    findingIndex: number;
  }
): { decision: RedactionDecision; finding: RedactionFinding | null } {
  const metadataPresent = hasExplicitMetadata(item) || intersects(UNSAFE_DETAIL_KEYS, item);
  const fieldClass = stringOrUnknown(item.field_class ?? "unknown");
  let privacy: unknown = item.privacy_classification;
  let redistribution: unknown = item.redistribution_status;
  let reviewStatus: unknown = item.review_status ?? "unknown";
  const intent = localPrivateIntent(item, args.explicitLocalPrivateIntent);

  const provenance = item.provenance;
  if (isPlainObject(provenance)) {
    privacy = privacy !== null && privacy !== undefined ? privacy : provenance.privacy_classification;
    redistribution =
      redistribution !== null && redistribution !== undefined ? redistribution : provenance.redistribution_status;
    reviewStatus =
      reviewStatus !== null && reviewStatus !== undefined && reviewStatus !== "unknown"
        ? reviewStatus
        : (provenance.review_status ?? "unknown");
  }

  const { action, reason } = actionFor({
    privacy,
    redistribution,
    reviewStatus,
    metadataPresent,
    exportContext: args.exportContext,
    explicitLocalPrivateIntent: intent,
    professionalClaim: truthy(item.professional_claim ?? false),
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
  return { decision, finding: findingFor(decision, args.findingIndex) };
}

export function classifyExportItem(
  item: PlainObject,
  options: {
    exportContext: RedactionExportContext;
    explicitLocalPrivateIntent?: boolean;
    path?: string;
  }
): RedactionDecision {
  assertContext(options.exportContext);
  return classify(item, {
    path: options.path ?? "$",
    exportContext: options.exportContext,
    explicitLocalPrivateIntent: options.explicitLocalPrivateIntent ?? false,
    decisionIndex: 1,
    findingIndex: 1
  }).decision;
}

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
  if (!EXPORT_CONTEXTS.includes(exportContext as RedactionExportContext)) {
    throw new Error(`unsupported export_context: ${exportContext}`);
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

export function redactExportPayload(
  payload: unknown,
  options: { exportContext: RedactionExportContext; explicitLocalPrivateIntent?: boolean }
): RedactionRunResult {
  assertContext(options.exportContext);
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
    const directPrivacy = value.privacy_classification ?? value.classification;
    const boundedPublicBasis =
      ["public", "public_metadata", "invented_public_example"].includes(String(directPrivacy)) &&
        ["public_permissive", "invented_non_engineering_example"].includes(
          String(value.redistribution_status)
        );
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

export function controlRouteExport(
  payload: unknown,
  options: {
    routeId: string;
    exportContext: RedactionExportContext;
    explicitLocalPrivateIntent?: boolean;
    requireLosslessMaterialization?: boolean;
  }
): ControlledRouteExport {
  const structuralProjection = structuralProjectionFor(payload, options.routeId);
  const projected = projectRouteValue(stripRouteIntent(payload), {
    routeId: options.routeId,
    path: "$",
    publicBasis: false,
    structuralProjection
  });
  const controlled = redactExportPayload(projected, {
    exportContext: options.exportContext,
    explicitLocalPrivateIntent: options.explicitLocalPrivateIntent ?? false
  });
  const destructive = controlled.decisions.some((decision) =>
    ["redact_value", "redact_field", "omit_field", "block_export"].includes(decision.action)
  );
  const materializationWithheld = Boolean(options.requireLosslessMaterialization && destructive);
  const blocked = controlled.blocked || materializationWithheld;
  return {
    ...controlled,
    payload: blocked ? null : materializeRouteValue(controlled.payload),
    blocked,
    summary: {
      ...controlled.summary,
      route_id: options.routeId,
      materialization_withheld: materializationWithheld
    }
  };
}
