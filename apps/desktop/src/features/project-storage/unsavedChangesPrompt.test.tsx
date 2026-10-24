import { act, fireEvent, render, screen, waitFor } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";

const invokeMock = vi.hoisted(() => vi.fn());
vi.mock("@tauri-apps/api/core", async () => ({ invoke: (await import("../../test/nativeIngress")).ingressDecodingInvoke(invokeMock) }));

import demoModel from "../../../../../fixtures/product_preview/invented_demo_model.json";
import { App } from "../../App";
import type { LocalProjectEnvelope, PreviewModel } from "../../types";

// The unsaved-changes guard (W-12 S1; Agent 0, 2026-10-24): Open Local
// Project, New Blank and Open Model Document ask before they replace a working
// model with unsaved edits. Cancel (the default, focused as the prompt
// appears, and Escape) keeps everything; Save Local Project first goes ahead
// only when that save lands; Discard goes ahead. Open State uses the same
// guard (features/history/modelStateHistory.session.test.tsx).

afterEach(() => {
  vi.restoreAllMocks();
  invokeMock.mockReset();
  delete (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__;
});

const DEMO = demoModel as unknown as PreviewModel;
const OTHER_ID = "project:invented-guard-other";


function setTauriRuntime(present: boolean) {
  if (present) (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__ = {};
  else delete (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__;
}

function nativeMenuCommand(command: string) {
  window.dispatchEvent(new CustomEvent("openpipestress-native-menu-command", { detail: command }));
}

function envelope(model: PreviewModel, message: string): LocalProjectEnvelope {
  return {
    summary: { project_id: model.project.id, project_name: model.project.name, message },
    model, editor_intents: [], proposal: null, selected_review_target: null, mechanics_result: null,
    analysis_run: null, model_hash: null, project_envelope_hash: { value: "sha256:guard-base" }, model_document_version: null
  } as unknown as LocalProjectEnvelope;
}

function otherProject(): PreviewModel {
  const other = structuredClone(DEMO);
  other.project.id = OTHER_ID;
  other.project.name = "Invented guard other project";
  return other;
}

/** The native commands the flows reach. `saveRefusal` makes Save Local Project fail. */
const STORED_DEMO = {
  project_id: DEMO.project.id, project_name: "Stored invented demo", storage_mode: "local_sqlite",
  created_at_unix: 1760000000, updated_at_unix: 1760086400, revision: "sha256:stored@1760086400", saved_state_count: 0
};

function nativeCommands(options: { saveRefusal?: string; storedDemo?: boolean } = {}) {
  invokeMock.mockImplementation((command: string, args?: Record<string, unknown>) => {
    switch (command) {
      case "sync_native_shell_state": return Promise.resolve(null);
      case "list_model_states": return Promise.resolve([]);
      case "find_local_project":
        return Promise.resolve(options.storedDemo && args?.projectId === DEMO.project.id ? STORED_DEMO : null);
      case "save_local_project":
        if (options.saveRefusal) return Promise.reject(options.saveRefusal);
        if (options.storedDemo) {
          const request = args!.request as { model: PreviewModel; confirmed_revision: string | null };
          if (request.model.project.id === DEMO.project.id && request.confirmed_revision !== STORED_DEMO.revision) {
            return Promise.reject(`PROJECT-ID-EXISTS: project \`${DEMO.project.id}\` ("Stored invented demo", last saved at unix 1760086400) is already stored; replacing it needs the user's confirmation.`);
          }
        }
        return Promise.resolve(envelope((args!.request as { model: PreviewModel }).model, "Saved current project model snapshot without external file copies."));
      case "open_local_project": return Promise.resolve(envelope(otherProject(), "Opened local SQLite project snapshot."));
      case "open_model_document_file": return Promise.resolve({ outcome: "cancelled" });
      default: return Promise.reject(new Error(`Unexpected command ${command}`));
    }
  });
}

const called = (command: string) => invokeMock.mock.calls.filter(([name]) => name === command).length;
/** The persistence and model-replacing commands sent, in order. */
const order = () => invokeMock.mock.calls.map(([name]) => name as string)
  .filter((name) => ["save_local_project", "create_local_project", "open_local_project", "open_model_document_file"].includes(name));
const projectMessage = () => screen.getByTestId("local-project-message");
const toolbarProject = () => screen.getByTestId("toolbar-project-name");

async function renderApp() {
  render(<App />);
  await screen.findByTestId("desktop-preview-shell");
  await screen.findByTestId(`tree-row-project-${encodeURIComponent(DEMO.project.id)}`);
}

function openNodeGridReview() {
  const summary = screen.getByTestId("node-grid-review-disclosure");
  if (summary.getAttribute("aria-expanded") !== "true") fireEvent.click(summary);
}

/** One applied edit of node N-100's y through the browser operation engine. */
async function applyNodeEdit() {
  fireEvent.click(screen.getByTestId("layout-mode-grid"));
  openNodeGridReview();
  fireEvent.doubleClick(screen.getByTestId("review-cell-node:N-100-y"));
  fireEvent.change(screen.getByTestId("engineering-table-review").querySelector("input")!, { target: { value: "0.5" } });
  fireEvent.click(screen.getByTestId("queue-entity-grid-intents"));
  fireEvent.click(screen.getAllByTestId(/^apply-intent-editor-intent-\d+$/).at(-1)!);
  await waitFor(() => expect(screen.getByTestId("operation-apply-summary")).toHaveTextContent("1 applied"), { timeout: 10000 });
  await waitFor(() => expect(screen.getByTestId("project-edited")).toHaveTextContent("Edited"));
}

/** The bundled demo with one unsaved edit, then the native runtime. */
async function editedSession(options: { saveRefusal?: string; storedDemo?: boolean } = {}) {
  await renderApp();
  await applyNodeEdit();
  setTauriRuntime(true);
  nativeCommands(options);
}

/** Runs the command and asserts, synchronously, that the prompt is up with Cancel focused. */
function promptFor(command: string) {
  act(() => nativeMenuCommand(command));
  const prompt = screen.getByTestId("unsaved-changes-prompt");
  expect(prompt).toHaveAttribute("role", "alertdialog");
  expect(screen.getByTestId("unsaved-changes-cancel")).toHaveFocus();
  expect(screen.getByTestId("desktop-preview-shell")).toHaveAttribute("inert");
  return prompt;
}

function expectEditsKept() {
  expect(screen.getByTestId("project-edited")).toHaveTextContent("Edited");
  expect(toolbarProject()).toHaveTextContent(DEMO.project.name);
  expect(toolbarProject()).not.toHaveTextContent("Invented guard other project");
}

const KEPT = "the working model and its unsaved edits were kept.";

const commands = [
  {
    name: "Open Local Project",
    command: "file.open-local",
    detail: "Opening the most recent local project replaces the working model.",
    refused: `Not opened: ${KEPT}`,
    replaced: "open_local_project",
    landed: async () => { await waitFor(() => expect(toolbarProject()).toHaveTextContent("Invented guard other project")); }
  },
  {
    name: "New Blank",
    command: "file.new-blank",
    detail: "Creating a new blank project replaces the working model.",
    refused: `Not created: ${KEPT}`,
    replaced: null,
    landed: async () => { expect(await screen.findByTestId("project-units-dialog")).toBeInTheDocument(); }
  },
  {
    name: "Open Model Document",
    command: "file.open-model",
    detail: "Opening a model document replaces the working model.",
    refused: `Not opened: ${KEPT}`,
    replaced: "open_model_document_file",
    landed: async () => { await waitFor(() => expect(called("open_model_document_file")).toBe(1)); }
  }
] as const;

describe.each(commands)("$name with unsaved edits asks first", ({ command, detail, refused, replaced, landed }) => {
  it("Cancel, the focused default, keeps the working model and replaces nothing", async () => {
    await editedSession();
    promptFor(command);
    expect(screen.getByTestId("unsaved-changes-detail")).toHaveTextContent(detail);
    fireEvent.click(screen.getByTestId("unsaved-changes-cancel"));
    await waitFor(() => expect(projectMessage()).toHaveTextContent(refused));
    expect(screen.queryByTestId("unsaved-changes-prompt")).not.toBeInTheDocument();
    expect(screen.queryByTestId("project-units-dialog")).not.toBeInTheDocument();
    expect(order()).toEqual([]);
    expectEditsKept();
  });

  it("Escape is Cancel; Tab stays in the prompt and window shortcuts are held", async () => {
    await editedSession();
    promptFor(command);
    fireEvent.keyDown(screen.getByTestId("unsaved-changes-cancel"), { key: "Tab" });
    expect(screen.getByTestId("unsaved-changes-save")).toHaveFocus();
    fireEvent.keyDown(screen.getByTestId("unsaved-changes-save"), { key: "Tab" });
    fireEvent.keyDown(screen.getByTestId("unsaved-changes-discard"), { key: "Tab" });
    expect(screen.getByTestId("unsaved-changes-cancel")).toHaveFocus();
    fireEvent.keyDown(screen.getByTestId("unsaved-changes-cancel"), { key: "z", metaKey: true });
    expect(screen.getByTestId("project-edited")).toHaveTextContent("Edited");
    fireEvent.keyDown(screen.getByTestId("unsaved-changes-cancel"), { key: "Escape" });
    await waitFor(() => expect(projectMessage()).toHaveTextContent(refused));
    expect(screen.queryByTestId("unsaved-changes-prompt")).not.toBeInTheDocument();
    expect(order()).toEqual([]);
    expectEditsKept();
  });

  it("Discard goes ahead without saving", async () => {
    await editedSession();
    promptFor(command);
    fireEvent.click(screen.getByTestId("unsaved-changes-discard"));
    await landed();
    expect(called("save_local_project")).toBe(0);
    if (replaced) expect(order()).toEqual([replaced]);
  });

  it("Save Local Project first saves the edits, then goes ahead", async () => {
    await editedSession();
    promptFor(command);
    fireEvent.click(screen.getByTestId("unsaved-changes-save"));
    await landed();
    expect(order()).toEqual(replaced ? ["save_local_project", replaced] : ["save_local_project"]);
    const saved = invokeMock.mock.calls.find(([name]) => name === "save_local_project")![1] as { request: { model: PreviewModel } };
    expect(saved.request.model.nodes.find((node) => node.id === "node:N-100")!.position.y).toBe(0.5);
  });

  it("Save first that does not land keeps the working model and replaces nothing", async () => {
    await editedSession({ saveRefusal: "STORE-UNAVAILABLE: the invented store refused the save." });
    promptFor(command);
    fireEvent.click(screen.getByTestId("unsaved-changes-save"));
    await waitFor(() => expect(projectMessage()).toHaveTextContent(refused.replace(KEPT, `the local project was not saved, so ${KEPT}`)));
    expect(order()).toEqual(["save_local_project"]);
    expect(screen.queryByTestId("project-units-dialog")).not.toBeInTheDocument();
    expectEditsKept();
  });

  it("Save first that meets a stored id and is saved as a new project still goes ahead", async () => {
    await editedSession({ storedDemo: true });
    promptFor(command);
    fireEvent.click(screen.getByTestId("unsaved-changes-save"));
    fireEvent.click(await screen.findByTestId("replace-project-save-as-new"));
    await landed();
    const saves = invokeMock.mock.calls.filter(([name]) => name === "save_local_project")
      .map(([, args]) => (args as { request: { model: PreviewModel } }).request.model.project.id);
    expect(saves).toHaveLength(2);
    expect(saves[1]).toMatch(new RegExp(`^${DEMO.project.id}:copy-`));
    expect(order()).toEqual(replaced ? ["save_local_project", "save_local_project", replaced] : ["save_local_project", "save_local_project"]);
  });

  it("does not ask when there are no unsaved edits", async () => {
    await renderApp();
    setTauriRuntime(true);
    nativeCommands();
    expect(screen.queryByTestId("project-edited")).not.toBeInTheDocument();
    act(() => nativeMenuCommand(command));
    expect(screen.queryByTestId("unsaved-changes-prompt")).not.toBeInTheDocument();
    await landed();
    expect(called("save_local_project")).toBe(0);
  });
});
