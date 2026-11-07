// File > Import MBF… (DEL-17-04 Design `desktop-mbf-export-import.md` §4.8.6, slice 5): the import's one session
// barrier. Every native import start (`open_mbf_import`, `review_mbf_import`, `adopt_mbf_import`) and every retirement
// (`cancel_mbf_import`) goes through this module. It has the export barrier's rules (`mbfExportCancel.ts`), on its own
// state, page token and commands, so an import retirement never retires an export and the reverse:
//
// - retirements are serialized: each delivery is sent after the one before it has settled;
// - each logical retirement has its own id `{page, sequence}` (a random page token made once per page load, and a
//   rising sequence), and a re-delivery reuses it, so a duplicate delivery is a no-op natively;
// - a start captures the retirement generation, waits for the latest delivery (delivering a handshake first if none
//   has been delivered in this page), re-delivers an unacknowledged latest retirement once with the same id, and is
//   refused `MBF-IMPORT-RETIREMENT-UNCONFIRMED` if that is not acknowledged either;
// - a start takes the latest acknowledged native epoch once as its ticket (`expectedEpoch`) and carries it in every
//   re-send; a generation check precedes every send (`invokeWithIngress`'s `stillCurrent`), so a start no longer current
//   is never sent and resolves cancelled;
// - `cancel_mbf_import` is sent as Tauri's own JSON arguments, the one form that survives the transport's `postMessage`
//   fallback; the starts are raw-body only, and after the fallback they are refused `INGRESS-RAW-BODY-REQUIRED`. Import
//   writes nothing, so that outcome is known and no "unavailable" page state is needed (§4.8.6).
//
// The ticket and ids authorize nothing: a retirement only cancels the run in progress and drops the chosen source.
import { invoke } from "@tauri-apps/api/core";
import { IngressRequestRetiredError, invokeWithIngress } from "../../services/nativeIngress";
import {
  MBF_IMPORT_CANCELLED,
  MBF_IMPORT_RETIREMENT_UNCONFIRMED,
  importCodeText,
  type MbfAdoptImportArgs,
  type MbfImportAdoptOutcome,
  type MbfImportOpenOutcome,
  type MbfImportReviewOutcome,
  type MbfReviewImportArgs
} from "./mbfImport";

export type MbfImportRetirementId = { page: string; sequence: number };

/** `cancel_mbf_import`'s reply. There is no `too_late`: import has no commit. */
export type MbfImportCancelReceipt = {
  cancelled: number;
  epoch: number;
  retirement_id: MbfImportRetirementId;
  duplicate?: true;
  exhausted?: true;
};

export const RETIREMENT_ID = "retirementId";
export const EXPECTED_EPOCH = "expectedEpoch";

/** One delivery of `cancel_mbf_import` for `id`. Never throws: `null` when unacknowledged (no id, a rejection, a reply
 * for another id, or an exhausted epoch). */
export async function cancelMbfImport(id: MbfImportRetirementId | null): Promise<MbfImportCancelReceipt | null> {
  if (id === null) return null;
  try {
    const answer = await invoke<MbfImportCancelReceipt>("cancel_mbf_import", { [RETIREMENT_ID]: id });
    const answers = answer?.retirement_id;
    return answer !== null && typeof answer === "object" && Number.isSafeInteger(answer.epoch) && answer.epoch >= 0 && answer.exhausted !== true
      && answers?.page === id.page && answers?.sequence === id.sequence
      ? answer
      : null;
  } catch {
    return null;
  }
}

let pageToken: string | null = null;
let sequence = 0;

function nextRetirementId(): MbfImportRetirementId | null {
  try {
    pageToken ??= Array.from(globalThis.crypto.getRandomValues(new Uint8Array(16)), (byte) => byte.toString(16).padStart(2, "0")).join("");
  } catch {
    return null;
  }
  if (!Number.isSafeInteger(sequence + 1)) return null;
  sequence += 1;
  return { page: pageToken, sequence };
}

let generation = 0;
let tail: Promise<boolean> | null = null;
let tailId: MbfImportRetirementId | null = null;
let deliveries = 0;
let acknowledgedEpoch: number | null = null;

