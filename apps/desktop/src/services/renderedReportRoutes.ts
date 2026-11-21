// The rendered report's native routes in the desktop (DEL-08-01 REQ-022;
// DEL-12-02 AC-007): Save canonical HTML… and Print…, both route
// DREP-HTML-SAVE-005. In the native application a `download` link and a print
// iframe do nothing (the shell has no download handler, and its print reaches
// only the main frame), so the report panel asks
// `save_rendered_report_html` (system save chooser, atomic writer) and
// `print_rendered_report` (the print view in a window of its own, the system
// print dialog) for the render it shows. The native side keeps the current
// render's canonical HTML; a request carries no document and names the render
// by the SHA-256 the render's reply gave, which is admitted only when it is the
// current render's. The native side reads the context and the block from its
// record, saves exactly the bytes it holds and derives the print view from them
// (`src-tauri/src/rendered_report_routes.rs`, `rendered_report_print.rs`).
// Each request names the report source number its render was prepared for
// (`features/workspace/reportSourceSequence.ts`); the native side refuses one
// that is retired or not yet registered.
// Outside the native application the panel keeps its link and iframe.
// Open in a window (`open_rendered_report_view`, `src-tauri/src/report_view.rs`)
// shows the held document in a native window of its own, named the same way;
// nothing leaves through it, so it carries no controlled export's evidence.
import { invokeWithIngress, IngressRequestRetiredError } from "./nativeIngress";
import { ResourceUnavailableError } from "./memoryPool";
import type { ReportRenderRequest, ReportSourceMark } from "../features/workspace/reportSourceSequence";
import { isTauriRuntime } from "./nativeMenu";
import { invoke } from "@tauri-apps/api/core";
import {
  controlRouteExport,
  type ControlledRouteExport
} from "../features/redaction-controls/redactionExportControls";

export const RENDERED_REPORT_ROUTE_ID = "DREP-HTML-SAVE-005";

export type RenderedReportRouteContext = "public_report" | "local_private";

/** Whether the panel's save and print go through the native routes. */
export const isNativeRenderedReportRuntime = isTauriRuntime;

export type RenderedReportHtmlSaveReceipt = {
  outcome: "saved" | "cancelled";
  route_id: string;
  export_context: RenderedReportRouteContext;
  sha256_hex: string;
  byte_count: number;
  file_name: string;
  replaced_existing: boolean;
  durability: "durable" | "saved_durability_uncertain" | "not_applicable";
};

export type RenderedReportPrintReceipt = {
  outcome: "print_window_opened";
  route_id: string;
  export_context: RenderedReportRouteContext;
  canonical_sha256_hex: string;
  print_sha256_hex: string;
};

/** A native refusal or failure. `destination`: `untouched` (nothing was chosen
 * or written), `as_it_was` (the write failed, or the save was retired, before the
 * file was published) or `unknown` (the native worker did not complete). */
export type RenderedReportRouteError = {
  code: string;
  stage: string;
  message: string;
  destination: "untouched" | "as_it_was" | "unknown";
  cleanup_code: string | null;
  selected_basename: string | null;
};

export type RenderedReportRouteOutcome<T> =
  | { kind: "done"; receipt: T }
  | { kind: "refused"; error: RenderedReportRouteError }
  | { kind: "retired" }
  /** No native answer: `failure` is what the request rejected with. */
  | { kind: "failed"; message: string; failure: unknown };

/** The route's controlled export of the canonical HTML under the render's
 * context, returned before the route exposes anything (DEL-12-02 AC-007). */
export function controlRenderedReportHtml(
  html: string,
  context: RenderedReportRouteContext,
  explicitLocalPrivateIntent: boolean
): ControlledRouteExport {
  return controlRouteExport(html, {
    routeId: RENDERED_REPORT_ROUTE_ID,
    exportContext: context,
    explicitLocalPrivateIntent,
    requireLosslessMaterialization: true
  });
}

/** Whether a controlled export lets the route expose the canonical HTML: not
 * blocked, and the controlled payload is the canonical HTML unchanged (the
 * route is lossless). */
export function renderedReportRouteOpen(controlled: ControlledRouteExport, html: string): boolean {
  return !controlled.blocked && controlled.payload === html && controlled.summary.blocking_count === 0;
}

/** `controlled` is the HTML route's controlled export where the panel applies
 * it to the sink: the save always, the print of a local-private render. It is
 * `null` for the print of a public render, whose sink the panel has always
 * gated by the render's own public gate alone (the render input controlled
 * under DREP-IPC-003 and the renderer's export gates), as before. */
type RouteRequest = {
  /** The SHA-256 the render's reply gave: it names the render; no document is sent. */
  sha256Hex: string;
  /** The canonical HTML as the web view holds it. Used here only to check the
   * controlled export against it; it is never part of what is sent. */
  html: string;
  context: RenderedReportRouteContext;
  explicitLocalPrivateIntent: boolean;
  controlled: ControlledRouteExport | null;
  /** The report source number the render was prepared for, registered natively. */
  source: ReportSourceMark;
};

