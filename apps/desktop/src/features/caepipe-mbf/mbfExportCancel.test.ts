// DEL-17-04 slice 4b.2a: the MBF export's session barrier (`mbfExportCancel.ts`) with a mocked native host. Each
// test loads a fresh module, so the barrier starts as a new page does: no retirement acknowledged yet.
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

function deferred<T>() {
  let resolve!: (value: T) => void;
  const promise = new Promise<T>((yes) => { resolve = yes; });
  return { promise, resolve };
}

/** A native host with a retirement epoch: each acknowledged `cancel_mbf_export` moves it on and replies it. Starts
 * are recorded with the arguments they carried and answered by `reply`. */
function epochHost(reply: (command: string) => Promise<unknown> = () => Promise.resolve({ outcome: "reviewed", review: {} })) {
  const host = { epoch: 0, calls: [] as Array<{ command: string; args?: Record<string, unknown> }> };
  invokeMock.mockImplementation((command: string, args?: Record<string, unknown>) => {
    host.calls.push({ command, args });
    if (command === "cancel_mbf_export") {
      host.epoch += 1;
      return Promise.resolve({ cancelled: 0, too_late: 0, epoch: host.epoch });
    }
    return reply(command);
  });
  const tickets = () => host.calls.filter(({ command }) => command !== "cancel_mbf_export").map(({ args }) => args?.[barrier.EXPECTED_EPOCH]);
  const commands = () => host.calls.map(({ command }) => command);
  return Object.assign(host, { tickets, commands });
}

