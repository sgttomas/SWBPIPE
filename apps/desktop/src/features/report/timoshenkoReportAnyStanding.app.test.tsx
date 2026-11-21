/** DEL-08-01 REQ-017 and REQ-024 through the application: a solved, readable pressure-free
 * Timoshenko result (`preview-physics-timoshenko-1`) reaches the Report page's rendered report
 * whatever its numerical standing, as an exact-route result does (`renderedReportGate.app.test.tsx`).
 *
 * - For an eligible result nothing a person sees changes: the rendered report and both package
 *   saves are offered, and the render request carries the `pf_timoshenko` section and the
 *   standing record the reader derived.
 * - For a result whose standing is not eligible, the rendered report, its print and its HTML save
 *   are offered, and the render request carries that result's standing record, from which the
 *   native render composes the standing line and each load case's numerical health, integrity code
 *   and reasons. Its report package and its shareable package are not offered (LIM-016): the panel
 *   says why, and so does each File menu package command when sent.
 *
 * The eligible result is the product's own committed Timoshenko envelope for its frozen request
 * (`core/product_physics/tests/pf_timoshenko_envelopes.rs` keeps it current). The not-eligible
 * result is the loads lane's produced witness of that same producer; until its path is set below,
 * that case does not run. Nothing here edits a standing record.
 *
 * Unit transport replay of committed producer bytes through mocked IPC; the render, print and
 * save commands' answers are unit simulations. NOT a native UI qualification witness. All data
 * are invented. */
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
import { REPORT_SOURCE_COMMANDS, reportSourceRegistration } from "../../test/reportSourceRegistration";
import { getLocalStorageCapability } from "../../services/projectService";
import { loadDesignKnowledge, resetSessionResultCustodyForTests } from "../../services/previewService";
import { PREVIEW_PHYSICS_TIMOSHENKO_CONTRACT_ID } from "../results/numericalResultQuality";
import { CURRENT_NUMERICAL_INTEGRITY_NEEDS_RECOMPUTE, N_REPORT_PACKAGE_PRESSURE_FREE_STANDING } from "./reportPackageRequest";
import type { PreviewModel } from "../../types";
import demoModel from "../../../../../fixtures/product_preview/invented_demo_model.json";

type Json = any; // eslint-disable-line @typescript-eslint/no-explicit-any
const root = resolve(__dirname, "../../../../../");

/** A solve's producer bytes and the model they were solved for (a solve request's `model`, or a
 * model document). */
type Source = { raw: string; model: string };
/** The product's own eligible Timoshenko result for its frozen request. */
const ELIGIBLE: Source = {
  raw: "fixtures/results/preview_physics_timoshenko_straight_sparse.json",
  model: "validation/references/pf_timoshenko/requests/PFT-SB-L1-CM0-TIMOSHENKO.request.json"
};
/** The loads lane's solved, readable Timoshenko witness whose standing is not eligible, from the
 * same producer (`pf_timoshenko_envelopes.rs`). Null until its path is given. */
const NOT_ELIGIBLE: Source | null = null;

useModelledVisibility();