function routeArgs(request: RouteRequest): Record<string, unknown> {
  return {
    sha256Hex: request.sha256Hex,
    exportContext: request.context,
    explicitLocalPrivateIntent: request.explicitLocalPrivateIntent,
    evidence: {
      route_id: request.controlled?.summary.route_id ?? RENDERED_REPORT_ROUTE_ID,
      export_context: request.context,
      explicit_local_private_intent: request.explicitLocalPrivateIntent,
      blocked: request.controlled?.blocked ?? false,
      blocking_count: request.controlled?.summary.blocking_count ?? 0
    },
    source: { incarnation: request.source.incarnation, sequence: request.source.sequence }
  };
}

function isRouteError(value: unknown): value is RenderedReportRouteError {
  if (typeof value !== "object" || value === null) return false;
  const candidate = value as Record<string, unknown>;
  return typeof candidate.code === "string" && typeof candidate.stage === "string" && typeof candidate.message === "string";
}

// The two commands take their arguments as a raw body under the ingress lease,
// as `render_calculation_report` does. The route check (`renderedReportRouteOpen`,
// which compares the controlled payload with the canonical HTML) runs here as
// before, on the web view's own copy; the document itself is not sent.
async function send<T>(
  command: "save_rendered_report_html" | "print_rendered_report",
  request: RouteRequest,
  stillCurrent: () => boolean
): Promise<RenderedReportRouteOutcome<T>> {
  if (request.controlled !== null && !renderedReportRouteOpen(request.controlled, request.html)) {
    return {
      kind: "refused",
      error: {
        code: "REPORT-ROUTE-CONTROLLED-EXPORT-BLOCKED",
        stage: "controlled_export",
        message: "The route's controlled export did not pass; nothing was sent.",
        destination: "untouched",
        cleanup_code: null,
        selected_basename: null
      }
    };
  }
  try {
    const receipt = await invokeWithIngress<T>(command, routeArgs(request), { stillCurrent });
    return { kind: "done", receipt };
  } catch (error) {
    if (error instanceof IngressRequestRetiredError) return { kind: "retired" };
    if (isRouteError(error)) return { kind: "refused", error };
    return { kind: "failed", message: error instanceof Error ? error.message : String(error), failure: error };
  }
}

export function saveRenderedReportHtml(request: RouteRequest & { controlled: ControlledRouteExport }, stillCurrent: () => boolean) {
  return send<RenderedReportHtmlSaveReceipt>("save_rendered_report_html", request, stillCurrent);
}

export function printRenderedReport(request: RouteRequest, stillCurrent: () => boolean) {
  return send<RenderedReportPrintReceipt>("print_rendered_report", request, stillCurrent);
}

/** The panel retires one render request it issued and no longer shows: the
 * native side retires that exact request (pending or completed, closing its
 * print window), or refuses its later entry if it has not entered; any other
 * request, older or newer, stays. A failure only leaves the record to the next
 * render, which replaces it. */
export function retireRenderedReport(request: ReportRenderRequest): void {
  try {
    void Promise.resolve(
      invoke("retire_rendered_report", {
        source: { incarnation: request.source.incarnation, sequence: request.source.sequence },
        request: request.request
      })
    ).catch(() => undefined);
  } catch {
    // No native runtime to reach: nothing is held.
  }
}

/** The outcome in words, for the panel's status line. */
export function saveOutcomeText(outcome: RenderedReportRouteOutcome<RenderedReportHtmlSaveReceipt>): string {
  switch (outcome.kind) {
    case "done": {
      const receipt = outcome.receipt;
      if (receipt.outcome === "cancelled") return "Save cancelled: nothing was written.";
      const replaced = receipt.replaced_existing ? " (replaced the file that was there)" : "";
      const durability = receipt.durability === "durable"
        ? ""
        : " The whole file was written, but the system did not confirm it is durable on disk.";
      return `Saved ${receipt.file_name}${replaced}: ${receipt.byte_count} bytes, SHA-256 ${receipt.sha256_hex}.${durability}`;
    }
    case "refused":
      if (outcome.error.destination === "unknown") return `Save outcome unknown; check the file you selected. ${outcome.error.message}`;
      return `Not saved (${outcome.error.code}): ${outcome.error.message}${destinationText(outcome.error)}`;
    case "retired":
      return "Not saved: the report is no longer current; nothing was sent.";
    case "failed":
      // Refused before anything ran only when the native side said so; otherwise the
      // file may have been written, so the outcome is unknown, never "not saved".
      return refusedBeforeRunning(outcome.failure)
        ? `Not saved: ${outcome.message}`
        : `Save outcome unknown; check the file you selected. ${outcome.message}`;
  }
}

/** Whether a request with no route answer was refused before the command ran. */
export function refusedBeforeRunning(failure: unknown): boolean {
  if (failure instanceof ResourceUnavailableError) return true;
  return failure instanceof Error && failure.message.startsWith("INVALID_REQUEST:");
}

