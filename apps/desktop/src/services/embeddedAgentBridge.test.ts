import { describe, expect, it, vi } from "vitest";
import catalog from "../../src-tauri/resources/agent-capability-catalog.json";
import { AGENT_TOOLS, EmbeddedAgentBridge, type AgentActual, type AgentBridgePort, type AgentWorkspaceOwner } from "./embeddedAgentBridge";
const scope = { workspaceHandle: "workspace-invented", applicationGeneration: 1 };
const basis = { ...scope, modelRevision: 2, modelHash: null, stateId: null, runId: null };
const payload = (requestRef = "session-invented:1") => ({ requestRef, threadId: "thread-1", turnId: "turn-1", callId: "call-1", tool: "swbpipe_read_model_summary", basis, arguments: {} });
const actual: AgentActual = { outcome: JSON.stringify({ status: "unknown", basis, message: "Invented", data: null }), record: { standing: "unknown" } };
function deferred<T>() { let resolve!: (v: T) => void; const promise = new Promise<T>((r) => { resolve = r; }); return { promise, resolve }; }
function setup() {
  const port = { claim: vi.fn(async (ref: string) => JSON.stringify(payload(ref))), complete: vi.fn<AgentBridgePort["complete"]>(async () => "queued") };
  const owner = { isCurrent: vi.fn(() => true), execute: vi.fn<AgentWorkspaceOwner["execute"]>(async () => actual) };
  return { bridge: new EmbeddedAgentBridge(scope, port, owner), port, owner };
}
describe("embedded agent claimed-request bridge", () => {
  it("uses the catalog tool names without a parallel operation language", () => {
    expect(AGENT_TOOLS).toEqual(catalog.tools.map((tool) => tool.name));
  });
  it("ignores event input and executes only an original native claim", async () => {
    const { bridge, port, owner } = setup();
    expect(await bridge.notify({ ...payload(), arbitraryRpc: 123 })).toEqual({ status: "ignored" });
    expect(port.claim).not.toHaveBeenCalled();
    expect(await bridge.notify({ requestRef: "session-invented:1" })).toEqual({ status: "recorded", delivery: "queued" });
    expect(owner.execute).toHaveBeenCalledWith(payload());
    expect(port.complete).toHaveBeenCalledWith("session-invented:1", actual.outcome);
  });
  it("does no domain work on a forged or failed native claim", async () => {
    const { bridge, port, owner } = setup();
    port.claim.mockRejectedValueOnce(new Error("secret supplier text"));
    expect(await bridge.notify({ requestRef: "forged" })).toEqual({ status: "claim_refused" });
    expect(owner.execute).not.toHaveBeenCalled();
    expect(port.complete).not.toHaveBeenCalled();
  });
  it.each([
    { requestRef: "other" }, { basis: { ...basis, workspaceHandle: "other" } },
    { basis: { ...basis, applicationGeneration: 2 } }, { tool: "apply" }, { arguments: [] }
  ])("rejects mixed claims before domain work: %j", async (change) => {
    const { bridge, port, owner } = setup();
    port.claim.mockResolvedValueOnce(JSON.stringify({ ...payload(), ...change }));
    expect(await bridge.notify({ requestRef: "session-invented:1" })).toEqual({ status: "invalid_claim" });
    expect(owner.execute).not.toHaveBeenCalled();
  });
  it("deduplicates notices while a claim or execution is unresolved and after taking actual custody", async () => {
    const { bridge, port, owner } = setup();
    const gate = deferred<string>(); port.claim.mockReturnValueOnce(gate.promise);
    const first = bridge.notify({ requestRef: "session-invented:1" });
    expect(await bridge.notify({ requestRef: "session-invented:1" })).toEqual({ status: "ignored" });
    gate.resolve(JSON.stringify(payload())); await first;
    expect(bridge.takeActual("session-invented:1")?.record).toBe(actual.record);
    expect(bridge.actual("session-invented:1")).toBeUndefined();
    expect(await bridge.notify({ requestRef: "session-invented:1" })).toEqual({ status: "ignored" });
    expect(owner.execute).toHaveBeenCalledOnce();
  });
  it("rechecks the actual workspace after claim and starts no work after close", async () => {
    const { bridge, port, owner } = setup();
    owner.isCurrent.mockReturnValueOnce(false);
    expect(await bridge.notify({ requestRef: "session-invented:1" })).toEqual({ status: "retired" });
    const gate = deferred<string>(); port.claim.mockReturnValueOnce(gate.promise);
    const second = bridge.notify({ requestRef: "session-invented:2" }); bridge.close();
    gate.resolve(JSON.stringify(payload("session-invented:2")));
    expect(await second).toEqual({ status: "retired" });
    expect(owner.execute).not.toHaveBeenCalled();
  });
  it("retains a late actual before a failed send, including after close, without replay", async () => {
    const { bridge, port, owner } = setup();
    const gate = deferred<AgentActual>(); owner.execute.mockReturnValueOnce(gate.promise);
    port.complete.mockImplementationOnce(async () => {
      expect(bridge.actual("session-invented:1")?.record).toBe(actual.record);
      throw new Error("no queue");
    });
    const first = bridge.notify({ requestRef: "session-invented:1" });
    await Promise.resolve(); bridge.close(); gate.resolve(actual);
    expect(await first).toEqual({ status: "recorded", delivery: "unconfirmed" });
    expect(bridge.actual("session-invented:1")?.outcome).toBe(actual.outcome);
    expect(await bridge.notify({ requestRef: "session-invented:1" })).toEqual({ status: "ignored" });
    expect(owner.execute).toHaveBeenCalledOnce(); expect(port.complete).toHaveBeenCalledOnce();
  });
  it.each(["detached", "lost", "unconfirmed"] as const)("keeps actual outcome separate from %s delivery", async (delivery) => {
    const { bridge, port } = setup(); port.complete.mockResolvedValueOnce(delivery);
    expect(await bridge.notify({ requestRef: "session-invented:1" })).toEqual({ status: "recorded", delivery });
    expect(bridge.actual("session-invented:1")?.record).toBe(actual.record);
  });
  it("never fabricates rejection or retries domain work after an execution exception", async () => {
    const { bridge, port, owner } = setup(); owner.execute.mockRejectedValueOnce(new Error("unknown after admission"));
    expect(await bridge.notify({ requestRef: "session-invented:1" })).toEqual({ status: "execution_unconfirmed" });
    expect(await bridge.notify({ requestRef: "session-invented:1" })).toEqual({ status: "ignored" });
    expect(port.complete).not.toHaveBeenCalled(); expect(owner.execute).toHaveBeenCalledOnce();
  });
  it("bounds UTF-8 before parsing claims and before sending outcomes, while retaining actual records", async () => {
    const { bridge, port, owner } = setup();
    port.claim.mockResolvedValueOnce('"' + "λ".repeat(40000) + '"');
    expect(await bridge.notify({ requestRef: "session-invented:1" })).toEqual({ status: "invalid_claim" });
    owner.execute.mockResolvedValueOnce({ ...actual, outcome: "λ".repeat(40000) });
    expect(await bridge.notify({ requestRef: "session-invented:2" })).toEqual({ status: "invalid_outcome" });
    expect(bridge.actual("session-invented:2")?.record).toBe(actual.record);
    expect(port.complete).not.toHaveBeenCalled();
  });
  it("bounds unresolved work and retains 64 tombstones without implicit eviction", async () => {
    const { bridge, owner } = setup();
    const gate = deferred<AgentActual>(); owner.execute.mockReturnValue(gate.promise);
    const active = Array.from({ length: 16 }, (_, i) => bridge.notify({ requestRef: `session-invented:${i}` }));
    expect(await bridge.notify({ requestRef: "session-invented:16" })).toEqual({ status: "capacity" });
    gate.resolve(actual); await Promise.all(active);
    for (let i = 16; i < 64; i++) await bridge.notify({ requestRef: `session-invented:${i}` });
    bridge.takeActual("session-invented:0");
    expect(await bridge.notify({ requestRef: "session-invented:64" })).toEqual({ status: "capacity" });
    expect(owner.execute).toHaveBeenCalledTimes(64);
  });
});