afterEach(() => {
  cleanup();
  vi.restoreAllMocks();
  invokeMock.mockReset();
  resetSessionResultCustodyForTests();
  window.localStorage.clear();
  delete (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__;
});

/** The solved model, with only the display members the desktop shell needs and the solve request
 * omits added (none is read by the solver or the reader), as `renderedReportGate.app.test.tsx` does. */
function sourceModel(source: Source): PreviewModel {
  const document = JSON.parse(readFileSync(resolve(root, source.model), "utf8"));
  const model = structuredClone(document.model ?? document) as PreviewModel;
  for (const loadCase of model.load_cases) if (loadCase.label === undefined) Object.assign(loadCase, { label: loadCase.id, kind: "invented_test_case", status: "preview_only" });
  if (typeof model.project.name !== "string") Object.assign(model.project, { name: model.project.id });
  const record = model as unknown as Record<string, unknown>;
  if (record.diagnostics === undefined) record.diagnostics = [];
  if (record.data_boundary === undefined) record.data_boundary = structuredClone((demoModel as unknown as Record<string, unknown>).data_boundary);
  return model;
}

const HTML = "<!DOCTYPE html>\n<html><body>invented rendered report</body></html>";
const HTML_SHA256 = createHash("sha256").update(HTML).digest("hex");
function renderedOutcome() {
  return {
    html: HTML, sha256_hex: HTML_SHA256, export_blocked: false, blocking_reasons: [],
    report_validation_diagnostics: [], section_validation_diagnostics: [], pre_render_findings: [], post_render_findings: [],
    derived_print_html: "<div>invented derived view</div>"
  };
}

function nativeMenuCommand(command: string) {
  window.dispatchEvent(new CustomEvent("openpipestress-native-menu-command", { detail: command }));
}

async function openReportStage() {
  act(() => nativeMenuCommand("view.section.report"));
  return screen.findByTestId("rendered-report-panel");
}

let mountSequence = 0;
async function mountApp(source: Source) {
  const model = sourceModel(source);
  const raw = readFileSync(resolve(root, source.raw), "utf8");
  const [storage, knowledge] = [await getLocalStorageCapability(), await loadDesignKnowledge()];
  const transport = createSolveJobTransport(async () => raw, { jobIdPrefix: `invented-timoshenko-report-job:${++mountSequence}` });
  const renders: Json[] = [];
  const prints: Json[] = [];
  const htmlSaves: Json[] = [];
  const packageSaves: Json[] = [];
  const reportSource = reportSourceRegistration();
  const refused: string[] = [];
  const jobCommands = (): string[] => transport.commands().map((c: Json) => (typeof c === "string" ? c : c.command));
  invokeMock.mockImplementation(async (command: string, args?: unknown, options?: unknown) => {
    if (command === "get_local_storage_capability") return storage;
    if (command === "load_design_knowledge") return knowledge;
    if (command === "sync_native_shell_state") return null;
    if (command === "load_preview_model") return structuredClone(model);
    if (command === "render_calculation_report") { renders.push(args); return renderedOutcome(); }
    if (command === "print_rendered_report") {
      prints.push(args);
      return { outcome: "print_window_opened", route_id: "DREP-HTML-SAVE-005", export_context: "local_private", canonical_sha256_hex: HTML_SHA256, print_sha256_hex: "c".repeat(64) };
    }
    if (command === "save_rendered_report_html") {
      htmlSaves.push(args);
      return { outcome: "saved", route_id: "DREP-HTML-SAVE-005", export_context: "local_private", sha256_hex: HTML_SHA256, byte_count: HTML.length, file_name: "invented-timoshenko-report.html", replaced_existing: false, durability: "durable" };
    }
    if (command === "retire_rendered_report") return null;
    if (command === "save_report_package") { packageSaves.push(args); throw new Error("INVENTED-SAVE-NOT-WRITTEN: this unit transport writes no package"); }
    if (REPORT_SOURCE_COMMANDS.includes(command)) return reportSource(command, args);
    if (!REPLAY_COMMANDS.includes(command) && !transport.handles(command)) { refused.push(command); throw new Error(`TIMOSHENKO_REPORT_COMMAND_UNSUPPORTED: ${command}`); }
    return transport.invoke(command, args, options);
  });
  (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__ = {};
  render(<App />);
  await screen.findByTestId("desktop-preview-shell");
  await screen.findByTestId(`tree-row-project-${encodeURIComponent(model.project.id)}`);
  const why = () => `refused commands: ${refused.join(", ")}; job commands: ${jobCommands().join(", ")}; alerts: ${[...document.querySelectorAll('[role="alert"], [data-testid*="error"], [data-testid*="refus"], [data-testid*="unavailable"]')].map((e) => `${e.getAttribute("data-testid") ?? "alert"}=${(e.textContent ?? "").slice(0, 300)}`).join(" | ")}`;
  return { raw, renders, prints, htmlSaves, packageSaves, jobCommands, why };
}

/** Solves, opens Report and waits until the solved result is in the rendered report panel. */
async function solveAndOpenReport(app: Awaited<ReturnType<typeof mountApp>>) {
  act(() => nativeMenuCommand("analyze.run"));
  await waitFor(() => expect(app.jobCommands(), app.why()).toContain("read_preview_mechanics_job_analysis_run_preview"), { timeout: 60_000 });
  await openReportStage();
  await waitFor(() => {
    expect(screen.queryByTestId("c2-result-failed")?.textContent ?? null, app.why()).toBeNull();
    const panel = screen.getByTestId("rendered-report-panel");
    expect(within(panel).queryByTestId("rendered-report-precondition"), app.why()).toBeNull();
    expect(within(panel).getByTestId("rendered-report-render"), app.why()).toBeEnabled();
  }, { timeout: 60_000 });
}

/** Renders for local review and returns the render request the panel sent. */
async function renderReport(app: Awaited<ReturnType<typeof mountApp>>) {
  const panel = () => screen.getByTestId("rendered-report-panel");
  fireEvent.click(within(panel()).getByTestId("rendered-report-local-private-intent"));
  fireEvent.click(within(panel()).getByTestId("rendered-report-render"));
  await waitFor(() => {
    expect(within(panel()).queryByTestId("rendered-report-error")?.textContent ?? null, app.why()).toBeNull();
    expect(app.renders.length, app.why()).toBe(1);
  }, { timeout: 60_000 });
  await waitFor(() => expect(within(panel()).getByTestId("rendered-report-preview")).toBeInTheDocument());
  return app.renders[0];
}

describe("the rendered report of a solved pressure-free Timoshenko result, through App's gate", () => {
  it("for an eligible result: the rendered report and both package saves are offered, as before", async () => {
    const app = await mountApp(ELIGIBLE);
    await solveAndOpenReport(app);
    const panel = screen.getByTestId("rendered-report-panel");
    for (const id of ["report-package-save", "report-package-user-release"]) {
      expect(within(panel).getByTestId(id), app.why()).toBeEnabled();
      expect(within(panel).getByTestId(id).getAttribute("title")).toBeNull();
    }
    expect(screen.queryByTestId("report-package-source-unavailable")).toBeNull();
    const sent = await renderReport(app);
    expect(sent.exactResultText).toBe(app.raw);
    expect(sent.input.pf_timoshenko.semantic_contract_id).toBe(PREVIEW_PHYSICS_TIMOSHENKO_CONTRACT_ID);
    expect(sent.input.numerical_standing.cases.map((c: Json) => c.health)).toEqual(sent.input.numerical_standing.cases.map(() => "checks_passed"));
  }, 240_000);

  it.skipIf(NOT_ELIGIBLE === null)("for a result that is not eligible: the rendered report, its print and its HTML save are offered with its standing record; its packages are not, and each says why", async () => {
    const app = await mountApp(NOT_ELIGIBLE!);
    await solveAndOpenReport(app);
    const panel = () => screen.getByTestId("rendered-report-panel");
    // The package controls are not offered, and the page says why in words.
    expect(within(panel()).queryByTestId("report-package-controls")).toBeNull();
    expect(screen.getByTestId("report-package-source-unavailable")).toHaveTextContent(`Report package not offered: ${N_REPORT_PACKAGE_PRESSURE_FREE_STANDING}`);
    // The result JSON export, which needs numerical eligibility, is still not offered.
    act(() => nativeMenuCommand("view.section.exports"));
    const exports = await screen.findByTestId("result-export-panel");
    expect(within(exports).queryByTestId("result-export-link")).toBeNull();
    await openReportStage();

    // The render request carries the reader's standing record for this result, and the section the
    // native render binds to the delivered result.
    const sent = await renderReport(app);
    expect(sent.exactResultText).toBe(app.raw);
    expect(sent.input.pf_timoshenko.semantic_contract_id).toBe(PREVIEW_PHYSICS_TIMOSHENKO_CONTRACT_ID);
    const record = sent.input.numerical_standing;
    expect(record.record_version).toBe("historical_1");
    expect(record.cases.some((c: Json) => c.health !== "checks_passed"), JSON.stringify(record.cases)).toBe(true);

    // Its print and its HTML save are offered and reach their native routes.
    fireEvent.click(within(panel()).getByTestId("rendered-report-print"));
    await waitFor(() => expect(within(panel()).getByTestId("rendered-report-print-status")).toHaveTextContent("Print window opened"));
    fireEvent.click(within(panel()).getByTestId("rendered-report-save-local-private"));
    await waitFor(() => expect(within(panel()).getByTestId("rendered-report-save-status")).toHaveTextContent("Saved invented-timoshenko-report.html"));
    expect(app.prints.map((p: Json) => p.sha256Hex)).toEqual([HTML_SHA256]);
    expect(app.htmlSaves.map((s: Json) => s.sha256Hex)).toEqual([HTML_SHA256]);

    // The File menu's two package commands, which the native menu keeps selectable, each refuse in words.
    act(() => nativeMenuCommand("file.save-report-package"));
    await waitFor(() => expect(within(panel()).getByTestId("report-package-save-status"), app.why()).toHaveTextContent(
      `${CURRENT_NUMERICAL_INTEGRITY_NEEDS_RECOMPUTE}: ${N_REPORT_PACKAGE_PRESSURE_FREE_STANDING}`
    ), { timeout: 60_000 });
    act(() => nativeMenuCommand("file.save-shareable-report-package"));
    const dialog = await screen.findByTestId("user-release-dialog", undefined, { timeout: 60_000 });
    expect(within(dialog).getByTestId("user-release-refusal-reason")).toHaveTextContent(
      `this result is ${PREVIEW_PHYSICS_TIMOSHENKO_CONTRACT_ID}. It can be saved in a local private package when its numerical standing is eligible.`
    );
    expect(app.packageSaves).toHaveLength(0);
  }, 240_000);
});
