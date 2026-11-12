// DEL-08-01 REQ-022: the report panel's Save canonical HTML and Print in the
// desktop runtime go to the native commands (`renderedReportRoutes.ts`), carry
// the canonical HTML and the controlled export's evidence, and show the
// outcome in words; a blocked render offers neither; a discarded render is
// retired natively; outside the desktop the link and the print iframe stay.
// The render path is mocked: these tests exercise only the two controls.
// All fixture data is invented. Mocked invoke only.
import { fireEvent, render, screen, waitFor } from "@testing-library/react";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import type { AnalysisRunEnvelope, MechanicsResult, PreviewModel } from "../../types";

const invokeMock = vi.hoisted(() => vi.fn());
const outcomeRef = vi.hoisted(() => ({ current: null as Record<string, unknown> | null }));
vi.mock("@tauri-apps/api/core", () => ({ invoke: invokeMock }));
vi.mock("./ReportComposer", () => ({ ReportComposer: () => null }));
vi.mock("../results/LoadReferenceOutputGate", () => ({
  LoadReferenceOutputGate: ({ children }: { children: unknown }) => children
}));
vi.mock("../results/outputPolicy", async (original) => ({
  ...(await original<typeof import("../results/outputPolicy")>()),
  surfaceOutputRefusal: () => null
}));
vi.mock("./renderableReportInput", async (original) => ({
  ...(await original<typeof import("./renderableReportInput")>()),
  buildUnitDisplaySummary: () => ({ model_units: {}, result_units: [], conversion_performed: false })
}));
vi.mock("./renderedReportAssembly", () => ({
  assembleRenderedReport: async ({ context }: { context: string }) => ({
    controlled: {
      payload: { report_context: { export_context: context } },
      decisions: [],
      findings: [],
      blocked: false,
      summary: {
        route_id: "DREP-IPC-003",
        decision_count: 0,
        finding_count: 0,
        warning_count: 0,
        blocking_count: 0,
        local_first: { explicit_local_private_intent: context === "local_private" }
      }
    },
    exactSubmission: null
  })
}));
vi.mock("../../services/reportRenderService", () => ({
  renderCalculationReport: async () => ({ route: "tauri_renderer", outcome: outcomeRef.current })
}));

import { RenderedReportPanel } from "./RenderedReportPanel";

const HTML = "<!DOCTYPE html>\n<html><body>invented rendered report</body></html>";
const SHA = "b".repeat(64);

function outcome(blocked = false) {
  return {
    html: HTML,
    sha256_hex: SHA,
    export_blocked: blocked,
    blocking_reasons: blocked ? ["pre-render lint: InventedBlock"] : [],
    report_validation_diagnostics: [],
    section_validation_diagnostics: [],
    pre_render_findings: [],
    post_render_findings: [],
    derived_print_html: "<div>DERIVED VIEW bbbb</div>"
  };
}

const model = { project: { id: "project:invented", units: {} } } as unknown as PreviewModel;
const analysisRun = { analysis_run: { analysis_status: [] } } as unknown as AnalysisRunEnvelope;
function result(id = "run:invented-1") {
  return { run_id: id, results: [], diagnostics: [] } as unknown as MechanicsResult;
}

function panel(source = result()) {
  return <RenderedReportPanel model={model} result={source} analysisRun={analysisRun} projectSummary={null} />;
}

/** The route command calls, with their raw ingress bodies decoded. */
function routeCalls(command: string): Record<string, unknown>[] {
  return invokeMock.mock.calls
    .filter(([name]) => name === command)
    .map(([, body, options]) => {
      expect(Object.prototype.toString.call(body)).toBe("[object Uint8Array]");
      expect(options).toEqual({ headers: { "x-swbpipe-ingress": "args-json-v1" } });
      return JSON.parse(new TextDecoder().decode(body as Uint8Array)) as Record<string, unknown>;
    });
}

async function renderShown(localPrivate = false) {
  const view = render(panel());
  if (localPrivate) fireEvent.click(screen.getByTestId("rendered-report-local-private-intent"));
  fireEvent.click(screen.getByTestId("rendered-report-render"));
  await waitFor(() => expect(screen.getByTestId("rendered-report-hash")).toBeInTheDocument());
  return view;
}

