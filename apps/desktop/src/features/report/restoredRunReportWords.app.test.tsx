/** The Report section's disabled controls say why, for the state a person is in: with no run they
 * say to solve first; with a run restored from the saved project (File > Save Local, then File >
 * Open Local) they say that the restored run cannot be rendered or packaged, and to solve again.
 * Both states keep Render and both package saves disabled. The whole App renders; the solve is
 * the native job transport replay of committed producer bytes and the store a unit fake (unit
 * transport replay; NOT a native UI qualification witness). All data are invented. */
import { act, cleanup, fireEvent, render, screen, waitFor, within } from "@testing-library/react";
import { afterEach, expect, it, vi } from "vitest";
const invokeMock = vi.hoisted(() => vi.fn());
vi.mock("@tauri-apps/api/core", async () => ({ invoke: (await import("../../test/nativeIngress")).ingressDecodingInvoke(invokeMock) }));
import { App } from "../../App";
import { createNativeMechanicsReplay, REPLAY_COMMANDS } from "../../test/nativeMechanicsReplay";
import { createNativePersistenceFake } from "../../test/nativePersistenceFake";
import { NATIVE_SAVE_PROJECT_COMMAND } from "../../services/nativePersistence";
import { getLocalStorageCapability } from "../../services/projectService";
import { loadDesignKnowledge, resetSessionResultCustodyForTests } from "../../services/previewService";
import type { LocalProjectEnvelope } from "../../types";

type Json = any; // eslint-disable-line @typescript-eslint/no-explicit-any

afterEach(() => {
  cleanup();
  vi.restoreAllMocks();
  invokeMock.mockReset();
  resetSessionResultCustodyForTests();
  window.localStorage.clear();
  delete (window as Json).__TAURI_INTERNALS__;
});

const NO_RUN = {
  render: "Disabled: rendering needs a completed mechanics preview and its analysis-run record in this session. Open the Solve section and run the mechanics preview first.",
  line: "Solve first: rendering needs a mechanics result and analysis-run record.",
  package: "Solve first: report-package save needs the current mechanics result and analysis-run record."
};
const RESTORED = {
  render: "This run was restored from the saved project and cannot be rendered. Solve the model again to render its report.",
  line: "This run was restored from the saved project and cannot be rendered. Solve the model again to render its report.",
  package: "This run was restored from the saved project and cannot be packaged. Solve the model again to save a report package."
};

async function installStore() {
  const storage = await getLocalStorageCapability(), knowledge = await loadDesignKnowledge();
  const mechanics = createNativeMechanicsReplay({ profile: "preview" });
  const rows = new Map<string, string>();
  const saved: LocalProjectEnvelope[] = [];
  let sequence = 0;
  const saveProject = (request: Json): LocalProjectEnvelope => {
    const id = request.model.project.id as string;
    const base = `sha256:${id}#${++sequence}`;
    rows.set(id, base);
    const envelope = {
      summary: { project_id: id, project_name: request.model.project.name, message: "Saved current project model snapshot without external file copies." },
      model: request.model, editor_intents: [], proposal: null, selected_review_target: null, mechanics_result: request.mechanics_result ?? null,
      analysis_run: request.analysis_run ?? null, model_hash: request.model_hash, project_envelope_hash: { value: base }, model_document_version: null
    } as unknown as LocalProjectEnvelope;
    saved.push(envelope);
    return envelope;
  };
  const fake = createNativePersistenceFake({ heldBytes: mechanics.heldBytes, heldIntake: mechanics.heldIntake, save: ({ request }) => saveProject(request) });
  invokeMock.mockImplementation(async (command: string, args?: Json, options?: unknown) => {
    switch (command) {
      case "get_local_storage_capability": return storage;
      case "load_design_knowledge": return knowledge;
      case "sync_native_shell_state": return null;
      case "check_model_library_refs": return [];
      case "find_local_project": {
        const id = args.projectId as string;
        return rows.has(id) ? { project_id: id, project_name: "Stored", storage_mode: "local_sqlite", created_at_unix: 1, updated_at_unix: 2, revision: `rev:${rows.get(id)}`, saved_state_count: 0, recorded_run_count: 0 } : null;
      }
      case "save_local_project": return saveProject(args.request);
      case NATIVE_SAVE_PROJECT_COMMAND: return fake.invoke(command, args);
      case "open_local_project": return structuredClone(saved.at(-1) ?? null);
      case "list_model_states": return [];
      case "list_analysis_runs": return [];
    }
    if (REPLAY_COMMANDS.includes(command)) return mechanics.invoke(command, args, options);
    throw new Error(`RESTORED_RUN_WORDS_COMMAND_UNSUPPORTED: ${command}`);
  });
  (window as Json).__TAURI_INTERNALS__ = {};
}

function nativeMenuCommand(command: string) {
  act(() => { window.dispatchEvent(new CustomEvent("openpipestress-native-menu-command", { detail: command })); });
}
async function settle() {
  await act(async () => { await new Promise((resolve) => setTimeout(resolve, 50)); });
}

/** Render and both package saves in the Report section: disabled, with these words. */
async function expectReportControls(words: { render: string; line: string; package: string }) {
  nativeMenuCommand("view.section.report");
  const panel = await within(screen.getByTestId("workspace-section-report")).findByTestId("rendered-report-panel");
  const renderButton = within(panel).getByTestId("rendered-report-render");
  expect(renderButton).toBeDisabled();
  expect(renderButton).toHaveAttribute("title", words.render);
  expect(within(panel).getByTestId("rendered-report-precondition")).toHaveTextContent(words.line);
  for (const id of ["report-package-save", "report-package-user-release"]) {
    const control = within(panel).getByTestId(id);
    expect(control).toBeDisabled();
    expect(control).toHaveAttribute("title", words.package);
  }
}

it("says to solve first with no run, and that a run restored from the saved project cannot be rendered or packaged", async () => {
  await installStore();
  render(<App />);
  await screen.findByTestId("workspace-toolbar");
  await expectReportControls(NO_RUN);

  nativeMenuCommand("view.section.solve");
  const run = within(screen.getByTestId("workspace-section-solve")).getByTestId("run-mechanics-preview");
  await waitFor(() => expect(run).toBeEnabled());
  fireEvent.click(run);
  await waitFor(() => expect(screen.getByTestId("status-pill-mechanics")).toHaveAttribute("title", "MECHANICS_SOLVED"), { timeout: 30_000 });
  await settle();
  nativeMenuCommand("file.save-local");
  await waitFor(() => expect(invokeMock.mock.calls.some(([command]) => command === NATIVE_SAVE_PROJECT_COMMAND || command === "save_local_project")).toBe(true));
  await settle();
  nativeMenuCommand("file.open-local");
  nativeMenuCommand("view.section.results");
  expect(await within(screen.getByTestId("workspace-section-results")).findByTestId("historical-run-context", {}, { timeout: 10_000 })).toBeInTheDocument();
  await expectReportControls(RESTORED);
}, 90_000);
