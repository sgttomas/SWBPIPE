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
//
// The two report package saves (Save Report Package… and Save Shareable Report Package…) are
// kinds of their own beside the HTML save and the print (issue 508). A package save's entry is
// begun when its request is sent to the native host and settled from the native reply whatever
// has happened to the workspace since; a request that did not proceed before anything was sent
// is recorded settled, as not sent. Its basis is taken when the save began and never read again.
// The entry keeps the receipt's warnings, which are shown with it, and the receipt's figures,
// which are shown behind its details; and `listEntry`, which is where the saved package's entry
// in the project's list of saved reports will be stated (`ReportPackageFacts`).
import { useState, useSyncExternalStore } from "react";
import { ResourceUnavailableError, describeResourceRefusal } from "../../services/memoryPool";
import type { ReportPackageSaveReceipt } from "../../services/reportPackageSaveService";
import type {
  RenderedReportHtmlSaveReceipt,
  RenderedReportPrintReceipt,
  RenderedReportRouteOutcome
} from "../../services/renderedReportRoutes";

/** What the report was prepared from: its report source number, its project's identity (null
 * where there is none to compare), its model by the name the toolbar shows, and its run. */
export type ReportOutputBasis = { sequence: number; projectId: string | null; model: string; run: string | null };

export type ReportOutputState =
  | "pending"
  | "saved"
  | "saved_durability_unconfirmed"
  | "cancelled"
  | "refused"
  | "failed"
  | "unknown"
  | "print_window_opened";

/** The two report package saves: Save Report Package… and Save Shareable Report Package…. */
export type ReportPackageKind = "report_package" | "shareable_report_package";
export type ReportOutputKind = "save" | "print" | ReportPackageKind;

type ReportOutputEntryCommon = {
  id: number;
  basis: ReportOutputBasis;
  state: ReportOutputState;
  /** The native or frontend refusal code, when there is one. */
  code: string | null;
  /** The outcome's own detail (a refusal's message, the bytes and hash saved, …). */
  detail: string;
  /** The basename of the file the person selected, when known. */
  basename: string | null;
};

/** A non-blocking finding of the native screen that a saved package's receipt carries. */
export type ReportPackageWarning = { code: string; path: string; message: string };

/** What a package save's entry keeps of its native reply, beyond the common members. */
export type ReportPackageFacts = {
  /** Whether the request was sent to the native host; false: nothing was sent. */
  sent: boolean;
  /** Known to have written nothing: a request not sent, or one whose only sending was refused
   * before the save chooser and before any file (the memory pool's refusal) and then retired. */
  nothingWritten: boolean;
  /** The receipt's warnings, shown with the entry. */
  warnings: readonly ReportPackageWarning[];
  /** A saved package's figures, shown behind the entry's details; null for any other outcome. */
  receipt: {
    byteCount: number;
    containerSha256: string;
    packageIdentitySha256: string;
    replacedExisting: boolean;
    userReleaseRecorded: boolean;
    limitation: string;
  } | null;
  /** A write that failed and whose temporary file could not be removed. */
  temporaryFileMayRemain: boolean;
  /** A line kept behind the entry's details that carries its code, in place of the code on its
   * line (as a saved package's receipt code is kept): a size refusal's code and exact sizes.
   * Null for every other entry, whose code stays on its line. */
  detailsLine: string | null;
  /** The saved package's entry in the project's list of saved reports (issue 498), as the native
   * receipt states it. Null while the receipt states nothing of it, as every receipt does before
   * the list lands. The list's interface widens this to `null | <its outcome>` and adds its words
   * in `reportOutputText`, so an entry kept before then keeps its shape and its words. */
  listEntry: null;
};

export type ReportOutputEntry =
  | (ReportOutputEntryCommon & { kind: "save" | "print" })
  | ReportPackageOutputEntry;
export type ReportPackageOutputEntry = ReportOutputEntryCommon & { kind: ReportPackageKind; package: ReportPackageFacts };