function deliver(id: MbfImportRetirementId | null): Promise<MbfImportCancelReceipt | null> {
  const previous: Promise<unknown> = tail ?? Promise.resolve();
  const receipt = previous.then(() => cancelMbfImport(id));
  deliveries += 1;
  tailId = id;
  tail = receipt.then((answer) => {
    if (answer !== null) acknowledgedEpoch = Math.max(acknowledgedEpoch ?? 0, answer.epoch);
    return answer !== null;
  });
  return receipt;
}

/** Retires the import: starts not yet sent are never sent, and `cancel_mbf_import` is delivered once every earlier
 * delivery has settled. Natively it cancels the run in progress and drops the chosen source. */
export function retireMbfImportSession(): Promise<MbfImportCancelReceipt | null> {
  generation += 1;
  return deliver(nextRetirementId());
}

async function retirementSettled(): Promise<boolean> {
  if (tail === null) void deliver(nextRetirementId());
  const before = deliveries;
  for (;;) {
    const seen = tail as Promise<boolean>;
    const acknowledged = await seen;
    if (seen !== tail) continue;
    if (acknowledged) return true;
    if (deliveries > before) return false;
    void deliver(tailId);
  }
}

/** A start's result: its outcome and whether it is still current (no retirement since, and the caller's own check). A
 * caller discards a result that is not current. */
export type MbfImportStarted<T> = { outcome: T; current: () => boolean };
export type MbfImportStillCurrent = () => boolean;

type StartCommand = "open_mbf_import" | "review_mbf_import" | "adopt_mbf_import";

async function start<T>(command: StartCommand, args: Record<string, unknown>, stillCurrent: MbfImportStillCurrent | undefined, refused: (code: string) => T): Promise<MbfImportStarted<T>> {
  const started = generation;
  const current = () => started === generation && (stillCurrent?.() ?? true);
  const result = (outcome: T): MbfImportStarted<T> => ({ outcome, current });
  const confirmed = await retirementSettled();
  if (!current()) return result(refused(MBF_IMPORT_CANCELLED));
  const ticket = acknowledgedEpoch;
  if (!confirmed || ticket === null) return result(refused(MBF_IMPORT_RETIREMENT_UNCONFIRMED));
  try {
    return result(await invokeWithIngress<T>(command, { ...args, [EXPECTED_EPOCH]: ticket }, { stillCurrent: current }));
  } catch (error) {
    if (error instanceof IngressRequestRetiredError) return result(refused(MBF_IMPORT_CANCELLED));
    throw error;
  }
}

const diagnostic = (code: string) => [{ code, path: "", message: importCodeText(code) }];

/** The native `open_mbf_import` (the chooser, the selection and the first review) through the barrier. */
export function openMbfImport(stillCurrent?: MbfImportStillCurrent): Promise<MbfImportStarted<MbfImportOpenOutcome>> {
  return start<MbfImportOpenOutcome>("open_mbf_import", {}, stillCurrent, (code) => ({ outcome: "refused", fileName: "", diagnostics: diagnostic(code) }));
}

/** The native `review_mbf_import` (a declaration re-run) through the barrier. */
export function reviewMbfImport(args: MbfReviewImportArgs, stillCurrent?: MbfImportStillCurrent): Promise<MbfImportStarted<MbfImportReviewOutcome>> {
  return start<MbfImportReviewOutcome>("review_mbf_import", args, stillCurrent, (code) => ({ outcome: "refused", fileName: "", diagnostics: diagnostic(code) }));
}

/** The native `adopt_mbf_import` through the barrier; its reply is consumed once, by the import source's `read()`. */
export function adoptMbfImport(args: MbfAdoptImportArgs, stillCurrent?: MbfImportStillCurrent): Promise<MbfImportStarted<MbfImportAdoptOutcome>> {
  return start<MbfImportAdoptOutcome>("adopt_mbf_import", args, stillCurrent, (code) => ({ outcome: "refused", file_name: "", diagnostics: diagnostic(code) }));
}
