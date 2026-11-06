// DEL-04-11 increment 3c part 2: the native hanger sizing's service (`hangerSizingService.ts`) with a mocked native
// host: the raw-body request, the reply, the keyed cancel, the generation guard and the typed failures. The reply is
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

function deferred<T>() {
  let resolve!: (value: T) => void;
  let reject!: (error: unknown) => void;
  const promise = new Promise<T>((yes, no) => { resolve = yes; reject = no; });
  return { promise, resolve, reject };
}

describe("sizeHangers", () => {
  it("sends the headless input's JSON text with its run id as a raw body, and reads the reply's document and evidence", async () => {
    invokeMock.mockResolvedValue(structuredClone(twoSpanReply));
    const completion = await service.sizeHangers(input());
    expect(invokeMock).toHaveBeenCalledTimes(1);
    const [command, args] = invokeMock.mock.calls[0] as [string, { runId: { page: string; sequence: number }; inputText: string }];
    expect(command).toBe("size_hangers");
    expect(Object.keys(args).sort()).toEqual(["inputText", "runId"]);
    expect(args.runId.page).toMatch(/^[0-9a-f]{32}$/);
    expect(args.runId.sequence).toBe(1);
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

  it("reads each native refusal as its code and message, and a memory refusal as a typed memory failure", async () => {
    const failures: Array<[unknown, string]> = [
      ["HANGER-SIZING-REPLY-TOO-LARGE: the sizing reply of 67108865 bytes is above the desktop's 67108864-byte limit for one reply", service.HANGER_SIZING_REPLY_TOO_LARGE],
      ["HANGER_SIZING_INPUT_TOO_LARGE: the hanger-sizing input has 1 bytes", "HANGER_SIZING_INPUT_TOO_LARGE"],
      ["HANGER-SIZING-WORKER-FAILED: SOLVE_WORKER_EXITED: signal 9", service.HANGER_SIZING_WORKER_FAILED],
      ["HANGER-SIZING-SUPERSEDED: a newer sizing request replaced this one", service.HANGER_SIZING_SUPERSEDED],
      [{ code: "RESOURCE_UNAVAILABLE", refusal: { kind: "worker_limit_reached", running: 4, limit: 4 }, pool: null }, service.HANGER_SIZING_MEMORY_UNAVAILABLE],
      ["no code here", service.HANGER_SIZING_WORKER_FAILED]
    ];
    for (const [rejection, code] of failures) {
      invokeMock.mockRejectedValueOnce(rejection);
      const completion = await service.sizeHangers(input());
      expect(completion.kind, String(rejection)).toBe("failed");
      if (completion.kind === "failed") expect(completion.code).toBe(code);
    }
    for (const reply of [null, { document: {} , evidence: {} }, { document: twoSpanDocument }]) {
      invokeMock.mockResolvedValueOnce(reply);
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

describe("the generation guard and the keyed cancel", () => {
  it("never lets an older completion stay current once a newer request is made", async () => {
    const first = deferred<unknown>();
    const second = deferred<unknown>();
    invokeMock.mockReturnValueOnce(first.promise).mockReturnValueOnce(second.promise);
    const older = service.sizeHangers(input());
    const newer = service.sizeHangers(input());
    first.resolve(structuredClone(twoSpanReply));
    second.resolve(structuredClone(twoSpanReply));
    const [olderDone, newerDone] = await Promise.all([older, newer]);
    expect(olderDone.current()).toBe(false);
    expect(newerDone.current()).toBe(true);
    const sequences = invokeMock.mock.calls.map(([, args]) => (args as { runId: { sequence: number } }).runId.sequence);
    expect(sequences).toEqual([1, 2]);
  });

  it("retires a request in progress: the cancel names its page through its sequence, and its late reply is not current", async () => {
    const pending = deferred<unknown>();
    invokeMock.mockImplementation((command: string) => (command === "size_hangers" ? pending.promise : Promise.resolve({ cancelled: true, duplicate: false })));
    const running = service.sizeHangers(input());
    const [, sent] = invokeMock.mock.calls[0] as [string, { runId: { page: string; sequence: number } }];
    await service.retireHangerSizing();
    const cancels = invokeMock.mock.calls.filter(([command]) => command === "cancel_hanger_sizing");
    expect(cancels).toEqual([["cancel_hanger_sizing", { runId: sent.runId }]]);
    pending.resolve(structuredClone(twoSpanReply));
    expect((await running).current()).toBe(false);
    // Nothing started since: a second retirement sends nothing.
    await service.retireHangerSizing();
    expect(invokeMock.mock.calls.filter(([command]) => command === "cancel_hanger_sizing")).toHaveLength(1);
  });

  it("sends no cancel when this page never started a sizing, and a failed cancel never throws", async () => {
    await service.retireHangerSizing();
    expect(invokeMock).not.toHaveBeenCalled();
    invokeMock.mockImplementation((command: string) => (command === "size_hangers" ? Promise.resolve(structuredClone(twoSpanReply)) : Promise.reject(new Error("transport"))));
    await service.sizeHangers(input());
    await expect(service.retireHangerSizing()).resolves.toBeUndefined();
  });
});
