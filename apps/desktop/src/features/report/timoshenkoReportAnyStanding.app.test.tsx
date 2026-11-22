/** DEL-08-01 REQ-017 and REQ-024 through the application: a solved, readable pressure-free
 * Timoshenko result (`preview-physics-timoshenko-1`) reaches the Report page's rendered report
 * whatever its numerical standing, as an exact-route result does (`renderedReportGate.app.test.tsx`).
 *
 * - For an eligible and for a not-eligible result, the rendered report, its print and its HTML save
 *   are offered, and the render request carries the `pf_timoshenko` section, exactly the source
 *   rows' projection and the standing record the reader derived, from which the native render
 *   composes the standing line and each load case's numerical health, integrity code and reasons.
 * - Each of the four package commands (the panel's two save buttons, the File menu's two) offers
 *   the save and makes its request, whose report carries the same standing record, the identity's
 *   limitation 0 and no `pf_timoshenko` section (DEL-08-01 REQ-024, "Package and user release"); the
 *   shareable request carries record 1.3.0, and its dialog states the report frame's statements as
 *   the native host gives them, and that the report states the result's standing, before the
 *   user's statement. The native save, which derives the record again and requires it equal, is
 *   not run here (`report_session_diagnostics_tests.rs`, `user_release_package.rs`).
 *
 * The eligible result is the product's own committed Timoshenko envelope for its frozen request
 * (`core/product_physics/tests/pf_timoshenko_envelopes.rs` keeps it current). The not-eligible
 * result is the product's sensitive torsion result of that same producer, which the same test keeps
 * current; its render request is committed and rendered natively by
 * `report_render_pf_timoshenko_tests.rs`, which reads the standing in the report's words. Nothing
 * here edits a standing record.
 *
 * Unit transport replay of committed producer bytes through mocked IPC; the render, print and
 * save commands' answers are unit simulations. NOT a native UI qualification witness. All data
 * are invented. */
import { act, cleanup, fireEvent, render, screen, waitFor, within } from "@testing-library/react";
import { createHash } from "node:crypto";
import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
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
import { USER_RELEASE_PF_TIMOSHENKO_RECORD_VERSION } from "./reportPackageRequest";
import type { PreviewModel } from "../../types";
import demoModel from "../../../../../fixtures/product_preview/invented_demo_model.json";

type Json = any; // eslint-disable-line @typescript-eslint/no-explicit-any
const root = resolve(__dirname, "../../../../../");

/** A solve's producer bytes and the model they were solved for (a solve request's `model`, or a
 * model document, with `shearModel` selected where given). */
type Source = { raw: string; model: string; shearModel?: string };
/** The product's own eligible Timoshenko result for its frozen request. */
const ELIGIBLE: Source = {
  raw: "fixtures/results/preview_physics_timoshenko_straight_sparse.json",
  model: "validation/references/pf_timoshenko/requests/PFT-SB-L1-CM0-TIMOSHENKO.request.json"
};
/** The product's solved, readable Timoshenko result whose standing is not eligible: the committed
 * numerically sensitive torsion model with Timoshenko straight members selected, its one load case
 * sensitive (`pf_timoshenko_envelopes.rs` keeps the bytes current). */
const NOT_ELIGIBLE: Source = {
  raw: "fixtures/results/preview_physics_timoshenko_sensitive_torsion_sparse.json",
  model: "fixtures/product_preview/numerical_sensitive_torsion_model.json",
  shearModel: "timoshenko_annulus_energy_v1"
};
/** The not-eligible result's render request as this App sends it, committed (stale fails;
 * `OPENPIPESTRESS_WRITE_PF_TIMOSHENKO_APP_RENDER_REQUEST=<dir>` writes it) and rendered through the
 * native production render command with live custody by `report_render_pf_timoshenko_tests.rs`
 * (`the_app_render_request_of_a_not_eligible_timoshenko_result_states_its_standing`), which reads
 * the words a person sees: the standing line and each load case's numerical health. */
const APP_RENDER_REQUEST = "fixtures/reports/invented/pf_timoshenko_not_eligible/sensitive_torsion.app_render_request.json";
const WRITE_APP_RENDER_REQUEST = "OPENPIPESTRESS_WRITE_PF_TIMOSHENKO_APP_RENDER_REQUEST";
/** The render's clock, pinned so the request's generated_at is reproducible. */
const PINNED_RENDER_TIME = new Date("2026-11-21T12:00:00.000Z");

useModelledVisibility();

