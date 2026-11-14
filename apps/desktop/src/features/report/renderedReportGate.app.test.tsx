/** DEL-08-01 REQ-017 and REQ-024 through the application, not around it (PR 328's text
 * review, finding 1: the component witnesses did not establish the desktop path). The
 * whole App renders; a native solve of the committed coupled-friction witness, a sensitive
 * pressure-3 result that is not numerically eligible, completes; the Report stage's
 * rendered report panel then takes it through App's own gate (the session's
 * renderableSolvedResult), and its render request carries the reader's standing record,
 * from which the native render composes the standing line (shown by the Rust
 * connecting-path test `a_readable_solved_result_that_is_not_checks_passed_renders_with_its_standing`
 * on the render input this build step makes). The rendered document is shown and its print
 * view reached. The report package stays refused (no package controls are offered and the
 * menu's package command is disabled), and so do the result JSON export and the rule check. A replacement (a fresh solve) retires the shown report.
 *
 * Not made here, because App's UI offers no way to make them: a received copy of the result
 * and another run's record. Those removals are tested on the session state, where both
 * Current and the rendered report's source are computed
 * (`loadReference.resultsSessionState.test.ts`).
 *
 * Unit transport replay of committed producer bytes through mocked IPC; the render command's
 * answer is a unit simulation. NOT a native UI qualification witness. All data are invented. */
import { act, cleanup, fireEvent, render, screen, waitFor, within } from "@testing-library/react";
import { createHash } from "node:crypto";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { afterEach, describe, expect, it, vi } from "vitest";
const invokeMock = vi.hoisted(() => vi.fn());
vi.mock("@tauri-apps/api/core", async () => ({ invoke: (await import("../../test/nativeIngress")).ingressDecodingInvoke(invokeMock) }));
import { App } from "../../App";
import { createSolveJobTransport, REPLAY_COMMANDS } from "../../test/nativeMechanicsReplay";
import { useModelledVisibility } from "../../test-support/modelledVisibility";
import { getLocalStorageCapability } from "../../services/projectService";
import { loadDesignKnowledge, resetSessionResultCustodyForTests } from "../../services/previewService";
import type { PreviewModel } from "../../types";

type Json = any; // eslint-disable-line @typescript-eslint/no-explicit-any
const root = resolve(__dirname, "../../../../../");
const DIR = "fixtures/reports/invented/report_standing_witnesses";
const RAW = `${DIR}/pressure_3_coupled_friction_sensitive-sparse_interactive.raw.json`;
const REQUEST = `${DIR}/coupled_friction.request.json`;

// jsdom does no layout: the shell's `.inactive` sections are modelled as hidden.
useModelledVisibility();

