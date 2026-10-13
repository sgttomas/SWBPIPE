// The raw-body ingress helper (DEL-04-07): bytes plus the ingress header, no
// JSON argument object; string rejections pass through; typed rejections are
// read as the native memory-pool verdicts they are. Mocked invoke only.
import { afterEach, describe, expect, it, vi } from "vitest";

const invokeMock = vi.hoisted(() => vi.fn());
vi.mock("@tauri-apps/api/core", () => ({ invoke: invokeMock }));

import { invokeWithIngress, NATIVE_INGRESS_COMMANDS, nativeFailureText } from "./nativeIngress";
import { ResourceUnavailableError } from "./memoryPool";
import { INGRESS_COMMANDS, ingressArgs } from "../test/nativeIngress";

const GiB = 1024 ** 3;
afterEach(() => invokeMock.mockReset());

describe("invokeWithIngress", () => {
  it("names exactly the commands the native host takes as a raw body", () => {
    expect(new Set(NATIVE_INGRESS_COMMANDS)).toEqual(INGRESS_COMMANDS);
  });

  it("sends the UTF-8 JSON bytes of the argument object with the ingress header and no JSON arguments", async () => {
    invokeMock.mockResolvedValue({ ok: true });
    const args = { model: { id: "Ω-1", value: -0, absent: undefined, list: [1.5, 2e-7] }, intent: { kind: "x" }, claimedModelHash: null };
    expect(await invokeWithIngress("apply_model_operation", args)).toEqual({ ok: true });
    expect(invokeMock).toHaveBeenCalledOnce();
    const call = invokeMock.mock.calls[0];
    expect(call[0]).toBe("apply_model_operation");
    expect(Object.prototype.toString.call(call[1])).toBe("[object Uint8Array]");
    expect(call[2]).toEqual({ headers: { "x-swbpipe-ingress": "args-json-v1" } });
    expect(call).toHaveLength(3);
    // Exactly the text Tauri would have sent for the same argument object.
    expect(new TextDecoder().decode(call[1] as Uint8Array)).toBe(JSON.stringify(args));
    expect(ingressArgs(call)).toEqual({ model: { id: "Ω-1", value: 0, list: [1.5, 2e-7] }, intent: { kind: "x" }, claimedModelHash: null });
  });

  it("passes a plain string rejection through unchanged", async () => {
    invokeMock.mockRejectedValue("PROJECT-BASE-MOVED: the stored project changed");
    await expect(invokeWithIngress("save_local_project", { request: {} })).rejects.toBe("PROJECT-BASE-MOVED: the stored project changed");
  });

  it("reads a typed resource refusal as a ResourceUnavailableError with its plain-language sentence", async () => {
    const pool = null;
    invokeMock.mockRejectedValue({ code: "RESOURCE_UNAVAILABLE", refusal: { kind: "headroom_insufficient", required_bytes: 3 * GiB, available_bytes: GiB }, pool });
    const failure = await invokeWithIngress("validate_model_operation", { model: {}, intent: {}, claimedModelHash: null }).catch((error: unknown) => error);
    expect(failure).toBeInstanceOf(ResourceUnavailableError);
    expect((failure as ResourceUnavailableError).refusal).toEqual({ kind: "headroom_insufficient", required_bytes: 3 * GiB, available_bytes: GiB });
    expect((failure as Error).message).toBe("RESOURCE_UNAVAILABLE: Not enough free memory on this Mac right now: needs 3 GiB, 1 GiB available.");
    expect(nativeFailureText(failure)).toBe("Refused by the native memory pool: Not enough free memory on this Mac right now: needs 3 GiB, 1 GiB available.");
  });

  it("reads a typed invalid request as an INVALID_REQUEST error", async () => {
    invokeMock.mockRejectedValue({ code: "INVALID_REQUEST", message: "the request body is not valid JSON" });
    await expect(invokeWithIngress("convert_display_quantities", { input: { items: [] } })).rejects.toThrow("INVALID_REQUEST: the request body is not valid JSON");
  });

  it("keeps any other failure's text as before", () => {
    expect(nativeFailureText(new Error("boom"))).toBe("Error: boom");
    expect(nativeFailureText("PROJECT-ID-EXISTS: p")).toBe("PROJECT-ID-EXISTS: p");
  });
});
