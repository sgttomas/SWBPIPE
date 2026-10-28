// RT-R6 CSV save tests (DEL-08-04 AC-002, VER-002; Design `csv-export.md` §4).
import { afterEach, describe, expect, it, vi } from "vitest";
import { invoke } from "@tauri-apps/api/core";
import { ingressArgs } from "../../test/nativeIngress";
import { csvSaveRequest, saveCsvTable } from "./csvSave";
import { csvNumber, csvText, serializeCsvTable } from "./csvTable";
import { saveNativeResultJson, type NativeResultSaveRequest } from "./nativeResultSave";

vi.mock("@tauri-apps/api/core", () => ({ invoke: vi.fn() }));
afterEach(() => { delete (window as any).__TAURI_INTERNALS__; vi.mocked(invoke).mockReset(); vi.restoreAllMocks(); });

const text = serializeCsvTable({
  preamble: { table: "stress-ratio", softwareVersion: "0.9.0", modelHash: "sha256:11", resultHash: "sha256:22",
    analysisRoutes: ["linear-static"], caseOrFilter: "=SUS, \"case\"", unitBasis: "SI" },
  columns: [{ kind: "text", label: "Node" }, { kind: "number", label: "S", unit: "MPa" }],
  rows: [[csvText("10"), csvNumber(-12.5)], [csvText("é ☃ %25"), csvNumber(1e-7)]]
});
const fileName = "swbpipe-stress-ratio-sus.csv";
const receipt = { outcome: "saved", file_name: fileName, byte_count: new TextEncoder().encode(text).length,
  replaced_existing: false, durability: "not_guaranteed", path_containment: "best_effort_non_adversarial" };

describe("CSV save request (DOTH-CSV-011)", () => {
  it("screens the text on DOTH-CSV-011 for local_private with explicit intent, lossless_required=false", () => {
    const request = csvSaveRequest({ text, fileName, explicitLocalPrivateIntent: true });
    expect(request.href.startsWith("data:text/csv;charset=utf-8,%EF%BB%BF")).toBe(true);
    expect(decodeURIComponent(request.href.slice("data:text/csv;charset=utf-8,".length))).toBe(text);
    expect(request).toEqual({
      href: request.href,
      file_name: fileName,
      screening: { route_id: "DOTH-CSV-011", export_context: "local_private", explicit_local_private_intent: true, blocked: false,
        materialization_withheld: false, lossless_required: false, exact_payload_match: true, blocking_count: 0 },
      local_first: { route_id: "DOTH-CSV-011", export_context: "local_private", storage_context: "local_private",
        action: "include_metadata_only", reason_code: "PRIVATE_LOCAL_METADATA_ALLOWED", blocked: false, metadata_only: true,
        explicit_local_private_intent: true }
    });
  });

  it("refuses without explicit intent, with a non-CSV name, or with text that is not serializer output", () => {
    for (const intent of [false, undefined, "true", 1]) {
      expect(() => csvSaveRequest({ text, fileName, explicitLocalPrivateIntent: intent as boolean })).toThrow(
        expect.objectContaining({ code: "LOCAL_PRIVATE_INTENT_REQUIRED", stage: "request", cleanup: "not_needed" })
      );
    }
    for (const name of ["swbpipe-stress-ratio.json", "openpipestress-preview-results-run-1.json", "swbpipe-results.csv", "../swbpipe-stress-ratio.csv", "swbpipe-stress-ratio (1).csv"]) {
      expect(() => csvSaveRequest({ text, fileName: name, explicitLocalPrivateIntent: true })).toThrow(expect.objectContaining({ code: "INVALID_FILE_NAME" }));
    }
    for (const bad of [text.slice(1), "﻿", "", 12]) {
      expect(() => csvSaveRequest({ text: bad as string, fileName, explicitLocalPrivateIntent: true })).toThrow(expect.objectContaining({ code: "INVALID_CSV_TEXT" }));
    }
  });
});

