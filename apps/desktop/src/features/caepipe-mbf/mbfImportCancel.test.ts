// DEL-17-04 slice 5 (§4.8.6): the import's session barrier with a mocked native host. Each test loads a fresh module,
// so the barrier starts as a new page does.
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";

const invokeMock = vi.hoisted(() => vi.fn());
vi.mock("@tauri-apps/api/core", async () => ({ invoke: (await import("../../test/nativeIngress")).ingressDecodingInvoke(invokeMock) }));

type Barrier = typeof import("./mbfImportCancel");
let barrier: Barrier;

beforeEach(async () => {
  vi.resetModules();
  barrier = await import("./mbfImportCancel");
});
afterEach(() => { vi.useRealTimers(); invokeMock.mockReset(); });

function epochHost(options: { acknowledge?: () => boolean } = {}) {
  const host = { epoch: 0, calls: [] as Array<{ command: string; args?: Record<string, unknown> }> };
  invokeMock.mockImplementation((command: string, args?: Record<string, unknown>) => {
    host.calls.push({ command, args });
    if (command === "cancel_mbf_import") {
      if (options.acknowledge && !options.acknowledge()) return Promise.reject(new Error("lost"));
      host.epoch += 1;
      return Promise.resolve({ cancelled: 0, epoch: host.epoch, retirement_id: args?.retirementId });
    }
    return Promise.resolve({ outcome: "cancelled" });
  });
  return host;
}

describe("the MBF import's session barrier", () => {
  it("delivers a handshake as JSON arguments, then sends the start raw with its ticket; it never sends the export's cancel", async () => {
    const host = epochHost();
    await barrier.openMbfImport();
    expect(host.calls.map((call) => call.command)).toEqual(["cancel_mbf_import", "open_mbf_import"]);
    expect(host.calls[0].args).toEqual({ retirementId: { page: expect.stringMatching(/^[0-9a-f]{32}$/), sequence: 1 } });
    expect(host.calls[1].args).toEqual({ expectedEpoch: 1, requestSequence: 1 });
    await barrier.retireMbfImportSession();
    await barrier.reviewMbfImport({ sourceId: 1, declaration: null });
    expect(host.calls.at(-1)).toEqual({ command: "review_mbf_import", args: { sourceId: 1, declaration: null, expectedEpoch: 2, requestSequence: 2 } });
    expect(host.calls.some((call) => call.command === "cancel_mbf_export")).toBe(false);
  });

  it("re-delivers an unacknowledged retirement once with the same id, then refuses the start unconfirmed", async () => {
    let answers = 0;
    const host = epochHost({ acknowledge: () => { answers += 1; return false; } });
    const started = await barrier.openMbfImport();
    const cancels = host.calls.filter((call) => call.command === "cancel_mbf_import");
    expect(cancels).toHaveLength(2);
    expect(cancels[1].args).toEqual(cancels[0].args);
    expect(started.outcome).toMatchObject({ outcome: "refused", diagnostics: [{ code: "MBF-IMPORT-RETIREMENT-UNCONFIRMED" }] });
    expect(host.calls.some((call) => call.command === "open_mbf_import")).toBe(false);
    expect(answers).toBe(2);
  });

  it("does not send a start its caller retired, and a retirement makes an earlier result not current", async () => {
    const host = epochHost();
    const notSent = await barrier.reviewMbfImport({ sourceId: 1, declaration: null }, () => false);
    expect(notSent.outcome).toMatchObject({ outcome: "refused", diagnostics: [{ code: "MBF-IMPORT-CANCELLED" }] });
    expect(host.calls.some((call) => call.command === "review_mbf_import")).toBe(false);
    const sent = await barrier.reviewMbfImport({ sourceId: 1, declaration: null });
    expect(sent.current()).toBe(true);
    void barrier.retireMbfImportSession();
    expect(sent.current()).toBe(false);
    // Serialized: a start issued during a retirement is sent only after it is acknowledged, with its epoch.
    await barrier.adoptMbfImport({ sourceId: 1, declaration: null, reviewSha256: "a".repeat(64) });
    const commands = host.calls.map((call) => call.command);
    expect(commands.slice(-2)).toEqual(["cancel_mbf_import", "adopt_mbf_import"]);
    expect(host.calls.at(-1)?.args?.expectedEpoch).toBe(host.epoch);
  });

  it("never re-sends an older review after a newer start, or after its declaration currency changes", async () => {
    const host = epochHost();
    await barrier.retireMbfImportSession();
    const sent: number[] = [];
    invokeMock.mockImplementation((command: string, args?: Record<string, unknown>) => {
      if (command !== "review_mbf_import") return Promise.resolve({ outcome: "cancelled" });
      sent.push(args?.sourceId as number);
      if (args?.sourceId === 1) return Promise.reject({ code: "RESOURCE_UNAVAILABLE", refusal: { kind: "headroom_insufficient", required_bytes: 2, available_bytes: 1 }, pool: null });
      return Promise.resolve({ outcome: "reviewed", sourceId: 2 });
    });
    vi.useFakeTimers();
    const older = barrier.reviewMbfImport({ sourceId: 1, declaration: null });
    await vi.advanceTimersByTimeAsync(0);
    expect(sent).toEqual([1]);
    const newer = await barrier.reviewMbfImport({ sourceId: 2, declaration: null });
    await vi.advanceTimersByTimeAsync(100);
    expect((await older).outcome).toMatchObject({ outcome: "refused", diagnostics: [{ code: "MBF-IMPORT-CANCELLED" }] });
    expect(sent).toEqual([1, 2]);
    expect(newer.current()).toBe(true);

    let current = true;
    const reverted = barrier.reviewMbfImport({ sourceId: 1, declaration: null }, () => current);
    await vi.advanceTimersByTimeAsync(0);
    expect(sent).toEqual([1, 2, 1]);
    current = false;
    await vi.advanceTimersByTimeAsync(100);
    expect((await reverted).outcome).toMatchObject({ outcome: "refused", diagnostics: [{ code: "MBF-IMPORT-CANCELLED" }] });
    expect(sent).toEqual([1, 2, 1]);
    expect(host.calls.some((call) => call.command === "cancel_mbf_export")).toBe(false);
  });
});
