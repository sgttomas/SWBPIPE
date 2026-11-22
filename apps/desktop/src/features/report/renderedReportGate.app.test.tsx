/** DEL-08-01 REQ-017 and REQ-024 through the application, not around it (PR 328's text
 * review, finding 1: the component witnesses did not establish the desktop path). The
 * whole App renders; a native solve of the committed coupled-friction witness, a sensitive
 * pressure-3 result that is not numerically eligible, completes; the Report stage's
 * rendered report panel then takes it through App's own gate (the session's
 * renderableSolvedResult), and its render request carries the reader's standing record,
 * from which the native render composes the standing line (shown by the Rust
 * connecting-path test `a_readable_solved_result_that_is_not_checks_passed_renders_with_its_standing`
 * on the render input this build step makes). The rendered document is shown and its print
 * reached: in the desktop runtime Print names the shown render by its SHA-256 to the native print
 * route, which holds the document (DEL-08-01 REQ-022, `renderedReportRoutes.ts`), and mounts no
 * print frame. The report package and the
 * user release take the same source (DEL-08-01 REQ-024, "Package and user release"): each of the four
 * package commands (the panel's two save buttons, the File menu's two commands) offers the save and makes
 * its request, which carries the result's standing record, and the user-release dialog states the
 * report frame's statements, as the native host gives them, before the statement. The result JSON
 * export and the rule check, which need numerical eligibility, are still not offered. A replacement (a fresh solve) retires the shown report. A second
 * test opens Report while the run is going, so that completion arrives with Report visible: the
 * solved result reaches the panel with no further action. The replacement also retires the
 * shown render in the desktop's custody (`retire_rendered_report`).
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
import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { resolve } from "node:path";
import { afterEach, describe, expect, it, vi } from "vitest";
const invokeMock = vi.hoisted(() => vi.fn());
vi.mock("@tauri-apps/api/core", async () => ({ invoke: (await import("../../test/nativeIngress")).ingressDecodingInvoke(invokeMock) }));
import { App } from "../../App";
import { createSolveJobTransport, REPLAY_COMMANDS } from "../../test/nativeMechanicsReplay";
import { useModelledVisibility } from "../../test-support/modelledVisibility";
import { REPORT_SOURCE_COMMANDS, reportSourceRegistration } from "../../test/reportSourceRegistration";
import { REPORT_PACKAGE_DENIED_STATES } from "../../test/reportPackageDeniedStates";
import { getLocalStorageCapability } from "../../services/projectService";
import { loadWasmEngine } from "../../services/wasmEngine/loadWasmEngine";
import { outcomeFromWasmEngineJson } from "../../services/operationService";
import { loadDesignKnowledge, resetSessionResultCustodyForTests } from "../../services/previewService";
import type { PreviewModel } from "../../types";
import demoModel from "../../../../../fixtures/product_preview/invented_demo_model.json";

type Json = any; // eslint-disable-line @typescript-eslint/no-explicit-any
const root = resolve(__dirname, "../../../../../");
const DIR = "fixtures/reports/invented/report_standing_witnesses";
const RAW = `${DIR}/pressure_3_coupled_friction_sensitive-sparse_interactive.raw.json`;
const REQUEST = `${DIR}/coupled_friction.request.json`;
/** The render request this App sends through its gate, committed (stale fails;
 * `OPENPIPESTRESS_WRITE_APP_RENDER_REQUEST=<dir>` writes it), and consumed through the native
 * production render command with live custody by `report_render_exact_tests.rs`
 * (`the_app_gate_render_request_renders_through_production_with_its_standing`). Both tests hold
 * its SHA-256, so each reads exactly the bytes the other makes or checks. */
