/** Claimed-request transport, explicitly constructed by the workspace owner.
 * No listener, native command or launcher is installed at module startup.
 * Notices are hints only: execute uses the native claim response, never event
 * arguments. The live adapter and its actual queue/job lookups are separate.
 */
export const AGENT_TOOLS = [
  "swbpipe_read_model_summary", "swbpipe_read_nodes", "swbpipe_propose_operations",
  "swbpipe_observe_proposal", "swbpipe_solve", "swbpipe_read_results"
] as const;
export type AgentTool = typeof AGENT_TOOLS[number];
export type AgentBasis = Readonly<{
  workspaceHandle: string;
  applicationGeneration: number;
  modelRevision: number;
  modelHash: unknown;
  stateId: string | null;
  runId: string | null;
}>;
export type AgentCall = Readonly<{
  requestRef: string;
  threadId: string;
  turnId: string;
  callId: string;
  tool: AgentTool;
  basis: AgentBasis;
  arguments: Readonly<Record<string, unknown>>;
}>;
export type AgentScope = Readonly<{ workspaceHandle: string; applicationGeneration: number }>;
export type AgentDelivery = "queued" | "detached" | "lost" | "unconfirmed";
export type AgentActual = Readonly<{
  /** Bounded catalog output generated from actual owner records. */
  outcome: string;
  /** Actual queue/job/read record; never reconstructed from delivery standing.
   * Its authoritative owner retains it independently of this transport. */
  record: unknown;
}>;
export interface AgentBridgePort {
  /** Adapter authenticates the actual native invoking webview/session; one shot.
   * No automatic retries, including after response loss. */
  claim(requestRef: string): Promise<string>;
  /** Native ingress admits raw bytes, validates catalog/binding and resolves
   * evidence from actual owners. This is not a generic RPC destination. */
  complete(requestRef: string, outcome: string): Promise<AgentDelivery>;
}
export interface AgentWorkspaceOwner {
  /** Recheck the actual workspace/generation and call-specific live/historical
   * basis. execute must repeat its own checks across asynchronous preparation. */
  isCurrent(call: AgentCall): boolean;
  /** Reads bounded pages, registers immutable proposals or requests the owned
   * solve route. Never applies or accepts an operation. Retains actual records
   * even when this throws after an effect; interruption never means rollback. */
  execute(call: AgentCall): Promise<AgentActual>;
}
export type AgentNoticeResult =
  | { status: "ignored" | "capacity" | "claim_refused" | "invalid_claim" | "retired" | "execution_unconfirmed" | "invalid_outcome" }
  | { status: "recorded"; delivery: AgentDelivery };
const MAX_CLAIM_BYTES = 64 * 1024;
const MAX_OUTCOME_BYTES = 64 * 1024;
const MAX_ENTRIES = 64;
const MAX_ACTIVE = 16;
type Entry = { actual?: AgentActual };

/** No implicit eviction or redispatch. Taking an outcome transfers only its
 * custody; the tombstone still prevents executing the original request again.
 * Framework/JS heap admission belongs to the eventual live IPC adapter. */