afterEach(() => {
  cleanup();
  vi.restoreAllMocks();
  invokeMock.mockReset();
  resetSessionResultCustodyForTests();
  window.localStorage.clear();
  delete (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__;
});

function witnessModel(): PreviewModel {
  const model = structuredClone(JSON.parse(readFileSync(resolve(root, REQUEST), "utf8")).model) as PreviewModel;
  for (const loadCase of model.load_cases) if (loadCase.label === undefined) Object.assign(loadCase, { label: loadCase.id, kind: "invented_test_case", status: "invented_test_status" });
  if (typeof model.project.name !== "string") Object.assign(model.project, { name: "Invented report-standing witness pressure_3_coupled_friction_sensitive" });
  return model;
}

const HTML = "<!DOCTYPE html>\n<html><body>invented rendered report</body></html>";
function renderedOutcome() {
  return {
    html: HTML, sha256_hex: createHash("sha256").update(HTML).digest("hex"), export_blocked: false, blocking_reasons: [],
    report_validation_diagnostics: [], section_validation_diagnostics: [], pre_render_findings: [], post_render_findings: [],
    derived_print_html: "<div>invented derived view</div>"
  };
}

function nativeMenuCommand(command: string) {
  window.dispatchEvent(new CustomEvent("openpipestress-native-menu-command", { detail: command }));
}

async function openReportStage() {
  fireEvent.click(await screen.findByTestId("rail-stage-review"));
  fireEvent.click(await screen.findByTestId("stage-tab-report"));
  return screen.findByTestId("rendered-report-panel");
}

describe("the rendered report of a solved exact-route result that is not numerically eligible, through App's gate", () => {
  it("is offered with its standing record; the package stays refused; a stale result is taken away again", async () => {
    const model = witnessModel();
    const [storage, knowledge] = [await getLocalStorageCapability(), await loadDesignKnowledge()];
    const raw = readFileSync(resolve(root, RAW), "utf8");
    const transport = createSolveJobTransport(() => raw, { jobIdPrefix: "invented-rendered-report-gate-job" });
    const renders: Json[] = [];
    invokeMock.mockImplementation(async (command: string, args?: unknown, options?: unknown) => {
      if (command === "get_local_storage_capability") return storage;
      if (command === "load_design_knowledge") return knowledge;
      if (command === "sync_native_shell_state") return null;
      if (command === "load_preview_model") {
        if (args !== undefined) throw new Error("RENDERED_REPORT_GATE_REQUEST_INVALID");
        return structuredClone(model);
      }
      if (command === "render_calculation_report") { renders.push(args); return renderedOutcome(); }
      // Every other command is the native mechanics job contract, or refused by name.
      if (!REPLAY_COMMANDS.includes(command)) throw new Error(`RENDERED_REPORT_GATE_COMMAND_UNSUPPORTED: ${command}`);
      return transport.invoke(command, args, options);
    });
    (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__ = {};
    render(<App />);
    await screen.findByTestId("desktop-preview-shell");
    await screen.findByTestId("workspace-toolbar");
    await screen.findByTestId(`tree-row-project-${encodeURIComponent(model.project.id)}`);

    // Before any solve the panel has no result.
    let panel = await openReportStage();
    expect(within(panel).getByTestId("rendered-report-precondition")).toBeInTheDocument();

    act(() => nativeMenuCommand("analyze.run"));
    // The solved result reaches the panel through the gate (its full copy read for it).
    await waitFor(() => expect(within(screen.getByTestId("rendered-report-panel")).queryByTestId("rendered-report-precondition")).toBeNull(), { timeout: 30_000 });
    panel = screen.getByTestId("rendered-report-panel");
    expect(within(panel).getByTestId("rendered-report-render")).toBeEnabled();
    // The report package is not offered for it, and the menu's command is disabled.
    expect(within(panel).queryByTestId("report-package-controls")).toBeNull();
    fireEvent.click(screen.getByTestId("menu-file"));
    expect(await screen.findByTestId("menu-item-file.save-report-package")).toBeDisabled();
    fireEvent.keyDown(document.activeElement ?? document.body, { key: "Escape" });
    // The result JSON export and the rule check, which need numerical eligibility, are not offered.
    act(() => nativeMenuCommand("view.section.exports"));
    const exports = await screen.findByTestId("result-export-panel");
    expect(within(exports).getByTestId("result-export-empty")).toBeInTheDocument();
    expect(within(exports).queryByTestId("result-export-link")).toBeNull();
    act(() => nativeMenuCommand("view.section.solve"));
    expect(await screen.findByTestId("rule-check-run")).toBeDisabled();
    panel = await openReportStage();

    // The render request carries the reader's standing record for this sensitive result.
    fireEvent.click(within(panel).getByTestId("rendered-report-render"));
    await waitFor(() => expect(renders.length).toBe(1), { timeout: 30_000 });
    const record = renders[0].input.numerical_standing;
    expect(record.record_version).toBe("historical_1");
    expect(record.cases.map((c: Json) => c.health)).toEqual(["sensitive"]);
    expect(record.cases[0].confirmation.outcome).toBe("solver_reported_established");
    expect(record.cases[0].confirmation.status).toBe("conditional");
    expect(renders[0].exactResultText).toBe(raw);
    // The rendered report is shown, and its print view is reached (the derived view's frame).
    await waitFor(() => expect(within(screen.getByTestId("rendered-report-panel")).getByTestId("rendered-report-preview")).toBeInTheDocument());
    fireEvent.click(within(screen.getByTestId("rendered-report-panel")).getByTestId("rendered-report-print"));
    expect(within(screen.getByTestId("rendered-report-panel")).getByTestId("rendered-report-print-frame")).toBeInTheDocument();

    // A replacement (a fresh solve) retires the shown report: the panel shows no rendered
    // document until it is rendered again for the new result.
    act(() => nativeMenuCommand("analyze.run"));
    await waitFor(() => expect(within(screen.getByTestId("rendered-report-panel")).queryByTestId("rendered-report-preview")).toBeNull(), { timeout: 30_000 });
    expect(within(screen.getByTestId("rendered-report-panel")).queryByTestId("rendered-report-print-frame")).toBeNull();
  });
});
