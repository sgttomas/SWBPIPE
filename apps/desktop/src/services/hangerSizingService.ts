// Hanger sizing in the desktop (increment 3c part 2): the native
// `size_hangers` command (worker task 4 in the capped worker, `hanger_sizing_job.rs`), its keyed retirement
// `cancel_hanger_sizing`, and the generation guard that keeps an obsolete start from being sent and a stale completion
// off the page.
//
// One request, one bounded reply: `{document, evidence}`, the hanger-sizing result document and the Rust reader's
// verdict bound to its checked hash (reproduction `not_checked`). Nothing is recorded, persisted or held natively after
// the reply, and the model is not written.
//
// Retirement and the ticket (MBF's design, `features/caepipe-mbf/mbfExportCancel.ts`, for the sizing's own native
// slot). A *retirement* (`cancel_hanger_sizing`) moves the native retirement epoch on, stops the sizing in progress and
// refuses every start that has not yet registered natively; it replies the epoch after its bump. Each logical
// retirement carries an id, `retirementId` = {page, sequence}: a random token made once per page load and a sequence
// that rises by one per logical retirement in this page; the host applies an id once, and a repeated or older delivery
// replies `duplicate` with the current epoch. It is sent as Tauri's JSON arguments, the form that survives Tauri's
// `postMessage` fallback; the start is a raw body (`invokeWithIngress`), which that fallback cannot carry (the host
// refuses it, `INGRESS-RAW-BODY-REQUIRED`).
//
// A *start* (`sizeHangers`) moves this page's request generation on, retires natively (so a newer request retires the
// one in progress), awaits that retirement's acknowledgement, and only then sends `size_hangers` with the acknowledged
// epoch as its ticket (`expectedEpoch`), the same in every re-send. After the await and immediately before every send
// it re-checks that its generation is still the latest and that the caller's session is still current; if not, it
// sends nothing and resolves as cancelled. A retirement that was not acknowledged (the invoke failed, or replied for
// another id or without a usable epoch) authorizes nothing: the start is not sent and resolves refused
// (`HANGER-SIZING-RETIREMENT-UNCONFIRMED`, or `HANGER-SIZING-EXHAUSTED` when the host replies so). Natively a start
// whose ticket a later retirement has passed is refused (`HANGER-SIZING-REQUEST-STALE`) whenever its task runs, and
// never replaces or cancels the run in progress.
//
// `retireHangerSizing` (the person's cancel, the form's project session ending) moves the generation on
// synchronously, so no start of the retired session is sent after it and no reply that arrives later is current, and
// retires natively when a start has been made since the last such retirement. The ticket authorizes nothing.
import { invoke } from "@tauri-apps/api/core";
import type { HangerSizingDocument, HangerSizingRequest } from "../features/results/hangerSizingResult";
import type { HangerSizingReaderEvidence } from "../features/results/hangerSizingView";
import { IngressRequestRetiredError, invokeWithIngress, nativeFailureText } from "./nativeIngress";
import { ResourceUnavailableError } from "./memoryPool";

export const SIZE_HANGERS_COMMAND = "size_hangers";
export const CANCEL_HANGER_SIZING_COMMAND = "cancel_hanger_sizing";

/** The request member that carries a start's ticket, and the cancel's member that carries a retirement's id. */
export const EXPECTED_EPOCH = "expectedEpoch";
export const RETIREMENT_ID = "retirementId";

/** The native codes a refusal can carry (`hanger_sizing_job.rs`), and the page's own. */
export const HANGER_SIZING_CANCELLED = "HANGER-SIZING-CANCELLED";
export const HANGER_SIZING_REQUEST_STALE = "HANGER-SIZING-REQUEST-STALE";
export const HANGER_SIZING_TICKET_INVALID = "HANGER-SIZING-TICKET-INVALID";
export const HANGER_SIZING_EXHAUSTED = "HANGER-SIZING-EXHAUSTED";
export const HANGER_SIZING_REPLY_TOO_LARGE = "HANGER-SIZING-REPLY-TOO-LARGE";
export const HANGER_SIZING_WORKER_FAILED = "HANGER-SIZING-WORKER-FAILED";
export const HANGER_SIZING_MEMORY_UNAVAILABLE = "HANGER-SIZING-MEMORY-UNAVAILABLE";
export const HANGER_SIZING_REPLY_UNREADABLE = "HANGER-SIZING-REPLY-UNREADABLE";
export const HANGER_SIZING_NATIVE_ONLY = "HANGER-SIZING-NATIVE-ONLY";
export const HANGER_SIZING_RETIREMENT_UNCONFIRMED = "HANGER-SIZING-RETIREMENT-UNCONFIRMED";

