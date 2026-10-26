// The raw-body ingress helper (DEL-04-07): bytes plus the ingress header, no
// JSON argument object; string rejections pass through; typed rejections are
// read as the native memory-pool verdicts they are. Mocked invoke only.
import { afterEach, describe, expect, it, vi } from "vitest";

const invokeMock = vi.hoisted(() => vi.fn());
vi.mock("@tauri-apps/api/core", () => ({ invoke: invokeMock }));

import { HEADROOM_RESEND_INTERVAL_MS, HEADROOM_RESEND_WINDOW_MS, IngressRequestRetiredError, invokeWithIngress, NATIVE_INGRESS_COMMANDS, nativeFailureText } from "./nativeIngress";
import { ResourceUnavailableError } from "./memoryPool";
import { INGRESS_COMMANDS, ingressArgs } from "../test/nativeIngress";

const GiB = 1024 ** 3;
afterEach(() => {
  invokeMock.mockReset();
  vi.useRealTimers();
});

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
    vi.useFakeTimers();
    const pool = null;
    invokeMock.mockRejectedValue({ code: "RESOURCE_UNAVAILABLE", refusal: { kind: "headroom_insufficient", required_bytes: 3 * GiB, available_bytes: GiB }, pool });
    const pending = invokeWithIngress("validate_model_operation", { model: {}, intent: {}, claimedModelHash: null }).catch((error: unknown) => error);
    await vi.runAllTimersAsync();
    const failure = await pending;
    expect(failure).toBeInstanceOf(ResourceUnavailableError);
    expect((failure as ResourceUnavailableError).refusal).toEqual({ kind: "headroom_insufficient", required_bytes: 3 * GiB, available_bytes: GiB });
    expect((failure as Error).message).toBe("RESOURCE_UNAVAILABLE: Not enough available memory on this Mac right now (free plus reclaimable file cache and purgeable memory, less the kernel's reserve): needs 3 GiB, 1 GiB available.");
    expect(nativeFailureText(failure)).toBe("Refused by the native memory pool: Not enough available memory on this Mac right now (free plus reclaimable file cache and purgeable memory, less the kernel's reserve): needs 3 GiB, 1 GiB available.");
  });

  const headroomRefusal = (required: number, available: number) => ({
    code: "RESOURCE_UNAVAILABLE",
    refusal: { kind: "headroom_insufficient", required_bytes: required, available_bytes: available },
    pool: null
  });

  it("re-sends the same bytes after a headroom_insufficient refusal and returns the first admitted result", async () => {
    vi.useFakeTimers();
    invokeMock
      .mockRejectedValueOnce(headroomRefusal(17_363_664, 6_752_829))
      .mockRejectedValueOnce(headroomRefusal(17_363_664, 0))
      .mockResolvedValueOnce({ outcome: "saved" });
    const pending = invokeWithIngress("save_local_result_json", { request: { href: "data:application/json;charset=utf-8,%7B%7D" } });
    await vi.runAllTimersAsync();
    expect(await pending).toEqual({ outcome: "saved" });
    expect(invokeMock).toHaveBeenCalledTimes(3);
    const bodies = invokeMock.mock.calls.map((call) => new TextDecoder().decode(call[1] as Uint8Array));
    expect(new Set(bodies).size).toBe(1);
    for (const call of invokeMock.mock.calls) expect(call[2]).toEqual({ headers: { "x-swbpipe-ingress": "args-json-v1" } });
  });

  it("stops re-sending once the window would be exceeded and reports the last refusal", async () => {
    vi.useFakeTimers();
    let sends = 0;
    invokeMock.mockImplementation(() => Promise.reject(headroomRefusal(17_363_664, ++sends)));
    const started = performance.now();
    const pending = invokeWithIngress("save_local_project", { request: {} }).catch((error: unknown) => error);
    await vi.runAllTimersAsync();
    const failure = await pending;
    expect(failure).toBeInstanceOf(ResourceUnavailableError);
    expect((failure as ResourceUnavailableError).refusal).toEqual({ kind: "headroom_insufficient", required_bytes: 17_363_664, available_bytes: sends });
    expect(performance.now() - started).toBeLessThanOrEqual(HEADROOM_RESEND_WINDOW_MS);
    // Sends at 0, 100, …, 2000 ms: none starts after the window.
    expect(sends).toBe(Math.floor(HEADROOM_RESEND_WINDOW_MS / HEADROOM_RESEND_INTERVAL_MS) + 1);
  });

  it("starts no send after the window even when a timer fires late", async () => {
    vi.useFakeTimers();
    invokeMock
      .mockRejectedValueOnce(headroomRefusal(17_363_664, 1))
      .mockResolvedValueOnce({ outcome: "saved" });
    const pending = invokeWithIngress("save_local_result_json", { request: {} }).catch((error: unknown) => error);
    await vi.advanceTimersByTimeAsync(0);
    expect(invokeMock).toHaveBeenCalledOnce();
    // The host stalls past the window before the pause's timer runs.
    const stalled = performance.now() + HEADROOM_RESEND_WINDOW_MS + 3000;
    const clock = vi.spyOn(performance, "now").mockReturnValue(stalled);
    await vi.runAllTimersAsync();
    clock.mockRestore();
    const failure = await pending;
    expect(failure).toBeInstanceOf(ResourceUnavailableError);
    expect((failure as ResourceUnavailableError).refusal).toEqual({ kind: "headroom_insufficient", required_bytes: 17_363_664, available_bytes: 1 });
    expect(invokeMock).toHaveBeenCalledOnce();
  });

  it("sends nothing for a request already retired", async () => {
    invokeMock.mockResolvedValue({ ok: true });
    await expect(invokeWithIngress("render_calculation_report", { input: {} }, { stillCurrent: () => false })).rejects.toBeInstanceOf(IngressRequestRetiredError);
    expect(invokeMock).not.toHaveBeenCalled();
  });

  it("does not re-send a request retired during the pause after a headroom refusal", async () => {
    vi.useFakeTimers();
    let current = true;
    invokeMock.mockRejectedValueOnce(headroomRefusal(17_363_664, 0)).mockResolvedValue({ ok: true });
    const pending = invokeWithIngress("render_calculation_report", { input: {} }, { stillCurrent: () => current }).catch((error: unknown) => error);
    await vi.advanceTimersByTimeAsync(0);
    expect(invokeMock).toHaveBeenCalledOnce();
    current = false;
    await vi.runAllTimersAsync();
    expect(await pending).toBeInstanceOf(IngressRequestRetiredError);
    expect(invokeMock).toHaveBeenCalledOnce();
  });

  it("does not re-send any other refusal or failure", async () => {
    for (const rejection of [
      { code: "RESOURCE_UNAVAILABLE", refusal: { kind: "pool_exhausted", requested_bytes: 1, reserved_bytes: 2, pool_bytes: 3 }, pool: null },
      { code: "RESOURCE_UNAVAILABLE", refusal: { kind: "headroom_unknown" }, pool: null },
      { code: "RESOURCE_UNAVAILABLE", refusal: { kind: "ingress_exceeds_budget", required_bytes: 2, budget_bytes: 1 }, pool: null },
      { code: "INVALID_REQUEST", message: "no" },
      "PROJECT-BASE-MOVED: the stored project changed"
    ]) {
      invokeMock.mockReset();
      invokeMock.mockRejectedValue(rejection);
      await invokeWithIngress("save_local_project", { request: {} }).catch(() => undefined);
      expect(invokeMock).toHaveBeenCalledOnce();
    }
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
