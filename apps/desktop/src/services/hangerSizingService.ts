// Hanger sizing in the desktop (DEL-04-11 Design `hanger-sizing.md` §13, increment 3c part 2): the native
// `size_hangers` command (worker task 4 in the capped worker, `hanger_sizing_job.rs`) and its keyed cancel
// `cancel_hanger_sizing`, with the generation guard that keeps a stale completion off the page.
//
// One request, one bounded reply: `{document, evidence}`, the hanger-sizing result document and the Rust reader's
// verdict bound to its checked hash (reproduction `not_checked`). Nothing is recorded, persisted or held natively after
// the reply, and the model is not written.
//
// Run ids. Every request carries `{page, sequence}`: a random token made once per page load and a sequence that rises
// by one per request in this page. Natively one sizing is current: a newer request supersedes the one in progress, an
// older delivery is refused stale, and `cancel_hanger_sizing` cancels a page through a sequence (a repeated or older
// cancel is a no-op). The cancel is sent as Tauri's JSON arguments, the form that survives its `postMessage`
// fallback; the request is a raw body (`invokeWithIngress`).
//
// The generation guard. A completion is current only while its sequence is this page's latest request and no
// retirement has passed it (`isCurrentHangerSizing`). `retireHangerSizing` (a project-session change, an unmount, the
// person's cancel) moves the retirement past the latest request synchronously, so a reply that arrives later is never
// shown, and cancels natively only when a request was started since the last retirement.
import { invoke } from "@tauri-apps/api/core";
import type { HangerSizingDocument, HangerSizingRequest } from "../features/results/hangerSizingResult";
import type { HangerSizingReaderEvidence } from "../features/results/hangerSizingView";
import { IngressRequestRetiredError, invokeWithIngress, nativeFailureText } from "./nativeIngress";
import { ResourceUnavailableError } from "./memoryPool";

export const SIZE_HANGERS_COMMAND = "size_hangers";
export const CANCEL_HANGER_SIZING_COMMAND = "cancel_hanger_sizing";

/** The native codes a refusal can carry (`hanger_sizing_job.rs`), and the page's own. */
export const HANGER_SIZING_CANCELLED = "HANGER-SIZING-CANCELLED";
export const HANGER_SIZING_SUPERSEDED = "HANGER-SIZING-SUPERSEDED";
export const HANGER_SIZING_REQUEST_STALE = "HANGER-SIZING-REQUEST-STALE";
export const HANGER_SIZING_REPLY_TOO_LARGE = "HANGER-SIZING-REPLY-TOO-LARGE";
export const HANGER_SIZING_WORKER_FAILED = "HANGER-SIZING-WORKER-FAILED";
export const HANGER_SIZING_MEMORY_UNAVAILABLE = "HANGER-SIZING-MEMORY-UNAVAILABLE";
export const HANGER_SIZING_REPLY_UNREADABLE = "HANGER-SIZING-REPLY-UNREADABLE";
export const HANGER_SIZING_NATIVE_ONLY = "HANGER-SIZING-NATIVE-ONLY";
export const HANGER_SIZING_RUN_ID_UNAVAILABLE = "HANGER-SIZING-RUN-ID-UNAVAILABLE";

/** The headless input (§8.3.2), exactly what the native entry admits: the desktop solve's request, the sizing request
 * and the full hanger library document. */
export type HangerSizingInput = {
  request: { model: unknown; materials: [] };
  sizing: HangerSizingRequest;
  library: Record<string, unknown>;
};

export type HangerSizingRunId = { page: string; sequence: number };

/** How one request ended. `replied` carries the document (sized or refused) and its evidence; `failed` carries the
 * native or page code and its message, as stated (task 4's own refusal, the reply limit, a worker failure, a
 * cancellation or supersession, a memory refusal). */
export type HangerSizingOutcome =
  | Readonly<{ kind: "replied"; runId: HangerSizingRunId; document: HangerSizingDocument; evidence: HangerSizingReaderEvidence }>
  | Readonly<{ kind: "failed"; runId: HangerSizingRunId | null; code: string; message: string }>;

/** An outcome with its guard: `current()` is false once a newer request or a retirement has passed it. */
export type HangerSizingCompletion = HangerSizingOutcome & Readonly<{ current: () => boolean }>;

/** A typed refusal of the page or the native host. */
export class HangerSizingError extends Error {
  constructor(readonly code: string, message: string) {
    super(`${code}: ${message}`);
    this.name = "HangerSizingError";
  }
}

// This page's run ids, the latest request and the latest retirement.
let pageToken: string | null = null;
let latestSequence = 0;
let retiredThrough = 0;

function isTauri(): boolean {
  return typeof window !== "undefined" && "__TAURI_INTERNALS__" in window;
}