describe("the MBF export's session barrier", () => {
  it("delivers a handshake before the first start in a page and sends the start with its epoch, through ingress, tagged current", async () => {
    const host = epochHost();
    const started = await barrier.reviewMbfExport(REVIEW_ARGS);
    expect(host.commands()).toEqual(["cancel_mbf_export", "review_mbf_export"]);
    expect(invokeMock.mock.calls[1]).toEqual(["review_mbf_export", { ...REVIEW_ARGS, expectedEpoch: 1 }]);
    expect(started.outcome).toEqual({ outcome: "reviewed", review: {} });
    expect(started.current()).toBe(true);
    // Acknowledged now: the next start is sent without another cancel, with the same ticket.
    await barrier.saveMbfExport(SAVE_ARGS);
    expect(host.commands()).toEqual(["cancel_mbf_export", "review_mbf_export", "save_mbf_export"]);
    expect(host.tickets()).toEqual([1, 1]);
    barrier.retireMbfExportSession();
    expect(started.current()).toBe(false);
  });

  it("concurrent first starts wait for one shared handshake", async () => {
    const host = epochHost();
    await Promise.all([barrier.reviewMbfExport(REVIEW_ARGS), barrier.saveMbfExport(SAVE_ARGS)]);
    expect(host.commands()).toEqual(["cancel_mbf_export", "review_mbf_export", "save_mbf_export"]);
    expect(host.tickets()).toEqual([1, 1]);
  });

  it("a new start after a retirement carries that retirement's epoch, including one issued while the retirement is on its way", async () => {
    const host = epochHost();
    await barrier.reviewMbfExport(REVIEW_ARGS);
    void barrier.retireMbfExportSession();
    // Issued before the retirement's acknowledgement: it waits, then takes the acknowledged epoch, not the older one.
    const during = barrier.saveMbfExport(SAVE_ARGS);
    await during;
    await barrier.retireMbfExportSession();
    await barrier.reviewMbfExport(REVIEW_ARGS);
    expect(host.commands()).toEqual(["cancel_mbf_export", "review_mbf_export", "cancel_mbf_export", "save_mbf_export", "cancel_mbf_export", "review_mbf_export"]);
    expect(host.tickets()).toEqual([1, 2, 3]);
  });

  it("keeps an old start's ticket through its headroom re-sends and never re-sends it after a retirement, while a new start takes the new epoch", async () => {
    let sends = 0;
    const host = epochHost((command) => {
      if (command === "save_mbf_export") return Promise.resolve({ outcome: "saved" });
      sends += 1;
      // The native memory pool's typed refusal for a short headroom queue, which the ingress helper re-sends.
      return Promise.reject({ code: "RESOURCE_UNAVAILABLE", refusal: { kind: "headroom_insufficient", required_bytes: 2, available_bytes: 1 }, pool: null });
    });
    const started = barrier.reviewMbfExport(REVIEW_ARGS);
    await vi.waitFor(() => expect(sends).toBeGreaterThanOrEqual(2));
    const retired = await barrier.retireMbfExportSession();
    expect(retired).toEqual({ cancelled: 0, too_late: 0, epoch: 2 });
    const fresh = await barrier.saveMbfExport(SAVE_ARGS);
    expect(fresh.outcome).toEqual({ outcome: "saved" });
    const outcome = await started;
    expect(codes(outcome.outcome)).toEqual([barrier.MBF_EXPORT_CANCELLED]);
    await flush();
    // Every send of the old start carried its own ticket (1), never the newer acknowledgement (2); only the new start did.
    const reviewTickets = host.calls.filter(({ command }) => command === "review_mbf_export").map(({ args }) => args?.expectedEpoch);
    expect(reviewTickets.length).toBeGreaterThanOrEqual(2);
    expect(new Set(reviewTickets)).toEqual(new Set([1]));
    expect(host.calls.filter(({ command }) => command === "save_mbf_export").map(({ args }) => args?.expectedEpoch)).toEqual([2]);
    expect(host.commands().lastIndexOf("review_mbf_export")).toBeLessThan(host.commands().indexOf("cancel_mbf_export", 1));
  });

  it("an old start waiting on a retirement is never sent once a newer retirement is requested, and the newer start carries its epoch", async () => {
    const answers: Array<ReturnType<typeof deferred<unknown>>> = [];
    const calls: Array<{ command: string; args?: Record<string, unknown> }> = [];
    let epoch = 0;
    invokeMock.mockImplementation((command: string, args?: Record<string, unknown>) => {
      calls.push({ command, args });
      if (command === "cancel_mbf_export") {
        const answer = deferred<unknown>();
        answers.push(answer);
        return answer.promise;
      }
      return Promise.resolve({ outcome: "reviewed", review: {} });
    });
    // Deliveries are serialized: each is sent once the one before it is answered, so answer in rounds.
    const answerAll = async () => {
      for (await flush(); answers.length > 0; await flush()) {
        for (const answer of answers.splice(0)) answer.resolve({ cancelled: 0, too_late: 0, epoch: ++epoch });
      }
    };
    void barrier.retireMbfExportSession();
    const old = barrier.reviewMbfExport(REVIEW_ARGS);
    void barrier.retireMbfExportSession();
    const fresh = barrier.reviewMbfExport(REVIEW_ARGS);
    await answerAll();
    expect(codes((await old).outcome as never)).toEqual([barrier.MBF_EXPORT_CANCELLED]);
    expect((await fresh).outcome).toEqual({ outcome: "reviewed", review: {} });
    expect(calls.filter(({ command }) => command === "review_mbf_export").map(({ args }) => args?.expectedEpoch)).toEqual([2]);
  });

  it("does not treat an unacknowledged retirement as done: it re-delivers it, refuses by name if that fails too, and the next start tries again", async () => {
    const commands: string[] = [];
    let cancelFails = true;
    let epoch = 0;
    invokeMock.mockImplementation((command: string) => {
      commands.push(command);
      if (command === "cancel_mbf_export") return cancelFails ? Promise.reject(new Error("no answer")) : Promise.resolve({ cancelled: 0, too_late: 0, epoch: ++epoch });
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

  it("a cancel that throws synchronously, or replies without an epoch, is unacknowledged too", async () => {
    for (const answer of [() => { throw new Error("no native host"); }, () => Promise.resolve({ cancelled: 0, too_late: 0 })]) {
      vi.resetModules();
      barrier = await import("./mbfExportCancel");
      invokeMock.mockReset();
      invokeMock.mockImplementation((command: string) => (command === "cancel_mbf_export" ? answer() : Promise.resolve({ outcome: "reviewed", review: {} })));
      await expect(barrier.retireMbfExportSession()).resolves.toBeNull();
      const refused = await barrier.reviewMbfExport(REVIEW_ARGS);
      expect(codes(refused.outcome as never)).toEqual([barrier.MBF_EXPORT_RETIREMENT_UNCONFIRMED]);
      expect(invokeMock.mock.calls.map(([command]) => command)).not.toContain("review_mbf_export");
    }
  });

  it("a start retired during a headroom re-send is not sent again and resolves cancelled", async () => {
    let sends = 0;
    epochHost(() => {
      sends += 1;
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

  it("serializes retirements: a later one is sent only once the earlier one has settled, so an earlier one applied late cannot cancel a fresh start", async () => {
    // A native host that applies each cancel when it is answered: the epoch moves on and every run in progress whose
    // ticket is below it is cancelled. The test answers the newest pending cancel first (Tauri's requests are
    // independent, so a later cancel may be applied before an earlier one).
    const pending: Array<ReturnType<typeof deferred<unknown>>> = [];
    const runs: Array<{ ticket: unknown; reply: ReturnType<typeof deferred<unknown>> }> = [];
    const commands: string[] = [];
    let epoch = 0;
    invokeMock.mockImplementation((command: string, args?: Record<string, unknown>) => {
      commands.push(command);
      const answer = deferred<unknown>();
      if (command === "cancel_mbf_export") pending.push(answer);
      else runs.push({ ticket: args?.[barrier.EXPECTED_EPOCH], reply: answer });
      return answer.promise;
    });
    const applyNewestCancel = async () => {
      const answer = pending.pop();
      if (!answer) return false;
      epoch += 1;
      for (const run of runs.splice(0)) {
        if ((run.ticket as number) < epoch) run.reply.resolve({ outcome: "refused", diagnostics: [{ code: barrier.MBF_EXPORT_CANCELLED, path: "", message: "" }] });
        else runs.push(run);
      }
      answer.resolve({ cancelled: 0, too_late: 0, epoch });
      await flush();
      return true;
    };
    // The page's first start, with its handshake, so the barrier has acknowledged an epoch.
    const first = barrier.reviewMbfExport(REVIEW_ARGS);
    await flush();
    await applyNewestCancel();
    runs.splice(0).forEach((run) => run.reply.resolve({ outcome: "reviewed", review: {} }));
    await first;
    commands.length = 0;
    void barrier.retireMbfExportSession(); // A
    void barrier.retireMbfExportSession(); // B
    const fresh = barrier.reviewMbfExport(REVIEW_ARGS);
    await flush();
    // Only A is on its way: B waits for it to settle, and the fresh start for B.
    expect(commands).toEqual(["cancel_mbf_export"]);
    while (await applyNewestCancel()) { /* A, then B once it is sent */ }
    expect(commands).toEqual(["cancel_mbf_export", "cancel_mbf_export", "review_mbf_export"]);
    // No cancel was applied after the fresh start was sent: it carries the latest epoch and is not cancelled.
    expect(runs.map(({ ticket }) => ticket)).toEqual([epoch]);
    runs.splice(0).forEach((run) => run.reply.resolve({ outcome: "reviewed", review: {} }));
    const started = await fresh;
    expect(started.outcome).toEqual({ outcome: "reviewed", review: {} });
    expect(started.current()).toBe(true);
  });

  it("a failed earlier retirement neither holds a start nor counts as acknowledged: the later acknowledged one's epoch is the ticket", async () => {
    const answers: Array<ReturnType<typeof deferred<unknown>>> = [];
    const calls: Array<{ command: string; args?: Record<string, unknown> }> = [];
    invokeMock.mockImplementation((command: string, args?: Record<string, unknown>) => {
      calls.push({ command, args });
      if (command !== "cancel_mbf_export") return Promise.resolve({ outcome: "reviewed", review: {} });
      const answer = deferred<unknown>();
      answers.push(answer);
      return answer.promise;
    });
    const a = barrier.retireMbfExportSession();
    const b = barrier.retireMbfExportSession();
    await flush();
    expect(answers).toHaveLength(1);
    // A fails (no answer the host acknowledges); B is then sent and acknowledged with epoch 5.
    answers.shift()!.resolve(Promise.reject(new Error("no answer")));
    await expect(a).resolves.toBeNull();
    await flush();
    expect(answers).toHaveLength(1);
    answers.shift()!.resolve({ cancelled: 0, too_late: 0, epoch: 5 });
    await expect(b).resolves.toEqual({ cancelled: 0, too_late: 0, epoch: 5 });
    const started = await barrier.reviewMbfExport(REVIEW_ARGS);
    expect(started.outcome).toEqual({ outcome: "reviewed", review: {} });
    // No re-delivery (B covers A), and the start carries B's epoch.
    expect(calls.map(({ command }) => command)).toEqual(["cancel_mbf_export", "cancel_mbf_export", "review_mbf_export"]);
    expect(calls[2].args?.[barrier.EXPECTED_EPOCH]).toBe(5);
  });

  it("sends no retirement, and so no start, before the page-load reset has settled", async () => {
    // The page-load reset (`dispose_all_preview_mechanics_results`) moves the native epoch on too.
    (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__ = {};
    try {
      vi.resetModules();
      barrier = await import("./mbfExportCancel");
      const reset = deferred<unknown>();
      const host = epochHost();
      const answer = invokeMock.getMockImplementation()!;
      invokeMock.mockImplementation((command: string, args?: Record<string, unknown>) => {
        if (command !== "dispose_all_preview_mechanics_results") return answer(command, args);
        host.calls.push({ command });
        return reset.promise.then(() => { host.epoch += 1; return { released_results: 0, released_bytes: 0, abandoned_jobs: 0 }; });
      });
      void barrier.retireMbfExportSession();
      const first = barrier.reviewMbfExport(REVIEW_ARGS);
      await flush();
      expect(host.commands()).toEqual(["dispose_all_preview_mechanics_results"]);
      reset.resolve(undefined);
      const started = await first;
      expect(started.outcome).toEqual({ outcome: "reviewed", review: {} });
      // The reset (epoch 1), then the retirement (epoch 2), then the start with its epoch: no reset after a start.
      expect(host.commands()).toEqual(["dispose_all_preview_mechanics_results", "cancel_mbf_export", "review_mbf_export"]);
      expect(host.calls.filter(({ command }) => command === "review_mbf_export").map(({ args }) => args?.[barrier.EXPECTED_EPOCH])).toEqual([2]);
    } finally {
      delete (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__;
    }
  });
});
