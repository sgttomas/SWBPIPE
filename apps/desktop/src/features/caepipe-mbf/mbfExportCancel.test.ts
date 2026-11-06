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
      return Promise.resolve({ cancelled: 0, too_late: 0, epoch: host.epoch, retirement_id: args?.retirementId });
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
    expect(retired).toEqual({ cancelled: 0, too_late: 0, epoch: 2, retirement_id: expect.objectContaining({ sequence: 2 }) });
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
    const answers: Array<ReturnType<typeof deferred<unknown>> & { id: unknown }> = [];
    const calls: Array<{ command: string; args?: Record<string, unknown> }> = [];
    let epoch = 0;
    invokeMock.mockImplementation((command: string, args?: Record<string, unknown>) => {
      calls.push({ command, args });
      if (command === "cancel_mbf_export") {
        const answer = Object.assign(deferred<unknown>(), { id: args?.[barrier.RETIREMENT_ID] });
        answers.push(answer);
        return answer.promise;
      }
      return Promise.resolve({ outcome: "reviewed", review: {} });
    });
    // Deliveries are serialized: each is sent once the one before it is answered, so answer in rounds.
    const answerAll = async () => {
      for (await flush(); answers.length > 0; await flush()) {
        for (const answer of answers.splice(0)) answer.resolve({ cancelled: 0, too_late: 0, epoch: ++epoch, retirement_id: answer.id });
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
    invokeMock.mockImplementation((command: string, args?: Record<string, unknown>) => {
      commands.push(command);
      if (command === "cancel_mbf_export") {
        return cancelFails ? Promise.reject(new Error("no answer")) : Promise.resolve({ cancelled: 0, too_late: 0, epoch: ++epoch, retirement_id: args?.retirementId });
      }
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
    const pending: Array<ReturnType<typeof deferred<unknown>> & { id: unknown }> = [];
    const runs: Array<{ ticket: unknown; reply: ReturnType<typeof deferred<unknown>> }> = [];
    const commands: string[] = [];
    let epoch = 0;
    invokeMock.mockImplementation((command: string, args?: Record<string, unknown>) => {
      commands.push(command);
      const answer = Object.assign(deferred<unknown>(), { id: args?.[barrier.RETIREMENT_ID] });
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
      answer.resolve({ cancelled: 0, too_late: 0, epoch, retirement_id: answer.id });
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
    const answers: Array<ReturnType<typeof deferred<unknown>> & { id: unknown }> = [];
    const calls: Array<{ command: string; args?: Record<string, unknown> }> = [];
    invokeMock.mockImplementation((command: string, args?: Record<string, unknown>) => {
      calls.push({ command, args });
      if (command !== "cancel_mbf_export") return Promise.resolve({ outcome: "reviewed", review: {} });
      const answer = Object.assign(deferred<unknown>(), { id: args?.[barrier.RETIREMENT_ID] });
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
    const second = answers.shift()!;
    second.resolve({ cancelled: 0, too_late: 0, epoch: 5, retirement_id: second.id });
    await expect(b).resolves.toEqual({ cancelled: 0, too_late: 0, epoch: 5, retirement_id: second.id });
    const started = await barrier.reviewMbfExport(REVIEW_ARGS);
    expect(started.outcome).toEqual({ outcome: "reviewed", review: {} });
    // No re-delivery (B covers A), and the start carries B's epoch.
    expect(calls.map(({ command }) => command)).toEqual(["cancel_mbf_export", "cancel_mbf_export", "review_mbf_export"]);
    expect(calls[2].args?.[barrier.EXPECTED_EPOCH]).toBe(5);
  });

  it("neither waits for nor sends the performance session reset: native MBF retirement is only the keyed cancel", async () => {
    (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__ = {};
    try {
      vi.resetModules();
      barrier = await import("./mbfExportCancel");
      const host = epochHost();
      const answer = invokeMock.getMockImplementation()!;
      // A reset that never settles holds nothing of the export's.
      invokeMock.mockImplementation((command: string, args?: Record<string, unknown>) =>
        command === "dispose_all_preview_mechanics_results" ? new Promise(() => {}) : answer(command, args));
      await barrier.retireMbfExportSession();
      const started = await barrier.reviewMbfExport(REVIEW_ARGS);
      expect(started.outcome).toEqual({ outcome: "reviewed", review: {} });
      expect(host.commands()).toEqual(["cancel_mbf_export", "review_mbf_export"]);
    } finally {
      delete (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__;
    }
  });
});

// Repair 5: idempotent keyed retirement. Each logical retirement carries `retirementId` = {page, sequence}; the
// native host applies an id only when its sequence is above the last applied from its page (`mbf_worker.rs`
// `MbfRuns::retire_keyed`). `idempotentHost` is a mock of that rule.
type Id = { page: string; sequence: number };

/** A native host that applies `cancel_mbf_export` by the keyed rule (or, with `ignoreSequence`, applies every
 * delivery: the mutation control). Each applied retirement moves the epoch on and cancels every run in progress; runs
 * are answered by the test. `deliver` lets a test hand the host a delivery the transport delayed. */
function idempotentHost(options: { ignoreSequence?: boolean } = {}) {
  const host = {
    epoch: 0,
    last: new Map<string, number>(),
    cancels: [] as Id[],
    runs: [] as Array<{ ticket: unknown; cancelled: boolean; reply: ReturnType<typeof deferred<unknown>> }>,
    apply(id: Id) {
      const last = host.last.get(id.page);
      if (!options.ignoreSequence && last !== undefined && id.sequence <= last) {
        return { cancelled: 0, too_late: 0, epoch: host.epoch, retirement_id: id, duplicate: true };
      }
      host.last.set(id.page, Math.max(last ?? 0, id.sequence));
      host.epoch += 1;
      let cancelled = 0;
      for (const run of host.runs.filter((run) => !run.cancelled)) {
        run.cancelled = true;
        cancelled += 1;
        run.reply.resolve({ outcome: "refused", diagnostics: [{ code: barrier.MBF_EXPORT_CANCELLED, path: "", message: "" }] });
      }
      return { cancelled, too_late: 0, epoch: host.epoch, retirement_id: id };
    }
  };
  invokeMock.mockImplementation((command: string, args?: Record<string, unknown>) => {
    if (command === "cancel_mbf_export") {
      const id = args?.[barrier.RETIREMENT_ID] as Id;
      host.cancels.push(id);
      return Promise.resolve(host.apply(id));
    }
    const reply = deferred<unknown>();
    host.runs.push({ ticket: args?.[barrier.EXPECTED_EPOCH], cancelled: false, reply });
    return reply.promise;
  });
  return host;
}

describe("the MBF export's keyed retirement (repair 5)", () => {
  it("gives each logical retirement its own id: one page token of 32 hex digits, the sequence rising by one", async () => {
    const host = idempotentHost();
    await barrier.retireMbfExportSession();
    await barrier.retireMbfExportSession();
    expect(host.cancels).toHaveLength(2);
    expect(host.cancels[0].page).toMatch(/^[0-9a-f]{32}$/);
    expect(host.cancels[1].page).toBe(host.cancels[0].page);
    expect(host.cancels.map(({ sequence }) => sequence)).toEqual([1, 2]);
    // A new page (a fresh module) makes a new token.
    vi.resetModules();
    const next = await import("./mbfExportCancel");
    await next.retireMbfExportSession();
    expect(host.cancels[2].page).not.toBe(host.cancels[0].page);
    expect(host.cancels[2].sequence).toBe(1);
  });

  // An injected-fault schedule at the helper level: after a failed fetch, Tauri 2.11.1's transport resends the cancel
  // through `postMessage` (its JSON arguments survive that), without cancelling the original delivery. The test hands
  // the host the same id twice, as such a resend would, to pin the native rule's effect on the barrier.
  it("injected fault: a delayed duplicate of an acknowledged retirement does not cancel a fresh start", async () => {
    for (const ignoreSequence of [false, true]) {
      vi.resetModules();
      barrier = await import("./mbfExportCancel");
      invokeMock.mockReset();
      const host = idempotentHost({ ignoreSequence });
      // The resend (the transport's fallback) of r1 is applied and acknowledged.
      const acknowledged = await barrier.retireMbfExportSession();
      expect(acknowledged).toMatchObject({ epoch: 1 });
      const r1 = host.cancels[0];
      // A fresh start enters at epoch 1.
      const fresh = barrier.reviewMbfExport(REVIEW_ARGS);
      await vi.waitFor(() => expect(host.runs).toHaveLength(1));
      expect(host.runs[0].ticket).toBe(1);
      // The delayed original r1 arrives.
      const delayed = host.apply({ ...r1 });
      if (ignoreSequence) {
        // Mutation control: a host that ignores the sequence cancels the fresh start.
        expect(host.runs[0].cancelled).toBe(true);
        expect(codes((await fresh).outcome)).toEqual([barrier.MBF_EXPORT_CANCELLED]);
        continue;
      }
      expect(delayed).toEqual({ cancelled: 0, too_late: 0, epoch: 1, retirement_id: r1, duplicate: true });
      expect(host.epoch).toBe(1);
      expect(host.runs[0].cancelled).toBe(false);
      host.runs[0].reply.resolve({ outcome: "reviewed", review: {} });
      const started = await fresh;
      expect(started.outcome).toEqual({ outcome: "reviewed", review: {} });
      expect(started.current()).toBe(true);
    }
  });

  it("re-delivers an unacknowledged retirement with the same id, and takes the host's duplicate receipt for it as acknowledgement", async () => {
    const host = idempotentHost();
    const answer = invokeMock.getMockImplementation()!;
    let lost = 1;
    invokeMock.mockImplementation((command: string, args?: Record<string, unknown>) => {
      if (command === "cancel_mbf_export" && lost > 0) {
        // Applied natively, but the reply is lost.
        lost -= 1;
        host.cancels.push(args?.[barrier.RETIREMENT_ID] as Id);
        host.apply(args?.[barrier.RETIREMENT_ID] as Id);
        return Promise.reject(new Error("the reply was lost"));
      }
      return answer(command, args);
    });
    await expect(barrier.retireMbfExportSession()).resolves.toBeNull();
    const started = barrier.reviewMbfExport(REVIEW_ARGS);
    await vi.waitFor(() => expect(host.runs).toHaveLength(1));
    // The re-delivery reused the id; the host found it applied (a no-op), and its receipt acknowledged the epoch.
    expect(host.cancels).toHaveLength(2);
    expect(host.cancels[1]).toEqual(host.cancels[0]);
    expect(host.epoch).toBe(1);
    expect(host.runs[0]).toMatchObject({ ticket: 1, cancelled: false });
    host.runs[0].reply.resolve({ outcome: "reviewed", review: {} });
    expect((await started).outcome).toEqual({ outcome: "reviewed", review: {} });
  });

  it("is no acknowledgement when the receipt answers another id, or the page cannot make one", async () => {
    invokeMock.mockImplementation((command: string, args?: Record<string, unknown>) => {
      const id = args?.[barrier.RETIREMENT_ID] as Id;
      return Promise.resolve({ cancelled: 0, too_late: 0, epoch: 3, retirement_id: { ...id, sequence: id.sequence + 1 }, duplicate: true });
    });
    await expect(barrier.retireMbfExportSession()).resolves.toBeNull();
    const refused = await barrier.reviewMbfExport(REVIEW_ARGS);
    expect(codes(refused.outcome as never)).toEqual([barrier.MBF_EXPORT_RETIREMENT_UNCONFIRMED]);
    vi.resetModules();
    barrier = await import("./mbfExportCancel");
    invokeMock.mockReset();
    const random = vi.spyOn(globalThis.crypto, "getRandomValues").mockImplementation(() => { throw new Error("no random source"); });
    try {
      await expect(barrier.retireMbfExportSession()).resolves.toBeNull();
      expect(invokeMock).not.toHaveBeenCalled();
    } finally {
      random.mockRestore();
    }
  });

});

// Finding P2 (re-check at d4517a4e5): once Tauri's transport has fallen back to `postMessage`, a retirement must still
// reach the native host in a form it admits. `tauriTransport` reproduces what Tauri 2.11.1 hands the host for one
// invoke (`scripts/ipc-protocol.js` with `scripts/process-ipc-message-fn.js`, decoded by `src/ipc/protocol.rs`), and
// `nativeCancelId` the forms `cancel_mbf_export` admits (`mbf_worker.rs` `cancel_command`).
const INGRESS = { header: "x-swbpipe-ingress", format: "args-json-v1" };
type Received = { kind: "raw"; bytes: Uint8Array; headers: Record<string, string> } | { kind: "json"; value: unknown; headers: Record<string, string> };

/** Tauri's IPC payload replacer (`process-ipc-message-fn.js`): a `Uint8Array` or `ArrayBuffer` becomes an array of
 * numbers. */
function ipcReplacer(_key: string, value: unknown) {
  if (value instanceof Uint8Array) return Array.from(value);
  if (value instanceof ArrayBuffer) return Array.from(new Uint8Array(value));
  return value;
}

/** What the host receives for `invoke(command, payload, options)`: on the custom protocol a byte payload is posted as
 * bytes and anything else as JSON text, with `options.headers` as request headers; after the fallback the whole
 * message, payload included, goes through `JSON.stringify` with the replacer, and the host parses it. */
function tauriTransport(command: string, payload: unknown, options: { headers?: Record<string, string> } | undefined, fellBack: boolean): Received {
  const headers = options?.headers ?? {};
  if (!fellBack) {
    if (ArrayBuffer.isView(payload) || payload instanceof ArrayBuffer || Array.isArray(payload)) {
      const view = payload as ArrayBufferView;
      return { kind: "raw", bytes: new Uint8Array(view.buffer, view.byteOffset, view.byteLength), headers };
    }
    return { kind: "json", value: JSON.parse(JSON.stringify(payload, ipcReplacer)), headers };
  }
  const message = JSON.stringify(
    { cmd: command, callback: 1, error: 2, payload, options: { ...options, customProtocolIpcBlocked: true }, __TAURI_INVOKE_KEY__: "key" },
    ipcReplacer
  );
  const parsed = JSON.parse(message) as { payload: unknown; options?: { headers?: Record<string, string> } };
  return { kind: "json", value: parsed.payload, headers: parsed.options?.headers ?? {} };
}

/** The id `cancel_mbf_export` reads from what it received, or the refusal it replies (the native rules: a raw body
 * needs the ingress header; a JSON body is the JSON form only without it, one object of at most 4 KiB). */
function nativeCancelId(received: Received): Id {
  let args: unknown;
  if (received.kind === "raw") {
    if (received.headers[INGRESS.header] !== INGRESS.format) throw new Error("INGRESS-HEADER-REQUIRED");
    args = JSON.parse(new TextDecoder().decode(received.bytes));
  } else {
    if (received.headers[INGRESS.header] !== undefined) throw new Error("INGRESS-RAW-BODY-REQUIRED");
    if (JSON.stringify(received.value).length > 4096) throw new Error("INGRESS-ARGS-TOO-LARGE");
    args = received.value;
  }
  if (!args || typeof args !== "object" || Array.isArray(args)) throw new Error("INGRESS-ARGS-NOT-OBJECT");
  const id = (args as Record<string, unknown>)[barrier.RETIREMENT_ID] as Partial<Id> | undefined;
  if (!id || typeof id.page !== "string" || !/^[0-9a-f]{32}$/.test(id.page) || !Number.isSafeInteger(id.sequence) || (id.sequence as number) < 1) {
    throw new Error("MBF-EXPORT-RETIREMENT-ID-INVALID");
  }
  return { page: id.page, sequence: id.sequence as number };
}

describe("the MBF export's keyed retirement after the transport falls back (finding P2)", () => {
  it("still reaches the native host in the JSON form, stops an admitted save, and re-delivers with the same id", async () => {
    let fellBack = false;
    let fetchFails = false;
    let lostReplies = 0;
    const deliveries: Array<{ form: Received["kind"]; fellBack: boolean; id: Id; duplicate: boolean }> = [];
    const refusals: string[] = [];
    const host = idempotentHost();
    const answer = invokeMock.getMockImplementation()!;
    invokeMock.mockImplementation((command: string, payload?: unknown, options?: { headers?: Record<string, string> }) => {
      // A start is a raw body: after the fallback it arrives as an array of numbers, which the host refuses.
      if (command !== "cancel_mbf_export") {
        return fellBack ? Promise.reject({ code: "INVALID_REQUEST", message: "INGRESS-RAW-BODY-REQUIRED" }) : answer(command, payload);
      }
      // The custom protocol's `fetch` fails once: Tauri marks the page fallen back and resends this same invoke
      // through `postMessage`, which the page does not see.
      if (fetchFails && !fellBack) fellBack = true;
      const received = tauriTransport(command, payload, options, fellBack);
      let id: Id;
      try {
        id = nativeCancelId(received);
      } catch (error) {
        refusals.push((error as Error).message);
        return Promise.reject(error);
      }
      host.cancels.push(id);
      const receipt = host.apply(id);
      deliveries.push({ form: received.kind, fellBack, id, duplicate: receipt.duplicate === true });
      if (lostReplies > 0) {
        lostReplies -= 1;
        return Promise.reject(new Error("the reply was lost"));
      }
      return Promise.resolve(receipt);
    });
    // A save is admitted (its handshake retirement acknowledged, the save sent with its ticket) and waits for its
    // chooser natively.
    const saving = barrier.saveMbfExport(SAVE_ARGS);
    await vi.waitFor(() => expect(host.runs).toHaveLength(1));
    expect(host.runs[0].ticket).toBe(1);
    // The transport falls back; the session is then retired, and the reply to that delivery is lost.
    fetchFails = true;
    lostReplies = 1;
    await expect(barrier.retireMbfExportSession()).resolves.toBeNull();
    expect(refusals).toEqual([]);
    expect(host.runs[0].cancelled).toBe(true);
    expect(codes((await saving).outcome as never)).toEqual([barrier.MBF_EXPORT_CANCELLED]);
    // The next start re-delivers the unacknowledged retirement with the same id; the host finds it applied (a no-op),
    // which acknowledges it. The start itself is then sent raw and refused natively (starts stay raw-body only): it
    // fails closed, and nothing runs.
    await expect(barrier.reviewMbfExport(REVIEW_ARGS)).rejects.toThrow("INGRESS-RAW-BODY-REQUIRED");
    expect(deliveries.map(({ form, fellBack, duplicate }) => ({ form, fellBack, duplicate }))).toEqual([
      { form: "json", fellBack: false, duplicate: false },
      { form: "json", fellBack: true, duplicate: false },
      { form: "json", fellBack: true, duplicate: true }
    ]);
    expect(deliveries[2].id).toEqual(deliveries[1].id);
    expect(deliveries[1].id).toEqual({ page: deliveries[0].id.page, sequence: 2 });
    expect(host.epoch).toBe(2);
    expect(host.runs).toHaveLength(1);
  });

  it("is sent as Tauri's JSON arguments with no ingress header, never as a raw body", async () => {
    const id: Id = { page: "0123456789abcdef0123456789abcdef", sequence: 1 };
    invokeMock.mockResolvedValueOnce({ cancelled: 0, too_late: 0, epoch: 1, retirement_id: id });
    await barrier.cancelMbfExport(id);
    const [command, payload, options] = invokeMock.mock.calls[0] as [string, unknown, { headers?: Record<string, string> } | undefined];
    expect(command).toBe("cancel_mbf_export");
    expect(payload).toEqual({ [barrier.RETIREMENT_ID]: id });
    expect(options?.headers?.[INGRESS.header]).toBeUndefined();
    // The same id reaches the host on both transports.
    for (const fellBack of [false, true]) expect(nativeCancelId(tauriTransport(command, payload, options, fellBack))).toEqual(id);
    // Control: the raw body the starts use would not survive the fallback.
    const raw = new TextEncoder().encode(JSON.stringify({ [barrier.RETIREMENT_ID]: id }));
    const headers = { headers: { [INGRESS.header]: INGRESS.format } };
    expect(nativeCancelId(tauriTransport(command, raw, headers, false))).toEqual(id);
    expect(() => nativeCancelId(tauriTransport(command, raw, headers, true))).toThrow("INGRESS-RAW-BODY-REQUIRED");
  });
});