const APP_RENDER_REQUEST = `${DIR}/pressure_3_coupled_friction_sensitive.app_render_request.json`;
const APP_RENDER_REQUEST_SHA256 = "217708dc55ad57f8c8a6ff241008ffdffa4a057fe980dadf8a7f5717e45ca27f";
const WRITE_APP_RENDER_REQUEST = "OPENPIPESTRESS_WRITE_APP_RENDER_REQUEST";
/** The render's clock, pinned so the request's generated_at is reproducible. */
const PINNED_RENDER_TIME = new Date("2026-11-14T12:00:00.000Z");
/** The frame statements the native host gives for this witness, held equal to the native
 * operation by `report_standing_package_tests.rs`. */
const FRAME_STATEMENTS = `${DIR}/pressure_3_coupled_friction_sensitive.frame_statements.json`;

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
  // The two display members the desktop's model shape needs and this solve request omits; neither is
  // read by the solver or the reader (an empty diagnostics list; the bundled demo's data boundary).
  const record = model as unknown as Record<string, unknown>;
  if (record.diagnostics === undefined) record.diagnostics = [];
  if (record.data_boundary === undefined) record.data_boundary = structuredClone((demoModel as unknown as Record<string, unknown>).data_boundary);
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

/** Opens the Report section through the menu command a person's View menu sends. */
async function openReportStage() {
  act(() => nativeMenuCommand("view.section.report"));
  return screen.findByTestId("rendered-report-panel");
}

/** Each mount's job ids are its own (the native host's are unique in a session). */
let mountSequence = 0;
/** The App over the replay transport. `hold`, when given, keeps the job's start pending until it
 * settles, so that a test can act while the run is going. `source`, when given, is the model and
 * producer bytes solved in place of the witness's. */
