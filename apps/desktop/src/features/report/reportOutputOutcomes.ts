// DEL-08-01 REQ-022: the outcome of every explicit report output (an HTML save, a print),
// owned by the workspace session and shown outside the Report section's dormant subtree
// (`ReportOutputStatus.tsx`), so that it survives the panel or render that started it, a
// hidden Report section and a project switch. Each operation has an id from its start; every
// return and every rejection of its request settles it, whichever panel is mounted. It keeps
// only metadata (the selected file's basename, the report basis and the outcome) until it is
// dismissed or the application session ends; nothing is written to disk and no journal is
// kept, so an application that ends during an operation leaves no outcome to show.
//
// A change of the workspace that the desktop application's native process had not registered
// before publication can leave the earlier selected report saved. The status then says that
// the selected report was saved and that the workspace has changed since it was prepared;
// it never presents that save as the current report's, and it never turns an unknown
// outcome into "not saved".
import { useState, useSyncExternalStore } from "react";
import { ResourceUnavailableError, describeResourceRefusal } from "../../services/memoryPool";
import type {
  RenderedReportHtmlSaveReceipt,
  RenderedReportPrintReceipt,
  RenderedReportRouteOutcome
} from "../../services/renderedReportRoutes";

/** What the report was prepared from: its report source number and its model and run. */
export type ReportOutputBasis = { sequence: number; model: string; run: string | null };

export type ReportOutputState =
  | "pending"
  | "saved"
  | "saved_durability_unconfirmed"
  | "cancelled"
  | "refused"
  | "failed"
  | "unknown"
  | "print_window_opened";

export type ReportOutputEntry = {
  id: number;
  kind: "save" | "print";
  basis: ReportOutputBasis;
  state: ReportOutputState;
  /** The native or frontend refusal code, when there is one. */
  code: string | null;
  /** The outcome's own detail (a refusal's message, the bytes and hash saved, …). */
  detail: string;
  /** The basename of the file the person selected, when known. */
  basename: string | null;
};

export type ReportOutputSettlement = Pick<ReportOutputEntry, "state" | "code" | "detail" | "basename">;

/** The session's report output outcomes (see the module comment). */
export class ReportOutputOutcomes {
  private entries: readonly ReportOutputEntry[] = [];
  private nextId = 1;
  private readonly listeners = new Set<() => void>();

  readonly subscribe = (listener: () => void) => {
    this.listeners.add(listener);
    return () => {
      this.listeners.delete(listener);
    };
  };

  readonly snapshot = () => this.entries;

  private publish(entries: readonly ReportOutputEntry[]) {
    this.entries = entries;
    for (const listener of this.listeners) listener();
  }

  /** An operation starts: its entry is pending until settled. */
  begin(kind: "save" | "print", basis: ReportOutputBasis): number {
    const id = this.nextId++;
    this.publish([...this.entries, { id, kind, basis, state: "pending", code: null, detail: "", basename: null }]);
    return id;
  }

  /** Settles an operation; an operation settles once (a dismissed one stays dismissed). */
  settle(id: number, settlement: ReportOutputSettlement): void {
    if (!this.entries.some((entry) => entry.id === id && entry.state === "pending")) return;
    this.publish(this.entries.map((entry) => (entry.id === id ? { ...entry, ...settlement } : entry)));
  }

  dismiss(id: number): void {
    this.publish(this.entries.filter((entry) => entry.id !== id));
  }
}

/** The session's stable owner and its current entries. */
export function useReportOutputOutcomes() {
  const [owner] = useState(() => new ReportOutputOutcomes());
  const entries = useSyncExternalStore(owner.subscribe, owner.snapshot, owner.snapshot);
  return { owner, entries };
}

function refusedText(error: { code: string; stage: string; message: string; destination: string; cleanup_code: string | null }): ReportOutputSettlement {
  // A save retired before publication (stage `currency` or `source`) is a refusal, though a
  // temporary file had been written and removed; a write that failed is a failure.
  const refusedStage = error.stage === "currency" || error.stage === "source" || error.stage === "admission";
  const state = error.destination === "unknown" ? "unknown" : error.destination === "as_it_was" && !refusedStage ? "failed" : "refused";
  const cleanup = error.cleanup_code ? `; a temporary file could not be removed (${error.cleanup_code})` : "";
  const destination = error.destination === "as_it_was" ? ` The destination was left as it was${cleanup}.` : "";
  return { state, code: error.code, detail: `${error.message}${destination}`, basename: null };
}

/** A request that failed without a native answer: refused before it ran when the native
 * side said so (a memory-pool refusal, an invalid request), otherwise unknown. */
