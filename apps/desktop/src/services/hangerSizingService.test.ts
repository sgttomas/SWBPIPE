// DEL-04-11 increment 3c part 2: the native hanger sizing's service (`hangerSizingService.ts`) with a mocked native
// host (`invoke` is mocked: these tests show what the page sends and when, not what Tauri's transport delivers): the
// retirement each start awaits and the ticket it carries, the raw-body request, the reply, the keyed retirement, the
// generation guard and the typed failures. The reply is
// the committed `fixtures/results/hanger_sizing_reply_two_span.json`, which the Rust command test pins byte for byte
// as the capped worker's own reply to `hanger_sizing_input_two_span.json`.
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import twoSpanInput from "../../../../fixtures/results/hanger_sizing_input_two_span.json";
import twoSpanReply from "../../../../fixtures/results/hanger_sizing_reply_two_span.json";
import twoSpanDocument from "../../../../fixtures/results/hanger_sizing_result_two_span.json";
import { hangerSizingView } from "../features/results/hangerSizingView";
import type { HangerSizingInput } from "./hangerSizingService";

const invokeMock = vi.hoisted(() => vi.fn());
vi.mock("@tauri-apps/api/core", async () => ({ invoke: (await import("../test/nativeIngress")).ingressDecodingInvoke(invokeMock) }));

type Service = typeof import("./hangerSizingService");
let service: Service;

