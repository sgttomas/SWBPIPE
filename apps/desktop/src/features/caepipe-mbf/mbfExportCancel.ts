// File > Export MBF… (DEL-17-04 Design `desktop-mbf-export-import.md` §3.9.5, slice 4b.2a): the export's one
// session barrier. Every native MBF start (`review_mbf_export`, `saveMbfExport`'s `save_mbf_export`) and every
// retirement (`cancel_mbf_export`) goes through this module.
//
// A *retirement* (`retireMbfExportSession`) cancels natively every MBF run that started before it completed. The
// workspace session calls it when it retires a project session (a new session generation, a project replaced by one
// with the same id included) and when it unmounts, whether or not a model check is in progress; slice 4b.2b's dialog
// calls it when it closes. Natively a waiting run stops waiting, a running worker is killed and its grant released,
// a save before its commit writes nothing, a command that entered before the retirement but had not yet registered
// its run is cancelled when it does, and a save past its commit completes and is reported too late.
//
// A *start* (`reviewMbfExport`, `saveMbfExport`) captures the retirement generation as it is called, waits for any
// retirement in progress to be acknowledged, then checks, immediately before every send (`invokeWithIngress`'s
// `stillCurrent`, a headroom re-send included), that no retirement has been requested since and that the caller's own
// session is still current. A start that is no longer current is not sent and resolves as cancelled
// (`MBF-EXPORT-CANCELLED`, as the native cancel replies). So a start issued during a retirement reaches native only
// after the retirement has completed there, and is not cancelled by it; an obsolete start never reaches native. Each
// result carries its generation and `current()`, so a caller discards a completion that arrives after its session
// was retired.
//
// The dispatch ticket: `cancel_mbf_export` replies the native retirement epoch after its bump, and the barrier records
// the latest one acknowledged. A start takes that epoch once, when its wait is over and it is still current, and
// sends it as `expectedEpoch` in its request, the same value in every re-send. Natively a request whose ticket a
// retirement has passed is cancelled whenever its task runs (Tauri extracts an async command's arguments inside the
// task it spawns), so a start sent just before a retirement cannot escape it; a request without a valid current
// ticket is refused `MBF-EXPORT-EPOCH-INVALID`. A stale start never refreshes its ticket: only a new, current start
// takes a newer acknowledgement. Until a retirement has been acknowledged in this page (the page-load reset moves the
// native epoch on too), the first start delivers one as its handshake. The ticket authorizes nothing.
//
// Retirements are serialized. Each delivery of `cancel_mbf_export` is sent only after the one before it has settled
// (acknowledged or not), and the first one in a page only after the page-load reset (`beginSessionResultCustodyReset`,
// which moves the native epoch on too) has settled. Tauri sends each invoke as its own request, so send order is no
// barrier: without the chain, a retirement sent earlier could be applied natively after a later one was acknowledged
// and after a fresh start went out, and cancel that start. A start waits for the latest delivery; by the chain, every
// delivery sent before it (the reset included) has then settled. The acknowledged epoch is the largest any settled
// delivery replied; an unacknowledged delivery adds none.
//
// Fail-safe rule for an unacknowledged retirement: a start never treats a retirement whose `cancel_mbf_export` was
// not acknowledged (the invoke rejected or threw) as done. When the latest delivery is unacknowledged it re-delivers
// it first (one re-delivery, shared by the starts waiting on it); if that is not acknowledged either, the start is not
// sent and refuses `MBF-EXPORT-RETIREMENT-UNCONFIRMED`, and the next start tries the re-delivery again. A later
// acknowledged delivery covers an earlier unacknowledged one (it cancels every run started before it), and an earlier
// failure never holds a start: the chain moves on once a delivery settles. Re-delivering is safe: no start has been
// sent since the retirement was requested, so only runs of retired sessions are cancelled.
import { IngressRequestRetiredError, invokeWithIngress } from "../../services/nativeIngress";
import { invoke } from "@tauri-apps/api/core";
import { beginSessionResultCustodyReset } from "../../services/previewService";
import type { MbfExportArgs, MbfExportSaveOutcome, MbfReviewArgs, MbfReviewOutcome } from "./mbfExport";

/** What the cancel did: the runs it stopped (nothing written), the saves past their commit (not interrupted), and the
 * native retirement epoch after it (the next starts' ticket). `exhausted` (present only when true): the native epoch
 * can move on no further, so every start is refused `MBF-EXPORT-EPOCH-EXHAUSTED`; it is no acknowledgement. */
export type MbfCancelReceipt = { cancelled: number; too_late: number; epoch: number; exhausted?: true };

/** The request member that carries the dispatch ticket. */
export const EXPECTED_EPOCH = "expectedEpoch";

/** A run retired before it wrote anything (the native code, used here too for a start that was never sent). */
export const MBF_EXPORT_CANCELLED = "MBF-EXPORT-CANCELLED";
/** A start refused because the latest retirement was not acknowledged by the native host, even when re-delivered. */
export const MBF_EXPORT_RETIREMENT_UNCONFIRMED = "MBF-EXPORT-RETIREMENT-UNCONFIRMED";

const CANCELLED_MESSAGE = "The export was cancelled. Nothing was written.";
const UNCONFIRMED_MESSAGE =
  "The export did not start: the previous project session's export could not be confirmed stopped. Nothing was written.";

