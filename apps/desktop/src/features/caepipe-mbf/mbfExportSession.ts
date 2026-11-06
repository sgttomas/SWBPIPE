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
// past its commit (`too_late`), the save may still be writing, or may fail and undo itself: the session reports only
// that it is finishing. Whatever the save then does, its own outcome is reported: a saved receipt names both files, a
// refusal states its codes. A save the retirement cancelled before its commit wrote nothing and is not reported, unless
// its outcome says it saved. Each report carries the request it belongs to, so the session can attribute it.
import type { MbfCancelReceipt, MbfStarted } from "./mbfExportCancel";
import { mbfFailureDiagnostic, type MbfExportSaveOutcome } from "./mbfExport";
import type { ModelFileDiagnostic } from "../model-file/modelDocumentSave";

/** What the session is told about a closed dialog's save. */
export type ClosedMbfSaveReport =
  | { kind: "finishing" }
  | { kind: "saved"; outcome: Extract<MbfExportSaveOutcome, { outcome: "saved" }> }
  | { kind: "failed"; diagnostics: ModelFileDiagnostic[] };

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

type Settled = { kind: "outcome"; outcome: MbfExportSaveOutcome } | { kind: "error"; error: unknown };

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
        const outcome: Promise<Settled> = save.then(
          (started) => { settled = true; return { kind: "outcome", outcome: started.outcome }; },
          (error: unknown) => { settled = true; return { kind: "error", error }; }
        );
        void receipt.then((answer) => {
          if (answer !== null && answer.too_late > 0 && !settled) onClosedSave({ kind: "finishing" });
        });
        void Promise.all([receipt, outcome]).then(([answer, result]) => {
          if (result.kind === "outcome" && result.outcome.outcome === "saved") {
            onClosedSave({ kind: "saved", outcome: result.outcome });
            return;
          }
          // Not past its commit: cancelled before anything was written, which the close itself asked for. A chooser
          // dismissed without a choice wrote nothing either.
          if (answer === null || answer.too_late === 0) return;
          if (result.kind === "error") {
            onClosedSave({ kind: "failed", diagnostics: [mbfFailureDiagnostic(result.error)] });
          } else if (result.outcome.outcome === "refused") {
            onClosedSave({ kind: "failed", diagnostics: result.outcome.diagnostics });
          }
        });
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
