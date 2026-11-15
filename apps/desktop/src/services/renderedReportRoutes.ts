// The rendered report's native routes in the desktop (DEL-08-01 REQ-022;
// DEL-12-02 AC-007): Save canonical HTML… and Print…, both route
// DREP-HTML-SAVE-005. In the native application a `download` link and a print
// iframe do nothing (the shell has no download handler, and its print reaches
// only the main frame), so the report panel sends the canonical HTML to
// `save_rendered_report_html` (system save chooser, atomic writer) and to
// `print_rendered_report` (the print view in a window of its own, the system
// print dialog). The native side admits the HTML only when its SHA-256 is the
// one it recorded for the current render, reads the context and the block from
// that record and derives the print view itself
// (`src-tauri/src/rendered_report_routes.rs`, `rendered_report_print.rs`).
// Each request names the report source number its render was prepared for
// (`features/workspace/reportSourceSequence.ts`); the native side refuses one
// that is retired or not yet registered.
// Outside the native application the panel keeps its link and iframe.
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
  html: string;
  context: RenderedReportRouteContext;
  explicitLocalPrivateIntent: boolean;
  controlled: ControlledRouteExport | null;
  /** The report source number the render was prepared for, registered natively. */
  source: ReportSourceMark;
};

function routeArgs(request: RouteRequest): Record<string, unknown> {
  return {
    html: request.html,
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

// The two commands take the canonical HTML as a raw body under the ingress
// lease, as `render_calculation_report` does.
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