function failedText(failure: unknown): ReportOutputSettlement {
  if (failure instanceof ResourceUnavailableError) {
    return { state: "refused", code: "RESOURCE_UNAVAILABLE", detail: `Refused by the native memory pool before anything ran: ${describeResourceRefusal(failure.refusal)}`, basename: null };
  }
  const message = failure instanceof Error ? failure.message : String(failure);
  if (message.startsWith("INVALID_REQUEST:")) return { state: "refused", code: "INVALID_REQUEST", detail: message, basename: null };
  return { state: "unknown", code: null, detail: message, basename: null };
}

/** The settlement of an HTML save's route outcome. */
export function saveSettlement(outcome: RenderedReportRouteOutcome<RenderedReportHtmlSaveReceipt>): ReportOutputSettlement {
  switch (outcome.kind) {
    case "done": {
      const receipt = outcome.receipt;
      if (receipt.outcome === "cancelled") return { state: "cancelled", code: null, detail: "", basename: null };
      const replaced = receipt.replaced_existing ? " It replaced the file that was there." : "";
      return {
        state: receipt.durability === "durable" ? "saved" : "saved_durability_unconfirmed",
        code: null,
        detail: `${receipt.byte_count} bytes, SHA-256 ${receipt.sha256_hex}.${replaced}`,
        basename: receipt.file_name
      };
    }
    case "refused":
      return { ...refusedText(outcome.error), basename: outcome.error.selected_basename };
    case "retired":
      return { state: "refused", code: "REPORT-REQUEST-RETIRED", detail: "The report changed before the request was sent; nothing was sent.", basename: null };
    case "failed":
      return failedText(outcome.failure);
  }
}

/** The settlement of a print's route outcome. */
export function printSettlement(outcome: RenderedReportRouteOutcome<RenderedReportPrintReceipt>): ReportOutputSettlement {
  switch (outcome.kind) {
    case "done":
      return { state: "print_window_opened", code: null, detail: `Derived print view of SHA-256 ${outcome.receipt.canonical_sha256_hex}.`, basename: null };
    case "refused":
      return refusedText(outcome.error);
    case "retired":
      return { state: "refused", code: "REPORT-REQUEST-RETIRED", detail: "The report changed before the request was sent; nothing was sent.", basename: null };
    case "failed":
      return failedText(outcome.failure);
  }
}

/** A refusal made before anything was sent (the source was not registered, the view was
 * stale). */
export function notSentSettlement(code: string, detail: string): ReportOutputSettlement {
  return { state: "refused", code, detail, basename: null };
}

/** The outcome in words. `currentSequence` is the workspace's report source number now. */
export function reportOutputText(entry: ReportOutputEntry, currentSequence: number): string {
  const changed = entry.basis.sequence !== currentSequence;
  const file = entry.basename ? ` File: ${entry.basename}.` : "";
  const basis = ` Report basis: model ${entry.basis.model}, run ${entry.basis.run ?? "none recorded"}.`;
  const code = entry.code ? ` (${entry.code})` : "";
  const detail = entry.detail ? ` ${entry.detail}` : "";
  if (entry.kind === "save") {
    switch (entry.state) {
      case "pending":
        return `Saving the selected report: choose where to save it.${basis}`;
      case "saved":
      case "saved_durability_unconfirmed": {
        const head = changed ? "Saved the selected report. The workspace has changed since it was prepared." : "Saved the selected report.";
        const durability = entry.state === "saved_durability_unconfirmed"
          ? " The whole file was written, but the system did not confirm it is durable on disk."
          : "";
        return `${head}${file}${detail}${durability}${basis}`;
      }
      case "cancelled":
        return `Save cancelled: nothing was written.${basis}`;
      case "refused":
      case "failed":
        return `Not saved${code}:${detail}${file}${basis}`;
      case "unknown":
        return `Save outcome unknown; check the file you selected.${detail}${basis}`;
      case "print_window_opened":
        return `${detail}${basis}`;
    }
  }
  switch (entry.state) {
    case "pending":
      return `Opening the print window for the selected report.${basis}`;
    case "print_window_opened": {
      const head = changed
        ? "Print window opened for the selected report. The workspace has changed since it was prepared."
        : "Print window opened for the selected report.";
      return `${head} Opening it does not mean anything was printed: choose Print or Cancel in its dialog.${detail}${basis}`;
    }
    case "unknown":
      return `Print outcome unknown.${detail}${basis}`;
    default:
      return `Not printed${code}:${detail}${basis}`;
  }
}
