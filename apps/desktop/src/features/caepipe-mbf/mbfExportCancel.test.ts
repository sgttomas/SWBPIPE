// DEL-17-04 slice 4b.2a: the MBF export's session barrier (`mbfExportCancel.ts`) with a mocked native host. Each
// test loads a fresh module, so the barrier's state starts acknowledged.
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";

const invokeMock = vi.hoisted(() => vi.fn());
vi.mock("@tauri-apps/api/core", async () => ({ invoke: (await import("../../test/nativeIngress")).ingressDecodingInvoke(invokeMock) }));

type Barrier = typeof import("./mbfExportCancel");
let barrier: Barrier;

beforeEach(async () => {
  vi.resetModules();
  barrier = await import("./mbfExportCancel");
});
afterEach(() => invokeMock.mockReset());

const REVIEW_ARGS = { canonicalText: "{}", claimedModelHash: "sha256:0", settingsJson: "{}" };
const SAVE_ARGS = { ...REVIEW_ARGS, suggestedName: "swbpipe-model.mbf", reviewed: { mbfSha256: "0", sidecarSha256: "0" } } as never;
const flush = () => new Promise((resolve) => setTimeout(resolve, 0));

function codes(outcome: { outcome: string; diagnostics?: Array<{ code: string }> }) {
  return outcome.outcome === "refused" ? outcome.diagnostics?.map((item) => item.code) : outcome.outcome;
}

describe("the MBF export's session barrier", () => {
  it("sends a start with nothing retired, through ingress, tagged current", async () => {
    invokeMock.mockResolvedValue({ outcome: "reviewed", review: {} });
    const started = await barrier.reviewMbfExport(REVIEW_ARGS);
    expect(invokeMock.mock.calls).toEqual([["review_mbf_export", REVIEW_ARGS]]);
    expect(started.outcome).toEqual({ outcome: "reviewed", review: {} });
    expect(started.current()).toBe(true);
    barrier.retireMbfExportSession();
    expect(started.current()).toBe(false);
  });

  it("does not treat an unacknowledged retirement as done: it re-delivers it, refuses by name if that fails too, and the next start tries again", async () => {
    const commands: string[] = [];
    let cancelFails = true;
    invokeMock.mockImplementation((command: string) => {
      commands.push(command);
      if (command === "cancel_mbf_export") return cancelFails ? Promise.reject(new Error("no answer")) : Promise.resolve({ cancelled: 0, too_late: 0 });
      return Promise.resolve({ outcome: "saved" });
    });
    await expect(barrier.retireMbfExportSession()).resolves.toBeNull();
    for (const start of [barrier.reviewMbfExport(REVIEW_ARGS), barrier.saveMbfExport(SAVE_ARGS)]) {
      const refused = await start;
      expect(codes(refused.outcome as never)).toEqual([barrier.MBF_EXPORT_RETIREMENT_UNCONFIRMED]);
    }
    // The first retirement, then one shared re-delivery per waiting round; never a start.
    expect(commands.every((command) => command === "cancel_mbf_export")).toBe(true);
    expect(commands.length).toBeGreaterThanOrEqual(2);
    cancelFails = false;
    const saved = await barrier.saveMbfExport(SAVE_ARGS);
    expect(saved.outcome).toEqual({ outcome: "saved" });
    expect(commands.slice(-2)).toEqual(["cancel_mbf_export", "save_mbf_export"]);
    // Acknowledged now: the next start is sent without another cancel.
    await barrier.reviewMbfExport(REVIEW_ARGS);
    expect(commands.slice(-1)).toEqual(["review_mbf_export"]);
  });

  it("a cancel that throws synchronously is unacknowledged too", async () => {
    invokeMock.mockImplementation((command: string) => {
      if (command === "cancel_mbf_export") throw new Error("no native host");
      return Promise.resolve({ outcome: "reviewed", review: {} });
    });
    await barrier.retireMbfExportSession();
    const refused = await barrier.reviewMbfExport(REVIEW_ARGS);
    expect(codes(refused.outcome as never)).toEqual([barrier.MBF_EXPORT_RETIREMENT_UNCONFIRMED]);
    expect(invokeMock.mock.calls.map(([command]) => command)).not.toContain("review_mbf_export");
  });

  it("a start retired during a headroom re-send is not sent again and resolves cancelled", async () => {
    let sends = 0;
    invokeMock.mockImplementation((command: string) => {
      if (command === "cancel_mbf_export") return Promise.resolve({ cancelled: 0, too_late: 0 });
      sends += 1;
      // The native memory pool's typed refusal for a short headroom queue, which the ingress helper re-sends.
      return Promise.reject({ code: "RESOURCE_UNAVAILABLE", refusal: { kind: "headroom_insufficient", required_bytes: 2, available_bytes: 1 }, pool: null });
    });
    const started = barrier.reviewMbfExport(REVIEW_ARGS);
    await vi.waitFor(() => expect(sends).toBe(1));
    barrier.retireMbfExportSession();
    const outcome = await started;
    expect(codes(outcome.outcome)).toEqual([barrier.MBF_EXPORT_CANCELLED]);
    await flush();
    expect(sends).toBe(1);
  });
});
