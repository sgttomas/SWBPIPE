/** DEL-08-01 REQ-017 and REQ-024 through the application, not around it (PR 328's text
 * review, finding 1: the component witnesses did not establish the desktop path). The
 * whole App renders; a native solve of the committed coupled-friction witness, a sensitive
 * pressure-3 result that is not numerically eligible, completes; the Report stage's
 * rendered report panel then takes it through App's own gate (the session's
 * renderableSolvedResult), and its render request carries the reader's standing record,
 * from which the native render composes the standing line (shown by the Rust
 * connecting-path test `a_readable_solved_result_that_is_not_checks_passed_renders_with_its_standing`
 * on the render input this build step makes). The rendered document is shown and its print
 * reached: in the desktop runtime Print sends the shown canonical HTML to the native print route
 * (DEL-08-01 REQ-022, `renderedReportRoutes.ts`) and mounts no print frame. The report package stays refused (the panel's save buttons are disabled with
 * the reason stated, and the native menu commands are refused with a reason), and so do the result JSON
 * export and the rule check. A replacement (a fresh solve) retires the shown report. A second
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
import { getLocalStorageCapability } from "../../services/projectService";
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
const APP_RENDER_REQUEST_SHA256 = "7072ee35b1fdc822932613792b89b7b2cf3738fe97982d5b10da02e3a0295a4a";
const WRITE_APP_RENDER_REQUEST = "OPENPIPESTRESS_WRITE_APP_RENDER_REQUEST";
/** The render's clock, pinned so the request's generated_at is reproducible. */
const PINNED_RENDER_TIME = new Date("2026-11-14T12:00:00.000Z");

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
 * settles, so that a test can act while the run is going. */
async function mountApp(hold?: Promise<void>) {
  const model = witnessModel();
  const [storage, knowledge] = [await getLocalStorageCapability(), await loadDesignKnowledge()];
  const raw = readFileSync(resolve(root, RAW), "utf8");
  const transport = createSolveJobTransport(async () => { if (hold) await hold; return raw; }, { jobIdPrefix: `invented-rendered-report-gate-job:${++mountSequence}` });
  const renders: Json[] = [];
  const prints: Json[] = [];
  let retirements = 0;
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
  it("is offered with its standing record; the package stays refused; a stale result is taken away again", async () => {
    const { raw, renders, prints, retirements, jobCommands, why } = await mountApp();

    // Before any solve the panel has no result, and the native menu's package command says why
    // nothing was saved.
    let panel = await openReportStage();
    expect(within(panel).getByTestId("rendered-report-precondition")).toBeInTheDocument();
    act(() => nativeMenuCommand("file.save-report-package"));
    await waitFor(() => expect(within(screen.getByTestId("rendered-report-panel")).getByTestId("report-package-save-status")).toHaveTextContent(
      "Not saved: report-package save needs the current mechanics result and analysis-run record. Solve first."
    ));

    act(() => nativeMenuCommand("analyze.run"));
    // The run makes Solve the active section, and the hidden Report section keeps its props while
    // dormant, so Report is opened again once the solve's run record has been read (as a person
    // must, to see it); the solved result is then in the panel.
    await waitFor(() => expect(jobCommands()).toContain("read_preview_mechanics_job_analysis_run_preview"), { timeout: 60_000 });
    await openReportStage();
    await waitFor(() => expectSolvedInPanel(why), { timeout: 60_000 });
    panel = screen.getByTestId("rendered-report-panel");
    // The report package is refused for it: the panel's save buttons are disabled with the reason
    // stated. The native File menu's two package commands are always enabled, so each is sent as
    // the OS menu sends it: nothing is saved, and the panel says why.
    for (const id of ["report-package-save", "report-package-user-release"]) {
      expect(within(panel).getByTestId(id)).toBeDisabled();
      expect(within(panel).getByTestId(id).getAttribute("title")).toContain("needs a numerically eligible result");
    }
    for (const command of ["file.save-report-package", "file.save-shareable-report-package"]) {
      act(() => nativeMenuCommand(command));
      await waitFor(() => expect(within(screen.getByTestId("rendered-report-panel")).getByTestId("report-package-save-status")).toHaveTextContent(
        "Not saved: report-package save needs a numerically eligible result. The rendered report states this result's numerical standing."
      ));
    }
    expect(screen.queryByTestId("user-release-dialog")).toBeNull();
    expect(invokeMock.mock.calls.map(([command]) => command).filter((command) => /report_package|user_release/.test(String(command)))).toEqual([]);
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
    expect(Object.keys(renders[0]).sort()).toEqual(["exactIntakeText", "exactResultText", "input", "inputManifestText"]);
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
    // print route receives the shown canonical HTML under the render's context, and no print frame
    // is mounted (DEL-08-01 REQ-022).
    await waitFor(() => expect(within(screen.getByTestId("rendered-report-panel")).getByTestId("rendered-report-preview")).toBeInTheDocument());
    fireEvent.click(within(screen.getByTestId("rendered-report-panel")).getByTestId("rendered-report-print"));
    await waitFor(() => expect(within(screen.getByTestId("rendered-report-panel")).getByTestId("rendered-report-print-status")).toHaveTextContent("Print window opened"));
    expect(prints.map((p: Json) => [p.html, p.exportContext, p.explicitLocalPrivateIntent])).toEqual([[HTML, "local_private", true]]);
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

  it("the shareable menu refusal appears without a preceding private-save request", async () => {
    const { jobCommands, why } = await mountApp();
    act(() => nativeMenuCommand("analyze.run"));
    await waitFor(() => expect(jobCommands()).toContain("read_preview_mechanics_job_analysis_run_preview"), { timeout: 60_000 });
    await openReportStage();
    await waitFor(() => expectSolvedInPanel(why), { timeout: 60_000 });
    expect(within(screen.getByTestId("rendered-report-panel")).queryByTestId("report-package-save-status")).toBeNull();
    act(() => nativeMenuCommand("file.save-shareable-report-package"));
    await waitFor(() => expect(within(screen.getByTestId("rendered-report-panel")).getByTestId("report-package-save-status")).toHaveTextContent(
      "Not saved: report-package save needs a numerically eligible result. The rendered report states this result's numerical standing."
    ));
    expect(screen.queryByTestId("user-release-dialog")).toBeNull();
    expect(invokeMock.mock.calls.map(([command]) => command).filter((command) => /report_package|user_release/.test(String(command)))).toEqual([]);
  }, 240_000);
});
