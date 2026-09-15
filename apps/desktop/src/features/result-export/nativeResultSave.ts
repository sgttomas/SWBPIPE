import { invoke } from "@tauri-apps/api/core";
import { isTauriRuntime } from "../../services/nativeMenu";
import type { LocalFirstRouteEvidence } from "../redaction-controls/redactionExportControls";

export const isNativeResultSaveRuntime = isTauriRuntime;
export type NativeResultSaveRequest = {
  href: string;
  file_name: string;
  screening: {
    route_id: string;
    export_context: string;
    explicit_local_private_intent: boolean;
    blocked: boolean;
    materialization_withheld: boolean;
    lossless_required: boolean;
    exact_payload_match: boolean;
    blocking_count: number;
  };
  local_first: LocalFirstRouteEvidence;
};
export type NativeResultSaveReceipt = {
  outcome: "saved";
  file_name: string;
  byte_count: number;
  replaced_existing: false;
  durability: "not_guaranteed";
  path_containment: "best_effort_non_adversarial";
};
export type NativeResultSaveError = {
  code: string;
  stage: string;
  message: string;
  partial_file_name: string | null;
  cleanup: "not_needed" | "removed" | "retained" | "failed" | "unknown";
};
const requestedNames = {
  "DOTH-JSON-001": /^openpipestress-preview-results-[a-z0-9]+(?:-[a-z0-9]+)*\.json$/,
  "DOTH-FORMAT-003": /^openpipestress-preview-stress-neutral-[a-z0-9]+(?:-[a-z0-9]+)*\.json$/
} as const;
const receiptName = /^openpipestress-preview-(?:results|stress-neutral)-[a-z0-9]+(?:-[a-z0-9]+)*(?: \([1-9][0-9]*\))?\.json$/;
const cleanups = ["not_needed", "removed", "retained", "failed", "unknown"];
function object(value: unknown): value is Record<string, unknown> {
  return value !== null && typeof value === "object" && !Array.isArray(value);
}
function failure(code: string, stage: string, cleanup: NativeResultSaveError["cleanup"]): NativeResultSaveError {
  return { code, stage, cleanup, message: "Local result save did not complete.", partial_file_name: null };
}
function normalizeError(error: unknown, request: NativeResultSaveRequest): NativeResultSaveError {
  let value = error;
  if (typeof value === "string") {
    try { value = JSON.parse(value); } catch { return failure("IPC_REJECTED", "invoke", "unknown"); }
  }
  if (object(value) && typeof value.code === "string" && /^[A-Z_]+$/.test(value.code)
    && typeof value.stage === "string" && ["request", "decode", "downloads", "containment", "open", "write", "flush", "busy", "worker"].includes(value.stage)
    && typeof value.cleanup === "string" && cleanups.includes(value.cleanup)
    && (value.partial_file_name === null || (typeof value.partial_file_name === "string"
      && receiptName.test(value.partial_file_name) && sameRequestedFamily(value.partial_file_name, request.file_name)))) {
    return { code: value.code, stage: value.stage, message: "Local result save did not complete.",
      partial_file_name: value.partial_file_name as string | null, cleanup: value.cleanup as NativeResultSaveError["cleanup"] };
  }
  return failure("IPC_REJECTED", "invoke", "unknown");
}
function admitted(request: NativeResultSaveRequest): boolean {
  if (!object(request) || !object(request.screening) || !object(request.local_first)) return false;
  const s = request.screening, l = request.local_first;
  return Object.keys(request).sort().join(",") === "file_name,href,local_first,screening"
    && Object.keys(s).sort().join(",") === "blocked,blocking_count,exact_payload_match,explicit_local_private_intent,export_context,lossless_required,materialization_withheld,route_id"
    && Object.keys(l).sort().join(",") === "action,blocked,explicit_local_private_intent,export_context,metadata_only,reason_code,route_id,storage_context"
    && typeof request.href === "string" && request.href.startsWith("data:application/json;charset=utf-8,") && request.href.length > "data:application/json;charset=utf-8,".length
    && typeof request.file_name === "string"
    && (s.route_id === "DOTH-JSON-001" || s.route_id === "DOTH-FORMAT-003")
    && requestedNames[s.route_id].test(request.file_name)
    && s.export_context === "local_private" && s.explicit_local_private_intent === true
    && s.blocked === false && s.materialization_withheld === false && s.lossless_required === true && s.exact_payload_match === true && s.blocking_count === 0
    && l.route_id === s.route_id && l.export_context === s.export_context && l.storage_context === "local_private"
    && l.action === "include_metadata_only" && l.reason_code === "PRIVATE_LOCAL_METADATA_ALLOWED"
    && l.blocked === false && l.metadata_only === true && l.explicit_local_private_intent === true;
}
function sameRequestedFamily(value: string, basename: string): boolean {
  const stem = basename.slice(0, -5);
  if (value === basename) return true;
  const suffix = value.startsWith(`${stem} (`) && value.endsWith(").json") ? value.slice(stem.length + 2, -6) : "";
  return /^[1-9][0-9]*$/.test(suffix);
}
function receipt(value: unknown, request: NativeResultSaveRequest): value is NativeResultSaveReceipt {
  return object(value) && Object.keys(value).sort().join(",") === "byte_count,durability,file_name,outcome,path_containment,replaced_existing"
    && value.outcome === "saved" && typeof value.file_name === "string" && receiptName.test(value.file_name) && sameRequestedFamily(value.file_name, request.file_name)
    && typeof value.byte_count === "number" && Number.isSafeInteger(value.byte_count) && value.byte_count > 0
    && value.replaced_existing === false && value.durability === "not_guaranteed"
    && value.path_containment === "best_effort_non_adversarial";
}

// Forward the screened string unchanged. Decoding and exact-byte writes belong
// only to the private native boundary; neither policy projection nor this service
// reconstructs the canonical document.
export async function saveNativeResultJson(request: NativeResultSaveRequest): Promise<NativeResultSaveReceipt> {
  if (!isTauriRuntime()) throw failure("NATIVE_RUNTIME_REQUIRED", "invoke", "not_needed");
  if (!admitted(request)) throw failure("REQUEST_EVIDENCE_DENIED", "request", "not_needed");
  let value: unknown;
  try { value = await invoke<unknown>("save_local_result_json", { request }); }
  catch (error) { throw normalizeError(error, request); }
  if (!receipt(value, request)) throw failure("INVALID_SAVE_RECEIPT", "worker", "unknown");
  return value;
}
