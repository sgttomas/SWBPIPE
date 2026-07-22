import { invoke } from "@tauri-apps/api/core";

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
};

export type RenderReportRoute =
  | { route: "tauri_renderer"; outcome: RenderedReportOutcome }
  | { route: "unavailable_browser_preview"; diagnostic: string }
  | { route: "redaction_blocked"; diagnostic: string };

function isTauriRuntime(): boolean {
  return typeof window !== "undefined" && "__TAURI_INTERNALS__" in window;
}

export async function renderCalculationReport(input: unknown): Promise<RenderReportRoute> {
  if (!isControlledReportInput(input)) {
    return {
      route: "redaction_blocked",
      diagnostic:
        "REPORT-REDACTION-CONTROL-REQUIRED: renderer input must be a non-blocked DREP-IPC-003 controlled result."
    };
  }
  if (input.blocked || input.payload === null) {
    return {
      route: "redaction_blocked",
      diagnostic: `REPORT-REDACTION-BLOCKED: ${input.findings.length} finding(s) prevent renderer exposure.`
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
  const outcome = await invoke<RenderedReportOutcome>("render_calculation_report", { input: input.payload });
  return { route: "tauri_renderer", outcome };
}

function isControlledReportInput(value: unknown): value is {
  payload: unknown;
  findings: unknown[];
  blocked: boolean;
  summary: { route_id: string };
} {
  if (typeof value !== "object" || value === null) return false;
  const candidate = value as Record<string, unknown>;
  const summary = candidate.summary as Record<string, unknown> | undefined;
  return (
    "payload" in candidate &&
    Array.isArray(candidate.findings) &&
    typeof candidate.blocked === "boolean" &&
    summary?.route_id === "DREP-IPC-003"
  );
}