/** The headless input, exactly what the native entry admits: the desktop solve's request, the sizing request
 * and the full hanger library document. */
export type HangerSizingInput = {
  request: { model: unknown; materials: [] };
  sizing: HangerSizingRequest;
  library: Record<string, unknown>;
};

/** One logical retirement's id: this page's token and the retirement's sequence in this page. */
export type HangerSizingRetirementId = { page: string; sequence: number };

/** `cancel_hanger_sizing`'s reply: whether it stopped the sizing in progress, the native retirement epoch after it (the
 * next start's ticket) and the id it answers. `duplicate` (only when true): that id had been applied already, and
 * `epoch` is the current one. `exhausted` (only when true): the sizing can be retired no further; it is no
 * acknowledgement. */
export type HangerSizingRetirementReceipt = {
  cancelled: boolean;
  epoch: number;
  retirement_id: HangerSizingRetirementId;
  duplicate?: true;
  exhausted?: true;
};

/** How one request ended. `replied` carries the document (sized or refused) and its evidence; `failed` carries the
 * native or page code and its message, as stated (task 4's own refusal, the reply limit, a worker failure, a
 * retirement, an unconfirmed retirement, a memory refusal). */
export type HangerSizingOutcome =
  | Readonly<{ kind: "replied"; document: HangerSizingDocument; evidence: HangerSizingReaderEvidence }>
  | Readonly<{ kind: "failed"; code: string; message: string }>;

/** An outcome with its guard: `current()` is false once a newer request or a retirement has passed it, or the
 * caller's session is no longer current. */
export type HangerSizingCompletion = HangerSizingOutcome & Readonly<{ current: () => boolean }>;

/** Whether the caller's own session is still the one that started (the form's mount and project). */
export type HangerSizingStillCurrent = () => boolean;

/** A typed refusal of the page or the native host. */
export class HangerSizingError extends Error {
  constructor(readonly code: string, message: string) {
    super(`${code}: ${message}`);
    this.name = "HangerSizingError";
  }
}

// This page's retirement ids (the token is made once, at the first retirement), the request generation (moved on by
// every start and every retirement), and whether a start has been made since the last `retireHangerSizing`.
let pageToken: string | null = null;
let retirementSequence = 0;
let generation = 0;
let startedSinceRetirement = false;

function isTauri(): boolean {
  return typeof window !== "undefined" && "__TAURI_INTERNALS__" in window;
}

/** The next logical retirement's id, or `null` when the page cannot make one (no random source, or the sequence is
 * exhausted): its retirement is then unconfirmed, which fails closed. */
function nextRetirementId(): HangerSizingRetirementId | null {
  try {
    pageToken ??= Array.from(globalThis.crypto.getRandomValues(new Uint8Array(16)), (byte) => byte.toString(16).padStart(2, "0")).join("");
  } catch {
    return null;
  }
  if (!Number.isSafeInteger(retirementSequence + 1)) return null;
  retirementSequence += 1;
  return { page: pageToken, sequence: retirementSequence };
}

type Retired = Readonly<{ ok: true; epoch: number }> | Readonly<{ ok: false; code: string; message: string }>;

const UNCONFIRMED_MESSAGE = "the native host did not confirm that the previous sizing was retired, so this request was not sent.";

/** One logical retirement: `cancel_hanger_sizing` for a new id. Never throws. Acknowledged only by a reply that
 * echoes the id with a safe, non-negative integer epoch and is not exhausted. */