async function mountApp(hold?: Promise<void>, onSave?: (args: Json) => unknown, source?: { model: PreviewModel; raw: string }) {
  const model = source?.model ?? witnessModel();
  const [storage, knowledge] = [await getLocalStorageCapability(), await loadDesignKnowledge()];
  const raw = source?.raw ?? readFileSync(resolve(root, RAW), "utf8");
  const transport = createSolveJobTransport(async () => { if (hold) await hold; return raw; }, { jobIdPrefix: `invented-rendered-report-gate-job:${++mountSequence}` });
  const renders: Json[] = [];
  const prints: Json[] = [];
  let retirements = 0;
  const reportSource = reportSourceRegistration();
  // Any command outside the job contract is refused by name, as the native-replay App tests do; the
  // App's startup chores it refuses (autosave, unit catalogue, model states) are tolerated by App, and
  // the refused names are reported with any stalled step.
  const refused: string[] = [];
  const jobCommands = (): string[] => transport.commands().map((c: Json) => (typeof c === "string" ? c : c.command));
  invokeMock.mockImplementation(async (command: string, args?: unknown, options?: unknown) => {
    if (command === "get_local_storage_capability") return storage;
    if (command === "load_design_knowledge") return knowledge;
    if (command === "sync_native_shell_state") return null;
    if (command === "load_preview_model") {
      if (args !== undefined) throw new Error("RENDERED_REPORT_GATE_REQUEST_INVALID");
      return structuredClone(model);
    }
    if (command === "render_calculation_report") { renders.push(args); return renderedOutcome(); }
    // DEL-08-01 REQ-022's native print route and the panel's retirement of a discarded render.
    if (command === "print_rendered_report") {
      prints.push(args);
      const canonical = createHash("sha256").update(HTML).digest("hex");
      return { outcome: "print_window_opened", route_id: "DREP-HTML-SAVE-005", export_context: "local_private", canonical_sha256_hex: canonical, print_sha256_hex: "c".repeat(64) };
    }
    if (command === "retire_rendered_report") { retirements += 1; return null; }
    // The package save's native command: the test records each request it is sent.
    if (command === "save_report_package" && onSave) return onSave(args);
    // DEL-08-01 REQ-022: the report source registration, as the native process answers it
    // (observation, conditional activation, highest number of the current incarnation).
    if (REPORT_SOURCE_COMMANDS.includes(command)) return reportSource(command, args);
    // A model edit's operation, through the maintained operation engine (as App.mbfRoundTrip.test.tsx
    // does), not a fabricated edited document: App still performs the edit, its hash checks and the
    // session commit. No actual native operation witness.
    if (command === "validate_model_operation" || command === "apply_model_operation") {
      const operation = args as { model: unknown; intent: unknown; claimedModelHash?: unknown };
      const engine = await loadWasmEngine();
      const execute = command === "apply_model_operation" ? engine.applyOperationJson : engine.validateOperationJson;
      return outcomeFromWasmEngineJson(execute(JSON.stringify(operation.model), JSON.stringify(operation.intent), JSON.stringify(operation.claimedModelHash ?? null)));
    }
    // Every other command is the native mechanics job contract, or refused by name.
    if (!REPLAY_COMMANDS.includes(command) && !transport.handles(command)) { refused.push(command); throw new Error(`RENDERED_REPORT_GATE_COMMAND_UNSUPPORTED: ${command}`); }
    return transport.invoke(command, args, options);
  });
  (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__ = {};
  render(<App />);
  await screen.findByTestId("desktop-preview-shell");
  await screen.findByTestId("workspace-toolbar");
  await screen.findByTestId(`tree-row-project-${encodeURIComponent(model.project.id)}`);
  const why = () => `refused commands: ${refused.join(", ")}; job commands: ${jobCommands().join(", ")}; status: ${screen.queryByTestId("local-project-message")?.textContent ?? ""}; alerts: ${[...document.querySelectorAll('[role="alert"], [data-testid*="error"], [data-testid*="refus"]')].map((e) => `${e.getAttribute("data-testid") ?? "alert"}=${(e.textContent ?? "").slice(0, 300)}`).join(" | ")}`;
  return { raw, renders, prints, retirements: () => retirements, jobCommands, why };
}

/** The solved result has reached the shown panel through the gate (its full copy read for it). */
function expectSolvedInPanel(why: () => string) {
  expect(screen.queryByTestId("c2-result-failed")?.textContent ?? null, why()).toBeNull();
  expect(within(screen.getByTestId("rendered-report-panel")).queryByTestId("rendered-report-precondition"), why()).toBeNull();
  expect(within(screen.getByTestId("rendered-report-panel")).getByTestId("rendered-report-render"), why()).toBeEnabled();
}

describe("the rendered report of a solved exact-route result that is not numerically eligible, through App's gate", () => {
  it("is offered with its standing record, and so is its package; a stale result is taken away again", async () => {
    const { raw, renders, prints, retirements, jobCommands, why } = await mountApp();

    // Before any solve the panel has no result, and the native menu's package command says why
    // nothing was saved.
    let panel = await openReportStage();
    expect(within(panel).getByTestId("rendered-report-precondition")).toBeInTheDocument();
    act(() => nativeMenuCommand("file.save-report-package"));
    await waitFor(() => expect(within(screen.getByTestId("rendered-report-panel")).getByTestId("report-package-save-status")).toHaveTextContent(
      "Report package not saved: the model has no current result. Solve it, then save. Nothing was written."
    ));

    act(() => nativeMenuCommand("analyze.run"));
    // The run makes Solve the active section, and the hidden Report section keeps its props while
    // dormant, so Report is opened again once the solve's run record has been read (as a person
    // must, to see it); the solved result is then in the panel.
    await waitFor(() => expect(jobCommands()).toContain("read_preview_mechanics_job_analysis_run_preview"), { timeout: 60_000 });
    await openReportStage();
    await waitFor(() => expectSolvedInPanel(why), { timeout: 60_000 });
    panel = screen.getByTestId("rendered-report-panel");
    // The report package is offered for it (DEL-08-01 REQ-024): the panel's save buttons are
    // enabled, with no reason against them (the four commands' requests: the test below).
    for (const id of ["report-package-save", "report-package-user-release"]) {
      expect(within(panel).getByTestId(id)).toBeEnabled();
      expect(within(panel).getByTestId(id).getAttribute("title")).toBeNull();
    }
    // The result JSON export and the rule check, which need numerical eligibility, are not offered.
    act(() => nativeMenuCommand("view.section.exports"));
    const exports = await screen.findByTestId("result-export-panel");
    expect(within(exports).getByTestId("result-export-empty")).toBeInTheDocument();
    expect(within(exports).queryByTestId("result-export-link")).toBeNull();
    act(() => nativeMenuCommand("view.section.solve"));
    expect(await screen.findByTestId("rule-check-run")).toBeDisabled();
    panel = await openReportStage();

    // The render request carries the reader's standing record for this sensitive result. It is
    // rendered for the person's own local review, the context the committed witness render inputs
    // were made in (`reportStandingWitness.test.tsx`); the public-report context withholds this
    // invented model's private inputs, which is the redaction boundary's own behaviour.
    fireEvent.click(within(panel).getByTestId("rendered-report-local-private-intent"));
    vi.useFakeTimers({ toFake: ["Date"], now: PINNED_RENDER_TIME });
    try {
      fireEvent.click(within(screen.getByTestId("rendered-report-panel")).getByTestId("rendered-report-render"));
      await waitFor(() => {
        expect(within(screen.getByTestId("rendered-report-panel")).queryByTestId("rendered-report-error")?.textContent ?? null, why()).toBeNull();
        expect(renders.length, why()).toBe(1);
      }, { timeout: 60_000 });
    } finally {
      vi.useRealTimers();
    }
    const record = renders[0].input.numerical_standing;
    expect(record.record_version).toBe("historical_1");
    expect(record.cases.map((c: Json) => c.health)).toEqual(["sensitive"]);
    expect(record.cases[0].confirmation.outcome).toBe("solver_reported_established");
    expect(record.cases[0].confirmation.status).toBe("conditional");
    expect(renders[0].exactResultText).toBe(raw);
    // The request as sent, committed and bound by digest: the delivered result is named by its
    // committed raw file (the bytes asserted equal above), the rest is carried whole.
    expect(Object.keys(renders[0]).sort()).toEqual(["exactIntakeText", "exactResultText", "input", "inputManifestText", "request", "source"]);
    // DEL-08-01 REQ-022: the render names the report source number the native process registered,
    // and its owner's request ordinal, allocated when this first preparation began.
    expect(renders[0].source.incarnation).toBe(1);
    expect(renders[0].request).toBe(1);
    expect(JSON.stringify(renders[0].input)).toContain(`"generated_at":"${PINNED_RENDER_TIME.toISOString()}"`);
    const requestFile = `${JSON.stringify({
      description: "DEL-08-01 REQ-017 and REQ-024: the render request the desktop App sends through its gate (renderableSolvedResult) for a solved exact-route result that is not numerically eligible, rendered for local review under a pinned clock. The delivered result bytes are the committed raw file. Written by renderedReportGate.app.test.tsx; job ids are invented; unit transport replay of committed producer bytes, not a native UI qualification witness.",
      shape: "pressure_3_coupled_friction_sensitive",
      raw_path: RAW,
      solver_mode: "sparse_interactive",
      exact_intake_text: renders[0].exactIntakeText,
      input_manifest_text: renders[0].inputManifestText,
      input: renders[0].input
    })}\n`;
    const requestSha256 = createHash("sha256").update(requestFile).digest("hex");
    const outDir = process.env[WRITE_APP_RENDER_REQUEST];
    if (outDir) {
      mkdirSync(outDir, { recursive: true });
      writeFileSync(resolve(outDir, APP_RENDER_REQUEST.split("/").pop()!), requestFile);
      console.info(`APP_RENDER_REQUEST_SHA256=${requestSha256}`);
    } else {
      expect(requestFile === readFileSync(resolve(root, APP_RENDER_REQUEST), "utf8"), `${APP_RENDER_REQUEST} is stale; ${WRITE_APP_RENDER_REQUEST}=<dir> rewrites it`).toBe(true);
      expect(requestSha256).toBe(APP_RENDER_REQUEST_SHA256);
    }
    // The rendered report is shown, and its print is reached: in the desktop runtime the native
    // print route is asked for the shown render, named by its SHA-256 (no HTML is sent; the desktop
    // keeps the document), under the render's context, and no print frame is mounted (DEL-08-01
    // REQ-022).
    await waitFor(() => expect(within(screen.getByTestId("rendered-report-panel")).getByTestId("rendered-report-preview")).toBeInTheDocument());
    fireEvent.click(within(screen.getByTestId("rendered-report-panel")).getByTestId("rendered-report-print"));
    await waitFor(() => expect(within(screen.getByTestId("rendered-report-panel")).getByTestId("rendered-report-print-status")).toHaveTextContent("Print window opened"));
    expect(prints.map((p: Json) => ["html" in p, p.sha256Hex, p.exportContext, p.explicitLocalPrivateIntent])).toEqual([
      [false, createHash("sha256").update(HTML).digest("hex"), "local_private", true]
    ]);
    expect(within(screen.getByTestId("rendered-report-panel")).queryByTestId("rendered-report-print-frame")).toBeNull();
    const retiredBefore = retirements();

    // A replacement (a fresh solve) retires the shown report: once the new run record has been
    // read, the Report section shows no rendered document until it is rendered again.
    const runRecordsRead = jobCommands().filter((c) => c === "read_preview_mechanics_job_analysis_run_preview").length;
    act(() => nativeMenuCommand("analyze.run"));
    await waitFor(() => expect(jobCommands().filter((c) => c === "read_preview_mechanics_job_analysis_run_preview").length).toBe(runRecordsRead + 1), { timeout: 60_000 });
    await openReportStage();
    await waitFor(() => {
      const shown = screen.getByTestId("rendered-report-panel");
      expect(within(shown).queryByTestId("rendered-report-preview")).toBeNull();
      expect(within(shown).queryByTestId("rendered-report-print-frame")).toBeNull();
      expect(within(shown).getByTestId("rendered-report-render")).toBeEnabled();
    }, { timeout: 60_000 });
    expect(renders.length).toBe(1);
    // The panel retired the replaced render in the desktop's custody.
    expect(retirements()).toBeGreaterThan(retiredBefore);
  }, 240_000);

  it("reaches a Report section that is open when the run completes, with no further action", async () => {
    let release!: () => void;
    const hold = new Promise<void>((resolveHold) => { release = resolveHold; });
    const { jobCommands, why } = await mountApp(hold);

    act(() => nativeMenuCommand("analyze.run"));
    // The run makes Solve the active section; the job's start is held, so the run is still going.
    await waitFor(() => expect(jobCommands()).toContain("start_preview_mechanics_job"));
    expect(screen.getByTestId("workspace-section-solve").className).toBe("workspace-dock-section");
    // The person opens Report during the run; it shows no result yet.
    const panel = await openReportStage();
    expect(within(panel).getByTestId("rendered-report-precondition")).toBeInTheDocument();
    expect(jobCommands()).not.toContain("read_preview_mechanics_job_analysis_run_preview");

    // Completion arrives while Report is the active, visible section. Nothing further is done: the
    // solved result reaches the panel through the gate.
    act(() => release());
    await waitFor(() => expect(jobCommands(), why()).toContain("read_preview_mechanics_job_analysis_run_preview"), { timeout: 60_000 });
    await waitFor(() => expectSolvedInPanel(why), { timeout: 60_000 });
    expect(screen.getByTestId("workspace-section-report").className).toBe("workspace-dock-section");
  }, 240_000);

  /** DEL-08-01 REQ-022 and AC-011 (independent round-1 review of PR 269, finding R3): the native
   * render and its separate print window are retired when their source is replaced while the
   * Report section is hidden, not only once Report is opened again. App keeps Report in its real
   * DormantSection, which does not reconcile an inactive subtree; the native side is the mocked
   * IPC above, where `retire_rendered_report` is the panel's retirement of the native custody and
   * print window. On the reviewed behaviour this passes: the new solve releases the session result
   * that C2ResultGate reads from session context, which reaches the gate through the dormant memo,
   * so the gate unmounts the panel and its unmount cleanup retires. It guards that path; it does
   * not cover an ordinary model edit, which leaves the session result in place. */
  it("retires the native render and its print window when a new solve replaces the result while Report is hidden", async () => {
    const { prints, retirements, jobCommands, why } = await mountApp();
    act(() => nativeMenuCommand("analyze.run"));
    await waitFor(() => expect(jobCommands()).toContain("read_preview_mechanics_job_analysis_run_preview"), { timeout: 60_000 });
    let panel = await openReportStage();
    await waitFor(() => expectSolvedInPanel(why), { timeout: 60_000 });
    panel = screen.getByTestId("rendered-report-panel");
    fireEvent.click(within(panel).getByTestId("rendered-report-local-private-intent"));
    fireEvent.click(within(screen.getByTestId("rendered-report-panel")).getByTestId("rendered-report-render"));
    await waitFor(() => expect(within(screen.getByTestId("rendered-report-panel")).getByTestId("rendered-report-preview")).toBeInTheDocument(), { timeout: 60_000 });
    fireEvent.click(within(screen.getByTestId("rendered-report-panel")).getByTestId("rendered-report-print"));
    await waitFor(() => expect(within(screen.getByTestId("rendered-report-panel")).getByTestId("rendered-report-print-status")).toHaveTextContent("Print window opened"));
    expect(prints).toHaveLength(1);
    const retiredBefore = retirements();

    // Report is hidden; a new solve then replaces the result the shown render was made from.
    act(() => nativeMenuCommand("view.section.solve"));
    expect(screen.getByTestId("workspace-section-report").className).not.toBe("workspace-dock-section");
    const runRecordsRead = jobCommands().filter((c) => c === "read_preview_mechanics_job_analysis_run_preview").length;
    act(() => nativeMenuCommand("analyze.run"));
    await waitFor(() => expect(jobCommands().filter((c) => c === "read_preview_mechanics_job_analysis_run_preview").length).toBe(runRecordsRead + 1), { timeout: 60_000 });

    // Before Report is opened again, the native render and its print window are retired.
    await waitFor(() => expect(
      retirements() - retiredBefore,
      `retire_rendered_report calls while Report stays hidden after the replacing solve (Report section class: ${screen.getByTestId("workspace-section-report").className}); ${why()}`
    ).toBeGreaterThan(0), { timeout: 10_000 });
    expect(screen.getByTestId("workspace-section-report").className).not.toBe("workspace-dock-section");
  }, 240_000);

  /** DEL-08-01 REQ-022 and AC-011 (round-1 finding R3, the model-edit case): an ordinary model edit
   * made while Report is hidden replaces the rendered report's source (REQ-022: "A report or
   * package is produced only for the result and model current when it is saved"), so the native
   * render and its print window are retired before Report reopens. The edit goes through the
   * Layout grid, the model-operation queue and the maintained operation engine. */
  it("retires the native render and its print window when a model edit is made while Report is hidden", async () => {
    const { prints, retirements, jobCommands, why } = await mountApp();
    act(() => nativeMenuCommand("analyze.run"));
    await waitFor(() => expect(jobCommands()).toContain("read_preview_mechanics_job_analysis_run_preview"), { timeout: 60_000 });
    await openReportStage();
    await waitFor(() => expectSolvedInPanel(why), { timeout: 60_000 });
    fireEvent.click(within(screen.getByTestId("rendered-report-panel")).getByTestId("rendered-report-local-private-intent"));
    fireEvent.click(within(screen.getByTestId("rendered-report-panel")).getByTestId("rendered-report-render"));
    await waitFor(() => expect(within(screen.getByTestId("rendered-report-panel")).getByTestId("rendered-report-preview")).toBeInTheDocument(), { timeout: 60_000 });
    fireEvent.click(within(screen.getByTestId("rendered-report-panel")).getByTestId("rendered-report-print"));
    await waitFor(() => expect(within(screen.getByTestId("rendered-report-panel")).getByTestId("rendered-report-print-status")).toHaveTextContent("Print window opened"));
    expect(prints).toHaveLength(1);
    const retiredBefore = retirements();
    const reportHidden = () => screen.getByTestId("workspace-section-report").className !== "workspace-dock-section";

    // Report is hidden; one witness node's coordinate is edited in the Layout grid and applied.
    act(() => nativeMenuCommand("view.stage.model"));
    expect(reportHidden(), `Report section class: ${screen.getByTestId("workspace-section-report").className}`).toBe(true);
    const applies = () => invokeMock.mock.calls.filter(([command]) => command === "apply_model_operation").length;
    fireEvent.click(screen.getByTestId("layout-mode-grid"));
    const cell = () => screen.getByTestId("table-cell-node:RS-020-y");
    fireEvent.click(cell()); fireEvent.click(cell());
    fireEvent.change(screen.getByRole("textbox", { name: "node:RS-020 Y [m]" }), { target: { value: "0.05" } });
    fireEvent.click(within(screen.getByTestId("engineering-table")).getByRole("button", { name: "Apply" }));
    await waitFor(() => expect(cell(), why()).toHaveTextContent("0.05"), { timeout: 30_000 });
    expect(applies()).toBe(1);
    expect(reportHidden()).toBe(true);

    // Before Report is opened again, the native render and its print window are retired.
    await waitFor(() => expect(
      retirements() - retiredBefore,
      `retire_rendered_report calls while Report stays hidden after the model edit (Report section class: ${screen.getByTestId("workspace-section-report").className}); ${why()}`
    ).toBeGreaterThan(0), { timeout: 10_000 });
    expect(reportHidden()).toBe(true);
  }, 240_000);

  /** DEL-08-01 REQ-024, "Package and user release": each of the four package commands offers the
   * save for this result and makes its request, carrying the result's standing record; the native
   * save, which derives the record again and requires it equal, is not run here (its unit test is
   * `report_standing_package_tests.rs`). The user-release dialog states the report frame's
   * statements, as the native host gives them, each on its own line before the statement, and its
   * save waits for the statement. */
  it("each of the four package commands makes its request, and the dialog states the frame's statements first", async () => {
    const saves: Json[] = [];
    const { why } = await mountApp(undefined, (args) => {
      saves.push(args);
      throw new Error("INVENTED-SAVE-NOT-WRITTEN: this unit transport writes no package");
    });
    act(() => nativeMenuCommand("analyze.run"));
    await waitFor(() => expect(invokeMock.mock.calls.map(([command]) => command)).toContain("read_preview_mechanics_job_analysis_run_preview"), { timeout: 60_000 });
    await openReportStage();
    await waitFor(() => expectSolvedInPanel(why), { timeout: 60_000 });
    const panel = () => screen.getByTestId("rendered-report-panel");
    // The standing record this App's render request carries for the same result (committed above).
    const record = JSON.parse(readFileSync(resolve(root, APP_RENDER_REQUEST), "utf8")).input.numerical_standing;
    const statements: string[] = JSON.parse(readFileSync(resolve(root, FRAME_STATEMENTS), "utf8")).frame_statements;
    expect(statements).toHaveLength(1);
    const saved = async (count: number) => {
      await waitFor(() => expect(saves.length, why()).toBe(count), { timeout: 60_000 });
      await waitFor(() => expect(within(panel()).getByTestId("report-package-save")).toBeEnabled(), { timeout: 60_000 });
      return JSON.parse(saves[count - 1].requestCanonicalText);
    };
    // The local private package: from the panel's button, then from the File menu.
    fireEvent.click(within(panel()).getByTestId("report-package-private-intent"));
    fireEvent.click(within(panel()).getByTestId("report-package-save"));
    for (const [count, command] of [[1, null], [2, "file.save-report-package"]] as const) {
      if (command) act(() => nativeMenuCommand(command));
      const request = await saved(count);
      expect(request.export_profile_id, why()).toBe("desktop_local_private_report_package_1");
      expect(request.report.numerical_standing, why()).toEqual(record);
    }
    // The shareable package: from the panel's button, then from the File menu. Each opens the dialog,
    // which states the frame's statements before the statement; only the statement enables its save.
    for (const [count, open] of [[3, () => fireEvent.click(within(panel()).getByTestId("report-package-user-release"))], [4, () => act(() => nativeMenuCommand("file.save-shareable-report-package"))]] as const) {
      open();
      const dialog = await screen.findByTestId("user-release-dialog", undefined, { timeout: 60_000 });
      expect(within(dialog).queryByTestId("user-release-refusal"), why()).toBeNull();
      expect(within(dialog).getAllByTestId("user-release-frame-statement").map((line) => line.textContent)).toEqual(statements);
      expect(dialog).not.toHaveTextContent("Kept for inspection.");
      const statement = within(dialog).getByTestId("user-release-statement");
      expect(within(dialog).getByTestId("user-release-frame-statements").compareDocumentPosition(statement) & Node.DOCUMENT_POSITION_FOLLOWING).toBeTruthy();
      expect(within(dialog).getByTestId("user-release-confirm")).toBeDisabled();
      fireEvent.click(statement);
      fireEvent.click(within(dialog).getByTestId("user-release-confirm"));
      const request = await saved(count);
      expect(request.export_profile_id, why()).toBe("desktop_user_released_report_package_1");
      expect(request.report.report_context.user_release.record_version).toBe("1.2.0");
      expect(request.report.numerical_standing, why()).toEqual(record);
    }
    expect(saves).toHaveLength(4);
  }, 240_000);
});

/** DEL-08-01 REQ-024, "Package and user release": for each state the session test
 * (`reportPackageAdmission.session.test.tsx`) observes the four package commands refusing, App
 * offers no package once the solve has ended. The panel's two save buttons are disabled. The File
 * menu is the native one in the desktop runtime, and the state the web view gives it
 * (`NativeShellState`) carries no package items, so its two package commands stay selectable:
 * each, when sent, leaves its refusal word for word in the panel, with no dialog and no save. */
describe.each(REPORT_PACKAGE_DENIED_STATES)("the report package's controls for %s, through App", (_name, make, message) => {
  it.each(["file.save-report-package", "file.save-shareable-report-package"])("the panel's two save buttons are disabled, and the native menu's %s, alone in a fresh App, refuses", async (command) => {
    const { model, bytes, state } = make();
    const { why } = await mountApp(undefined, () => { throw new Error("INVENTED-SAVE-NOT-EXPECTED"); }, { model, raw: bytes });
    act(() => nativeMenuCommand("analyze.run"));
    await waitFor(() => expect(screen.getByTestId("solve-progress-panel"), why()).toHaveAttribute("data-state", state), { timeout: 60_000 });
    const panel = await openReportStage();
    await waitFor(() => expect(within(panel).getByTestId("rendered-report-precondition"), why()).toBeInTheDocument(), { timeout: 60_000 });
    for (const id of ["report-package-save", "report-package-user-release"]) {
      expect(within(panel).getByTestId(id), `${id}; ${why()}`).toBeDisabled();
    }
    // No status before the command, so the refusal is this command's.
    expect(within(panel).queryByTestId("report-package-save-status")).toBeNull();
    act(() => nativeMenuCommand(command));
    await waitFor(() => expect(within(screen.getByTestId("rendered-report-panel")).getByTestId("report-package-save-status"), why()).toHaveTextContent(message));
    expect(screen.queryByTestId("user-release-dialog")).toBeNull();
    expect(invokeMock.mock.calls.filter(([name]) => name === "save_report_package")).toHaveLength(0);
  }, 120_000);
});
