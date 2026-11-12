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
// Outside the native application the panel keeps its link and iframe.
import { invokeWithIngress, IngressRequestRetiredError, type NativeIngressCommand } from "./nativeIngress";
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
 * or written) or `as_it_was` (the write failed before the file was published). */
export type RenderedReportRouteError = {
  code: string;
  stage: string;
  message: string;
  destination: "untouched" | "as_it_was";
  cleanup_code: string | null;
  selected_basename: string | null;
};

export type RenderedReportRouteOutcome<T> =
  | { kind: "done"; receipt: T }
  | { kind: "refused"; error: RenderedReportRouteError }
  | { kind: "retired" }
  | { kind: "failed"; message: string };

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

type RouteRequest = {
  html: string;
  context: RenderedReportRouteContext;
  explicitLocalPrivateIntent: boolean;
  controlled: ControlledRouteExport;
};

function routeArgs(request: RouteRequest): Record<string, unknown> {
  return {
    html: request.html,
    exportContext: request.context,
    explicitLocalPrivateIntent: request.explicitLocalPrivateIntent,
    evidence: {
      route_id: request.controlled.summary.route_id,
      export_context: request.context,
      explicit_local_private_intent: request.explicitLocalPrivateIntent,
      blocked: request.controlled.blocked,
      blocking_count: request.controlled.summary.blocking_count
    }
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
  if (!renderedReportRouteOpen(request.controlled, request.html)) {
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
    const receipt = await invokeWithIngress<T>(command as unknown as NativeIngressCommand, routeArgs(request), { stillCurrent });
    return { kind: "done", receipt };
  } catch (error) {
    if (error instanceof IngressRequestRetiredError) return { kind: "retired" };
    if (isRouteError(error)) return { kind: "refused", error };
    return { kind: "failed", message: error instanceof Error ? error.message : String(error) };
  }
}

export function saveRenderedReportHtml(request: RouteRequest, stillCurrent: () => boolean) {
  return send<RenderedReportHtmlSaveReceipt>("save_rendered_report_html", request, stillCurrent);
}

export function printRenderedReport(request: RouteRequest, stillCurrent: () => boolean) {
  return send<RenderedReportPrintReceipt>("print_rendered_report", request, stillCurrent);
}

/** The panel discards the render it shows: the native side retires its record
 * and closes an open print window. A failure only leaves the record to the
 * next render, which replaces it. */
export function retireRenderedReport(): void {
  void Promise.resolve()
    .then(() => invoke("retire_rendered_report"))
    .catch(() => undefined);
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
      return `Not saved (${outcome.error.code}): ${outcome.error.message}${destinationText(outcome.error)}`;
    case "retired":
      return "Not saved: the report is no longer current.";
    case "failed":
      return `Not saved: ${outcome.message}`;
  }
}

export function printOutcomeText(outcome: RenderedReportRouteOutcome<RenderedReportPrintReceipt>): string {
  switch (outcome.kind) {
    case "done":
      return `Print window opened with the derived print view of SHA-256 ${outcome.receipt.canonical_sha256_hex}; choose Print or Cancel in its dialog.`;
    case "refused":
      return `Not printed (${outcome.error.code}): ${outcome.error.message}`;
    case "retired":
      return "Not printed: the report is no longer current.";
    case "failed":
      return `Not printed: ${outcome.message}`;
  }
}

function destinationText(error: RenderedReportRouteError): string {
  if (error.destination !== "as_it_was") return "";
  return ` The destination was left as it was${error.cleanup_code ? `; a temporary file could not be removed (${error.cleanup_code})` : ""}.`;
}