export type ReportOutputSettlement = Pick<ReportOutputEntryCommon, "state" | "code" | "detail" | "basename">;
export type ReportPackageSettlement = ReportOutputSettlement & { package: ReportPackageFacts };

export function isReportPackageEntry(entry: ReportOutputEntry): entry is ReportPackageOutputEntry {
  return entry.kind === "report_package" || entry.kind === "shareable_report_package";
}

const SENT_PACKAGE: ReportPackageFacts = { sent: true, nothingWritten: false, warnings: [], receipt: null, temporaryFileMayRemain: false, detailsLine: null, listEntry: null };

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

  /** An operation starts: its entry is pending until settled. A package save begins when its
   * request is sent to the native host. */
  begin(kind: ReportOutputKind, basis: ReportOutputBasis): number {
    const id = this.nextId++;
    const common = { id, basis, state: "pending" as const, code: null, detail: "", basename: null };
    const entry: ReportOutputEntry = kind === "save" || kind === "print" ? { ...common, kind } : { ...common, kind, package: SENT_PACKAGE };
    this.publish([...this.entries, entry]);
    return id;
  }

  /** A package save that settled before anything was sent: its entry, already settled. */
  record(kind: ReportPackageKind, basis: ReportOutputBasis, settlement: ReportPackageSettlement): number {
    const id = this.nextId++;
    this.publish([...this.entries, { id, kind, basis, ...settlement }]);
    return id;
  }

  /** Settles an operation; an operation settles once (a dismissed one stays dismissed). A
   * package save's facts settle only a package save's entry. */
  settle(id: number, settlement: ReportOutputSettlement | ReportPackageSettlement): void {
    if (!this.entries.some((entry) => entry.id === id && entry.state === "pending")) return;
    const { state, code, detail, basename } = settlement;
    this.publish(this.entries.map((entry): ReportOutputEntry => {
      if (entry.id !== id) return entry;
      if (!isReportPackageEntry(entry)) return { ...entry, state, code, detail, basename };
      return { ...entry, state, code, detail, basename, package: "package" in settlement ? settlement.package : entry.package };
    }));
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
    return { state: "refused", code: "RESOURCE_UNAVAILABLE", detail: `Refused by the memory pool before anything ran: ${describeResourceRefusal(failure.refusal)}`, basename: null };
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

/** An output's basis, taken once, from the committed report source the press was made on: the
 * project's identity (`model.project.id`), the model by the name the toolbar shows (the saved
 * project's summary name, else the document's project name, else its id) and the run. */
export function reportOutputBasisOf(
  sequence: number,
  project: { name?: string | null; id: string },
  summaryName: string | null | undefined,
  run: string | null
): ReportOutputBasis {
  return { sequence, projectId: project.id, model: summaryName ?? project.name ?? project.id, run };
}

/** Whether the open project is not the one the entry's report was prepared from. Only two known
 * identities are compared; an entry or a workspace without one is never called another project. */
function ofAnotherProject(basis: ReportOutputBasis, currentProjectId: string | null | undefined): boolean {
  return basis.projectId !== null && currentProjectId != null && basis.projectId !== currentProjectId;
}

/** How every outcome names the model and the run it was prepared from. */
export function reportOutputBasisNames(basis: ReportOutputBasis): { model: string; run: string } {
  return { model: basis.model, run: basis.run ?? "none recorded" };
}

const REQUEST_RETIRED = "REPORT-REQUEST-RETIRED";
/** A package save's reading of the memory pool's refusal for want of free memory (the "Memory
 * pool" panel's name for the allowance). */
const MEMORY_POOL_SHORT = "the memory pool had too little free at that moment";
/** The native save's own worker failure: the save's outcome is not known. */
const PACKAGE_WORKER_FAILED = "REPORT-PACKAGE-SAVE-WORKER-FAILED";
/** Native stages that refuse before anything is written: admission, the producer and the
 * chooser's path, and the writer's validation of the destination. */
const PACKAGE_REFUSED_STAGES = new Set(["admit", "assemble", "validate"]);
/** The atomic writer's stages before its rename returns: a failure leaves the destination as it
 * was, and its temporary file is removed unless its cleanup failed. */
const PACKAGE_WRITER_STAGES = new Set(["create_temp", "write", "flush", "temp_sync", "rename"]);

function packageSettlement(settlement: ReportOutputSettlement, facts: Partial<ReportPackageFacts> = {}): ReportPackageSettlement {
  return { ...settlement, package: { ...SENT_PACKAGE, ...facts } };
}

/** The shareable command's reason when another project operation or dialog is open. One text:
 * the session sets it and compares a shown reason with it (a "not requested" reason is shown
 * while it is still true), so the two must not drift apart. */
export const PROJECT_OPERATION_OPEN_REFUSAL = "Not saved: another project operation or dialog was open.";

/** The settlement of a package save's native receipt (saved or cancelled at the chooser). */
export function packageReceiptSettlement(receipt: ReportPackageSaveReceipt): ReportPackageSettlement {
  if (receipt.outcome === "cancelled") return packageSettlement({ state: "cancelled", code: null, detail: "", basename: null });
  if (receipt.outcome !== "saved") {
    // A receipt outcome this application does not know: whether anything was written is unknown.
    const outcome: unknown = receipt.outcome;
    return packageSettlement({
      state: "unknown",
      code: typeof receipt.code === "string" ? receipt.code : null,
      detail: `The native receipt's outcome ${JSON.stringify(outcome) ?? String(outcome)} is not one this application knows.`,
      basename: typeof receipt.selected_basename === "string" && receipt.selected_basename !== "" ? receipt.selected_basename : null
    });
  }
  return packageSettlement({
    state: receipt.durability === "durable" ? "saved" : "saved_durability_unconfirmed",
    code: receipt.code,
    detail: "",
    basename: receipt.selected_basename || receipt.container_file_name
  }, {
    warnings: (receipt.warnings ?? []).map(({ code, path, message }) => ({ code, path, message })),
    receipt: {
      byteCount: receipt.byte_count,
      containerSha256: receipt.container_sha256_hex,
      packageIdentitySha256: receipt.package_identity_sha256_hex,
      replacedExisting: receipt.replaced_existing,
      userReleaseRecorded: (receipt as { user_release_recorded?: unknown }).user_release_recorded === true,
      limitation: receipt.limitation
    }
  });
}

/** The settlement of a package save whose request was sent and rejected: refused where the
 * native side refused before anything was written, failed where the writer failed before its
 * rename returned, and otherwise unknown (never "nothing written"). */
export function packageFailureSettlement(failure: unknown): ReportPackageSettlement {
  if (failure instanceof ResourceUnavailableError) {
    const reading = failure.refusal.kind === "headroom_insufficient" ? MEMORY_POOL_SHORT : describeResourceRefusal(failure.refusal);
    return packageSettlement({ state: "refused", code: "RESOURCE_UNAVAILABLE", detail: reading, basename: null });
  }
  if (failure instanceof Error && failure.message.startsWith("INVALID_REQUEST:")) return packageSettlement(failedText(failure));
  if (failure instanceof Error) return packageSettlement({ state: "unknown", code: null, detail: failure.message, basename: null });
  const error = (failure && typeof failure === "object" ? failure : {}) as Record<string, unknown>;
  if (typeof error.code !== "string" || typeof error.stage !== "string" || typeof error.message !== "string") {
    let detail: string;
    try { detail = JSON.stringify(failure) ?? String(failure); } catch { detail = String(failure); }
    return packageSettlement({ state: "unknown", code: null, detail, basename: null });
  }
  const basename = typeof error.selected_basename === "string" && error.selected_basename !== "" ? error.selected_basename : null;
  const known = { code: error.code, detail: error.message, basename };
  if (error.code === PACKAGE_WORKER_FAILED) return packageSettlement({ state: "unknown", ...known });
  if (PACKAGE_REFUSED_STAGES.has(error.stage)) return packageSettlement({ state: "refused", ...known });
  if (PACKAGE_WRITER_STAGES.has(error.stage)) {
    return packageSettlement({ state: "failed", ...known }, { temporaryFileMayRemain: typeof error.cleanup_code === "string" });
  }
  return packageSettlement({ state: "unknown", ...known });
}

/** A package save that did not proceed before anything was sent, with its reading and its
 * code (the error's own, else the reading's leading code). */
export function packageNotSentSettlement(reason: unknown): ReportPackageSettlement {
  const reading = reason instanceof Error ? reason.message : typeof reason === "string" ? reason : String(reason);
  const own = reason && typeof reason === "object" && typeof (reason as { code?: unknown }).code === "string" ? (reason as { code: string }).code : null;
  const code = own ?? /^([A-Z][A-Z0-9]*(?:[-_][A-Z0-9]+)+)(?=[:\s(]|$)/.exec(reading)?.[1] ?? null;
  return packageSettlement({ state: "refused", code, detail: reading, basename: null }, { sent: false, nothingWritten: true });
}

/** A package save refused before anything was sent because the result or its run record is too
 * large for the report package's full copy: `reading` is the line's (the size refusal's lower-case
 * reading, with no code), and `details` (the code and the exact sizes) is kept behind its details. */
export function packageSizeRefusedSettlement(code: string, words: { reading: string; details: string }): ReportPackageSettlement {
  return packageSettlement({ state: "refused", code, detail: words.reading, basename: null }, { sent: false, nothingWritten: true, detailsLine: words.details });
}

/** A package save whose basis changed before its request was sent (or, after the memory pool
 * refused it for want of free memory, before it was sent again): nothing was written. The memory
 * pool's refusal of the package save, on either route, comes before the save chooser and before
 * any file is created: at the command's ingress and its lease growth (`lib.rs`,
 * `save_report_package`) and, on the shareable route, in admission step 5
 * (`report_package_save.rs`, `admit_user_released`), all before `save_admitted_report_package_in`
 * opens the chooser and writes. */
export function packageRetiredSettlement(resend = false): ReportPackageSettlement {
  const detail = resend ? `${MEMORY_POOL_SHORT}, and the workspace changed before it could be tried again` : "the workspace changed before the save began";
  return packageSettlement({ state: "refused", code: REQUEST_RETIRED, detail, basename: null }, { sent: resend, nothingWritten: true });
}

function packageName(kind: ReportPackageKind): string {
  return kind === "report_package" ? "report package" : "shareable report package";
}
const capitalized = (text: string) => text.charAt(0).toUpperCase() + text.slice(1);

/** A refusal's reading on the line: its detail without a leading `<CODE>: ` (the code follows
 * in parentheses) or a trailing full stop, then the code unless the reading begins with it. */
function readingWithCode(detail: string, code: string | null): string {
  let reading = code && detail.startsWith(`${code}: `) ? detail.slice(code.length + 2) : detail;
  reading = reading.trim().replace(/\.$/, "");
  const coded = code && !reading.startsWith(code) ? ` (${code})` : "";
  return reading ? `: ${reading}${coded}` : coded;
}

/** A package save's line: the outcome first, named by the command, then the file; where another
 * project is open, that the entry is of another project, by that project's name; where the same
 * project has changed since the save began, the model and the run it was prepared from. */
function packageOutputText(entry: ReportPackageOutputEntry, changed: boolean, anotherProject: boolean): string {
  const name = packageName(entry.kind);
  const { model, run } = reportOutputBasisNames(entry.basis);
  const earlier = anotherProject
    ? ` It is of another project: ${model}.`
    : changed ? ` It is of an earlier run: ${model}, run ${run}. The model or its results have changed since.` : "";
  const file = entry.basename ?? "the selected file";
  const Name = capitalized(name);
  switch (entry.state) {
    case "pending":
      return `Choose where to save the ${name}.${earlier}`;
    case "saved":
      return `${Name} saved as ${file}.${earlier}`;
    case "saved_durability_unconfirmed":
      return `${Name} saved as ${file}. The system did not confirm it is safely on disk.${earlier}`;
    case "cancelled":
      return `${Name} not saved: the save was cancelled.${earlier}`;
    case "refused":
    case "failed": {
      const temporary = entry.state === "failed" && entry.package.temporaryFileMayRemain ? ` A temporary file may remain beside ${file}.` : "";
      const nothing = entry.package.nothingWritten ? " Nothing was written." : "";
      // An entry whose code is kept behind its details reads without it on its line.
      const reading = entry.package.detailsLine !== null ? `: ${entry.detail}` : readingWithCode(entry.detail, entry.code);
      return `${Name} not saved${reading}.${temporary}${nothing}${earlier}`;
    }
    case "unknown":
    case "print_window_opened":
      return `It is not known whether the ${name} was saved. Check ${entry.basename ?? "the file you selected"}.${earlier}`;
  }
}

/** What an entry shows behind its details: the full basis, a saved package's figures and
 * statements, and a reading that is not on its line. Empty for an HTML save and a print. */
export function reportOutputDetails(entry: ReportOutputEntry): string[] {
  if (!isReportPackageEntry(entry) || entry.state === "pending") return [];
  const { model, run } = reportOutputBasisNames(entry.basis);
  const lines = [`Report basis: model ${model}, run ${run}.`];
  const receipt = entry.package.receipt;
  if (receipt) {
    lines.push(`${receipt.byteCount} bytes; container SHA-256 ${receipt.containerSha256}; package identity SHA-256 ${receipt.packageIdentitySha256}.`);
    if (receipt.replacedExisting) lines.push("It replaced the file that was there.");
    lines.push(entry.state === "saved"
      ? "The system confirmed the file is safely on disk."
      : "The whole file was written, but the system did not confirm it is safely on disk.");
    if (receipt.userReleaseRecorded) lines.push("The package records the user release.");
    if (entry.code) lines.push(`Receipt code: ${entry.code}.`);
    lines.push(`Limitation: ${receipt.limitation}`);
  }
  if (entry.package.detailsLine !== null) lines.push(entry.package.detailsLine);
  if (entry.state === "failed") lines.push("The selected file was left as it was.");
  if (entry.state === "unknown" && entry.detail) lines.push(entry.code && !entry.detail.startsWith(entry.code) ? `${entry.detail} (${entry.code})` : entry.detail);
  return lines;
}

/** The outcome in words. `currentSequence` is the workspace's report source number now and
 * `currentProjectId` the open project's identity (unknown: undefined). */
export function reportOutputText(entry: ReportOutputEntry, currentSequence: number, currentProjectId?: string | null): string {
  const changed = entry.basis.sequence !== currentSequence;
  const anotherProject = ofAnotherProject(entry.basis, currentProjectId);
  if (isReportPackageEntry(entry)) return packageOutputText(entry, changed, anotherProject);
  const elsewhere = anotherProject ? ` It is of another project: ${entry.basis.model}.` : "";
  const file = entry.basename ? ` File: ${entry.basename}.` : "";
  const names = reportOutputBasisNames(entry.basis);
  const basis = ` Report basis: model ${names.model}, run ${names.run}.`;
  const code = entry.code ? ` (${entry.code})` : "";
  const detail = entry.detail ? ` ${entry.detail}` : "";
  if (entry.kind === "save") {
    switch (entry.state) {
      case "pending":
        return `Saving the selected report: choose where to save it.${basis}`;
      case "saved":
      case "saved_durability_unconfirmed": {
        const head = changed ? `Saved the selected report. The workspace has changed since it was prepared.${elsewhere}` : "Saved the selected report.";
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
        ? `Print window opened for the selected report. The workspace has changed since it was prepared.${elsewhere}`
        : "Print window opened for the selected report.";
      return `${head} Opening it does not mean anything was printed: choose Print or Cancel in its dialog.${detail}${basis}`;
    }
    case "unknown":
      return `Print outcome unknown.${detail}${basis}`;
    default:
      return `Not printed${code}:${detail}${basis}`;
  }
}
