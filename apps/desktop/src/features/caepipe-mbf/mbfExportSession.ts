// File > Export MBF… (DEL-17-04 Design `desktop-mbf-export-import.md` §3.9.5, slice 4b.2b): the workspace session's
// ownership of the export's retirements and of a closed dialog's save.
//
// Retirement ownership. The export is retired once for each of these: the dialog closes (Close or Escape), the App
// unmounts, or the project session is replaced. A dialog request stays the session's until the user closes it
// (`release`) and its retirement has been sent. Until then the dialog's cleanup and the session's end both call the
// request's one retire, and only the first call sends anything, in whichever order they run: so an App unmount (the
// dialog's cleanup, then the session's) and a session replacement with the dialog open (the session's end, then the
// dialog's cleanup as the dialog closes) each send one. With no request held, the session's end retires on its own. A
// request opened but never mounted is retired by the session's end.
//
// A closed dialog's save. A save the dialog started and that had not settled when its request was retired is the
// session's to report, since the dialog no longer shows anything. When the retirement's receipt says it reached a save
// past its commit (`too_late`) that has not settled yet, the session first reports only that it is finishing. The
// save's own outcome is then reported by itself, whatever the receipt said (too late, none, or no receipt at all): the
// receipt does not prove that the save was cancelled before it wrote, since a save that has already completed natively
// (a write failure or an undone save included) is no longer counted in it. A saved outcome names both files; a refusal
// or a failure states its codes, a rollback's backups included. Only an outcome that itself establishes that nothing was
// written is not reported (`nothingWritten`). A save refused `INGRESS-RAW-BODY-REQUIRED` is reported as unknown, never
// as nothing written: Tauri's transport resends an invoke through its fallback when its `fetch` or the read of its reply
// fails, so the original save may have committed, written or undone itself before the resend was refused, and the page
// cannot see that it has fallen back (`mbfSaveOutcomeUnknown`). Each report carries the request it belongs to, so the
// session can attribute it.
import type { MbfCancelReceipt, MbfStarted } from "./mbfExportCancel";
import {
  MBF_EXPORT_CANCELLED,
  MBF_EXPORT_EPOCH_EXHAUSTED,
  MBF_EXPORT_EPOCH_INVALID,
  MBF_EXPORT_RETIREMENT_UNCONFIRMED,
  MBF_EXPORT_REVIEW_STALE,
  mbfFailureDiagnostic,
  mbfSaveOutcomeUnknown,
  type MbfExportSaveOutcome
} from "./mbfExport";
import type { ModelFileDiagnostic } from "../model-file/modelDocumentSave";

/** What the session is told about a closed dialog's save. */
export type ClosedMbfSaveReport =
  | { kind: "finishing" }
  | { kind: "saved"; outcome: Extract<MbfExportSaveOutcome, { outcome: "saved" }> }
  | { kind: "failed"; diagnostics: ModelFileDiagnostic[] }
  | { kind: "unknown"; diagnostics: ModelFileDiagnostic[] };

/** One open dialog request's part of the ownership. */
export type MbfExportRequestRetirement = {
  /** Retires the export for this request, once: later calls (the dialog's cleanup after the session's end, or the
   * reverse) do nothing. */
  retire: () => void;
  /** The user closed the dialog (Close or Escape): once its retirement has been sent, the session no longer holds it,
   * and a later session end retires on its own. */
  release: () => void;
  /** Tracks a save this request's dialog started, so that it is reported if the request is retired before it settles. */
  trackSave: (save: Promise<MbfStarted<MbfExportSaveOutcome>>) => void;
};

export type MbfExportRetirementOwner = {
  /** Opens a request: its retirement becomes the session's one retirement until it has been sent. */
  openRequest: (onClosedSave: (report: ClosedMbfSaveReport) => void) => MbfExportRequestRetirement;
  /** The project session ended (a new generation, or the workspace unmounting): one retirement, the open request's if
   * there is one. */
  sessionEnded: () => void;
};