export class EmbeddedAgentBridge {
  private readonly entries = new Map<string, Entry>();
  private active = 0;
  private closed = false;
  private readonly scope: AgentScope;
  constructor(scope: AgentScope, private readonly port: AgentBridgePort, private readonly owner: AgentWorkspaceOwner) {
    this.scope = { ...scope };
  }
  close(): void { this.closed = true; }
  actual(requestRef: string): AgentActual | undefined { return this.entries.get(requestRef)?.actual; }
  takeActual(requestRef: string): AgentActual | undefined {
    const entry = this.entries.get(requestRef);
    const actual = entry?.actual;
    if (entry) delete entry.actual;
    return actual;
  }
  async notify(notice: unknown): Promise<AgentNoticeResult> {
    if (this.closed || !object(notice) || !onlyRequestRef(notice) || !control(notice.requestRef, 160)) return { status: "ignored" };
    const reference = notice.requestRef;
    if (this.entries.has(reference)) return { status: "ignored" };
    if (this.entries.size >= MAX_ENTRIES || this.active >= MAX_ACTIVE) return { status: "capacity" };
    const entry: Entry = {};
    this.entries.set(reference, entry);
    this.active++;
    try {
      let raw: string;
      try { raw = await this.port.claim(reference); }
      catch {
        // No execution occurred here. Native's consumed claim remains consumed
        // on response loss; another notice can never authorize domain replay.
        this.entries.delete(reference);
        return { status: "claim_refused" };
      }
      const call = claim(raw, reference, this.scope);
      if (!call) return { status: "invalid_claim" };
      if (this.closed) return { status: "retired" };
      let actual: AgentActual;
      try {
        if (!this.owner.isCurrent(call)) return { status: "retired" };
        actual = await this.owner.execute(call);
      } catch {
        // An exception says nothing about an already admitted domain effect.
        // The real record owner preserves actual/unknown outcome, never retries.
        return { status: "execution_unconfirmed" };
      }
      // Retain before any serialization/send. Closing during execute does not
      // erase a late actual outcome, and delivery refusal cannot erase it.
      entry.actual = Object.freeze({ outcome: actual.outcome, record: actual.record });
      if (!fits(actual.outcome, MAX_OUTCOME_BYTES)) return { status: "invalid_outcome" };
      let delivery: AgentDelivery;
      try { delivery = await this.port.complete(reference, actual.outcome); }
      catch { delivery = "unconfirmed"; }
      return { status: "recorded", delivery };
    } finally { this.active--; }
  }
}
function onlyRequestRef(notice: Record<string, unknown>): boolean {
  let found = false;
  for (const key in notice) {
    if (!Object.hasOwn(notice, key)) continue;
    if (key !== "requestRef") return false;
    found = true;
  }
  return found;
}
function object(value: unknown): value is Record<string, unknown> {
  return typeof value === "object" && value !== null && !Array.isArray(value);
}
function control(value: unknown, cap: number): value is string {
  return typeof value === "string" && value.length > 0 && value.length <= cap && !/[\u0000-\u001f\u007f]/u.test(value);
}
/** Count UTF-8 without allocating another body before its cap is known. */
function fits(value: unknown, cap: number): value is string {
  if (typeof value !== "string" || value.length > cap) return false;
  let bytes = 0;
  for (let i = 0; i < value.length; i++) {
    const code = value.charCodeAt(i);
    if (code < 0x80) bytes++;
    else if (code < 0x800) bytes += 2;
    else if (code >= 0xd800 && code <= 0xdbff && i + 1 < value.length && value.charCodeAt(i + 1) >= 0xdc00 && value.charCodeAt(i + 1) <= 0xdfff) { bytes += 4; i++; }
    else bytes += 3;
    if (bytes > cap) return false;
  }
  return true;
}
function claim(raw: string, reference: string, scope: AgentScope): AgentCall | null {
  if (!fits(raw, MAX_CLAIM_BYTES)) return null;
  let value: unknown;
  try { value = JSON.parse(raw); } catch { return null; }
  if (!object(value) || value.requestRef !== reference || !control(value.threadId, 256) || !control(value.turnId, 256) || !control(value.callId, 256) ||
    !(AGENT_TOOLS as readonly unknown[]).includes(value.tool) || !object(value.arguments) || !object(value.basis)) return null;
  const basis = value.basis;
  if (basis.workspaceHandle !== scope.workspaceHandle || basis.applicationGeneration !== scope.applicationGeneration ||
    !Number.isSafeInteger(basis.applicationGeneration) || Number(basis.applicationGeneration) < 0 || !Number.isSafeInteger(basis.modelRevision) || Number(basis.modelRevision) < 0 ||
    !(basis.stateId === null || control(basis.stateId, 160)) || !(basis.runId === null || control(basis.runId, 160))) return null;
  // Native already validated the complete catalog input. Do not invent a
  // competing tool schema here; this check guards transport/scope confusion.
  return value as unknown as AgentCall;
}