export function printOutcomeText(outcome: RenderedReportRouteOutcome<RenderedReportPrintReceipt>): string {
  switch (outcome.kind) {
    case "done":
      return `Print window opened with the derived print view of SHA-256 ${outcome.receipt.canonical_sha256_hex}; choose Print or Cancel in its dialog.`;
    case "refused":
      if (outcome.error.destination === "unknown") return `Print outcome unknown. ${outcome.error.message}`;
      return `Not printed (${outcome.error.code}): ${outcome.error.message}`;
    case "retired":
      return "Not printed: the report is no longer current; nothing was sent.";
    case "failed":
      return refusedBeforeRunning(outcome.failure) ? `Not printed: ${outcome.message}` : `Print outcome unknown. ${outcome.message}`;
  }
}

function destinationText(error: RenderedReportRouteError): string {
  if (error.destination !== "as_it_was") return "";
  return ` The destination was left as it was${error.cleanup_code ? `; a temporary file could not be removed (${error.cleanup_code})` : ""}.`;
}

// Open in a window (DEL-08-01 REQ-022; `src-tauri/src/report_view.rs`). The words a person sees
// are each here once.

/** The panel's button. */
export const OPEN_REPORT_VIEW_LABEL = "Open in a window";
export const REPORT_VIEW_OPENING = "Opening the report window…";
export const REPORT_VIEW_OPENED = "Report opened in a window.";
export const REPORT_VIEW_BROUGHT_FORWARD = "The report's window was brought forward.";
/** Shown on the Report page when the desktop closed a view window of the current render because
 * the result it showed is no longer current, until the next render or until dismissed. */
export const REPORT_VIEW_CLOSED_NOT_CURRENT = "The report window closed because the result it showed is no longer current.";
export const REPORT_VIEW_CLOSED_DISMISS = "Dismiss";

/** A refusal: the message first, the code last in brackets. */
export function reportViewRefusalText(message: string, code: string | null): string {
  return `The report could not be opened in a window: ${message}${code ? ` (${code})` : ""}`;
}

export type RenderedReportViewReceipt = {
  outcome: "opened" | "brought_forward";
  kind: string;
  sha256_hex: string;
};

/** Opens the shown render in a window of its own: the request names the render by the SHA-256
 * the render's reply gave and the source it was prepared for; it carries no document. */
export async function openRenderedReportView(
  request: { sha256Hex: string; source: ReportSourceMark },
  stillCurrent: () => boolean
): Promise<RenderedReportRouteOutcome<RenderedReportViewReceipt>> {
  try {
    const receipt = await invokeWithIngress<RenderedReportViewReceipt>(
      "open_rendered_report_view",
      { sha256Hex: request.sha256Hex, source: { incarnation: request.source.incarnation, sequence: request.source.sequence } },
      { stillCurrent }
    );
    return { kind: "done", receipt };
  } catch (error) {
    if (error instanceof IngressRequestRetiredError) return { kind: "retired" };
    if (isRouteError(error)) return { kind: "refused", error };
    return { kind: "failed", message: error instanceof Error ? error.message : String(error), failure: error };
  }
}

export function viewOutcomeText(outcome: RenderedReportRouteOutcome<RenderedReportViewReceipt>): string {
  switch (outcome.kind) {
    case "done":
      return outcome.receipt.outcome === "brought_forward" ? REPORT_VIEW_BROUGHT_FORWARD : REPORT_VIEW_OPENED;
    case "refused":
      return reportViewRefusalText(outcome.error.message, outcome.error.code);
    case "retired":
      return reportViewRefusalText("the report is no longer current; nothing was sent.", null);
    case "failed":
      return refusedBeforeRunning(outcome.failure)
        ? reportViewRefusalText(outcome.message, null)
        : `Whether the report's window opened is not known: ${outcome.message}`;
  }
}

/** Asks the desktop to close every window showing the current render: the panel no longer shows
 * it as current. It closes windows only; the render, and any save of it, are untouched. */
export function closeRenderedReportViews(): Promise<void> {
  try {
    return Promise.resolve(invoke("close_rendered_report_views")).then(() => undefined, () => undefined);
  } catch {
    return Promise.resolve();
  }
}

export type ReportViewClosure = { kind: string; reason: string };

/** The desktop's pending notices of view windows it closed because what they showed is no longer
 * showable, taken once. Only what the desktop returns here is reported; nothing is inferred. */
export async function takeReportViewClosures(): Promise<ReportViewClosure[]> {
  try {
    const taken: unknown = await invoke("take_report_view_closures");
    if (!Array.isArray(taken)) return [];
    return taken.filter((entry): entry is ReportViewClosure =>
      typeof entry === "object" && entry !== null && typeof (entry as ReportViewClosure).kind === "string" && typeof (entry as ReportViewClosure).reason === "string"
    );
  } catch {
    return [];
  }
}