const SAVED = {
  outcome: "saved",
  route_id: "DREP-HTML-SAVE-005",
  export_context: "public_report",
  sha256_hex: SHA,
  byte_count: HTML.length,
  file_name: "invented-report.html",
  replaced_existing: false,
  durability: "durable"
};

beforeEach(() => {
  invokeMock.mockReset();
  outcomeRef.current = outcome();
});

afterEach(() => {
  delete (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__;
});

describe("native runtime", () => {
  beforeEach(() => {
    (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__ = {};
  });

  it("saves the canonical HTML through the native command and states the outcome", async () => {
    invokeMock.mockImplementation(async (command: string) => (command === "save_rendered_report_html" ? SAVED : undefined));
    await renderShown();
    const save = screen.getByTestId("rendered-report-save");
    expect(save.tagName).toBe("BUTTON");
    expect(save).toHaveAttribute("type", "button");
    expect(save).not.toHaveAttribute("href");
    expect(save).toBeEnabled();
    save.focus();
    expect(document.activeElement).toBe(save);
    expect(save.closest("[data-route-id]")).toHaveAttribute("data-route-id", "DREP-HTML-SAVE-005");
    expect(screen.getByTestId("rendered-report-save-redaction-summary")).toHaveTextContent("blocked=false");

    fireEvent.click(save);
    await waitFor(() => expect(screen.getByTestId("rendered-report-save-status")).toHaveTextContent("Saved invented-report.html"));
    expect(screen.getByTestId("rendered-report-save-status")).toHaveAttribute("role", "status");
    expect(screen.getByTestId("rendered-report-save-status")).toHaveTextContent(`SHA-256 ${SHA}`);
    expect(routeCalls("save_rendered_report_html")).toEqual([
      {
        html: HTML,
        exportContext: "public_report",
        explicitLocalPrivateIntent: false,
        evidence: { route_id: "DREP-HTML-SAVE-005", export_context: "public_report", explicit_local_private_intent: false, blocked: false, blocking_count: 0 }
      }
    ]);
    expect(screen.queryByTestId("rendered-report-print-frame")).not.toBeInTheDocument();
  });

  it("sends a local-private render with the panel's intent", async () => {
    invokeMock.mockImplementation(async (command: string) =>
      command === "save_rendered_report_html" ? { ...SAVED, export_context: "local_private" } : undefined
    );
    await renderShown(true);
    fireEvent.click(screen.getByTestId("rendered-report-save-local-private"));
    await waitFor(() => expect(routeCalls("save_rendered_report_html")).toHaveLength(1));
    expect(routeCalls("save_rendered_report_html")[0]).toMatchObject({
      exportContext: "local_private",
      explicitLocalPrivateIntent: true,
      evidence: { export_context: "local_private", explicit_local_private_intent: true }
    });
  });

  it("states a cancel, a refusal, an unconfirmed durability and a failed write in words", async () => {
    const answers: unknown[] = [
      { ...SAVED, outcome: "cancelled", durability: "not_applicable" },
      { ...SAVED, durability: "saved_durability_uncertain" }
    ];
    const rejections: unknown[] = [
      { code: "REPORT-ROUTE-NOT-CURRENT", stage: "admission", message: "No rendered report is current.", destination: "untouched", cleanup_code: null, selected_basename: null },
      { code: "REPORT-HTML-SAVE-RENAME-FAILED", stage: "rename", message: "injected", destination: "as_it_was", cleanup_code: null, selected_basename: "r.html" }
    ];
    invokeMock.mockImplementation(async (command: string) => {
      if (command !== "save_rendered_report_html") return undefined;
      if (answers.length) return answers.shift();
      throw rejections.shift();
    });
    await renderShown();
    const status = () => screen.getByTestId("rendered-report-save-status");
    const click = async (expected: string) => {
      await waitFor(() => expect(screen.getByTestId("rendered-report-save")).toBeEnabled());
      fireEvent.click(screen.getByTestId("rendered-report-save"));
      await waitFor(() => expect(status()).toHaveTextContent(expected));
    };
    await click("Save cancelled: nothing was written.");
    await click("did not confirm it is durable on disk");
    await click("Not saved (REPORT-ROUTE-NOT-CURRENT): No rendered report is current.");
    await click("The destination was left as it was.");
  });

  it("prints through the native command and mounts no print iframe", async () => {
    invokeMock.mockImplementation(async (command: string) =>
      command === "print_rendered_report"
        ? { outcome: "print_window_opened", route_id: "DREP-HTML-SAVE-005", export_context: "public_report", canonical_sha256_hex: SHA, print_sha256_hex: "c".repeat(64) }
        : undefined
    );
    await renderShown();
    fireEvent.click(screen.getByTestId("rendered-report-print"));
    await waitFor(() => expect(screen.getByTestId("rendered-report-print-status")).toHaveTextContent(`Print window opened with the derived print view of SHA-256 ${SHA}`));
    expect(routeCalls("print_rendered_report")).toEqual([
      {
        html: HTML,
        exportContext: "public_report",
        explicitLocalPrivateIntent: false,
        evidence: { route_id: "DREP-HTML-SAVE-005", export_context: "public_report", explicit_local_private_intent: false, blocked: false, blocking_count: 0 }
      }
    ]);
    expect(screen.queryByTestId("rendered-report-print-frame")).not.toBeInTheDocument();
  });

  it("states a refused print by name", async () => {
    invokeMock.mockImplementation(async (command: string) => {
      if (command === "print_rendered_report") {
        throw { code: "REPORT-PRINT-ALREADY-OPEN", stage: "print", message: "A print window is already open.", destination: "untouched", cleanup_code: null, selected_basename: null };
      }
      return undefined;
    });
    await renderShown();
    fireEvent.click(screen.getByTestId("rendered-report-print"));
    await waitFor(() => expect(screen.getByTestId("rendered-report-print-status")).toHaveTextContent("Not printed (REPORT-PRINT-ALREADY-OPEN)"));
  });

  it("offers neither route for a blocked render and sends nothing", async () => {
    outcomeRef.current = outcome(true);
    await renderShown();
    expect(screen.queryByTestId("rendered-report-save")).not.toBeInTheDocument();
    expect(screen.queryByTestId("rendered-report-print")).not.toBeInTheDocument();
    expect(screen.queryByTestId("rendered-report-save-status")).not.toBeInTheDocument();
    expect(invokeMock).not.toHaveBeenCalled();
  });

  it("retires the native render when its source is replaced, and shows no stale outcome", async () => {
    let resolveSave: (value: unknown) => void = () => undefined;
    invokeMock.mockImplementation((command: string) =>
      command === "save_rendered_report_html" ? new Promise((resolve) => { resolveSave = resolve; }) : Promise.resolve(undefined)
    );
    const view = await renderShown();
    fireEvent.click(screen.getByTestId("rendered-report-save"));
    await waitFor(() => expect(routeCalls("save_rendered_report_html")).toHaveLength(1));
    view.rerender(panel(result("run:invented-2")));
    expect(invokeMock.mock.calls.filter(([command]) => command === "retire_rendered_report")).toHaveLength(1);
    expect(screen.queryByTestId("rendered-report-save")).not.toBeInTheDocument();
    resolveSave(SAVED);
    await new Promise((resolve) => setTimeout(resolve, 0));
    expect(screen.queryByText(/Saved invented-report/)).not.toBeInTheDocument();
  });
});

describe("browser runtime (no native shell)", () => {
  it("keeps the controlled download link and the print iframe and calls no native route", async () => {
    await renderShown();
    const save = screen.getByTestId("rendered-report-save");
    expect(save.tagName).toBe("A");
    expect(save).toHaveAttribute("href", `data:text/html;charset=utf-8,${encodeURIComponent(HTML)}`);
    fireEvent.click(screen.getByTestId("rendered-report-print"));
    expect(screen.getByTestId("rendered-report-print-frame")).toHaveAttribute("srcdoc", outcome().derived_print_html);
    expect(screen.queryByTestId("rendered-report-save-status")).not.toBeInTheDocument();
    expect(invokeMock).not.toHaveBeenCalled();
  });
});