describe("saveCsvTable", () => {
  it("in the desktop app, forwards the screened request to the existing native save command", async () => {
    (window as any).__TAURI_INTERNALS__ = {};
    vi.mocked(invoke).mockResolvedValue(receipt);
    const click = vi.spyOn(HTMLAnchorElement.prototype, "click");
    expect(await saveCsvTable({ text, fileName, explicitLocalPrivateIntent: true })).toEqual({ mode: "native", receipt });
    expect(invoke).toHaveBeenCalledOnce();
    expect(vi.mocked(invoke).mock.calls[0][0]).toBe("save_local_result_json");
    const sent = ingressArgs(vi.mocked(invoke).mock.calls[0]).request as NativeResultSaveRequest;
    expect(sent).toEqual(csvSaveRequest({ text, fileName, explicitLocalPrivateIntent: true }));
    expect(click).not.toHaveBeenCalled();
  });

  it("accepts a collision-suffixed CSV receipt and refuses a JSON or unrelated one", async () => {
    (window as any).__TAURI_INTERNALS__ = {};
    const suffixed = { ...receipt, file_name: "swbpipe-stress-ratio-sus (2).csv" };
    vi.mocked(invoke).mockResolvedValue(suffixed);
    expect(await saveCsvTable({ text, fileName, explicitLocalPrivateIntent: true })).toEqual({ mode: "native", receipt: suffixed });
    for (const name of ["swbpipe-stress-ratio-sus.json", "swbpipe-stress-ratio-sus (1).json", "swbpipe-weight.csv", "openpipestress-preview-results-run-1.json", "swbpipe-stress-ratio-sus (0).csv"]) {
      vi.mocked(invoke).mockResolvedValue({ ...receipt, file_name: name });
      await expect(saveCsvTable({ text, fileName, explicitLocalPrivateIntent: true }), name).rejects.toMatchObject({ code: "INVALID_SAVE_RECEIPT" });
    }
  });

  it("in the browser runtime, keeps the anchor download of the same bytes", async () => {
    let captured: { href: string; download: string; attached: boolean } | null = null;
    vi.spyOn(HTMLAnchorElement.prototype, "click").mockImplementation(function (this: HTMLAnchorElement) {
      captured = { href: this.href, download: this.download, attached: document.body.contains(this) };
    });
    const outcome = await saveCsvTable({ text, fileName, explicitLocalPrivateIntent: true });
    expect(outcome).toEqual({ mode: "browser", file_name: fileName, byte_count: receipt.byte_count });
    expect(captured).toEqual({ href: csvSaveRequest({ text, fileName, explicitLocalPrivateIntent: true }).href, download: fileName, attached: true });
    expect(document.querySelectorAll("a[download]").length).toBe(0);
    expect(invoke).not.toHaveBeenCalled();
  });

  it("never saves without explicit intent, in either runtime", async () => {
    const click = vi.spyOn(HTMLAnchorElement.prototype, "click");
    await expect(saveCsvTable({ text, fileName, explicitLocalPrivateIntent: false })).rejects.toMatchObject({ code: "LOCAL_PRIVATE_INTENT_REQUIRED" });
    (window as any).__TAURI_INTERNALS__ = {};
    await expect(saveCsvTable({ text, fileName, explicitLocalPrivateIntent: false })).rejects.toMatchObject({ code: "LOCAL_PRIVATE_INTENT_REQUIRED" });
    expect(click).not.toHaveBeenCalled();
    expect(invoke).not.toHaveBeenCalled();
  });
});

describe("native save admission for DOTH-CSV-011", () => {
  const csv = (): NativeResultSaveRequest => csvSaveRequest({ text, fileName, explicitLocalPrivateIntent: true });
  const jsonRequest: NativeResultSaveRequest = {
    href: "data:application/json;charset=utf-8,%7B%7D",
    file_name: "openpipestress-preview-results-run-350.json",
    screening: { route_id: "DOTH-JSON-001", export_context: "local_private", explicit_local_private_intent: true,
      blocked: false, materialization_withheld: false, lossless_required: true, exact_payload_match: true, blocking_count: 0 },
    local_first: { route_id: "DOTH-JSON-001", export_context: "local_private", storage_context: "local_private",
      action: "include_metadata_only", reason_code: "PRIVATE_LOCAL_METADATA_ALLOWED", blocked: false, metadata_only: true, explicit_local_private_intent: true }
  };
  const stressRequest: NativeResultSaveRequest = {
    ...jsonRequest,
    file_name: "openpipestress-preview-stress-neutral-result-run-350.json",
    screening: { ...jsonRequest.screening, route_id: "DOTH-FORMAT-003" },
    local_first: { ...jsonRequest.local_first, route_id: "DOTH-FORMAT-003" }
  };

  it("refuses missing intent, every other context, a lossless claim and crossed prefixes or names before invoking", async () => {
    (window as any).__TAURI_INTERNALS__ = {};
    const base = csv();
    const denied: NativeResultSaveRequest[] = [
      { ...base, screening: { ...base.screening, explicit_local_private_intent: false } },
      { ...base, local_first: { ...base.local_first, explicit_local_private_intent: false } },
      { ...base, screening: { ...base.screening, lossless_required: true } },
      { ...base, href: `data:application/json;charset=utf-8,${base.href.slice("data:text/csv;charset=utf-8,".length)}` },
      { ...base, href: "data:text/csv;charset=utf-8," },
      { ...base, file_name: "openpipestress-preview-results-run-350.json" },
      { ...jsonRequest, href: `data:text/csv;charset=utf-8,${encodeURIComponent("{}")}` },
      { ...stressRequest, href: `data:text/csv;charset=utf-8,${encodeURIComponent("{}")}` },
      { ...jsonRequest, screening: { ...jsonRequest.screening, lossless_required: false } },
      { ...stressRequest, screening: { ...stressRequest.screening, lossless_required: false } },
      { ...jsonRequest, file_name: fileName },
      ...["public_report", "public_example", "shared_model", "downstream_tool", "user_released_report"].map((context) => ({
        ...base, screening: { ...base.screening, export_context: context }, local_first: { ...base.local_first, export_context: context, storage_context: context }
      }))
    ];
    for (const request of denied) {
      await expect(saveNativeResultJson(request), JSON.stringify(request.screening)).rejects.toMatchObject({ code: "REQUEST_EVIDENCE_DENIED", stage: "request" });
    }
    expect(invoke).not.toHaveBeenCalled();
  });
});