/** The refusals that themselves establish that nothing was written: a run cancelled before its commit (or a start
 * never sent), a start not sent because an earlier retirement was unconfirmed, a start refused at the native entry by
 * its ticket, and a save whose review was stale (refused before the chooser). `INGRESS-RAW-BODY-REQUIRED` is not one:
 * for a save it may answer only the transport's resend of a save that wrote (`mbfSaveOutcomeUnknown`). */
const NOTHING_WRITTEN_CODES: ReadonlySet<string> = new Set([
  MBF_EXPORT_CANCELLED,
  MBF_EXPORT_RETIREMENT_UNCONFIRMED,
  MBF_EXPORT_EPOCH_INVALID,
  MBF_EXPORT_EPOCH_EXHAUSTED,
  MBF_EXPORT_REVIEW_STALE
]);

/** Whether a closed save's own outcome establishes that it wrote nothing: a chooser dismissed without a choice, a
 * refusal before the chooser from the save's worker run (it carries the review), or a refusal whose every code is one
 * of `NOTHING_WRITTEN_CODES`. Anything else (a write failure, an undone save, any other refusal or failure) may follow
 * a write and is reported. */
function nothingWritten(outcome: MbfExportSaveOutcome): boolean {
  if (outcome.outcome === "cancelled") return true;
  if (outcome.outcome !== "refused") return false;
  if (outcome.review !== undefined) return true;
  return outcome.diagnostics.length > 0 && outcome.diagnostics.every((item) => NOTHING_WRITTEN_CODES.has(item.code));
}

/** The report of a closed save's settled outcome, or `null` when it establishes that nothing was written. */
function closedSaveReport(settled: { outcome: MbfExportSaveOutcome } | { error: unknown }): ClosedMbfSaveReport | null {
  if ("error" in settled) {
    const diagnostics = [mbfFailureDiagnostic(settled.error)];
    if (mbfSaveOutcomeUnknown(diagnostics)) return { kind: "unknown", diagnostics };
    return NOTHING_WRITTEN_CODES.has(diagnostics[0].code) ? null : { kind: "failed", diagnostics };
  }
  const { outcome } = settled;
  if (outcome.outcome === "saved") return { kind: "saved", outcome };
  if (outcome.outcome === "refused" && mbfSaveOutcomeUnknown(outcome.diagnostics)) return { kind: "unknown", diagnostics: outcome.diagnostics };
  if (nothingWritten(outcome)) return null;
  return { kind: "failed", diagnostics: outcome.outcome === "refused" ? outcome.diagnostics : [] };
}

/** The session's owner, over the barrier's `retireMbfExportSession`. */
export function mbfExportRetirementOwner(retireSession: () => Promise<MbfCancelReceipt | null>): MbfExportRetirementOwner {
  let open: (() => void) | null = null;
  return {
    openRequest(onClosedSave) {
      let retired = false;
      let released = false;
      let pending: Promise<MbfStarted<MbfExportSaveOutcome>> | null = null;
      const retire = () => {
        if (retired) return;
        retired = true;
        if (released && open === retire) open = null;
        const save = pending;
        const receipt = retireSession();
        if (!save) return;
        let settled = false;
        // The save's own outcome, reported as soon as it settles, independently of the receipt.
        const report = (closed: ClosedMbfSaveReport | null) => {
          settled = true;
          if (closed) onClosedSave(closed);
        };
        void save.then(
          (started) => report(closedSaveReport({ outcome: started.outcome })),
          (error: unknown) => report(closedSaveReport({ error }))
        );
        // Only a save still unsettled when the receipt says it was past its commit is finishing.
        void receipt.then(
          (answer) => { if (answer !== null && answer.too_late > 0 && !settled) onClosedSave({ kind: "finishing" }); },
          () => {}
        );
      };
      open = retire;
      return {
        retire,
        release() {
          released = true;
          if (retired && open === retire) open = null;
        },
        trackSave(save) {
          pending = save;
          const clear = () => { if (pending === save) pending = null; };
          save.then(clear, clear);
        }
      };
    },
    sessionEnded() {
      const held = open;
      open = null;
      if (held) held();
      else void retireSession();
    }
  };
}