/** Cancels the MBF export's runs in progress (one delivery of `cancel_mbf_export`). Never throws: `null` when the
 * native host did not acknowledge it with a retirement epoch (the browser preview has none, and the page-load reset
 * cancels natively too), or replied that the epoch is exhausted. */
export async function cancelMbfExport(): Promise<MbfCancelReceipt | null> {
  try {
    const answer = await invoke<MbfCancelReceipt>("cancel_mbf_export");
    return answer !== null && typeof answer === "object" && Number.isSafeInteger(answer.epoch) && answer.epoch >= 0 && answer.exhausted !== true
      ? answer
      : null;
  } catch {
    return null;
  }
}

// The barrier's state: the retirement generation (moved on synchronously by each retirement), the latest delivery of
// `cancel_mbf_export` in the chain (none yet in this page: the first start delivers a handshake) with whether it was
// acknowledged, the number of deliveries so far, and the largest native retirement epoch acknowledged.
let generation = 0;
let tail: Promise<boolean> | null = null;
let deliveries = 0;
let acknowledgedEpoch: number | null = null;

/** Queues one delivery of `cancel_mbf_export` after every earlier one (the first after the page-load reset) has
 * settled. Its receipt, `null` when unacknowledged. */
function deliver(): Promise<MbfCancelReceipt | null> {
  const previous: Promise<unknown> = tail ?? beginSessionResultCustodyReset();
  // Neither the reset nor a delivery rejects, so the chain always moves on.
  const receipt = previous.then(() => cancelMbfExport());
  deliveries += 1;
  tail = receipt.then((answer) => {
    // Native epochs only move on: the largest acknowledged is the one every current start must carry.
    if (answer !== null) acknowledgedEpoch = Math.max(acknowledgedEpoch ?? 0, answer.epoch);
    return answer !== null;
  });
  return receipt;
}

/** Retires every MBF run started so far: starts not yet sent are never sent, and `cancel_mbf_export` is delivered
 * once every earlier delivery has settled. The receipt is `null` when the native host did not acknowledge it (see the
 * fail-safe rule above). */
export function retireMbfExportSession(): Promise<MbfCancelReceipt | null> {
  generation += 1;
  return deliver();
}

/** Waits until every delivery sent so far has settled (the latest one, by the chain); re-delivers an unacknowledged
 * latest one (or, before any in this page, delivers the handshake) once. Whether it is acknowledged. */
async function retirementSettled(): Promise<boolean> {
  if (tail === null) void deliver();
  const before = deliveries;
  for (;;) {
    const seen = tail as Promise<boolean>;
    const acknowledged = await seen;
    // A delivery queued meanwhile (a retirement, or another start's re-delivery): wait for it too.
    if (seen !== tail) continue;
    if (acknowledged) return true;
    // Re-delivered once since this start began waiting (by it or by another waiting start): not confirmed.
    if (deliveries > before) return false;
    void deliver();
  }
}

/** A start's result: the outcome, the retirement generation it was started in, and whether it is still current (no
 * retirement since, and the caller's session still current). A caller discards a result that is not current. */
export type MbfStarted<T> = { outcome: T; generation: number; current: () => boolean };

/** Whether the caller's own session is still the one that started (its generation check: same-id replacement,
 * unmount and React Strict Mode's re-run all make it false). */
export type MbfStillCurrent = () => boolean;

async function start<T>(
  command: "review_mbf_export" | "save_mbf_export",
  args: Record<string, unknown>,
  stillCurrent: MbfStillCurrent | undefined,
  refused: (code: string, message: string) => T
): Promise<MbfStarted<T>> {
  const started = generation;
  const current = () => started === generation && (stillCurrent?.() ?? true);
  const result = (outcome: T): MbfStarted<T> => ({ outcome, generation: started, current });
  const confirmed = await retirementSettled();
  if (!current()) return result(refused(MBF_EXPORT_CANCELLED, CANCELLED_MESSAGE));
  const ticket = acknowledgedEpoch;
  if (!confirmed || ticket === null) return result(refused(MBF_EXPORT_RETIREMENT_UNCONFIRMED, UNCONFIRMED_MESSAGE));
  // The ticket is taken once, here, and every send of this start (a headroom re-send included) carries it.
  const request = { ...args, [EXPECTED_EPOCH]: ticket };
  try {
    return result(await invokeWithIngress<T>(command, request, { stillCurrent: current }));
  } catch (error) {
    if (error instanceof IngressRequestRetiredError) return result(refused(MBF_EXPORT_CANCELLED, CANCELLED_MESSAGE));
    throw error;
  }
}

/** The native `review_mbf_export` through the barrier. */
export function reviewMbfExport(args: MbfReviewArgs, stillCurrent?: MbfStillCurrent): Promise<MbfStarted<MbfReviewOutcome>> {
  return start<MbfReviewOutcome>("review_mbf_export", args, stillCurrent, (code, message) => ({
    outcome: "refused",
    diagnostics: [{ code, path: "", message }]
  }));
}

/** The native `save_mbf_export` through the barrier. */
export function saveMbfExport(args: MbfExportArgs, stillCurrent?: MbfStillCurrent): Promise<MbfStarted<MbfExportSaveOutcome>> {
  return start<MbfExportSaveOutcome>("save_mbf_export", args as unknown as Record<string, unknown>, stillCurrent, (code, message) => ({
    outcome: "refused",
    diagnostics: [{ code, path: "", message }]
  }));
}