function nextRunId(): HangerSizingRunId | null {
  try {
    pageToken ??= Array.from(globalThis.crypto.getRandomValues(new Uint8Array(16)), (byte) => byte.toString(16).padStart(2, "0")).join("");
  } catch {
    return null;
  }
  if (!Number.isSafeInteger(latestSequence + 1)) return null;
  latestSequence += 1;
  return { page: pageToken, sequence: latestSequence };
}

/** Whether a request's completion may still be shown: it is this page's latest request and no retirement has passed it. */
export function isCurrentHangerSizing(runId: HangerSizingRunId | null): boolean {
  return runId !== null && runId.page === pageToken && runId.sequence === latestSequence && runId.sequence > retiredThrough;
}

/** Reads a native rejection as `{code, message}`: a memory refusal, an invalid request, or `<code>: <message>`. */
export function hangerSizingFailure(error: unknown): { code: string; message: string } {
  if (error instanceof HangerSizingError) return { code: error.code, message: error.message.slice(error.code.length + 2) };
  if (error instanceof ResourceUnavailableError) return { code: HANGER_SIZING_MEMORY_UNAVAILABLE, message: nativeFailureText(error) };
  const text = error instanceof Error ? error.message : String(error);
  const match = /^([A-Z][A-Z0-9_-]*): ([\s\S]*)$/.exec(text);
  return match ? { code: match[1], message: match[2] } : { code: HANGER_SIZING_WORKER_FAILED, message: text };
}

const isRecord = (value: unknown): value is Record<string, unknown> => typeof value === "object" && value !== null && !Array.isArray(value);

/** The reply's shape: a document of a known status and an evidence object. Its standings are bound to the document by
 * the display adapter (`hangerSizingView`), which checks the evidence's hash against the document's own. */
function readReply(reply: unknown): { document: HangerSizingDocument; evidence: HangerSizingReaderEvidence } {
  if (!isRecord(reply) || !isRecord(reply.document) || !isRecord(reply.evidence)) {
    throw new HangerSizingError(HANGER_SIZING_REPLY_UNREADABLE, "the native reply is not a document with its evidence.");
  }
  const status = reply.document.status;
  if (status !== "sized" && status !== "refused") {
    throw new HangerSizingError(HANGER_SIZING_REPLY_UNREADABLE, "the native reply's document has no known status.");
  }
  return { document: reply.document as unknown as HangerSizingDocument, evidence: reply.evidence as unknown as HangerSizingReaderEvidence };
}

/** Sizes the hangers of `input` natively. Resolves, never rejects. A newer request supersedes this one; a
 * retirement makes its completion not current. */
export async function sizeHangers(input: HangerSizingInput): Promise<HangerSizingCompletion> {
  if (!isTauri()) {
    return {
      kind: "failed",
      runId: null,
      code: HANGER_SIZING_NATIVE_ONLY,
      message: "hanger sizing runs in the native desktop application; the browser preview has no sizing worker.",
      current: () => false
    };
  }
  const runId = nextRunId();
  if (runId === null) {
    return { kind: "failed", runId: null, code: HANGER_SIZING_RUN_ID_UNAVAILABLE, message: "this page cannot form a run id; nothing was sent.", current: () => false };
  }
  const current = () => isCurrentHangerSizing(runId);
  try {
    const reply = await invokeWithIngress<unknown>(SIZE_HANGERS_COMMAND, { runId, inputText: JSON.stringify(input) }, { stillCurrent: current });
    return { kind: "replied", runId, ...readReply(reply), current };
  } catch (error) {
    if (error instanceof IngressRequestRetiredError) {
      return { kind: "failed", runId, code: HANGER_SIZING_CANCELLED, message: "the request was retired before it was sent.", current };
    }
    return { kind: "failed", runId, ...hangerSizingFailure(error), current };
  }
}

/** Retires every request of this page made so far: their completions are no longer current, and when one was started
 * since the last retirement the native host cancels this page through it (`cancel_hanger_sizing`). Best effort;
 * never throws. */
export async function retireHangerSizing(): Promise<void> {
  if (latestSequence <= retiredThrough || pageToken === null) return;
  retiredThrough = latestSequence;
  const runId: HangerSizingRunId = { page: pageToken, sequence: latestSequence };
  if (!isTauri()) return;
  try {
    await invoke(CANCEL_HANGER_SIZING_COMMAND, { runId });
  } catch {
    // Best effort: a newer request or the session reset also stops it, and its completion is no longer current.
  }
}

/** Test support: a fresh page. */
export function resetHangerSizingPageForTests(): void {
  pageToken = null;
  latestSequence = 0;
  retiredThrough = 0;
}
