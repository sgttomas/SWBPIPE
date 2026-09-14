import { afterEach, describe, expect, it, vi } from "vitest";
import { invoke } from "@tauri-apps/api/core";
import { saveNativeResultJson, type NativeResultSaveRequest } from "./nativeResultSave";
vi.mock("@tauri-apps/api/core", () => ({ invoke: vi.fn() }));
afterEach(() => { delete (window as any).__TAURI_INTERNALS__; vi.mocked(invoke).mockReset(); });
const request: NativeResultSaveRequest = {
  href: "data:application/json;charset=utf-8,%7B%22text%22%3A%22%C3%A9%2B%2525%5Cn%22%7D",
  file_name: "openpipestress-preview-results-run-350.json",
  screening: { route_id: "DOTH-JSON-001", export_context: "local_private", explicit_local_private_intent: true,
    blocked: false, materialization_withheld: false, lossless_required: true, exact_payload_match: true, blocking_count: 0 },
  local_first: { route_id: "DOTH-JSON-001", export_context: "local_private", storage_context: "local_private",
    action: "include_metadata_only", reason_code: "PRIVATE_LOCAL_METADATA_ALLOWED", blocked: false, metadata_only: true, explicit_local_private_intent: true }
};
const receipt = { outcome: "saved", file_name: "openpipestress-preview-results-run-350 (1).json", byte_count: 27,
  replaced_existing: false, durability: "not_guaranteed", path_containment: "best_effort_non_adversarial" };
describe("private native result service", () => {
  it("forwards the original screened request/href unchanged and accepts only typed completion", async () => {
    (window as any).__TAURI_INTERNALS__ = {}; vi.mocked(invoke).mockResolvedValue(receipt);
    expect(await saveNativeResultJson(request)).toEqual(receipt);
    expect(invoke).toHaveBeenCalledExactlyOnceWith("save_local_result_json", { request });
    expect((vi.mocked(invoke).mock.calls[0][1] as { request: NativeResultSaveRequest }).request).toBe(request);
  });
  it("does not invoke outside native runtime", async () => {
    await expect(saveNativeResultJson(request)).rejects.toMatchObject({ code: "NATIVE_RUNTIME_REQUIRED", cleanup: "not_needed" });
    expect(invoke).not.toHaveBeenCalled();
  });
  it("rejects missing, forged and contradictory success receipts", async () => {
    (window as any).__TAURI_INTERNALS__ = {};
    for (const value of [null, {}, { ...receipt, outcome: "pending" }, { ...receipt, byte_count: 0 }, { ...receipt, byte_count: 1.5 }, { ...receipt, replaced_existing: true }, { ...receipt, durability: "guaranteed" }, { ...receipt, file_name: "../outside.json" }, { ...receipt, href: request.href }, { ...receipt, file_name: "openpipestress-preview-results-unrelated.json" }, { ...receipt, file_name: "openpipestress-preview-results-unrelated (1).json" }]) {
      vi.mocked(invoke).mockResolvedValue(value);
      await expect(saveNativeResultJson(request)).rejects.toMatchObject({ code: "INVALID_SAVE_RECEIPT", cleanup: "unknown" });
    }
  });
  it("denied contradictory request evidence and path injection never invoke", async () => {
    (window as any).__TAURI_INTERNALS__ = {};
    const requests = [
      { ...request, file_name: "../outside.json" }, { ...request, file_name: "openpipestress-preview-results-run-350 (1).json" },
      ...Object.entries({ route_id: "other", export_context: "public_report", explicit_local_private_intent: false, blocked: true, materialization_withheld: true, lossless_required: false, exact_payload_match: false, blocking_count: 1 }).map(([key, value]) => ({ ...request, screening: { ...request.screening, [key]: value } })),
      ...Object.entries({ route_id: "other", export_context: "public_report", storage_context: "remote", action: "block_storage", reason_code: "SAFE_PUBLIC_METADATA", blocked: true, metadata_only: false, explicit_local_private_intent: false }).map(([key, value]) => ({ ...request, local_first: { ...request.local_first, [key]: value } })),
      { ...request, directory: "/tmp" }, { ...request, screening: { ...request.screening, unexpected: true } }
    ];
    for (const denied of requests) await expect(saveNativeResultJson(denied as NativeResultSaveRequest)).rejects.toMatchObject({ code: "REQUEST_EVIDENCE_DENIED", stage: "request", cleanup: "not_needed" });
    expect(invoke).not.toHaveBeenCalled();
  });
  it("keeps native primary error and cleanup status without echoing payload", async () => {
    (window as any).__TAURI_INTERNALS__ = {};
    const error = { code: "WRITE_FAILED", stage: "write", message: "INVENTED_PRIVATE_PAYLOAD", partial_file_name: request.file_name, cleanup: "failed" };
    vi.mocked(invoke).mockRejectedValue(error);
    await expect(saveNativeResultJson(request)).rejects.toMatchObject({ code: "WRITE_FAILED", stage: "write", cleanup: "failed", partial_file_name: request.file_name, message: "Local result save did not complete." });
    vi.mocked(invoke).mockRejectedValue(JSON.stringify({ ...error, cleanup: "retained" }));
    await expect(saveNativeResultJson(request)).rejects.toMatchObject({ cleanup: "retained" });
    vi.mocked(invoke).mockRejectedValue("transport rejected with opaque private text");
    await expect(saveNativeResultJson(request)).rejects.toMatchObject({ code: "IPC_REJECTED", cleanup: "unknown", partial_file_name: null });
  });
});