beforeEach(async () => {
  vi.resetModules();
  service = await import("./hangerSizingService");
  (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__ = {};
});
afterEach(() => {
  invokeMock.mockReset();
  delete (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__;
});

const input = () => structuredClone(twoSpanInput) as unknown as HangerSizingInput;

type RetirementArgs = { retirementId: { page: string; sequence: number } };
type StartArgs = { expectedEpoch: number; inputText: string };

/** A native host whose retirements acknowledge their own id with epochs 1, 2, …, and whose `size_hangers` answers
 * `size` (the committed reply by default). */
function host(size: (args: StartArgs) => Promise<unknown> = () => Promise.resolve(structuredClone(twoSpanReply))) {
  let epoch = 0;
  invokeMock.mockImplementation((command: string, args: unknown) => {
    if (command === "cancel_hanger_sizing") {
      epoch += 1;
      return Promise.resolve({ cancelled: false, epoch, retirement_id: (args as RetirementArgs).retirementId });
    }
    if (command === "size_hangers") return size(args as StartArgs);
    return Promise.reject(new Error(`unexpected ${command}`));
  });
}

const calls = (command: string) => invokeMock.mock.calls.filter(([name]) => name === command);
const flush = () => new Promise((resolve) => setTimeout(resolve, 0));

function deferred<T>() {
  let resolve!: (value: T) => void;
  let reject!: (error: unknown) => void;
  const promise = new Promise<T>((yes, no) => { resolve = yes; reject = no; });
  return { promise, resolve, reject };
}

describe("retireHangerSizingOnMount", () => {
  it("retires on mount even when this page never sized, moving the epoch past every ticket issued before it", async () => {
    let epoch = 0;
    const firstStartSent = deferred<void>();
    const release = deferred<void>();
    invokeMock.mockImplementation((command: string, args: unknown) => {
      if (command === "cancel_hanger_sizing") {
        epoch += 1;
        return Promise.resolve({ cancelled: false, epoch, retirement_id: (args as RetirementArgs).retirementId });
      }
      if (command === "size_hangers") {
        // A host that refuses a ticket below its current epoch, as `register` does.
        const ticket = (args as StartArgs).expectedEpoch;
        firstStartSent.resolve();
        return release.promise.then(() => (ticket < epoch
          ? Promise.reject({ code: "HANGER-SIZING-REQUEST-STALE", message: "a retirement has passed this ticket" })
          : structuredClone(twoSpanReply)));
      }
      return Promise.reject(new Error(`unexpected ${command}`));
    });
    // A mount on a page that has never sized sends exactly one keyed retirement and nothing else.
    await service.retireHangerSizingOnMount();
    expect(invokeMock.mock.calls.map(([command]) => command)).toEqual(["cancel_hanger_sizing"]);
    expect(Object.keys(calls("cancel_hanger_sizing")[0][1] as object)).toEqual(["retirementId"]);
    // A start takes ticket 2 and is held at the host; a new mount retires (epoch 3) before it registers.
    const completion = service.sizeHangers(input());
    await firstStartSent.promise;
    expect((calls("size_hangers")[0][1] as StartArgs).expectedEpoch).toBe(2);
    await service.retireHangerSizingOnMount();
    expect(epoch).toBe(3);
    release.resolve();
    const done = await completion;
    expect(done.kind).toBe("failed");
    expect(done.current()).toBe(false);
  });
});

describe("sizeHangers", () => {
  it("retires first, then sends the headless input's JSON text as a raw body with the acknowledged epoch as its ticket, and reads the reply", async () => {
    host();
    const completion = await service.sizeHangers(input());
    expect(invokeMock.mock.calls.map(([command]) => command)).toEqual(["cancel_hanger_sizing", "size_hangers"]);
    const [, retirement] = calls("cancel_hanger_sizing")[0] as [string, RetirementArgs];
    expect(Object.keys(retirement)).toEqual(["retirementId"]);
    expect(retirement.retirementId.page).toMatch(/^[0-9a-f]{32}$/);
    expect(retirement.retirementId.sequence).toBe(1);
    const [, args] = calls("size_hangers")[0] as [string, StartArgs];
    expect(Object.keys(args).sort()).toEqual(["expectedEpoch", "inputText"]);
    expect(args.expectedEpoch).toBe(1);
    expect(JSON.parse(args.inputText)).toEqual(twoSpanInput);
    expect(completion.kind).toBe("replied");
    if (completion.kind !== "replied") return;
    expect(completion.document).toEqual(twoSpanDocument);
    expect(completion.current()).toBe(true);
    // The evidence the worker formed binds to the document it replied: every standing it checked is shown.
    const view = await hangerSizingView(completion.document, completion.evidence, input().library);
    expect(view.readerStandings.binding).toEqual({ state: "bound", documentSha256: twoSpanReply.evidence.document_sha256 });
    expect(view.readerStandings.standings.map((standing) => [standing.name, standing.status])).toEqual([
      ["closure", "passed"],
      ["binding", "passed"],
      ["arithmetic_consistent_with_published_positions", "passed"],
      ["derivation", "passed"],
      ["reproduction", "not_checked"]
    ]);
  });

  it("awaits its retirement before sending, and carries the epoch that retirement acknowledged", async () => {
    const acknowledged = deferred<unknown>();
    invokeMock.mockImplementation((command: string) => (command === "cancel_hanger_sizing" ? acknowledged.promise : Promise.resolve(structuredClone(twoSpanReply))));
    const running = service.sizeHangers(input());
    await flush();
    expect(calls("size_hangers")).toHaveLength(0);
    const [, retirement] = calls("cancel_hanger_sizing")[0] as [string, RetirementArgs];
    acknowledged.resolve({ cancelled: true, epoch: 41, retirement_id: retirement.retirementId, duplicate: true });
    expect((await running).kind).toBe("replied");
    expect((calls("size_hangers")[0][1] as StartArgs).expectedEpoch).toBe(41);
  });

  it("reads each native refusal as its code and message, and a memory refusal as a typed memory failure", async () => {
    const failures: Array<[unknown, string]> = [
      ["HANGER-SIZING-REPLY-TOO-LARGE: the sizing reply of 67108865 bytes is above the desktop's 67108864-byte limit for one reply", service.HANGER_SIZING_REPLY_TOO_LARGE],
      ["HANGER_SIZING_INPUT_TOO_LARGE: the hanger-sizing input has 1 bytes", "HANGER_SIZING_INPUT_TOO_LARGE"],
      ["HANGER-SIZING-WORKER-FAILED: SOLVE_WORKER_EXITED: signal 9", service.HANGER_SIZING_WORKER_FAILED],
      ["HANGER-SIZING-REQUEST-STALE: a retirement has passed this request's ticket", service.HANGER_SIZING_REQUEST_STALE],
      ["HANGER-SIZING-TICKET-INVALID: the request carries no ticket. Nothing was started.", service.HANGER_SIZING_TICKET_INVALID],
      [{ code: "RESOURCE_UNAVAILABLE", refusal: { kind: "worker_limit_reached", running: 4, limit: 4 }, pool: null }, service.HANGER_SIZING_MEMORY_UNAVAILABLE],
      ["no code here", service.HANGER_SIZING_WORKER_FAILED]
    ];
    for (const [rejection, code] of failures) {
      host(() => Promise.reject(rejection));
      const completion = await service.sizeHangers(input());
      expect(completion.kind, String(rejection)).toBe("failed");
      if (completion.kind === "failed") expect(completion.code).toBe(code);
    }
    for (const reply of [null, { document: {}, evidence: {} }, { document: twoSpanDocument }]) {
      host(() => Promise.resolve(reply));
      const completion = await service.sizeHangers(input());
      expect(completion.kind === "failed" && completion.code).toBe(service.HANGER_SIZING_REPLY_UNREADABLE);
    }
  });

  it("refuses in the browser preview without sending anything", async () => {
    delete (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__;
    const completion = await service.sizeHangers(input());
    expect(completion.kind === "failed" && completion.code).toBe(service.HANGER_SIZING_NATIVE_ONLY);
    expect(completion.current()).toBe(false);
    expect(invokeMock).not.toHaveBeenCalled();
  });
});

describe("an unconfirmed retirement authorizes nothing", () => {
  it("sends no start when its retirement fails, answers another id, carries no usable epoch, or reports exhaustion, and says so", async () => {
    const answers: Array<[(args: RetirementArgs) => Promise<unknown>, string]> = [
      [() => Promise.reject(new Error("transport")), service.HANGER_SIZING_RETIREMENT_UNCONFIRMED],
      [() => Promise.reject("HANGER-SIZING-RETIREMENT-ID-INVALID: the cancel carries a malformed retirement id. Nothing was retired."), service.HANGER_SIZING_RETIREMENT_UNCONFIRMED],
      [() => Promise.resolve({ cancelled: false, duplicate: false }), service.HANGER_SIZING_RETIREMENT_UNCONFIRMED],
      [(args) => Promise.resolve({ cancelled: false, epoch: 3, retirement_id: { ...args.retirementId, sequence: args.retirementId.sequence + 1 } }), service.HANGER_SIZING_RETIREMENT_UNCONFIRMED],
      [(args) => Promise.resolve({ cancelled: false, epoch: -1, retirement_id: args.retirementId }), service.HANGER_SIZING_RETIREMENT_UNCONFIRMED],
      [(args) => Promise.resolve({ cancelled: false, epoch: 2 ** 53, retirement_id: args.retirementId }), service.HANGER_SIZING_RETIREMENT_UNCONFIRMED],
      [(args) => Promise.resolve({ cancelled: false, epoch: "3", retirement_id: args.retirementId }), service.HANGER_SIZING_RETIREMENT_UNCONFIRMED],
      [(args) => Promise.resolve({ cancelled: true, epoch: 9, retirement_id: args.retirementId, exhausted: true }), service.HANGER_SIZING_EXHAUSTED]
    ];
    for (const [answer, code] of answers) {
      invokeMock.mockReset();
      invokeMock.mockImplementation((command: string, args: unknown) => (command === "cancel_hanger_sizing" ? answer(args as RetirementArgs) : Promise.resolve(structuredClone(twoSpanReply))));
      const completion = await service.sizeHangers(input());
      expect(calls("size_hangers"), code).toHaveLength(0);
      expect(completion.kind === "failed" && completion.code).toBe(code);
      expect(completion.current(), "the refusal is the current request's, so it is shown").toBe(true);
    }
  });
});

describe("the generation guard and the keyed retirement", () => {
  it("sends nothing when the caller's session ends, a newer request is made, or a retirement is made while it awaits its retirement", async () => {
    const changes: Array<[string, (live: { value: boolean }) => void]> = [
      ["session", (live) => { live.value = false; }],
      ["newer request", () => { void service.sizeHangers(input()); }],
      ["retirement", () => { void service.retireHangerSizing(); }]
    ];
    for (const [change, apply] of changes) {
      invokeMock.mockReset();
      const acknowledged = deferred<unknown>();
      let first = true;
      invokeMock.mockImplementation((command: string, args: unknown) => {
        if (command !== "cancel_hanger_sizing") return new Promise(() => {});
        if (!first) return new Promise(() => {});
        first = false;
        return acknowledged.promise.then(() => ({ cancelled: false, epoch: 5, retirement_id: (args as RetirementArgs).retirementId }));
      });
      const live = { value: true };
      const running = service.sizeHangers(input(), () => live.value);
      await flush();
      apply(live);
      acknowledged.resolve(null);
      const completion = await running;
      await flush();
      expect(calls("size_hangers"), change).toHaveLength(0);
      expect(completion.kind === "failed" && completion.code, change).toBe(service.HANGER_SIZING_CANCELLED);
      expect(completion.current(), change).toBe(false);
    }
  });

  it("never lets an older completion stay current once a newer request is made, and each start carries its own ticket", async () => {
    const first = deferred<unknown>();
    const second = deferred<unknown>();
    const replies = [first, second];
    host(() => replies.shift()!.promise);
    const older = service.sizeHangers(input());
    await flush();
    const newer = service.sizeHangers(input());
    await flush();
    first.resolve(structuredClone(twoSpanReply));
    second.resolve(structuredClone(twoSpanReply));
    const [olderDone, newerDone] = await Promise.all([older, newer]);
    expect(olderDone.current()).toBe(false);
    expect(newerDone.current()).toBe(true);
    expect(calls("size_hangers").map(([, args]) => (args as StartArgs).expectedEpoch)).toEqual([1, 2]);
    expect(calls("cancel_hanger_sizing").map(([, args]) => (args as RetirementArgs).retirementId.sequence)).toEqual([1, 2]);
  });

  it("retires a request in progress with a new retirement id of this page, and its late reply is not current", async () => {
    const pending = deferred<unknown>();
    host(() => pending.promise);
    const running = service.sizeHangers(input());
    await flush();
    expect(calls("size_hangers")).toHaveLength(1);
    const [, own] = calls("cancel_hanger_sizing")[0] as [string, RetirementArgs];
    await service.retireHangerSizing();
    const retirements = calls("cancel_hanger_sizing").map(([, args]) => (args as RetirementArgs).retirementId);
    expect(retirements).toEqual([own.retirementId, { page: own.retirementId.page, sequence: own.retirementId.sequence + 1 }]);
    pending.resolve(structuredClone(twoSpanReply));
    expect((await running).current()).toBe(false);
    // Nothing started since: a second retirement sends nothing.
    await service.retireHangerSizing();
    expect(calls("cancel_hanger_sizing")).toHaveLength(2);
  });

  it("sends no retirement when this page never started a sizing, and a failed retirement never throws", async () => {
    await service.retireHangerSizing();
    expect(invokeMock).not.toHaveBeenCalled();
    host();
    await service.sizeHangers(input());
    invokeMock.mockImplementation(() => Promise.reject(new Error("transport")));
    await expect(service.retireHangerSizing()).resolves.toBeUndefined();
  });
});