afterEach(() => {
  cleanup();
  vi.useRealTimers();
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
  if (source.shearModel) model.model_settings = { ...(model.model_settings ?? {}), shear_model: source.shearModel };
  for (const loadCase of model.load_cases) if (loadCase.label === undefined) Object.assign(loadCase, { label: loadCase.id, kind: "invented_test_case", status: "preview_only" });
  // A model document's case can carry its label without the status the input manifest's load basis needs.
  for (const loadCase of model.load_cases) if (loadCase.status === undefined) Object.assign(loadCase, { status: "preview_only" });
  if (typeof model.project.name !== "string") Object.assign(model.project, { name: model.project.id });
  const record = model as unknown as Record<string, unknown>;
  if (record.diagnostics === undefined) record.diagnostics = [];
  // The model index reads a component list; a model document without one has none.
  if (record.components === undefined) record.components = [];
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

/** The four package commands' requests, after the render whose request `sent` is: each carries
 * `sent`'s standing record and the limitation 0 entry, and no `pf_timoshenko` section. */
async function expectFourPackageRequests(app: Awaited<ReturnType<typeof mountApp>>, sent: Json) {
  const panel = () => screen.getByTestId("rendered-report-panel");
  const record = sent.input.numerical_standing;
  const saved = async (count: number) => {
    await waitFor(() => expect(app.packageSaves.length, app.why()).toBe(count), { timeout: 60_000 });
    await waitFor(() => expect(within(panel()).getByTestId("report-package-save")).toBeEnabled(), { timeout: 60_000 });
    return JSON.parse(app.packageSaves[count - 1].requestCanonicalText);
  };
  const expectReport = (request: Json) => {
    expect(request.report.numerical_standing, app.why()).toEqual(record);
    expect("pf_timoshenko" in request.report).toBe(false);
    const runId = request.source_results[0].run_id;
    expect(request.report.report_sections.limitations.map((item: Json) => item.limitation_id)).toEqual([`limitation:${runId}:technical-preview`, `limitation:${runId}:pf-timoshenko:1`]);
    expect(request.source_results[0].producer.semantic_contract_id).toBe(PREVIEW_PHYSICS_TIMOSHENKO_CONTRACT_ID);
  };
  fireEvent.click(within(panel()).getByTestId("report-package-private-intent"));
  fireEvent.click(within(panel()).getByTestId("report-package-save"));
  for (const [count, command] of [[1, null], [2, "file.save-report-package"]] as const) {
    if (command) act(() => nativeMenuCommand(command));
    const request = await saved(count);
    expect(request.export_profile_id, app.why()).toBe("desktop_local_private_report_package_1");
    expectReport(request);
  }
  for (const [count, open] of [[3, () => fireEvent.click(within(panel()).getByTestId("report-package-user-release"))], [4, () => act(() => nativeMenuCommand("file.save-shareable-report-package"))]] as const) {
    open();
    const dialog = await screen.findByTestId("user-release-dialog", undefined, { timeout: 60_000 });
    expect(within(dialog).queryByTestId("user-release-refusal"), app.why()).toBeNull();
    expect(within(dialog).getByTestId("user-release-terms")).toHaveTextContent("The report states that result's numerical standing, whatever it is.");
    const statement = within(dialog).getByTestId("user-release-statement");
    expect(within(dialog).getByTestId("user-release-confirm")).toBeDisabled();
    fireEvent.click(statement);
    fireEvent.click(within(dialog).getByTestId("user-release-confirm"));
    const request = await saved(count);
    expect(request.export_profile_id, app.why()).toBe("desktop_user_released_report_package_1");
    expect(request.report.report_context.user_release.record_version).toBe(USER_RELEASE_PF_TIMOSHENKO_RECORD_VERSION);
    expectReport(request);
  }
  expect(app.packageSaves).toHaveLength(4);
}

describe("the rendered report and the packages of a solved pressure-free Timoshenko result, through App's gate", () => {
  it("for an eligible result: the rendered report, its print, its HTML save and each of the four package commands state its standing", async () => {
    const app = await mountApp(ELIGIBLE);
    await solveAndOpenReport(app);
    const panel = () => screen.getByTestId("rendered-report-panel");
    for (const id of ["report-package-save", "report-package-user-release"]) {
      expect(within(panel()).getByTestId(id), app.why()).toBeEnabled();
      expect(within(panel()).getByTestId(id).getAttribute("title")).toBeNull();
    }
    expect(screen.queryByTestId("report-package-source-unavailable")).toBeNull();
    const sent = await renderReport(app);
    expect(sent.exactResultText).toBe(app.raw);
    expect(sent.input.pf_timoshenko.semantic_contract_id).toBe(PREVIEW_PHYSICS_TIMOSHENKO_CONTRACT_ID);
    expect(sent.input.numerical_standing.cases.map((c: Json) => c.health)).toEqual(sent.input.numerical_standing.cases.map(() => "checks_passed"));
    // The rows are the source rows' projection, which the native render requires: no generic
    // numerical-health row (each load case's health is stated from the standing record).
    expect((sent.input.result_rows as Json[]).filter((row) => String(row.row_id).startsWith("numerical-health:"))).toEqual([]);

    // Its print and its HTML save reach their native routes.
    fireEvent.click(within(panel()).getByTestId("rendered-report-print"));
    await waitFor(() => expect(within(panel()).getByTestId("rendered-report-print-status")).toHaveTextContent("Print window opened"));
    fireEvent.click(within(panel()).getByTestId("rendered-report-save-local-private"));
    await waitFor(() => expect(within(panel()).getByTestId("rendered-report-save-status")).toHaveTextContent("Saved invented-timoshenko-report.html"));
    expect(app.prints.map((p: Json) => p.sha256Hex)).toEqual([HTML_SHA256]);
    expect(app.htmlSaves.map((s: Json) => s.sha256Hex)).toEqual([HTML_SHA256]);

    await expectFourPackageRequests(app, sent);
  }, 240_000);

  it("for a result that is not eligible: the rendered report, its print, its HTML save and each of the four package commands state its standing", async () => {
    const app = await mountApp(NOT_ELIGIBLE);
    await solveAndOpenReport(app);
    const panel = () => screen.getByTestId("rendered-report-panel");
    expect(screen.queryByTestId("report-package-source-unavailable")).toBeNull();
    // The result JSON export, which needs numerical eligibility, is still not offered.
    act(() => nativeMenuCommand("view.section.exports"));
    const exports = await screen.findByTestId("result-export-panel");
    expect(within(exports).queryByTestId("result-export-link")).toBeNull();
    await openReportStage();

    // The render request carries the reader's standing record for this result, and the section the
    // native render binds to the delivered result.
    vi.useFakeTimers({ toFake: ["Date"], now: PINNED_RENDER_TIME });
    const sent = await renderReport(app);
    expect(sent.exactResultText).toBe(app.raw);
    expect(sent.input.pf_timoshenko.semantic_contract_id).toBe(PREVIEW_PHYSICS_TIMOSHENKO_CONTRACT_ID);
    const record = sent.input.numerical_standing;
    expect(record.record_version).toBe("historical_1");
    expect(record.cases.map((c: Json) => c.health), JSON.stringify(record.cases)).toEqual(["sensitive"]);
    expect((sent.input.result_rows as Json[]).filter((row) => String(row.row_id).startsWith("numerical-health:"))).toEqual([]);
    // The request as sent, committed; the native render of it states the standing in words.
    const requestFile = `${JSON.stringify({
      description: "DEL-08-01 REQ-017 and REQ-024: the render request the desktop App sends through its gate (renderableSolvedResult) for a solved pressure-free Timoshenko result that is not numerically eligible, rendered for local review under a pinned clock. The delivered result bytes are the committed raw file. Written by timoshenkoReportAnyStanding.app.test.tsx; job ids are invented; unit transport replay of committed producer bytes, not a native UI qualification witness.",
      shape: "pf_timoshenko_sensitive_torsion",
      raw_path: NOT_ELIGIBLE.raw,
      solver_mode: "sparse_interactive",
      exact_intake_text: sent.exactIntakeText,
      input_manifest_text: sent.inputManifestText,
      input: sent.input
    })}\n`;
    const outDir = process.env[WRITE_APP_RENDER_REQUEST];
    if (outDir) {
      mkdirSync(outDir, { recursive: true });
      writeFileSync(resolve(outDir, APP_RENDER_REQUEST.split("/").pop()!), requestFile);
    } else {
      expect(requestFile === readFileSync(resolve(root, APP_RENDER_REQUEST), "utf8"), `${APP_RENDER_REQUEST} is stale; ${WRITE_APP_RENDER_REQUEST}=<dir> rewrites it`).toBe(true);
    }
    vi.useRealTimers();

    fireEvent.click(within(panel()).getByTestId("rendered-report-print"));
    await waitFor(() => expect(within(panel()).getByTestId("rendered-report-print-status")).toHaveTextContent("Print window opened"));
    fireEvent.click(within(panel()).getByTestId("rendered-report-save-local-private"));
    await waitFor(() => expect(within(panel()).getByTestId("rendered-report-save-status")).toHaveTextContent("Saved invented-timoshenko-report.html"));
    expect(app.prints.map((p: Json) => p.sha256Hex)).toEqual([HTML_SHA256]);
    expect(app.htmlSaves.map((s: Json) => s.sha256Hex)).toEqual([HTML_SHA256]);
    await expectFourPackageRequests(app, sent);
  }, 240_000);
});
