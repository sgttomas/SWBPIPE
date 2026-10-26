import { IngressRequestRetiredError, invokeWithIngress } from "./nativeIngress";

// DEC-021 (A7): the hash-bound report renderer is a Rust crate reached
// through the desktop (Tauri) command seam. The browser preview exposes no
// rendering seam and there is no fallback renderer — browser mode reports an
// explicit unavailable route instead of silently substituting.

export type RenderOutcomeDiagnostic = {
  code: string;
  severity: string;
  message: string;
};

export type RenderOutcomeFinding = {
  finding_id: string;
  code: string;
  severity: string;
  path: string;
  line: number;
  column: number;
  excerpt: string;
  message: string;
  blocking: boolean;
};

export type RenderedReportOutcome = {
  html: string;
  sha256_hex: string;
  export_blocked: boolean;
  blocking_reasons: string[];
  report_validation_diagnostics: RenderOutcomeDiagnostic[];
  section_validation_diagnostics: RenderOutcomeDiagnostic[];
  pre_render_findings: RenderOutcomeFinding[];
  post_render_findings: RenderOutcomeFinding[];
  derived_print_html: string;
  /** W-11 slice (c): present and true only for a render refused for its provenance records,
   * which is metadata-only (empty `html` and `derived_print_html`, no findings). */
  content_withheld?: boolean;
};

/** W-11 (DEL-08-01 Design `exact-route-report-sections.md` §3.6): the texts an exact
 * render resubmits beside its input: the bytes the solve job delivered, the model text it
 * was sent and the admission's input manifest text. The native host binds them to the
 * digests it recorded for the job. A pressure-free render carries none. The native render
 * refuses every exact identity at its step 0 until the owner opens exact-route reports. */
export type ExactRenderSubmission = { exactResultText: string; exactIntakeText: string; inputManifestText: string };

export type RenderReportRoute =
  | { route: "tauri_renderer"; outcome: RenderedReportOutcome }
  | { route: "unavailable_browser_preview"; diagnostic: string }
  | { route: "redaction_blocked"; diagnostic: string };

function isTauriRuntime(): boolean {
  return typeof window !== "undefined" && "__TAURI_INTERNALS__" in window;
}

export async function renderCalculationReport(
  input: unknown,
  options: { stillCurrent?: () => boolean; exactSubmission?: ExactRenderSubmission | null } = {}
): Promise<RenderReportRoute> {
  if (!isControlledReportInput(input)) {
    return {
      route: "redaction_blocked",
      diagnostic:
        "REPORT-REDACTION-CONTROL-REQUIRED: renderer input must be a non-blocked DREP-IPC-003 controlled result with route evidence."
    };
  }
  if (input.blocked || input.payload === null) {
    return {
      route: "redaction_blocked",
      diagnostic: `REPORT-REDACTION-BLOCKED: ${input.findings.length} finding(s) prevent renderer exposure.`
    };
  }
  if (!controlledContextConsistent(input)) {
    return {
      route: "redaction_blocked",
      diagnostic:
        "REPORT-REDACTION-CONTEXT-MISMATCH: the controlled route context, explicit intent and the report's recorded export context differ."
    };
  }
  if (!isTauriRuntime()) {
    return {
      route: "unavailable_browser_preview",
      diagnostic:
        "REPORT-RENDERER-DESKTOP-ONLY: the hash-bound calculation-report renderer runs in the " +
        "desktop (Tauri) runtime only; the browser preview exposes no rendering seam and no " +
        "fallback renderer exists."
    };
  }
  // The caller's request may have been retired (panel unmounted, source
  // replaced, selection changed); a retired request never reaches IPC, on the
  // first send or on any re-send after a headroom refusal.
  const retired: RenderReportRoute = {
    route: "redaction_blocked",
    diagnostic: "REPORT-RENDER-REQUEST-RETIRED: the render request is no longer current."
  };
  if (options.stillCurrent && !options.stillCurrent()) return retired;
  try {
    const outcome = await invokeWithIngress<RenderedReportOutcome>(
      "render_calculation_report",
      options.exactSubmission ? { input: input.payload, ...options.exactSubmission } : { input: input.payload },
      { stillCurrent: options.stillCurrent }
    );
    return { route: "tauri_renderer", outcome };
  } catch (error) {
    if (error instanceof IngressRequestRetiredError) return retired;
    throw error;
  }
}

function isControlledReportInput(value: unknown): value is {
  payload: unknown;
  findings: unknown[];
  blocked: boolean;
  summary: { route_id: string; local_first: Record<string, unknown> };
} {
  if (typeof value !== "object" || value === null) return false;
  const candidate = value as Record<string, unknown>;
  const summary = candidate.summary as Record<string, unknown> | undefined;
  return (
    "payload" in candidate &&
    Array.isArray(candidate.findings) &&
    typeof candidate.blocked === "boolean" &&
    summary?.route_id === "DREP-IPC-003" &&
    typeof summary.local_first === "object" &&
    summary.local_first !== null
  );
}

/** The controlled envelope's route evidence and the context recorded in the
 * renderer input must agree: a local-private render needs the local-private
 * context with explicit wrapper intent, and a public render must not carry a
 * local-private record (or the reverse). */
function controlledContextConsistent(input: {
  payload: unknown;
  summary: { local_first: Record<string, unknown> };
}): boolean {
  const evidence = input.summary.local_first;
  if (evidence.route_id !== "DREP-IPC-003" || evidence.blocked !== false) return false;
  const payload = input.payload as Record<string, unknown> | null;
  const reportContext = payload?.report_context as Record<string, unknown> | null | undefined;
  const recorded = reportContext?.export_context;
  if (evidence.export_context === "local_private") {
    return evidence.explicit_local_private_intent === true && recorded === "local_private";
  }
  if (evidence.export_context === "public_report") {
    return recorded === "public_report";
  }
  return false;
}