async function deliverRetirement(): Promise<Retired> {
  const id = nextRetirementId();
  if (id === null) return { ok: false, code: HANGER_SIZING_RETIREMENT_UNCONFIRMED, message: UNCONFIRMED_MESSAGE };
  let answer: unknown;
  try {
    answer = await invoke<HangerSizingRetirementReceipt>(CANCEL_HANGER_SIZING_COMMAND, { [RETIREMENT_ID]: id });
  } catch {
    return { ok: false, code: HANGER_SIZING_RETIREMENT_UNCONFIRMED, message: UNCONFIRMED_MESSAGE };
  }
  const answers = isRecord(answer) && isRecord(answer.retirement_id) ? answer.retirement_id : null;
  if (!isRecord(answer) || answers?.page !== id.page || answers?.sequence !== id.sequence) {
    return { ok: false, code: HANGER_SIZING_RETIREMENT_UNCONFIRMED, message: UNCONFIRMED_MESSAGE };
  }
  if (answer.exhausted === true) {
    return { ok: false, code: HANGER_SIZING_EXHAUSTED, message: "hanger sizing cannot be retired any further in this session; restart the application to size again. Nothing was sent." };
  }
  const epoch = answer.epoch;
  if (typeof epoch !== "number" || !Number.isSafeInteger(epoch) || epoch < 0) {
    return { ok: false, code: HANGER_SIZING_RETIREMENT_UNCONFIRMED, message: UNCONFIRMED_MESSAGE };
  }
  return { ok: true, epoch };
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

/** Sizes the hangers of `input` natively. Resolves, never rejects. Retires the sizing in progress first and sends
 * only once that retirement is acknowledged, with its epoch as the ticket; a newer request, a retirement or the
 * caller's session ending makes it not current, before or after it is sent. */
export async function sizeHangers(input: HangerSizingInput, stillCurrent?: HangerSizingStillCurrent): Promise<HangerSizingCompletion> {
  if (!isTauri()) {
    return {
      kind: "failed",
      code: HANGER_SIZING_NATIVE_ONLY,
      message: "hanger sizing runs in the native desktop application; the browser preview has no sizing worker.",
      current: () => false
    };
  }
  generation += 1;
  const started = generation;
  startedSinceRetirement = true;
  const current = () => started === generation && (stillCurrent?.() ?? true);
  const notSent = (): HangerSizingCompletion => ({ kind: "failed", code: HANGER_SIZING_CANCELLED, message: "the request was retired before it was sent; nothing was sent.", current });
  const retired = await deliverRetirement();
  if (!current()) return notSent();
  if (!retired.ok) return { kind: "failed", code: retired.code, message: retired.message, current };
  // The ticket is taken once, here, and every send of this start (a headroom re-send included) carries it.
  const request = { [EXPECTED_EPOCH]: retired.epoch, inputText: JSON.stringify(input) };
  try {
    const reply = await invokeWithIngress<unknown>(SIZE_HANGERS_COMMAND, request, { stillCurrent: current });
    return { kind: "replied", ...readReply(reply), current };
  } catch (error) {
    if (error instanceof IngressRequestRetiredError) return notSent();
    return { kind: "failed", ...hangerSizingFailure(error), current };
  }
}

/** Retires every request of this page made so far: none not yet sent is sent, their completions are no longer
 * current, and when a start has been made since the last retirement the native host retires the sizing
 * (`cancel_hanger_sizing`). Best effort; never throws. */
export async function retireHangerSizing(): Promise<void> {
  generation += 1;
  if (!startedSinceRetirement || !isTauri()) return;
  startedSinceRetirement = false;
  await deliverRetirement();
}

/** The retirement a newly mounted form sends, whether or not this page has sized before: it moves the sizing
 * epoch on, so a ticket issued before the mount (a start of an earlier session or page, still pending) can no longer
 * register. Nothing is sized. A failed retirement authorizes nothing: every start retires again before it is sent. */
export async function retireHangerSizingOnMount(): Promise<void> {
  generation += 1;
  if (!isTauri()) return;
  startedSinceRetirement = false;
  await deliverRetirement();
}

/** Test support: a fresh page. */
export function resetHangerSizingPageForTests(): void {
  pageToken = null;
  retirementSequence = 0;
  generation = 0;
  startedSinceRetirement = false;
}
