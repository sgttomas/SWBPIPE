import { act, fireEvent, render, screen, waitFor, within } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";

const invokeMock = vi.hoisted(() => vi.fn());
vi.mock("@tauri-apps/api/core", async () => ({ invoke: (await import("../../test/nativeIngress")).ingressDecodingInvoke(invokeMock) }));

import demoModel from "../../../../../fixtures/product_preview/invented_demo_model.json";
import { App } from "../../App";
import type { StoredProjectEntry } from "../../services/projectService";
import type { LocalProjectEnvelope, PreviewModel } from "../../types";
import { keptStoredProjectMessage, storedProjectDescription } from "./storedProjectConflict";

// HELP_HUMAN ruling (2026-10-10): the app never silently replaces a stored project.

afterEach(() => {
  vi.restoreAllMocks();
  invokeMock.mockReset();
  delete (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__;
});

const STORED: StoredProjectEntry = {
  project_id: demoModel.project.id,
  project_name: "Stored invented demo",
  storage_mode: "local_sqlite",
  created_at_unix: 1760000000,
  updated_at_unix: 1760086400,
  revision: "sha256:stored@1760086400"
};
const REFUSAL = `PROJECT-ID-EXISTS: project \`${STORED.project_id}\` ("${STORED.project_name}", last saved at unix ${STORED.updated_at_unix}) is already stored; replacing it needs the user's confirmation.`;

function nativeMenuCommand(command: string) {
  window.dispatchEvent(new CustomEvent("openpipestress-native-menu-command", { detail: command }));
}

type WriteArgs = {
  model: PreviewModel;
  editorIntents?: unknown[];
  proposal?: unknown;
  selectedReviewTarget?: unknown;
  mechanicsResult?: unknown;
  analysisRun?: unknown;
  modelHash?: unknown;
  projectEnvelopeHash?: unknown;
  confirmedRevision?: string | null;
};

// The store's answer to an admitted write: the request echoed as a snapshot.
function stored(args: WriteArgs, message: string): LocalProjectEnvelope {
  return {
    summary: {
      project_id: args.model.project.id,
      project_name: args.model.project.name,
      database_path: "/invented/local/store.sqlite",
      storage_mode: "local_sqlite",
      migration_status: "current",
      migration_framework: "application_service_separate_db_and_product_schema",
      store_schema_version: 1,
      store_schema_target_version: 1,
      migrations_applied_on_open: [],
      fts_indexed: true,
      copied_external_files: false,
      editor_intent_count: 0,
      proposal_count: 0,
      selected_review_target_count: 0,
      selected_review_target_ref: "none",
      persisted_mechanics_result_count: args.mechanicsResult ? 1 : 0,
      persisted_analysis_run_count: 0,
      persisted_analysis_run_ref: "none",
      persisted_model_hash_count: 1,
      persisted_model_hash_ref: args.model.project.id,
      persisted_project_envelope_hash_count: 1,
      persisted_project_envelope_hash_ref: args.model.project.id,
      unit_round_trip_status: "not_checked_in_invented_test",
      unit_round_trip_checked_ref_count: 0,
      unit_round_trip_signature: "none",
      message
    },
    model: args.model,
    editor_intents: (args.editorIntents ?? []) as LocalProjectEnvelope["editor_intents"],
    proposal: (args.proposal ?? null) as LocalProjectEnvelope["proposal"],
    selected_review_target: (args.selectedReviewTarget ?? null) as LocalProjectEnvelope["selected_review_target"],
    mechanics_result: (args.mechanicsResult ?? null) as LocalProjectEnvelope["mechanics_result"],
    analysis_run: (args.analysisRun ?? null) as LocalProjectEnvelope["analysis_run"],
    model_hash: (args.modelHash ?? null) as LocalProjectEnvelope["model_hash"],
    project_envelope_hash: (args.projectEnvelopeHash ?? null) as LocalProjectEnvelope["project_envelope_hash"],
    model_document_version: null
  };
}

async function renderNativeApp(handler: (command: string, args: Record<string, unknown>) => unknown) {
  invokeMock.mockImplementation((command: string, args?: Record<string, unknown>) => {
    if (command === "sync_native_shell_state") return Promise.resolve(null);
    try {
      return Promise.resolve(handler(command, args ?? {}));
    } catch (error) {
      return Promise.reject(error);
    }
  });
  render(<App />);
  await screen.findByTestId("desktop-preview-shell");
  await screen.findByTestId(`tree-row-project-${encodeURIComponent(demoModel.project.id)}`);
  (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__ = {};
}

const message = () => screen.getByTestId("local-project-message");
const writes = (command: string) => invokeMock.mock.calls.filter(([name]) => name === command).map(([, args]) => args as WriteArgs);

describe("saving onto a stored project id asks first", () => {
  it("names the stored project, focuses Cancel, and Cancel keeps the stored project", async () => {
    await renderNativeApp((command, args) => {
      if (command === "create_local_project" && (args as WriteArgs).confirmedRevision !== STORED.revision) throw REFUSAL;
      if (command === "find_local_project") return STORED;
      throw new Error(`Unexpected command ${command}`);
    });
    act(() => nativeMenuCommand("file.new-local"));
    const prompt = await screen.findByTestId("replace-project-prompt");
    expect(prompt).toHaveAttribute("role", "alertdialog");
    expect(screen.getByTestId("replace-project-detail")).toHaveTextContent(
      `Creating this project would replace the stored project ${storedProjectDescription(STORED)}.`
    );
    expect(screen.getByTestId("replace-project-cancel")).toHaveFocus();
    fireEvent.click(screen.getByTestId("replace-project-cancel"));
    await waitFor(() => expect(message()).toHaveTextContent(keptStoredProjectMessage(STORED)));
    expect(screen.queryByTestId("replace-project-prompt")).not.toBeInTheDocument();
    expect(writes("create_local_project").map((args) => args.confirmedRevision)).toEqual([null]);
  });

  it("Escape is Cancel", async () => {
    await renderNativeApp((command, args) => {
      if (command === "create_local_project" && (args as WriteArgs).confirmedRevision !== STORED.revision) throw REFUSAL;
      if (command === "find_local_project") return STORED;
      throw new Error(`Unexpected command ${command}`);
    });
    act(() => nativeMenuCommand("file.new-local"));
    fireEvent.keyDown(await screen.findByTestId("replace-project-prompt"), { key: "Escape" });
    await waitFor(() => expect(message()).toHaveTextContent(keptStoredProjectMessage(STORED)));
    expect(writes("create_local_project")).toHaveLength(1);
  });

  it("Replace writes again, confirmed", async () => {
    await renderNativeApp((command, args) => {
      const write = args as WriteArgs;
      if (command === "create_local_project") {
        if (write.confirmedRevision !== STORED.revision) throw REFUSAL;
        return stored(write, "Created local SQLite project snapshot without external file copies.");
      }
      if (command === "find_local_project") return STORED;
      throw new Error(`Unexpected command ${command}`);
    });
    act(() => nativeMenuCommand("file.new-local"));
    fireEvent.click(await screen.findByTestId("replace-project-replace"));
    await waitFor(() => expect(message()).toHaveTextContent("Created local SQLite project snapshot"));
    expect(writes("create_local_project").map((args) => [args.model.project.id, args.confirmedRevision])).toEqual([
      [STORED.project_id, null],
      [STORED.project_id, STORED.revision]
    ]);
  });

  it("Replace confirms only the revision it showed: one saved meanwhile is asked about again", async () => {
    const LATER: StoredProjectEntry = { ...STORED, updated_at_unix: STORED.updated_at_unix + 60, revision: "sha256:later@1760086460" };
    let current = STORED;
    await renderNativeApp((command, args) => {
      const write = args as WriteArgs;
      if (command === "create_local_project") {
        if (write.confirmedRevision === current.revision) return stored(write, "Created local SQLite project snapshot without external file copies.");
        throw write.confirmedRevision ? REFUSAL.replace("PROJECT-ID-EXISTS", "PROJECT-BASE-MOVED") : REFUSAL;
      }
      if (command === "find_local_project") return current;
      throw new Error(`Unexpected command ${command}`);
    });
    act(() => nativeMenuCommand("file.new-local"));
    await screen.findByTestId("replace-project-prompt");
    current = LATER; // another window saves while the prompt is open
    fireEvent.click(screen.getByTestId("replace-project-replace"));
    expect(await screen.findByText("The stored copy of this project has changed")).toBeInTheDocument();
    expect(screen.getByTestId("replace-project-detail")).toHaveTextContent(storedProjectDescription(LATER));
    fireEvent.click(screen.getByTestId("replace-project-replace"));
    await waitFor(() => expect(message()).toHaveTextContent("Created local SQLite project snapshot"));
    expect(writes("create_local_project").map((args) => args.confirmedRevision)).toEqual([null, STORED.revision, LATER.revision]);
  });

  it("Save as a new project writes the same document under an unused id, without the old run", async () => {
    await renderNativeApp((command, args) => {
      const write = args as WriteArgs;
      if (command === "create_local_project") {
        if (write.model.project.id === STORED.project_id && write.confirmedRevision !== STORED.revision) throw REFUSAL;
        return stored(write, "Created local SQLite project snapshot without external file copies.");
      }
      if (command === "find_local_project") return (args as { projectId: string }).projectId === STORED.project_id ? STORED : null;
      throw new Error(`Unexpected command ${command}`);
    });
    act(() => nativeMenuCommand("file.new-local"));
    fireEvent.click(await screen.findByTestId("replace-project-save-as-new"));
    await waitFor(() => expect(message()).toHaveTextContent("Created local SQLite project snapshot"));
    const [, second] = writes("create_local_project");
    expect(second.confirmedRevision).toBeNull();
    expect(second.model.project.id).toMatch(new RegExp(`^${STORED.project_id}:copy-[0-9t]+z$`));
    expect(second.mechanicsResult).toBeNull();
    const { project: renamedProject, ...renamedRest } = second.model;
    const { project: sourceProject, ...sourceRest } = demoModel as unknown as PreviewModel;
    expect(renamedRest).toEqual(sourceRest);
    expect({ ...renamedProject, id: sourceProject.id }).toEqual(sourceProject);
    expect(screen.getByTestId(`tree-row-project-${encodeURIComponent(second.model.project.id)}`)).toBeInTheDocument();
    expect(screen.queryByTestId(`tree-row-project-${encodeURIComponent(STORED.project_id)}`)).not.toBeInTheDocument();
  });

  it("Save as a new project from Save Local Project saves under the unused id", async () => {
    await renderNativeApp((command, args) => {
      const request = (args as { request?: { confirmed_revision?: string | null; model: PreviewModel } }).request;
      if (command === "save_local_project") {
        if (request!.model.project.id === STORED.project_id && request!.confirmed_revision !== STORED.revision) throw REFUSAL;
        return stored({ model: request!.model }, "Saved local SQLite project snapshot.");
      }
      if (command === "find_local_project") return (args as { projectId: string }).projectId === STORED.project_id ? STORED : null;
      throw new Error(`Unexpected command ${command}`);
    });
    act(() => nativeMenuCommand("file.save-local"));
    fireEvent.click(await screen.findByTestId("replace-project-save-as-new"));
    await waitFor(() => expect(message()).toHaveTextContent("Saved local SQLite project snapshot."));
    const saves = invokeMock.mock.calls.filter(([name]) => name === "save_local_project")
      .map(([, args]) => (args as { request: { model: PreviewModel; confirmed_revision: string | null; mechanics_result: unknown } }).request);
    expect(saves.map((request) => request.confirmed_revision)).toEqual([null, null]);
    expect(saves[1].model.project.id).toMatch(new RegExp(`^${STORED.project_id}:copy-`));
    expect(saves[1].mechanics_result).toBeNull();
  });

  it("Save Local Project: the bundled demo asks; the session's own project saves in place; a moved base asks again", async () => {
    let storedBase = "sha256:stored-revision-1";
    const openedEnvelope = {
      ...stored({ model: demoModel as unknown as PreviewModel }, "Opened local SQLite project snapshot."),
      project_envelope_hash: { value: storedBase } as LocalProjectEnvelope["project_envelope_hash"]
    };
    await renderNativeApp((command, args) => {
      const request = (args as { request?: { confirmed_revision?: string | null; expected_base?: string | null; model: PreviewModel } }).request;
      if (command === "save_local_project") {
        if (request!.confirmed_revision !== STORED.revision && request!.expected_base !== storedBase) {
          throw request!.expected_base == null ? REFUSAL : REFUSAL.replace("PROJECT-ID-EXISTS", "PROJECT-BASE-MOVED");
        }
        return { ...stored({ model: request!.model }, "Saved local SQLite project snapshot."), project_envelope_hash: { value: storedBase } };
      }
      if (command === "find_local_project") return STORED;
      if (command === "open_local_project") return openedEnvelope;
      throw new Error(`Unexpected command ${command}`);
    });
    // The bundled demo is not the stored demo: its save asks.
    act(() => nativeMenuCommand("file.save-local"));
    expect(await screen.findByTestId("replace-project-detail")).toHaveTextContent("Saving this project would replace the stored project");
    fireEvent.click(screen.getByTestId("replace-project-cancel"));
    await waitFor(() => expect(message()).toHaveTextContent(keptStoredProjectMessage(STORED)));
    // Opened from the store, the session is that project: Save is a save in place.
    act(() => nativeMenuCommand("file.open-local"));
    await waitFor(() => expect(message()).toHaveTextContent("Opened local SQLite project snapshot."));
    act(() => nativeMenuCommand("file.save-local"));
    await waitFor(() => expect(message()).toHaveTextContent("Saved local SQLite project snapshot."));
    expect(screen.queryByTestId("replace-project-prompt")).not.toBeInTheDocument();
    // Another window saves that id: the session's base is no longer the stored revision.
    storedBase = "sha256:stored-revision-2";
    act(() => nativeMenuCommand("file.save-local"));
    expect(await screen.findByTestId("replace-project-detail")).toHaveTextContent(
      `The stored project ${storedProjectDescription(STORED)} has been saved since this session last saw it.`
    );
    fireEvent.click(screen.getByTestId("replace-project-cancel"));
    await waitFor(() => expect(message()).toHaveTextContent(keptStoredProjectMessage(STORED)));
    const saves = invokeMock.mock.calls.filter(([name]) => name === "save_local_project")
      .map(([, args]) => (args as { request: { confirmed_revision: string | null; expected_base: string | null } }).request);
    expect(saves.map((request) => [request.confirmed_revision, request.expected_base])).toEqual([
      [null, null],
      [null, "sha256:stored-revision-1"],
      [null, "sha256:stored-revision-1"]
    ]);
  });

  it("a save in place that lands while the user acts still moves the session's base", async () => {
    let storedBase = "sha256:rev-1";
    let release: (() => void) | null = null;
    const openedEnvelope = {
      ...stored({ model: demoModel as unknown as PreviewModel }, "Opened local SQLite project snapshot."),
      project_envelope_hash: { value: storedBase } as LocalProjectEnvelope["project_envelope_hash"]
    };
    await renderNativeApp((command, args) => {
      const request = (args as { request?: { confirmed_revision?: string | null; expected_base?: string | null; model: PreviewModel } }).request;
      if (command === "save_local_project") {
        if (request!.confirmed_revision !== STORED.revision && request!.expected_base !== storedBase) throw REFUSAL.replace("PROJECT-ID-EXISTS", "PROJECT-BASE-MOVED");
        const next = storedBase === "sha256:rev-1" ? "sha256:rev-2" : "sha256:rev-3";
        const answer = () => { storedBase = next; return { ...stored({ model: request!.model }, "Saved current project model snapshot."), project_envelope_hash: { value: next } }; };
        if (next === "sha256:rev-2") return new Promise((resolve) => { release = () => resolve(answer()); });
        return answer();
      }
      if (command === "find_local_project") return STORED;
      if (command === "open_local_project") return openedEnvelope;
      throw new Error(`Unexpected command ${command}`);
    });
    act(() => nativeMenuCommand("file.open-local"));
    await waitFor(() => expect(message()).toHaveTextContent("Opened local SQLite project snapshot."));
    act(() => nativeMenuCommand("file.save-local"));
    await waitFor(() => expect(release).not.toBeNull());
    // The user selects something while the save is in flight (the request epoch moves).
    const tree = screen.getByLabelText("Model tree");
    fireEvent.change(within(tree).getByTestId("model-tree-filter-input"), { target: { value: "node:N-100" } });
    const nodeRow = await within(tree).findByTestId(`tree-row-node-${encodeURIComponent("node:N-100")}`);
    fireEvent.click(nodeRow);
    expect(nodeRow).toHaveAttribute("aria-selected", "true");
    await act(async () => { release!(); });
    await waitFor(() => expect(storedBase).toBe("sha256:rev-2"));
    act(() => nativeMenuCommand("file.save-local"));
    await waitFor(() => expect(storedBase).toBe("sha256:rev-3"));
    expect(screen.queryByTestId("replace-project-prompt")).not.toBeInTheDocument();
    const bases = invokeMock.mock.calls.filter(([name]) => name === "save_local_project")
      .map(([, args]) => (args as { request: { expected_base: string | null } }).request.expected_base);
    expect(bases).toEqual(["sha256:rev-1", "sha256:rev-2"]);
  });

  it("is modal: the workspace is inert, and menu commands and shortcuts wait for the answer", async () => {
    await renderNativeApp((command, args) => {
      if (command === "create_local_project" && (args as WriteArgs).confirmedRevision !== STORED.revision) throw REFUSAL;
      if (command === "find_local_project") return STORED;
      throw new Error(`Unexpected command ${command}`);
    });
    act(() => nativeMenuCommand("file.new-local"));
    await screen.findByTestId("replace-project-prompt");
    expect(screen.getByTestId("desktop-preview-shell")).toHaveAttribute("inert");
    const before = invokeMock.mock.calls.length;
    act(() => nativeMenuCommand("file.save-local"));
    fireEvent.keyDown(screen.getByTestId("replace-project-cancel"), { key: "z", metaKey: true });
    fireEvent.keyDown(screen.getByTestId("replace-project-cancel"), { key: "Tab" });
    expect(screen.getByTestId("replace-project-save-as-new")).toHaveFocus();
    expect(invokeMock.mock.calls.slice(before).filter(([name]) => name !== "sync_native_shell_state")).toEqual([]);
    expect(screen.getByTestId("replace-project-prompt")).toBeInTheDocument();
    fireEvent.click(screen.getByTestId("replace-project-cancel"));
    await waitFor(() => expect(screen.getByTestId("desktop-preview-shell")).not.toHaveAttribute("inert"));
  });
});
